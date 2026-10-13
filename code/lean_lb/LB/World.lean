/-
  Obligation 3 (second half): Lemma `lem:world` ("one set at a time").

  For incomparable `x ≠ y` in `P`, `P1 = P + xy`, `P2 = P + yx`, a fixed
  down-set `L` of `P` has weight `ψ_L(Q) = 2^{b_Q(L)}` if `L` is still a
  down-set of `Q`, else `0`.  Dually `A` has weight `2^{a_Q(A)}`.
  The paper states the lemma for `L ∈ Down_{i-1}(P)`, `A ∈ Down_i(P)`; it
  holds for every down-set, which is what we prove.
-/
import LB.Closure

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

open Classical in
/-- `ψ_L(Q)`: weight of the fixed lower set `L` in the order `Q`. -/
noncomputable def psiL (Q : SPO α) (L : Finset α) : ℕ :=
  if IsDownSet Q L then 2 ^ bOf Q L else 0

open Classical in
/-- The order-reversed weight of a fixed initial set `A` in the order `Q`. -/
noncomputable def psiA (Q : SPO α) (A : Finset α) : ℕ :=
  if IsDownSet Q A then 2 ^ aOf Q A else 0

lemma psiL_of_isDownSet {Q : SPO α} {L : Finset α} (h : IsDownSet Q L) :
    psiL Q L = 2 ^ bOf Q L := by
  unfold psiL; simp [h]

lemma psiL_of_not_isDownSet {Q : SPO α} {L : Finset α} (h : ¬ IsDownSet Q L) :
    psiL Q L = 0 := by
  unfold psiL; simp [h]

lemma psiA_of_isDownSet {Q : SPO α} {A : Finset α} (h : IsDownSet Q A) :
    psiA Q A = 2 ^ aOf Q A := by
  unfold psiA; simp [h]

lemma psiA_of_not_isDownSet {Q : SPO α} {A : Finset α} (h : ¬ IsDownSet Q A) :
    psiA Q A = 0 := by
  unfold psiA; simp [h]

/-- `2^b ≤ 2^{b₁} + 2^{b₂}` when each child loses at most one candidate. -/
lemma two_pow_le_add {b b₁ b₂ : ℕ} (h₁ : b ≤ b₁ + 1) (h₂ : b ≤ b₂ + 1) :
    2 ^ b ≤ 2 ^ b₁ + 2 ^ b₂ := by
  rcases b with _ | k
  · rw [pow_zero]; exact le_add_right Nat.one_le_two_pow
  · have e₁ : 2 ^ k ≤ 2 ^ b₁ := Nat.pow_le_pow_right (by norm_num) (by omega)
    have e₂ : 2 ^ k ≤ 2 ^ b₂ := Nat.pow_le_pow_right (by norm_num) (by omega)
    rw [Nat.pow_succ]; omega

section World

variable {P : SPO α} {x y : α}

lemma not_le_of_incomp (hne : x ≠ y) (hyx : ¬ P.lt y x) : ¬ P.le y x := by
  rintro (h | h)
  · exact hne h.symm
  · exact hyx h

/-- Comparison with `x ∈ L`, outcome `x < y`: the weight of `L` is unchanged. -/
lemma psiL_add_of_mem (h : ¬ P.le y x) {L : Finset α} (hL : IsDownSet P L) (hx : x ∈ L) :
    psiL (P.add x y) L = psiL P L := by
  have hL' : IsDownSet (P.add x y) L := (isDownSet_add_iff h hL).2 fun _ => hx
  rw [psiL_of_isDownSet hL', psiL_of_isDownSet hL]
  unfold bOf
  rw [minimals_add_of_mem h hL hx]

/-- Outcome `x < y` with `y ∈ L`, `x ∉ L`: `L` is no longer a down-set. -/
lemma psiL_add_of_cross (h : ¬ P.le y x) {L : Finset α} (hL : IsDownSet P L) (hx : x ∉ L)
    (hy : y ∈ L) : psiL (P.add x y) L = 0 :=
  psiL_of_not_isDownSet fun hL' => hx ((isDownSet_add_iff h hL).1 hL' hy)

/-- Outcome `x < y` with `y ∉ L`: `L` stays a down-set and loses at most one
candidate. -/
lemma bOf_le_add (h : ¬ P.le y x) {L : Finset α} (hx : x ∉ L) (hy : y ∉ L) :
    bOf P L ≤ bOf (P.add x y) L + 1 := by
  have hne : Lᶜ.Nonempty := ⟨x, Finset.mem_compl.2 hx⟩
  have h1 := card_minimals_le_add (x := x) h hy (P := P) (L := L)
  have h2 := card_minimals_pos (P.add x y) hne
  unfold bOf; omega

