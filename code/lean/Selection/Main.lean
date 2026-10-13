import Selection.Spec
import Selection.Cert
import Selection.Sound
import Selection.Kernel

/-! THE MAIN THEOREM  (trusted: this statement, `Spec.lean`, and that
    `Cert.prog` is the certificate's algorithm, see README.txt).

    `Sel.Cert.prog` is select_7th_of_16_28cmp.rs translated to Lean data
    (Cert.lean).  For every list of 16 distinct naturals, running it returns
    one of the inputs that has exactly 6 inputs smaller than it (i.e. the 7th
    smallest), using at most 28 comparisons.  Hence V_7(16) <= 28. -/

namespace Sel

theorem select_7th_of_16_le_28 :
    ∀ xs : List Nat, xs.length = 16 → xs.Nodup →
      ∃ v c, exec Cert.prog Cert.prog.funs.size Cert.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 6 ∧ c ≤ 28 :=
  sound Cert.prog Cert.hints 16 6 28 Cert.check_ok

/-- The bound itself.  Every `Program` is a comparison algorithm (`exec` touches
    the input values only through `<` and by passing them on), so this says
    V_7(16) <= 28 whatever one thinks of the translation .rs -> `Cert.prog`. -/
theorem V_7_16_le_28 : ∃ P : Program, SelectsWithin P 16 6 28 :=
  ⟨Cert.prog, sound Cert.prog Cert.hints 16 6 28 Cert.check_ok⟩

end Sel
