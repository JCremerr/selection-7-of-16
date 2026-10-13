"""Structure analysis of the (16,7)@28 certificate and its relatives.

Usage:  python3 -I analyze.py  > report.txt        (from any directory)
Reads only the certificate files (and, if present, upstream's shipped optimal
algorithms 15_6.rs / 16_5.rs); uses cert.py (own parser) and shapes.py.
Deterministic (Monte Carlo uses fixed seeds).
"""

import collections
import math
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from cert import Program, closure_add, down_masks, popcount, summary   # noqa: E402
import shapes                                                           # noqa: E402
from shapes import (canon, canon_dual, canon_component, components,     # noqa: E402
                    shape_string, describe_cmp, reverse)

SEL = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
CERT = os.path.join(SEL, "certificates")
UPS = os.path.join(SEL, "external", "selection_generator", "logs", "backward", "algorithms")
FILES = [
    ("V7main", os.path.join(CERT, "V7_16_le28", "select_7th_of_16_28cmp.rs")),
    ("V7alt", os.path.join(CERT, "V7_16_le28", "select_7th_of_16_28cmp_alt.rs")),
    ("V8main", os.path.join(CERT, "V8_16_le29", "select_8th_of_16_29cmp.rs")),
    ("V8alt", os.path.join(CERT, "V8_16_le29", "select_8th_of_16_29cmp_alt_dfpn.rs")),
    ("V7at29", os.path.join(CERT, "V7_16_le29", "select_7th_of_16_29cmp.rs")),
    ("V7at30", os.path.join(CERT, "V7_16_le30", "select_7th_of_16_30cmp.rs")),
    ("up15_7", os.path.join(UPS, "15_6.rs")),
    ("up16_6", os.path.join(UPS, "16_5.rs")),
]


# ---------------------------------------------------------------- named shapes
def _register_R():
    """R8 = two 4-trees whose runner-ups were compared; R16 = two R8 whose
    runner-up winners were compared (4 four-trees + a 4-tree on the 4
    runner-ups).  Element ids: tree t has w=4t, b=4t+1, r=4t+2, d=4t+3."""
    def t4(t):
        w, b, r, d = 4 * t, 4 * t + 1, 4 * t + 2, 4 * t + 3
        return [(w, b), (w, r), (r, d)]
    r8 = t4(0) + t4(1) + [(2, 6)]
    r16 = t4(0) + t4(1) + t4(2) + t4(3) + [(2, 6), (10, 14), (2, 10)]
    shapes._register("R8", 8, r8)
    shapes._register("R16", 16, r16)
    return shapes._poset(16, r16)


R16_POSET = _register_R()
R16_CODE = canon_component(R16_POSET, list(range(16)))[0]


# ---------------------------------------------------------------- helpers
def root_rank1(p):
    f = p.funcs[p.root]
    return f.r + 1


def oriented(p, name, flipped):
    P = p.P[name]
    return reverse(P) if flipped else P


def trunk(p):
    """Follow the root while every outcome leads to the same callee (with the
    same orientation) -> list of steps."""
    steps = []
    name, flipped = p.root, p.root_flip
    depth = 0
    while True:
        f = p.funcs[name]
        sites = p.sites[name]
        calls = [st for Q, st in sites if st[0] == "call"]
        cm = p.cmps[name]
        if len(cm) != 1 or len(calls) != 2:
            break
        Q, lo, hi = cm[0]
        Po = oriented(p, name, flipped)
        if flipped:
            lo, hi = hi, lo
        kind, text = describe_cmp(Po, lo, hi)
        steps.append(dict(depth=depth, name=name, m=f.m, i=f.r + 1, W=p.W[name],
                          shape=shape_string(Po), kind=kind, text=text,
                          sym=(calls[0][1] == calls[1][1] and calls[0][3] == calls[1][3])))
        if not steps[-1]["sym"]:
            break
        flipped ^= calls[0][3]
        name = calls[0][1]
        depth += 1
    end = dict(name=name, depth=depth, m=p.funcs[name].m, i=p.funcs[name].r + 1, W=p.W[name],
               shape=shape_string(oriented(p, name, flipped)),
               code=canon(oriented(p, name, flipped)), flipped=flipped)
    return steps, end


