"""Basic poset utilities for the selection problem (analytic track).

A selection problem is (n, i, up) with
  n  = number of elements 0..n-1,
  i  = rank sought, 1-based (find the i-th smallest),
  up = tuple of n bitmasks, up[a] = set of elements strictly greater than a
       (the relation must be transitively closed and irreflexive).

Written from the definitions only (no upstream code).
"""
from functools import lru_cache

popcount = int.bit_count


def down_masks(n, up):
    down = [0] * n
    for a in range(n):
        m = up[a]
        while m:
            b = (m & -m).bit_length() - 1
            m &= m - 1
            down[b] |= 1 << a
    return down


def is_closed(n, up):
    for a in range(n):
        if up[a] >> a & 1:
            return False
        m = up[a]
        while m:
            b = (m & -m).bit_length() - 1
            m &= m - 1
            if up[b] & ~up[a]:
                return False
            if up[b] >> a & 1:
                return False
    return True


def add_relation(n, up, a, b):
    """Return closure of up + (a < b). Requires a, b incomparable."""
    down = down_masks(n, up)
    new_up = list(up)
    add = up[b] | (1 << b)
    m = down[a] | (1 << a)
    while m:
        x = (m & -m).bit_length() - 1
        m &= m - 1
        new_up[x] |= add
    return tuple(new_up)


def incomparable(up, a, b):
    return not (up[a] >> b & 1) and not (up[b] >> a & 1)


def restrict(n, up, keep_mask):
    """Restrict poset to the elements in keep_mask, relabelled 0..k-1 in order."""
    idx = [x for x in range(n) if keep_mask >> x & 1]
    pos = {x: j for j, x in enumerate(idx)}
    new_up = []
    for x in idx:
        m = up[x] & keep_mask
        nm = 0
        while m:
            b = (m & -m).bit_length() - 1
            m &= m - 1
            nm |= 1 << pos[b]
        new_up.append(nm)
    return len(idx), tuple(new_up)


def reduce_problem(n, i, up):
    """SEA Obs. 3 reduction: delete elements that cannot be the answer.
    x with >= i elements below is above the answer (delete, i unchanged);
    x with >= n-i+1 elements above is below the answer (delete, i -= 1).
    Repeat until stable. Returns (n, i, up)."""
    while True:
        down = down_masks(n, up)
        keep = (1 << n) - 1
        di = 0
        for x in range(n):
            if popcount(down[x]) >= i:
                keep &= ~(1 << x)
            elif popcount(up[x]) >= n - i + 1:
                keep &= ~(1 << x)
                di += 1
        if keep == (1 << n) - 1:
            return n, i, tuple(up)
        n, up = restrict(n, up, keep)
        i -= di


def is_solved(n, i, up):
    down = down_masks(n, up)
    return any(popcount(down[x]) == i - 1 and popcount(up[x]) == n - i for x in range(n))


def reverse(n, up):
    return tuple(down_masks(n, up))


# ---------------------------------------------------------------- canonical form
def _refine(n, up, down, colors):
    """Colour refinement; colors = list of ints. Returns stable refined colours
    (ranked canonically)."""
    k = len(set(colors))
    while True:
        sigs = []
        for x in range(n):
            ua = sorted(colors[y] for y in range(n) if up[x] >> y & 1)
            da = sorted(colors[y] for y in range(n) if down[x] >> y & 1)
            sigs.append((colors[x], tuple(ua), tuple(da)))
        ranks = {s: r for r, s in enumerate(sorted(set(sigs)))}
        colors = [ranks[s] for s in sigs]
        k2 = len(ranks)
        if k2 == k:
            return colors
        k = k2


def _encode(n, up, colors):
    order = sorted(range(n), key=lambda x: colors[x])
    pos = [0] * n
    for j, x in enumerate(order):
        pos[x] = j
    enc = []
    for x in order:
        m = up[x]
        nm = 0
        while m:
            b = (m & -m).bit_length() - 1
            m &= m - 1
            nm |= 1 << pos[b]
        enc.append(nm)
    return tuple(enc)


