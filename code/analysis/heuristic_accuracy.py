"""how good are move-ordering heuristics at picking an OPTIMAL move?

Ground truth: data/groundtruth_table.tsv (exact V for reached reduced problems).
For sampled unsolved problems with n in [NMIN, NMAX], every incomparable pair
is labelled optimal iff max(V(child1), V(child2)) == V(P) - 1 (children
looked up by canonical form; problems with a missing child are skipped).
For each heuristic key (smaller = tried first) we report
  top1  : P(a uniformly random move among the best-key ties is optimal)
  rank  : expected rank of the first optimal move (ties broken uniformly)
  base  : fraction of optimal moves among all moves (random ordering)
Usage: python3 -I heuristic_accuracy.py TABLE NMIN NMAX SAMPLES_PER_N
"""
import os, sys, random
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from posets import canon_problem, add_relation, reduce_problem, is_solved, incomparable
from lb_candidates import bounds_fast

def load(path, nmax):
    rows = []
    with open(path) as f:
        for line in f:
            p = line.split('\t')
            n = int(p[0])
            if n > nmax: continue
            rows.append((n, int(p[1]), int(p[2]), [int(h, 16) for h in p[3].split(',')]))
    return rows

def stats(n, i, up):
    if is_solved(n, i, up):
        return {"C": 1, "phistar": 1, "lb_best": 0, "key": 0}
    return bounds_fast(n, i, up)

def look1(n, i, q):
    """1-ply lookahead lower bound of the reduced child problem (valid LB):
    0 if solved, else 1 + min over pairs of max(LB_best of grandchildren)."""
    rn, ri, rup = reduce_problem(n, i, q)
    if is_solved(rn, ri, rup): return 0
    best = None
    for a in range(rn):
        for b in range(a + 1, rn):
            if not incomparable(rup, a, b): continue
            m = max(stats(rn, ri, add_relation(rn, rup, a, b))["lb_best"],
                    stats(rn, ri, add_relation(rn, rup, b, a))["lb_best"])
            if best is None or m < best: best = m
            if best is not None and best <= stats(rn, ri, rup)["lb_best"] - 1: break
    return 1 + best

def exp_rank(keys, opt):
    """expected rank of first optimal move when sorting by key, ties uniform."""
    items = sorted(zip(keys, opt))
    groups = {}
    for k, o in items: groups.setdefault(k, []).append(o)
    pos = 0
    for k in sorted(groups):
        g = groups[k]; m = len(g); good = sum(g)
        if good:
            # expected position of first good among m shuffled with `good` goods
            return pos + (m + 1) / (good + 1)
        pos += m
    return float('nan')

LOOK = os.environ.get("LOOK") == "1"

def main():
    path, nmin, nmax, per = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    rows = load(path, nmax)
    V = {canon_problem(n, i, up): v for n, i, v, up in rows}
    rnd = random.Random(7)
    H = {
      "cs":   lambda s1, s2: (max(s1["C"], s2["C"]), min(s1["C"], s2["C"])),
      "phi":  lambda s1, s2: (max(s1["phistar"], s2["phistar"]), min(s1["phistar"], s2["phistar"])),
      "lb":   lambda s1, s2: (max(s1["lb_best"], s2["lb_best"]), max(s1["phistar"], s2["phistar"])),
      "lbsum":lambda s1, s2: (max(s1["lb_best"], s2["lb_best"]), s1["phistar"] + s2["phistar"]),
      "phisum":lambda s1, s2: (s1["phistar"] + s2["phistar"],),
    }
    if LOOK:
        H["look1"] = lambda s1, s2: (max(s1["look1"], s2["look1"]), max(s1["phistar"], s2["phistar"]))
    for n in range(nmin, nmax + 1):
        cand = [r for r in rows if r[0] == n and r[2] > 0]
        rnd.shuffle(cand)
        acc = {h: [0.0, 0.0, 0] for h in H}; base = 0.0; used = 0; skipped = 0
        for (n_, i, v, up) in cand:
            if used >= per: break
            moves = []
            ok = True
            for a in range(n):
                for b in range(a + 1, n):
                    if not incomparable(up, a, b): continue
                    ch = []
                    for (x, y) in ((a, b), (b, a)):
                        q = add_relation(n, up, x, y)
                        rn, ri, rup = reduce_problem(n, i, q)
                        if is_solved(rn, ri, rup): cv = 0
                        else:
                            cv = V.get(canon_problem(rn, ri, rup))
                            if cv is None: ok = False; break
                        st = dict(stats(n, i, q)); st["look1"] = look1(n, i, q) if LOOK else 0
                        ch.append((cv, st))
                    if not ok: break
                    moves.append((max(ch[0][0], ch[1][0]) == v - 1, ch[0][1], ch[1][1]))
                if not ok: break
            if not ok or not moves: skipped += 1; continue
            used += 1
            opt = [m[0] for m in moves]
            base += sum(opt) / len(opt)
            for h, f in H.items():
                keys = [f(m[1], m[2]) for m in moves]
                best = min(keys); tie = [o for k, o in zip(keys, opt) if k == best]
                acc[h][0] += sum(tie) / len(tie)
                acc[h][1] += exp_rank(keys, opt)
        print(f"n={n}: {used} problems ({skipped} skipped: child not in table); random-move optimal rate {base/max(used,1):.3f}")
        for h in H:
            print(f"   {h:7s} top1 optimal {acc[h][0]/max(used,1):.3f}   expected rank of first optimal move {acc[h][1]/max(used,1):.2f}")
        sys.stdout.flush()

main()
