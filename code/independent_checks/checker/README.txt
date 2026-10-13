INDEPENDENT AUDIT OF THE STATIC CHECKER
======================================
Does not import the checker (code/verifier).  "All 16 archived certificates" refers
to the 9 certificates of this repository plus 7 superseded weaker ones that are
not included here.

  oracle.py, pathcheck.py  own parser + exact path-sensitive oracle (no
               precondition intersection, doc ranks ignored, ranks propagated
               from the root).  All 16 archived certificates correct; exact
               worst case = the checker's W in every one; every function is
               reached with exactly one poset.
  mutate.py, boundary.py   3,450 mutants of the real certificates (329, 365,
               252, 2937, V5..V9(17) files): 0 accepted-but-wrong.
  closure_test.py          add_relation/restrict vs plain Warshall closure,
               5,745 random cases up to m = 17: 0 mismatches.
  parse_adv.py             parser edge cases (unused let slots are accepted;
               harmless, see verifier README section 8).
  hand/        hand-built cases (transitivity via a dropped element, let swaps,
               shared function with one weak caller, drop one short of
               certified); checker verdicts agree with rustc execution.
  cov/         instrumentation: on the same 2e7 randtest permutations every
               call edge is exercised (min hits 55 / 615 / 29 / 52 for the
               329 / 252 / 2937 / 1493-function files).
  rerun.txt    strict re-verification of all 9 certificates in the paper's
               table, and rejection at budget - 1.
