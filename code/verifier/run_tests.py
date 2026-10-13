#!/usr/bin/env python3
"""
run_tests.py -- test suite for verify_rs_algorithm.py.

    python3 -I run_tests.py [--quick] [--path-max-n N] [--stress DIR] [--out FILE]

  A. unit tests       hand-written programs: every grammar variant of both
                      dialects and every rejection rule
  B. shipped files    all upstream backward-search algorithms: ACCEPT with the
                      known V_i(n); brute force over all n! inputs (n <= 9) and
                      exhaustive path oracle (n <= --path-max-n) must agree
  C. forward files    the certificates under certificates/*/
  D. synthetic        random correct programs of both dialects (make_synthetic)
  E. mutations        systematic and sampled mutants; static verdict vs. the
                      ground truth from reference_semantics.py
  F. stress (opt.)    timing on large synthetic files

Ground truth for D/E comes from reference_semantics.py (own parser, literal
interpreter).  Writes a plain-text report (default: test_results.txt).
"""

import argparse
import glob
import os
import random
import re
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True           # keep this directory free of __pycache__
sys.path.insert(0, HERE)
import verify_rs_algorithm as V          # noqa: E402  (the checker under test)
import reference_semantics as R          # noqa: E402  (independent test oracle)
import make_synthetic as S               # noqa: E402  (random correct programs)

SELECTION = os.path.normpath(os.path.join(HERE, "..", ".."))
DEFAULT_ALG_DIR = os.path.join(SELECTION, "external", "selection_generator", "logs", "backward", "algorithms")
DEFAULT_FORWARD_GLOB = os.path.join(SELECTION, "certificates", "*", "*.rs")


def forward_files(pattern):
    """Forward-search outputs that contain an algorithm.  A search that refutes its
    budget writes a header-only file (run wrapper, no select_ function); skip those."""
    paths = sorted(glob.glob(pattern))
    return [p for p in paths if "fn select_" in open(p, encoding="ascii", errors="replace").read()]

# Known values V_i(n), i 1-based (row n lists i = 1, 2, ...).
KNOWN_ROWS = {
    1: [0], 2: [1], 3: [2, 3], 4: [3, 4], 5: [4, 6, 6], 6: [5, 7, 8],
    7: [6, 8, 10, 10], 8: [7, 9, 11, 12], 9: [8, 11, 12, 14, 14],
    10: [9, 12, 14, 15, 16], 11: [10, 13, 15, 17, 18, 18],
    12: [11, 14, 17, 18, 19, 20], 13: [12, 15, 18, 20, 21, 22, 23],
    14: [13, 16, 19, 21, 23, 24, 25], 15: [14, 17, 20, 23, 24, 26, 26, 27],
    16: [15, 18, 21, 24, 26, 27],
}
KNOWN = {(n, i1): v for n, row in KNOWN_ROWS.items() for i1, v in enumerate(row, 1)}

OUT = []


def say(s=""):
    print(s, flush=True)
    OUT.append(s)


def read(path):
    with open(path, encoding="ascii") as fh:
        return fh.read()


def reference_verdict(text, use_brute, use_paths):
    """Ground truth: CORRECT / WRONG / ILL-FORMED / UNDECIDED, worst case, detail."""
    try:
        prog = R.load(text)
    except R.IllFormed as e:
        return "ILL-FORMED", None, str(e)
    if prog.cyclic and not use_brute:
        # the path oracle needs an acyclic call graph; sample inputs instead
        rng = random.Random(1)
        sample = [tuple(rng.sample(range(prog.n), prog.n)) for _ in range(20000)]
        ok, _, cex = R.brute_force(prog, sample)
        return ("UNDECIDED" if ok else "WRONG"), None, cex
    results = []
    if use_brute:
        results.append(("brute",) + R.brute_force(prog))
    if use_paths and not prog.cyclic:
        ok, worst, info = R.path_oracle(prog)
        if ok is False:      # confirm the symbolic counterexample by running it
            assert R.execute(prog, info)[0] != prog.rank, "path oracle: bogus counterexample"
        results.append(("paths", ok, worst, info))
    if len({r[1] for r in results}) != 1:
        raise AssertionError(f"brute force and path oracle disagree: {results}")
    if results[0][1] and len({r[2] for r in results}) != 1:
        raise AssertionError(f"brute force and path oracle disagree on the worst case: {results}")
    return ("CORRECT" if results[0][1] else "WRONG"), results[0][2], results[0][3]


# =========================================================================
# A. Unit tests
# =========================================================================

def fn(name, n, i, params, body, dual="is_dual", claims=""):
    doc = f"/// n = {n}" + ("" if i is None else f", i = {i}") + claims
    sig_dual = "" if dual is None else f", {dual}: bool"
    return (f"{doc}\nfn {name}([{', '.join(params)}]: [usize; {len(params)}]{sig_dual}) -> usize {{\n"
            + "".join("    " + line + "\n" for line in body) + "}\n")


def cmp(x, y, then, els):
    return [f"if (!is_dual && {x} < {y}) || (is_dual && {x} > {y}) {{", f"    {then}",
            "} else {", f"    {els}", "}"]


def pcmp(x, y, then, els):
    return [f"if {x} < {y} {{", f"    {then}", "} else {", f"    {els}", "}"]


def plet(pat, x, y, tv, ev, after):
    return [f"let {pat} = if {x} < {y} {{", f"    {tv}", "} else {", f"    {ev}", "};", after]


