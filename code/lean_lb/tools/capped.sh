#!/bin/bash
# capped.sh MEMMAX COMMAND...   run COMMAND in code/lean_lb in the foreground
# under a systemd memory cap (only this job is killed if it exceeds MEMMAX).
# Uses the elan shims in ~/.elan/bin; the toolchain comes from ./lean-toolchain
# (leanprover/lean4:v4.34.1), never the elan default.
M=$1; shift
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." || exit 1
export PATH=$HOME/.elan/bin:/usr/bin:/bin
exec systemd-run --user --scope --quiet -p MemoryMax="$M" -p MemorySwapMax=0 \
  -p CPUWeight=50 "$@"
