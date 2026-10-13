#!/bin/bash
# Translator cross-check (M2), run from code/lean:
#   structural (Cert.lean vs reference_semantics parse of the .rs) and
#   behavioural (Lean `exec` vs reference interpreter on 20002 inputs).
set -e
mkdir -p ../../out/lean
RS=../../certificates/V7_16_le28/select_7th_of_16_28cmp.rs
R=../../out/lean
python3 -I tools/crosscheck.py struct $RS Selection/Cert.lean
python3 -I tools/crosscheck.py perms $R/perms.txt 20000 2027
lake env lean tests/CrossCheck.lean
python3 -I tools/crosscheck.py compare $RS $R/perms.txt $R/lean_results.txt
rm -f $R/perms.txt $R/lean_results.txt