LEAF1 = fn("select_9", 1, 0, ["a"], ["a"], dual="_")
MIN2 = fn("select_8", 2, 0, ["a", "b"], cmp("a", "b", "select_9([a], is_dual)", "select_9([b], is_dual)"))
RUN = ("#[allow(unused)]\npub const N: usize = {n};\n#[allow(unused)]\npub const I: usize = {i};\n\n"
       "#[allow(unused)]\npub fn run(input: [usize; {n}]) -> usize {{\n    select_0(input{d})\n}}\n\n")
PLEAF1 = fn("select_9", 1, None, ["a"], ["a"], dual=None)
PMIN2 = fn("select_8", 2, 0, ["a", "b"], plet("a", "a", "b", "a", "b", "select_9([a])"), dual=None)
PMED3 = fn("select_2", 3, 1, ["a", "b", "c"], pcmp("c", "a", "select_8([a, b])", "select_9([c])"),
           dual=None, claims=", c < b")

UNIT_TESTS = [
    # (name, text, expected verdict, expected worst case or None)
    # --- DUAL dialect
    ("leaf returns inside branches",
     fn("select_0", 2, 0, ["a", "b"], cmp("a", "b", "a", "b")), True, 1),
    ("leaf returns the wrong parameter",
     fn("select_0", 2, 0, ["a", "b"], cmp("a", "b", "b", "a")), False, None),
    ("max of 2 via a !is_dual call to min",
     LEAF1 + MIN2 + fn("select_0", 2, 1, ["a", "b"], ["select_8([a, b], !is_dual)"]), True, 1),
    ("max of 2 via an is_dual call to min (wrong)",
     LEAF1 + MIN2 + fn("select_0", 2, 1, ["a", "b"], ["select_8([a, b], is_dual)"]), False, None),
    ("median of 3, function without comparison that permutes its arguments",
     LEAF1 + MIN2
     + fn("select_2", 3, 1, ["a", "b", "c"], cmp("c", "a", "select_8([a, b], is_dual)", "select_9([c], is_dual)"))
     + fn("select_1", 3, 1, ["a", "b", "c"], ["select_2([b, c, a], is_dual)"])
     + fn("select_0", 3, 1, ["a", "b", "c"], cmp("a", "b", "select_1([a, c, b], is_dual)", "select_1([b, c, a], is_dual)")),
     True, 3),
    ("run wrapper + consts, median of 3",
     RUN.format(n=3, i=1, d=", false") + LEAF1 + MIN2
     + fn("select_2", 3, 1, ["a", "b", "c"], cmp("c", "a", "select_8([a, b], is_dual)", "select_9([c], is_dual)"))
     + fn("select_0", 3, 1, ["a", "b", "c"], cmp("c", "b", "select_2([a, b, c], is_dual)", "select_2([a, c, b], is_dual)")),
     True, 3),
    ("run wrapper with a wrong const I",
     RUN.format(n=3, i=0, d=", false") + LEAF1 + MIN2
     + fn("select_2", 3, 1, ["a", "b", "c"], cmp("c", "a", "select_8([a, b], is_dual)", "select_9([c], is_dual)"))
     + fn("select_0", 3, 1, ["a", "b", "c"], cmp("c", "b", "select_2([a, b, c], is_dual)", "select_2([a, c, b], is_dual)")),
     False, None),
    ("run wrapper calling the root with is_dual = true (min program selects max)",
     RUN.format(n=3, i=2, d=", true") + LEAF1 + MIN2
     + fn("select_0", 3, 0, ["a", "b", "c"], cmp("a", "c", "select_8([a, b], is_dual)", "select_8([b, c], is_dual)")),
     True, 2),
    ("comparison whose outcome is already known (wasted comparison)",
     LEAF1
     + fn("select_1", 2, 0, ["a", "b"], cmp("a", "b", "select_9([a], is_dual)", "select_9([b], is_dual)"))
     + fn("select_0", 2, 0, ["a", "b"], cmp("a", "b", "select_1([a, b], is_dual)", "select_9([b], is_dual)")),
     False, None),
    ("drops an element that may be the answer",
     LEAF1 + MIN2 + fn("select_0", 3, 0, ["a", "b", "c"], cmp("a", "b", "select_8([a, c], is_dual)", "select_9([b], is_dual)")),
     False, None),
    ("repeated argument",
     LEAF1 + MIN2 + fn("select_0", 3, 0, ["a", "b", "c"], cmp("a", "b", "select_8([a, a], is_dual)", "select_8([b, c], is_dual)")),
     False, None),
    ("callee rank annotation inconsistent",
     LEAF1 + fn("select_8", 2, 1, ["a", "b"], cmp("a", "b", "select_9([b], is_dual)", "select_9([a], is_dual)"))
     + fn("select_0", 3, 0, ["a", "b", "c"], cmp("a", "b", "select_8([a, c], is_dual)", "select_8([b, c], is_dual)")),
     False, None),
    ("recursion (cycle in the call graph below the root)",
     fn("select_2", 2, 0, ["a", "b"], ["select_1([b, a], is_dual)"])
     + fn("select_1", 2, 0, ["a", "b"], ["select_2([b, a], is_dual)"])
     + fn("select_0", 2, 0, ["a", "b"], ["select_1([a, b], is_dual)"]), False, None),
    ("unreachable functions (a cycle not reachable from the root)",
     LEAF1 + MIN2 + fn("select_6", 2, 0, ["a", "b"], ["select_5([b, a], is_dual)"])
     + fn("select_5", 2, 0, ["a", "b"], ["select_6([b, a], is_dual)"])
     + fn("select_0", 2, 0, ["a", "b"], ["select_8([a, b], is_dual)"]), False, None),
    ("call with wrong number of arguments",
     LEAF1 + MIN2 + fn("select_0", 3, 0, ["a", "b", "c"], cmp("a", "b", "select_8([a, b, c], is_dual)", "select_8([b, c], is_dual)")),
     False, None),
    ("doc comment n differs from arity",
     LEAF1 + fn("select_0", 3, 0, ["a", "b"], cmp("a", "b", "select_9([a], is_dual)", "select_9([b], is_dual)")), False, None),
    ("`_` parameter but body uses is_dual",
     LEAF1 + fn("select_0", 2, 0, ["a", "b"], cmp("a", "b", "select_9([a], is_dual)", "select_9([b], is_dual)"), dual="_"),
     False, None),
    ("nested if (unsupported construct)",
     LEAF1 + fn("select_0", 2, 0, ["a", "b"], ["if (!is_dual && a < b) || (is_dual && a > b) {",
                                                "    if (!is_dual && a < b) || (is_dual && a > b) {", "        a",
                                                "    } else {", "        b", "    }", "} else {", "    b", "}"]),
     False, None),
    ("a // comment line",
     "// hello\n" + fn("select_0", 1, 0, ["a"], ["a"], dual="_"), False, None),
    ("comparison with different operands in the two halves",
     LEAF1 + fn("select_0", 2, 0, ["a", "b"], ["if (!is_dual && a < b) || (is_dual && b > a) {", "    a", "} else {", "    b", "}"]),
     False, None),
    ("comparison of a parameter with itself",
     fn("select_0", 2, 0, ["a", "b"], cmp("a", "a", "a", "b")), False, None),
    ("two candidate roots",
     LEAF1 + MIN2 + fn("select_0", 2, 0, ["a", "b"], ["select_8([a, b], is_dual)"])
     + fn("select_5", 2, 0, ["a", "b"], ["select_8([b, a], is_dual)"]), False, None),
    ("parameter named like a function",
     fn("select_0", 1, 0, ["select_3"], ["select_3"], dual="_"), False, None),
    ("single element", fn("select_0", 1, 0, ["a"], ["a"], dual="_"), True, 0),
    # --- PLAIN dialect
    ("plain: run wrapper, let-single min, plain-if median with claims",
     RUN.format(n=3, i=1, d="") + PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], pcmp("c", "b", "select_2([a, b, c])", "select_2([a, c, b])"), dual=None),
     True, 3),
    ("plain: let-tuple rebinding (median of 3 via a tuple swap)",
     PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], plet("(b, c)", "c", "b", "(b, c)", "(c, b)", "select_2([a, b, c])"), dual=None),
     True, 3),
    ("plain: let with fresh pattern names",
     PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], plet("(x0, x1)", "c", "b", "(b, c)", "(c, b)", "select_2([a, x0, x1])"), dual=None),
     True, 3),
    ("plain: let tuples swapped (wrong)",
     PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], plet("(b, c)", "c", "b", "(c, b)", "(b, c)", "select_2([a, b, c])"), dual=None),
     False, None),
    ("plain: let pattern and tuple of different length",
     PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], plet("(b, c)", "c", "b", "(b, c, a)", "(c, b, a)", "select_2([a, b, c])"), dual=None),
     False, None),
    ("plain: let pattern with a repeated name",
     PLEAF1 + PMIN2 + PMED3
     + fn("select_0", 3, 1, ["a", "b", "c"], plet("(b, b)", "c", "b", "(b, c)", "(c, b)", "select_2([a, b, c])"), dual=None),
     False, None),
    ("plain: claim not implied (warning only; behaviour correct)",
     PLEAF1 + PMIN2 + fn("select_2", 3, 1, ["a", "b", "c"], pcmp("c", "a", "select_8([a, b])", "select_9([c])"),
                         dual=None, claims=", c < b, a < c")
     + fn("select_0", 3, 1, ["a", "b", "c"], pcmp("c", "b", "select_2([a, b, c])", "select_2([a, c, b])"), dual=None),
     True, 3),
    ("plain: claim naming an unknown parameter",
     PLEAF1 + PMIN2 + fn("select_2", 3, 1, ["a", "b", "c"], pcmp("c", "a", "select_8([a, b])", "select_9([c])"),
                         dual=None, claims=", c < z")
     + fn("select_0", 3, 1, ["a", "b", "c"], pcmp("c", "b", "select_2([a, b, c])", "select_2([a, c, b])"), dual=None),
     False, None),
    ("plain: claims are not trusted (claimed relation makes a wrong drop look fine)",
     PLEAF1 + PMIN2 + fn("select_0", 3, 0, ["a", "b", "c"], pcmp("a", "b", "select_8([a, c])", "select_9([b])"),
                         dual=None, claims=", b < c"),
     False, None),
    ("plain: doc comment without i for n > 1",
     PLEAF1 + fn("select_0", 2, None, ["a", "b"], pcmp("a", "b", "select_9([a])", "select_9([b])"), dual=None),
     False, None),
    ("plain: run wrapper passing an is_dual literal",
     RUN.format(n=2, i=0, d=", false") + PLEAF1 + fn("select_0", 2, 0, ["a", "b"], pcmp("a", "b", "a", "b"), dual=None),
     False, None),
    ("mixed dialects in one file",
     LEAF1 + fn("select_0", 2, 0, ["a", "b"], pcmp("a", "b", "a", "b"), dual=None), False, None),
    ("plain: call passing is_dual",
     PLEAF1 + fn("select_0", 2, 0, ["a", "b"], pcmp("a", "b", "select_9([a], is_dual)", "b"), dual=None),
     False, None),
]


