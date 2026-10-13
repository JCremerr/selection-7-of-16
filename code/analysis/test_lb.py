"""Validity and strength tests for the candidate lower bounds.

  python3 -I test_lb.py values NMAX        # LB <= V on all problems, n <= NMAX
  python3 -I test_lb.py local NMAX NAME    # local potential inequality check
"""
import os
import sys
import time
from collections import defaultdict
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from posets import (add_relation, all_posets_unlabeled, incomparable,
                    is_solved, reduce_problem, canon_problem)
from exact_small import Solver
import lb_candidates as LBC


def all_problems(nmax):
    """All (n, i, up) with up an unlabelled poset on n <= nmax elements."""
    ps = all_posets_unlabeled(nmax)
    for n in range(1, nmax + 1):
        for up in ps[n]:
            for i in range(1, n + 1):
                yield n, i, up


def reduced_problem_set(nmax):
    """Distinct canonical reduced unsolved problems from all posets n<=nmax."""
    seen = {}
    for n, i, up in all_problems(nmax):
        rn, ri, rup = reduce_problem(n, i, up)
        if rn == 1:
            continue
        key = canon_problem(rn, ri, rup)
        if key not in seen:
            seen[key] = (rn, ri, rup)
    return list(seen.values())


def test_values(nmax, names=None, solver=None):
    names = names or list(LBC.ALL)
    solver = solver or Solver()
    probs = reduced_problem_set(nmax)
    stats = {nm: defaultdict(int) for nm in names}
    viol = {nm: [] for nm in names}
    gaps = {nm: [] for nm in names}
    t = time.time()
    for (n, i, up) in probs:
        v = solver.value(n, i, up)
        for nm in names:
            lb = LBC.ALL[nm](n, i, up)
            if lb > v:
                viol[nm].append((n, i, up, v, lb))
            gaps[nm].append(v - lb)
    print(f"{len(probs)} reduced problems with n<={nmax}  ({time.time()-t:.1f}s)")
    print(f"{'bound':12s} {'viol':>5s} {'avg gap':>8s} {'max gap':>8s} {'exact%':>7s}")
    for nm in names:
        g = gaps[nm]
        print(f"{nm:12s} {len(viol[nm]):5d} {sum(g)/len(g):8.3f} {max(g):8d} "
              f"{100*sum(1 for x in g if x == 0)/len(g):7.1f}")
    return viol, gaps


def check_local(nmax, phi, form="sum", verbose=3):
    """Check phi(P) <= phi(P+ab) + phi(P+ba)  (form='sum') or
    phi(P) <= 2 max(...) (form='max') for ALL posets n<=nmax, all i, all
    incomparable pairs, and phi(solved) <= 1.  No reduction is applied."""
    bad = 0
    tot = 0
    worst = None
    for n, i, up in all_problems(nmax):
        p = phi(n, i, up)
        if is_solved(n, i, up):
            if p > 1:
                bad += 1
                if verbose:
                    print("solved with phi>1", n, i, up, p)
                    verbose -= 1
            continue
        for a in range(n):
            for b in range(a + 1, n):
                if not incomparable(up, a, b):
                    continue
                tot += 1
                p1 = phi(n, i, add_relation(n, up, a, b))
                p2 = phi(n, i, add_relation(n, up, b, a))
                rhs = (p1 + p2) if form == "sum" else 2 * max(p1, p2)
                ratio = Fraction(rhs) / Fraction(p) if p else None
                if ratio is not None and (worst is None or ratio < worst[0]):
                    worst = (ratio, n, i, up, a, b, p, p1, p2)
                if rhs < p:
                    bad += 1
                    if verbose:
                        print("VIOLATION", n, i, up, (a, b), float(p), float(p1), float(p2))
                        verbose -= 1
    print(f"checked {tot} (P,i,ab) triples, n<={nmax}: {bad} violations; "
          f"min rhs/phi = {float(worst[0]):.4f} at {worst[1:6]}")
    return bad, worst


if __name__ == "__main__":
    mode = sys.argv[1]
    nmax = int(sys.argv[2])
    if mode == "values":
        names = sys.argv[3].split(",") if len(sys.argv) > 3 else None
        test_values(nmax, names)
    elif mode == "local":
        nm = sys.argv[3]
        form = sys.argv[4] if len(sys.argv) > 4 else "sum"
        check_local(nmax, getattr(LBC, "phi_" + nm), form)
