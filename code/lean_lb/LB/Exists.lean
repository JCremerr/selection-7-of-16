/-
  A correct tree exists for every valid rank, so `V_i(P)` (an infimum over
  correct trees) is attained, and the tree bounds become bounds on `V_i(P)`:

    Phi*(P) ≤ 2^{V_i(P)},   ⌈log₂ Phi*(P)⌉ ≤ V_i(P),   KEY(P) ≤ V_i(P).

  The existence proof uses the naive tree that compares every ordered pair of
  labels once and then returns a label of rank `i` for the observed outcomes.
  (This only shows that `V` is well defined; its size is irrelevant.)
-/
import LB.Key
import Mathlib.Data.Nat.Log

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### Every input has a label of rank `i` -/

/-- For an injective input and `1 ≤ i ≤ n` some label has exactly `i - 1`
smaller values. -/
lemma exists_rank {f : α → ℕ} (hf : Function.Injective f) {i : ℕ} (hi : 1 ≤ i)
    (hin : i ≤ Fintype.card α) : ∃ e, (univ.filter fun u => f u < f e).card = i - 1 := by
  let Pf : SPO α := ⟨fun u v => f u < f v, fun _ => lt_irrefl _, fun h₁ h₂ => lt_trans h₁ h₂⟩
  obtain ⟨L, hL, hc⟩ := exists_downSet Pf (j := i - 1) (by omega)
  obtain ⟨e, he⟩ := exists_mem_minimals Pf
    (compl_nonempty_of_card_lt (L := L) (by rw [hc]; omega))
  rw [mem_minimals, Finset.mem_compl] at he
  refine ⟨e, ?_⟩
  have : univ.filter (fun u => f u < f e) = L := by
    ext u
    simp only [Finset.mem_filter, Finset.mem_univ, true_and]
    constructor
    · intro hu
      by_contra huL
      exact he.2 u (Finset.mem_compl.2 huL) hu
    · intro hu
      rcases lt_trichotomy (f u) (f e) with h | h | h
      · exact h
      · exact absurd (hf h ▸ hu) he.1
      · exact absurd (hL e u h hu) he.1
  rw [this, hc]

/-! ### The naive tree -/

namespace CTree

/-- Compare the pairs of `ps` in order; after the outcomes `bs` (true = "less")
the leaf returns `k bs`. -/
def allPairs {α : Type*} : List (α × α) → (List Bool → α) → CTree α
  | [], k => .leaf (k [])
  | p :: ps, k =>
    .node p.1 p.2 (allPairs ps fun bs => k (true :: bs)) (allPairs ps fun bs => k (false :: bs))

lemma eval_allPairs {α : Type*} (ps : List (α × α)) (k : List Bool → α) (f : α → ℕ) :
    (allPairs ps k).eval f = k (ps.map fun p => decide (f p.1 < f p.2)) := by
  induction ps generalizing k with
  | nil => rfl
  | cons p ps ih =>
    by_cases h : f p.1 < f p.2
    · simp [allPairs, eval, h, ih]
    · simp [allPairs, eval, h, ih]

end CTree

open Classical in
/-- The naive correct tree for rank `i` (`d` is a default label, never returned
on a valid input). -/
noncomputable def naiveTree (i : ℕ) (d : α) : CTree α :=
  CTree.allPairs (univ : Finset (α × α)).toList fun bs =>
    if h : ∃ (e : α) (g : α → ℕ), Function.Injective g ∧
        (univ : Finset (α × α)).toList.map (fun p => decide (g p.1 < g p.2)) = bs ∧
        (univ.filter fun u => g u < g e).card = i - 1
    then h.choose else d

theorem naiveTree_correct (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) (d : α) :
    Correct P i (naiveTree i d) := by
  classical
  intro f hf
  unfold naiveTree
  rw [CTree.eval_allPairs]
  have hex : ∃ (e : α) (g : α → ℕ), Function.Injective g ∧
      (univ : Finset (α × α)).toList.map (fun p => decide (g p.1 < g p.2)) =
        (univ : Finset (α × α)).toList.map (fun p => decide (f p.1 < f p.2)) ∧
      (univ.filter fun u => g u < g e).card = i - 1 := by
    obtain ⟨e, he⟩ := exists_rank hf.1 hi hin
    exact ⟨e, f, hf.1, rfl, he⟩
  rw [dite_eq_left hex]
  obtain ⟨g, -, hmap, hcount⟩ := hex.choose_spec
  have hfg : (univ.filter fun u => f u < f hex.choose) =
      (univ.filter fun u => g u < g hex.choose) := by
    ext u
    simp only [Finset.mem_filter, Finset.mem_univ, true_and]
    have := List.map_inj_left.1 hmap (u, hex.choose) (by simp)
    simpa using this.symm
  rw [hfg]
  exact hcount

/-- **A correct tree exists for every valid rank.** -/
theorem exists_correct (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    ∃ T, Correct P i T := by
  have : Nonempty α := Fintype.card_pos_iff.1 (by omega)
  exact ⟨naiveTree i (Classical.arbitrary α), naiveTree_correct P hi hin _⟩

/-! ### `V_i(P)` is attained -/

theorem V_le {P : SPO α} {i : ℕ} {T : CTree α} (hT : Correct P i T) {d : ℕ}
    (hd : ∀ f, IsLinExt P f → T.pathLen f ≤ d) : V P i ≤ d :=
  Nat.sInf_le ⟨T, hT, hd⟩

/-- Some correct tree makes at most `V_i(P)` comparisons on every consistent input. -/
theorem V_spec (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    ∃ T, Correct P i T ∧ ∀ f, IsLinExt P f → T.pathLen f ≤ V P i := by
  obtain ⟨T, hT⟩ := exists_correct P hi hin
  exact Nat.sInf_mem (s := {d | ∃ T : CTree α, Correct P i T ∧
    ∀ f, IsLinExt P f → T.pathLen f ≤ d}) ⟨T.depth, T, hT, fun f _ => T.pathLen_le_depth f⟩

/-! ### The bounds on `V_i(P)` -/

/-- `Phi*(P) ≤ 2^{V_i(P)}`. -/
theorem Phi_le_two_pow_V (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Phi P i ≤ 2 ^ V P i := by
  obtain ⟨T, hT, hd⟩ := V_spec P hi hin
  exact Phi_le_two_pow_of_cost P hi hin T hT _ hd

/-- `⌈log₂ Phi*(P)⌉ ≤ V_i(P)` (`Phi*` is a positive integer, `LB.one_le_Phi`). -/
theorem clog_Phi_le_V (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Nat.clog 2 (Phi P i).toNat ≤ V P i := by
  rw [Nat.clog_le_iff_le_pow (by norm_num)]
  have h1 := Phi_le_two_pow_V P hi hin
  have h2 := one_le_Phi P hi hin
  have h3 : ((2 ^ V P i : ℕ) : ℤ) = 2 ^ V P i := by push_cast; rfl
  omega

/-- `KEY(P) ≤ V_i(P)`. -/
theorem KEY_le_V (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    KEY P i ≤ V P i := by
  obtain ⟨T, hT, hd⟩ := V_spec P hi hin
  exact KEY_le_of_cost hT _ hd

end LB
