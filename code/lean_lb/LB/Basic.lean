/-
  Basic facts: membership lemmas, an explicit consistent input, minimal and
  maximal elements exist, down-sets of every size exist, and the down-set
  characterization of compatible solutions (both views of the paper:
  `(L, e) ∈ C(P)` iff `L ∈ Down_{i-1}` and `e` is minimal in the complement,
  iff `A = L ∪ {e} ∈ Down_i` and `e` is maximal in `A`).
-/
import LB.Spec
import Mathlib.Data.Finset.Max
import Mathlib.Data.Fintype.Card
import Mathlib.Logic.Equiv.Fin.Basic

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### The reflexive closure of a strict partial order -/

namespace SPO

/-- `u ≤ v` in `P`: equal or known smaller. -/
def le (P : SPO α) (u v : α) : Prop := u = v ∨ P.lt u v

variable (P : SPO α) {u v w : α}

lemma le_rfl' : P.le u u := Or.inl rfl

lemma le_of_lt (h : P.lt u v) : P.le u v := Or.inr h

lemma le_trans' (h₁ : P.le u v) (h₂ : P.le v w) : P.le u w := by
  rcases h₁ with rfl | h₁
  · exact h₂
  · rcases h₂ with rfl | h₂
    · exact Or.inr h₁
    · exact Or.inr (P.trans h₁ h₂)

lemma lt_of_le_of_lt (h₁ : P.le u v) (h₂ : P.lt v w) : P.lt u w := by
  rcases h₁ with rfl | h₁
  · exact h₂
  · exact P.trans h₁ h₂

lemma lt_of_lt_of_le (h₁ : P.lt u v) (h₂ : P.le v w) : P.lt u w := by
  rcases h₂ with rfl | h₂
  · exact h₁
  · exact P.trans h₁ h₂

lemma asymm (h : P.lt u v) : ¬ P.lt v u := fun h' => P.irrefl _ (P.trans h h')

lemma ne_of_lt (h : P.lt u v) : u ≠ v := by
  rintro rfl; exact P.irrefl _ h

end SPO

/-! ### Membership lemmas -/

@[simp] lemma mem_Down {P : SPO α} {j : ℕ} {S : Finset α} :
    S ∈ Down P j ↔ IsDownSet P S ∧ S.card = j := by
  unfold Down; simp

@[simp] lemma mem_minimals {P : SPO α} {S : Finset α} {v : α} :
    v ∈ minimals P S ↔ v ∈ S ∧ ∀ u ∈ S, ¬ P.lt u v := by
  unfold minimals; simp

@[simp] lemma mem_maximals {P : SPO α} {S : Finset α} {v : α} :
    v ∈ maximals P S ↔ v ∈ S ∧ ∀ u ∈ S, ¬ P.lt v u := by
  unfold maximals; simp

lemma mem_C_def {P : SPO α} {i : ℕ} {s : Finset α × α} :
    s ∈ C P i ↔ s.1.card = i - 1 ∧
      ∃ f, IsLinExt P f ∧ (∀ l ∈ s.1, f l < f s.2) ∧
        (∀ u, u ∉ s.1 → u ≠ s.2 → f s.2 < f u) := by
  unfold C; simp

lemma minimals_subset (P : SPO α) (S : Finset α) : minimals P S ⊆ S := fun _ h =>
  (mem_minimals.1 h).1

lemma maximals_subset (P : SPO α) (S : Finset α) : maximals P S ⊆ S := fun _ h =>
  (mem_maximals.1 h).1

/-! ### An explicit consistent input -/

open scoped Classical in
/-- Number of known predecessors. -/
noncomputable def npred (P : SPO α) (u : α) : ℕ := (univ.filter fun v => P.lt v u).card

lemma npred_lt {P : SPO α} {u v : α} (h : P.lt u v) : npred P u < npred P v := by
  classical
  unfold npred
  apply Finset.card_lt_card
  rw [Finset.ssubset_iff_of_subset]
  · exact ⟨u, by simp [h], by simp [P.irrefl]⟩
  · intro w hw
    simp only [Finset.mem_filter, Finset.mem_univ, true_and] at hw ⊢
    exact P.trans hw h

