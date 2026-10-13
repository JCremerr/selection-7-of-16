#!/bin/bash
# Replay every module of the LB library through the kernel (leanchecker), one
# module at a time.  Run from code/lean_lb (through tools/capped.sh).
# leanchecker treats a module name as a PREFIX, so the root `LB` would replay
# all LB.* modules at once; we list them explicitly.
# Optional: `tools/leancheck_all.sh --fresh` additionally replays LB.Main
# FROM SCRATCH, i.e. together with every imported declaration (core + the
# Mathlib modules it uses), not trusting any .olean.
set -e
MODS="LB.Spec LB.Basic LB.Count LB.Closure LB.World LB.Local LB.Tree LB.Key
  LB.Exists LB.Optimality LB.Family LB.Examples LB.Main"
for m in $MODS; do
  /usr/bin/time -f "  $m: %e s, max RSS %M KB" lake env leanchecker $m
done
echo "leanchecker: all modules OK ($(echo $MODS | wc -w) modules)"
if [ "$1" = "--fresh" ]; then
  /usr/bin/time -f "  LB.Main --fresh: %e s, max RSS %M KB" lake env leanchecker --fresh LB.Main
  echo "leanchecker --fresh LB.Main: OK"
fi
