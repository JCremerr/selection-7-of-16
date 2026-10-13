import Selection.Cert5of17
import Selection.Cert6of17
import Selection.Cert7of17
import Selection.Cert8of17
import Selection.Cert9of17

/-! Cross-check of the translator for the five n = 17 certificates
    (tools/run_crosscheck17.sh drives this file): for K = 5..9 reads
    permutations from ../../out/lean/permsK.txt, runs `CertKof17.prog` on each with the
    spec semantics `exec`, writes "v c" per line to ../../out/lean/lean_resultsK.txt.
    Not part of the proof. -/

open Sel

def runFile (P : Program) (inp out : String) : IO Unit := do
  let txt ← IO.FS.readFile inp
  let mut res : Array String := #[]
  for line in txt.splitOn "\n" do
    if line.isEmpty then continue
    let xs := (line.splitOn " ").map String.toNat!
    match exec P P.funs.size P.root xs with
    | some (v, c) => res := res.push s!"{v} {c}"
    | none => res := res.push "none"
  IO.FS.writeFile out (String.intercalate "\n" res.toList ++ "\n")
  IO.println s!"crosscheck: {inp}: ran {res.size} inputs"

#eval runFile Cert5of17.prog "../../out/lean/perms5.txt" "../../out/lean/lean_results5.txt"
#eval runFile Cert6of17.prog "../../out/lean/perms6.txt" "../../out/lean/lean_results6.txt"
#eval runFile Cert7of17.prog "../../out/lean/perms7.txt" "../../out/lean/lean_results7.txt"
#eval runFile Cert8of17.prog "../../out/lean/perms8.txt" "../../out/lean/lean_results8.txt"
#eval runFile Cert9of17.prog "../../out/lean/perms9.txt" "../../out/lean/lean_results9.txt"
