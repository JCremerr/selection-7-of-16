INDEPENDENT EXHAUSTIVE CHECKS OF THE LOWER BOUNDS (paper, Section "Lower bounds")
=================================================================================
Written from the paper text only; no other code of this repository is used
(own exact solver, own canonical form).

  sel.cpp      exact V_i(P) and minimum leaf count for all posets up to
               isomorphism, n <= 9.  Checks ceil(log2 Phi*) <= V, KEY <= V,
               Phi*, |C|, FG, FGd <= minimum number of leaves, the local
               inequality of Theorem "Phi*", and reduction invariance.
               Poset counts match OEIS A000112.
  leafcount.py leaf count / depth profile of a certificate (used for the
               248,381,440 leaves of the 252-function V_8(16) certificate):
               python3 -I leafcount.py ../../../certificates/V8_16_le29/select_8th_of_16_29cmp.rs
  out9.txt     output of ./sel 9 9 (zero violations).

Build and run:  g++ -O2 -o sel sel.cpp && ./sel 9 9 > out9.txt