/-- An explicit input consistent with `P`: `n · npred(u) + index(u)`. -/
noncomputable def linExt (P : SPO α) (u : α) : ℕ :=
  Fintype.card α * npred P u + (Fintype.equivFin α u : ℕ)

lemma linExt_lt_of_npred_lt {P : SPO α} {u v : α} (h : npred P u < npred P v) :
    linExt P u < linExt P v := by
  unfold linExt
  have hu : (Fintype.equivFin α u : ℕ) < Fintype.card α := (Fintype.equivFin α u).isLt
  have h1 : Fintype.card α * (npred P u + 1) ≤ Fintype.card α * npred P v :=
    Nat.mul_le_mul_left _ h
  rw [Nat.mul_succ] at h1
  omega

lemma isLinExt_linExt (P : SPO α) : IsLinExt P (linExt P) := by
  refine ⟨?_, fun u v h => linExt_lt_of_npred_lt (npred_lt h)⟩
  intro u v huv
  rcases lt_trichotomy (npred P u) (npred P v) with h | h | h
  · exact absurd huv (linExt_lt_of_npred_lt h).ne
  · have : (Fintype.equivFin α u : ℕ) = Fintype.equivFin α v := by
      unfold linExt at huv; rw [h] at huv; omega
    exact (Fintype.equivFin α).injective (Fin.ext this)
  · exact absurd huv (linExt_lt_of_npred_lt h).ne'

lemma exists_linExt (P : SPO α) : ∃ f, IsLinExt P f := ⟨_, isLinExt_linExt P⟩

/-! ### Minimal and maximal elements exist -/

lemma exists_mem_minimals (P : SPO α) {S : Finset α} (hS : S.Nonempty) :
    ∃ m, m ∈ minimals P S := by
  obtain ⟨m, hm, hmin⟩ := S.exists_min_image (linExt P) hS
  refine ⟨m, mem_minimals.2 ⟨hm, fun u hu hum => ?_⟩⟩
  have := (isLinExt_linExt P).2 u m hum
  have := hmin u hu
  omega

lemma exists_mem_maximals (P : SPO α) {S : Finset α} (hS : S.Nonempty) :
    ∃ m, m ∈ maximals P S := by
  obtain ⟨m, hm, hmax⟩ := S.exists_max_image (linExt P) hS
  refine ⟨m, mem_maximals.2 ⟨hm, fun u hu hmu => ?_⟩⟩
  have := (isLinExt_linExt P).2 m u hmu
  have := hmax u hu
  omega

lemma card_minimals_pos (P : SPO α) {S : Finset α} (hS : S.Nonempty) :
    0 < (minimals P S).card := by
  obtain ⟨m, hm⟩ := exists_mem_minimals P hS
  exact Finset.card_pos.2 ⟨m, hm⟩

lemma card_maximals_pos (P : SPO α) {S : Finset α} (hS : S.Nonempty) :
    0 < (maximals P S).card := by
  obtain ⟨m, hm⟩ := exists_mem_maximals P hS
  exact Finset.card_pos.2 ⟨m, hm⟩

/-! ### Down-sets of every size exist -/

lemma isDownSet_erase_of_maximal {P : SPO α} {S : Finset α} {m : α} (hS : IsDownSet P S)
    (hm : m ∈ maximals P S) : IsDownSet P (S.erase m) := by
  intro u v huv hv
  rw [Finset.mem_erase] at hv ⊢
  refine ⟨?_, hS u v huv hv.2⟩
  rintro rfl
  exact (mem_maximals.1 hm).2 v hv.2 huv

