"""Independent reader for selection-algorithm certificates.

Written for the structure analysis only.  It does NOT import the verifier
(code/verifier) or any search code; it re-implements, from the definition of
selection, what a certificate means:

  * a token-level recursive-descent parser for both upstream dialects
    (PLAIN: `if x < y`, `let PAT = if ..`; DUAL: `is_dual` / `!is_dual`);
  * preconditions P_f (intersection over all call sites of the relations known
    there, mapped to the callee's parameters, reversed for `!is_dual`);
  * a mini check of the local correctness rules (comparison of incomparable
    elements, dropped elements certified below/above, rank bookkeeping,
    leaf rank) -- used only to confirm that the reading is right;
  * worst case W(f) = longest path (comparisons) from f to a leaf;
  * depths at which f is reached from the root, number of root paths.

Posets are tuples `up` of bitmasks: bit v of up[u] set  <=>  u < v.
Usage as a script:  python3 -I cert.py FILE.rs   (prints summary statistics)
"""

import re
import sys

# ---------------------------------------------------------------- tokenizer
_TOK = re.compile(r"""
    (?P<ws>\s+)
  | (?P<doc>///[^\n]*)
  | (?P<com>//[^\n]*)
  | (?P<id>[A-Za-z_][A-Za-z0-9_]*)
  | (?P<num>\d+)
  | (?P<op>->|&&|\|\||[\[\](){}<>,;:=!#])
""", re.X)


def tokenize(text):
    toks = []
    pos = 0
    while pos < len(text):
        m = _TOK.match(text, pos)
        if not m:
            raise SyntaxError("bad character at offset %d: %r" % (pos, text[pos:pos + 20]))
        pos = m.end()
        k = m.lastgroup
        if k in ("ws", "com"):
            continue
        toks.append((k, m.group(k)))
    return toks


class Func:
    __slots__ = ("name", "m", "r", "doc_rels", "body", "dual_param")

    def __init__(self, name, m, r, doc_rels, body, dual_param):
        self.name, self.m, self.r = name, m, r
        self.doc_rels, self.body, self.dual_param = doc_rels, body, dual_param


# statements:  ('ret', pos) | ('call', fname, args tuple, flip) |
#              ('cmp', lo, hi, then_stmt, else_stmt)   [then: x_lo < x_hi]

