#!/usr/bin/env python3
"""
verify_rs_algorithm.py -- independent correctness checker for exact selection
algorithms printed as Rust code (format of selection_generator, Doerrer et al.).

    python3 -I verify_rs_algorithm.py FILE.rs [--from-filename]
            [--expect-n N] [--expect-i I1] [--expect-worst W] [--json]

Prints ACCEPT or REJECT, n, i (1-based and 0-based), the certified worst-case
number of comparisons, statistics, and on REJECT the first violation.
Exit status: 0 = ACCEPT, 1 = REJECT, 2 = usage / I/O error.

This file is written from the mathematical definition of selection only; it
does not use any code of the generator.  README.txt contains the grammar, the
checking rules and the soundness proof.  Structure of this file:

    1. parser        text  -> Program (strict line grammar, rejects the rest)
    2. linker        names -> positions / Function objects, the checks the Rust
                     compiler would do (arity, unknown names, ...)
    3. checker       per-function preconditions (posets), local rules,
                     longest-path comparison count
    4. command line
"""

import argparse
import json
import os
import re
import sys
import time


class Reject(Exception):
    """First violation found.  stage is PARSE, LINK or CHECK."""

    def __init__(self, stage, line, msg):
        super().__init__(msg)
        self.stage, self.line, self.msg = stage, line, msg

    def __str__(self):
        where = f"line {self.line}: " if self.line else ""
        return f"[{self.stage}] {where}{self.msg}"


# =========================================================================
# 1. Parser
# =========================================================================
#
# Two dialects are accepted (one per file; mixing is rejected by the linker):
#
#   DUAL dialect  (upstream backward search; files logs/backward/algorithms)
#       fn select_K([a, b, ..]: [usize; M], is_dual: bool) -> usize {
#           if (!is_dual && X < Y) || (is_dual && X > Y) { STMT } else { STMT }
#       }
#       STMT = `select_K([args], is_dual)` | `select_K([args], !is_dual)` | `p`
#
#   PLAIN dialect (upstream forward search)
#       fn select_K([a, b, ..]: [usize; M]) -> usize {
#           if X < Y { STMT } else { STMT }                      or
#           let PAT = if X < Y { TUPLE } else { TUPLE }; STMT    or  STMT
#       }
#       STMT = `select_K([args])` | `p`;  PAT/TUPLE = `p` or `(p, q, ..)`
#
# A body may also be a single STMT.  Both dialects are translated into one
# model.  Positions are 0-based indices into a function's parameter list.

class Leaf:
    """Return the parameter `name`."""
    __slots__ = ("name", "pos", "line")

    def __init__(self, name, line):
        self.name, self.pos, self.line = name, None, line


class Call:
    """Call callee_name with the parameters arg_names; flip = passes !is_dual."""
    __slots__ = ("callee_name", "arg_names", "flip", "callee", "args", "line")

    def __init__(self, callee_name, arg_names, flip, line):
        self.callee_name, self.arg_names, self.flip = callee_name, arg_names, flip
        self.callee, self.args, self.line = None, None, line


class If:
    """Comparison of the parameters X and Y.  In the function's orientation
    <_d (< if is_dual is false or absent, > if is_dual is true) the THEN
    statement runs iff X <_d Y.  THEN and ELSE are a Leaf or a Call."""
    __slots__ = ("x_name", "y_name", "x", "y", "then", "els", "line")

    def __init__(self, x_name, y_name, then, els, line):
        self.x_name, self.y_name, self.then, self.els = x_name, y_name, then, els
        self.x = self.y = None
        self.line = line


class Function:
    __slots__ = ("name", "line", "params", "arity", "doc_n", "doc_i", "claims",
                 "dual_param", "body", "claim_pos")

    def __init__(self, name, line, params, arity, doc_n, doc_i, claims, dual_param, body):
        self.name, self.line, self.params, self.arity = name, line, params, arity
        self.doc_n, self.doc_i, self.claims = doc_n, doc_i, claims
        self.dual_param = dual_param     # "is_dual", "_" (DUAL dialect) or None (PLAIN)
        self.body = body
        self.claim_pos = None