def unit_tests():
    say("A. UNIT TESTS (hand-written programs)")
    fails = 0
    for name, text, exp_ok, exp_worst in UNIT_TESTS:
        res = V.verify_text(text)
        ok = res["accepted"] == exp_ok and (exp_worst is None or res.get("worst_case") == exp_worst)
        ref, _, _ = reference_verdict(text, True, True)
        # The reference judges behaviour only.  Certificate-level rules
        # (annotations, unreachable code, wasted comparisons, ...) may make the
        # checker stricter than behaviour, never more lenient.
        if res["accepted"] and ref != "CORRECT":
            ok = False
        fails += not ok
        verdict = "ACCEPT" if res["accepted"] else "REJECT"
        extra = res["error"] if not res["accepted"] else "; ".join(res["warnings"])
        say(f"  [{'ok' if ok else 'FAIL'}] {name}: {verdict} (expected {'ACCEPT' if exp_ok else 'REJECT'}), "
            f"reference: {ref}" + (f"; {extra}" if extra else ""))
    say(f"  -> {len(UNIT_TESTS) - fails}/{len(UNIT_TESTS)} unit tests passed")
    say()
    return fails


def respace(text, rng):
    """Same program with different (legal) whitespace: extra spaces and tabs
    around every token, as Rust allows.  Line structure is kept."""
    out = []
    for line in text.split("\n"):
        if line.lstrip().startswith("///"):
            out.append(line)
            continue
        out.append(re.sub(r"\s*([\[\](),;{}<>!]|&&|\|\||->)\s*",
                          lambda m: rng.choice(["", " ", "\t", " \t "]) + m.group(1) + rng.choice(["", " ", "\t"]),
                          line))
    return "\n".join(out)


