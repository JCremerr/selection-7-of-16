#!/bin/bash
# Usage: run_budget.sh N I1 K CACHE_GB OUTDIR [TIMEOUT_S]
# Runs the patched upstream forward search (first-solution mode) for the
# I1-th smallest (1-BASED) of N with fixed budget K. Output + algorithm in OUTDIR.
set -u
N=$1; I1=$2; K=$3; GB=$4; OUT=$5; TO=${6:-3600}
BIN=${SELGEN_BIN:?set SELGEN_BIN to the patched upstream binary}
I0=$((I1-1))
mkdir -p "$OUT/n${N}_i${I1}_k${K}" && cd "$OUT/n${N}_i${I1}_k${K}" || exit 1
/usr/bin/time -v timeout "$TO" "$BIN" --search-mode forward --single -n "$N" -i "$I0" \
  --budget "$K" --weight-function "${SELGEN_WF:-w}" --print-algorithm --max-cache-size $((GB*1024*1024*1024)) > out.txt 2> err.txt
echo "exit=$?" >> out.txt
grep -a -E "Comparisons:|needs at least|no algorithm|resolves|Posets searched|Cache entries|Duration" out.txt | tail -8
grep -E "Elapsed|Maximum resident" err.txt
