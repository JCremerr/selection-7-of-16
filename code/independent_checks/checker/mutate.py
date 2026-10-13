"""Mutation campaign against verify_rs_algorithm.py on the real certificates.
Soundness violation = verifier ACCEPTs with (n, i0, W) but the exact oracle says
the program does not select rank i0, or its exact worst case exceeds W."""
import random, re, sys, collections
import os; sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "verifier"))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import verify_rs_algorithm as V
from oracle import oracle

def index(lines):
    """name -> dict(doc, sig, params, cmp, calls=[line idx], let=bool, i)"""
    fns, cur = {}, None
    for k, s in enumerate(lines):
        m = re.match(r"\s*fn (select_\d+)\(\[([a-z0-9_, ]*)\]", s)
        if m:
            cur = dict(name=m.group(1), doc=k - 1, sig=k, params=[t.strip() for t in m.group(2).split(",")],
                       cmp=None, calls=[], let=None, vals=[])
            fns[cur["name"]] = cur
            dm = re.search(r"i = (\d+)", lines[k - 1]); cur["i"] = int(dm.group(1)) if dm else 0
            continue
        if cur is None: continue
        t = s.strip()
        if re.match(r"(let .* = )?if [a-z]", t): cur["cmp"] = k; cur["let"] = t.startswith("let")
        elif re.fullmatch(r"select_\d+\(\[.*\]\)", t): cur["calls"].append(k)
        elif cur.get("let") and re.fullmatch(r"[a-z][a-z0-9_]*|\(.*\)", t): cur["vals"].append(k)
    return fns

def args_of(s):
    m = re.search(r"\(\[(.*)\]\)", s); return [a.strip() for a in m.group(1).split(",")]
def set_args(s, args):
    return re.sub(r"\(\[(.*)\]\)", "([" + ", ".join(args) + "])", s)