class Tracer:
    """Follow concrete paths with global element labels 0..n-1 (root order)."""

    def __init__(self, p):
        self.p = p

    def start(self):
        n = self.p.funcs[self.p.root].m
        return dict(name=self.p.root, lab=list(range(n)), K=tuple([0] * n),
                    flip=self.p.root_flip, depth=0, dropped=[])

    def step(self, s, outcome):
        """outcome True = then-branch.  Returns new state and (glo, ghi) of the
        comparison in root orientation as 'glo < ghi' for the taken outcome."""
        p = self.p
        st = p.funcs[s["name"]].body
        assert st[0] == "cmp"
        lo, hi = st[1], st[2]
        a, b = s["lab"][lo], s["lab"][hi]
        if s["flip"]:
            a, b = b, a
        if outcome:
            K = closure_add(s["K"], a, b)
            rel = (a, b)
            nxt = st[3]
        else:
            K = closure_add(s["K"], b, a)
            rel = (b, a)
            nxt = st[4]
        assert nxt[0] == "call"
        _, g, args, fl = nxt
        lab = [s["lab"][x] for x in args]
        dropped = s["dropped"] + [s["lab"][x] for x in range(len(s["lab"])) if x not in args]
        return dict(name=g, lab=lab, K=K, flip=s["flip"] ^ fl, depth=s["depth"] + 1,
                    dropped=dropped), rel

    def cmp_labels(self, s):
        st = self.p.funcs[s["name"]].body
        if st[0] != "cmp":
            return None
        a, b = s["lab"][st[1]], s["lab"][st[2]]
        return (b, a) if s["flip"] else (a, b)


def r16_role_names(K, alive):
    """Name the 16 elements of an R16 poset (global knowledge K) as
    wA bA rA dA ... following the definition in shapes/_register_R."""
    n = len(K)
    dn = down_masks(K)
    mins = [v for v in alive if dn[v] == 0]

    def covers(u):
        ups = [v for v in range(n) if (K[u] >> v) & 1]
        return [v for v in ups if not any((K[u] >> z) & 1 and (K[z] >> v) & 1 for z in range(n))]
    tree = {}
    for w in mins:
        cv = covers(w)
        b = [v for v in cv if K[v] == 0]
        r = [v for v in cv if K[v] != 0]
        assert len(b) == 1 and len(r) == 1, (w, cv)
        r = r[0]
        d = [v for v in covers(r) if K[v] == 0]
        assert len(d) == 1
        tree[w] = (w, b[0], r, d[0])
    # classify trees by runner-up relations
    rs = {tree[w][2]: w for w in tree}
    letter = {}
    for rv, w in rs.items():
        preds_r = [x for x in rs if (K[x] >> rv) & 1]
        succ_r = [x for x in rs if (K[rv] >> x) & 1]
        if not preds_r:
            letter[w] = "A"
        elif len(preds_r) == 1 and succ_r:
            letter[w] = "C"
        elif len(preds_r) == 1:
            letter[w] = "B"
        else:
            letter[w] = "D"
    assert sorted(letter.values()) == list("ABCD"), letter
    names = {}
    for w, (ww, b, r, d) in tree.items():
        L = letter[w]
        names[ww], names[b], names[r], names[d] = "w" + L, "b" + L, "r" + L, "d" + L
    return names


# ---------------------------------------------------------------- per-file stats
def depth_profile(p, B):
    """per depth: #functions entered, n range, tight fraction (function,depth)."""
    rows = collections.defaultdict(list)
    for name in p.funcs:
        if p.funcs[name].m == 1:
            continue
        for d in p.depthset[name]:
            rows[d].append(name)
    out = []
    for d in sorted(rows):
        names = rows[d]
        slack = [B - d - p.W[nm] for nm in names]
        ns = [p.funcs[nm].m for nm in names]
        out.append((d, len(names), min(ns), max(ns), sum(1 for s in slack if s == 0),
                    collections.Counter(slack)))
    return out


