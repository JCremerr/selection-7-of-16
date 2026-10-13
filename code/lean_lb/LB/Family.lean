/-
  main.tex, Proposition prop:phiopt and the text around it:
    "validity is a weaker requirement than bounding the number of leaves of
     every correct tree, and the latter is weaker than satisfying the local
     inequality used in Theorem thm:phi" ... "Thus Phi* is pointwise largest
     among valid formulas in this family, including those that bound leaves
     or satisfy the local inequality."

  * Pruning: deleting the comparisons whose outcome is already known (and
    with them the unreachable branches) keeps a tree correct, does not
    increase its number of leaves, and makes its depth at most its
    worst-case number of comparisons on consistent inputs.  Hence V_i(P) is
    also the least DEPTH of a correct tree.
  * local inequality ⇒ bounds leaves ⇒ valid, for any weight, and the
    family members Psi_{F,G} (F(0) = G(0) = 0) equal 1 at solved states.
  * Consequently prop:phiopt applies to members that bound leaves or satisfy
    the local inequality.
-/
import LB.Optimality

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### Pruning infeasible branches -/

open Classical in
/-- Remove the comparisons whose outcome is known in the current state `Q`
(following the only feasible branch); keep incomparable comparisons. -/
noncomputable def prune : CTree α → SPO α → CTree α
  | .leaf e, _ => .leaf e
  | .node x y t₁ t₂, Q =>
    if Q.lt x y then prune t₁ Q
    else if Q.le y x then prune t₂ Q
    else .node x y (prune t₁ (Q.add x y)) (prune t₂ (Q.add y x))

