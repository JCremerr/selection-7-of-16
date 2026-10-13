#!/bin/bash
# Usage: build_and_run.sh ALGO_FILE TRIALS
# Compiles the certificate with rustc into a fresh temp dir ($TMPDIR or /tmp)
# and runs it on TRIALS pseudorandom permutations (a test, not a proof).
set -e
D=$(mktemp -d "${TMPDIR:-/tmp}/randtest.XXXXXX")
ALGO_FILE=$(readlink -f "$1") rustc -O --edition 2021 -o "$D/rt" "$(dirname "$(readlink -f "$0")")/randtest.rs" 2>&1 | grep -v "^warning\|^ *|\|^ *=\|^$" | head -5 || true
"$D/rt" "$2"
