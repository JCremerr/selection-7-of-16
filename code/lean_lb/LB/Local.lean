/-
  Obligations 4 and 5: the local inequality  Phi(P) ≤ Phi(P + xy) + Phi(P + yx)
  for incomparable `x ≠ y` (proof of Theorem `thm:phi`, Steps 2 and 3).

  Obligation 4 is proved EXACTLY, as an identity of natural numbers:
    |C(P1)| + |C(P2)| = |C(P)| + ∑_{L ∈ Down_{i-1}, x,y ∈ L} (b(L)+1)
                               + ∑_{A ∈ Down_i,   x,y ∉ A} (a(A)+1),
  from "every solution survives in at least one child, the children's
  solutions are solutions of P, and a solution survives in both exactly when
  x and y lie on the same side of its answer".
-/
import LB.World
import Mathlib.Algebra.BigOperators.Ring.Finset
import Mathlib.Tactic.Ring

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

section Local

variable {P : SPO α} {x y : α} {i : ℕ}

/-- The children's solutions are solutions of `P` (adding a relation creates no
new compatible solution). -/
lemma C_add_subset : C (P.add x y) i ⊆ C P i := by
  intro s hs
  rw [mem_C_def] at hs ⊢
  obtain ⟨hc, f, hf, h1, h2⟩ := hs
  exact ⟨hc, f, hf.of_add, h1, h2⟩

/-- Every compatible solution survives in at least one child. -/
lemma C_subset_union (hne : x ≠ y) : C P i ⊆ C (P.add x y) i ∪ C (P.add y x) i := by
  intro s hs
  rw [mem_C_def] at hs
  obtain ⟨hc, f, hf, h1, h2⟩ := hs
  rw [Finset.mem_union, mem_C_def, mem_C_def]
  rcases lt_or_gt_of_ne (hf.1.ne hne) with h | h
  · exact Or.inl ⟨hc, f, isLinExt_add hf h, h1, h2⟩
  · exact Or.inr ⟨hc, f, isLinExt_add hf h, h1, h2⟩

lemma C_union_eq (hne : x ≠ y) : C (P.add x y) i ∪ C (P.add y x) i = C P i :=
  Finset.Subset.antisymm (Finset.union_subset C_add_subset C_add_subset) (C_subset_union hne)

/-- A solution `(L, e)` survives both outcomes iff `x` and `y` lie on the same
side of its answer: both in `L`, or both in `U` (i.e. both outside `L ∪ {e}`). -/
theorem mem_C_add_both (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x)
    {s : Finset α × α} :
    (s ∈ C (P.add x y) i ∧ s ∈ C (P.add y x) i) ↔
      s ∈ C P i ∧ ((x ∈ s.1 ∧ y ∈ s.1) ∨ (x ∉ insert s.2 s.1 ∧ y ∉ insert s.2 s.1)) := by
  have h₁ : ¬ P.le y x := not_le_of_incomp hne hyx
  have h₂ : ¬ P.le x y := not_le_of_incomp hne.symm hxy
  obtain ⟨L, e⟩ := s
  constructor
  · rintro ⟨hs1, hs2⟩
    refine ⟨C_add_subset hs1, ?_⟩
    rw [mem_C, mem_Down] at hs1 hs2
    obtain ⟨⟨hL1, -⟩, he1⟩ := hs1
    obtain ⟨⟨hL2, -⟩, he2⟩ := hs2
    have hLP : IsDownSet P L := hL1.of_add
    have i1 : y ∈ L → x ∈ L := (isDownSet_add_iff h₁ hLP).1 hL1
    have i2 : x ∈ L → y ∈ L := (isDownSet_add_iff h₂ hLP).1 hL2
    by_cases hx : x ∈ L
    · exact Or.inl ⟨hx, i2 hx⟩
    · have hy : y ∉ L := fun hy => hx (i1 hy)
      right
      simp only [Finset.mem_insert, not_or]
      refine ⟨⟨?_, hx⟩, ⟨?_, hy⟩⟩
      · rintro rfl
        exact (mem_minimals.1 he2).2 y (Finset.mem_compl.2 hy) (P.add_lt_self h₂)
      · rintro rfl
        exact (mem_minimals.1 he1).2 x (Finset.mem_compl.2 hx) (P.add_lt_self h₁)
  · rintro ⟨hsP, hside⟩
    rw [mem_C, mem_Down] at hsP
    obtain ⟨⟨hLP, hc⟩, he⟩ := hsP
    rw [mem_C, mem_C, mem_Down, mem_Down]
    rcases hside with ⟨hx, hy⟩ | ⟨hx, hy⟩
    · refine ⟨⟨⟨(isDownSet_add_iff h₁ hLP).2 fun _ => hx, hc⟩, ?_⟩,
        ⟨⟨(isDownSet_add_iff h₂ hLP).2 fun _ => hy, hc⟩, ?_⟩⟩
      · rw [minimals_add_of_mem h₁ hLP hx]; exact he
      · rw [minimals_add_of_mem h₂ hLP hy]; exact he
    · simp only [Finset.mem_insert, not_or] at hx hy
      refine ⟨⟨⟨(isDownSet_add_iff h₁ hLP).2 fun h => absurd h hy.2, hc⟩, ?_⟩,
        ⟨⟨(isDownSet_add_iff h₂ hLP).2 fun h => absurd h hx.2, hc⟩, ?_⟩⟩
      · exact minimals_erase_subset_add h₁ hy.2
          (Finset.mem_erase.2 ⟨fun h => hy.1 h.symm, he⟩)
      · exact minimals_erase_subset_add h₂ hx.2
          (Finset.mem_erase.2 ⟨fun h => hx.1 h.symm, he⟩)