def mc(p, samples, seed):
    rng = random.Random(seed)
    n = p.funcs[p.root].m
    rank = root_rank1(p)
    cnt = collections.Counter()
    first_drop = collections.Counter()
    for _ in range(samples):
        xs = list(range(n))
        rng.shuffle(xs)
        trace = []
        name, lab, flip, k = p.root, list(range(n)), p.root_flip, 0
        fd = None
        while True:
            st = p.funcs[name].body
            if st[0] == "cmp":
                lo, hi = st[1], st[2]
                less = (xs[lab[lo]] < xs[lab[hi]]) != flip
                st = st[3] if less else st[4]
                k += 1
            if st[0] == "ret":
                assert xs[lab[st[1]]] == rank - 1
                break
            _, g, args, fl = st
            if len(args) < len(lab) and fd is None:
                fd = k
            lab = [lab[a] for a in args]
            flip ^= fl
            name = g
        cnt[k] += 1
        first_drop[fd] += 1
    return cnt, first_drop


def iso_classes(p):
    by_iso = collections.defaultdict(list)
    by_dual = collections.defaultdict(list)
    for name, f in p.funcs.items():
        by_iso[canon(p.P[name], f.r)].append(name)
        by_dual[canon_dual(p.P[name], f.r)].append(name)
    return by_iso, by_dual


def classify_all(p):
    """kind of every comparison site (pair / join / inside), by depth class."""
    out = collections.Counter()
    for name in p.funcs:
        for Q, lo, hi in p.cmps[name]:
            kind, _ = describe_cmp(p.P[name], lo, hi)
            out[kind] += 1
    return out


