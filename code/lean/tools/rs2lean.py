#!/usr/bin/env python3
"""rs2lean.py -- translate a PLAIN-dialect selection certificate (.rs) into Lean data.

    python3 -I tools/rs2lean.py CERT.rs --cert Selection/Cert.lean --hints Selection/Hints.lean [--ns Cert]

        [--parts K]
(--ns X puts the data into namespace Sel.X; default Sel.Cert.  --parts K writes
each array as the concatenation (`List.flatten`) of blocks of K entries (needed
for thousands of functions: one huge literal exceeds Lean's recursion and
heartbeat limits; Selection/Fast.lean uses the blocks for fast lookups).)

NOT TRUSTED.  The Lean theorem is about the data this script writes (Cert.lean);
tools/crosscheck.py compares that data with an independent parse of the .rs
file (code/verifier/reference_semantics.py) and compares Lean `exec` with the
reference interpreter on random inputs (see README.txt, M2).

Supported grammar (strict; anything else is an error):
    #[allow(...)] / pub const N|I: usize = <int>; / pub fn run(input: [usize; n]) -> usize { select_K(input) }
    /// n = <m>[, i = <r>][, u < v]*
    fn select_K([p0, p1, ..]: [usize; <m>]) -> usize {
        BODY
    }
    BODY = STMT | if X < Y {\\n STMT\\n } else {\\n STMT\\n }
    STMT = name | select_K([q0, q1, ..])
Function select_K becomes funs[K]; names must be exactly select_0 .. select_{F-1}.

Hints (also untrusted; the Lean checker re-checks everything): for every
function the precondition poset P_f (intersection over all call sites of the
knowledge mapped to the callee's positions, computed top-down as in
code/verifier/README.txt section 5), encoded as a Nat with bit STRIDE*u+v set iff
x_u < x_v is required (STRIDE = 32, must equal the constant in `preRel`,
Selection/Checker.lean; arities > STRIDE are refused); height = longest call chain below f; cost = largest
number of comparisons from f.
"""
import hashlib
import re
import sys

STRIDE = 32   # must match `preRel` in Selection/Checker.lean (was 16: arity <= 16 only)
NAME = r"[a-z][a-z0-9_]*"


def die(msg):
    sys.exit("rs2lean: " + msg)


def parse(text):
    lines = [l.strip() for l in text.split("\n")]
    i, funs, consts, run_root, doc = 0, {}, {}, None, None

    def call(s, params):
        m = re.fullmatch(r"select_(\d+)\(\[([^\]]*)\]\)", s)
        if not m:
            die("bad statement: %r" % s)
        args = [a.strip() for a in m.group(2).split(",")]
        for a in args:
            if a not in params:
                die("unknown argument %r" % a)
        return ("call", int(m.group(1)), [params.index(a) for a in args])

    def stmt(s, params):
        if re.fullmatch(NAME, s):
            if s not in params:
                die("unknown name %r" % s)
            return ("ret", params.index(s))
        return call(s, params)

    while i < len(lines):
        l = lines[i]
        if l == "" or l.startswith("#[allow("):
            i += 1
            continue
        m = re.fullmatch(r"pub const (N|I): usize = (\d+);", l)
        if m:
            consts[m.group(1)] = int(m.group(2))
            i += 1
            continue
        m = re.fullmatch(r"pub fn run\(input: \[usize; (\d+)\]\) -> usize \{", l)
        if m:
            m2 = re.fullmatch(r"select_(\d+)\(input\)", lines[i + 1])
            if not m2 or lines[i + 2] != "}":
                die("bad run wrapper")
            run_root, run_n = int(m2.group(1)), int(m.group(1))
            i += 3
            continue
        m = re.fullmatch(r"/// n = (\d+)(?:, i = (\d+))?((?:, %s < %s)*)" % (NAME, NAME), l)
        if m:
            doc = (int(m.group(1)), None if m.group(2) is None else int(m.group(2)),
                   re.findall(r"(%s) < (%s)" % (NAME, NAME), m.group(3)))
            i += 1
            continue
        m = re.fullmatch(r"fn select_(\d+)\(\[([^\]]*)\]: \[usize; (\d+)\]\) -> usize \{", l)
        if not m:
            die("line %d: unexpected %r" % (i + 1, l))
        k, params, arity = int(m.group(1)), [p.strip() for p in m.group(2).split(",")], int(m.group(3))
        if doc is None or doc[0] != arity or len(params) != arity or len(set(params)) != arity:
            die("line %d: doc comment / arity mismatch" % (i + 1))
        if k in funs:
            die("duplicate select_%d" % k)
        rank = doc[1] if doc[1] is not None else 0
        claims = [(params.index(u), params.index(v)) for u, v in doc[2]]
        m = re.fullmatch(r"if (%s) < (%s) \{" % (NAME, NAME), lines[i + 1])
        if m:
            if lines[i + 3] != "} else {" or lines[i + 5] != "}" or lines[i + 6] != "}":
                die("line %d: bad if layout" % (i + 2))
            x, y = params.index(m.group(1)), params.index(m.group(2))
            body = ("cmp", x, y, stmt(lines[i + 2], params), stmt(lines[i + 4], params))
            i += 7
        else:
            if lines[i + 2] != "}":
                die("line %d: bad body" % (i + 2))
            body = ("stmt", stmt(lines[i + 1], params))
            i += 3
        funs[k] = dict(arity=arity, rank=rank, body=body, claims=claims)
        doc = None
    if sorted(funs) != list(range(len(funs))):
        die("function names are not exactly select_0 .. select_%d" % (len(funs) - 1))
    if run_root is None:
        die("no run wrapper")
    if run_n != funs[run_root]["arity"] or consts.get("N", run_n) != run_n \
            or consts.get("I", funs[run_root]["rank"]) != funs[run_root]["rank"]:
        die("run / N / I inconsistent")
    return [funs[k] for k in range(len(funs))], run_root


