#!/usr/bin/env python3
"""Cross-check that Selection/Cert.lean is the algorithm of the .rs file.

  python3 -I tools/crosscheck.py struct RS CERT_LEAN
      structural: parse Cert.lean (own regexes) and compare every function
      with the parse of the .rs file by code/verifier/reference_semantics.py
      (an independent parser, shares no code with tools/rs2lean.py).
  python3 -I tools/crosscheck.py perms OUT K SEED [N]
      write K random permutations of 0..N-1 (+ identity, reverse) to OUT
      (default N = 16).
  python3 -I tools/crosscheck.py compare RS PERMS LEAN_RESULTS [W]
      behavioural: compare the Lean `exec` results (value, #comparisons) with
      reference_semantics.execute on the same inputs (and, if W is given,
      check max #comparisons <= W).
"""
import os, random, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "verifier"))
import reference_semantics as ref  # noqa: E402

FN = re.compile(r"/- (select_\d+) -/ \{ arity := (\d+), rank := (\d+), body := (.*) \},?$")


def parse_stmt(s):
    m = re.fullmatch(r"\.ret (\d+)", s)
    if m:
        return ("ret", int(m.group(1)))
    m = re.fullmatch(r"\.call (\d+) \[([\d, ]*)\]", s)
    if m:
        args = [int(t) for t in m.group(2).split(",")] if m.group(2).strip() else []
        return ("call", int(m.group(1)), args)
    raise ValueError(s)


def parse_body(b):
    m = re.fullmatch(r"\.stmt \((.*)\)", b)
    if m:
        return ("stmt", parse_stmt(m.group(1)))
    m = re.fullmatch(r"\.cmp (\d+) (\d+) \(([^()]*)\) \(([^()]*)\)", b)
    if m:
        return ("cmp", int(m.group(1)), int(m.group(2)), parse_stmt(m.group(3)), parse_stmt(m.group(4)))
    raise ValueError(b)


def struct(rs, lean):
    prog = ref.load(open(rs).read())
    text = open(lean).read()
    root = re.search(r"root := (\d+)", text)
    entries = [FN.search(l.strip()) for l in text.splitlines() if l.strip().startswith("/- select_")]
    assert all(entries), "unparsed Cert.lean line"
    assert len(entries) == len(prog.funcs), (len(entries), len(prog.funcs))
    for k, m in enumerate(entries):
        name = m.group(1)
        assert name == f"select_{k}", (k, name)
        doc_n, doc_i, params, dual, body = prog.funcs[name]
        assert not dual
        assert int(m.group(2)) == len(params) == doc_n, name
        assert int(m.group(3)) == doc_i, name
        pos = {v: j for j, v in enumerate(params)}

        def tr(s):
            if s[0] == "ret":
                return ("ret", pos[s[1]])
            assert s[3] is None
            return ("call", int(s[1][len("select_"):]), [pos[a] for a in s[2]])
        if body[0] == "stmt":
            want = ("stmt", tr(body[1]))
        else:
            assert body[0] == "if" and body[1][0] == "lt", name
            want = ("cmp", pos[body[1][1]], pos[body[1][2]], tr(body[2]), tr(body[3]))
        assert parse_body(m.group(4)) == want, (name, m.group(4), want)
    assert prog.root == "select_0" and int(root.group(1)) == 0 and not prog.root_dual
    print(f"struct: OK, {len(entries)} functions identical to reference_semantics' parse "
          f"(root select_0, n = {prog.n}, rank = {prog.rank})")


def perms(out, k, seed, n=16):
    rng = random.Random(seed)
    rows = [list(range(n)), list(range(n - 1, -1, -1))]
    for _ in range(k):
        p = list(range(n))
        rng.shuffle(p)
        rows.append(p)
    with open(out, "w") as f:
        for p in rows:
            f.write(" ".join(map(str, p)) + "\n")
    print(f"perms: wrote {len(rows)} permutations to {out}")


def compare(rs, permfile, results, w=None):
    prog = ref.load(open(rs).read())
    ps = [list(map(int, l.split())) for l in open(permfile) if l.strip()]
    assert all(sorted(p) == list(range(prog.n)) for p in ps), "inputs are not permutations of 0..n-1"
    rs_ = [l.split() for l in open(results) if l.strip()]
    assert len(ps) == len(rs_), (len(ps), len(rs_))
    worst = 0
    for p, r in zip(ps, rs_):
        v, c = ref.execute(prog, p)
        assert r == [str(v), str(c)], (p, r, v, c)
        assert v == prog.rank
        worst = max(worst, c)
    assert w is None or worst <= w, (worst, w)
    print(f"compare: OK, {len(ps)} inputs, Lean exec == reference semantics "
          f"(value and #comparisons), all return rank {prog.rank}, max comparisons {worst}"
          + ("" if w is None else f" <= {w}"))


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "struct":
        struct(sys.argv[2], sys.argv[3])
    elif cmd == "perms":
        perms(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), *map(int, sys.argv[5:6]))
    elif cmd == "compare":
        compare(sys.argv[2], sys.argv[3], sys.argv[4], *map(int, sys.argv[5:6]))
    else:
        sys.exit(__doc__)