class Program:
    def __init__(self):
        self.functions = {}       # name -> Function, in file order
        self.const_n = None       # optional `pub const N: usize = ..;`
        self.const_i = None       # optional `pub const I: usize = ..;`
        self.run = None           # optional (callee name, dual literal or None, n, line)
        self.root = None          # set by link()
        self.root_dual = False    # orientation the root is called with


IDENT = r"[a-z][a-z0-9_]*"
IDLIST = rf"{IDENT}(?:\s*,\s*{IDENT})*"
FNAME = r"select_[0-9]+"
A = re.ASCII          # \s means exactly Rust's ASCII whitespace

RE_ATTR = re.compile(r"#\s*\[\s*allow\s*\(\s*[a-z_]+(?:\s*,\s*[a-z_]+)*\s*\)\s*\]", A)
RE_CONST = re.compile(r"pub\s+const\s+(N|I)\s*:\s*usize\s*=\s*([0-9]+)\s*;", A)
RE_RUN_SIG = re.compile(
    r"pub\s+fn\s+run\s*\(\s*input\s*:\s*\[\s*usize\s*;\s*([0-9]+)\s*\]\s*\)\s*->\s*usize\s*\{", A)
RE_RUN_CALL = re.compile(rf"({FNAME})\s*\(\s*input\s*(?:,\s*(false|true)\s*)?\)", A)
RE_DOC = re.compile(rf"///\s*n\s*=\s*([0-9]+)(?:\s*,\s*i\s*=\s*([0-9]+))?"
                    rf"((?:\s*,\s*{IDENT}\s*<\s*{IDENT})*)", A)
RE_CLAIM = re.compile(rf"({IDENT})\s*<\s*({IDENT})", A)
RE_SIG = re.compile(
    rf"fn\s+({FNAME})\s*\(\s*\[\s*({IDLIST})\s*\]\s*:\s*\[\s*usize\s*;\s*([0-9]+)\s*\]"
    rf"\s*(?:,\s*(is_dual|_)\s*:\s*bool\s*)?\)\s*->\s*usize\s*\{{", A)
RE_IF_DUAL = re.compile(
    rf"if\s*\(\s*!\s*is_dual\s*&&\s*({IDENT})\s*<\s*({IDENT})\s*\)\s*\|\|"
    rf"\s*\(\s*is_dual\s*&&\s*({IDENT})\s*>\s*({IDENT})\s*\)\s*\{{", A)
RE_IF_PLAIN = re.compile(rf"if\s+({IDENT})\s*<\s*({IDENT})\s*\{{", A)
RE_LET = re.compile(rf"let(?:\s+({IDENT})|\s*(\(\s*{IDLIST}\s*\)))\s*=\s*if\s+({IDENT})\s*<\s*({IDENT})\s*\{{", A)
RE_VALUE = re.compile(rf"{IDENT}|\(\s*{IDLIST}\s*\)", A)
RE_LET_END = re.compile(r"\}\s*;", A)
RE_ELSE = re.compile(r"\}\s*else\s*\{", A)
RE_CLOSE = re.compile(r"\}", A)
RE_CALL_DUAL = re.compile(rf"({FNAME})\s*\(\s*\[\s*({IDLIST})\s*\]\s*,\s*(!\s*is_dual|is_dual)\s*\)", A)
RE_CALL_PLAIN = re.compile(rf"({FNAME})\s*\(\s*\[\s*({IDLIST})\s*\]\s*\)", A)
RE_LEAF = re.compile(IDENT, A)