def whitespace_tests(alg_dir, fwd_files):
    say("A2. WHITESPACE ROBUSTNESS (same programs re-spaced with tabs/spaces; verdict and all counts unchanged)")
    rng = random.Random(5)
    texts = [(f, read(os.path.join(alg_dir, f))) for f in ("4_1.rs", "7_3.rs", "12_5.rs", "16_5.rs")]
    texts += [(os.path.basename(p), read(p)) for p in fwd_files[:2]]
    texts += [(name, text) for name, text, ok, _ in UNIT_TESTS if ok]
    fails = total = 0
    keys = ("accepted", "n", "i0", "worst_case", "functions", "call_sites", "flipped_calls", "claims")
    for name, text in texts:
        a = V.verify_text(text)
        for _ in range(3):
            b = V.verify_text(respace(text, rng))
            total += 1
            if any(a.get(k) != b.get(k) for k in keys):
                fails += 1
                say(f"  [FAIL] {name}: {[(k, a.get(k), b.get(k)) for k in keys if a.get(k) != b.get(k)]} {b['error']}")
    say(f"  -> {total - fails}/{total} re-spaced programs ({len(texts)} programs x 3) give identical results")
    say()
    return fails


# =========================================================================
# B. Shipped backward-search files, C. forward-search certificates
# =========================================================================

def cross_check(text, n, static, brute_max_n, path_max_n):
    """Brute force / path oracle on an accepted file: (ok, brute col, paths col)."""
    ok, brute, paths = True, "-", "-"
    if n <= brute_max_n or n <= path_max_n:
        prog = R.load(text)
        if n <= brute_max_n:
            bok, bworst, _ = R.brute_force(prog)
            brute = f"{'ok' if bok else 'WRONG'} w={bworst}"
            ok &= bool(bok) and bworst == static.get("worst_case")
        if n <= path_max_n:
            t1 = time.time()
            pok, pworst, _ = R.path_oracle(prog)
            paths = f"{'ok' if pok else 'WRONG'} w={pworst} {time.time() - t1:.0f}s"
            ok &= bool(pok) and pworst == static.get("worst_case")
    return ok, brute, paths


HDR = (f"  {'file':26} {'n':>3} {'i':>3} {'verdict':7} {'worst':>5} {'expect':>6} {'functions':>9} "
       f"{'bytes':>9} {'check s':>8} {'brute':>10} {'paths':>15}")


def table_row(label, res, expect, size, dt, brute, paths, ok):
    return (f"  {label:26} {str(res['n']):>3} {str(res['i1']):>3} {'ACCEPT' if res['accepted'] else 'REJECT':7} "
            f"{str(res.get('worst_case')):>5} {str(expect):>6} {str(res.get('functions')):>9} {size:>9} "
            f"{dt:>8.3f} {brute:>10} {paths:>15}" + ("" if ok else "   <-- FAIL " + str(res["error"])))


def shipped_tests(alg_dir, brute_max_n, path_max_n):
    say("B. SHIPPED BACKWARD-SEARCH ALGORITHMS (upstream logs/backward/algorithms)")
    say(f"   i is 1-based (file names use 0-based i).  expect = known V_i(n).  brute force over all n!")
    say(f"   inputs for n <= {brute_max_n}; exhaustive path oracle for n <= {path_max_n}.")
    files = []
    for fn_ in os.listdir(alg_dir):
        m = re.fullmatch(r"(\d+)_(\d+)\.rs", fn_)
        if m:
            files.append((int(m.group(1)), int(m.group(2)), os.path.join(alg_dir, fn_)))
    say(HDR)
    fails = 0
    for n, i0, path in sorted(files):
        text = read(path)
        t0 = time.time()
        res = V.verify_text(text, expect_n=n, expect_i0=i0)
        dt = time.time() - t0
        known = KNOWN.get((n, i0 + 1))
        ok = res["accepted"] and res["worst_case"] == known
        cok, brute, paths = cross_check(text, n, res, brute_max_n, path_max_n) if res["accepted"] else (False, "", "")
        ok &= cok
        fails += not ok
        say(table_row(os.path.basename(path), res, known, len(text), dt, brute, paths, ok))
    say(f"  -> {len(files) - fails}/{len(files)} shipped files accepted with the known value"
        f" (and agreeing with brute force / path oracle where run)")
    say()
    return fails


