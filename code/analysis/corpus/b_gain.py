"""what would C1 buy if it were proved?  Over the ground-truth
table (n >= 2): LB = max(ceil log2 Phi*, KEY) (proved) versus
LB1 = max(ceil log2 W, KEY) (W = product, conjectural).  Exact counts and
average gaps V - bound, overall and for n = 12.

  python3 -I b_gain.py TABLE
"""
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import ceil_log2, phistar, prod, sol_hist  # noqa: E402

rows, ex0, ex1, gap0, gap1, better = Counter(), Counter(), Counter(), Counter(), Counter(), Counter()
with open(sys.argv[1]) as f:
    for line in f:
        p = line.split()
        n, i, v = int(p[0]), int(p[1]), int(p[2])
        if n < 2:
            continue
        up = tuple(int(h, 16) for h in p[3].split(","))
        c = sol_hist(n, i, up)
        key = max(a + b for a, b in c)
        lb0 = max(ceil_log2(phistar(c)), key)
        lb1 = max(ceil_log2(prod(c)), key)
        for k in ("all", n):
            rows[k] += 1
            ex0[k] += lb0 == v
            ex1[k] += lb1 == v
            gap0[k] += v - lb0
            gap1[k] += v - lb1
            better[k] += lb1 > lb0
for k in ["all"] + sorted(x for x in rows if x != "all"):
    r = rows[k]
    print(f"n={k}: rows {r}; LB exact {ex0[k]} ({100 * ex0[k] / r:.1f}%), avg gap {gap0[k] / r:.3f}; "
          f"max(C1,KEY) exact {ex1[k]} ({100 * ex1[k] / r:.1f}%), avg gap {gap1[k] / r:.3f}; "
          f"C1 raises LB on {better[k]} rows")