# Names that may not be used as variable names (keywords / names used by
# the grammar itself); names of the form select_K are rejected as well.
RESERVED = {"is_dual", "if", "else", "fn", "let", "mut", "return", "true",
            "false", "usize", "bool", "pub", "const", "as", "in", "for",
            "while", "loop", "match", "self", "super", "crate", "input", "run"}
WS = " \t\r\x0b\x0c"           # ASCII whitespace other than newline


def bad_name(name):
    return name in RESERVED or re.fullmatch(FNAME, name) is not None


def _split_names(s):
    return [t.strip(WS) for t in s.strip("()" + WS).split(",")]


def parse(text):
    """Parse the whole file.  Every non-blank line must be consumed by the
    grammar; anything else raises Reject('PARSE', ...)."""
    prog = Program()
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    pos = 0                         # index of next unread line
    nlines = len(lines)

    def peek():
        return lines[pos].strip(WS) if pos < nlines else ""

    def take(regex, what):
        """Consume the next line, which must fully match regex."""
        nonlocal pos
        if pos >= nlines:
            raise Reject("PARSE", nlines, f"unexpected end of file, expected {what}")
        s = lines[pos].strip(WS)
        m = regex.fullmatch(s)
        if m is None:
            raise Reject("PARSE", pos + 1, f"expected {what}, found {s[:80]!r}")
        pos += 1
        return m

    def stmt(dual):
        """A Leaf or a Call on one line."""
        nonlocal pos
        s, line = peek(), pos + 1
        m = (RE_CALL_DUAL if dual else RE_CALL_PLAIN).fullmatch(s)
        if m:
            pos += 1
            flip = dual and m.group(3).startswith("!")          # `!is_dual` (any whitespace)
            return Call(m.group(1), _split_names(m.group(2)), flip, line)
        if RE_LEAF.fullmatch(s) and not bad_name(s):
            pos += 1
            return Leaf(s, line)
        form = "select_K([..], is_dual|!is_dual)" if dual else "select_K([..])"
        raise Reject("PARSE", line, f"expected a call `{form}` or a parameter name, found {s[:80]!r}")

    def substitute(st, binding, line):
        """STMT after `let PAT = TUPLE;`: names bound by PAT denote the tuple's values."""
        if isinstance(st, Leaf):
            return Leaf(binding.get(st.name, st.name), line)
        return Call(st.callee_name, [binding.get(a, a) for a in st.arg_names], st.flip, line)

    def body(dual):
        nonlocal pos
        s, line = peek(), pos + 1
        if dual:
            m = RE_IF_DUAL.fullmatch(s)
            if not m:
                return stmt(dual)
            pos += 1
            x1, y1, x2, y2 = m.groups()
            if x1 != x2 or y1 != y2:
                raise Reject("PARSE", line, "comparison is not of the form "
                             "(!is_dual && X < Y) || (is_dual && X > Y) with the same X, Y")
            then = stmt(dual)
            take(RE_ELSE, "`} else {`")
            els = stmt(dual)
            take(RE_CLOSE, "`}` closing the if/else")
            return If(x1, y1, then, els, line)
        m = RE_IF_PLAIN.fullmatch(s)
        if m:
            pos += 1
            then = stmt(dual)
            take(RE_ELSE, "`} else {`")
            els = stmt(dual)
            take(RE_CLOSE, "`}` closing the if/else")
            return If(m.group(1), m.group(2), then, els, line)
        m = RE_LET.fullmatch(s)
        if not m:
            return stmt(dual)
        pos += 1
        pattern = _split_names(m.group(1) or m.group(2))
        if len(set(pattern)) != len(pattern) or any(bad_name(p) for p in pattern):
            raise Reject("PARSE", line, "let pattern with repeated or reserved names")
        then_line = pos + 1
        then_vals = _split_names(take(RE_VALUE, "a name or tuple `(..)`").group(0))
        take(RE_ELSE, "`} else {`")
        else_line = pos + 1
        else_vals = _split_names(take(RE_VALUE, "a name or tuple `(..)`").group(0))
        take(RE_LET_END, "`};` closing the let")
        if not len(then_vals) == len(else_vals) == len(pattern):
            raise Reject("PARSE", line, "let pattern and branch tuples have different lengths")
        after = stmt(dual)
        return If(m.group(3), m.group(4),
                  substitute(after, dict(zip(pattern, then_vals)), then_line),
                  substitute(after, dict(zip(pattern, else_vals)), else_line), line)

    while pos < nlines:
        s, line = peek(), pos + 1
        if s == "" or RE_ATTR.fullmatch(s):
            pos += 1
            continue
        m = RE_CONST.fullmatch(s)
        if m:
            pos += 1
            attr = "const_n" if m.group(1) == "N" else "const_i"
            if getattr(prog, attr) is not None:
                raise Reject("PARSE", line, f"duplicate constant {m.group(1)}")
            setattr(prog, attr, int(m.group(2)))
            continue
        m = RE_RUN_SIG.fullmatch(s)
        if m:
            pos += 1
            if prog.run is not None:
                raise Reject("PARSE", line, "duplicate fn run")
            mc = take(RE_RUN_CALL, "`select_K(input)` or `select_K(input, false|true)` in fn run")
            take(RE_CLOSE, "`}` closing fn run")
            dual_lit = None if mc.group(2) is None else mc.group(2) == "true"
            prog.run = (mc.group(1), dual_lit, int(m.group(1)), line)
            continue
        m = RE_DOC.fullmatch(s)
        if not m:
            raise Reject("PARSE", line, f"expected a doc comment `/// n = .., i = ..` starting a "
                         f"function (or a header item), found {s[:80]!r}")
        pos += 1
        doc_n = int(m.group(1))
        doc_i = None if m.group(2) is None else int(m.group(2))
        claims = RE_CLAIM.findall(m.group(3))
        sig = take(RE_SIG, "function signature `fn select_K([..]: [usize; M]{, is_dual: bool}) -> usize {`")
        sig_line = pos
        name, dual_param = sig.group(1), sig.group(4)
        b = body(dual_param is not None)
        take(RE_CLOSE, f"`}}` closing fn {name} (nested or additional statements are not supported)")
        if name in prog.functions:
            raise Reject("PARSE", sig_line, f"duplicate function name {name}")
        prog.functions[name] = Function(name, sig_line, _split_names(sig.group(2)), int(sig.group(3)),
                                        doc_n, doc_i, claims, dual_param, b)
    if not prog.functions:
        raise Reject("PARSE", 0, "no functions in file")
    return prog


