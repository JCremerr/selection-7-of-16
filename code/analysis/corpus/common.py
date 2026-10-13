"""Shared helpers: world/solution histograms, exact V and ML (min leaves).

Conventions as in ../posets.py: a problem is (n, i, up), i 1-based, up[x] =
bitmask of elements strictly above x (transitively closed).

sol_hist(n, i, up) -> Counter {(a_s, b_s): number of compatible solutions}.
  From it:  #L-worlds with b(L) = b  is  (sum_a cnt[a, b]) / (b+1),
            #A-worlds with a(A) = a  is  (sum_b cnt[a, b]) / (a+1),
  because every L-world with b(L) = b carries exactly b+1 solutions, all with
  b_s = b (dually for A).  So every potential of the families studied here is
  a linear function of this histogram.
"""
import os
import sys
from collections import Counter
from fractions import Fraction
from functools import lru_cache

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
from posets import (add_relation, canon_problem, down_masks, downsets_of_size,  # noqa: E402
                    incomparable, is_solved, popcount, all_posets_unlabeled)


def sol_hist(n, i, up):
    down = down_masks(n, up)
    full = (1 << n) - 1
    cnt = Counter()
    for L in downsets_of_size(n, up, i - 1, down):
        M = full & ~L
        minM = [x for x in range(n) if M >> x & 1 and down[x] & M == 0]
        maxL = 0
        for x in range(n):
            if L >> x & 1 and up[x] & L == 0:
                maxL |= 1 << x
        b = len(minM) - 1
        for e in minM:
            a = popcount(maxL & ~down[e])        # a_s = |Max(P|L+e)| - 1
            cnt[(a, b)] += 1
    return cnt


def worlds(cnt):
    """(Lw, Aw, C): Counters b -> #L-worlds, a -> #A-worlds, and |C|."""
    Lw, Aw = Counter(), Counter()
    C = 0
    for (a, b), c in cnt.items():
        Lw[b] += c
        Aw[a] += c
        C += c
    for b in list(Lw):
        assert Lw[b] % (b + 1) == 0
        Lw[b] //= (b + 1)
    for a in list(Aw):
        assert Aw[a] % (a + 1) == 0
        Aw[a] //= (a + 1)
    return Lw, Aw, C


def Fstar(b):
    return (1 << b) - b - 1


def psi(cnt, F, G):
    """Psi_{F,G} = |C| + sum_L F(b(L)) + sum_A G(a(A))."""
    Lw, Aw, C = worlds(cnt)
    return C + sum(c * F(b) for b, c in Lw.items()) + sum(c * G(a) for a, c in Aw.items())


def phistar(cnt):
    return psi(cnt, Fstar, Fstar)


def g(x):
    return Fraction(1 << x, x + 1)


def prod(cnt):
    return sum(c * g(a) * g(b) for (a, b), c in cnt.items())


def ceil_log2(x):
    """smallest h >= 0 with 2^h >= x (x int or Fraction)."""
    h = 0
    while Fraction(2) ** h < x:
        h += 1
    return h


def antichain(n):
    return tuple([0] * n)


# ---------------------------------------------------------------- exact solvers
class Exact:
    """Exact V_i(P) and ML_i(P) (minimum number of leaves of a correct decision
    tree) by memoised minimax over ALL incomparable pairs, WITHOUT reduction
    (leaf = solved poset), memoised on the canonical form with duality.
    Duality preserves both V and ML (reverse every comparison).
    Intended for n <= 7."""

    def __init__(self):
        self.V = {}
        self.ML = {}

    def _children(self, n, i, up):
        seen = set()
        for x in range(n):
            for y in range(x + 1, n):
                if not incomparable(up, x, y):
                    continue
                p1 = add_relation(n, up, x, y)
                p2 = add_relation(n, up, y, x)
                k1 = canon_problem(n, i, p1)
                k2 = canon_problem(n, i, p2)
                key = (min(k1, k2), max(k1, k2))
                if key in seen:
                    continue
                seen.add(key)
                yield p1, p2

    def value(self, n, i, up):
        if is_solved(n, i, up):
            return 0, 1
        key = canon_problem(n, i, up)
        if key in self.V:
            return self.V[key], self.ML[key]
        bv, bm = 10 ** 9, 10 ** 18
        for p1, p2 in self._children(n, i, up):
            v1, m1 = self.value(n, i, p1)
            v2, m2 = self.value(n, i, p2)
            bv = min(bv, 1 + max(v1, v2))
            bm = min(bm, m1 + m2)
        self.V[key], self.ML[key] = bv, bm
        return bv, bm


def all_problems(nmax, nmin=1):
    ps = all_posets_unlabeled(nmax)
    for n in range(nmin, nmax + 1):
        for up in ps[n]:
            for i in range(1, n + 1):
                yield n, i, up


def local_triples(nmax, nmin=1):
    """Yield (n, i, up, x, y, up1, up2) for all unlabelled posets n <= nmax,
    all ranks i, all incomparable pairs x < y (labels), unsolved P only."""
    for n, i, up in all_problems(nmax, nmin):
        if is_solved(n, i, up):
            continue
        for x in range(n):
            for y in range(x + 1, n):
                if incomparable(up, x, y):
                    yield n, i, up, x, y, add_relation(n, up, x, y), add_relation(n, up, y, x)
