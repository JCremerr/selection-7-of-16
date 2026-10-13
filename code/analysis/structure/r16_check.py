"""Search-free test of the runner-up-tournament opening R16 for (16, i).

R16 = four 4-trees T4 (w<b, w<r, r<d) plus the runner-up knockout
r_A<r_B, r_A<r_C, r_C<r_D (15 comparisons, every outcome isomorphic).
For each rank i = 1..8 (i > 8 by duality): reduce (R16, i) (SEA Obs. 3),
compute the proven lower bounds ceil(log2|C|) and LB_best (paper, Corollary "LB";
code/analysis/lb_candidates.py), and compare 15 + LB with V_i(16).
If 15 + LB_best > V_i(16), the opening R16 cannot be part of an optimal
algorithm for (16, i) (REFUTED).  Otherwise it is only "not refuted".
V_i(16), i <= 6: worst case of the upstream optimal algorithm
logs/backward/algorithms/16_{i-1}.rs, read with our own parser (cert.py).
i = 7: 28 (our certificate + SEA lower bound); i = 8: in [28, 29].

Usage: python3 -I r16_check.py
"""

import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, ".."))
from posets import add_relation, reduce_problem      # noqa: E402
from lb_candidates import lb_best, lb_baseline     # noqa: E402
from cert import Program                           # noqa: E402

SEL = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
UP = os.path.join(SEL, "external", "selection_generator", "logs", "backward", "algorithms")


def r16():
    names = [t + x for x in "ABCD" for t in "wbrd"]
    ix = {s: k for k, s in enumerate(names)}
    rel = []
    for x in "ABCD":
        rel += [("w" + x, "b" + x), ("w" + x, "r" + x), ("r" + x, "d" + x)]
    rel += [("rA", "rB"), ("rA", "rC"), ("rC", "rD")]
    up = tuple([0] * 16)
    for a, b in rel:
        up = add_relation(16, up, ix[a], ix[b])
    return up


def main():
    up = r16()
    V = {}
    for i in range(1, 7):
        f = os.path.join(UP, "16_%d.rs" % (i - 1))
        p = Program(f)
        V[i] = (p.W[p.root],) * 2
    V[7] = (28, 28)
    V[8] = (28, 29)
    print("# i  V_i(16)  reduced(n,i)  ceil(log2|C|)  LB_best  15+LB_best  verdict")
    for i in range(1, 9):
        n2, i2, up2 = reduce_problem(16, i, up)
        lbc = lb_baseline(n2, i2, up2)
        lbb = lb_best(n2, i2, up2)
        lo, hi = V[i]
        if 15 + lbb > hi:
            verdict = "REFUTED (15+LB > V)"
        elif 15 + lbb > lo:
            verdict = "refuted unless V_i(16) = %d" % hi
        else:
            verdict = "not refuted (slack %d for the endgame)" % (lo - 15 - lbb)
        vs = "%d" % lo if lo == hi else "%d-%d" % (lo, hi)
        print("  %d  %-7s  (%2d,%d)  %13d  %7d  %10d  %s" % (i, vs, n2, i2, lbc, lbb, 15 + lbb, verdict))


if __name__ == "__main__":
    main()
