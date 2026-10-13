/-
  Obligation 2: counting compatible solutions by lower sets and by initial
  sets,  |C(P)| = ∑_L (b(L)+1) = ∑_A (a(A)+1),  in a filtered form (any
  condition on L, resp. on A) that the duplicate accounting needs later.
  Also: the "minus one" in b and a never truncates for valid ranks,
  |C| ≤ FG, |C| ≤ FGdual, |C| ≥ 1 and Phi ≥ 1.
-/
import LB.Basic
import Mathlib.Algebra.Order.BigOperators.Group.Finset

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### The subtraction in `b` and `a` is exact -/

lemma compl_nonempty_of_card_lt {L : Finset α} (h : L.card < Fintype.card α) : Lᶜ.Nonempty := by
  rw [← Finset.card_pos, Finset.card_compl]; omega

/-- `b(L) + 1` is the number of minimal elements of the complement whenever
the complement is nonempty. -/
lemma bOf_add_one_of_card_lt (P : SPO α) {L : Finset α} (h : L.card < Fintype.card α) :
    bOf P L + 1 = (minimals P Lᶜ).card := by
  have := card_minimals_pos P (compl_nonempty_of_card_lt h)
  unfold bOf; omega

/-- For valid ranks, `b(L) + 1` is the number of candidates, for every `L ∈ Down_{i-1}`. -/
lemma bOf_add_one {P : SPO α} {i : ℕ} {L : Finset α} (hL : L ∈ Down P (i - 1)) (hi : 1 ≤ i)
    (hin : i ≤ Fintype.card α) : bOf P L + 1 = (minimals P Lᶜ).card :=
  bOf_add_one_of_card_lt P (by rw [(mem_Down.1 hL).2]; omega)

lemma aOf_add_one_of_nonempty (P : SPO α) {A : Finset α} (h : A.Nonempty) :
    aOf P A + 1 = (maximals P A).card := by
  have := card_maximals_pos P h
  unfold aOf; omega

/-- For `1 ≤ i`, `a(A) + 1` is the number of candidates, for every `A ∈ Down_i`. -/
lemma aOf_add_one {P : SPO α} {i : ℕ} {A : Finset α} (hA : A ∈ Down P i) (hi : 1 ≤ i) :
    aOf P A + 1 = (maximals P A).card :=
  aOf_add_one_of_nonempty P (Finset.card_pos.1 (by rw [(mem_Down.1 hA).2]; omega))

/-! ### Grouping solutions by `L` and by `A = L ∪ {e}` -/

/-- Solutions whose lower set satisfies `p`, counted by lower sets. -/
theorem card_C_filter_L (P : SPO α) (i : ℕ) (p : Finset α → Prop) [DecidablePred p] :
    ((C P i).filter fun s => p s.1).card =
      ∑ L ∈ (Down P (i - 1)).filter p, (minimals P Lᶜ).card := by
  rw [Finset.card_eq_sum_card_fiberwise (f := Prod.fst) (t := (Down P (i - 1)).filter p)]
  · refine Finset.sum_congr rfl fun L hL => ?_
    rw [Finset.mem_filter] at hL
    refine Finset.card_nbij' Prod.snd (fun e => (L, e)) ?_ ?_ ?_ ?_
    · intro s hs
      simp only [Finset.mem_coe, Finset.mem_filter] at hs
      obtain ⟨⟨hsC, -⟩, rfl⟩ := hs
      exact (mem_C.1 hsC).2
    · intro e he
      simp only [Finset.mem_coe, Finset.mem_filter] at he ⊢
      exact ⟨⟨mem_C.2 ⟨hL.1, he⟩, hL.2⟩, trivial⟩
    · intro s hs
      simp only [Finset.mem_coe, Finset.mem_filter] at hs
      obtain ⟨-, rfl⟩ := hs
      rfl
    · intro e _; rfl
  · intro s hs
    simp only [Finset.mem_coe, Finset.mem_filter] at hs
    simp only [Finset.mem_coe, Finset.mem_filter]
    exact ⟨(mem_C.1 hs.1).1, hs.2⟩