def canon(n, up):
    """Exact canonical form (individualisation-refinement). Returns tuple of
    up-masks of a canonical relabelling."""
    down = down_masks(n, up)
    init = [(popcount(down[x]), popcount(up[x])) for x in range(n)]
    r = {s: j for j, s in enumerate(sorted(set(init)))}
    colors = _refine(n, up, down, [r[s] for s in init])
    best = [None]

    def rec(colors):
        cells = {}
        for x in range(n):
            cells.setdefault(colors[x], []).append(x)
        target = None
        for c in sorted(cells):
            if len(cells[c]) > 1:
                target = cells[c]
                break
        if target is None:
            e = _encode(n, up, colors)
            if best[0] is None or e < best[0]:
                best[0] = e
            return
        # twins (same up and same down sets) are interchangeable: branch once
        x0 = target[0]
        if all(up[x] == up[x0] and down[x] == down[x0] for x in target):
            cands = [x0]
        else:
            cands = target
        for v in cands:
            c2 = [2 * c + (0 if x == v else 1) if colors[x] == colors[v] else 2 * c
                  for x, c in enumerate(colors)]
            # re-rank
            rr = {s: j for j, s in enumerate(sorted(set(c2)))}
            c2 = [rr[s] for s in c2]
            rec(_refine(n, up, down, c2))

    rec(colors)
    return best[0]


def canon_problem(n, i, up):
    """Canonical key of a selection problem, including duality
    V_i(P) = V_{n+1-i}(P^rev)."""
    a = (n, i, canon(n, up))
    b = (n, n + 1 - i, canon(n, reverse(n, up)))
    return min(a, b)


# ------------------------------------------------------------- enumerations
def linear_extension_order(n, up):
    down = down_masks(n, up)
    return sorted(range(n), key=lambda x: popcount(down[x]))


def downsets_of_size(n, up, k, down=None):
    """Yield all down-closed sets (bitmasks) of size exactly k."""
    if down is None:
        down = down_masks(n, up)
    order = linear_extension_order(n, up)
    out = []

    def rec(j, S, size):
        if size == k:
            out.append(S)
            return
        if n - j < k - size:
            return
        x = order[j]
        # include x if allowed
        if down[x] & ~S == 0:
            rec(j + 1, S | (1 << x), size + 1)
        rec(j + 1, S, size)

    rec(0, 0, 0)
    return out


def all_downsets(n, up, down=None):
    if down is None:
        down = down_masks(n, up)
    order = linear_extension_order(n, up)
    out = []

    def rec(j, S):
        if j == n:
            out.append(S)
            return
        x = order[j]
        if down[x] & ~S == 0:
            rec(j + 1, S | (1 << x))
        rec(j + 1, S)

    rec(0, 0)
    return out


def compatible_solutions(n, i, up):
    """List of (e, L) with L a down-set of size i-1 and e minimal in V \\ L."""
    down = down_masks(n, up)
    sols = []
    for L in downsets_of_size(n, up, i - 1, down):
        for e in range(n):
            if not (L >> e & 1) and down[e] & ~L == 0:
                sols.append((e, L))
    return sols


def all_posets_unlabeled(nmax):
    """All unlabelled posets with 1..nmax elements, as dict n -> list of up tuples.
    Built by adding a new maximal element with an arbitrary down-closed set."""
    res = {1: [((0,))[0:1]]}
    res[1] = [(0,)]
    for n in range(2, nmax + 1):
        seen = set()
        lst = []
        for up in res[n - 1]:
            m = n - 1
            for D in all_downsets(m, up):
                new_up = list(up) + [0]
                x = D
                while x:
                    b = (x & -x).bit_length() - 1
                    x &= x - 1
                    new_up[b] |= 1 << m
                new_up = tuple(new_up)
                c = canon(n, new_up)
                if c not in seen:
                    seen.add(c)
                    lst.append(c)
        res[n] = lst
    return res