/-- **Obligation 4 (exact accounting of duplicated solutions).** -/
theorem card_C_add (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) (hne : x ≠ y)
    (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    (C (P.add x y) i).card + (C (P.add y x) i).card =
      (C P i).card + ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), (bOf P L + 1)
        + ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), (aOf P A + 1) := by
  rw [← Finset.card_union_add_card_inter, C_union_eq hne]
  have hinter : C (P.add x y) i ∩ C (P.add y x) i =
      (C P i).filter (fun s => x ∈ s.1 ∧ y ∈ s.1) ∪
        (C P i).filter (fun s => x ∉ insert s.2 s.1 ∧ y ∉ insert s.2 s.1) := by
    ext s
    rw [Finset.mem_inter, mem_C_add_both hne hxy hyx, Finset.mem_union, Finset.mem_filter,
      Finset.mem_filter]
    tauto
  rw [hinter, Finset.card_union_of_disjoint]
  · rw [card_C_filter_L P i (fun L => x ∈ L ∧ y ∈ L),
      card_C_filter_A P hi (fun A => x ∉ A ∧ y ∉ A), add_assoc]
    congr 2
    · exact Finset.sum_congr rfl fun L hL => (bOf_add_one (Finset.mem_filter.1 hL).1 hi hin).symm
    · exact Finset.sum_congr rfl fun A hA => (aOf_add_one (Finset.mem_filter.1 hA).1 hi).symm
  · rw [Finset.disjoint_filter]
    intro s _ h1 h2
    exact h2.1 (Finset.mem_insert_of_mem h1.1)

/-! ### The weights as sums of per-set weights over the parent's sets -/

lemma FG_eq_sum_psiL (P : SPO α) (i : ℕ) : FG P i = ∑ L ∈ Down P (i - 1), psiL P L :=
  Finset.sum_congr rfl fun _ hL => (psiL_of_isDownSet (mem_Down.1 hL).1).symm

lemma FGdual_eq_sum_psiA (P : SPO α) (i : ℕ) : FGdual P i = ∑ A ∈ Down P i, psiA P A :=
  Finset.sum_congr rfl fun _ hA => (psiA_of_isDownSet (mem_Down.1 hA).1).symm

/-- "Down-sets of the children are down-sets of `P`." -/
lemma Down_add (P : SPO α) (x y : α) (j : ℕ) [DecidablePred (IsDownSet (P.add x y))] :
    Down (P.add x y) j = (Down P j).filter (IsDownSet (P.add x y)) := by
  ext L
  simp only [mem_Down, Finset.mem_filter]
  constructor
  · rintro ⟨h1, h2⟩; exact ⟨⟨h1.of_add, h2⟩, h1⟩
  · rintro ⟨⟨_, h2⟩, h1⟩; exact ⟨h1, h2⟩

lemma FG_add_eq_sum_psiL (P : SPO α) (x y : α) (i : ℕ) :
    FG (P.add x y) i = ∑ L ∈ Down P (i - 1), psiL (P.add x y) L := by
  classical
  unfold FG
  rw [Down_add, Finset.sum_filter]
  refine Finset.sum_congr rfl fun L _ => ?_
  by_cases hL : IsDownSet (P.add x y) L
  · rw [ite_eq_left hL, psiL_of_isDownSet hL]
  · rw [ite_eq_right hL, psiL_of_not_isDownSet hL]

lemma FGdual_add_eq_sum_psiA (P : SPO α) (x y : α) (i : ℕ) :
    FGdual (P.add x y) i = ∑ A ∈ Down P i, psiA (P.add x y) A := by
  classical
  unfold FGdual
  rw [Down_add, Finset.sum_filter]
  refine Finset.sum_congr rfl fun A _ => ?_
  by_cases hA : IsDownSet (P.add x y) A
  · rw [ite_eq_left hA, psiA_of_isDownSet hA]
  · rw [ite_eq_right hA, psiA_of_not_isDownSet hA]

