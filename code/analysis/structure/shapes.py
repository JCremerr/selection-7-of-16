"""Poset shapes for the structure analysis.

Exact canonical forms by colour refinement + individualisation (exhaustive
branching, no automorphism pruning), computed per connected component of the
comparability graph; a poset's form is the sorted multiset of its component
forms.  Isomorphism of (P, r) <=> equal canonical forms (exact, not hashed).

Named component shapes (orientation: "smaller" = winner, min at the bottom):
  x   isolated element               P   pair a<b
  T4  4-tree: binomial tree B2 = two pairs whose winners were compared:
      w<b, w<r, r<d   (w winner, r runner-up = loser of the final, b and d
      first-round losers of w and r)
  T8  binomial tree B3 (two T4 whose winners were compared)
  C3  chain of 3       V  cherry a<b, a<c       A  a<c, b<c
Duals get a trailing "'" (e.g. T4' = the 4-tree with the max on top).
Element roles: friendly names for x/P/T4, otherwise "d<below>u<above>"
(counts inside the element's component).
"""

from cert import popcount, down_masks


def components(up):
    m = len(up)
    dn = down_masks(up)
    seen = 0
    comps = []
    for s in range(m):
        if (seen >> s) & 1:
            continue
        comp = 1 << s
        frontier = comp
        while frontier:
            nb = 0
            x = frontier
            while x:
                b = x & -x
                j = b.bit_length() - 1
                nb |= up[j] | dn[j]
                x ^= b
            frontier = nb & ~comp
            comp |= nb
        seen |= comp
        comps.append([j for j in range(m) if (comp >> j) & 1])
    return comps


def _refine(col, U, D):
    k = len(col)
    while True:
        sig = [(col[j], tuple(sorted(col[x] for x in U[j])), tuple(sorted(col[x] for x in D[j])))
               for j in range(k)]
        keys = {s: t for t, s in enumerate(sorted(set(sig)))}
        new = [keys[s] for s in sig]
        if len(keys) == len(set(col)):
            return new
        col = new


_cache = {}


def canon_component(up, verts):
    """Return (code, order): code = (k, masks in canonical labelling);
    order[t] = global vertex that gets canonical label t."""
    k = len(verts)
    loc = {v: j for j, v in enumerate(verts)}
    U = [[loc[w] for w in verts if (up[v] >> w) & 1] for v in verts]
    D = [[] for _ in range(k)]
    for j in range(k):
        for x in U[j]:
            D[x].append(j)
    key = (k, tuple(tuple(sorted(u)) for u in U))
    if key in _cache:
        code, lorder = _cache[key]
        return code, [verts[j] for j in lorder]
    col0 = _refine([(len(D[j]), len(U[j])) for j in range(k)], U, D)
    best = [None, None]
    leaves = [0]

    def search(col):
        if len(set(col)) == k:
            leaves[0] += 1
            order = sorted(range(k), key=lambda j: col[j])
            lab = {j: t for t, j in enumerate(order)}
            masks = []
            for j in order:
                mk = 0
                for x in U[j]:
                    mk |= 1 << lab[x]
                masks.append(mk)
            code = (k, tuple(masks))
            if best[0] is None or code < best[0]:
                best[0], best[1] = code, order
            return
        if leaves[0] > 200000:
            raise RuntimeError("canonical form search too large (k=%d)" % k)
        # smallest colour with a non-singleton class
        cnt = {}
        for c in col:
            cnt[c] = cnt.get(c, 0) + 1
        c0 = min(c for c in cnt if cnt[c] > 1)
        for v in range(k):
            if col[v] == c0:
                col2 = [2 * c + (0 if x == v else 1) for x, c in enumerate(col)]
                search(_refine(col2, U, D))

    search(col0)
    _cache[key] = (best[0], best[1])
    return best[0], [verts[j] for j in best[1]]


def canon(up, r=None):
    comps = components(up)
    codes = sorted(canon_component(up, c)[0] for c in comps)
    return (len(up), r, tuple(codes))


def reverse(up):
    return tuple(down_masks(up))


def canon_dual(up, r):
    """canonical form up to isomorphism AND duality (P,r) ~ (P^d, m-1-r)."""
    a = canon(up, r)
    b = canon(reverse(up), len(up) - 1 - r)
    return min(a, b)


