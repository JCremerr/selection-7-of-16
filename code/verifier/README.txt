INDEPENDENT CHECKER FOR EXACT SELECTION ALGORITHMS  (verify_rs_algorithm.py)
============================================================================

Purpose.  Trust anchor of the selection project: a claimed upper bound
V_i(n) <= W is accepted only if this checker ACCEPTs an algorithm file for
(n, i) with certified worst case <= W.  It reads selection algorithms printed
as Rust code by upstream's selection_generator (Doerrer et al., SEA 2025), in
both upstream formats (backward search: "DUAL" dialect; forward search:
"PLAIN" dialect).  It was written from the mathematical definition of
selection only; no generator code was read, imported or executed (only the
algorithm files, README.md, test_algorithms.sh and src/algorithm_test.rs).

Contents
  1. Quick start            6. Soundness theorem and proof
  2. Files                  7. Strictness policy (deliberate rejections)
  3. Input format           8. Limitations and trusted base
  4. Meaning of a program   9. Test results
  5. The checking rules    10. How to audit this in 30 minutes


1. QUICK START
--------------
    python3 -I verify_rs_algorithm.py FILE.rs                  # verdict + statistics
    python3 -I verify_rs_algorithm.py --from-filename 16_5.rs  # also check n, i from {n}_{i0}.rs
    python3 -I verify_rs_algorithm.py FILE.rs --expect-n 16 --expect-i 7 --expect-worst 32
    python3 -I verify_rs_algorithm.py FILE.rs --json           # machine-readable
    python3 -I run_tests.py [--quick] [--stress DIR]            # test suite -> test_results.txt

  Output: ACCEPT/REJECT, n, i (1-based AND 0-based), certified worst-case
  number of comparisons, #functions, #call sites, warnings, and on REJECT the
  first violation ([PARSE]/[LINK]/[CHECK], line number, reason).
  Exit status 0 = ACCEPT, 1 = REJECT, 2 = cannot read file.
  --expect-i is 1-BASED (i-th smallest); upstream file names use 0-based i.
  Only the Python 3 standard library is used; single process.


2. FILES
--------
  verify_rs_algorithm.py  THE CHECKER (parser, linker, checker, CLI). ~680 lines,
                          of which ~350 are logic.  Standard library only.
  reference_semantics.py  test oracle: own parser + literal interpreter; brute
                          force over all n! inputs and exhaustive symbolic path
                          exploration.  Not part of the trusted base.
  make_synthetic.py       test data: random correct programs (both dialects).
  run_tests.py            test suite (unit, whitespace robustness, shipped
                          files, forward files, synthetic, mutation, stress).
  test_results.txt        output of the last full run of run_tests.py.


