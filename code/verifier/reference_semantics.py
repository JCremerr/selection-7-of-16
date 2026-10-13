#!/usr/bin/env python3
"""
reference_semantics.py -- TEST ORACLE for verify_rs_algorithm.py.

Executes a generated Rust selection algorithm literally (own, separate parser
and interpreter; no code shared with verify_rs_algorithm.py) and decides
correctness by exhaustion, in two independent ways:

  brute_force(prog)  runs the program on all n! permutations of 0..n-1 and
                     checks that it returns the claimed rank (for a
                     permutation of 0..n-1 the element of rank I is the value
                     I).  This is exactly what upstream's algorithm_test.rs does.
  path_oracle(prog)  explores every feasible execution path symbolically,
                     keeping the poset of comparison outcomes on the n inputs;
                     a path is correct iff the returned input has rank I in
                     every linear extension of that poset.  Exact (equivalent
                     to brute force), cost ~ #paths instead of n!.

Both report the exact worst-case number of comparisons executed.  Variables
are kept in environments (name -> value) and `let` rebinding is executed as
in Rust: the chosen tuple is evaluated in the old environment, then the
pattern names are (re)bound.  The parser accepts only the exact layout the
generator prints (both dialects); it is a cross-check, not the trust anchor.

    python3 -I reference_semantics.py FILE.rs [--brute] [--paths]
"""

import itertools
import re
import sys

W = r"[a-z][a-z0-9_]*"
FN_RE = re.compile(
    rf"/// n = (\d+)(?:, i = (\d+))?((?:, {W} < {W})*)\n"
    rf"fn (select_\d+)\(\[([a-z0-9_, ]*)\]: \[usize; (\d+)\](?:, (is_dual|_): bool)?\) -> usize \{{\n"
    r"(.*?)\n\}\n", re.S)
RUN_RE = re.compile(r"pub fn run\(input: \[usize; (\d+)\]\) -> usize \{\n *(select_\d+)\(input(?:, (false|true))?\)\n\}\n")
CONST_RE = re.compile(r"pub const (N|I): usize = (\d+);\n")
ATTR_RE = re.compile(r"#\[allow\([a-z_, ]+\)\]\n")
IF_DUAL_RE = re.compile(rf"if \(!is_dual && ({W}) < ({W})\) \|\| \(is_dual && ({W}) > ({W})\) \{{")
IF_PLAIN_RE = re.compile(rf"if ({W}) < ({W}) \{{")
LET_RE = re.compile(rf"let ({W}|\([a-z0-9_, ]+\)) = if ({W}) < ({W}) \{{")
TUPLE_RE = re.compile(rf"{W}|\([a-z0-9_, ]+\)")
CALL_DUAL_RE = re.compile(r"(select_\d+)\(\[([a-z0-9_, ]*)\], (!?is_dual)\)")
CALL_PLAIN_RE = re.compile(r"(select_\d+)\(\[([a-z0-9_, ]*)\]\)")
RET_RE = re.compile(W)


class IllFormed(Exception):
    """The Rust compiler would reject the program (or we cannot read it)."""


class RefProgram:
    pass


def _names(s):
    return [t.strip() for t in s.strip("()").split(",")]


