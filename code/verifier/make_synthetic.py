#!/usr/bin/env python3
"""
make_synthetic.py -- TEST DATA generator: random but correct selection
algorithms in the DUAL (backward-search) or PLAIN (forward-search) dialect.

    python3 -I make_synthetic.py N I0 SEED {dual|plain} OUT.rs [--run] [--pair-heuristic]

The algorithm is a random decision procedure on posets: in a state (poset of
the live elements, rank r) it returns a determined element, or drops all
certified elements and forwards, or compares a random incomparable pair.
States are memoised (after a deterministic relabelling), so functions are
shared by several call sites.  Random choices exercise every grammar feature
the checker accepts: leaf returns at any arity, forwarding calls without
comparison, !is_dual flips, `_` parameters, plain if, let-tuple and
let-single rebinding (with parameter or fresh pattern names), doc-comment
claims, and run wrappers (also with is_dual = true at the root).

It is not optimal and not meant to be; it produces structurally varied
inputs for the tests and large files for the timing test.
"""

import random
import sys

LETTERS = "abcdefghijklmnopqrstuvwxyz"


def popcount(x):
    return bin(x).count("1")


def below_of(above):
    m = len(above)
    return [sum(1 << u for u in range(m) if (above[u] >> v) & 1) for v in range(m)]


def add(above, lo, hi):
    up = above[hi] | (1 << hi)
    return tuple(a | up if (u == lo or (a >> lo) & 1) else a for u, a in enumerate(above))