def forward_tests(pattern, path_max_n):
    say("C. FORWARD-SEARCH CERTIFICATES (PLAIN dialect; " + pattern.replace(SELECTION, "selection") + ")")
    say("   expect = k from the directory name V{i1}_{n}_le{k} or n{n}_i{i1}_k{k} (claimed worst case).")
    say(HDR)
    fails, count = 0, 0
    for path in forward_files(pattern):
        m = re.search(r"n(\d+)_i(\d+)_k(\d+)", path)
        mf = re.fullmatch(r"(\d+)_(\d+)\.rs", os.path.basename(path))
        mc = re.search(r"V(\d+)_(\d+)_le(\d+)", path)   # certificates/V{i1}_{n}_le{k}/*.rs
        if m and mf:
            n, i0, k = int(mf.group(1)), int(mf.group(2)), int(m.group(3))
            assert int(m.group(1)) == n and int(m.group(2)) == i0 + 1
        elif mc:
            n, i0, k = int(mc.group(2)), int(mc.group(1)) - 1, int(mc.group(3))
        else:
            continue
        count += 1
        text = read(path)
        t0 = time.time()
        res = V.verify_text(text, expect_n=n, expect_i0=i0)
        dt = time.time() - t0
        ok = res["accepted"] and res["worst_case"] == k
        cok, brute, paths = cross_check(text, n, res, 0, path_max_n) if res["accepted"] else (False, "", "")
        ok &= cok
        fails += not ok
        label = ("/".join(path.split(os.sep)[-4:-2]) + "/" if m and mf else "") + os.path.basename(path)
        say(table_row(label[-26:], res, k, len(text), dt, brute, paths, ok)
            + f"   claims {res.get('claims')}, not implied {res.get('claims_not_implied')}")
    say(f"  -> {count - fails}/{count} forward-search files accepted with the claimed worst case")
    say()
    return fails


# =========================================================================
# D. Synthetic programs
# =========================================================================

