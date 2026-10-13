"""Exact semantic oracle for PLAIN-dialect certificates (audit; independent of
verify_rs_algorithm.py).  Path-sensitive symbolic execution memoised on
(function, exact known poset on its arguments, rank needed by the caller).
Doc comments (ranks and relations) are IGNORED: the rank each function must
return is derived from the root's claimed rank.  Returns (ok, exact_worst, msg)."""
import re
W = r"[a-z][a-z0-9_]*"
FN = re.compile(rf"/// n = (\d+)(?:, i = (\d+))?(?:, {W} < {W})*\nfn (select_\d+)\(\[([a-z0-9_, ]*)\]: \[usize; (\d+)\]\) -> usize \{{\n(.*?)\n\}}\n", re.S)
RUN = re.compile(r"pub fn run\(input: \[usize; (\d+)\]\) -> usize \{\n\s*(select_\d+)\(input\)\n\}")
class Bad(Exception): pass
def names(s): return [t.strip() for t in s.strip().strip("()").split(",")]
def parse_stmt(s):
    m = re.fullmatch(r"(select_\d+)\(\[([a-z0-9_, ]*)\]\)", s)
    if m: return ("call", m.group(1), names(m.group(2)))
    if not re.fullmatch(W, s): raise Bad("stmt " + s)
    return ("ret", s)
def load(text):
    F = {}
    for m in FN.finditer(text):
        n, i, name, ps, ar, body = m.groups()
        ps = names(ps)
        if not (len(ps) == int(ar) == int(n)) or len(set(ps)) != len(ps): raise Bad("arity")
        b = [x.strip() for x in body.split("\n")]
        if len(b) == 1:
            F[name] = (ps, None, [(parse_stmt(b[0]), {})])
        elif len(b) == 5:
            c = re.fullmatch(rf"if ({W}) < ({W}) \{{", b[0])
            if not (c and b[2] == "} else {" and b[4] == "}"): raise Bad("if")
            F[name] = (ps, (c.group(1), c.group(2)), [(parse_stmt(b[1]), {}), (parse_stmt(b[3]), {})])
        elif len(b) == 6:
            c = re.fullmatch(rf"let ({W}|\([a-z0-9_, ]+\)) = if ({W}) < ({W}) \{{", b[0])
            if not (c and b[2] == "} else {" and b[4] == "};"): raise Bad("let")
            pat = names(c.group(1)); tv = names(b[1]); ev = names(b[3])
            if not (len(pat) == len(tv) == len(ev)) or len(set(pat)) != len(pat): raise Bad("let len")
            after = parse_stmt(b[5])
            F[name] = (ps, (c.group(2), c.group(3)), [(after, dict(zip(pat, tv))), (after, dict(zip(pat, ev)))])
        else:
            raise Bad("body")
    if len(re.findall(r"^fn select_", text, re.M)) != len(F): raise Bad("unparsed fn")
    r = RUN.search(text)
    return F, r.group(2)
def pc(x): return bin(x).count("1")
def add(Q, lo, hi):
    up = Q[hi] | (1 << hi)
    return tuple(a | up if (u == lo or (a >> lo) & 1) else a for u, a in enumerate(Q))
def oracle(text, n_claim, rank_claim):
    """Does the program select rank `rank_claim` (0-based) of n_claim inputs?"""
    try:
        F, root = load(text)
    except Bad as e:
        return (False, None, "ill-formed: " + str(e))
    if len(F[root][0]) != n_claim: return (False, None, "wrong n")
    memo, onstack = {}, set()
    def solve(f, Q, r):
        key = (f, Q, r)
        if key in memo: return memo[key]
        if key in onstack: raise Bad("cycle state")
        if len(memo) > 300000: raise Bad("UNDECIDED state cap")
        onstack.add(key)
        ps, cmp, branches = F[f]
        m = len(ps); idx = {p: k for k, p in enumerate(ps)}
        def ix(name):
            if name not in idx: raise Bad("unknown name " + name)
            return idx[name]
        if cmp is None:
            outs = [(Q, branches[0])]
        else:
            x, y = ix(cmp[0]), ix(cmp[1])
            if x == y: outs = [(Q, branches[1])]           # a < a is false
            else:
                outs = []
                if not (Q[y] >> x) & 1: outs.append((Q if (Q[x] >> y) & 1 else add(Q, x, y), branches[0]))
                if not (Q[x] >> y) & 1: outs.append((Q if (Q[y] >> x) & 1 else add(Q, y, x), branches[1]))
        worst = 0
        for Qb, (st, bind) in outs:
            bl = [0] * m
            for u in range(m):
                for v in range(m):
                    if (Qb[u] >> v) & 1: bl[v] |= 1 << u
            if st[0] == "ret":
                p = ix(bind.get(st[1], st[1]))
                if not (pc(bl[p]) == r and pc(Qb[p]) == m - 1 - r): raise Bad(f"{f}: wrong leaf")
                c = 0
            else:
                g = st[1]
                if g not in F: raise Bad("undefined fn")
                args = [ix(bind.get(a, a)) for a in st[2]]
                if len(args) != len(F[g][0]): raise Bad("arity mismatch")
                if len(set(args)) != len(args):
                    raise Bad("repeated arg")   # spec is for distinct inputs; treat as wrong
                kb = 0
                for e in range(m):
                    if e in args: continue
                    if m - 1 - pc(Qb[e]) < r: kb += 1
                    elif pc(bl[e]) > r: pass
                    else: raise Bad(f"{f}: drops possible answer {ps[e]}")
                k = len(args)
                Qg = tuple(sum(1 << l for l in range(k) if (Qb[args[j]] >> args[l]) & 1) for j in range(k))
                c = solve(g, Qg, r - kb)
            worst = max(worst, c)
        res = worst + (1 if cmp is not None else 0)
        onstack.discard(key)
        memo[key] = res
        return res
    try:
        w = solve(root, tuple([0] * n_claim), rank_claim)
    except Bad as e:
        return (False, None, str(e))
    except RecursionError:
        return (False, None, "recursion")
    return (True, w, f"states={len(memo)}")
