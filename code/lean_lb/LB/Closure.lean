/-
  Obligation 3 (first half): the order after one comparison.

  `P.add x y` is the paper's `P + xy` (`P1 = P + xy`, `P2 = P + yx`): the known
  order after learning `x < y`.  We define it by the explicit formula of the
  paper ("adding x < y creates only relations u < v with u ≤ x and y ≤ v") and
  PROVE that it is exactly the transitive closure of `P ∪ {(x, y)}`
  (`add_lt_iff_transGen`), so the formula is not an assumption.

  Then the facts the one-set lemma needs: how inputs, down-sets, minimal
  elements of a complement and maximal elements of a down-set change.
-/
import LB.Count
import Mathlib.Logic.Relation

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

namespace SPO

variable (P : SPO α) {x y u v w : α}

/-- The relation after learning `x < y`: `u < v` iff `P u v` or `u ≤ x ∧ y ≤ v`. -/
def addRel (x y u v : α) : Prop := P.lt u v ∨ (P.le u x ∧ P.le y v)

lemma addRel_irrefl (h : ¬ P.le y x) (u : α) : ¬ P.addRel x y u u := by
  rintro (hu | ⟨h₁, h₂⟩)
  · exact P.irrefl u hu
  · exact h (P.le_trans' h₂ h₁)

lemma addRel_trans (h : ¬ P.le y x) {u v w : α} (h₁ : P.addRel x y u v)
    (h₂ : P.addRel x y v w) : P.addRel x y u w := by
  rcases h₁ with h₁ | ⟨hux, hyv⟩ <;> rcases h₂ with h₂ | ⟨hvx, hyw⟩
  · exact Or.inl (P.trans h₁ h₂)
  · exact Or.inr ⟨P.le_of_lt (P.lt_of_lt_of_le h₁ hvx), hyw⟩
  · exact Or.inr ⟨hux, P.le_trans' hyv (P.le_of_lt h₂)⟩
  · exact absurd (P.le_trans' hyv hvx) h

open Classical in
/-- `P + xy`: the known order after the comparison outcome `x < y`.  Only
meaningful when `y ≤ x` is not already known (then `x < y` is consistent);
otherwise it returns `P` unchanged (this case is never used). -/
noncomputable def add (x y : α) : SPO α :=
  if h : P.le y x then P
  else ⟨P.addRel x y, P.addRel_irrefl h, P.addRel_trans h⟩

lemma add_lt_iff (h : ¬ P.le y x) :
    (P.add x y).lt u v ↔ P.lt u v ∨ (P.le u x ∧ P.le y v) := by
  unfold add; simp only [h, ↓reduceDIte]; rfl

/-- Known relations stay known. -/
lemma lt_add_of_lt (huv : P.lt u v) : (P.add x y).lt u v := by
  by_cases h : P.le y x
  · unfold add; simp only [h, ↓reduceDIte]; exact huv
  · exact (P.add_lt_iff h).2 (Or.inl huv)

lemma add_lt_self (h : ¬ P.le y x) : (P.add x y).lt x y :=
  (P.add_lt_iff h).2 (Or.inr ⟨P.le_rfl', P.le_rfl'⟩)

/-- Sanity check of the definition: `P + xy` is the transitive closure of `P`
together with the new pair `(x, y)`. -/
theorem add_lt_iff_transGen (h : ¬ P.le y x) :
    (P.add x y).lt u v ↔ Relation.TransGen (fun a b => P.lt a b ∨ (a = x ∧ b = y)) u v := by
  constructor
  · intro huv
    rcases (P.add_lt_iff h).1 huv with huv | ⟨hux, hyv⟩
    · exact Relation.TransGen.single (Or.inl huv)
    · have hxy : Relation.TransGen (fun a b => P.lt a b ∨ (a = x ∧ b = y)) x y :=
        Relation.TransGen.single (Or.inr ⟨rfl, rfl⟩)
      have huy : Relation.TransGen (fun a b => P.lt a b ∨ (a = x ∧ b = y)) u y := by
        rcases hux with rfl | hux
        · exact hxy
        · exact Relation.TransGen.head (Or.inl hux) hxy
      rcases hyv with rfl | hyv
      · exact huy
      · exact Relation.TransGen.tail huy (Or.inl hyv)
  · intro huv
    induction huv with
    | single hab =>
      rcases hab with hab | ⟨rfl, rfl⟩
      · exact P.lt_add_of_lt hab
      · exact P.add_lt_self h
    | tail _ hbc ih =>
      refine (P.add x y).trans ih ?_
      rcases hbc with hbc | ⟨rfl, rfl⟩
      · exact P.lt_add_of_lt hbc
      · exact P.add_lt_self h

end SPO

/-! ### Inputs -/

lemma not_le_of_lt_input {P : SPO α} {f : α → ℕ} {x y : α} (hf : IsLinExt P f)
    (hxy : f x < f y) : ¬ P.le y x := by
  rintro (rfl | h)
  · exact lt_irrefl _ hxy
  · exact absurd (hf.2 y x h) (by omega)

/-- An input consistent with `P` and with `x < y` is consistent with `P + xy`. -/
lemma isLinExt_add {P : SPO α} {f : α → ℕ} {x y : α} (hf : IsLinExt P f)
    (hxy : f x < f y) : IsLinExt (P.add x y) f := by
  refine ⟨hf.1, fun u v huv => ?_⟩
  rcases (P.add_lt_iff (not_le_of_lt_input hf hxy)).1 huv with huv | ⟨hux, hyv⟩
  · exact hf.2 u v huv
  · have h1 : f u ≤ f x := by
      rcases hux with rfl | hux
      · exact le_rfl
      · exact (hf.2 _ _ hux).le
    have h2 : f y ≤ f v := by
      rcases hyv with rfl | hyv
      · exact le_rfl
      · exact (hf.2 _ _ hyv).le
    omega

/-- An input consistent with `P + xy` is consistent with `P`. -/
lemma IsLinExt.of_add {P : SPO α} {f : α → ℕ} {x y : α} (hf : IsLinExt (P.add x y) f) :
    IsLinExt P f :=
  ⟨hf.1, fun u v huv => hf.2 u v (P.lt_add_of_lt huv)⟩

lemma IsLinExt.lt_of_add {P : SPO α} {f : α → ℕ} {x y : α} (h : ¬ P.le y x)
    (hf : IsLinExt (P.add x y) f) : f x < f y :=
  hf.2 x y (P.add_lt_self h)

/-! ### Down-sets and candidates after one comparison -/

/-- A down-set `L` of `P` stays a down-set of `P + xy` iff the outcome does not
place an element outside `L` below one inside (`y ∈ L → x ∈ L`). -/
lemma isDownSet_add_iff {P : SPO α} {L : Finset α} {x y : α} (h : ¬ P.le y x)
    (hL : IsDownSet P L) : IsDownSet (P.add x y) L ↔ (y ∈ L → x ∈ L) := by
  constructor
  · intro hL' hy
    exact hL' x y (P.add_lt_self h) hy
  · intro hyx u v huv hv
    rcases (P.add_lt_iff h).1 huv with huv | ⟨hux, hyv⟩
    · exact hL u v huv hv
    · have hy : y ∈ L := by
        rcases hyv with rfl | hyv
        · exact hv
        · exact hL _ _ hyv hv
      rcases hux with rfl | hux
      · exact hyx hy
      · exact hL _ _ hux (hyx hy)

/-- If `x ∈ L` (a down-set), the outcome `x < y` creates no relation starting
outside `L`. -/
lemma add_lt_iff_of_mem {P : SPO α} {L : Finset α} {x y u v : α} (h : ¬ P.le y x)
    (hL : IsDownSet P L) (hx : x ∈ L) (hu : u ∉ L) :
    (P.add x y).lt u v ↔ P.lt u v := by
  rw [P.add_lt_iff h]
  constructor
  · rintro (huv | ⟨hux, -⟩)
    · exact huv
    · exfalso
      rcases hux with rfl | hux
      · exact hu hx
      · exact hu (hL _ _ hux hx)
  · exact Or.inl

/-- Comparison with `x` inside `L`: the candidates (minimal elements of the
complement) are unchanged. -/
lemma minimals_add_of_mem {P : SPO α} {L : Finset α} {x y : α} (h : ¬ P.le y x)
    (hL : IsDownSet P L) (hx : x ∈ L) : minimals (P.add x y) Lᶜ = minimals P Lᶜ := by
  ext v
  simp only [mem_minimals]
  refine and_congr_right fun _ => forall₂_congr fun u hu => ?_
  rw [add_lt_iff_of_mem h hL hx (Finset.mem_compl.1 hu)]

/-- Comparison with `y` outside `L`: at most one candidate (namely `y`) is lost. -/
lemma minimals_erase_subset_add {P : SPO α} {L : Finset α} {x y : α} (h : ¬ P.le y x)
    (hy : y ∉ L) : (minimals P Lᶜ).erase y ⊆ minimals (P.add x y) Lᶜ := by
  intro v hv
  rw [Finset.mem_erase, mem_minimals] at hv
  obtain ⟨hvy, hvL, hmin⟩ := hv
  refine mem_minimals.2 ⟨hvL, fun u hu huv => ?_⟩
  rcases (P.add_lt_iff h).1 huv with huv | ⟨-, hyv⟩
  · exact hmin u hu huv
  · rcases hyv with rfl | hyv
    · exact hvy rfl
    · exact hmin y (Finset.mem_compl.2 hy) hyv

lemma card_minimals_le_add {P : SPO α} {L : Finset α} {x y : α} (h : ¬ P.le y x)
    (hy : y ∉ L) : (minimals P Lᶜ).card ≤ (minimals (P.add x y) Lᶜ).card + 1 := by
  have h1 := Finset.card_le_card (minimals_erase_subset_add (x := x) h hy)
  have h2 := Finset.pred_card_le_card_erase (s := minimals P Lᶜ) (a := y)
  omega

/-- If `y ∉ A` (a down-set), the outcome `x < y` creates no relation ending
inside `A`. -/
lemma add_lt_iff_of_not_mem {P : SPO α} {A : Finset α} {x y u v : α} (h : ¬ P.le y x)
    (hA : IsDownSet P A) (hy : y ∉ A) (hv : v ∈ A) :
    (P.add x y).lt u v ↔ P.lt u v := by
  rw [P.add_lt_iff h]
  constructor
  · rintro (huv | ⟨-, hyv⟩)
    · exact huv
    · exfalso
      rcases hyv with rfl | hyv
      · exact hy hv
      · exact hy (hA _ _ hyv hv)
  · exact Or.inl

/-- Comparison with `y` outside `A`: the candidates (maximal elements of `A`)
are unchanged. -/
lemma maximals_add_of_not_mem {P : SPO α} {A : Finset α} {x y : α} (h : ¬ P.le y x)
    (hA : IsDownSet P A) (hy : y ∉ A) : maximals (P.add x y) A = maximals P A := by
  ext v
  simp only [mem_maximals]
  refine and_congr_right fun _ => forall₂_congr fun u hu => ?_
  rw [add_lt_iff_of_not_mem h hA hy hu]

/-- Comparison with `x` inside `A`: at most one candidate (namely `x`) is lost. -/
lemma maximals_erase_subset_add {P : SPO α} {A : Finset α} {x y : α} (h : ¬ P.le y x)
    (hx : x ∈ A) : (maximals P A).erase x ⊆ maximals (P.add x y) A := by
  intro v hv
  rw [Finset.mem_erase, mem_maximals] at hv
  obtain ⟨hvx, hvA, hmax⟩ := hv
  refine mem_maximals.2 ⟨hvA, fun u hu hvu => ?_⟩
  rcases (P.add_lt_iff h).1 hvu with hvu | ⟨hvx', -⟩
  · exact hmax u hu hvu
  · rcases hvx' with rfl | hvx'
    · exact hvx rfl
    · exact hmax x hx hvx'

lemma card_maximals_le_add {P : SPO α} {A : Finset α} {x y : α} (h : ¬ P.le y x)
    (hx : x ∈ A) : (maximals P A).card ≤ (maximals (P.add x y) A).card + 1 := by
  have h1 := Finset.card_le_card (maximals_erase_subset_add (y := y) h hx)
  have h2 := Finset.pred_card_le_card_erase (s := maximals P A) (a := x)
  omega

/-- Down-sets of `P + xy` are down-sets of `P`. -/
lemma IsDownSet.of_add {P : SPO α} {S : Finset α} {x y : α} (hS : IsDownSet (P.add x y) S) :
    IsDownSet P S := fun u v huv hv => hS u v (P.lt_add_of_lt huv) hv

end LB
