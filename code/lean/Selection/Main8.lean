import Selection.Spec
import Selection.Cert8
import Selection.Sound
import Selection.Kernel8

/-! SECOND MAIN THEOREM: V_8(16) <= 29  (trusted: these statements and
    `Spec.lean`; to identify `Cert8.prog` with the Rust file also Cert8.lean,
    see README.txt).

    `Sel.Cert8.prog` is select_8th_of_16_29cmp_alt_dfpn.rs (2937 functions)
    translated to Lean data (Cert8.lean).  For every list of 16 distinct
    naturals, running it returns one of the inputs that has exactly 7 inputs
    smaller than it (i.e. the 8th smallest), using at most 29 comparisons. -/

namespace Sel

theorem select_8th_of_16_le_29 :
    ∀ xs : List Nat, xs.length = 16 → xs.Nodup →
      ∃ v c, exec Cert8.prog Cert8.prog.funs.size Cert8.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 7 ∧ c ≤ 29 :=
  sound Cert8.prog Cert8.hints 16 7 29 Cert8.check_ok

/-- The bound itself (every `Program` is a comparison algorithm, see Main.lean). -/
theorem V_8_16_le_29 : ∃ P : Program, SelectsWithin P 16 7 29 :=
  ⟨Cert8.prog, sound Cert8.prog Cert8.hints 16 7 29 Cert8.check_ok⟩

end Sel
