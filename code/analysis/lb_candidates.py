"""Candidate lower bounds LB(P, i) on the remaining number of comparisons for
selecting the i-th smallest (1-based) element of a poset P.

Interface: lb_name(n, i, above_masks) -> int, where above_masks[a] is the
bitmask of elements strictly greater than a (transitively closed).

Notation (see the paper):
  k = i-1 (size of the lower set L), u = n-i (size of the upper set U).
  A compatible solution s = (e, L): L a down-set of size k, e a minimal
  element of M = V \\ L (U = M \\ {e}).
  b(L)  = #Min(P|M) - 1        (depends only on L)
  a(A)  = #Max(P|A) - 1, A = L+e  (depends only on the cut A)
"""
import math
import os
import sys
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from posets import (down_masks, downsets_of_size, popcount, reverse)


def _ceil_log2(x):
    """ceil(log2 x) for a positive int / Fraction / float, exact for
    int and Fraction."""
    if x <= 1:
        return 0
    if isinstance(x, int):
        return (x - 1).bit_length()
    if isinstance(x, Fraction):
        # smallest h with 2^h >= x
        h = max(0, (x.numerator // x.denominator).bit_length() - 1)
        while Fraction(2) ** h < x:
            h += 1
        while h > 0 and Fraction(2) ** (h - 1) >= x:
            h -= 1
        return h
    return math.ceil(math.log2(x) - 1e-12)


def _min_count(M, down, n):
    """Number of minimal elements of P restricted to M."""
    c = 0
    m = M
    while m:
        x = (m & -m).bit_length() - 1
        m &= m - 1
        if down[x] & M == 0:
            c += 1
    return c


def _structure(n, i, up):
    """Return list of (L, e, a, b) for all compatible solutions."""
    down = down_masks(n, up)
    full = (1 << n) - 1
    sols = []
    for L in downsets_of_size(n, up, i - 1, down):
        M = full & ~L
        mins = [x for x in range(n) if M >> x & 1 and down[x] & M == 0]
        b = len(mins) - 1
        for e in mins:
            A = L | (1 << e)
            # maximal elements of P|A
            a = sum(1 for x in range(n) if A >> x & 1 and up[x] & A == 0) - 1
            sols.append((L, e, a, b))
    return sols


# --------------------------------------------------------------- baseline
def lb_baseline(n, i, up):
    """SEA 2025 Thm 6: ceil(log2 |C(P,i)|)."""
    return _ceil_log2(len(_structure(n, i, up)))


# ------------------------------------------------------- key comparisons
def lb_key(n, i, up):
    """max over compatible solutions of a_s + b_s (comparisons that every
    path to the leaf of solution s must still contain)."""
    return max(a + b for (_, _, a, b) in _structure(n, i, up))


# ---------------------------------------------- Fussenegger-Gabow, general P
def phi_fg(n, i, up):
    """sum over down-sets L of size i-1 of 2^{b(L)}."""
    down = down_masks(n, up)
    full = (1 << n) - 1
    tot = 0
    for L in downsets_of_size(n, up, i - 1, down):
        tot += 1 << (_min_count(full & ~L, down, n) - 1)
    return tot


def lb_fg(n, i, up):
    """max of FG on P and FG on the dual (reversed poset, rank n+1-i)."""
    return max(_ceil_log2(phi_fg(n, i, up)),
               _ceil_log2(phi_fg(n, n + 1 - i, reverse(n, up))))


# ------------------------------------- product weight (two-sided FG), test
def _g(a):
    return Fraction(1 << a, a + 1)


def phi_prod(n, i, up):
    """sum over solutions of g(a_s) g(b_s), g(a) = 2^a/(a+1)."""
    return sum(_g(a) * _g(b) for (_, _, a, b) in _structure(n, i, up))


def lb_prod(n, i, up):
    return _ceil_log2(phi_prod(n, i, up))


ALL = {
    "baseline": lb_baseline,
    "key": lb_key,
    "fg": lb_fg,
    "prod": lb_prod,
}


# ------------------------------------------------ deletion / adversary lemmas
def _fg_both(n, i, up):
    from posets import reduce_problem
    n, i, up = reduce_problem(n, i, up)
    if n == 1:
        return 0
    return lb_fg(n, i, up)


def lb_fg_iso(n, i, up, base=None):
    """Isolated-element lemma: if X_t, X_b are disjoint sets of isolated
    elements, |X_t| <= n-i, |X_b| <= i-1, then
        V_i(P) >= V_{i-|X_b|}(P - X_t - X_b) + |X_t| + |X_b|.
    Applied with base bound B = max(FG, FG-dual) on the remainder."""
    from posets import restrict, down_masks
    base = base or _fg_both
    down = down_masks(n, up)
    iso = [x for x in range(n) if up[x] == 0 and down[x] == 0]
    best = base(n, i, up)
    for t in range(0, min(len(iso), n - i) + 1):
        for bb in range(0, min(len(iso) - t, i - 1) + 1):
            if t + bb == 0:
                continue
            rem = iso[:t + bb]
            keep = ((1 << n) - 1)
            for x in rem:
                keep &= ~(1 << x)
            m, up2 = restrict(n, up, keep)
            best = max(best, base(m, i - bb, up2) + t + bb)
    return best


def lb_fg_del(n, i, up, base=None, _memo=None):
    """Extremal-deletion recursion (adversary declares a maximal element
    larger than all others / a minimal element smaller than all others):
        V_i(P) >= V_i(P - x) + [x isolated]      x maximal, i <= n-1
        V_i(P) >= V_{i-1}(P - x) + [x isolated]  x minimal, i >= 2
    R(P) = max(B(P), recursive terms).  Exponential; for testing only."""
    from posets import restrict, down_masks, reduce_problem
    base = base or _fg_both
    if _memo is None:
        _memo = {}
    key = (n, i, up)
    if key in _memo:
        return _memo[key]
    best = base(n, i, up)
    down = down_masks(n, up)
    full = (1 << n) - 1
    for x in range(n):
        iso = 1 if (up[x] == 0 and down[x] == 0) else 0
        if up[x] == 0 and i <= n - 1:
            m, up2 = restrict(n, up, full & ~(1 << x))
            best = max(best, lb_fg_del(m, i, up2, base, _memo) + iso)
        if down[x] == 0 and i >= 2:
            m, up2 = restrict(n, up, full & ~(1 << x))
            best = max(best, lb_fg_del(m, i - 1, up2, base, _memo) + iso)
    _memo[key] = best
    return best


# --------------------------------------- LP combination of world constraints
def phi_lp(n, i, up):
    """Min total leaves subject to: leaves with lower set L >= 2^{b(L)},
    leaves with cut A >= 2^{a(A)}, every solution >= 1 leaf.
    = |C| + max weight independent set {Y of L-worlds, Z of A-worlds, no
    solution joining Y and Z} with weights 2^b-(b+1), 2^a-(a+1);
    computed as total weight - min vertex cover via max-flow."""
    S = _structure(n, i, up)
    Lw, Aw = {}, {}
    edges = []
    for (L, e, a, b) in S:
        A = L | (1 << e)
        Lw[L] = (1 << b) - (b + 1)
        Aw[A] = (1 << a) - (a + 1)
        edges.append((L, A))
    tot = sum(Lw.values()) + sum(Aw.values())
    # max-flow: source -> L (cap r_L), L -> A (inf), A -> sink (cap q_A)
    flow = _maxflow_bip(Lw, Aw, edges)
    return len(S) + tot - flow


def _maxflow_bip(Lw, Aw, edges):
    """Simple Dinic-free augmenting max-flow for bipartite capacities."""
    import collections
    Lid = {L: j for j, L in enumerate(Lw)}
    Aid = {A: j for j, A in enumerate(Aw)}
    nL, nA = len(Lid), len(Aid)
    N = 2 + nL + nA
    s, t = 0, 1
    graph = [[] for _ in range(N)]
    cap = []
    to = []

    def add(u, v, c):
        graph[u].append(len(to)); to.append(v); cap.append(c)
        graph[v].append(len(to)); to.append(u); cap.append(0)
    INF = 1 << 60
    for L, j in Lid.items():
        if Lw[L] > 0:
            add(s, 2 + j, Lw[L])
    for A, j in Aid.items():
        if Aw[A] > 0:
            add(2 + nL + j, t, Aw[A])
    for (L, A) in edges:
        add(2 + Lid[L], 2 + nL + Aid[A], INF)
    flow = 0
    while True:
        # BFS levels (Dinic)
        level = [-1] * N
        level[s] = 0
        q = collections.deque([s])
        while q:
            u = q.popleft()
            for eid in graph[u]:
                if cap[eid] > 0 and level[to[eid]] < 0:
                    level[to[eid]] = level[u] + 1
                    q.append(to[eid])
        if level[t] < 0:
            return flow
        it = [0] * N

        def dfs(u, f):
            if u == t:
                return f
            while it[u] < len(graph[u]):
                eid = graph[u][it[u]]
                v = to[eid]
                if cap[eid] > 0 and level[v] == level[u] + 1:
                    d = dfs(v, min(f, cap[eid]))
                    if d > 0:
                        cap[eid] -= d
                        cap[eid ^ 1] += d
                        return d
                it[u] += 1
            return 0
        while True:
            f = dfs(s, INF)
            if f == 0:
                break
            flow += f


def lb_lp(n, i, up):
    return _ceil_log2(phi_lp(n, i, up))


ALL.update({
    "fg_iso": lb_fg_iso,
    "fg_del": lb_fg_del,
    "lp": lb_lp,
})


def lb_fg_d2(n, i, up, base=None, max_size=None):
    """Lemma D2 (adversary reveals a bottom down-set X and a top up-set Y):
        V_i(P) >= V_{i-|X|}(P - X - Y) + |X & Max(P)| + |Y & Min(P)|
    for disjoint X (down-set, |X| <= i-1) and Y (up-set, |Y| <= n-i).
    Base bound B = max(FG, FG-dual) on the core.  Enumerates all X, Y
    (optionally only |X|,|Y| <= max_size)."""
    from posets import restrict, down_masks, all_downsets
    base = base or _fg_both
    down = down_masks(n, up)
    full = (1 << n) - 1
    maxP = sum(1 << x for x in range(n) if up[x] == 0)
    minP = sum(1 << x for x in range(n) if down[x] == 0)
    best = base(n, i, up)
    Xs = [X for X in all_downsets(n, up, down) if popcount(X) <= i - 1]
    Ys = [full & ~D for D in all_downsets(n, up, down) if popcount(full & ~D) <= n - i]
    if max_size is not None:
        Xs = [X for X in Xs if popcount(X) <= max_size]
        Ys = [Y for Y in Ys if popcount(Y) <= max_size]
    for X in Xs:
        bx = popcount(X & maxP)
        for Y in Ys:
            if X & Y:
                continue
            by = popcount(Y & minP)
            if bx + by == 0:
                continue
            m, up2 = restrict(n, up, full & ~X & ~Y)
            best = max(best, base(m, i - popcount(X), up2) + bx + by)
    return best


ALL.update({"fg_d2": lb_fg_d2})


# ------------------------------------------------ two-sided additive (NEW)
def phi_fgsum(n, i, up):
    """Phi* = FG + FGdual - |C| = sum_s [g(a_s) + g(b_s) - 1]
    (g(x) = 2^x/(x+1)).  Proven sum-superadditive (paper, Theorem "Phi*")."""
    return phi_fg(n, i, up) + phi_fg(n, n + 1 - i, reverse(n, up)) - len(_structure(n, i, up))


def lb_fgsum(n, i, up):
    return _ceil_log2(phi_fgsum(n, i, up))


ALL.update({"fgsum": lb_fgsum})


def _fgsum_red(n, i, up):
    from posets import reduce_problem
    n, i, up = reduce_problem(n, i, up)
    if n == 1:
        return 0
    return lb_fgsum(n, i, up)


def lb_fgsum_d2(n, i, up):
    """Lemma D2 with base bound Phi* (fgsum)."""
    return lb_fg_d2(n, i, up, base=_fgsum_red)


def lb_best(n, i, up):
    """max of the proven cheap bounds: fgsum, key."""
    return max(lb_fgsum(n, i, up), lb_key(n, i, up))


ALL.update({"fgsum_d2": lb_fgsum_d2, "best": lb_best})


# ======================================================================
# FAST single-pass evaluation of all proven bounds (reference for Rust).
# One DFS over down-sets L of size k = i-1 (in a linear-extension order).
# For each L:  M = V\L, minM = {x in M : down[x] subset of L},
#              maxL = {x in L : up[x] & L == 0},  b = |minM| - 1.
#   |C|   += b+1
#   FG    += 2^b
#   for e in minM:  a = |maxL \ down[e]|   (a_s = |Max(P|L+e)| - 1)
#       key  = max(key, a+b)
#       FGd += 2^a  iff e is the highest-index element of Max(P|L+e)
#              (counts every down-set A of size i exactly once)
# Phi* = FG + FGd - |C|.
# ======================================================================
def bounds_fast(n, i, up):
    down = [0] * n
    for a in range(n):
        m = up[a]
        while m:
            b = (m & -m).bit_length() - 1
            m &= m - 1
            down[b] |= 1 << a
    order = sorted(range(n), key=lambda x: popcount(down[x]))
    k = i - 1
    full = (1 << n) - 1
    C = FG = FGd = 0
    key = 0
    stack = [(0, 0, 0)]          # (position in order, L mask, |L|)
    while stack:
        j, L, sz = stack.pop()
        if sz == k:
            M = full & ~L
            minM = 0
            mm = M
            while mm:
                x = (mm & -mm).bit_length() - 1
                mm &= mm - 1
                if down[x] & M == 0:
                    minM |= 1 << x
            maxL = 0
            ll = L
            while ll:
                x = (ll & -ll).bit_length() - 1
                ll &= ll - 1
                if up[x] & L == 0:
                    maxL |= 1 << x
            b = popcount(minM) - 1
            C += b + 1
            FG += 1 << b
            mm = minM
            while mm:
                e = (mm & -mm).bit_length() - 1
                mm &= mm - 1
                rest = maxL & ~down[e]
                a = popcount(rest)
                if a + b > key:
                    key = a + b
                if rest >> e == 0:      # e has the highest index in Max(A)
                    FGd += 1 << a
            continue
        if n - j < k - sz:
            continue
        x = order[j]
        stack.append((j + 1, L, sz))
        if down[x] & ~L == 0:
            stack.append((j + 1, L | (1 << x), sz + 1))
    phistar = FG + FGd - C
    return {
        "C": C, "FG": FG, "FGdual": FGd, "phistar": phistar, "key": key,
        "lb_baseline": _ceil_log2(C),
        "lb_fg": max(_ceil_log2(FG), _ceil_log2(FGd)),
        "lb_fgsum": _ceil_log2(phistar),
        "lb_key": key,
        "lb_best": max(_ceil_log2(phistar), key),
    }


def lb_fast_best(n, i, up):
    return bounds_fast(n, i, up)["lb_best"]


ALL.update({"fast_best": lb_fast_best})


# Proof status of every entry in ALL (proofs in the paper).  Only PROVEN
# bounds may be used for pruning in a lower-bound / refutation search.
STATUS = {
    "baseline": "PROVEN (SEA 2025 Thm 6)",
    "key": "PROVEN (Thm 2)",
    "fg": "PROVEN (Thm 3, Fussenegger-Gabow generalised to arbitrary P)",
    "fgsum": "PROVEN (Thm 4, Phi* = FG + FGdual - |C|, new)",
    "best": "PROVEN (Thm 5 = max(fgsum, key))  <- RECOMMENDED",
    "fast_best": "PROVEN (same as best, single-pass reference for Rust)",
    "lp": "PROVEN (Thm 6), weak, not recommended",
    "fg_iso": "PROVEN (Thm 7 special case), no measured gain",
    "fg_del": "PROVEN (Thm 7 special case), exponential, testing only",
    "fg_d2": "PROVEN (Thm 7), exponential, testing only",
    "fgsum_d2": "PROVEN (Thm 7 with Phi* base), exponential, testing only",
    "prod": "CONJECTURE C1 -- NOT locally valid, NOT a leaf bound; never prune with it",
}
