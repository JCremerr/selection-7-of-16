"""Does a program contain a subproblem isomorphic to reduce(R16, i), i = the
program's rank?  Checks the function preconditions P_f (closure of what all
call sites know), so it can miss a path whose knowledge is R16 but whose
function also serves weaker call sites.  Positive controls: V7main, V7alt,
V8main (traced to R16 in report.txt). 
Usage: python3 -I r16_reach.py"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, ".."))
from cert import Program
from analyze import FILES
from shapes import canon
from posets import reduce_problem
from r16_check import r16
for key, path in FILES:
    if not os.path.exists(path):
        continue
    p = Program(path)
    n0, r0 = len(p.P[p.root]), p.funcs[p.root].r          # r0 is 0-based
    if n0 != 16:
        print("%-7s n = %d, R16 impossible" % (key, n0)); continue
    m, i, up = reduce_problem(16, r0 + 1, r16())
    target = canon(up)
    hits = [(f, p.dmin[f]) for f in p.funcs
            if len(p.P[f]) == m and p.funcs[f].r == i - 1 and canon(p.P[f]) == target]
    print("%-7s i=%d reduce(R16) = (%d,%d): %d function(s) %s" % (key, r0 + 1, m, i, len(hits), hits[:3]))
