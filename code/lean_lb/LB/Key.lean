/-
  Theorem `thm:key`:  V_i(P) ≥ KEY(P) = max_{s ∈ C(P)} (a_s + b_s).

  The paper's argument, along ONE input: fix a solution s = (L, e, U) and an
  input f consistent with P that lists L, then e, then U.  Let A = L ∪ {e}
  (an initial segment of f) and M = complement of L (a final segment).  Along
  the path of f, a comparison removes at most one maximal element of A, and
  only if both compared labels lie in A; it removes at most one minimal
  element of M, and only if both lie in M; A ∩ M = {e}, so no comparison does
  both.  At the (correct) leaf, e is the only maximal element of A and the
  only minimal element of M.  Hence the path has ≥ a_s + b_s comparisons.

  Formally we prove, by induction over the tree with the known order Q along
  the path:  |Max_Q(A)| + |Min_Q(M)| ≤ pathLen + 2.
-/
import LB.Tree

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-- The induction behind `thm:key`. -/
lemma key_aux {i : ℕ} {f : α → ℕ} {L : Finset α} {e : α}
    (hLf : ∀ u, u ∈ L ↔ f u < f e) (hci : L.card = i - 1) :
    ∀ (T : CTree α) (Q : SPO α), IsLinExt Q f → Correct Q i T →
      (maximals Q (insert e L)).card + (minimals Q Lᶜ).card ≤ T.pathLen f + 2 := by
  intro T
  induction T with
  | leaf e' =>
    intro Q hf hT
    -- the leaf returns e (the rank-i element of f), and Q is solved there
    have hbelow : univ.filter (fun u => f u < f e) = L := by
      ext u; simp [hLf u]
    have he' : e' = e := by
      apply eq_of_card_below_eq hf.1
      have : (univ.filter fun u => f u < f e').card = i - 1 := hT f hf
      rw [this, hbelow, hci]
    subst he'
    obtain ⟨hcomp, -⟩ := solved_of_correct_leaf hT
    have hmax : maximals Q (insert e' L) ⊆ {e'} := by
      intro v hv
      rw [mem_maximals, Finset.mem_insert] at hv
      rw [Finset.mem_singleton]
      obtain ⟨hv, hvmax⟩ := hv
      by_contra hve
      rcases hcomp v hve with h | h
      · exact hvmax e' (Finset.mem_insert_self _ _) h
      · rcases hv with rfl | hv
        · exact hve rfl
        · have := hf.2 _ _ h; have := (hLf v).1 hv; omega
    have hmin : minimals Q Lᶜ ⊆ {e'} := by
      intro v hv
      rw [mem_minimals, Finset.mem_compl] at hv
      rw [Finset.mem_singleton]
      obtain ⟨hv, hvmin⟩ := hv
      by_contra hve
      rcases hcomp v hve with h | h
      · have := hf.2 _ _ h
        exact hv ((hLf v).2 this)
      · exact hvmin e' (Finset.mem_compl.2 fun h' => lt_irrefl _ ((hLf e').1 h')) h
    have h1 := Finset.card_le_card hmax
    have h2 := Finset.card_le_card hmin
    simp only [Finset.card_singleton] at h1 h2
    simp only [CTree.pathLen]
    omega
  | node x y t₁ t₂ ih₁ ih₂ =>
    intro Q hf hT
    -- L and A = insert e L are down-sets of Q (initial segments of f)
    have hA : ∀ u, u ∈ insert e L ↔ f u ≤ f e := by
      intro u
      rw [Finset.mem_insert, hLf u]
      constructor
      · rintro (rfl | h) <;> omega
      · intro h
        rcases Nat.lt_or_eq_of_le h with h | h
        · exact Or.inr h
        · exact Or.inl (hf.1 h)
    have hLdown : IsDownSet Q L := fun u v huv hv => by
      have := hf.2 u v huv; rw [hLf] at hv ⊢; omega
    have hAdown : IsDownSet Q (insert e L) := fun u v huv hv => by
      have := hf.2 u v huv; rw [hA] at hv ⊢; omega
    -- one comparison `a < b` (in f) changes the count by at most one
    have step : ∀ a b : α, f a < f b →
        (maximals Q (insert e L)).card + (minimals Q Lᶜ).card ≤
          (maximals (Q.add a b) (insert e L)).card + (minimals (Q.add a b) Lᶜ).card + 1 := by
      intro a b hab
      have h := not_le_of_lt_input hf hab
      by_cases hb : f b ≤ f e
      · -- both in A: only maximal elements of A can be lost, and only `a`
        have haL : a ∈ L := (hLf a).2 (by omega)
        have e1 := card_maximals_le_add h ((hA a).2 (by omega)) (y := b) (P := Q)
          (A := insert e L)
        rw [minimals_add_of_mem h hLdown haL]
        omega
      · -- `b` outside A: only minimal elements of M can be lost, and only `b`
        have hbA : b ∉ insert e L := fun h' => hb ((hA b).1 h')
        have hbL : b ∉ L := fun h' => hbA (Finset.mem_insert_of_mem h')
        have e2 := card_minimals_le_add h hbL (x := a) (P := Q) (L := L)
        rw [maximals_add_of_not_mem h hAdown hbA]
        omega
    simp only [CTree.pathLen]
    by_cases hxy : f x < f y
    · rw [ite_eq_left hxy]
      have h := not_le_of_lt_input hf hxy
      have := ih₁ (Q.add x y) (isLinExt_add hf hxy) (correct_add_left hT h)
      have := step x y hxy
      omega
    · rw [ite_eq_right hxy]
      by_cases hxy' : x = y
      · subst hxy'
        have := ih₂ Q hf (correct_ge_of_le hT (Or.inl rfl))
        omega
      · have hyx : f y < f x := by
          rcases lt_or_gt_of_ne (hf.1.ne hxy') with h | h
          · exact absurd h hxy
          · exact h
        have h := not_le_of_lt_input hf hyx
        have := ih₂ (Q.add y x) (isLinExt_add hf hyx) (correct_add_right hT h)
        have := step y x hyx
        omega

/-- **Theorem `thm:key`, path form.**  For a correct tree `T`, a compatible
solution `s = (L, e)` and ANY input `f` consistent with `P` that lists `L`,
then `e`, then the rest, the path of `f` makes at least `a_s + b_s`
comparisons. -/
theorem key_le_pathLen {P : SPO α} {i : ℕ} {T : CTree α} (hT : Correct P i T)
    {L : Finset α} {e : α} (hs : (L, e) ∈ C P i) {f : α → ℕ} (hf : IsLinExt P f)
    (h1 : ∀ l ∈ L, f l < f e) (h2 : ∀ u, u ∉ L → u ≠ e → f e < f u) :
    aOf P (insert e L) + bOf P L ≤ T.pathLen f := by
  have hLf : ∀ u, u ∈ L ↔ f u < f e := by
    intro u
    rw [← below_eq_of_realize h1 h2]; simp
  have hci : L.card = i - 1 := (mem_Down.1 (mem_C.1 hs).1).2
  have := key_aux hLf hci T P hf hT
  have hmax := card_maximals_pos P (Finset.insert_nonempty e L)
  have hmin : 0 < (minimals P Lᶜ).card := Finset.card_pos.2 ⟨e, (mem_C.1 hs).2⟩
  unfold aOf bOf
  omega

/-- **Theorem `thm:key`.**  If a correct tree for `(P, i)` makes at most `d`
comparisons on every input consistent with `P`, then `KEY(P) ≤ d`. -/
theorem KEY_le_of_cost {P : SPO α} {i : ℕ} {T : CTree α} (hT : Correct P i T) (d : ℕ)
    (hd : ∀ f, IsLinExt P f → T.pathLen f ≤ d) : KEY P i ≤ d := by
  unfold KEY
  refine Finset.sup_le fun s hs => ?_
  obtain ⟨-, f, hf, h1, h2⟩ := mem_C_def.1 hs
  have := key_le_pathLen hT (L := s.1) (e := s.2) hs hf h1 h2
  have := hd f hf
  omega

theorem KEY_le_depth {P : SPO α} {i : ℕ} {T : CTree α} (hT : Correct P i T) :
    KEY P i ≤ T.depth :=
  KEY_le_of_cost hT T.depth fun f _ => T.pathLen_le_depth f

end LB