def stmts(body):
    return [body[1]] if body[0] == "stmt" else [body[3], body[4]]


def closure_add(rel, lo, hi):
    """rel: set of pairs (closed).  Return closure(rel + {lo < hi})."""
    below = {u for (u, v) in rel if v == lo} | {lo}
    above = {v for (u, v) in rel if u == hi} | {hi}
    return rel | {(u, v) for u in below for v in above}


def closure(pairs, n):
    rel = set()
    for (u, v) in pairs:
        rel = closure_add(rel, u, v)
    return rel


def hints(funs, root):
    F = len(funs)
    # topological order (callers before callees) by DFS post-order
    order, seen = [], set()

    def visit(f):
        if f in seen:
            return
        seen.add(f)
        for s in stmts(funs[f]["body"]):
            if s[0] == "call":
                visit(s[1])
        order.append(f)
    sys.setrecursionlimit(100000)
    visit(root)
    if len(order) != F:
        die("unreachable functions")
    order.reverse()
    pos = {f: t for t, f in enumerate(order)}
    pre = {root: set()}
    for f in order:
        if f not in pre:
            die("select_%d never called" % f)
        P, body = pre[f], funs[f]["body"]
        if body[0] == "stmt":
            sites = [(P, body[1])]
        else:
            _, x, y, s, t = body
            sites = [(closure_add(P, x, y), s), (closure_add(P, y, x), t)]
        for Q, s in sites:
            if s[0] != "call":
                continue
            g, args = s[1], s[2]
            if pos[g] <= pos[f]:
                die("cycle")
            M = {(j, l) for j in range(len(args)) for l in range(len(args)) if (args[j], args[l]) in Q}
            pre[g] = M if g not in pre else (pre[g] & M)
    height, cost = {}, {}
    for f in reversed(order):
        body = funs[f]["body"]
        hs = [0] + [height[s[1]] + 1 for s in stmts(body) if s[0] == "call"]
        cs = [0] + [cost[s[1]] for s in stmts(body) if s[0] == "call"]
        height[f] = max(hs)
        cost[f] = max(cs) + (1 if body[0] == "cmp" else 0)
    for f in range(F):
        if closure(funs[f]["claims"], funs[f]["arity"]) != pre[f]:
            print("note: select_%d: doc-comment relations differ from computed precondition" % f)
        if max(funs[f]["arity"], 1) > STRIDE:
            die("arity > %d" % STRIDE)
    enc = [sum(1 << (STRIDE * u + v) for (u, v) in pre[f]) for f in range(F)]
    return enc, height, cost


def lean_stmt(s):
    if s[0] == "ret":
        return "(.ret %d)" % s[1]
    return "(.call %d [%s])" % (s[1], ", ".join(map(str, s[2])))