# =========================================================================
# 2. Linker: everything the Rust compiler would also reject, plus the
#    well-formedness of the doc comments, plus the choice of the root.
# =========================================================================

def call_sites(f):
    """The Call statements of f (0, 1 or 2 of them)."""
    b = f.body
    stmts = [b.then, b.els] if isinstance(b, If) else [b]
    return [s for s in stmts if isinstance(s, Call)]


def link(prog):
    funcs = prog.functions
    dialects = {f.dual_param is None for f in funcs.values()}
    if len(dialects) != 1:
        raise Reject("LINK", 0, "file mixes functions with and without an is_dual parameter")
    plain = dialects.pop()
    for f in funcs.values():
        if len(f.params) != f.arity:
            raise Reject("LINK", f.line, f"{f.name}: {len(f.params)} parameter names but array length {f.arity}")
        if len(set(f.params)) != len(f.params):
            raise Reject("LINK", f.line, f"{f.name}: duplicate parameter names")
        for p in f.params:
            if bad_name(p):
                raise Reject("LINK", f.line, f"{f.name}: reserved parameter name {p!r}")
        if f.doc_n != f.arity:
            raise Reject("LINK", f.line, f"{f.name}: doc comment says n = {f.doc_n} but the function has {f.arity} parameters")
        if f.doc_i is None:
            if f.doc_n != 1:
                raise Reject("LINK", f.line, f"{f.name}: doc comment lacks the rank i")
            f.doc_i = 0
        if not 0 <= f.doc_i < f.doc_n:
            raise Reject("LINK", f.line, f"{f.name}: doc comment rank i = {f.doc_i} out of range for n = {f.doc_n}")
        index = {p: k for k, p in enumerate(f.params)}

        def resolve(name, line):
            if name not in index:
                raise Reject("LINK", line, f"{f.name}: unknown name {name!r}")
            return index[name]

        f.claim_pos = [(resolve(u, f.line - 1), resolve(v, f.line - 1)) for u, v in f.claims]
        b = f.body
        if f.dual_param == "_" and not isinstance(b, Leaf):
            raise Reject("LINK", f.line, f"{f.name}: body uses is_dual but the parameter is `_`")
        stmts = [b]
        if isinstance(b, If):
            b.x = resolve(b.x_name, b.line)
            b.y = resolve(b.y_name, b.line)
            stmts = [b.then, b.els]
        for s in stmts:
            if isinstance(s, Leaf):
                s.pos = resolve(s.name, s.line)
            else:
                if s.callee_name not in funcs:
                    raise Reject("LINK", s.line, f"{f.name}: call of undefined function {s.callee_name}")
                s.callee = funcs[s.callee_name]
                s.args = tuple(resolve(a, s.line) for a in s.arg_names)
                if len(s.args) != s.callee.arity:
                    raise Reject("LINK", s.line, f"{f.name}: passes {len(s.args)} arguments to "
                                 f"{s.callee.name} which takes {s.callee.arity}")
    # root
    called = {c.callee_name for f in funcs.values() for c in call_sites(f)}
    uncalled = [f for f in funcs.values() if f.name not in called]
    if prog.run is not None:
        rname, rdual, rn, rline = prog.run
        if rname not in funcs:
            raise Reject("LINK", rline, f"run calls undefined function {rname}")
        if (rdual is None) != plain:
            raise Reject("LINK", rline, "run passes an is_dual argument iff the functions take one")
        prog.root, prog.root_dual = funcs[rname], bool(rdual)
        if rn != prog.root.arity:
            raise Reject("LINK", rline, f"run takes {rn} inputs but {rname} takes {prog.root.arity}")
        others = [f.name for f in uncalled if f is not prog.root]
        if others or prog.root.name in called:
            raise Reject("LINK", rline, "run's callee must be the only function that is never called; "
                         f"never-called: {[f.name for f in uncalled][:5]}")
    else:
        if len(uncalled) != 1:
            raise Reject("LINK", 0, "cannot determine the root: expected exactly one function that is "
                         f"never called, found {len(uncalled)}: {[f.name for f in uncalled][:5]}")
        prog.root, prog.root_dual = uncalled[0], False
    return prog