/-- Solutions whose initial set `A = L ∪ {e}` satisfies `p`, counted by
initial sets. -/
theorem card_C_filter_A (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (p : Finset α → Prop)
    [DecidablePred p] :
    ((C P i).filter fun s => p (insert s.2 s.1)).card =
      ∑ A ∈ (Down P i).filter p, (maximals P A).card := by
  rw [Finset.card_eq_sum_card_fiberwise (f := fun s => insert s.2 s.1)
    (t := (Down P i).filter p)]
  · refine Finset.sum_congr rfl fun A hA => ?_
    rw [Finset.mem_filter] at hA
    refine Finset.card_nbij' Prod.snd (fun e => (A.erase e, e)) ?_ ?_ ?_ ?_
    · intro s hs
      simp only [Finset.mem_coe, Finset.mem_filter] at hs
      obtain ⟨⟨hsC, -⟩, rfl⟩ := hs
      exact ((mem_C_iff_A hi).1 hsC).2.2
    · intro e he
      simp only [Finset.mem_coe] at he
      have heA : e ∈ A := maximals_subset P A he
      have hins : insert e (A.erase e) = A := Finset.insert_erase heA
      simp only [Finset.mem_coe, Finset.mem_filter]
      refine ⟨⟨(mem_C_iff_A hi).2 ⟨Finset.notMem_erase e A, ?_, ?_⟩, ?_⟩, hins⟩
      · simpa only [hins] using hA.1
      · simpa only [hins] using he
      · simpa only [hins] using hA.2
    · intro s hs
      simp only [Finset.mem_coe, Finset.mem_filter] at hs
      obtain ⟨⟨hsC, -⟩, rfl⟩ := hs
      have he : s.2 ∉ s.1 := ((mem_C_iff_A hi).1 hsC).1
      simp only [Finset.erase_insert he]
    · intro e _; rfl
  · intro s hs
    simp only [Finset.mem_coe, Finset.mem_filter] at hs
    simp only [Finset.mem_coe, Finset.mem_filter]
    exact ⟨((mem_C_iff_A hi).1 hs.1).2.1, hs.2⟩

/-! ### The two identities of the paper -/

/-- `|C(P)| = ∑_{L ∈ Down_{i-1}(P)} (b(L) + 1)`. -/
theorem card_C_eq_sum_b (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (C P i).card = ∑ L ∈ Down P (i - 1), (bOf P L + 1) := by
  have h := card_C_filter_L P i (fun _ => True)
  simp only [Finset.filter_true] at h
  rw [h]
  exact Finset.sum_congr rfl fun L hL => (bOf_add_one hL hi hin).symm

/-- `|C(P)| = ∑_{A ∈ Down_i(P)} (a(A) + 1)`. -/
theorem card_C_eq_sum_a (P : SPO α) {i : ℕ} (hi : 1 ≤ i) :
    (C P i).card = ∑ A ∈ Down P i, (aOf P A + 1) := by
  have h := card_C_filter_A P hi (fun _ => True)
  simp only [Finset.filter_true] at h
  rw [h]
  exact Finset.sum_congr rfl fun A hA => (aOf_add_one hA hi).symm

/-! ### Consequences: `|C| ≤ FG`, `|C| ≤ FGdual`, `1 ≤ |C|`, `1 ≤ Phi` -/

lemma succ_le_two_pow (t : ℕ) : t + 1 ≤ 2 ^ t := Nat.lt_two_pow_self

theorem card_C_le_FG (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (C P i).card ≤ FG P i := by
  rw [card_C_eq_sum_b P hi hin, FG]
  exact Finset.sum_le_sum fun L _ => succ_le_two_pow _

theorem card_C_le_FGdual (P : SPO α) {i : ℕ} (hi : 1 ≤ i) :
    (C P i).card ≤ FGdual P i := by
  rw [card_C_eq_sum_a P hi, FGdual]
  exact Finset.sum_le_sum fun A _ => succ_le_two_pow _

theorem one_le_card_C (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    1 ≤ (C P i).card := by
  obtain ⟨L, hL, hc⟩ := exists_downSet P (j := i - 1) (by omega)
  obtain ⟨e, he⟩ := exists_mem_minimals P
    (compl_nonempty_of_card_lt (L := L) (by rw [hc]; omega))
  exact Finset.card_pos.2 ⟨(L, e), mem_C.2 ⟨mem_Down.2 ⟨hL, hc⟩, he⟩⟩

theorem FG_le_Phi (P : SPO α) {i : ℕ} (hi : 1 ≤ i) :
    (FG P i : ℤ) ≤ Phi P i := by
  have := card_C_le_FGdual P hi
  unfold Phi; omega

theorem FGdual_le_Phi (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (FGdual P i : ℤ) ≤ Phi P i := by
  have := card_C_le_FG P hi hin
  unfold Phi; omega

theorem one_le_Phi (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    1 ≤ Phi P i := by
  have h1 := card_C_le_FG P hi hin
  have h2 := card_C_le_FGdual P hi
  have h3 := one_le_card_C P hi hin
  unfold Phi; omega

end LB