# ------------------------------------------------------------ named shapes
def _poset(k, rels):
    up = [0] * k
    for (a, b) in rels:
        up[a] |= 1 << b
    # transitive closure
    changed = True
    while changed:
        changed = False
        for a in range(k):
            x = up[a]
            y = x
            for b in range(k):
                if (x >> b) & 1:
                    y |= up[b]
            if y != x:
                up[a] = y
                changed = True
    return tuple(up)


def _binomial(j):
    """binomial tree B_j (min at the root) on 2^j elements, as relations."""
    if j == 0:
        return 1, []
    k, rels = _binomial(j - 1)
    rels2 = rels + [(a + k, b + k) for (a, b) in rels] + [(0, k)]
    return 2 * k, rels2


NAMED = {}


def _register(name, k, rels):
    up = _poset(k, rels)
    code, _ = canon_component(up, list(range(k)))
    NAMED.setdefault(code, name)
    upd = reverse(up)
    coded, _ = canon_component(upd, list(range(k)))
    NAMED.setdefault(coded, name + "'")


_register("x", 1, [])
_register("P", 2, [(0, 1)])
_register("T4", *_binomial(2))
_register("T8", *_binomial(3))
_register("C3", 3, [(0, 1), (1, 2)])
_register("V", 3, [(0, 1), (0, 2)])
_register("C4", 4, [(0, 1), (1, 2), (2, 3)])

_other = {}


def comp_name(code):
    if code in NAMED:
        return NAMED[code]
    if code not in _other:
        _other[code] = "K%d.%d" % (code[0], 1 + sum(1 for c in _other if c[0] == code[0]))
    return _other[code]


def other_shapes():
    return dict(_other)


def hasse_edges_of_code(code):
    k, masks = code
    edges = []
    for a in range(k):
        for b in range(k):
            if (masks[a] >> b) & 1:
                if not any((masks[a] >> c) & 1 and (masks[c] >> b) & 1 for c in range(k)):
                    edges.append((a, b))
    return edges


def shape_string(up):
    """e.g. '2 T4 + 4 P + 4 x'  (largest components first)."""
    comps = components(up)
    names = {}
    for c in comps:
        code, _ = canon_component(up, c)
        nm = comp_name(code)
        names[nm] = names.get(nm, 0) + 1

    def keyf(nm):
        return (-_size_of(nm), nm)
    parts = []
    for nm in sorted(names, key=keyf):
        parts.append(("%d %s" % (names[nm], nm)) if names[nm] > 1 else nm)
    return " + ".join(parts)


_sizes = {}


def _size_of(nm):
    if not _sizes:
        for code, n in NAMED.items():
            _sizes[n] = code[0]
    if nm in _sizes:
        return _sizes[nm]
    for code, n in _other.items():
        if n == nm:
            return code[0]
    return 0


FRIENDLY = {
    ("x", (0, 0)): "x",
    ("P", (0, 1)): "P.w", ("P", (1, 0)): "P.l",
    ("T4", (0, 3)): "T4.w", ("T4", (1, 1)): "T4.r", ("T4", (1, 0)): "T4.b", ("T4", (2, 0)): "T4.d",
}


def element_role(up, comp, v):
    """role of element v (global index) in its component comp (list)."""
    code, _ = canon_component(up, comp)
    nm = comp_name(code)
    cm = 0
    for u in comp:
        cm |= 1 << u
    dn = down_masks(up)
    du = (popcount(dn[v] & cm), popcount(up[v] & cm))
    return FRIENDLY.get((nm, du), "%s[d%du%d]" % (nm, du[0], du[1])), nm


def comp_of(up):
    comps = components(up)
    where = {}
    for c in comps:
        for v in c:
            where[v] = c
    return where


def describe_cmp(up, lo, hi):
    """Describe comparison lo:hi in poset up by roles; returns (kind, text)."""
    where = comp_of(up)
    ra, na = element_role(up, where[lo], lo)
    rb, nb = element_role(up, where[hi], hi)
    if where[lo] is where[hi]:
        kind = "inside"
    elif na == "x" and nb == "x":
        kind = "pair"
    else:
        kind = "join"
    a, b = sorted([ra, rb])
    return kind, "%s : %s" % (a, b)