def root_rank(prog):
    """0-based rank (in the normal order <) that the whole program claims to
    select: the root's doc rank, mirrored if run calls it with is_dual=true."""
    r = prog.root.doc_i
    return prog.root.arity - 1 - r if prog.root_dual else r


# =========================================================================
# 3. Checker
# =========================================================================
#
# Posets.  A strict partial order on positions 0..m-1 is stored as a list
# `above` of m bitmasks: bit v of above[u] is set iff u < v.  All posets
# handled here are transitively closed.  Orders are always meant in the
# orientation of the function they belong to (see README).

def popcount(x):
    return bin(x).count("1")


def below_masks(above):
    m = len(above)
    below = [0] * m
    for u in range(m):
        a = above[u]
        for v in range(m):
            if (a >> v) & 1:
                below[v] |= 1 << u
    return below


def add_relation(above, lo, hi):
    """Closure of (above + {lo < hi}); `above` closed, lo and hi incomparable.
    New pairs are exactly (u, v) with u <= lo and hi <= v."""
    up_hi = above[hi] | (1 << hi)
    new = list(above)
    for u in range(len(above)):
        if u == lo or (above[u] >> lo) & 1:
            new[u] |= up_hi
    return new


def restrict(above, args, flip):
    """Relations among the arguments, in the callee's positions 0..len(args)-1.
    If flip (call with !is_dual), the callee's order is the reverse one, so
    every pair is reversed."""
    k = len(args)
    out = [0] * k
    for j in range(k):
        a = above[args[j]]
        for l in range(k):
            if (a >> args[l]) & 1:
                if flip:
                    out[l] |= 1 << j        # args[j] < args[l]  becomes  l < j
                else:
                    out[j] |= 1 << l        # args[j] < args[l]  stays    j < l
    return out