def synthetic_programs():
    """Small random correct programs (both dialects): (label, text, n)."""
    progs = []
    for dialect in ("dual", "plain"):
        for n in range(1, 10):
            for i0 in sorted({0, (n - 1) // 2, n - 1}):
                for seed in (1, 2):
                    run = (seed + n) % 2 == 0
                    text, _ = S.generate(n, i0, seed * 1000 + n * 10 + i0, dialect, run=run, heuristic=n >= 8)
                    progs.append((f"syn_{dialect}_{n}_{i0}_s{seed}{'_run' if run else ''}", text, n, i0))
    return progs


def synthetic_tests(progs):
    say("D. SYNTHETIC PROGRAMS (make_synthetic.py; random, correct, not optimal; both dialects)")
    say("   every program: static ACCEPT; brute force (n <= 8) and path oracle agree; equal worst case.")
    fails = 0
    feats = {"let": 0, "fresh let names": 0, "!is_dual": 0, "leaf returns at arity > 1": 0, "run wrapper": 0,
             "run(input, true)": 0}
    for label, text, n, i0 in progs:
        res = V.verify_text(text, expect_n=n, expect_i0=i0)
        ok = res["accepted"]
        if ok:
            cok, _, _ = cross_check(text, n, res, 8, 9)
            ok &= cok
        if not ok:
            say(f"   FAIL {label}: {res['error']}")
        fails += not ok
        feats["let"] += text.count("let ")
        feats["fresh let names"] += len(re.findall(r"let \(?v\d", text))
        feats["!is_dual"] += text.count("!is_dual)")
        feats["leaf returns at arity > 1"] += res.get("leaf_returns", 0) - text.count("[usize; 1]")
        feats["run wrapper"] += "pub fn run" in text
        feats["run(input, true)"] += "(input, true)" in text
    say(f"   features covered: " + ", ".join(f"{k}: {v}" for k, v in feats.items()))
    say(f"  -> {len(progs) - fails}/{len(progs)} synthetic programs accepted and confirmed")
    say()
    return fails


# =========================================================================
# E. Mutation tests
# =========================================================================

M_DOC = re.compile(r"/// n = (\d+)(?:, i = (\d+))?(.*)")
M_SIG = re.compile(r"fn (select_\d+)\(\[(.*?)\]: \[usize; (\d+)\](, (?:is_dual|_): bool)?\) -> usize \{")
M_IF_DUAL = re.compile(r"(\s*)if \(!is_dual && (\w+) < (\w+)\) \|\| \(is_dual && (\w+) > (\w+)\) \{")
M_IF_PLAIN = re.compile(r"(\s*)if (\w+) < (\w+) \{")
M_LET = re.compile(r"(\s*)let (\w+|\([\w, ]+\)) = if (\w+) < (\w+) \{")
M_CALL = re.compile(r"(\s*)(select_\d+)\(\[(.*?)\](, !?is_dual)?\)")


def names(s):
    return [t.strip() for t in s.strip().strip("()").split(",")]


def structure(lines):
    """Locate functions, comparisons, let tuples and calls (to build mutants)."""
    funcs, calls = [], []
    for idx, line in enumerate(lines):
        if (m := M_DOC.fullmatch(line)):
            doc = (idx, int(m.group(1)), m.group(2), m.group(3))
        elif (m := M_SIG.fullmatch(line)):
            funcs.append({"doc": doc[0], "n": doc[1], "i": doc[2], "claims": doc[3], "name": m.group(1),
                          "params": names(m.group(2)), "cmp": None, "kind": None})
        elif (m := M_IF_DUAL.fullmatch(line)) or (m := M_IF_PLAIN.fullmatch(line)) or (m := M_LET.fullmatch(line)):
            funcs[-1]["cmp"] = idx
            funcs[-1]["kind"] = {M_IF_DUAL: "dual", M_IF_PLAIN: "plain", M_LET: "let"}[m.re]
        elif (m := M_CALL.fullmatch(line)):
            calls.append({"line": idx, "f": len(funcs) - 1, "callee": m.group(2), "indent": m.group(1),
                          "args": names(m.group(3)),
                          "flip": None if m.group(4) is None else m.group(4) == ", !is_dual"})
    return funcs, calls


def cmp_line(f, line, x, y, x2=None, y2=None):
    """Comparison line of f's kind with operands x, y (dual: second half x2, y2)."""
    if f["kind"] == "dual":
        ind = M_IF_DUAL.fullmatch(line).group(1)
        return f"{ind}if (!is_dual && {x} < {y}) || (is_dual && {x2 or x} > {y2 or y}) {{"
    if f["kind"] == "plain":
        return f"{M_IF_PLAIN.fullmatch(line).group(1)}if {x} < {y} {{"
    m = M_LET.fullmatch(line)
    return f"{m.group(1)}let {m.group(2)} = if {x} < {y} {{"


def operands(f, line):
    m = {"dual": M_IF_DUAL, "plain": M_IF_PLAIN, "let": M_LET}[f["kind"]].fullmatch(line)
    return (m.group(2), m.group(3)) if f["kind"] != "let" else (m.group(3), m.group(4))


def call_text(c, callee=None, args=None, flip=None):
    callee = c["callee"] if callee is None else callee
    args = c["args"] if args is None else args
    flip = c["flip"] if flip is None else flip
    dual = "" if c["flip"] is None else (", !is_dual" if flip else ", is_dual")
    return f"{c['indent']}{callee}([{', '.join(args)}]{dual})"


def doc_text(f, i=None, claims=None):
    i = f["i"] if i is None else str(i)
    return f"/// n = {f['n']}" + ("" if i is None else f", i = {i}") + (f["claims"] if claims is None else claims)


OPS = ["swap_branches", "flip_operands", "replace_operand", "half_condition", "remove_comparison",
       "tuple_value", "tuple_swap", "change_return", "replace_arg", "drop_arg", "swap_args", "flip_dual",
       "redirect_call", "doc_rank", "root_rank", "drop_claims", "bogus_claim"]
CLAIM_OPS = ("drop_claims", "bogus_claim")      # comments only: behaviour and verdict must not change


def enumerate_mutants(lines):
    funcs, calls = structure(lines)
    arity = {f["name"]: f["n"] for f in funcs}
    rank = {f["name"]: int(f["i"] or 0) for f in funcs}
    out = []
    for fi, f in enumerate(funcs):
        P = f["params"]
        if f["cmp"] is not None:
            k = f["cmp"]
            x, y = operands(f, lines[k])
            out += [("swap_branches", fi), ("flip_operands", fi)]
            out += [("replace_operand", fi, w, z) for w in (0, 1) for z in P if z not in (x, y)]
            if f["kind"] == "dual":
                out.append(("half_condition", fi))
            out += [("remove_comparison", fi, keep) for keep in ((0,) if f["kind"] == "let" else (0, 1))]
            if f["kind"] == "let":
                for br in (0, 1):
                    vals = names(lines[k + 1 + 2 * br])
                    out += [("tuple_value", fi, br, j, z) for j in range(len(vals)) for z in P if z != vals[j]]
                    out += [("tuple_swap", fi, br, j1, j2) for j1 in range(len(vals)) for j2 in range(j1 + 1, len(vals))]
        if f["name"] == "select_0":
            out += [("root_rank", fi, j) for j in range(f["n"]) if f["i"] is not None and j != int(f["i"])]
        elif f["n"] >= 2:
            out += [("doc_rank", fi, j) for j in range(f["n"]) if f["i"] is not None and j != int(f["i"])]
        if f["claims"]:
            out.append(("drop_claims", fi))
        claimed = set(re.findall(r"(\w+) < (\w+)", f["claims"]))
        out += [("bogus_claim", fi, u, v) for u in P for v in P if u != v and (u, v) not in claimed]
    for ci, c in enumerate(calls):
        params = funcs[c["f"]]["params"]
        k = len(c["args"])
        if c["flip"] is not None:
            out.append(("flip_dual", ci))
        for pos in range(k):
            for p in params:
                if p != c["args"][pos] and p in params:
                    out.append(("change_return" if k == 1 else "replace_arg", ci, pos, p))
            if k >= 2:
                out.append(("drop_arg", ci, pos))
        out += [("swap_args", ci, p, q) for p in range(k) for q in range(p + 1, k)]
        out += [("redirect_call", ci, g) for g in arity
                if g != c["callee"] and arity[g] == k and rank[g] == rank[c["callee"]]]
    return out, funcs, calls


def apply_mutant(lines, d, funcs, calls):
    L = list(lines)
    op = d[0]
    if op in ("swap_branches", "flip_operands", "half_condition", "replace_operand", "remove_comparison",
              "tuple_value", "tuple_swap"):
        f = funcs[d[1]]
        k = f["cmp"]
        x, y = operands(f, L[k])
        if op == "swap_branches":                 # if: the two statements; let: the two tuples
            L[k + 1], L[k + 3] = L[k + 3], L[k + 1]
        elif op == "flip_operands":
            L[k] = cmp_line(f, L[k], y, x)
        elif op == "half_condition":
            L[k] = cmp_line(f, L[k], x, y, y, x)
        elif op == "replace_operand":
            L[k] = cmp_line(f, L[k], d[3], y) if d[2] == 0 else cmp_line(f, L[k], x, d[3])
        elif op == "remove_comparison":
            if f["kind"] == "let":                # drop the let: the call sees the parameters
                del L[k:k + 5]
            else:
                L[k:k + 5] = [L[k][:len(L[k]) - len(L[k].lstrip())] + L[k + 1 + 2 * d[2]].strip()]
        else:
            t = k + 1 + 2 * d[2]
            vals = names(L[t])
            if op == "tuple_value":
                vals[d[3]] = d[4]
            else:
                vals[d[3]], vals[d[4]] = vals[d[4]], vals[d[3]]
            ind = L[t][:len(L[t]) - len(L[t].lstrip())]
            L[t] = ind + (vals[0] if len(vals) == 1 else f"({', '.join(vals)})")
    elif op in ("doc_rank", "root_rank"):
        f = funcs[d[1]]
        L[f["doc"]] = doc_text(f, i=d[2])
    elif op == "drop_claims":
        f = funcs[d[1]]
        L[f["doc"]] = doc_text(f, claims="")
    elif op == "bogus_claim":
        f = funcs[d[1]]
        L[f["doc"]] = doc_text(f, claims=f["claims"] + f", {d[2]} < {d[3]}")
    else:
        c = calls[d[1]]
        args = list(c["args"])
        if op == "flip_dual":
            L[c["line"]] = call_text(c, flip=not c["flip"])
        elif op in ("change_return", "replace_arg"):
            args[d[2]] = d[3]
            L[c["line"]] = call_text(c, args=args)
        elif op == "drop_arg":
            del args[d[2]]
            L[c["line"]] = call_text(c, args=args)
        elif op == "swap_args":
            args[d[2]], args[d[3]] = args[d[3]], args[d[2]]
            L[c["line"]] = call_text(c, args=args)
        elif op == "redirect_call":
            L[c["line"]] = call_text(c, callee=d[2])
    return L


def mutation_tests(jobs, brute_max_n, path_max_n):
    """jobs: list of (label, text, per_op); per_op None = all mutants."""
    say("E. MUTATION TESTS")
    say(f"   ground truth: brute force over all n! inputs (n <= {brute_max_n}) and/or exhaustive path oracle"
        f" (n <= {path_max_n})")
    rng = random.Random(20261012)
    keys = ["total", "rej", "bad", "bad_rej", "ok", "ok_acc", "acc_wrong", "worst_low", "undecided",
            "undecided_rej", "warned"]
    table = {op: dict.fromkeys(keys, 0) for op in OPS}
    reasons, examples = {}, []
    t0 = time.time()
    for label, text, per_op in jobs:
        lines = text.rstrip("\n").split("\n")
        descs, funcs, calls = enumerate_mutants(lines)
        n = next(f["n"] for f in funcs if f["name"] == "select_0")
        chosen = []
        for op in OPS:
            ds = [d for d in descs if d[0] == op]
            if op == "bogus_claim" and per_op is None:
                ds = rng.sample(ds, min(len(ds), 20))
            chosen += ds if per_op is None or len(ds) <= per_op else rng.sample(ds, per_op)
        for d in chosen:
            mtext = "\n".join(apply_mutant(lines, d, funcs, calls)) + "\n"
            res = V.verify_text(mtext)
            ref, ref_worst, detail = reference_verdict(mtext, n <= brute_max_n, n <= path_max_n)
            t = table[d[0]]
            t["total"] += 1
            t["rej"] += not res["accepted"]
            t["warned"] += bool(res.get("claims_not_implied"))
            if ref == "UNDECIDED":
                # cyclic mutant, n > brute_max_n, no error in 20000 random inputs
                t["undecided"] += 1
                t["undecided_rej"] += not res["accepted"]
                if res["accepted"]:
                    t["acc_wrong"] += 1
                    say(f"   !!! accepted a cyclic program: {label} {d}")
            elif ref == "CORRECT":
                t["ok"] += 1
                if res["accepted"]:
                    t["ok_acc"] += 1
                    t["worst_low"] += res["worst_case"] < ref_worst     # would be an unsound count
                else:
                    msg = re.sub(r"^\[\w+\] (line \d+: )?", "", res["error"])
                    key = re.sub(r"\d+|select_\w+|\b[a-p]\b|\[.*\]|'[a-z]'", "#", msg)[:90]
                    reasons.setdefault(d[0], {}).setdefault(key, 0)
                    reasons[d[0]][key] += 1
                    if all(e[1][0] != d[0] for e in examples):
                        examples.append((label, d, res["error"]))
            else:
                t["bad"] += 1
                t["bad_rej"] += not res["accepted"]
                if res["accepted"]:
                    t["acc_wrong"] += 1
                    say(f"   !!! SOUNDNESS FAILURE: {label} {d} accepted but {ref}: {detail}")
        say(f"   {label:28} n={n:<3} mutants={len(chosen):<6} "
            f"({'all' if per_op is None else f'<= {per_op} per operator'})  elapsed {time.time() - t0:.0f}s")
    say()
    say("   operator           mutants  rejected | wrong/ill-formed  rejected | behaviour-correct  accepted"
        " | accepted-but-wrong")
    tot = dict.fromkeys(keys, 0)
    for op in OPS + ["TOTAL"]:
        t = table[op] if op != "TOTAL" else tot
        if op != "TOTAL":
            for k in keys:
                tot[k] += t[k]
        if t["total"]:
            say(f"   {op:18} {t['total']:>7} {t['rej']:>9} | {t['bad']:>16} {t['bad_rej']:>9} | "
                f"{t['ok']:>17} {t['ok_acc']:>9} | {t['acc_wrong']:>5}")
    say(f"   semantic mutants (wrong or ill-formed) rejected: {tot['bad_rej']}/{tot['bad']}"
        f" = {100.0 * tot['bad_rej'] / max(1, tot['bad']):.2f}%")
    say(f"   accepted mutants whose certified worst case is below the true worst case: {tot['worst_low']}")
    say(f"   undecided (cyclic, n > {brute_max_n}, no error in 20000 random inputs): {tot['undecided']},"
        f" rejected: {tot['undecided_rej']}")
    cl = [table[op] for op in CLAIM_OPS]
    say(f"   doc-claim mutants (comments only): {sum(t['total'] for t in cl)}, accepted "
        f"{sum(t['ok_acc'] for t in cl)}; bogus claims reported as not implied: {table['bogus_claim']['warned']}"
        f"/{table['bogus_claim']['total']}")
    say()
    say("   behaviour-correct mutants that the checker REJECTS, by reason (normalised message):")
    for op in OPS:
        for key, cnt in sorted(reasons.get(op, {}).items(), key=lambda kv: -kv[1]):
            say(f"     {op:18} {cnt:>6}  {key.strip()}")
    say("   one example per operator:")
    for label, d, err in examples:
        say(f"     {label} {d}: {err}")
    say()
    bad = tot["acc_wrong"] + tot["worst_low"]
    bad += sum(t["total"] - t["ok_acc"] for t in cl)          # claims must never change the verdict
    return bad


# =========================================================================
# F. Stress test
# =========================================================================

def stress_tests(directory):
    say("F. STRESS TEST (large synthetic files; static check only)")
    for path in sorted(glob.glob(os.path.join(directory, "*.rs"))):
        t0 = time.time()
        res = V.verify_text(read(path))
        dt = time.time() - t0
        say(f"   {os.path.basename(path):24} {os.path.getsize(path) / 1e6:7.1f} MB  "
            f"{'ACCEPT' if res['accepted'] else 'REJECT'}  n={res['n']} i={res['i1']} worst={res.get('worst_case')}"
            f"  functions={res.get('functions')}  {dt:.1f} s")
    say()
    return 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--alg-dir", default=DEFAULT_ALG_DIR)
    ap.add_argument("--forward-glob", default=DEFAULT_FORWARD_GLOB)
    ap.add_argument("--out", default=os.path.join(HERE, "test_results.txt"))
    ap.add_argument("--quick", action="store_true", help="small mutation sample, path oracle only up to n = 10")
    ap.add_argument("--path-max-n", type=int, default=12)
    ap.add_argument("--stress", metavar="DIR", help="also time the checker on every .rs file in DIR")
    a = ap.parse_args()
    path_max_n = 10 if a.quick else a.path_max_n
    t_start = time.time()
    say(f"verify_rs_algorithm.py test suite   ({time.strftime('%Y-%m-%d %H:%M')}, python {sys.version.split()[0]})")
    say()
    bad = unit_tests()
    skipped = len(glob.glob(a.forward_glob)) - len(forward_files(a.forward_glob))
    if skipped:
        say(f"note: {skipped} header-only forward-search outputs (refuted budgets, no algorithm) skipped")
        say()
    bad += whitespace_tests(a.alg_dir, forward_files(a.forward_glob))
    bad += shipped_tests(a.alg_dir, brute_max_n=9, path_max_n=path_max_n)
    bad += forward_tests(a.forward_glob, path_max_n=path_max_n)
    progs = synthetic_programs()
    bad += synthetic_tests(progs)
    ship = {os.path.basename(p): p for p in glob.glob(os.path.join(a.alg_dir, "*.rs"))}
    fwd = forward_files(a.forward_glob)
    syn = {}
    for label, text, _, _ in progs:           # also reachable without the "_run" suffix
        syn[label] = syn[label.replace("_run", "")] = text
    if a.quick:
        jobs = [("5_2.rs", read(ship["5_2.rs"]), None), ("7_3.rs", read(ship["7_3.rs"]), 4),
                ("syn_plain_6_2_s1", syn["syn_plain_6_2_s1"], 4)]
    else:
        jobs = [(f, read(ship[f]), None) for f in ("4_1.rs", "5_2.rs", "6_2.rs", "7_3.rs")]
        jobs += [(f, read(ship[f]), 12) for f in ("6_1.rs", "7_1.rs", "7_2.rs", "8_1.rs", "8_2.rs", "8_3.rs")]
        jobs += [(f, read(ship[f]), 6) for f in ("9_2.rs", "9_3.rs", "9_4.rs", "10_3.rs", "10_4.rs",
                                                  "11_3.rs", "11_4.rs", "11_5.rs")]
        jobs += [(lab, syn[lab], None) for lab in ("syn_plain_5_2_s1", "syn_plain_6_2_s2", "syn_plain_7_3_s1")]
        jobs += [(lab, syn[lab], 12) for lab in ("syn_plain_8_3_s1", "syn_dual_8_3_s2", "syn_plain_9_4_s2")]
        jobs += [("/".join(p.split(os.sep)[-3:-2]) + "/" + os.path.basename(p), read(p), 3)
                 for p in fwd if re.search(r"n12_i6_k2[02]", p)]
    bad += mutation_tests(jobs, brute_max_n=8, path_max_n=12)
    if a.stress:
        bad += stress_tests(a.stress)
    say(f"OVERALL: {'PASS' if bad == 0 else 'FAIL (%d problems)' % bad}   (total time {time.time() - t_start:.0f} s)")
    with open(a.out, "w") as fh:
        fh.write("\n".join(OUT) + "\n")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