lemma exists_downSet (P : SPO α) {j : ℕ} (hj : j ≤ Fintype.card α) :
    ∃ L, IsDownSet P L ∧ L.card = j := by
  suffices h : ∀ k, k ≤ Fintype.card α → ∃ L, IsDownSet P L ∧ L.card = Fintype.card α - k by
    obtain ⟨L, hL, hc⟩ := h (Fintype.card α - j) (Nat.sub_le _ _)
    exact ⟨L, hL, by omega⟩
  intro k
  induction k with
  | zero => exact fun _ => ⟨univ, fun _ _ _ _ => mem_univ _, by simp⟩
  | succ k ih =>
    intro hk
    obtain ⟨L, hL, hc⟩ := ih (by omega)
    have hne : L.Nonempty := Finset.card_pos.1 (by omega)
    obtain ⟨m, hm⟩ := exists_mem_maximals P hne
    refine ⟨L.erase m, isDownSet_erase_of_maximal hL hm, ?_⟩
    rw [Finset.card_erase_of_mem (maximals_subset P L hm), hc]
    omega

/-! ### Realizing a solution by an input -/

/-- Given a down-set `L` and a minimal element `e` of its complement, some
input consistent with `P` lists `L`, then `e`, then the rest. -/
lemma realize {P : SPO α} {L : Finset α} {e : α} (hL : IsDownSet P L)
    (he : e ∈ minimals P Lᶜ) :
    ∃ f, IsLinExt P f ∧ (∀ l ∈ L, f l < f e) ∧ ∀ u, u ∉ L → u ≠ e → f e < f u := by
  classical
  obtain ⟨heL, hemin⟩ := mem_minimals.1 he
  rw [Finset.mem_compl] at heL
  obtain ⟨g, hgP⟩ := exists_linExt P
  obtain ⟨N, hN⟩ : ∃ N, N = univ.sup g + 1 := ⟨_, rfl⟩
  have hgN : ∀ u, g u < N := fun u => hN ▸ Nat.lt_succ_of_le (Finset.le_sup (f := g) (mem_univ u))
  let f : α → ℕ := fun u => if u ∈ L then g u else if u = e then N else N + 1 + g u
  have fL : ∀ u, u ∈ L → f u = g u := fun u hu => by simp [f, hu]
  have fe : f e = N := by simp [f, heL]
  have fU : ∀ u, u ∉ L → u ≠ e → f u = N + 1 + g u := fun u hu hue => by simp [f, hu, hue]
  refine ⟨f, ⟨?_, ?_⟩, ?_, ?_⟩
  · intro u v huv
    by_cases hu : u ∈ L <;> by_cases hv : v ∈ L
    · rw [fL u hu, fL v hv] at huv; exact hgP.1 huv
    · exfalso
      rw [fL u hu] at huv
      have := hgN u
      by_cases hve : v = e
      · subst hve; rw [fe] at huv; omega
      · rw [fU v hv hve] at huv; omega
    · exfalso
      rw [fL v hv] at huv
      have := hgN v
      by_cases hue : u = e
      · subst hue; rw [fe] at huv; omega
      · rw [fU u hu hue] at huv; omega
    · by_cases hue : u = e <;> by_cases hve : v = e
      · rw [hue, hve]
      · subst hue; rw [fe, fU v hv hve] at huv; omega
      · subst hve; rw [fe, fU u hu hue] at huv; omega
      · rw [fU u hu hue, fU v hv hve] at huv; exact hgP.1 (by omega)
  · intro u v huv
    by_cases hv : v ∈ L
    · have hu : u ∈ L := hL u v huv hv
      rw [fL u hu, fL v hv]; exact hgP.2 u v huv
    · by_cases hu : u ∈ L
      · rw [fL u hu]
        have := hgN u
        by_cases hve : v = e
        · rw [hve, fe]; exact this
        · rw [fU v hv hve]; omega
      · by_cases hue : u = e
        · subst hue
          have hve : v ≠ u := fun h => P.irrefl u (h ▸ huv)
          rw [fe, fU v hv hve]; omega
        · by_cases hve : v = e
          · subst hve
            exact absurd huv (hemin u (Finset.mem_compl.2 hu))
          · rw [fU u hu hue, fU v hv hve]
            have := hgP.2 u v huv
            omega
  · intro l hl
    rw [fL l hl, fe]; exact hgN l
  · intro u hu hue
    rw [fe, fU u hu hue]; omega

