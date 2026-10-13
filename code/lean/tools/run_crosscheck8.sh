#!/bin/bash
# Translator cross-check for the V_8(16) certificate (see run_crosscheck.sh).
set -e
mkdir -p ../../out/lean
RS=../../certificates/V8_16_le29/select_8th_of_16_29cmp_alt_dfpn.rs
R=../../out/lean
python3 -I tools/crosscheck.py struct $RS Selection/Cert8.lean
python3 -I tools/crosscheck.py perms $R/perms8.txt 20000 2028
lake env lean tests/CrossCheck8.lean
python3 -I tools/crosscheck.py compare $RS $R/perms8.txt $R/lean_results8.txt
rm -f $R/perms8.txt $R/lean_results8.txt