def lean_body(b):
    if b[0] == "stmt":
        return ".stmt " + lean_stmt(b[1])
    return ".cmp %d %d %s %s" % (b[1], b[2], lean_stmt(b[3]), lean_stmt(b[4]))


def main(argv):
    ns, parts = "Cert", None
    opts = argv[6:]
    argv = argv[:6]
    while opts:
        if len(opts) >= 2 and opts[0] == "--ns" and re.fullmatch(r"[A-Z][A-Za-z0-9]*", opts[1]):
            ns = opts[1]
        elif len(opts) >= 2 and opts[0] == "--parts" and opts[1].isdigit() and int(opts[1]) > 0:
            parts = int(opts[1])
        else:
            die(__doc__.split("\n")[2].strip())
        opts = opts[2:]
    if len(argv) != 6 or argv[2] != "--cert" or argv[4] != "--hints":
        die(__doc__.split("\n")[2].strip())
    src, cert_out, hints_out = argv[1], argv[3], argv[5]
    raw = open(src, "rb").read()
    sha = hashlib.sha256(raw).hexdigest()
    funs, root = parse(raw.decode("ascii"))
    enc, height, cost = hints(funs, root)
    name = src.split("/")[-1]
    F = len(funs)
    fun_lines = ["/- select_%d -/ { arity := %d, rank := %d, body := %s }"
                 % (k, f["arity"], f["rank"], lean_body(f["body"])) for k, f in enumerate(funs)]
    hint_lines = ["/- select_%d -/ ⟨0x%x, %d, %d⟩" % (k, enc[k], height[k], cost[k]) for k in range(F)]

    def items(out, lines):
        for k, l in enumerate(lines):
            out.write("    %s%s\n" % (l, "," if k + 1 < len(lines) else ""))

    def parted(out, base, typ, lines):
        """blocks of `parts` entries `def base_j : List typ`, collected in
        `def base_parts : List (List typ)`; returns the term for the array."""
        names = []
        for j, s0 in enumerate(range(0, len(lines), parts)):
            names.append("%s_%d" % (base, j))
            out.write("def %s : List %s := [\n" % (names[-1], typ))
            items(out, lines[s0:s0 + parts])
            out.write("  ]\n\n")
        out.write("/-- the blocks in order; the table is their concatenation -/\n")
        out.write("def %s_parts : List (List %s) :=\n  [%s]\n\n" % (base, typ, ", ".join(names)))
        return "%s_parts.flatten.toArray" % base

    with open(cert_out, "w") as out:
        out.write("import Selection.Spec\n\n")
        out.write("/-! GENERATED by tools/rs2lean.py from %s\n    (sha256 %s).\n" % (name, sha))
        out.write("    Entry K of `funs` is `select_K` of the Rust file; argument positions are\n")
        out.write("    0-based indexes into the parameter list [a, b, c, ...]. -/\n\n")
        out.write("namespace Sel.%s\nopen Sel\n\n" % ns)
        if parts is None:
            out.write("def prog : Program where\n  root := %d\n  funs := #[\n" % root)
            items(out, fun_lines)
            out.write("  ]\n\nend Sel.%s\n" % ns)
        else:
            term = parted(out, "funs", "Fn", fun_lines)
            out.write("def prog : Program where\n  root := %d\n  funs := %s\n\nend Sel.%s\n"
                      % (root, term, ns))
    with open(hints_out, "w") as out:
        out.write("import Selection.Checker\n\n")
        out.write("/-! GENERATED by tools/rs2lean.py from %s (sha256 %s).\n" % (name, sha))
        out.write("    Untrusted hints for the checker: precondition poset (bit %d*u+v set iff\n" % STRIDE)
        out.write("    x_u < x_v is required), call height, comparison cost. -/\n\n")
        out.write("namespace Sel.%s\nopen Sel\n\n" % ns)
        if parts is None:
            out.write("def hints : Array Hint := #[\n")
            items(out, hint_lines)
            out.write("  ]\n\nend Sel.%s\n" % ns)
        else:
            term = parted(out, "hints", "Hint", hint_lines)
            out.write("def hints : Array Hint := %s\n\nend Sel.%s\n" % (term, ns))
    print("rs2lean: %s: %d functions, root select_%d (n = %d, rank %d), height %d, cost %d"
          % (name, F, root, funs[root]["arity"], funs[root]["rank"], height[root], cost[root]))


if __name__ == "__main__":
    main(sys.argv)