def mutants(text, rng, per_op):
    lines = text.split("\n"); fns = index(lines)
    names = list(fns)
    by_arity = collections.defaultdict(list)
    for f in fns.values(): by_arity[(len(f["params"]), f["i"])].append(f["name"])
    def out(op, L, info): return (op, "\n".join(L), info)
    res = []
    callfns = [f for f in fns.values() if f["calls"]]
    for _ in range(per_op):
        # replace_arg: pass a dropped element instead of an argument
        f = rng.choice(callfns); k = rng.choice(f["calls"]); L = list(lines)
        a = args_of(L[k]); dropped = [p for p in f["params"] if p not in a]
        if dropped and not f.get("let"):
            j = rng.randrange(len(a)); a[j] = rng.choice(dropped); L[k] = set_args(L[k], a)
            res.append(out("replace_arg", L, (f["name"], k)))
        # swap_args
        f = rng.choice(callfns); k = rng.choice(f["calls"]); L = list(lines); a = args_of(L[k])
        if len(a) >= 2:
            j, l = rng.sample(range(len(a)), 2); a[j], a[l] = a[l], a[j]; L[k] = set_args(L[k], a)
            res.append(out("swap_args", L, (f["name"], k)))
        # redirect to another function of the same arity and same doc rank (only precondition differs)
        f = rng.choice(callfns); k = rng.choice(f["calls"]); L = list(lines)
        g = re.search(r"select_\d+", L[k].strip()).group(0)
        cands = [h for h in by_arity[(len(fns[g]["params"]), fns[g]["i"])] if h != g]
        if cands:
            h = rng.choice(cands); L[k] = L[k].replace(g + "(", h + "(", 1)
            res.append(out("redirect_same_rank", L, (f["name"], g, h)))
        # flip comparison operands
        f = rng.choice([f for f in fns.values() if f["cmp"] is not None]); L = list(lines); k = f["cmp"]
        L[k] = re.sub(r"if ([a-z][a-z0-9_]*) < ([a-z][a-z0-9_]*) \{", r"if \2 < \1 {", L[k])
        res.append(out("flip_cmp", L, (f["name"],)))
        # replace one compared operand
        f = rng.choice([f for f in fns.values() if f["cmp"] is not None and len(f["params"]) > 2]); L = list(lines); k = f["cmp"]
        m = re.search(r"if ([a-z][a-z0-9_]*) < ([a-z][a-z0-9_]*) \{", L[k]); x, y = m.groups()
        z = rng.choice([p for p in f["params"] if p not in (x, y)])
        L[k] = L[k][:m.start()] + (f"if {z} < {y} {{" if rng.random() < .5 else f"if {x} < {z} {{") + L[k][m.end():]
        res.append(out("replace_operand", L, (f["name"],)))
        # doc rank +-1
        f = rng.choice([f for f in fns.values() if len(f["params"]) > 1]); L = list(lines)
        d = rng.choice([-1, 1]); ni = f["i"] + d
        if 0 <= ni < len(f["params"]):
            L[f["doc"]] = re.sub(r"i = \d+", f"i = {ni}", L[f["doc"]], count=1)
            res.append(out("doc_rank", L, (f["name"], d)))
        # let tuple value changed
        lf = [f for f in fns.values() if f.get("let") and f["vals"]]
        if lf:
            f = rng.choice(lf); k = rng.choice(f["vals"]); L = list(lines); t = L[k].strip()
            vals = [v.strip() for v in t.strip("()").split(",")]; j = rng.randrange(len(vals))
            vals[j] = rng.choice([p for p in f["params"] if p != vals[j]] or [vals[j]])
            new = "(" + ", ".join(vals) + ")" if t.startswith("(") else vals[0]
            L[k] = L[k].replace(t, new); res.append(out("let_value", L, (f["name"],)))
        # extra comparison: wrap one call site in a new function comparing two args (cost +1 on that path)
        f = rng.choice(callfns); k = rng.choice(f["calls"]); L = list(lines); a = args_of(L[k])
        g = re.search(r"select_\d+", L[k].strip()).group(0)
        if len(a) >= 2:
            ps = [f"q{j}" for j in range(len(a))]; x, y = rng.sample(ps, 2); nm = "select_999999"
            gi = fns[g]["i"]
            newf = [f"/// n = {len(a)}, i = {gi}", f"fn {nm}([{', '.join(ps)}]: [usize; {len(a)}]) -> usize {{",
                    f"    if {x} < {y} {{", f"        {g}([{', '.join(ps)}])", "    } else {",
                    f"        {g}([{', '.join(ps)}])", "    }", "}"]
            L[k] = L[k].replace(g + "(", nm + "(", 1)
            L = L[:f["doc"]] + newf + L[f["doc"]:]
            res.append(out("extra_cmp", L, (f["name"], g)))
        # root rank / const I changed consistently by d (claims a different rank)
        L = list(lines); d = rng.choice([-1, 1])
        for j, s in enumerate(L):
            if s.startswith("pub const I"):
                m = re.search(r"(\d+);", s); L[j] = s.replace(m.group(0), f"{int(m.group(1)) + d};")
        root = fns["select_0"]
        L[root["doc"]] = re.sub(r"i = (\d+)", lambda mm: f"i = {int(mm.group(1)) + d}", L[root["doc"]], count=1)
        res.append(out("root_rank", L, (d,)))
    return res

def main():
    seed, per_op = int(sys.argv[1]), int(sys.argv[2])
    files = sys.argv[3:]
    rng = random.Random(seed)
    WD = collections.Counter()
    tab = collections.defaultdict(lambda: [0, 0, 0, 0, 0])   # total, v_accept, oracle_ok, both_ok, UNSOUND
    for path in files:
        text = open(path, encoding="ascii").read()
        W0 = V.verify_text(text)["worst_case"]
        for op, mt, info in mutants(text, rng, per_op):
            r = V.verify_text(mt)
            t = tab[op]; t[0] += 1
            if r["accepted"]:
                t[1] += 1
                ok, w, msg = oracle(mt, r["n"], r["i0"])
                WD[(op, r["worst_case"] - W0, (w - W0) if ok else None)] += 1
                if ok: t[3] += 1
                if not ok or w > r["worst_case"]:
                    t[4] += 1
                    print("UNSOUND?", path, op, info, r["n"], r["i0"], r["worst_case"], ok, w, msg)
                    open(f"unsound_{op}_{t[4]}.rs", "w").write(mt)
            else:
                ok, w, msg = oracle(mt, *( (r["n"], r["i0"]) if r["n"] is not None else (0, 0)))
                if ok: t[2] += 1
    print(f"{'operator':20s} total v_accept oracle_ok_but_v_reject accepted_and_correct UNSOUND")
    for op, t in tab.items(): print(f"{op:20s} {t[0]:6d} {t[1]:6d} {t[2]:6d} {t[3]:6d} {t[4]:6d}")
    print("accepted mutants: (op, verifier W - W0, exact W - W0): count", dict(WD))
main()