def is_strict_partial_order(above):
    """Sanity check of an internal invariant (irreflexive + transitive)."""
    for u, a in enumerate(above):
        if (a >> u) & 1:
            return False
        for v in range(len(above)):
            if (a >> v) & 1 and above[v] & ~a:
                return False
    return True


def topological_order(prog):
    """Reachable functions, every caller before its callees.  Rejects
    unreachable functions and cycles in the call graph."""
    funcs = prog.functions
    seen = {prog.root.name}
    stack = [prog.root]
    while stack:
        f = stack.pop()
        for c in call_sites(f):
            if c.callee.name not in seen:
                seen.add(c.callee.name)
                stack.append(c.callee)
    if len(seen) != len(funcs):
        dead = [n for n in funcs if n not in seen]
        raise Reject("CHECK", funcs[dead[0]].line,
                     f"{len(dead)} function(s) unreachable from the root {prog.root.name}: {dead[:5]}")
    indeg = {name: 0 for name in funcs}
    for f in funcs.values():
        for c in call_sites(f):
            indeg[c.callee.name] += 1
    order, queue = [], [prog.root]
    while queue:
        f = queue.pop()
        order.append(f)
        for c in call_sites(f):
            indeg[c.callee.name] -= 1
            if indeg[c.callee.name] == 0:
                queue.append(c.callee)
    if len(order) != len(funcs):
        bad = [n for n in funcs if indeg[n] > 0]
        raise Reject("CHECK", funcs[bad[0]].line, f"the call graph has a cycle (involving e.g. {bad[:5]})")
    return order


def check_stmt(f, Q, stmt, pre):
    """Check a Leaf or Call of f under the known relations Q (closed poset on
    f's positions, f's orientation).  For a Call, intersect the relations it
    guarantees into the callee's precondition pre[callee]."""
    m, r = f.arity, f.doc_i
    below = below_masks(Q)
    if isinstance(stmt, Leaf):
        p = stmt.pos
        nb, na = popcount(below[p]), popcount(Q[p])
        if nb != r or na != m - 1 - r:
            raise Reject("CHECK", stmt.line,
                         f"{f.name}: returns {f.params[p]} which is not certainly of rank {r} "
                         f"among {m}: known {nb} below and {na} above (need {r} and {m - 1 - r})")
        return
    g, args = stmt.callee, stmt.args
    if len(set(args)) != len(args):
        raise Reject("CHECK", stmt.line, f"{f.name}: call of {g.name} with a repeated argument")
    passed = set(args)
    k_below = 0
    for e in range(m):
        if e in passed:
            continue
        na, nb = popcount(Q[e]), popcount(below[e])
        if na >= m - r:          # >= m-r elements above e: rank(e) <= r-1
            k_below += 1
        elif nb >= r + 1:        # >= r+1 elements below e: rank(e) >= r+1
            pass
        else:
            raise Reject("CHECK", stmt.line,
                         f"{f.name} (rank {r} of {m}): drops {f.params[e]} which is not certainly "
                         f"below or above the answer: known {na} above (need >= {m - r}) and "
                         f"{nb} below (need >= {r + 1})")
    k = len(args)
    r_callee = r - k_below
    if stmt.flip:
        r_callee = k - 1 - r_callee
    if g.doc_i != r_callee:
        raise Reject("CHECK", stmt.line,
                     f"{f.name}: the answer has rank {r_callee} among the {k} arguments"
                     f"{' in the flipped order' if stmt.flip else ''}, but {g.name} claims to select "
                     f"rank {g.doc_i}")
    rel = restrict(Q, args, stmt.flip)
    old = pre.get(g.name)
    pre[g.name] = rel if old is None else [a & b for a, b in zip(old, rel)]


