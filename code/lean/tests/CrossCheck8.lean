import Selection.Cert8

/-! Cross-check of the translator (tools/crosscheck.py drives this file):
    reads permutations (one per line, space-separated) from
    ../../out/lean/perms8.txt, runs the Lean program `Cert8.prog` (V_8(16) certificate) on each with
    the spec semantics `exec`, writes "v c" per line to
    ../../out/lean/lean_results8.txt.  Not part of the proof. -/

open Sel Sel.Cert8

#eval show IO Unit from do
  let txt ← IO.FS.readFile "../../out/lean/perms8.txt"
  let mut out : Array String := #[]
  for line in txt.splitOn "\n" do
    if line.isEmpty then continue
    let xs := (line.splitOn " ").map String.toNat!
    match exec prog prog.funs.size prog.root xs with
    | some (v, c) => out := out.push s!"{v} {c}"
    | none => out := out.push "none"
  IO.FS.writeFile "../../out/lean/lean_results8.txt" (String.intercalate "\n" out.toList ++ "\n")
  IO.println s!"crosscheck: ran {out.size} inputs"