def main():
    progs = {}
    for key, path in FILES:
        if os.path.exists(path):
            progs[key] = Program(path)
    out = []
    w = out.append

    w("STRUCTURE REPORT (generated by analyze.py; do not edit by hand)")
    w("=" * 72)
    w("")
    w("1. PARSER CHECK (compare with the verifier's statistics)")
    for key, p in progs.items():
        w("[%s]" % key)
        w(summary(p))
        w("")

    # ------------------------------------------------------------- trunks
    w("2. TRUNKS (initial symmetric segment: both outcomes call the same function)")
    trunks = {}
    for key, p in progs.items():
        steps, end = trunk(p)
        trunks[key] = (steps, end)
        nsym = sum(1 for s in steps if s["sym"])
        w("[%s] symmetric comparisons: %d; poset after them: %s (n=%d, i=%d, W=%d)%s"
          % (key, nsym, end["shape"], end["m"], end["i"], end["W"],
             "  [root-oriented]" if end["flipped"] else ""))
        for s in steps:
            w("   %2d  %-26s cmp %-6s %-26s %s" % (s["depth"] + 1, s["shape"], s["kind"], s["text"],
                                                  "" if s["sym"] else "<- first asymmetric"))
    w("")
    # The trunk end before reduction: compare the 16-element posets.
    w("   Poset reached after the 15 trunk comparisons, BEFORE reduction (traced with labels):")
    ends = {}
    for key in progs:
        p = progs[key]
        tr = Tracer(p)
        s = tr.start()
        nsteps = sum(1 for x in trunks[key][0] if x["sym"])
        for _ in range(nsteps):
            s, rel = tr.step(s, True)
        code = canon(s["K"])
        ends[key] = (code, nsteps, s)
        isr16 = (code == (16, None, (R16_CODE,)))
        w("   [%s] after %d comparisons: %s%s" % (key, nsteps, shape_string(s["K"]),
                                                 "  (= R16)" if isr16 else ""))
    w("")

    # ------------------------------------------------------------- main V7
    p = progs["V7main"]
    B = p.W[p.root]
    w("3. V7main IN DETAIL (budget %d)" % B)
    tr = Tracer(p)
    s = tr.start()
    trunk_rels = []
    for t in range(15):
        s, rel = tr.step(s, True)
        trunk_rels.append(rel)
    names = r16_role_names(s["K"], range(16))
    w("   trunk on the all-'first operand smaller' path, in R16 role names:")
    w("   " + ", ".join("%s<%s" % (names[a], names[b]) for a, b in trunk_rels))
    w("")
    w("   Post-trunk tree (depth = comparisons made before the node; budget = %d - depth):" % B)

    def rec(st, depth_left, indent):
        f = p.funcs[st["name"]]
        cl = tr.cmp_labels(st)
        live = st["lab"]
        drop = [names[x] for x in st["dropped"]]
        base = "%s%s d=%d budget=%d W=%d n=%d i=%d" % (
            "   " + "  " * indent, st["name"], st["depth"], B - st["depth"], p.W[st["name"]], f.m, f.r + 1)
        if drop:
            base += " dropped={%s}" % ",".join(drop)
        if cl is None:
            w(base + " LEAF")
            return
        w(base + "  next: %s:%s  [%s]" % (names[cl[0]], names[cl[1]], shape_string(p.P[st["name"]])))
        if depth_left == 0:
            return
        for outcome in (True, False):
            s2, rel = tr.step(st, outcome)
            w("   " + "  " * (indent + 1) + "if %s<%s:" % (names[rel[0]], names[rel[1]]))
            rec(s2, depth_left - 1, indent + 2)
    rec(s, 3, 0)
    w("")

    # depth profile
    w("   Depth profile ((function, depth) pairs; slack = budget - W(f)):")
    w("   depth  #fun  n-range  tight  slack histogram")
    for d, k, nmin, nmax, tight, hist in depth_profile(p, B):
        w("   %5d  %4d  %2d-%-2d    %4d   %s" % (d, k, nmin, nmax, tight,
                                                " ".join("%d:%d" % (a, hist[a]) for a in sorted(hist))))
    # functions by (n,i)
    ni = collections.Counter((f.m, f.r + 1) for f in p.funcs.values())
    w("")
    w("   functions by n (all i):  " + "  ".join(
        "%d:%d" % (n_, sum(v for (a, b), v in ni.items() if a == n_)) for n_ in range(16, 0, -1)))
    w("   functions by (n,i):  " + "  ".join("(%d,%d):%d" % (a, b, ni[(a, b)])
                                         for (a, b) in sorted(ni, key=lambda x: (-x[0], x[1]))))
    # slack overall
    tot_pairs = sum(len(p.depthset[nm]) for nm in p.funcs if p.funcs[nm].m > 1)
    tight_pairs = sum(1 for nm in p.funcs if p.funcs[nm].m > 1 for d in p.depthset[nm] if B - d - p.W[nm] == 0)
    tight_funcs = sum(1 for nm in p.funcs if p.funcs[nm].m > 1 and B - p.dmax[nm] - p.W[nm] == 0)
    always_tight = sum(1 for nm in p.funcs if p.funcs[nm].m > 1 and B - p.dmin[nm] - p.W[nm] == 0)
    nf = sum(1 for nm in p.funcs if p.funcs[nm].m > 1)
    w("")
    w("   comparison functions: %d; tight at some depth (on a worst-case path): %d; tight at every"
      " depth where entered: %d" % (nf, tight_funcs, always_tight))
    w("   (function, depth) pairs: %d, of which tight: %d" % (tot_pairs, tight_pairs))
    multi_depth = sum(1 for nm in p.funcs if len(p.depthset[nm]) > 1)
    w("   functions entered at more than one depth: %d" % multi_depth)
    # reuse
    indeg = collections.Counter(p.in_sites[nm] for nm in p.funcs if nm != p.root)
    w("   in-degree (call sites per function): " + "  ".join("%d:%d" % (a, indeg[a]) for a in sorted(indeg)))
    shared = [nm for nm in p.funcs if p.in_sites[nm] >= 2]
    first_merge = min((p.dmin[nm] for nm in p.funcs
                       if len({(c) for c in callers_of(p, nm)}) >= 2), default=None)
    w("   functions called from >= 2 different functions: %d; shallowest such: depth %s"
      % (sum(1 for nm in p.funcs if len(set(callers_of(p, nm))) >= 2), first_merge))
    big = sorted(p.funcs, key=lambda nm: -p.npaths[nm])
    w("   leaves of the tree unfolding: %d = %.3f * 2^28" % (p.count_leaves(), p.count_leaves() / 2 ** 28))
    w("   worst-case (28-comparison) root-to-leaf paths: %d (%.1f%% of tree leaves)"
      % (p.count_worst_paths(), 100.0 * p.count_worst_paths() / p.count_leaves()))
    by_iso, by_dual = iso_classes(p)
    w("   distinct subproblems (P_f, i) up to isomorphism: %d of %d functions; up to iso+duality: %d"
      % (len(by_iso), len(p.funcs), len(by_dual)))
    kinds = classify_all(p)
    w("   comparison kinds over all %d comparison sites: %s" % (sum(kinds.values()), dict(kinds)))
    w("")

    # one worst-case path, with R16 role names after the trunk
    w("   One worst-case path (at each node take 'first operand smaller' if that keeps depth+W = 28):")
    st = tr.start()
    for t in range(15):
        st, _ = tr.step(st, True)
    while True:
        cl = tr.cmp_labels(st)
        f = p.funcs[st["name"]]
        if cl is None:
            ans = st["lab"][f.body[1]]
            w("     d=%2d LEAF answer %s" % (st["depth"], names[ans]))
            break
        sA, relA = tr.step(st, True)
        if sA["depth"] + p.W[sA["name"]] == B:
            nxt, rel = sA, relA
        else:
            nxt, rel = tr.step(st, False)
        new = [names[x] for x in nxt["dropped"][len(st["dropped"]):]]
        w("     d=%2d (n,i)=(%2d,%d) budget %2d  compare %s:%s -> %s<%s%s" % (
            st["depth"], f.m, f.r + 1, B - st["depth"], names[cl[0]], names[cl[1]],
            names[rel[0]], names[rel[1]], ("  discard " + ",".join(new)) if new else ""))
        st = nxt
    w("")
    w("   n by depth over (function, depth) pairs (n:count):")
    nb = collections.defaultdict(collections.Counter)
    for nm in p.funcs:
        for d in p.depthset[nm]:
            nb[d][p.funcs[nm].m] += 1
    for d in sorted(nb):
        if d >= 15:
            w("     depth %2d: %s" % (d, " ".join("%d:%d" % (k, nb[d][k]) for k in sorted(nb[d], reverse=True))))
    w("")

    # Monte Carlo
    w("4. INPUT-WEIGHTED STATISTICS (Monte Carlo, uniform random permutations)")
    for key in ("V7main", "V7alt", "V8main"):
        q = progs[key]
        S = 200000
        cnt, fdrop = mc(q, S, 20261012)
        mean = sum(k * v for k, v in cnt.items()) / S
        w("[%s] %d samples (seed 20261012): mean comparisons %.4f; distribution %s"
          % (key, S, mean, " ".join("%d:%.4f" % (k, cnt[k] / S) for k in sorted(cnt))))
        w("        first element discarded after comparison k: %s"
          % " ".join("%s:%.4f" % (k, fdrop[k] / S) for k in sorted(fdrop, key=lambda x: (x is None, x))))
    w("")

    # ------------------------------------------------------------- cross-file
    w("5. CROSS-FILE COMPARISON")
    isos = {k: iso_classes(q) for k, q in progs.items()}
    k7 = set(isos["V7main"][0])
    k7a = set(isos["V7alt"][0])
    w("   V7main vs V7alt: shared subproblems (P,i) up to iso: %d (of %d and %d)"
      % (len(k7 & k7a), len(k7), len(k7a)))
    for key in progs:
        if key in ("V7main", "V7alt"):
            continue
        kk = set(isos[key][1])
        w("   V7main vs %s: shared up to iso+duality: %d (of %d / %d)"
          % (key, len(set(isos["V7main"][1]) & kk), len(isos["V7main"][1]), len(kk)))
    w("")
    for key, q in progs.items():
        steps, end = trunks[key]
        w("   [%s] functions %d, worst %d, trunk %d symmetric comparisons, comparison kinds %s"
          % (key, len(q.funcs), q.W[q.root], sum(1 for x in steps if x["sym"]), dict(classify_all(q))))
    w("")
    w("6. NOTE: shape names K<n>.<k> are per-run labels for irregular shapes (not stable across runs);")
    w("   x, P, T4, T8, R8, R16, C3, V, C4 are fixed names defined in shapes.py / analyze.py.")
    print("\n".join(out))


_callers = {}


def callers_of(p, nm):
    if id(p) not in _callers:
        c = collections.defaultdict(list)
        for f in p.funcs:
            for Q, st in p.sites[f]:
                if st[0] == "call":
                    c[st[1]].append(f)
        _callers[id(p)] = c
    return _callers[id(p)][nm]


if __name__ == "__main__":
    main()