def check(prog, strict_claims=False):
    """Static check.  Returns a dict of results; raises Reject on the first
    violation.  Relations claimed in doc comments (`/// n = 4, i = 1, a < c`)
    are not used; a claim not implied by the computed precondition is
    reported as a warning (or rejected with strict_claims)."""
    order = topological_order(prog)
    pre = {prog.root.name: [0] * prog.root.arity}     # root: no precondition
    stats = {"functions": len(order), "call_sites": 0, "flipped_calls": 0,
             "comparison_sites": 0, "leaf_returns": 0, "max_arity": 0,
             "claims": 0, "claims_not_implied": 0, "warnings": []}
    for f in order:
        P = pre.pop(f.name)       # complete: all callers were processed before f
        if not is_strict_partial_order(P):          # cannot happen; see README
            raise Reject("CHECK", f.line, f"internal error: precondition of {f.name} not a poset")
        stats["max_arity"] = max(stats["max_arity"], f.arity)
        for u, v in f.claim_pos:
            stats["claims"] += 1
            if not (P[u] >> v) & 1:
                stats["claims_not_implied"] += 1
                msg = (f"line {f.line - 1}: {f.name}: doc comment claims {f.params[u]} < {f.params[v]}, "
                       f"which the callers do not guarantee")
                if strict_claims:
                    raise Reject("CHECK", f.line - 1, msg.split(": ", 1)[1])
                if len(stats["warnings"]) < 20:
                    stats["warnings"].append(msg)
        b = f.body
        if isinstance(b, If):
            x, y = b.x, b.y
            stats["comparison_sites"] += 1
            if x == y:
                raise Reject("CHECK", b.line, f"{f.name}: compares {f.params[x]} with itself")
            if (P[x] >> y) & 1 or (P[y] >> x) & 1:
                raise Reject("CHECK", b.line,
                             f"{f.name}: compares {f.params[x]} and {f.params[y]} whose order is already "
                             f"known from the precondition (wasted comparison, dead branch)")
            branches = [(add_relation(P, x, y), b.then), (add_relation(P, y, x), b.els)]
        else:
            branches = [(P, b)]
        for Q, stmt in branches:
            if isinstance(stmt, Call):
                stats["call_sites"] += 1
                stats["flipped_calls"] += stmt.flip
            else:
                stats["leaf_returns"] += 1
            check_stmt(f, Q, stmt, pre)
    assert not pre
    # Worst case = maximum number of comparisons on a root-to-leaf path of
    # the call DAG; computed callees first.
    cost = {}
    for f in reversed(order):
        b = f.body
        stmts = [b.then, b.els] if isinstance(b, If) else [b]
        cost[f.name] = (1 if isinstance(b, If) else 0) + max(
            cost[s.callee.name] if isinstance(s, Call) else 0 for s in stmts)
    stats["worst_case"] = cost[prog.root.name]
    return stats


# =========================================================================
# 4. Driver
# =========================================================================