def load(text):
    """funcs[name] = (doc_n, doc_i, params, has_dual, body) where body is
         ('stmt', S) | ('if', cond, S_then, S_else)
       | ('let', pattern, cond, then_values, else_values, S_after)
    S = ('ret', name) | ('call', callee, [names], flip)   (flip None: no is_dual argument)
    cond = ('lt', A, B)  for `A < B`
         | ('dual', A, B, C, D)  for `(!is_dual && A < B) || (is_dual && C > D)`."""
    rest = text if text.endswith("\n") else text + "\n"
    raw, run, consts = {}, None, {}
    pos = 0
    while pos < len(rest):
        if rest[pos] == "\n":
            pos += 1
            continue
        for rx in (FN_RE, RUN_RE, CONST_RE, ATTR_RE):
            m = rx.match(rest, pos)
            if m:
                break
        else:
            raise IllFormed(f"unparsed text at offset {pos}: {rest[pos:pos + 60]!r}")
        pos = m.end()
        if rx is FN_RE:
            if m.group(4) in raw:
                raise IllFormed(f"duplicate {m.group(4)}")
            raw[m.group(4)] = m
        elif rx is RUN_RE:
            run = (int(m.group(1)), m.group(2), m.group(3))
        elif rx is CONST_RE:
            consts[m.group(1)] = int(m.group(2))
    sig = {name: (int(m.group(6)), m.group(7)) for name, m in raw.items()}   # arity, dual param
    prog = RefProgram()
    prog.funcs = {}
    for name, m in raw.items():
        params = _names(m.group(5))
        dual_param = m.group(7)
        if len(params) != sig[name][0] or len(set(params)) != len(params):
            raise IllFormed(f"{name}: bad parameter list")

        def need(v, scope):
            if v not in scope:
                raise IllFormed(f"{name}: unknown variable {v}")
            return v

        def stmt(s, scope):
            c = CALL_DUAL_RE.fullmatch(s) or CALL_PLAIN_RE.fullmatch(s)
            if c:
                callee = c.group(1)
                args = [need(a, scope) for a in _names(c.group(2))]
                flip = None if c.re is CALL_PLAIN_RE else c.group(3) == "!is_dual"
                if callee not in sig:
                    raise IllFormed(f"{name}: unknown function {callee}")
                if len(args) != sig[callee][0]:
                    raise IllFormed(f"{name}: wrong number of arguments for {callee}")
                if (flip is None) != (sig[callee][1] is None):
                    raise IllFormed(f"{name}: is_dual argument does not match {callee}'s signature")
                if flip is not None and dual_param != "is_dual":
                    raise IllFormed(f"{name}: uses is_dual without having it")
                return ("call", callee, args, flip)
            if RET_RE.fullmatch(s):
                return ("ret", need(s, scope))
            raise IllFormed(f"{name}: cannot read statement {s!r}")

        def cond_of(line):
            c = IF_DUAL_RE.fullmatch(line)
            if c:
                if dual_param != "is_dual":
                    raise IllFormed(f"{name}: uses is_dual without having it")
                return ("dual",) + tuple(need(v, params) for v in c.groups())
            c = IF_PLAIN_RE.fullmatch(line)
            if c:
                return ("lt",) + tuple(need(v, params) for v in c.groups())
            return None

        body = [b.strip() for b in m.group(8).split("\n")]
        if len(body) == 1:
            fbody = ("stmt", stmt(body[0], params))
        elif len(body) == 5 and cond_of(body[0]) and body[2] == "} else {" and body[4] == "}":
            fbody = ("if", cond_of(body[0]), stmt(body[1], params), stmt(body[3], params))
        elif len(body) == 6 and LET_RE.fullmatch(body[0]) and body[2] == "} else {" and body[4] == "};" \
                and TUPLE_RE.fullmatch(body[1]) and TUPLE_RE.fullmatch(body[3]):
            c = LET_RE.fullmatch(body[0])
            pattern = _names(c.group(1))
            tv = [need(v, params) for v in _names(body[1])]
            ev = [need(v, params) for v in _names(body[3])]
            if len(set(pattern)) != len(pattern) or not len(pattern) == len(tv) == len(ev):
                raise IllFormed(f"{name}: bad let")
            cond = ("lt", need(c.group(2), params), need(c.group(3), params))
            fbody = ("let", pattern, cond, tv, ev, stmt(body[5], params + pattern))
        else:
            raise IllFormed(f"{name}: cannot read body")
        doc_i = m.group(2)
        if doc_i is None and int(m.group(1)) != 1:
            raise IllFormed(f"{name}: no rank in doc comment")
        prog.funcs[name] = (int(m.group(1)), int(doc_i or 0), params, dual_param is not None, fbody)
    if run is not None:
        prog.root, prog.root_dual = run[1], run[2] == "true"
        if prog.root not in prog.funcs or sig[prog.root][0] != run[0] \
                or (run[2] is None) != (sig[prog.root][1] is None):
            raise IllFormed("bad run wrapper")
    else:
        prog.root, prog.root_dual = "select_0", False      # upstream convention
        if prog.root not in prog.funcs:
            raise IllFormed("no select_0")
    prog.n = sig[prog.root][0]
    doc_i = prog.funcs[prog.root][1]
    prog.rank = prog.n - 1 - doc_i if prog.root_dual else doc_i
    if "I" in consts:
        prog.rank = consts["I"]
    prog.cyclic = _has_reachable_cycle(prog)
    return prog


def _callees(body):
    stmts = {"stmt": body[1:2], "if": body[2:4], "let": body[5:6]}[body[0]]
    return [s[1] for s in stmts if s[0] == "call"]


def _has_reachable_cycle(prog):
    """Is there a cycle in the call graph reachable from the root?"""
    state = {prog.root: 1}                       # 1 = on the DFS stack, 2 = done
    stack = [(prog.root, iter(_callees(prog.funcs[prog.root][4])))]
    while stack:
        name, it = stack[-1]
        nxt = next(it, None)
        if nxt is None:
            state[name] = 2
            stack.pop()
        elif state.get(nxt) == 1:
            return True
        elif nxt not in state:
            state[nxt] = 1
            stack.append((nxt, iter(_callees(prog.funcs[nxt][4]))))
    return False


def _lo_hi(cond, env, dual):
    """(lo, hi) such that the condition is true iff lo < hi."""
    if cond[0] == "lt":
        return env[cond[1]], env[cond[2]]
    _, a, b, c, d = cond                      # (!is_dual && a < b) || (is_dual && c > d)
    return (env[a], env[b]) if not dual else (env[d], env[c])


def _branch(body, taken, env):
    """Statement executed after the comparison outcome `taken`, and its environment."""
    if body[0] == "if":
        return (body[2] if taken else body[3]), env
    _, pattern, _, tv, ev, after = body       # let
    values = [env[v] for v in (tv if taken else ev)]   # evaluated in the old environment
    new = dict(env)
    new.update(zip(pattern, values))
    return after, new


