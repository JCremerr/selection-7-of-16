"""Boundary drop mutants: at a call site, swap a passed argument p that is ONE
short of being certified (above count m-r-1 or below count r) with a dropped,
certified element e.  p becomes an uncertified drop -> must be REJECTed."""
import sys, re, random
import os; sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "verifier"))
sys.path.insert(0, ".")
import verify_rs_algorithm as V
from oracle import oracle, load, add, pc
path, maxm = sys.argv[1], int(sys.argv[2])
text = open(path).read(); lines = text.split("\n")
F, root = load(text)
# exact poset per function (each function has a unique poset in these files; recompute via DFS)
P = {}
def walk(f, Q):
    if f in P:
        assert P[f] == Q; return
    P[f] = Q
    ps, cmp, br = F[f]; idx = {p: k for k, p in enumerate(ps)}
    outs = [(Q, br[0])] if cmp is None else [(add(Q, idx[cmp[0]], idx[cmp[1]]), br[0]), (add(Q, idx[cmp[1]], idx[cmp[0]]), br[1])]
    for Qb, (st, bind) in outs:
        if st[0] == "call":
            args = [idx[bind.get(a, a)] for a in st[2]]; k = len(args)
            walk(st[1], tuple(sum(1 << l for l in range(k) if (Qb[args[j]] >> args[l]) & 1) for j in range(k)))
walk(root, tuple([0] * len(F[root][0])))
r_of = {}
for k, s in enumerate(lines):
    m = re.match(r"fn (select_\d+)", s)
    if m:
        d = re.search(r"i = (\d+)", lines[k - 1]); r_of[m.group(1)] = int(d.group(1)) if d else 0
cands = []
cur = None; br = 0
for k, s in enumerate(lines):
    m = re.match(r"fn (select_\d+)", s)
    if m: cur = m.group(1); br = 0; continue
    t = s.strip()
    if cur and re.fullmatch(r"select_\d+\(\[.*\]\)", t):
        ps, cmp, _ = F[cur]; idx = {p: q for q, p in enumerate(ps)}; mm = len(ps); r = r_of[cur]
        Q = P[cur]
        if cmp is not None:
            Q = add(Q, idx[cmp[0]], idx[cmp[1]]) if br == 0 else add(Q, idx[cmp[1]], idx[cmp[0]])
        br += 1
        args = [a.strip() for a in re.search(r"\[(.*)\]", t).group(1).split(",")]
        bl = [sum(1 << u for u in range(mm) if (Q[u] >> v) & 1) for v in range(mm)]
        dropped = [p for p in ps if p not in args]
        for j, p in enumerate(args):
            q = idx[p]
            if (pc(Q[q]) == mm - r - 1 or pc(bl[q]) == r) and dropped:
                cands.append((k, j, rng_e := dropped[0], cur, mm))
rng = random.Random(3); rng.shuffle(cands)
nrej = nwrong = 0; tot = 0
for (k, j, e, f, mm) in cands[:maxm]:
    L = list(lines); t = L[k].strip()
    args = [a.strip() for a in re.search(r"\[(.*)\]", t).group(1).split(",")]; args[j] = e
    L[k] = re.sub(r"\[(.*)\]", "[" + ", ".join(args) + "]", L[k]); mt = "\n".join(L)
    res = V.verify_text(mt); ok, w, msg = oracle(mt, res["n"] or 0, res["i0"] or 0)
    tot += 1; nrej += not res["accepted"]; nwrong += not ok
    if res["accepted"]: print("ACCEPTED boundary mutant", f, k, ok, msg)
print(f"{path.split('/')[-1]}: boundary candidates {len(cands)}, tested {tot}, rejected {nrej}, oracle-wrong {nwrong}")