/-- **Lemma `lem:world`, lower sets.**  Let `x ≠ y` be incomparable in `P`, `L`
a down-set of `P`.  (1) both in `L`: both outcomes keep the weight; (2) exactly
one in `L`: one outcome keeps it and the other sets it to `0`; (3) neither in
`L`: the two outcomes together keep at least the weight. -/
theorem world_L (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) {L : Finset α}
    (hL : IsDownSet P L) :
    (x ∈ L → y ∈ L → psiL (P.add x y) L = psiL P L ∧ psiL (P.add y x) L = psiL P L) ∧
    (x ∈ L → y ∉ L → psiL (P.add x y) L = psiL P L ∧ psiL (P.add y x) L = 0) ∧
    (x ∉ L → y ∈ L → psiL (P.add x y) L = 0 ∧ psiL (P.add y x) L = psiL P L) ∧
    (x ∉ L → y ∉ L → psiL P L ≤ psiL (P.add x y) L + psiL (P.add y x) L) := by
  have h₁ : ¬ P.le y x := not_le_of_incomp hne hyx
  have h₂ : ¬ P.le x y := not_le_of_incomp hne.symm hxy
  refine ⟨fun hx hy => ⟨psiL_add_of_mem h₁ hL hx, psiL_add_of_mem h₂ hL hy⟩,
    fun hx hy => ⟨psiL_add_of_mem h₁ hL hx, psiL_add_of_cross h₂ hL hy hx⟩,
    fun hx hy => ⟨psiL_add_of_cross h₁ hL hx hy, psiL_add_of_mem h₂ hL hy⟩, fun hx hy => ?_⟩
  have hL₁ : IsDownSet (P.add x y) L := (isDownSet_add_iff h₁ hL).2 fun h => absurd h hy
  have hL₂ : IsDownSet (P.add y x) L := (isDownSet_add_iff h₂ hL).2 fun h => absurd h hx
  rw [psiL_of_isDownSet hL, psiL_of_isDownSet hL₁, psiL_of_isDownSet hL₂]
  exact two_pow_le_add (bOf_le_add h₁ hx hy) (bOf_le_add h₂ hy hx)

/-- Comparison with `y ∉ A`, outcome `x < y` (with `x ∉ A` or `x ∈ A`): the
weight of `A` is unchanged. -/
lemma psiA_add_of_not_mem (h : ¬ P.le y x) {A : Finset α} (hA : IsDownSet P A) (hy : y ∉ A) :
    psiA (P.add x y) A = psiA P A := by
  have hA' : IsDownSet (P.add x y) A := (isDownSet_add_iff h hA).2 fun h => absurd h hy
  rw [psiA_of_isDownSet hA', psiA_of_isDownSet hA]
  unfold aOf
  rw [maximals_add_of_not_mem h hA hy]

/-- Outcome `x < y` with `x ∈ A`: `A` stays a down-set and loses at most one
candidate. -/
lemma aOf_le_add (h : ¬ P.le y x) {A : Finset α} (hx : x ∈ A) :
    aOf P A ≤ aOf (P.add x y) A + 1 := by
  have h1 := card_maximals_le_add (y := y) h hx (P := P)
  have h2 := card_maximals_pos (P.add x y) ⟨x, hx⟩
  unfold aOf; omega

/-- **Lemma `lem:world`, initial sets** ("with inside and outside exchanged"):
(1) both outside `A`: both outcomes keep the weight; (2) exactly one in `A`:
one outcome keeps it, the other sets it to `0`; (3) both in `A`: the two
outcomes together keep at least the weight. -/
theorem world_A (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) {A : Finset α}
    (hA : IsDownSet P A) :
    (x ∉ A → y ∉ A → psiA (P.add x y) A = psiA P A ∧ psiA (P.add y x) A = psiA P A) ∧
    (x ∈ A → y ∉ A → psiA (P.add x y) A = psiA P A ∧ psiA (P.add y x) A = 0) ∧
    (x ∉ A → y ∈ A → psiA (P.add x y) A = 0 ∧ psiA (P.add y x) A = psiA P A) ∧
    (x ∈ A → y ∈ A → psiA P A ≤ psiA (P.add x y) A + psiA (P.add y x) A) := by
  have h₁ : ¬ P.le y x := not_le_of_incomp hne hyx
  have h₂ : ¬ P.le x y := not_le_of_incomp hne.symm hxy
  refine ⟨fun _ hy => ⟨psiA_add_of_not_mem h₁ hA hy, ?_⟩, fun hx hy => ⟨psiA_add_of_not_mem h₁ hA hy,
    psiA_of_not_isDownSet fun hA' => hy ((isDownSet_add_iff h₂ hA).1 hA' hx)⟩,
    fun hx hy => ⟨psiA_of_not_isDownSet fun hA' => hx ((isDownSet_add_iff h₁ hA).1 hA' hy),
      psiA_add_of_not_mem h₂ hA hx⟩, fun hx hy => ?_⟩
  · rename_i hx
    exact psiA_add_of_not_mem h₂ hA hx
  · have hA₁ : IsDownSet (P.add x y) A := (isDownSet_add_iff h₁ hA).2 fun _ => hx
    have hA₂ : IsDownSet (P.add y x) A := (isDownSet_add_iff h₂ hA).2 fun _ => hy
    rw [psiA_of_isDownSet hA, psiA_of_isDownSet hA₁, psiA_of_isDownSet hA₂]
    exact two_pow_le_add (aOf_le_add h₁ hx) (aOf_le_add h₂ hy)

end World

end LB