class Parser:
    def __init__(self, toks):
        self.t = toks
        self.i = 0

    def peek(self, k=0):
        j = self.i + k
        return self.t[j] if j < len(self.t) else (None, None)

    def take(self, val=None, kind=None):
        k, v = self.peek()
        if (val is not None and v != val) or (kind is not None and k != kind):
            raise SyntaxError("expected %r/%r got %r at token %d" % (val, kind, (k, v), self.i))
        self.i += 1
        return v

    def opt(self, val):
        if self.peek()[1] == val:
            self.i += 1
            return True
        return False

    def parse_file(self):
        funcs = {}
        root_call = None
        pending_doc = None
        while self.i < len(self.t):
            k, v = self.peek()
            if k == "doc":
                pending_doc = v
                self.i += 1
            elif v == "#":  # attribute #[ ... ]
                self.take("#")
                self.take("[")
                depth = 1
                while depth:
                    vv = self.take()
                    depth += (vv == "[") - (vv == "]")
            elif v == "pub" and self.peek(1)[1] == "const":
                while self.take() != ";":
                    pass
            elif v == "pub" and self.peek(1)[1] == "fn":
                self.take("pub")
                self.take("fn")
                self.take("run")
                while self.take() != "{":
                    pass
                fname = self.take(kind="id")
                self.take("(")
                self.take("input")
                flag = False
                if self.opt(","):
                    flag = self.take() == "true"
                self.take(")")
                self.opt(";")
                self.take("}")
                root_call = (fname, flag)
            elif v == "fn":
                f = self.parse_func(pending_doc)
                if f.name in funcs:
                    raise SyntaxError("duplicate " + f.name)
                funcs[f.name] = f
                pending_doc = None
            else:
                raise SyntaxError("unexpected %r at token %d" % (v, self.i))
        return funcs, root_call

    def parse_func(self, doc):
        self.take("fn")
        name = self.take(kind="id")
        self.take("(")
        self.take("[")
        params = []
        while True:
            params.append(self.take(kind="id"))
            if not self.opt(","):
                break
        self.take("]")
        self.take(":")
        self.take("[")
        self.take("usize")
        self.take(";")
        m = int(self.take(kind="num"))
        self.take("]")
        dual_param = None
        if self.opt(","):
            dual_param = self.take(kind="id")  # is_dual or _
            self.take(":")
            self.take("bool")
        self.take(")")
        self.take("->")
        self.take("usize")
        self.take("{")
        env = {p: j for j, p in enumerate(params)}
        if len(env) != m or len(params) != m:
            raise SyntaxError("arity mismatch in " + name)
        body = self.parse_stmt(env)
        self.take("}")
        # doc comment: /// n = m[, i = r][, u < v]*
        if doc is None:
            raise SyntaxError("missing doc comment for " + name)
        parts = [s.strip() for s in doc[3:].split(",")]
        mm = re.fullmatch(r"n = (\d+)", parts[0])
        if not mm or int(mm.group(1)) != m:
            raise SyntaxError("doc n mismatch in " + name)
        r = None
        rels = []
        for s in parts[1:]:
            mi = re.fullmatch(r"i = (\d+)", s)
            mr = re.fullmatch(r"([a-z][a-z0-9_]*) < ([a-z][a-z0-9_]*)", s)
            if mi:
                r = int(mi.group(1))
            elif mr:
                rels.append((env[mr.group(1)], env[mr.group(2)]))
            else:
                raise SyntaxError("bad doc item %r in %s" % (s, name))
        if r is None:
            if m != 1:
                raise SyntaxError("doc rank missing in " + name)
            r = 0
        return Func(name, m, r, rels, body, dual_param)

    def parse_cond(self, env):
        # PLAIN:  X < Y      DUAL: (!is_dual && X < Y) || (is_dual && X > Y)
        if self.peek()[1] == "(":
            self.take("(")
            self.take("!")
            self.take("is_dual")
            self.take("&&")
            x1 = self.take(kind="id")
            op1 = self.take()
            y1 = self.take(kind="id")
            self.take(")")
            self.take("||")
            self.take("(")
            self.take("is_dual")
            self.take("&&")
            x2 = self.take(kind="id")
            op2 = self.take()
            y2 = self.take(kind="id")
            self.take(")")
            if not (x1 == x2 and y1 == y2 and {op1, op2} == {"<", ">"}):
                raise SyntaxError("inconsistent dual condition")
            x, y, op = x1, y1, op1
        else:
            x = self.take(kind="id")
            op = self.take()
            y = self.take(kind="id")
        if op == "<":
            return env[x], env[y]
        if op == ">":
            return env[y], env[x]
        raise SyntaxError("bad comparison operator " + op)

    def parse_tuple(self, env):
        if self.opt("("):
            out = []
            while True:
                out.append(env[self.take(kind="id")])
                if not self.opt(","):
                    break
            self.take(")")
            return out
        return [env[self.take(kind="id")]]

    def parse_pat(self):
        if self.opt("("):
            out = []
            while True:
                out.append(self.take(kind="id"))
                if not self.opt(","):
                    break
            self.take(")")
            return out
        return [self.take(kind="id")]

    def parse_stmt(self, env):
        k, v = self.peek()
        if v == "if":
            self.take("if")
            lo, hi = self.parse_cond(env)
            self.take("{")
            s1 = self.parse_stmt(env)
            self.take("}")
            self.take("else")
            self.take("{")
            s2 = self.parse_stmt(env)
            self.take("}")
            return ("cmp", lo, hi, s1, s2)
        if v == "let":
            self.take("let")
            pat = self.parse_pat()
            self.take("=")
            self.take("if")
            lo, hi = self.parse_cond(env)
            self.take("{")
            t1 = self.parse_tuple(env)
            self.take("}")
            self.take("else")
            self.take("{")
            t2 = self.parse_tuple(env)
            self.take("}")
            self.take(";")
            save = self.i
            env1 = dict(env)
            env1.update(zip(pat, t1))
            s1 = self.parse_stmt(env1)
            end1 = self.i
            self.i = save
            env2 = dict(env)
            env2.update(zip(pat, t2))
            s2 = self.parse_stmt(env2)
            assert self.i == end1
            if len(pat) != len(t1) or len(pat) != len(t2):
                raise SyntaxError("let arity")
            return ("cmp", lo, hi, s1, s2)
        if k == "id" and v.startswith("select_") and self.peek(1)[1] == "(":
            fname = self.take()
            self.take("(")
            self.take("[")
            args = []
            while True:
                args.append(env[self.take(kind="id")])
                if not self.opt(","):
                    break
            self.take("]")
            flip = False
            if self.opt(","):
                if self.opt("!"):
                    self.take("is_dual")
                    flip = True
                else:
                    self.take("is_dual")
            self.take(")")
            self.opt(";")
            return ("call", fname, tuple(args), flip)
        if k == "id":
            self.take()
            self.opt(";")
            return ("ret", env[v])
        raise SyntaxError("bad statement at token %d: %r" % (self.i, v))