3. INPUT FORMAT
---------------
The parser is line based and strict: every non-blank line must match the
grammar below (whitespace inside a line is free, as in Rust; only ASCII
whitespace).  Anything else -- comments, nested ifs, extra statements,
trailing commas, unknown attributes -- is a PARSE error, never skipped.

  Header (optional, anywhere at top level, each at most once):
      #[allow(...)]                                  (lint attribute, ignored)
      pub const N: usize = <int>;   pub const I: usize = <int>;
      pub fn run(input: [usize; <n>]) -> usize {
          select_K(input, false|true)       (DUAL)   or   select_K(input)   (PLAIN)
      }
  Function:
      /// n = <m>[, i = <r>][, u < v]*        doc comment, REQUIRED, directly above
      fn select_K([p0, p1, ..]: [usize; <m>], is_dual: bool) -> usize {     DUAL
      fn select_K([p0, p1, ..]: [usize; <m>]) -> usize {                    PLAIN
          BODY
      }
  DUAL body:
      STMT
    | if (!is_dual && X < Y) || (is_dual && X > Y) {   (same X, Y in both halves)
          STMT
      } else {
          STMT
      }
      STMT = select_K([q0, q1, ..], is_dual) | select_K([..], !is_dual) | p
      The second parameter may be `_` instead of is_dual (then BODY must be p).
  PLAIN body:
      STMT
    | if X < Y { STMT } else { STMT }                  (on 5 lines as above)
    | let PAT = if X < Y {
          TUPLE
      } else {
          TUPLE
      };
      STMT
      STMT = select_K([q0, q1, ..]) | p;   PAT, TUPLE = name | (name, name, ..)
  Names: [a-z][a-z0-9_]*, not a keyword, not `is_dual`/`input`/`run`, not
  select_K.  Function names: select_<digits>.  One dialect per file.

Desugaring of let (exactly Rust's semantics: the tuple is evaluated in the
old scope, then PAT's names are (re)bound, then STMT runs):
      let PAT = if X < Y { T } else { E }; S    ==   if X < Y { S[PAT:=T] } else { S[PAT:=E] }
  e.g. `let (b, c) = if a < b { (b, c) } else { (a, d) }; select_7([b, c, e])`
       == `if a < b { select_7([b, c, e]) } else { select_7([a, d, e]) }`.

Observations about the real files (format "surprises"):
  * The 70 shipped backward files contain NO run wrapper and no N/I consts
    (upstream's algorithm_test.rs needs them); the root is select_0, the only
    function never called, and is printed last.  Without a run wrapper the
    checker assumes the root is called with is_dual = false.
  * Backward (DUAL) files: `!is_dual` IS passed (13,586 of 105,900 calls).
    In every file, every function has exactly one comparison except a single
    arity-1 function `fn select_K([a]: [usize; 1], _: bool) -> usize { a }`,
    which is the only leaf return.  "Returning" a determined element is done
    by calling that function with one argument (all other parameters
    dropped).  No nested ifs.  (Forward files: likewise exactly one leaf
    function per file.)
  * Functions are shared by many call sites (a DAG, not a tree), with
    permuted / subset argument lists.  In all 79 real files (70 backward +
    9 forward) every callee is printed before its callers, the root is
    select_0 and printed last, and the call graph is acyclic.
  * Forward (PLAIN) files: no is_dual; comparisons as plain if or as let-tuple
    rebinding; `/// n = 1` has no i; doc comments list relations.  In all 9
    forward files, in EVERY function, the listed relations are exactly the
    cover relations (Hasse diagram) of the precondition this checker computes
    independently (closure(claims) == P_f) -- strong cross-validation.
  * The doc-comment rank i is 0-based and refers to the function's own
    orientation (see 4).


4. MEANING OF A PROGRAM
-----------------------
Inputs are n distinct values (upstream's test uses permutations of 0..n-1).
Each executing function has an orientation d = the runtime value of is_dual
(always false in PLAIN files).  Write <_d for < if d = false and for > if
d = true.  Then:
  * the DUAL condition (!is_dual && X < Y) || (is_dual && X > Y) is true iff
    X <_d Y; the PLAIN condition X < Y is the same with d = false;
  * select_K([..], is_dual) runs the callee with the same d, `!is_dual` with
    the opposite d (the callee's order is the reverse of the caller's);
  * the root runs with d = false (or the literal given in run).

Specification of a function f with arity m, doc rank r = r_f and a strict
partial order P_f on its positions (P_f is COMPUTED, see 5):
  Spec(f): for every d and all distinct x_0..x_{m-1} with x_u <_d x_v for all
  (u, v) in P_f, the call f(x, d) terminates and returns the x_p that has
  exactly r of the x_q below it in <_d  ("the element of rank r", 0-based).
For the root, P = {} and d = false, so Spec(root) says: the program returns
the (r+1)-th smallest input.  The checker prints this as i = r+1 (1-based).


5. THE CHECKING RULES
---------------------
R0  PARSE/LINK.  Strict grammar (3).  Every name resolves; every call has
    exactly the callee's arity (= Rust's type check); doc n = arity;
    0 <= doc i < n (i may be omitted only for n = 1); one dialect per file;
    `_` parameter only with a leaf body; exactly one function is never called
    (the root; if run exists it must call exactly that one, with the right
    array length and the dialect's argument form); N, I consts (if present)
    match the root.  Then: every function reachable from the root; the call
    graph is acyclic.  Relations named in doc comments must name parameters.

Preconditions (computed, top-down; the core of the method).
    P_root = {}.  Functions are processed in a topological order of the call
    DAG (every caller before its callees), so P_g is final when g is checked.
    In f, the known relations ("knowledge") at a statement are
        Q = P_f                                  if f has no comparison,
        Q_then = closure(P_f + {X < Y}),  Q_else = closure(P_f + {Y < X}).
    A call site s: g(a_0..a_{k-1}) with flip phi (phi = passes !is_dual) maps
    Q_s to the callee's positions:
        M_s = { (j, l) : (a_j, a_l) in Q_s }     if not phi,
        M_s = { (j, l) : (a_l, a_j) in Q_s }     if phi (order reversed).
    P_g = intersection of M_s over ALL call sites s of g.
    Closure of P + {lo < hi} (P closed, lo, hi incomparable) adds exactly the
    pairs (u, v) with u <= lo and hi <= v.  Posets are bitmask rows.

R1  Comparison: X != Y, and X, Y incomparable in P_f.
R2  Call g(a_0..a_{k-1}) under knowledge Q (m = arity of f, r = r_f):
    - the a_j are pairwise distinct;
    - every DROPPED position e (not among the a_j) is certified:
        "below"  if  |{v : e < v in Q}| >= m - r     (so rank(e) <= r-1), or
        "above"  if  |{u : u < e in Q}| >= r + 1     (so rank(e) >= r+1);
    - with kb = number of dropped positions certified "below", the answer's
      rank among the arguments is r - kb (in f's order), i.e.
        r_needed = r - kb             if not phi,
        r_needed = k - 1 - (r - kb)   if phi;
      g's doc rank must equal r_needed (arity was checked in R0).
R3  Leaf return of p under Q:  |below_Q(p)| = r  and  |above_Q(p)| = m-1-r.
R4  Claims (doc-comment relations u < v): reported as WARNING if (u, v) is
    not in P_f (REJECT with --strict-claims).  They are never used otherwise.
Worst case.  W(f) = [f has a comparison] + max over f's branches of
    (W(callee) for a call, 0 for a leaf return), computed callees-first; the
    reported value is W(root) (longest path in the call DAG).


6. SOUNDNESS THEOREM AND PROOF
------------------------------
THEOREM.  If the checker ACCEPTs a file and reports n, i (1-based) and W,
then for all n distinct input values the program (root called with
is_dual = false, or as run says) terminates, returns the i-th smallest input,
and performs at most W comparisons.  Hence V_i(n) <= W.

Lemma 1 (P_f is a strict partial order).  Induction along the topological
order.  P_root = {} is one.  If P_f is one and X, Y are incomparable in it
(R1), then closure(P_f + {X < Y}) is irreflexive and transitive (a cycle would
need Y <= X in P_f).  Restricting a strict partial order to the distinct
positions a_j and possibly reversing it gives a strict partial order on the
callee's positions; an intersection of strict partial orders on the same set
is one.  (The checker also asserts this invariant for every function.)

Lemma 2 (knowledge holds).  Let x satisfy P_f in orientation d.  If the
then-branch runs, the condition was true, i.e. x_X <_d x_Y (section 4); with
P_f and transitivity of <_d every pair of Q_then holds.  If the else-branch
runs, the condition was false and the values are distinct, so x_Y <_d x_X,
and every pair of Q_else holds.  Without comparison Q = P_f holds.

Lemma 3 (callee precondition holds).  At call site s the callee receives
y_j = x_{a_j} with orientation d' (= d, or not d if phi).  The y_j are
distinct (R2: distinct positions, distinct values).  Let (j, l) in P_g; then
(j, l) in M_s.  If not phi: (a_j, a_l) in Q_s, so y_j <_d y_l, i.e.
y_j <_d' y_l.  If phi: (a_l, a_j) in Q_s, so y_l <_d y_j, which is
y_j <_d' y_l because <_{not d} is the converse of <_d.

Main induction.  The call graph is a finite DAG (R0), so induct over
functions callees-first.  Assume Spec(g) for all callees g of f; take x
satisfying P_f in orientation d.  f makes at most one comparison and then
executes one statement under knowledge Q that holds for x (Lemma 2).
  Leaf p (R3): r inputs are below x_p and m-1-r above; these are disjoint and
  cover all m-1 others, so x_p has rank exactly r.
  Call (R2): let x* be the input of rank r.  A dropped e certified "below"
  has >= m-r inputs above it, so rank(e) <= (m-1)-(m-r) = r-1; one certified
  "above" has rank >= r+1.  Hence x* is not dropped: x* = y_q for some q.  All
  kb dropped "below" elements are below x*, all other dropped ones above, so
  x* has r - kb arguments below it in <_d, i.e. rank r - kb in <_d and
  k-1-(r-kb) in <_d' if phi.  By R2 this is g's doc rank r_g.  By Lemma 3
  the callee's precondition holds, so by the induction hypothesis g returns
  the argument of rank r_g in <_d', which is y_q = x*.  g terminates (IH).
So Spec(f) holds for every f.  For the root, P = {} holds trivially, d =
false, so the program returns the input of rank r_root in <.  (With
run(input, true) the root's rank is mirrored: i = n - 1 - r_root, 0-based.)

Comparison count.  An execution visits a chain f_0 = root, f_1, ... of
functions along call edges (in the branch actually taken), ending at a leaf
return; each f_t makes at most one comparison.  So the number of comparisons
is at most the maximum over such chains, which is W(root) by definition.  []

Only the doc RANKS are used, and only as the induction hypothesis
("certificate"); a wrong rank makes some R2 equality fail.  Doc RELATIONS
are never used.  The precondition of a function is never taken from the
file: it is computed from the callers.


7. STRICTNESS POLICY (deliberate rejections; they can only cause false
   rejections, never a false acceptance)
-----------------------------------------------------------------------
  * Comparison of two elements already ordered in P_f: REJECT ("wasted
    comparison").  Since P_f is the intersection over all call sites, such a
    comparison is redundant at EVERY call site and one branch is dead; the if
    can be replaced by its live branch, giving an equivalent program with
    fewer comparisons, so nothing is lost.  Never occurs in real files.
  * Unreachable functions, more than one never-called function, recursion /
    cycles: REJECT.
  * Repeated argument in a call: REJECT (the specification is for distinct
    elements; Rust would compile it).
  * Doc rank inconsistent with what the caller needs: REJECT, even if the
    behaviour happens to be right (the ranks are the certificate).
  * Incompleteness of the intersection: a function that is correct only
    because each call site supplies a DIFFERENT extra relation would be
    rejected.  All 79 real files (70 backward + 9 forward) are accepted, so
    this did not occur.
  * Anything outside the grammar of section 3 (comments, nested if, other
    statement forms, `(!is_dual && X < Y) || (is_dual && Y > X)` with
    swapped operands in one half, mixed dialects): REJECT.


8. LIMITATIONS AND TRUSTED BASE
-------------------------------
  * Checks UPPER bounds only (correct algorithm with <= W comparisons).  It
    says nothing about optimality / lower bounds.
  * W is the longest path in the call DAG.  If some path is infeasible, W can
    exceed the algorithm's true worst case (still a valid upper bound).  On
    every real file where an exhaustive check was run (all files with
    n <= 12, plus forward n13_i7_k24 and n14_i7_k27), W equals the exact
    worst case.
  * Trusted: CPython 3, verify_rs_algorithm.py, and the modelling of Rust in
    section 4 (is_dual condition, !is_dual, let rebinding, root called with
    is_dual = false when no run wrapper is present).  Rust is not compiled;
    the grammar is narrow, but an accepted file need not compile (e.g. unused
    slots of a `let` tuple are not name-resolved; the checker does not look
    for a `fn main`).  This does not affect soundness: the checker verifies
    the program it models.  All archived certificates compile under rustc.
  * Files must be ASCII.  Performance is linear in the file size: 46 MB
    (145k functions) in 12-17 s and 0.6 GB RAM (see 9); the largest real
    file (14_6.rs, 3.5 MB, 11.6k functions) takes 0.6 s.


9. TEST RESULTS   (full output: test_results.txt)
-----------------
test_results.txt          run on this repository (section C = the 9 certificates
                          in certificates/; 11,956 mutants, 0 wrong verdicts).
test_results_paper_run.txt  the run cited in the paper (12,046 mutants); its
                          section C used intermediate search outputs that are
                          not part of this repository.  The summary below is
                          from an earlier run of the same suite.

Summary of the last full run (2026-10-12 16:21):
  -> 37/37 unit tests passed
  -> 48/48 re-spaced programs (16 programs x 3) give identical results
  -> 70/70 shipped files accepted with the known value (and agreeing with brute force / path oracle where run)
  -> 9/9 forward-search files accepted with the claimed worst case
  -> 96/96 synthetic programs accepted and confirmed
  OVERALL: PASS   (total time 773 s)

B. SHIPPED BACKWARD-SEARCH ALGORITHMS (upstream logs/backward/algorithms)
   i is 1-based (file names use 0-based i).  expect = known V_i(n).  brute force over all n!
   inputs for n <= 9; exhaustive path oracle for n <= 12.
  file                         n   i verdict worst expect functions     bytes  check s      brute           paths
  1_0.rs                       1   1 ACCEPT      0      0         1        74    0.000     ok w=0       ok w=0 0s
  2_0.rs                       2   1 ACCEPT      1      1         2       283    0.000     ok w=1       ok w=1 0s
  3_0.rs                       3   1 ACCEPT      2      2         3       501    0.000     ok w=2       ok w=2 0s
  3_1.rs                       3   2 ACCEPT      3      3         4       722    0.000     ok w=3       ok w=3 0s
  4_0.rs                       4   1 ACCEPT      3      3         4       728    0.000     ok w=3       ok w=3 0s
  4_1.rs                       4   2 ACCEPT      4      4         5       970    0.000     ok w=4       ok w=4 0s
  5_0.rs                       5   1 ACCEPT      4      4         5       964    0.000     ok w=4       ok w=4 0s
  5_1.rs                       5   2 ACCEPT      6      6         7      1448    0.000     ok w=6       ok w=6 0s
  5_2.rs                       5   3 ACCEPT      6      6         7      1457    0.000     ok w=6       ok w=6 0s
  6_0.rs                       6   1 ACCEPT      5      5         6      1209    0.000     ok w=5       ok w=5 0s
  6_1.rs                       6   2 ACCEPT      7      7        13      2891    0.001     ok w=7       ok w=7 0s
  6_2.rs                       6   3 ACCEPT      8      8         9      1971    0.000     ok w=8       ok w=8 0s
  7_0.rs                       7   1 ACCEPT      6      6         7      1463    0.000     ok w=6       ok w=6 0s
  7_1.rs                       7   2 ACCEPT      8      8        11      2500    0.000     ok w=8       ok w=8 0s
  7_2.rs                       7   3 ACCEPT     10     10        19      4424    0.001    ok w=10      ok w=10 0s
  7_3.rs                       7   4 ACCEPT     10     10        20      4694    0.001    ok w=10      ok w=10 0s
  8_0.rs                       8   1 ACCEPT      7      7         8      1726    0.000     ok w=7       ok w=7 0s
  8_1.rs                       8   2 ACCEPT      9      9        10      2309    0.000     ok w=9       ok w=9 0s
  8_2.rs                       8   3 ACCEPT     11     11        35      8535    0.001    ok w=11      ok w=11 0s
  8_3.rs                       8   4 ACCEPT     12     12        31      7540    0.002    ok w=12      ok w=12 0s
  9_0.rs                       9   1 ACCEPT      8      8         9      1998    0.000     ok w=8       ok w=8 0s
  9_1.rs                       9   2 ACCEPT     11     11        17      4114    0.001    ok w=11      ok w=11 0s
  9_2.rs                       9   3 ACCEPT     12     12        13      3191    0.001    ok w=12      ok w=12 0s
  9_3.rs                       9   4 ACCEPT     14     14        87     22437    0.005    ok w=14      ok w=14 0s
  9_4.rs                       9   5 ACCEPT     14     14        57     14330    0.003    ok w=14      ok w=14 0s
  10_0.rs                     10   1 ACCEPT      9      9        10      2281    0.001          -       ok w=9 0s
  10_1.rs                     10   2 ACCEPT     12     12        28      7113    0.002          -      ok w=12 0s
  10_2.rs                     10   3 ACCEPT     14     14        58     14780    0.003          -      ok w=14 0s
  10_3.rs                     10   4 ACCEPT     15     15        64     16466    0.004          -      ok w=15 0s
  10_4.rs                     10   5 ACCEPT     16     16       130     34060    0.008          -      ok w=16 0s
  11_0.rs                     11   1 ACCEPT     10     10        11      2576    0.001          -      ok w=10 0s
  11_1.rs                     11   2 ACCEPT     13     13        37      9819    0.002          -      ok w=13 0s
  11_2.rs                     11   3 ACCEPT     15     15        48     12589    0.003          -      ok w=15 0s
  11_3.rs                     11   4 ACCEPT     17     17       201     54029    0.013          -      ok w=17 1s
  11_4.rs                     11   5 ACCEPT     18     18       201     54200    0.013          -      ok w=18 1s
  11_5.rs                     11   6 ACCEPT     18     18       188     50277    0.008          -      ok w=18 1s
  12_0.rs                     12   1 ACCEPT     11     11        12      2880    0.000          -      ok w=11 0s
  12_1.rs                     12   2 ACCEPT     14     14        34      9030    0.001          -      ok w=14 0s
  12_2.rs                     12   3 ACCEPT     17     17       212     58462    0.009          -      ok w=17 0s
  12_3.rs                     12   4 ACCEPT     18     18        46     11834    0.002          -      ok w=18 1s
  12_4.rs                     12   5 ACCEPT     19     19       137     36396    0.006          -      ok w=19 3s
  12_5.rs                     12   6 ACCEPT     20     20       119     31630    0.005          -      ok w=20 8s
  13_0.rs                     13   1 ACCEPT     12     12        13      3193    0.001          -               -
  13_1.rs                     13   2 ACCEPT     15     15        61     16719    0.004          -               -
  13_2.rs                     13   3 ACCEPT     18     18       190     52894    0.013          -               -
  13_3.rs                     13   4 ACCEPT     20     20        58     16054    0.004          -               -
  13_4.rs                     13   5 ACCEPT     21     21       774    216703    0.056          -               -
  13_5.rs                     13   6 ACCEPT     22     22      1343    381791    0.102          -               -
  13_6.rs                     13   7 ACCEPT     23     23      3537   1028008    0.292          -               -
  14_0.rs                     14   1 ACCEPT     13     13        14      3515    0.001          -               -
  14_1.rs                     14   2 ACCEPT     16     16        71     19895    0.005          -               -
  14_2.rs                     14   3 ACCEPT     19     19       103     28220    0.007          -               -
  14_3.rs                     14   4 ACCEPT     21     21       704    198956    0.052          -               -
  14_4.rs                     14   5 ACCEPT     23     23       326     89780    0.023          -               -
  14_5.rs                     14   6 ACCEPT     24     24      3716   1086133    0.304          -               -
  14_6.rs                     14   7 ACCEPT     25     25     11591   3463248    1.082          -               -
  15_0.rs                     15   1 ACCEPT     14     14        15      3846    0.001          -               -
  15_1.rs                     15   2 ACCEPT     17     17        62     17440    0.005          -               -
  15_2.rs                     15   3 ACCEPT     20     20       361    104692    0.029          -               -
  15_3.rs                     15   4 ACCEPT     23     23      1441    420382    0.119          -               -
  15_4.rs                     15   5 ACCEPT     24     24       191     52545    0.013          -               -
  15_5.rs                     15   6 ACCEPT     26     26     10780   3232699    0.996          -               -
  15_6.rs                     15   7 ACCEPT     26     26      1430    408600    0.115          -               -
  15_7.rs                     15   8 ACCEPT     27     27      4412   1297981    0.376          -               -
  16_0.rs                     16   1 ACCEPT     15     15        16      4186    0.001          -               -
  16_1.rs                     16   2 ACCEPT     18     18        19      5520    0.002          -               -
  16_2.rs                     16   3 ACCEPT     21     21       170     48262    0.013          -               -
  16_3.rs                     16   4 ACCEPT     24     24      1377    402062    0.113          -               -
  16_4.rs                     16   5 ACCEPT     26     26      6482   1948976    0.594          -               -
  16_5.rs                     16   6 ACCEPT     27     27      1837    531422    0.151          -               -
  -> 70/70 shipped files accepted with the known value (and agreeing with brute force / path oracle where run)

C. FORWARD-SEARCH CERTIFICATES (PLAIN dialect; selection/runs/*/n*_i*_k*/algorithms/*.rs)
   expect = k from the directory name n{n}_i{i1}_k{k} (claimed worst case); the file name gives n, 0-based i.
  file                         n   i verdict worst expect functions     bytes  check s      brute           paths
  E002b/n12_i6_k20/12_5.rs    12   6 ACCEPT     20     20       126     29590    0.008          -      ok w=20 9s   claims 715, not implied 0
  E002b/n12_i6_k21/12_5.rs    12   6 ACCEPT     21     21       101     23376    0.007          -     ok w=21 11s   claims 516, not implied 0
  E002b/n12_i6_k22/12_5.rs    12   6 ACCEPT     22     22       106     24199    0.006          -     ok w=22 16s   claims 507, not implied 0
  E002b/n13_i7_k23/13_6.rs    13   7 ACCEPT     23     23       296     73829    0.022          -               -   claims 2024, not implied 0
  E002b/n13_i7_k24/13_6.rs    13   7 ACCEPT     24     24        77     18155    0.005          -               -   claims 403, not implied 0
  E002b/n13_i7_k25/13_6.rs    13   7 ACCEPT     25     25       206     50210    0.014          -               -   claims 1227, not implied 0
  E002b/n14_i7_k25/14_6.rs    14   7 ACCEPT     25     25       328     82177    0.024          -               -   claims 2298, not implied 0
  E002b/n14_i7_k26/14_6.rs    14   7 ACCEPT     26     26       171     41720    0.013          -               -   claims 1029, not implied 0
  E002b/n14_i7_k27/14_6.rs    14   7 ACCEPT     27     27        80     19195    0.007          -               -   claims 431, not implied 0
  -> 9/9 forward-search files accepted with the claimed worst case

D. SYNTHETIC PROGRAMS (make_synthetic.py; random, correct, not optimal; both dialects)
   every program: static ACCEPT; brute force (n <= 8) and path oracle agree; equal worst case.
   features covered: let: 224, fresh let names: 62, !is_dual: 1270, leaf returns at arity > 1: 158, run wrapper: 48, run(input, true): 13
  -> 96/96 synthetic programs accepted and confirmed

E. MUTATION TESTS
   ground truth: brute force over all n! inputs (n <= 8) and/or exhaustive path oracle (n <= 12)

   operator           mutants  rejected | wrong/ill-formed  rejected | behaviour-correct  accepted | accepted-but-wrong
   swap_branches          280       280 |              280       280 |                 0         0 |     0
   flip_operands          280       280 |              280       280 |                 0         0 |     0
   replace_operand        854       848 |              845       845 |                 9         6 |     0
   half_condition         164       164 |               36        36 |               128         0 |     0
   remove_comparison      395       395 |              395       395 |                 0         0 |     0
   tuple_value            304       304 |              304       304 |                 0         0 |     0
   tuple_swap              94        81 |               81        81 |                13        13 |     0
   change_return          163       163 |              163       163 |                 0         0 |     0
   replace_arg           4182      4182 |             4182      4182 |                 0         0 |     0
   drop_arg              1041      1041 |             1041      1041 |                 0         0 |     0
   swap_args             1921      1654 |             1654      1654 |               267       267 |     0
   flip_dual              206       190 |              190       190 |                16        16 |     0
   redirect_call         1012       976 |              918       918 |                93        36 |     0
   doc_rank               598       598 |                0         0 |               598         0 |     0
   root_rank              147       147 |              119       119 |                28         0 |     0
   drop_claims            103         0 |                0         0 |               103       103 |     0
   bogus_claim            302         0 |                0         0 |               302       302 |     0
   TOTAL                12046     11303 |            10488     10488 |              1557       743 |     0
   semantic mutants (wrong or ill-formed) rejected: 10488/10488 = 100.00%
   accepted mutants whose certified worst case is below the true worst case: 0
   undecided (cyclic, n > 8, no error in 20000 random inputs): 1, rejected: 1
   doc-claim mutants (comments only): 405, accepted 405; bogus claims reported as not implied: 275/302

   behaviour-correct mutants that the checker REJECTS, by reason (normalised message):
     replace_operand         3  #: compares # and # whose order is already known from the precondition (wasted comparison,
     half_condition        128  comparison is not of the form (!is_dual && X < Y) || (is_dual && X > Y) with the same X, Y
     redirect_call          29  run's callee must be the only function that is never called; never-called: #
     redirect_call          20  cannot determine the root: expected exactly one function that is never called, found #: #
     redirect_call           7  the call graph has # cycle (involving #.#. #)
     redirect_call           1  # function(s) unreachable from the root #: #
     doc_rank              575  #: the answer has rank # among the # arguments, but # claims to select rank #
     doc_rank               23  #: the answer has rank # among the # arguments in the flipped order, but # claims to selec
     root_rank              28  const I = # but the root selects rank #
   one example per operator:
     4_1.rs ('half_condition', 1): [PARSE] line 7: comparison is not of the form (!is_dual && X < Y) || (is_dual && X > Y) with the same X, Y
     4_1.rs ('redirect_call', 5, 'select_0'): [LINK] cannot determine the root: expected exactly one function that is never called, found 0: []
     4_1.rs ('doc_rank', 1, 1): [CHECK] line 16: select_2: the answer has rank 0 among the 2 arguments, but select_3 claims to select rank 1
     syn_plain_5_2_s1 ('root_rank', 18, 0): [CHECK] const I = 2 but the root selects rank 0
     syn_plain_7_3_s1 ('replace_operand', 7, 0, 'c'): [CHECK] line 58: select_16: compares b and a whose order is already known from the precondition (wasted comparison, dead branch)

F. STRESS TEST (large synthetic files; static check only)
   big_12.rs                   12.4 MB  ACCEPT  n=12 i=7 worst=32  functions=42902  2.6 s
   big_13_4p.rs                22.1 MB  ACCEPT  n=13 i=5 worst=34  functions=71806  5.6 s
   big_13_5d.rs                36.6 MB  ACCEPT  n=13 i=6 worst=36  functions=123420  7.9 s
   big_14_4p.rs                46.3 MB  ACCEPT  n=14 i=5 worst=38  functions=145365  11.5 s

Interpretation of the mutation table (section E):
  * "wrong/ill-formed" = some input gives a wrong answer or the program would
    not compile (ground truth from reference_semantics.py).  ALL of them are
    rejected (10488/10488).  No mutant was accepted with a wrong answer or a
    too-small worst case.
  * Behaviour-correct mutants that are ACCEPTED are genuinely harmless:
    swapping two arguments the callee treats symmetrically (swap_args,
    tuple_swap), flipping is_dual on a call whose callee is symmetric
    (e.g. arity 1), redirecting to another function with the same rank that
    is also valid with the knowledge at that call site, replacing a compared
    operand by an equivalent one.  Brute force / path oracle confirm them.
  * Behaviour-correct mutants that are REJECTED, all by deliberate policy
    (section 7): changed internal doc ranks (the ranks are the certificate;
    behaviour does not read them), `half_condition` (the dual half
    `is_dual && X > Y` changed; harmless only because that function never
    runs with is_dual = true; rejected by the grammar), redirections that
    create dead code / a second root / a cycle, root rank contradicting
    const I (behaviour follows I), and 3 wasted comparisons.
  * Doc-claim mutants (drop all relations of a function / add a bogus one)
    never change the verdict (405/405 accepted), and bogus relations that
    are not implied are reported as warnings (275/302; the other 27 bogus
    relations happen to be implied by the computed precondition).

Additional checks (run separately, not part of run_tests.py):
  * Forward files: in every function of all 9 files, closure(doc-comment
    relations) == the computed precondition P_f, and the listed relations are
    exactly its cover relations (no function where P_f is strictly stronger).
  * 300,000 random inputs through the literal interpreter
    (reference_semantics.execute): n13_i7_k24/13_6.rs correct, max 24
    comparisons observed; n14_i7_k27/14_6.rs correct, max 27 observed
    (= the certified W in both cases, so W is attained).
  * Exhaustive path oracle (reference_semantics.path_oracle, i.e. ALL inputs)
    on three forward-search certificates of the intermediate runs:
      n12_i6_k20/12_5.rs  correct, exact worst case 20   (in run_tests, 778,240 paths)
      n13_i7_k24/13_6.rs  correct, exact worst case 24   (5,184,000 paths, 35 s)
      n14_i7_k27/14_6.rs  correct, exact worst case 27   (41,472,000 paths, 297 s)


10. HOW TO AUDIT THIS IN 30 MINUTES
-----------------------------------
  1. Read sections 4-6 of this file (10 min).
  2. verify_rs_algorithm.py, in this order (15 min):
       check_stmt()   rules R2, R3 (the heart: certification of dropped
                      elements, rank bookkeeping, intersection of preconditions)
       check()        R1, order of processing, longest path W
       add_relation(), restrict()   closure and mapping to callee positions,
                      including the reversal for !is_dual
       topological_order()          reachability and acyclicity
       parse() / link()             grammar and name resolution; body() shows
                      the let desugaring (substitute())
  3. Skim test_results.txt (5 min): unit-test verdicts, the shipped-file
     table, and the mutation table (accepted-but-wrong must be 0).
