#!/bin/bash
# Replay every module of the library through the kernel (leanchecker), one
# module at a time (in parallel the replays need > 2 GB).  Run from code/lean.
# Optional arguments: module names (default: all).
set -e
MODS="$*"
[ -z "$MODS" ] && MODS="Selection.Spec Selection.Checker Selection.Count Selection.Hints Selection.Cert
  Selection.Sound Selection.Kernel Selection.Main Selection.Fast Selection.Cert8 Selection.Hints8
  Selection.Kernel8 Selection.Main8
  Selection.Cert5of17 Selection.Hints5of17 Selection.Kernel5of17
  Selection.Cert6of17 Selection.Hints6of17 Selection.Kernel6of17
  Selection.Cert7of17 Selection.Hints7of17 Selection.Kernel7of17
  Selection.Cert8of17 Selection.Hints8of17 Selection.Kernel8of17
  Selection.Cert9of17 Selection.Hints9of17 Selection.Kernel9of17
  Selection.Main17"
# NOT the root `Selection`: leanchecker treats a module name as a PREFIX, so
# `leanchecker Selection` replays all modules in parallel (OOM at 5 GB).
for m in $MODS; do
  /usr/bin/time -f "  $m: %e s, max RSS %M KB" lake env leanchecker $m
done
echo "leanchecker: all modules OK ($(echo $MODS | wc -w) modules)"