def verify_text(text, expect_n=None, expect_i0=None, expect_worst=None, strict_claims=False):
    """Returns a result dict.  result['accepted'] is True iff every check
    passed (including the optional expectations)."""
    res = {"accepted": False, "n": None, "i0": None, "i1": None, "worst_case": None,
           "functions": None, "error": None}
    try:
        prog = link(parse(text))
        n, i0 = prog.root.arity, root_rank(prog)
        res.update(n=n, i0=i0, i1=i0 + 1)
        if prog.const_n is not None and prog.const_n != n:
            raise Reject("CHECK", 0, f"const N = {prog.const_n} but the root has {n} parameters")
        if prog.const_i is not None and prog.const_i != i0:
            raise Reject("CHECK", 0, f"const I = {prog.const_i} but the root selects rank {i0}")
        stats = check(prog, strict_claims)
        res.update(stats)
        if expect_n is not None and expect_n != n:
            raise Reject("CHECK", 0, f"expected n = {expect_n}, the file selects from n = {n}")
        if expect_i0 is not None and expect_i0 != i0:
            raise Reject("CHECK", 0, f"expected i = {expect_i0 + 1} (1-based), the file selects i = {i0 + 1}")
        if expect_worst is not None and stats["worst_case"] > expect_worst:
            raise Reject("CHECK", 0, f"worst case {stats['worst_case']} exceeds the expected {expect_worst}")
        res["accepted"] = True
    except Reject as e:
        res["error"] = str(e)
        res["stage"] = e.stage
    return res


def main(argv=None):
    ap = argparse.ArgumentParser(description="Independent checker for selection algorithms in generated Rust.")
    ap.add_argument("file")
    ap.add_argument("--from-filename", action="store_true",
                    help="file is named {n}_{i}.rs with 0-BASED i (upstream convention); check n and i")
    ap.add_argument("--expect-n", type=int)
    ap.add_argument("--expect-i", type=int, help="expected i, 1-BASED (i-th smallest)")
    ap.add_argument("--expect-worst", type=int, help="require worst case <= this")
    ap.add_argument("--strict-claims", action="store_true",
                    help="reject (instead of warn) if a relation claimed in a doc comment is not implied")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args(argv)
    expect_n = a.expect_n
    expect_i0 = None if a.expect_i is None else a.expect_i - 1
    if a.from_filename:
        m = re.fullmatch(r"([0-9]+)_([0-9]+)\.rs", os.path.basename(a.file))
        if not m:
            print(f"error: file name {os.path.basename(a.file)!r} is not of the form n_i.rs", file=sys.stderr)
            return 2
        expect_n, expect_i0 = int(m.group(1)), int(m.group(2))
    try:
        with open(a.file, "r", encoding="ascii") as fh:
            text = fh.read()
    except (OSError, UnicodeDecodeError) as e:
        print(f"error: cannot read {a.file}: {e}", file=sys.stderr)
        return 2
    t0 = time.time()
    res = verify_text(text, expect_n, expect_i0, a.expect_worst, a.strict_claims)
    res["seconds"] = round(time.time() - t0, 3)
    res["file"] = a.file
    if a.json:
        print(json.dumps(res))
    else:
        verdict = "ACCEPT" if res["accepted"] else "REJECT"
        print(f"{verdict}  {a.file}")
        if res["n"] is not None:
            print(f"  n = {res['n']}, i = {res['i1']} (1-based; 0-based rank {res['i0']})")
        if res.get("worst_case") is not None:
            print(f"  worst-case comparisons = {res['worst_case']}")
            print(f"  functions = {res['functions']}, call sites = {res['call_sites']} "
                  f"({res['flipped_calls']} with !is_dual), comparison sites = {res['comparison_sites']}, "
                  f"leaf returns = {res['leaf_returns']}, max arity = {res['max_arity']}")
            print(f"  doc-comment relations: {res['claims']} claimed, "
                  f"{res['claims_not_implied']} not implied by the computed preconditions"
                  + (" (not used for correctness)" if res['claims'] else ""))
            for w in res["warnings"]:
                print(f"  WARNING {w}")
        if res["error"]:
            print(f"  first violation: {res['error']}")
        print(f"  time = {res['seconds']} s")
    return 0 if res["accepted"] else 1


if __name__ == "__main__":
    sys.exit(main())