# ---------------------------------------------------------------- posets
def closure_add(up, lo, hi):
    """up closed, lo || hi: return closure of up + {lo < hi}."""
    m = len(up)
    add = up[hi] | (1 << hi)
    new = list(up)
    for u in range(m):
        if u == lo or (up[u] >> lo) & 1:
            new[u] |= add
    return tuple(new)


def down_masks(up):
    m = len(up)
    dn = [0] * m
    for u in range(m):
        x = up[u]
        while x:
            b = x & -x
            dn[b.bit_length() - 1] |= 1 << u
            x ^= b
    return dn


def popcount(x):
    return bin(x).count("1")


def map_to_callee(Q, args, flip):
    k = len(args)
    out = [0] * k
    for j, a in enumerate(args):
        for l, b in enumerate(args):
            if flip:
                if (Q[b] >> a) & 1:
                    out[j] |= 1 << l
            else:
                if (Q[a] >> b) & 1:
                    out[j] |= 1 << l
    return tuple(out)


# ---------------------------------------------------------------- program
class Program:
    def __init__(self, path):
        self.path = path
        text = open(path, encoding="ascii").read()
        self.funcs, root_call = Parser(tokenize(text)).parse_file()
        called = set()
        for f in self.funcs.values():
            for st in self.stmts(f.body):
                if st[0] == "call":
                    called.add(st[1])
        roots = [n for n in self.funcs if n not in called]
        if root_call:
            self.root = root_call[0]
            self.root_flip = root_call[1]
        else:
            self.root = roots[0]
            self.root_flip = False
        assert roots == [self.root], roots
        self.dialect = "DUAL" if any(f.dual_param for f in self.funcs.values()) else "PLAIN"
        self._toposort()
        self._preconditions()
        self._worst()
        self._depths()

    @staticmethod
    def stmts(st):
        stack = [st]
        while stack:
            s = stack.pop()
            yield s
            if s[0] == "cmp":
                stack.append(s[3])
                stack.append(s[4])

    def callees(self, name):
        return [s[1] for s in self.stmts(self.funcs[name].body) if s[0] == "call"]

    def _toposort(self):
        order, state = [], {}
        stack = [(self.root, iter(self.callees(self.root)))]
        state[self.root] = 1
        while stack:
            name, it = stack[-1]
            nxt = next(it, None)
            if nxt is None:
                stack.pop()
                state[name] = 2
                order.append(name)
            elif nxt not in state:
                state[nxt] = 1
                stack.append((nxt, iter(self.callees(nxt))))
            elif state[nxt] == 1:
                raise ValueError("cycle at " + nxt)
        self.topo = order[::-1]          # callers first
        unreachable = set(self.funcs) - set(order)
        assert not unreachable, unreachable

    def walk_sites(self, name, P):
        """Yield (knowledge, stmt) for every call/ret leaf of f's body, plus
        ('cmp', knowledge, lo, hi) markers through `self.cmp_sites`."""
        out = []
        cmps = []

        def rec(st, Q):
            if st[0] == "cmp":
                lo, hi = st[1], st[2]
                cmps.append((Q, lo, hi))
                if (Q[lo] >> hi) & 1 or (Q[hi] >> lo) & 1:
                    self.violations.append("%s: comparison of comparable elements" % name)
                    return
                rec(st[3], closure_add(Q, lo, hi))
                rec(st[4], closure_add(Q, hi, lo))
            else:
                out.append((Q, st))
        rec(self.funcs[name].body, P)
        return out, cmps

    def _preconditions(self):
        self.violations = []
        self.P = {}
        acc = {}
        m0 = self.funcs[self.root].m
        acc[self.root] = tuple([0] * m0)
        self.sites = {}      # name -> list of (Q, stmt)
        self.cmps = {}
        self.in_sites = {n: 0 for n in self.funcs}
        for name in self.topo:
            f = self.funcs[name]
            P = acc[name]
            self.P[name] = P
            sites, cmps = self.walk_sites(name, P)
            self.sites[name] = sites
            self.cmps[name] = cmps
            r, m = f.r, f.m
            for Q, st in sites:
                if st[0] == "ret":
                    p = st[1]
                    dn = down_masks(Q)
                    if not (popcount(dn[p]) == r and popcount(Q[p]) == m - 1 - r):
                        self.violations.append("%s: leaf rank" % name)
                    continue
                _, g, args, flip = st
                self.in_sites[g] += 1
                gf = self.funcs[g]
                if len(set(args)) != len(args) or len(args) != gf.m:
                    self.violations.append("%s: bad call args" % name)
                dn = down_masks(Q)
                kb = 0
                for e in range(m):
                    if e in args:
                        continue
                    if popcount(Q[e]) >= m - r:
                        kb += 1
                    elif popcount(dn[e]) >= r + 1:
                        pass
                    else:
                        self.violations.append("%s: uncertified drop" % name)
                need = (r - kb) if not flip else (len(args) - 1 - (r - kb))
                if gf.r != need:
                    self.violations.append("%s: rank mismatch calling %s" % (name, g))
                M = map_to_callee(Q, args, flip)
                if g in acc:
                    acc[g] = tuple(a & b for a, b in zip(acc[g], M))
                else:
                    acc[g] = M
        # doc relations implied?
        self.doc_not_implied = 0
        self.doc_closure_equal = 0
        for name, f in self.funcs.items():
            P = self.P[name]
            C = tuple([0] * f.m)
            for (u, v) in f.doc_rels:
                if not (P[u] >> v) & 1:
                    self.doc_not_implied += 1
                if not (C[u] >> v) & 1 and not (C[v] >> u) & 1:
                    C = closure_add(C, u, v)
            if f.doc_rels and C == P:
                self.doc_closure_equal += 1

    def _worst(self):
        self.W = {}
        for name in reversed(self.topo):
            def rec(st):
                if st[0] == "cmp":
                    return 1 + max(rec(st[3]), rec(st[4]))
                if st[0] == "ret":
                    return 0
                return self.W[st[1]]
            self.W[name] = rec(self.funcs[name].body)

    def _depths(self):
        """min/max depth (comparisons before entry), #root paths, and the set
        of depths at which each function is entered."""
        INF = 10 ** 9
        self.dmin = {n: INF for n in self.funcs}
        self.dmax = {n: -1 for n in self.funcs}
        self.npaths = {n: 0 for n in self.funcs}
        self.depthset = {n: set() for n in self.funcs}
        self.dmin[self.root] = self.dmax[self.root] = 0
        self.npaths[self.root] = 1
        self.depthset[self.root] = {0}
        for name in self.topo:
            def rec(st, d):
                if st[0] == "cmp":
                    rec(st[3], d + 1)
                    rec(st[4], d + 1)
                elif st[0] == "call":
                    g = st[1]
                    self.dmin[g] = min(self.dmin[g], self.dmin[name] + d)
                    self.dmax[g] = max(self.dmax[g], self.dmax[name] + d)
                    self.npaths[g] += self.npaths[name]
                    self.depthset[g] |= {x + d for x in self.depthset[name]}
            rec(self.funcs[name].body, 0)

    def n_cmp_sites(self):
        return sum(1 for f in self.funcs.values() for s in self.stmts(f.body) if s[0] == "cmp")

    def n_call_sites(self):
        return sum(1 for f in self.funcs.values() for s in self.stmts(f.body) if s[0] == "call")

    def n_leaf_returns(self):
        return sum(1 for f in self.funcs.values() for s in self.stmts(f.body) if s[0] == "ret")

    def count_worst_paths(self):
        """number of root-to-leaf paths (in the tree unfolding) of length W(root)."""
        memo = {}
        W = self.W

        def cnt(name, budget):
            # number of paths from entry of name that use exactly `budget` comparisons
            key = (name, budget)
            if key in memo:
                return memo[key]

            def rec(st, b):
                if st[0] == "cmp":
                    return rec(st[3], b - 1) + rec(st[4], b - 1)
                if st[0] == "ret":
                    return 1 if b == 0 else 0
                g = st[1]
                if W[g] < b:
                    return 0
                return cnt(g, b)
            v = rec(self.funcs[name].body, budget)
            memo[key] = v
            return v
        return cnt(self.root, W[self.root])

    def count_leaves(self):
        memo = {}
        for name in reversed(self.topo):
            def rec(st):
                if st[0] == "cmp":
                    return rec(st[3]) + rec(st[4])
                if st[0] == "ret":
                    return 1
                return memo[st[1]]
            memo[name] = rec(self.funcs[name].body)
        return memo[self.root]

    def run(self, values, trace=None):
        """Execute on a list of distinct values (root orientation as in run)."""
        name, xs, d = self.root, list(values), self.root_flip
        ncmp = 0
        while True:
            st = self.funcs[name].body
            while st[0] == "cmp":
                lo, hi = st[1], st[2]
                less = (xs[lo] < xs[hi]) if not d else (xs[lo] > xs[hi])
                if trace is not None:
                    trace.append((name, lo, hi, less))
                ncmp += 1
                st = st[3] if less else st[4]
            if st[0] == "ret":
                return xs[st[1]], ncmp
            _, g, args, flip = st
            xs = [xs[a] for a in args]
            d = d ^ flip
            name = g


def summary(prog):
    f = prog.funcs[prog.root]
    rank1 = f.r + 1 if not prog.root_flip else f.m - f.r
    lines = [
        "file        %s" % prog.path,
        "dialect     %s" % prog.dialect,
        "n, i        %d, %d (1-based)" % (f.m, rank1),
        "worst case  %d" % prog.W[prog.root],
        "functions   %d   call sites %d   comparison sites %d   leaf returns %d"
        % (len(prog.funcs), prog.n_call_sites(), prog.n_cmp_sites(), prog.n_leaf_returns()),
        "violations  %d   doc relations not implied %d   functions with closure(doc)==P_f %d"
        % (len(prog.violations), prog.doc_not_implied, prog.doc_closure_equal),
        "worst paths %d   tree leaves %d" % (prog.count_worst_paths(), prog.count_leaves()),
    ]
    return "\n".join(lines)


if __name__ == "__main__":
    for p in sys.argv[1:]:
        print(summary(Program(p)))
        print()
