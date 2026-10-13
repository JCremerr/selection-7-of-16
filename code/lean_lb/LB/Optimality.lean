/-
  Corollary `cor:combined` and Proposition `prop:phiopt`.

  cor:combined   V_i(P) ≥ LB(P) = max{⌈log₂ Phi*⌉, KEY}, and LB is never
                 weaker than ⌈log₂|C|⌉, ⌈log₂ FG⌉, ⌈log₂ FGdual⌉.
  prop:phiopt    if Psi_{F,G}(E_n, 1) ≤ 2^{V_1(E_n)} and
                 Psi_{F,G}(E_n, n) ≤ 2^{V_n(E_n)} for all n ≥ 1, with
                 F(0) = G(0) = 0, then F(t), G(t) ≤ 2^t - t - 1 for all t,
                 hence Psi_{F,G} ≤ Phi* on every problem.
  For the hypotheses to bite we need V_1(E_n) ≤ n - 1 and V_n(E_n) ≤ n - 1:
  linear scans for the minimum and the maximum (built and verified here).
-/
import LB.Exists
import Mathlib.Algebra.Order.Field.Basic
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Algebra.BigOperators.Ring.Finset

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### Corollary `cor:combined` -/

/-- **Corollary `cor:combined`, first part.**  `V_i(P) ≥ max{⌈log₂ Phi*(P)⌉, KEY(P)}`. -/
theorem LB_le_V (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    max (Nat.clog 2 (Phi P i).toNat) (KEY P i) ≤ V P i :=
  max_le (clog_Phi_le_V P hi hin) (KEY_le_V P hi hin)

/-- **Corollary `cor:combined`, second part.**  The combined bound is never
weaker than the compatible-solution bound `⌈log₂|C|⌉` or either one-sided
weight bound `⌈log₂ FG⌉`, `⌈log₂ FGdual⌉`. -/
theorem clog_bounds_le_clog_Phi (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Nat.clog 2 (C P i).card ≤ Nat.clog 2 (Phi P i).toNat ∧
    Nat.clog 2 (FG P i) ≤ Nat.clog 2 (Phi P i).toNat ∧
    Nat.clog 2 (FGdual P i) ≤ Nat.clog 2 (Phi P i).toNat := by
  have h1 := card_C_le_FG P hi hin
  have h2 := FG_le_Phi P hi
  have h3 := FGdual_le_Phi P hi hin
  refine ⟨Nat.clog_mono_right _ ?_, Nat.clog_mono_right _ ?_, Nat.clog_mono_right _ ?_⟩ <;> omega

/-! ### The empty order -/

@[simp] lemma minimals_antichain (S : Finset α) : minimals (antichain α) S = S := by
  ext v; simp [antichain]

@[simp] lemma maximals_antichain (S : Finset α) : maximals (antichain α) S = S := by
  ext v; simp [antichain]

lemma isDownSet_antichain (S : Finset α) : IsDownSet (antichain α) S := fun _ _ h _ => h.elim

lemma Down_antichain_zero : Down (antichain α) 0 = {∅} := by
  ext S; simp [isDownSet_antichain]

lemma Down_antichain_card : Down (antichain α) (Fintype.card α) = {univ} := by
  ext S
  simp only [mem_Down, Finset.mem_singleton]
  constructor
  · rintro ⟨-, h⟩; exact Finset.eq_univ_of_card S h
  · rintro rfl; exact ⟨isDownSet_antichain _, Finset.card_univ⟩

/-- `Psi_{F,G}(E_n, 1) = n + F(n - 1)` (finding the minimum). -/
lemma Psi_antichain_min (F G : ℕ → ℝ) (hG0 : G 0 = 0) (hn : 1 ≤ Fintype.card α) :
    Psi F G (antichain α) 1 = Fintype.card α + F (Fintype.card α - 1) := by
  have hC : (C (antichain α) 1).card = Fintype.card α := by
    rw [card_C_eq_sum_b _ le_rfl hn]
    simp [Down_antichain_zero, bOf]
    omega
  unfold Psi
  rw [hC, show (1 : ℕ) - 1 = 0 from rfl, Down_antichain_zero, Finset.sum_singleton]
  have hb : bOf (antichain α) ∅ = Fintype.card α - 1 := by simp [bOf]
  have hA : ∑ A ∈ Down (antichain α) 1, G (aOf (antichain α) A) = 0 := by
    refine Finset.sum_eq_zero fun A hA => ?_
    have : aOf (antichain α) A = 0 := by
      simp only [aOf, maximals_antichain, (mem_Down.1 hA).2]
    rw [this, hG0]
  rw [hb, hA, add_zero]

/-- `Psi_{F,G}(E_n, n) = n + G(n - 1)` (finding the maximum). -/
lemma Psi_antichain_max (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hn : 1 ≤ Fintype.card α) :
    Psi F G (antichain α) (Fintype.card α) = Fintype.card α + G (Fintype.card α - 1) := by
  have hC : (C (antichain α) (Fintype.card α)).card = Fintype.card α := by
    rw [card_C_eq_sum_a _ hn, Down_antichain_card, Finset.sum_singleton]
    simp [aOf]
    omega
  unfold Psi
  rw [hC, Down_antichain_card, Finset.sum_singleton]
  have ha : aOf (antichain α) univ = Fintype.card α - 1 := by simp [aOf]
  have hL : ∑ L ∈ Down (antichain α) (Fintype.card α - 1), F (bOf (antichain α) L) = 0 := by
    refine Finset.sum_eq_zero fun L hL => ?_
    have : bOf (antichain α) L = 0 := by
      simp only [bOf, minimals_antichain, Finset.card_compl, (mem_Down.1 hL).2]
      omega
    rw [this, hF0]
  rw [ha, hL, add_zero]

/-! ### Linear scans: `V_1(E_n) ≤ n - 1` and `V_n(E_n) ≤ n - 1` -/

namespace CTree

/-- Linear scan for the minimum: current candidate `m`, remaining labels `l`. -/
def scanMin {α : Type*} : α → List α → CTree α
  | m, [] => .leaf m
  | m, j :: js => .node j m (scanMin j js) (scanMin m js)

/-- Linear scan for the maximum. -/
def scanMax {α : Type*} : α → List α → CTree α
  | m, [] => .leaf m
  | m, j :: js => .node m j (scanMax j js) (scanMax m js)

lemma scanMin_spec {α : Type*} (f : α → ℕ) (l : List α) : ∀ m : α,
    (scanMin m l).eval f ∈ m :: l ∧ (∀ v ∈ m :: l, f ((scanMin m l).eval f) ≤ f v) ∧
      (scanMin m l).pathLen f = l.length := by
  induction l with
  | nil => intro m; simp [scanMin, eval, pathLen]
  | cons j js ih =>
    intro m
    by_cases h : f j < f m
    · obtain ⟨h1, h2, h3⟩ := ih j
      simp only [scanMin, eval, pathLen, ite_eq_left h, List.mem_cons, List.length_cons] at *
      refine ⟨Or.inr h1, ?_, by omega⟩
      rintro v (rfl | rfl | hv)
      · have := h2 j (Or.inl rfl); omega
      · exact h2 v (Or.inl rfl)
      · exact h2 v (Or.inr hv)
    · obtain ⟨h1, h2, h3⟩ := ih m
      simp only [scanMin, eval, pathLen, ite_eq_right h, List.mem_cons, List.length_cons] at *
      refine ⟨by tauto, ?_, by omega⟩
      rintro v (rfl | rfl | hv)
      · exact h2 v (Or.inl rfl)
      · have := h2 m (Or.inl rfl); omega
      · exact h2 v (Or.inr hv)

lemma scanMax_spec {α : Type*} (f : α → ℕ) (l : List α) : ∀ m : α,
    (scanMax m l).eval f ∈ m :: l ∧ (∀ v ∈ m :: l, f v ≤ f ((scanMax m l).eval f)) ∧
      (scanMax m l).pathLen f = l.length := by
  induction l with
  | nil => intro m; simp [scanMax, eval, pathLen]
  | cons j js ih =>
    intro m
    by_cases h : f m < f j
    · obtain ⟨h1, h2, h3⟩ := ih j
      simp only [scanMax, eval, pathLen, ite_eq_left h, List.mem_cons, List.length_cons] at *
      refine ⟨Or.inr h1, ?_, by omega⟩
      rintro v (rfl | rfl | hv)
      · have := h2 j (Or.inl rfl); omega
      · exact h2 v (Or.inl rfl)
      · exact h2 v (Or.inr hv)
    · obtain ⟨h1, h2, h3⟩ := ih m
      simp only [scanMax, eval, pathLen, ite_eq_right h, List.mem_cons, List.length_cons] at *
      refine ⟨by tauto, ?_, by omega⟩
      rintro v (rfl | rfl | hv)
      · exact h2 v (Or.inl rfl)
      · have := h2 m (Or.inl rfl); omega
      · exact h2 v (Or.inr hv)

end CTree

lemma mem_cons_erase_toList (m u : α) : u ∈ m :: (univ.erase m).toList := by
  by_cases h : u = m
  · exact h ▸ List.mem_cons_self
  · exact List.mem_cons_of_mem _ (Finset.mem_toList.2 (Finset.mem_erase.2 ⟨h, mem_univ u⟩))

lemma length_erase_toList (m : α) : (univ.erase m).toList.length = Fintype.card α - 1 := by
  rw [Finset.length_toList, Finset.card_erase_of_mem (mem_univ m), Finset.card_univ]

/-- The linear scan is a correct tree for the minimum of `n` unordered labels
and makes `n - 1` comparisons, so `V_1(E_n) ≤ n - 1`. -/
theorem V_antichain_min_le (hn : 1 ≤ Fintype.card α) :
    V (antichain α) 1 ≤ Fintype.card α - 1 := by
  have : Nonempty α := Fintype.card_pos_iff.1 hn
  let m : α := Classical.arbitrary α
  refine V_le (T := CTree.scanMin m (univ.erase m).toList) ?_ fun f _ => ?_
  · intro f hf
    obtain ⟨-, h2, -⟩ := CTree.scanMin_spec f (univ.erase m).toList m
    rw [Finset.card_eq_zero, Finset.filter_eq_empty_iff]
    intro u _ hu
    have := h2 u (mem_cons_erase_toList m u)
    omega
  · rw [(CTree.scanMin_spec f (univ.erase m).toList m).2.2, length_erase_toList]

/-- The linear scan for the maximum: `V_n(E_n) ≤ n - 1`. -/
theorem V_antichain_max_le (hn : 1 ≤ Fintype.card α) :
    V (antichain α) (Fintype.card α) ≤ Fintype.card α - 1 := by
  have : Nonempty α := Fintype.card_pos_iff.1 hn
  let m : α := Classical.arbitrary α
  refine V_le (T := CTree.scanMax m (univ.erase m).toList) ?_ fun f _ => ?_
  · intro f hf
    obtain ⟨-, h2, -⟩ := CTree.scanMax_spec f (univ.erase m).toList m
    set r := (CTree.scanMax m (univ.erase m).toList).eval f
    have : univ.filter (fun u => f u < f r) = univ.erase r := by
      ext u
      simp only [Finset.mem_filter, Finset.mem_univ, true_and, Finset.mem_erase, and_true]
      constructor
      · rintro h rfl; exact lt_irrefl _ h
      · intro hu
        have := h2 u (mem_cons_erase_toList m u)
        have := hf.1.ne hu
        omega
    rw [this, Finset.card_erase_of_mem (mem_univ r), Finset.card_univ]
  · rw [(CTree.scanMax_spec f (univ.erase m).toList m).2.2, length_erase_toList]

/-! ### Proposition `prop:phiopt` -/

/-- `Phi*` is the member `F = G = 2^t - t - 1` of the family. -/
theorem Psi_two_pow_eq_Phi (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Psi (fun t => 2 ^ t - t - 1) (fun t => 2 ^ t - t - 1) P i = (Phi P i : ℝ) := by
  have e1 : ((C P i).card : ℝ) = ∑ L ∈ Down P (i - 1), ((bOf P L : ℝ) + 1) := by
    rw [card_C_eq_sum_b P hi hin]; push_cast; rfl
  have e2 : ((C P i).card : ℝ) = ∑ A ∈ Down P i, ((aOf P A : ℝ) + 1) := by
    rw [card_C_eq_sum_a P hi]; push_cast; rfl
  have e3 : (FG P i : ℝ) = ∑ L ∈ Down P (i - 1), (2 : ℝ) ^ bOf P L := by
    unfold FG; push_cast; rfl
  have e4 : (FGdual P i : ℝ) = ∑ A ∈ Down P i, (2 : ℝ) ^ aOf P A := by
    unfold FGdual; push_cast; rfl
  have s1 : ∑ L ∈ Down P (i - 1), ((2 : ℝ) ^ bOf P L - bOf P L - 1) =
      ∑ L ∈ Down P (i - 1), (2 : ℝ) ^ bOf P L - ∑ L ∈ Down P (i - 1), ((bOf P L : ℝ) + 1) := by
    rw [← Finset.sum_sub_distrib]; congr 1; ext L; ring
  have s2 : ∑ A ∈ Down P i, ((2 : ℝ) ^ aOf P A - aOf P A - 1) =
      ∑ A ∈ Down P i, (2 : ℝ) ^ aOf P A - ∑ A ∈ Down P i, ((aOf P A : ℝ) + 1) := by
    rw [← Finset.sum_sub_distrib]; congr 1; ext A; ring
  unfold Psi Phi
  push_cast
  rw [s1, s2]
  linarith

/-- Termwise comparison: if `F, G ≤ 2^t - t - 1` then `Psi_{F,G} ≤ Phi*`. -/
theorem Psi_le_Phi (F G : ℕ → ℝ) (hF : ∀ t, F t ≤ 2 ^ t - t - 1) (hG : ∀ t, G t ≤ 2 ^ t - t - 1)
    (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Psi F G P i ≤ (Phi P i : ℝ) := by
  rw [← Psi_two_pow_eq_Phi P hi hin]
  unfold Psi
  have h1 := Finset.sum_le_sum fun L (_ : L ∈ Down P (i - 1)) => hF (bOf P L)
  have h2 := Finset.sum_le_sum fun A (_ : A ∈ Down P i) => hG (aOf P A)
  linarith

/-- **Proposition `prop:phiopt`.**  Let `F, G` be real-valued functions on the
nonnegative integers with `F(0) = G(0) = 0` such that the formula `Psi_{F,G}` is
valid for finding the minimum and the maximum of every antichain `E_n`,
`n ≥ 1`.  Then `F(t), G(t) ≤ 2^t - t - 1` for all `t`, and `Psi_{F,G} ≤ Phi*`
on every selection problem (any finite strict partial order, any valid rank). -/
theorem phiopt (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0)
    (hmin : ∀ n : ℕ, 1 ≤ n →
      Psi F G (antichain (Fin n)) 1 ≤ 2 ^ V (antichain (Fin n)) 1)
    (hmax : ∀ n : ℕ, 1 ≤ n →
      Psi F G (antichain (Fin n)) n ≤ 2 ^ V (antichain (Fin n)) n) :
    (∀ t, F t ≤ 2 ^ t - t - 1) ∧ (∀ t, G t ≤ 2 ^ t - t - 1) ∧
    (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
      1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ)) := by
  have hF : ∀ t, F t ≤ 2 ^ t - t - 1 := by
    intro t
    have hn : 1 ≤ Fintype.card (Fin (t + 1)) := by simp
    have h1 := hmin (t + 1) (by omega)
    rw [Psi_antichain_min F G hG0 hn] at h1
    have hV := V_antichain_min_le hn
    simp only [Fintype.card_fin, Nat.add_sub_cancel] at h1 hV
    have h2 : (2 : ℝ) ^ V (antichain (Fin (t + 1))) 1 ≤ 2 ^ t :=
      pow_le_pow_right₀ (by norm_num) hV
    push_cast at h1
    linarith
  have hG : ∀ t, G t ≤ 2 ^ t - t - 1 := by
    intro t
    have hn : 1 ≤ Fintype.card (Fin (t + 1)) := by simp
    have h1 := hmax (t + 1) (by omega)
    have h1' : Psi F G (antichain (Fin (t + 1))) (Fintype.card (Fin (t + 1))) ≤
        2 ^ V (antichain (Fin (t + 1))) (Fintype.card (Fin (t + 1))) := by
      simpa only [Fintype.card_fin] using h1
    rw [Psi_antichain_max F G hF0 hn] at h1'
    have hV := V_antichain_max_le hn
    simp only [Fintype.card_fin, Nat.add_sub_cancel] at h1' hV
    have h2 : (2 : ℝ) ^ V (antichain (Fin (t + 1))) (t + 1) ≤ 2 ^ t :=
      pow_le_pow_right₀ (by norm_num) hV
    push_cast at h1'
    linarith
  exact ⟨hF, hG, fun β _ _ P i hi hin => Psi_le_Phi F G hF hG P hi hin⟩

end LB
