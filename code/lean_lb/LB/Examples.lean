/-
  Sanity checks (NOT part of the universal proofs; they show that the
  definitions compute the paper's numbers and that the hypotheses of the main
  theorems are satisfiable).

  1. Minimum of n unordered labels (any n ≥ 1): FG = 2^{n-1}, FGdual = |C| = n,
     Phi* = 2^{n-1}, KEY = n - 1, V_1(E_n) = n - 1 exactly, and the linear scan
     is a correct tree with exactly Phi* = 2^{n-1} leaves (the leaf bound of
     thm:phi is attained).  For n = 4: FG = 8, FGdual = 4, |C| = 4, Phi* = 8,
     KEY = 3, V = 3 (short.tex: "For the minimum of four elements, FG = 8,
     FGdual = 4, |C| = 4, and Phi* = 8 gives the correct bound 3").
  2. The median of {a, b, c} with b < c known (main.tex, three-element
     example): |C| = 3, FG = FGdual = Phi* = 3, KEY = 2, evaluated by the
     kernel (`decide`) on a computable restatement of the definitions.
-/
import LB.Optimality

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### 1. The minimum of `n` unordered labels -/

section AntichainMin

lemma card_C_antichain_min (hn : 1 ≤ Fintype.card α) :
    (C (antichain α) 1).card = Fintype.card α := by
  rw [card_C_eq_sum_b _ le_rfl hn]
  simp [Down_antichain_zero, bOf]
  omega

lemma FG_antichain_min : FG (antichain α) 1 = 2 ^ (Fintype.card α - 1) := by
  simp [FG, Down_antichain_zero, bOf]

lemma FGdual_antichain_min (hn : 1 ≤ Fintype.card α) :
    FGdual (antichain α) 1 = Fintype.card α := by
  rw [← card_C_antichain_min hn, card_C_eq_sum_a _ le_rfl, FGdual]
  refine Finset.sum_congr rfl fun A hA => ?_
  have : aOf (antichain α) A = 0 := by
    simp only [aOf, maximals_antichain, (mem_Down.1 hA).2]
  simp [this]

lemma Phi_antichain_min (hn : 1 ≤ Fintype.card α) :
    Phi (antichain α) 1 = 2 ^ (Fintype.card α - 1) := by
  unfold Phi
  rw [FG_antichain_min, FGdual_antichain_min hn, card_C_antichain_min hn]
  push_cast; ring

lemma KEY_antichain_min (hn : 1 ≤ Fintype.card α) :
    KEY (antichain α) 1 = Fintype.card α - 1 := by
  have hterm : ∀ s ∈ C (antichain α) 1,
      aOf (antichain α) (insert s.2 s.1) + bOf (antichain α) s.1 = Fintype.card α - 1 := by
    intro s hs
    have h1 : s.1 = ∅ := by
      have := (mem_C.1 hs).1
      rwa [show (1 : ℕ) - 1 = 0 from rfl, Down_antichain_zero, Finset.mem_singleton] at this
    simp [aOf, bOf, h1]
  apply le_antisymm
  · exact Finset.sup_le fun s hs => (hterm s hs).le
  · obtain ⟨s, hs⟩ := Finset.card_pos.1 (by rw [card_C_antichain_min hn]; omega :
      0 < (C (antichain α) 1).card)
    have hle := Finset.le_sup (f := fun s : Finset α × α =>
      aOf (antichain α) (insert s.2 s.1) + bOf (antichain α) s.1) hs
    have := hterm s hs
    unfold KEY
    omega

/-- Finding the minimum of `n` labels needs exactly `n - 1` comparisons. -/
theorem V_antichain_min (hn : 1 ≤ Fintype.card α) :
    V (antichain α) 1 = Fintype.card α - 1 :=
  le_antisymm (V_antichain_min_le hn)
    (KEY_antichain_min hn ▸ KEY_le_V (antichain α) le_rfl hn)

lemma CTree.leaves_scanMin {α : Type*} (m : α) (l : List α) :
    (CTree.scanMin m l).leaves = 2 ^ l.length := by
  induction l generalizing m with
  | nil => rfl
  | cons j js ih => simp [CTree.scanMin, CTree.leaves, ih, pow_succ]; ring

/-- The leaf bound of `thm:phi` is attained: the linear scan is a correct tree
for the minimum with exactly `Phi*(E_n) = 2^{n-1}` leaves. -/
theorem scanMin_tight (hn : 1 ≤ Fintype.card α) (m : α) :
    Correct (antichain α) 1 (CTree.scanMin m (univ.erase m).toList) ∧
    ((CTree.scanMin m (univ.erase m).toList).leaves : ℤ) = Phi (antichain α) 1 := by
  refine ⟨?_, ?_⟩
  · intro f _
    obtain ⟨-, h2, -⟩ := CTree.scanMin_spec f (univ.erase m).toList m
    rw [Finset.card_eq_zero, Finset.filter_eq_empty_iff]
    intro u _ hu
    have := h2 u (mem_cons_erase_toList m u)
    omega
  · rw [CTree.leaves_scanMin, length_erase_toList, Phi_antichain_min hn]
    push_cast; rfl

/-- The paper's numbers for the minimum of four elements. -/
example : FG (antichain (Fin 4)) 1 = 8 ∧ FGdual (antichain (Fin 4)) 1 = 4 ∧
    (C (antichain (Fin 4)) 1).card = 4 ∧ Phi (antichain (Fin 4)) 1 = 8 ∧
    KEY (antichain (Fin 4)) 1 = 3 ∧ V (antichain (Fin 4)) 1 = 3 ∧
    Nat.clog 2 (Phi (antichain (Fin 4)) 1).toNat = 3 := by
  have hn : 1 ≤ Fintype.card (Fin 4) := by simp
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · rw [FG_antichain_min]; rfl
  · rw [FGdual_antichain_min hn]; rfl
  · rw [card_C_antichain_min hn]; rfl
  · rw [Phi_antichain_min hn]; rfl
  · rw [KEY_antichain_min hn]; rfl
  · rw [V_antichain_min hn]; rfl
  · rw [Phi_antichain_min hn]; rfl

end AntichainMin

/-! ### 2. Computable restatements (for small examples) -/

section Computable

variable (P : SPO α) [DecidableRel P.lt]

instance decIsDownSet : DecidablePred (IsDownSet P) := fun S => by
  unfold IsDownSet; infer_instance

lemma Down_eq_filter (j : ℕ) :
    Down P j = (univ : Finset (Finset α)).filter (fun S => IsDownSet P S ∧ S.card = j) := by
  ext S; rw [mem_Down, Finset.mem_filter]; simp

lemma minimals_eq_filter (S : Finset α) :
    minimals P S = S.filter (fun v => ∀ u ∈ S, ¬ P.lt u v) := by
  ext v; rw [mem_minimals, Finset.mem_filter]

lemma maximals_eq_filter (S : Finset α) :
    maximals P S = S.filter (fun v => ∀ u ∈ S, ¬ P.lt v u) := by
  ext v; rw [mem_maximals, Finset.mem_filter]

/-- `C(P)` as the explicit set of pairs (down-set, candidate). -/
lemma C_eq_biUnion (i : ℕ) :
    C P i = (Down P (i - 1)).biUnion fun L => (minimals P Lᶜ).image fun e => (L, e) := by
  ext s
  rw [mem_C, Finset.mem_biUnion]
  constructor
  · rintro ⟨h1, h2⟩
    exact ⟨s.1, h1, Finset.mem_image.2 ⟨s.2, h2, rfl⟩⟩
  · rintro ⟨L, hL, he⟩
    obtain ⟨e, he', rfl⟩ := Finset.mem_image.1 he
    exact ⟨hL, he'⟩

end Computable

/-! ### 3. The median of three with `b < c` known -/

section Median

/-- Labels `a, b, c = 0, 1, 2`; the only known relation is `b < c`. -/
def medP : SPO (Fin 3) where
  lt u v := u = 1 ∧ v = 2
  irrefl u h := by have := h.1.symm.trans h.2; exact absurd this (by decide)
  trans h₁ h₂ := by have := h₁.2.symm.trans h₂.1; exact absurd this (by decide)

instance : DecidableRel medP.lt := fun u v => by unfold medP; infer_instance

example : (C medP 2).card = 3 ∧ FG medP 2 = 3 ∧ FGdual medP 2 = 3 ∧ Phi medP 2 = 3 ∧
    KEY medP 2 = 2 := by
  have hC : (C medP 2).card = 3 := by
    rw [C_eq_biUnion, Down_eq_filter]
    simp only [minimals_eq_filter]
    decide
  have hFG : FG medP 2 = 3 := by
    unfold FG bOf
    rw [Down_eq_filter]
    simp only [minimals_eq_filter]
    decide
  have hFGd : FGdual medP 2 = 3 := by
    unfold FGdual aOf
    rw [Down_eq_filter]
    simp only [maximals_eq_filter]
    decide
  refine ⟨hC, hFG, hFGd, ?_, ?_⟩
  · unfold Phi; rw [hC, hFG, hFGd]; rfl
  · unfold KEY aOf bOf
    rw [C_eq_biUnion, Down_eq_filter]
    simp only [minimals_eq_filter, maximals_eq_filter]
    decide

end Median

/-! ### 4. Negative controls: `Correct` is not trivially satisfiable -/

/-- Returning a fixed label without comparing is not correct for the minimum of
two labels (input `f = (1, 0)`). -/
example : ¬ Correct (antichain (Fin 2)) 1 (.leaf 0) := by
  intro h
  have := h ![1, 0] ⟨by decide, fun _ _ h => h.elim⟩
  revert this
  decide

/-- With the outcome branches swapped, a one-comparison tree is wrong. -/
example : ¬ Correct (antichain (Fin 2)) 1 (.node 0 1 (.leaf 1) (.leaf 0)) := by
  intro h
  have := h ![0, 1] ⟨by decide, fun _ _ h => h.elim⟩
  revert this
  decide

/-- One comparison is a correct tree for the minimum of two labels. -/
example : Correct (antichain (Fin 2)) 1 (.node 0 1 (.leaf 0) (.leaf 1)) := by
  intro f hf
  have h01 : f 0 ≠ f 1 := hf.1.ne (by decide)
  simp only [CTree.eval]
  split_ifs with h
  all_goals
    rw [Finset.card_eq_zero, Finset.filter_eq_empty_iff]
    simp only [Finset.mem_univ, true_implies, Fin.forall_fin_two]
    omega

end LB