lemma eval_prune (T : CTree α) : ∀ (Q : SPO α) (f : α → ℕ), IsLinExt Q f →
    (prune T Q).eval f = T.eval f := by
  induction T with
  | leaf e => intro Q f _; rfl
  | node x y t₁ t₂ ih₁ ih₂ =>
    intro Q f hf
    by_cases hxy : Q.lt x y
    · have h := hf.2 x y hxy
      simp only [prune, hxy, ↓reduceIte, CTree.eval, h]
      exact ih₁ Q f hf
    · by_cases hyx : Q.le y x
      · have h : ¬ f x < f y := by
          rcases hyx with rfl | hyx
          · exact lt_irrefl _
          · have := hf.2 y x hyx; omega
        simp only [prune, hxy, hyx, ↓reduceIte, CTree.eval, h]
        exact ih₂ Q f hf
      · simp only [prune, hxy, hyx, ↓reduceIte, CTree.eval]
        by_cases h : f x < f y
        · simp only [h, ↓reduceIte]
          exact ih₁ _ f (isLinExt_add hf h)
        · simp only [h, ↓reduceIte]
          have hne : x ≠ y := fun e => hyx (Or.inl e.symm)
          have h' : f y < f x := by
            rcases lt_or_gt_of_ne (hf.1.ne hne) with h'' | h''
            · exact absurd h'' h
            · exact h''
          exact ih₂ _ f (isLinExt_add hf h')

/-- Pruning keeps a tree correct. -/
lemma correct_prune {Q : SPO α} {i : ℕ} {T : CTree α} (hT : Correct Q i T) :
    Correct Q i (prune T Q) := by
  intro f hf
  rw [eval_prune T Q f hf]
  exact hT f hf

/-- "Removing infeasible paths only decreases the count." -/
lemma leaves_prune_le (T : CTree α) : ∀ Q : SPO α, (prune T Q).leaves ≤ T.leaves := by
  induction T with
  | leaf e => intro Q; simp [prune, CTree.leaves]
  | node x y t₁ t₂ ih₁ ih₂ =>
    intro Q
    simp only [prune, CTree.leaves]
    split_ifs
    · have := ih₁ Q; omega
    · have := ih₂ Q; omega
    · simp only [CTree.leaves]
      have := ih₁ (Q.add x y); have := ih₂ (Q.add y x); omega

/-- After pruning, the depth is at most the worst-case number of comparisons
on consistent inputs. -/
lemma depth_prune_le (T : CTree α) : ∀ (Q : SPO α) (d : ℕ),
    (∀ f, IsLinExt Q f → T.pathLen f ≤ d) → (prune T Q).depth ≤ d := by
  induction T with
  | leaf e => intro Q d _; simp [prune, CTree.depth]
  | node x y t₁ t₂ ih₁ ih₂ =>
    intro Q d hd
    by_cases hxy : Q.lt x y
    · simp only [prune, hxy, ↓reduceIte]
      have := ih₁ Q d fun f hf => by
        have := hd f hf
        simp only [CTree.pathLen, hf.2 x y hxy, ↓reduceIte] at this
        omega
      exact this
    · by_cases hyx : Q.le y x
      · simp only [prune, hxy, hyx, ↓reduceIte]
        exact ih₂ Q d fun f hf => by
          have := hd f hf
          have h : ¬ f x < f y := by
            rcases hyx with rfl | hyx
            · exact lt_irrefl _
            · have := hf.2 y x hyx; omega
          simp only [CTree.pathLen, h, ↓reduceIte] at this
          omega
      · simp only [prune, hxy, hyx, ↓reduceIte, CTree.depth]
        have hxy' : ¬ Q.le x y := fun h => by
          rcases h with h | h
          · exact hyx (Or.inl h.symm)
          · exact hxy h
        obtain ⟨f₀, hf₀⟩ := exists_linExt Q
        have hd0 := hd f₀ hf₀
        simp only [CTree.pathLen] at hd0
        have e₁ := ih₁ (Q.add x y) (d - 1) fun f hf => by
          have := hd f hf.of_add
          simp only [CTree.pathLen, hf.lt_of_add hyx, ↓reduceIte] at this
          omega
        have e₂ := ih₂ (Q.add y x) (d - 1) fun f hf => by
          have := hd f hf.of_add
          have h : ¬ f x < f y := by have := hf.lt_of_add hxy'; omega
          simp only [CTree.pathLen, h, ↓reduceIte] at this
          omega
        omega

/-- `V_i(P)` is also the least depth of a correct tree. -/
theorem V_eq_min_depth (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (∃ T, Correct P i T ∧ T.depth = V P i) ∧ ∀ T, Correct P i T → V P i ≤ T.depth := by
  have hall : ∀ T, Correct P i T → V P i ≤ T.depth := fun T hT =>
    V_le hT fun f _ => T.pathLen_le_depth f
  obtain ⟨T, hT, hd⟩ := V_spec P hi hin
  refine ⟨⟨prune T P, correct_prune hT, ?_⟩, hall⟩
  exact le_antisymm (depth_prune_le T P _ hd) (hall _ (correct_prune hT))

/-! ### local inequality ⇒ bounds leaves ⇒ valid -/

/-- Any real weight of the known order (rank fixed) that satisfies the local
inequality and is at most `1` at solved states bounds the number of leaves of
every correct tree. -/
theorem le_leaves_of_local {i : ℕ} (W : SPO α → ℝ)
    (hloc : ∀ (Q : SPO α) (x y : α), x ≠ y → ¬ Q.lt x y → ¬ Q.lt y x →
      W Q ≤ W (Q.add x y) + W (Q.add y x))
    (hleaf : ∀ (Q : SPO α) (e : α), Solved Q i e → W Q ≤ 1) :
    ∀ (T : CTree α) (Q : SPO α), Correct Q i T → W Q ≤ T.leaves := by
  intro T
  induction T with
  | leaf e =>
    intro Q hT
    simpa [CTree.leaves] using hleaf Q e (solved_of_correct_leaf hT)
  | node x y t₁ t₂ ih₁ ih₂ =>
    intro Q hT
    simp only [CTree.leaves, Nat.cast_add]
    have h₁ : (0 : ℝ) ≤ t₁.leaves := Nat.cast_nonneg _
    have h₂ : (0 : ℝ) ≤ t₂.leaves := Nat.cast_nonneg _
    by_cases hxy : Q.lt x y
    · have := ih₁ Q (correct_lt_of_lt hT hxy); linarith
    · by_cases hyx : Q.le y x
      · have := ih₂ Q (correct_ge_of_le hT hyx); linarith
      · have hne : x ≠ y := fun h => hyx (Or.inl h.symm)
        have hyx' : ¬ Q.lt y x := fun h => hyx (Or.inr h)
        have hxy' : ¬ Q.le x y := not_le_of_incomp hne.symm hxy
        have e₁ := ih₁ (Q.add x y) (correct_add_left hT hyx)
        have e₂ := ih₂ (Q.add y x) (correct_add_right hT hxy')
        have := hloc Q x y hne hxy hyx'
        linarith

/-- A weight that bounds the number of leaves of every correct tree for
`(P, i)` is valid: it is at most `2^{V_i(P)}`. -/
theorem le_two_pow_V_of_le_leaves (P : SPO α) {i : ℕ} (hi : 1 ≤ i)
    (hin : i ≤ Fintype.card α) (w : ℝ) (h : ∀ T, Correct P i T → w ≤ T.leaves) :
    w ≤ 2 ^ V P i := by
  obtain ⟨⟨T, hT, hd⟩, -⟩ := V_eq_min_depth P hi hin
  have h1 := h T hT
  have h2 : (T.leaves : ℝ) ≤ 2 ^ T.depth := by exact_mod_cast T.leaves_le_two_pow_depth
  rw [hd] at h2
  linarith

/-- Every member of the family equals `1` at a solved state. -/
theorem Psi_eq_one_of_solved (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0)
    {Q : SPO α} {i : ℕ} {e : α} (hi : 1 ≤ i) (hs : Solved Q i e) : Psi F G Q i = 1 := by
  classical
  obtain ⟨-, -, hC⟩ := counts_of_solved hi hs
  -- i ≤ n: the i - 1 predecessors of e do not include e
  have hin : i ≤ Fintype.card α := by
    have h1 : (univ.filter fun u => Q.lt u e) ⊆ univ.erase e := by
      intro u hu
      simp only [Finset.mem_filter, Finset.mem_univ, true_and] at hu
      exact Finset.mem_erase.2 ⟨Q.ne_of_lt hu, mem_univ u⟩
    have h2 := Finset.card_le_card h1
    rw [Finset.card_erase_of_mem (mem_univ e), Finset.card_univ] at h2
    have h3 := hs.2
    have h4 : 0 < Fintype.card α := Fintype.card_pos_iff.2 ⟨e⟩
    omega
  have hb : ∀ L ∈ Down Q (i - 1), bOf Q L = 0 := by
    intro L hL
    have := Finset.single_le_sum (f := fun L => bOf Q L + 1) (fun _ _ => Nat.zero_le _) hL
    rw [← card_C_eq_sum_b Q hi hin, hC] at this
    omega
  have ha : ∀ A ∈ Down Q i, aOf Q A = 0 := by
    intro A hA
    have := Finset.single_le_sum (f := fun A => aOf Q A + 1) (fun _ _ => Nat.zero_le _) hA
    rw [← card_C_eq_sum_a Q hi, hC] at this
    omega
  unfold Psi
  rw [hC, Finset.sum_eq_zero fun L hL => by rw [hb L hL, hF0],
    Finset.sum_eq_zero fun A hA => by rw [ha A hA, hG0]]
  norm_num

/-- A member of the family satisfying the local inequality bounds the leaves
of every correct tree. -/
theorem Psi_le_leaves_of_local (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0) {i : ℕ}
    (hi : 1 ≤ i)
    (hloc : ∀ (Q : SPO α) (x y : α), x ≠ y → ¬ Q.lt x y → ¬ Q.lt y x →
      Psi F G Q i ≤ Psi F G (Q.add x y) i + Psi F G (Q.add y x) i)
    (P : SPO α) (T : CTree α) (hT : Correct P i T) : Psi F G P i ≤ T.leaves :=
  le_leaves_of_local (fun Q => Psi F G Q i) hloc
    (fun _ _ hs => (Psi_eq_one_of_solved F G hF0 hG0 hi hs).le) T P hT

/-- **main.tex, after prop:phiopt:** the conclusion of the proposition also holds
for members that merely BOUND THE LEAVES of every correct tree for the
minimum and the maximum of every antichain ... -/
theorem phiopt_of_le_leaves (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0)
    (hmin : ∀ n : ℕ, 1 ≤ n → ∀ T, Correct (antichain (Fin n)) 1 T →
      Psi F G (antichain (Fin n)) 1 ≤ T.leaves)
    (hmax : ∀ n : ℕ, 1 ≤ n → ∀ T, Correct (antichain (Fin n)) n T →
      Psi F G (antichain (Fin n)) n ≤ T.leaves) :
    (∀ t, F t ≤ 2 ^ t - t - 1) ∧ (∀ t, G t ≤ 2 ^ t - t - 1) ∧
    (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
      1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ)) :=
  phiopt F G hF0 hG0
    (fun n hn => le_two_pow_V_of_le_leaves _ le_rfl (by simpa using hn) _ (hmin n hn))
    (fun n hn => le_two_pow_V_of_le_leaves _ (by omega) (by simp) _ (hmax n hn))

/-- ... and for members that SATISFY THE LOCAL INEQUALITY (on all orders of
`n` labels, at the ranks `1` and `n`). -/
theorem phiopt_of_local (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0)
    (hloc : ∀ n : ℕ, 1 ≤ n → ∀ i ∈ ({1, n} : Finset ℕ), ∀ (Q : SPO (Fin n)) (x y : Fin n),
      x ≠ y → ¬ Q.lt x y → ¬ Q.lt y x →
        Psi F G Q i ≤ Psi F G (Q.add x y) i + Psi F G (Q.add y x) i) :
    (∀ t, F t ≤ 2 ^ t - t - 1) ∧ (∀ t, G t ≤ 2 ^ t - t - 1) ∧
    (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
      1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ)) :=
  phiopt_of_le_leaves F G hF0 hG0
    (fun n hn T hT => Psi_le_leaves_of_local F G hF0 hG0 le_rfl
      (hloc n hn 1 (by simp)) _ T hT)
    (fun n hn T hT => Psi_le_leaves_of_local F G hF0 hG0 hn
      (hloc n hn n (by simp)) _ T hT)

end LB