/-! ### The two views of a compatible solution -/

/-- `(L, e) ∈ C(P)` iff `L ∈ Down_{i-1}(P)` and `e` is a minimal element of the
complement of `L`. -/
theorem mem_C {P : SPO α} {i : ℕ} {s : Finset α × α} :
    s ∈ C P i ↔ s.1 ∈ Down P (i - 1) ∧ s.2 ∈ minimals P s.1ᶜ := by
  rw [mem_C_def, mem_Down, mem_minimals]
  constructor
  · rintro ⟨hc, f, hf, hL, hU⟩
    have heL : s.2 ∉ s.1 := fun h => lt_irrefl _ (hL _ h)
    refine ⟨⟨?_, hc⟩, Finset.mem_compl.2 heL, ?_⟩
    · intro u v huv hv
      by_contra hu
      have h1 := hL v hv
      have h2 := hf.2 u v huv
      by_cases hue : u = s.2
      · subst hue; omega
      · have := hU u hu hue; omega
    · intro u hu hue
      rw [Finset.mem_compl] at hu
      have h1 := hf.2 u s.2 hue
      have h2 := hU u hu (P.ne_of_lt hue)
      omega
  · rintro ⟨⟨hL, hc⟩, he, hmin⟩
    exact ⟨hc, realize hL (mem_minimals.2 ⟨he, hmin⟩)⟩

/-- The second view (for `1 ≤ i`): `(L, e) ∈ C(P)` iff `e ∉ L`, `A = L ∪ {e}`
lies in `Down_i(P)`, and `e` is a maximal element of `A`. -/
theorem mem_C_iff_A {P : SPO α} {i : ℕ} (hi : 1 ≤ i) {s : Finset α × α} :
    s ∈ C P i ↔ s.2 ∉ s.1 ∧ insert s.2 s.1 ∈ Down P i ∧
      s.2 ∈ maximals P (insert s.2 s.1) := by
  rw [mem_C, mem_Down, mem_Down, mem_minimals, mem_maximals]
  obtain ⟨L, e⟩ := s
  simp only
  constructor
  · rintro ⟨⟨hL, hc⟩, he, hmin⟩
    rw [Finset.mem_compl] at he
    refine ⟨he, ⟨?_, ?_⟩, Finset.mem_insert_self _ _, ?_⟩
    · intro u v huv hv
      rw [Finset.mem_insert] at hv ⊢
      rcases hv with rfl | hv
      · by_cases hu : u ∈ L
        · exact Or.inr hu
        · exact absurd huv (hmin u (Finset.mem_compl.2 hu))
      · exact Or.inr (hL u v huv hv)
    · rw [Finset.card_insert_of_notMem he, hc]; omega
    · intro u hu heu
      rw [Finset.mem_insert] at hu
      rcases hu with rfl | hu
      · exact P.irrefl _ heu
      · exact he (hL e u heu hu)
  · rintro ⟨he, ⟨hA, hc⟩, -, hmax⟩
    refine ⟨⟨?_, ?_⟩, Finset.mem_compl.2 he, ?_⟩
    · intro u v huv hv
      have hu := hA u v huv (Finset.mem_insert_of_mem hv)
      rw [Finset.mem_insert] at hu
      rcases hu with rfl | hu
      · exact absurd huv (hmax v (Finset.mem_insert_of_mem hv))
      · exact hu
    · rw [Finset.card_insert_of_notMem he] at hc; omega
    · intro u hu hue
      rw [Finset.mem_compl] at hu
      have := hA u e hue (Finset.mem_insert_self _ _)
      rw [Finset.mem_insert] at this
      rcases this with rfl | h
      · exact P.irrefl _ hue
      · exact hu h

end LB