class Gen:
    def __init__(self, rng, dialect, heuristic):
        self.rng, self.dialect, self.heuristic = rng, dialect, heuristic
        self.memo, self.out, self.count = {}, [], 0

    # ---- choosing the algorithm ------------------------------------------
    def solve(self, r, above, name=None):
        key = (r, above)
        if key in self.memo:
            return self.memo[key]
        if name is None:
            self.count += 1
            name = f"select_{self.count}"
        self.memo[key] = name
        m, rng = len(above), self.rng
        below = below_of(above)
        det = [p for p in range(m) if popcount(below[p]) == r and popcount(above[p]) == m - 1 - r]
        certified = [e for e in range(m) if popcount(above[e]) >= m - r or popcount(below[e]) >= r + 1]
        if det:
            p = det[0]
            body = ("leaf", p) if m == 1 or rng.random() < 0.5 else ("stmt", self.call(above, r))
        elif certified and rng.random() < 0.5:
            body = ("stmt", self.call(above, r))
        else:
            pairs = [(x, y) for x in range(m) for y in range(m)
                     if x != y and not (above[x] >> y) & 1 and not (above[y] >> x) & 1]
            if self.heuristic:   # prefer pairs of elements with little known: smaller trees
                score = lambda xy: popcount(above[xy[0]] | below[xy[0]]) + popcount(above[xy[1]] | below[xy[1]])
                best = min(map(score, pairs))
                pairs = [xy for xy in pairs if score(xy) == best]
            x, y = rng.choice(pairs)
            body = ("if", x, y, self.branch(add(above, x, y), r), self.branch(add(above, y, x), r))
        self.out.append(self.emit(name, r, above, body))
        return name

    def branch(self, Q, r):
        m = len(Q)
        below = below_of(Q)
        det = [p for p in range(m) if popcount(below[p]) == r and popcount(Q[p]) == m - 1 - r]
        if det and self.rng.random() < 0.3:
            return ("leaf", det[0])
        return self.call(Q, r)

    def call(self, Q, r):
        """Drop every certified element of Q and call the function for the rest."""
        m, rng = len(Q), self.rng
        below = below_of(Q)
        kept, k_below = [], 0
        for e in range(m):
            if popcount(Q[e]) >= m - r:
                k_below += 1
            elif popcount(below[e]) < r + 1:
                kept.append(e)
        if len(kept) == m and rng.random() < 0.5:
            rng.shuffle(kept)          # pure permutation (only reached from forwarding bodies)
        k = len(kept)
        r2 = r - k_below
        flip = self.dialect == "dual" and rng.random() < 0.3
        rel = lambda u, v: (Q[kept[u]] >> kept[v]) & 1        # kept[u] < kept[v]
        if flip:
            r2 = k - 1 - r2
            rel = (lambda f: lambda u, v: f(v, u))(rel)
        # deterministic relabelling (more sharing); sometimes a random order
        sub_above = [sum(1 << v for v in range(k) if rel(u, v)) for u in range(k)]
        sub_below = below_of(sub_above)
        order = list(range(k))
        if rng.random() < 0.8:
            order.sort(key=lambda u: (popcount(sub_below[u]), popcount(sub_above[u]), u))
        else:
            rng.shuffle(order)
        pos = {u: j for j, u in enumerate(order)}
        new_above = tuple(sum(1 << pos[v] for v in range(k) if rel(u, v)) for u in order)
        callee = self.solve(r2, new_above)
        return ("call", callee, [kept[u] for u in order], flip)

    # ---- printing ------------------------------------------------------------
    def emit(self, name, r, above, body):
        m, rng = len(above), self.rng
        P = LETTERS[:m]
        dual = self.dialect == "dual"
        doc = f"/// n = {m}" if (m == 1 and not dual and rng.random() < 0.7) else f"/// n = {m}, i = {r}"
        if not dual:     # claims: the cover relations (sometimes all relations)
            below = below_of(above)
            full = rng.random() < 0.3
            claims = [(u, v) for u in range(m) for v in range(m) if (above[u] >> v) & 1
                      and (full or not any((above[u] >> w) & 1 and (above[w] >> v) & 1 for w in range(m)))]
            doc += "".join(f", {P[u]} < {P[v]}" for u, v in claims)
        uses_dual = body[0] != "leaf"
        if dual:
            dpar = "is_dual" if uses_dual or rng.random() < 0.5 else "_"
            sig = f"fn {name}([{', '.join(P)}]: [usize; {m}], {dpar}: bool) -> usize {{"
        else:
            sig = f"fn {name}([{', '.join(P)}]: [usize; {m}]) -> usize {{"

        def st(s):
            if s[0] == "leaf":
                return P[s[1]]
            _, callee, args, flip = s
            a = ", ".join(P[x] for x in args)
            if dual:
                return f"{callee}([{a}], {'!is_dual' if flip else 'is_dual'})"
            return f"{callee}([{a}])"

        lines = [doc, sig]
        if body[0] == "leaf":
            lines.append(f"    {P[body[1]]}")
        elif body[0] == "stmt":
            lines.append(f"    {st(body[1])}")
        else:
            _, x, y, t, e = body
            let = (not dual and t[0] == e[0] == "call" and t[1] == e[1] and rng.random() < 0.7
                   and self.let_form(P, x, y, t, e))
            if let:
                lines += let
            else:
                cond = (f"if (!is_dual && {P[x]} < {P[y]}) || (is_dual && {P[x]} > {P[y]}) {{" if dual
                        else f"if {P[x]} < {P[y]} {{")
                lines += [f"    {cond}", f"        {st(t)}", "    } else {", f"        {st(e)}", "    }"]
        lines.append("}")
        return "\n".join(lines) + "\n"

    def let_form(self, P, x, y, t, e):
        """`let PAT = if x < y { T } else { E }; callee([..])` with the same
        meaning as the two calls t and e; None if no simple form exists."""
        targs, eargs = [P[a] for a in t[2]], [P[a] for a in e[2]]
        diff = [j for j in range(len(targs)) if targs[j] != eargs[j]]
        fresh = self.rng.random() < 0.3
        pattern = [f"v{j}" if fresh else targs[j] for j in diff]
        shared = {targs[j] for j in range(len(targs)) if j not in diff}
        if not diff or shared & set(pattern):
            return None
        callargs = list(targs)
        for j, pname in zip(diff, pattern):
            callargs[j] = pname

        def tup(vals):
            return vals[0] if len(vals) == 1 else f"({', '.join(vals)})"
        pat = tup(pattern) if len(pattern) == 1 else f"({', '.join(pattern)})"
        return [f"    let {pat} = if {P[x]} < {P[y]} {{", f"        {tup([targs[j] for j in diff])}",
                "    } else {", f"        {tup([eargs[j] for j in diff])}", "    };",
                f"    {t[1]}([{', '.join(callargs)}])"]


def generate(n, i0, seed, dialect, run=False, heuristic=False):
    rng = random.Random(seed)
    g = Gen(rng, dialect, heuristic)
    root_dual = dialect == "dual" and run and rng.random() < 0.5
    r_root = n - 1 - i0 if root_dual else i0
    g.solve(r_root, (0,) * n, name="select_0")
    head = ""
    if run:
        call = f"select_0(input, {'true' if root_dual else 'false'})" if dialect == "dual" else "select_0(input)"
        head = (f"#[allow(unused)]\npub const N: usize = {n};\n#[allow(unused)]\npub const I: usize = {i0};\n\n"
                f"#[allow(unused)]\npub fn run(input: [usize; {n}]) -> usize {{\n  {call}\n}}\n\n")
    return head + "".join(g.out), len(g.out)


def main():
    n, i0, seed, dialect, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
    sys.setrecursionlimit(100000)
    text, nf = generate(n, i0, seed, dialect, "--run" in sys.argv, "--pair-heuristic" in sys.argv)
    with open(out, "w") as fh:
        fh.write(text)
    print(f"wrote {out}: {nf} functions, {len(text)} bytes")


if __name__ == "__main__":
    main()
