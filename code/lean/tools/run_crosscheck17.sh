#!/bin/bash
# Translator cross-checks for the five n = 17 certificates, run from the
# project dir: structural (CertKof17.lean vs reference_semantics' parse of the
# .rs) and behavioural (Lean `exec` vs reference interpreter on 20002 inputs:
# identity, reverse, 20000 random permutations of 0..16, seed 17000+K).
# Lean itself must be run memory-capped by the caller.
set -e
mkdir -p ../../out/lean
C=../../certificates
X=../../out/lean
declare -A RS=([5]=V5_17_le27/select_5th_of_17_27cmp.rs [6]=V6_17_le29/select_6th_of_17_29cmp.rs
  [7]=V7_17_le30/select_7th_of_17_30cmp.rs [8]=V8_17_le31/select_8th_of_17_31cmp.rs
  [9]=V9_17_le31/select_9th_of_17_31cmp.rs)
declare -A W=([5]=27 [6]=29 [7]=30 [8]=31 [9]=31)
case "$1" in
  pre)
    for k in 5 6 7 8 9; do
      python3 -I -B tools/crosscheck.py struct $C/${RS[$k]} Selection/Cert${k}of17.lean
      python3 -I -B tools/crosscheck.py perms $X/perms$k.txt 20000 $((17000 + k)) 17
    done ;;
  post)
    for k in 5 6 7 8 9; do
      python3 -I -B tools/crosscheck.py compare $C/${RS[$k]} $X/perms$k.txt $X/lean_results$k.txt ${W[$k]}
    done ;;
  *) echo "usage: run_crosscheck17.sh pre|post   (between them: lake env lean tests/CrossCheck17.lean)"; exit 2 ;;
esac
