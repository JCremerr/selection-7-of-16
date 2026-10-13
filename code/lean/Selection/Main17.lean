import Selection.Spec
import Selection.Cert5of17
import Selection.Cert6of17
import Selection.Cert7of17
import Selection.Cert8of17
import Selection.Cert9of17
import Selection.Sound
import Selection.Kernel5of17
import Selection.Kernel6of17
import Selection.Kernel7of17
import Selection.Kernel8of17
import Selection.Kernel9of17

/-! MAIN THEOREMS FOR n = 17  (trusted: these statements and `Spec.lean`; to
    identify `CertKof17.prog` with the Rust file also CertKof17.lean, see
    README.txt).  Same shape as Main.lean / Main8.lean.

    `Sel.CertKof17.prog` is the certificate select_Kth_of_17_Wcmp.rs translated
    to Lean data.  For every list of 17 distinct naturals, running it returns
    one of the inputs that has exactly K-1 inputs smaller than it (i.e. the
    K-th smallest), using at most W comparisons.  The second theorem of each
    pair is the bound V_K(17) <= W itself (every `Program` is a comparison
    algorithm, see Main.lean). -/

namespace Sel

/-- select_5th_of_17_27cmp.rs (146 functions): V_5(17) <= 27. -/
theorem select_5th_of_17_le_27 :
    ∀ xs : List Nat, xs.length = 17 → xs.Nodup →
      ∃ v c, exec Cert5of17.prog Cert5of17.prog.funs.size Cert5of17.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 4 ∧ c ≤ 27 :=
  sound Cert5of17.prog Cert5of17.hints 17 4 27 Cert5of17.check_ok

theorem V_5_17_le_27 : ∃ P : Program, SelectsWithin P 17 4 27 :=
  ⟨Cert5of17.prog, sound Cert5of17.prog Cert5of17.hints 17 4 27 Cert5of17.check_ok⟩

/-- select_6th_of_17_29cmp.rs (266 functions): V_6(17) <= 29. -/
theorem select_6th_of_17_le_29 :
    ∀ xs : List Nat, xs.length = 17 → xs.Nodup →
      ∃ v c, exec Cert6of17.prog Cert6of17.prog.funs.size Cert6of17.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 5 ∧ c ≤ 29 :=
  sound Cert6of17.prog Cert6of17.hints 17 5 29 Cert6of17.check_ok

theorem V_6_17_le_29 : ∃ P : Program, SelectsWithin P 17 5 29 :=
  ⟨Cert6of17.prog, sound Cert6of17.prog Cert6of17.hints 17 5 29 Cert6of17.check_ok⟩

/-- select_7th_of_17_30cmp.rs (977 functions): V_7(17) <= 30. -/
theorem select_7th_of_17_le_30 :
    ∀ xs : List Nat, xs.length = 17 → xs.Nodup →
      ∃ v c, exec Cert7of17.prog Cert7of17.prog.funs.size Cert7of17.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 6 ∧ c ≤ 30 :=
  sound Cert7of17.prog Cert7of17.hints 17 6 30 Cert7of17.check_ok

theorem V_7_17_le_30 : ∃ P : Program, SelectsWithin P 17 6 30 :=
  ⟨Cert7of17.prog, sound Cert7of17.prog Cert7of17.hints 17 6 30 Cert7of17.check_ok⟩

/-- select_8th_of_17_31cmp.rs (1077 functions): V_8(17) <= 31. -/
theorem select_8th_of_17_le_31 :
    ∀ xs : List Nat, xs.length = 17 → xs.Nodup →
      ∃ v c, exec Cert8of17.prog Cert8of17.prog.funs.size Cert8of17.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 7 ∧ c ≤ 31 :=
  sound Cert8of17.prog Cert8of17.hints 17 7 31 Cert8of17.check_ok

theorem V_8_17_le_31 : ∃ P : Program, SelectsWithin P 17 7 31 :=
  ⟨Cert8of17.prog, sound Cert8of17.prog Cert8of17.hints 17 7 31 Cert8of17.check_ok⟩

/-- select_9th_of_17_31cmp.rs (1493 functions): V_9(17) <= 31. -/
theorem select_9th_of_17_le_31 :
    ∀ xs : List Nat, xs.length = 17 → xs.Nodup →
      ∃ v c, exec Cert9of17.prog Cert9of17.prog.funs.size Cert9of17.prog.root xs = some (v, c) ∧
        v ∈ xs ∧ (xs.filter (· < v)).length = 8 ∧ c ≤ 31 :=
  sound Cert9of17.prog Cert9of17.hints 17 8 31 Cert9of17.check_ok

theorem V_9_17_le_31 : ∃ P : Program, SelectsWithin P 17 8 31 :=
  ⟨Cert9of17.prog, sound Cert9of17.prog Cert9of17.hints 17 8 31 Cert9of17.check_ok⟩

end Sel