def execute(prog, values):
    """Literal execution of run(values).  Returns (result, #comparisons) or
    raises RuntimeError on non-termination.  Execution is deterministic with
    finitely many states (function, arguments, orientation), so it loops
    forever iff a state repeats; states are only recorded for cyclic programs."""
    funcs = prog.funcs
    name, args, dual, count = prog.root, list(values), prog.root_dual, 0
    seen = set()
    while True:
        if prog.cyclic:
            state = (name, tuple(args), dual)
            if state in seen:
                raise RuntimeError("non-terminating (a state repeats)")
            seen.add(state)
        _, _, params, _, body = funcs[name]
        env = dict(zip(params, args))
        if body[0] == "stmt":
            s = body[1]
        else:
            count += 1
            lo, hi = _lo_hi(body[1] if body[0] == "if" else body[2], env, dual)
            s, env = _branch(body, lo < hi, env)
        if s[0] == "ret":
            return env[s[1]], count
        _, name, argnames, flip = s
        args = [env[a] for a in argnames]
        if flip:
            dual = not dual


def brute_force(prog, perms=None):
    """Run on all permutations of 0..n-1 (or on the given ones).  Returns
    (ok, worst, counterexample)."""
    worst = 0
    for perm in (itertools.permutations(range(prog.n)) if perms is None else perms):
        try:
            res, cnt = execute(prog, perm)
        except RuntimeError:
            return False, None, (perm, "non-terminating")
        if res != prog.rank:
            return False, None, (perm, res)
        worst = max(worst, cnt)
    return True, worst, None


def _add(above, lo, hi):
    """Transitive closure after adding lo < hi (both are original inputs)."""
    up = above[hi] | (1 << hi)
    return tuple(a | up if (u == lo or (a >> lo) & 1) else a for u, a in enumerate(above))


def _extension_with_rank_extreme(above, n, e, low):
    """A linear extension (smallest first) of the poset `above` that puts e
    as low (low=True) or as high as possible."""
    below = [0] * n
    for u in range(n):
        for v in range(n):
            if (above[u] >> v) & 1:
                below[v] |= 1 << u
    placed, order = 0, []

    def place_all(allowed):
        nonlocal placed
        progress = True
        while progress:
            progress = False
            for u in range(n):
                if not (placed >> u) & 1 and (allowed >> u) & 1 and below[u] & ~placed == 0:
                    placed |= 1 << u
                    order.append(u)
                    progress = True

    everything = (1 << n) - 1
    place_all(below[e] if low else everything & ~above[e] & ~(1 << e))
    place_all(1 << e)
    place_all(everything)
    assert len(order) == n
    return order


def path_oracle(prog):
    """Exhaustive symbolic execution.  Returns (ok, worst, info): info is a
    counterexample permutation if not ok, else the number of paths.  Returns
    ok = None for cyclic programs (not applicable)."""
    if prog.cyclic:
        return None, None, "cyclic call graph: path oracle not applicable"
    funcs, n, rank = prog.funcs, prog.n, prog.rank
    stack = [(prog.root, tuple(range(n)), prog.root_dual, (0,) * n, 0)]
    worst, paths = 0, 0
    while stack:
        name, elems, dual, above, depth = stack.pop()
        _, _, params, _, body = funcs[name]
        env = dict(zip(params, elems))
        if body[0] == "stmt":
            branches = [(body[1], env, above, depth)]
        else:
            lo, hi = _lo_hi(body[1] if body[0] == "if" else body[2], env, dual)
            branches = []
            if lo != hi and not (above[hi] >> lo) & 1:          # lo < hi is possible
                s, e2 = _branch(body, True, env)
                branches.append((s, e2, _add(above, lo, hi), depth + 1))
            if lo == hi or not (above[lo] >> hi) & 1:           # condition false is possible
                s, e2 = _branch(body, False, env)
                branches.append((s, e2, above if lo == hi else _add(above, hi, lo), depth + 1))
        for s, e2, ab, d in branches:
            if s[0] == "ret":
                paths += 1
                worst = max(worst, d)
                e = e2[s[1]]
                n_above = bin(ab[e]).count("1")
                n_below = sum((ab[u] >> e) & 1 for u in range(n))
                if n_below != rank or n_above != n - 1 - rank:
                    order = _extension_with_rank_extreme(ab, n, e, low=(n_below != rank))
                    perm = [0] * n
                    for value, u in enumerate(order):
                        perm[u] = value
                    return False, None, tuple(perm)
            else:
                _, callee, argnames, flip = s
                stack.append((callee, tuple(e2[a] for a in argnames), dual != bool(flip), ab, d))
    return True, worst, paths


def main():
    with open(sys.argv[1], encoding="ascii") as fh:
        prog = load(fh.read())
    print(f"n = {prog.n}, rank (0-based) = {prog.rank}, functions = {len(prog.funcs)}, cyclic = {prog.cyclic}")
    if "--brute" in sys.argv:
        print("brute force (ok, worst, counterexample):", brute_force(prog))
    if "--paths" in sys.argv:
        print("path oracle (ok, worst, #paths or counterexample):", path_oracle(prog))


if __name__ == "__main__":
    main()