/-- Lower sets: every `L` contributes at least `0` to `ΔFG`, and `L` with
`x, y ∈ L` contributes exactly `2^{b(L)}`. -/
theorem FG_add_ge (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    FG P i + ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), 2 ^ bOf P L ≤
      FG (P.add x y) i + FG (P.add y x) i := by
  classical
  rw [FG_eq_sum_psiL, FG_add_eq_sum_psiL, FG_add_eq_sum_psiL, Finset.sum_filter,
    ← Finset.sum_add_distrib, ← Finset.sum_add_distrib]
  refine Finset.sum_le_sum fun L hL => ?_
  have hLP := (mem_Down.1 hL).1
  obtain ⟨w1, w2, w3, w4⟩ := world_L hne hxy hyx hLP
  have e0 := psiL_of_isDownSet hLP
  by_cases hx : x ∈ L <;> by_cases hy : y ∈ L
  · rw [ite_eq_left ⟨hx, hy⟩]
    obtain ⟨e1, e2⟩ := w1 hx hy
    omega
  · rw [ite_eq_right (fun h => hy h.2)]
    obtain ⟨e1, e2⟩ := w2 hx hy
    omega
  · rw [ite_eq_right (fun h => hx h.1)]
    obtain ⟨e1, e2⟩ := w3 hx hy
    omega
  · rw [ite_eq_right (fun h => hx h.1)]
    have := w4 hx hy
    omega

/-- Initial sets: every `A` contributes at least `0` to `ΔFGdual`, and `A`
with `x, y ∉ A` contributes exactly `2^{a(A)}`. -/
theorem FGdual_add_ge (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    FGdual P i + ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), 2 ^ aOf P A ≤
      FGdual (P.add x y) i + FGdual (P.add y x) i := by
  classical
  rw [FGdual_eq_sum_psiA, FGdual_add_eq_sum_psiA, FGdual_add_eq_sum_psiA, Finset.sum_filter,
    ← Finset.sum_add_distrib, ← Finset.sum_add_distrib]
  refine Finset.sum_le_sum fun A hA => ?_
  have hAP := (mem_Down.1 hA).1
  obtain ⟨w1, w2, w3, w4⟩ := world_A hne hxy hyx hAP
  have e0 := psiA_of_isDownSet hAP
  by_cases hx : x ∈ A <;> by_cases hy : y ∈ A
  · rw [ite_eq_right (fun h => h.1 hx)]
    have := w4 hx hy
    omega
  · rw [ite_eq_right (fun h => h.1 hx)]
    obtain ⟨e1, e2⟩ := w2 hx hy
    omega
  · rw [ite_eq_right (fun h => h.2 hy)]
    obtain ⟨e1, e2⟩ := w3 hx hy
    omega
  · rw [ite_eq_left ⟨hx, hy⟩]
    obtain ⟨e1, e2⟩ := w1 hx hy
    omega

/-- **Obligation 5: the local inequality** `Phi(P) ≤ Phi(P + xy) + Phi(P + yx)`
for incomparable `x ≠ y` and a valid rank. -/
theorem local_ineq (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) (hne : x ≠ y)
    (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    Phi P i ≤ Phi (P.add x y) i + Phi (P.add y x) i := by
  have hC := card_C_add hi hin hne hxy hyx
  have hF := FG_add_ge (i := i) hne hxy hyx
  have hD := FGdual_add_ge (i := i) hne hxy hyx
  have hL : ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), (bOf P L + 1) ≤
      ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), 2 ^ bOf P L :=
    Finset.sum_le_sum fun _ _ => succ_le_two_pow _
  have hA : ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), (aOf P A + 1) ≤
      ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), 2 ^ aOf P A :=
    Finset.sum_le_sum fun _ _ => succ_le_two_pow _
  unfold Phi
  omega

/-- The paper's display in Step 3 of the proof of `thm:phi`:
`ΔPhi* ≥ ∑_{x,y ∈ L} (2^{b(L)} - b(L) - 1) + ∑_{x,y ∉ A} (2^{a(A)} - a(A) - 1)`,
computed in `ℤ`. -/
theorem delta_Phi_ge (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) (hne : x ≠ y)
    (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L),
        ((2 : ℤ) ^ bOf P L - bOf P L - 1) +
      ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), ((2 : ℤ) ^ aOf P A - aOf P A - 1) ≤
    Phi (P.add x y) i + Phi (P.add y x) i - Phi P i := by
  have hC := card_C_add hi hin hne hxy hyx
  have hF := FG_add_ge (i := i) hne hxy hyx
  have hD := FGdual_add_ge (i := i) hne hxy hyx
  have e1 : ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L),
      ((2 : ℤ) ^ bOf P L - bOf P L - 1) =
      ((∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), 2 ^ bOf P L : ℕ) : ℤ) -
      ((∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), (bOf P L + 1) : ℕ) : ℤ) := by
    push_cast; rw [← Finset.sum_sub_distrib]; congr 1; ext L; ring
  have e2 : ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A),
      ((2 : ℤ) ^ aOf P A - aOf P A - 1) =
      ((∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), 2 ^ aOf P A : ℕ) : ℤ) -
      ((∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), (aOf P A + 1) : ℕ) : ℤ) := by
    push_cast; rw [← Finset.sum_sub_distrib]; congr 1; ext A; ring
  rw [e1, e2]
  unfold Phi
  omega

end Local

end LB
