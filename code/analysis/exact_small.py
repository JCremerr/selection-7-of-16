"""Exact brute-force solver V_i(P) for small posets (n <= 8), memoised on
canonical form (with duality).  Uses NO lower bound for pruning, only exact
alpha-style cut-offs, so the values are independent of every candidate bound.

V_i(P) = 0 if (P,i) solved, else 1 + min_{a||b} max(V(P+a<b), V(P+b<a)),
evaluated on reduced problems (SEA Obs. 3).  Optionally without reduction
(reduce=False) to cross-check that reduction preserves the value.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from posets import (add_relation, canon_problem, down_masks, incomparable,
                    is_solved, reduce_problem, canon, all_posets_unlabeled)

sys.setrecursionlimit(10000)


class Solver:
    def __init__(self, use_reduction=True):
        self.memo = {}
        self.red = use_reduction

    def value(self, n, i, up):
        if self.red:
            n, i, up = reduce_problem(n, i, up)
            if n == 1:
                return 0
        else:
            if is_solved(n, i, up):
                return 0
        key = canon_problem(n, i, up)
        v = self.memo.get(key)
        if v is not None:
            return v
        # canonical representative for the recursion
        kn, ki, kup = key
        best = 10 ** 9
        seen = set()
        for a in range(kn):
            for b in range(a + 1, kn):
                if not incomparable(kup, a, b):
                    continue
                c1 = add_relation(kn, kup, a, b)
                c2 = add_relation(kn, kup, b, a)
                k1 = canon_problem(*self._norm(kn, ki, c1))
                k2 = canon_problem(*self._norm(kn, ki, c2))
                pk = (min(k1, k2), max(k1, k2))
                if pk in seen:
                    continue
                seen.add(pk)
                v1 = self.value(kn, ki, c1)
                if 1 + v1 >= best:
                    continue
                v2 = self.value(kn, ki, c2)
                v = 1 + max(v1, v2)
                if v < best:
                    best = v
        self.memo[key] = best
        return best

    def _norm(self, n, i, up):
        if self.red:
            n2, i2, up2 = reduce_problem(n, i, up)
            if n2 == 1:
                return (1, 1, (0,))
            return n2, i2, up2
        return n, i, up


def empty(n):
    return tuple([0] * n)


if __name__ == "__main__":
    import time
    s = Solver()
    t = time.time()
    for n in range(1, 9):
        row = [s.value(n, i, empty(n)) for i in range(1, n + 1)]
        print(n, row, len(s.memo), round(time.time() - t, 1), flush=True)
