/-
  FINAL THEOREMS of the Section 5 formalization (short.tex sec:lb).

  Read together with LB/Spec.lean.  Definitions used here that are NOT in
  Spec.lean (they occur only in intermediate statements or as witnesses):
    SPO.le       (Basic.lean)    reflexive closure: u = v ∨ P.lt u v;
    SPO.add      (Closure.lean)  the paper's P + xy, proved equal to the
                                 transitive closure of P ∪ {(x, y)}
                                 (add_is_closure); its "return P" branch for
                                 a contradictory outcome is never used here:
                                 every statement assumes x, y incomparable;
    psiL, psiA   (World.lean)    the per-set weights ψ_L(Q) of lem:world;
    prune        (Family.lean)   deletes comparisons with known outcomes;
    CTree.scanMin (Optimality.lean)  the linear scan (a witness only).
  Each statement is restated with all binders explicit and proved by the
  module that carries the proof.

  Conventions: `α` is any finite type of labels, `n = Fintype.card α`, ranks
  `i` with `1 ≤ i ≤ n` (the i-th smallest).  No other hypotheses.  Where a
  statement has no rank hypothesis it holds for every `i`. Outside the valid
  range the definitions retain Lean's total-function conventions: `Correct`
  and `V` at rank 0 agree with rank 1, but the weights need not. The paper's
  lower-bound claims concern only valid ranks.
  Universes: everything is universe-polymorphic in `α` except the conclusion
  of prop_phiopt, which quantifies over `β : Type`; every finite order is
  isomorphic to one on `Fin n : Type`, so this loses nothing.
-/
import LB.Examples
import LB.Family

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ## Theorem `thm:phi` (the combined weight) -/

/-- Every correct comparison decision tree for `(P, i)` has at least `Phi*(P)`
leaves.  (`T.leaves` counts all leaves, as the paper's statement does; the
pruned tree of `pruning` has no more leaves and is also correct, so the bound
holds for it too.) -/
theorem thm_phi_leaves (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) : Phi P i ≤ (T.leaves : ℤ) :=
  Phi_le_leaves P hi hin T hT

/-- If a correct tree for `(P, i)` makes at most `d` comparisons on every input
consistent with `P`, then `Phi*(P) ≤ 2^d`. -/
theorem thm_phi_cost (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) (d : ℕ)
    (hd : ∀ f : α → ℕ, IsLinExt P f → T.pathLen f ≤ d) : Phi P i ≤ 2 ^ d :=
  Phi_le_two_pow_of_cost P hi hin T hT d hd

/-- `Phi*(P) ≤ 2^{depth}` for every correct tree. -/
theorem thm_phi_depth (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) : Phi P i ≤ 2 ^ T.depth :=
  Phi_le_two_pow_depth P hi hin T hT

/-- `V_i(P) ≥ ⌈log₂ Phi*(P)⌉`, in both forms; `Phi*(P)` is a positive integer. -/
theorem thm_phi_V (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    1 ≤ Phi P i ∧ Phi P i ≤ 2 ^ V P i ∧ Nat.clog 2 (Phi P i).toNat ≤ V P i :=
  ⟨one_le_Phi P hi hin, Phi_le_two_pow_V P hi hin, clog_Phi_le_V P hi hin⟩

/-! ## The proof obligations of `thm:phi` as separate statements -/

/-- Obligation 2: the two identities counting compatible solutions. -/
theorem count_identities (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (C P i).card = ∑ L ∈ Down P (i - 1), (bOf P L + 1) ∧
    (C P i).card = ∑ A ∈ Down P i, (aOf P A + 1) :=
  ⟨card_C_eq_sum_b P hi hin, card_C_eq_sum_a P hi⟩

/-- `P + xy` is the transitive closure of `P` with the new relation `x < y`. -/
theorem add_is_closure (P : SPO α) (x y : α) (h : ¬ P.le y x) (u v : α) :
    (P.add x y).lt u v ↔
      Relation.TransGen (fun a b => P.lt a b ∨ (a = x ∧ b = y)) u v :=
  P.add_lt_iff_transGen h

/-- Obligation 3: Lemma `lem:world` for lower sets.  Stated for EVERY down-set
`L` (the paper: `L ∈ Down_{i-1}(P)`).  For the degenerate `L = univ` the
"minus one" in `bOf` truncates, but the statement remains true; on the
paper's sets (`i ≤ n`) it never truncates (`bOf_add_one`). -/
theorem lem_world_L (P : SPO α) (x y : α) (hne : x ≠ y) (hxy : ¬ P.lt x y)
    (hyx : ¬ P.lt y x) (L : Finset α) (hL : IsDownSet P L) :
    (x ∈ L → y ∈ L → psiL (P.add x y) L = psiL P L ∧ psiL (P.add y x) L = psiL P L) ∧
    (x ∈ L → y ∉ L → psiL (P.add x y) L = psiL P L ∧ psiL (P.add y x) L = 0) ∧
    (x ∉ L → y ∈ L → psiL (P.add x y) L = 0 ∧ psiL (P.add y x) L = psiL P L) ∧
    (x ∉ L → y ∉ L → psiL P L ≤ psiL (P.add x y) L + psiL (P.add y x) L) :=
  world_L hne hxy hyx hL

/-- Obligation 3: Lemma `lem:world` for initial sets (every down-set `A`; the
paper: `A ∈ Down_i(P)`, nonempty for `i ≥ 1`, where `aOf` never truncates). -/
theorem lem_world_A (P : SPO α) (x y : α) (hne : x ≠ y) (hxy : ¬ P.lt x y)
    (hyx : ¬ P.lt y x) (A : Finset α) (hA : IsDownSet P A) :
    (x ∉ A → y ∉ A → psiA (P.add x y) A = psiA P A ∧ psiA (P.add y x) A = psiA P A) ∧
    (x ∈ A → y ∉ A → psiA (P.add x y) A = psiA P A ∧ psiA (P.add y x) A = 0) ∧
    (x ∉ A → y ∈ A → psiA (P.add x y) A = 0 ∧ psiA (P.add y x) A = psiA P A) ∧
    (x ∈ A → y ∈ A → psiA P A ≤ psiA (P.add x y) A + psiA (P.add y x) A) :=
  world_A hne hxy hyx hA

/-- Obligation 4: which solutions survive, and the exact duplicate count. -/
theorem duplicate_accounting (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (x y : α) (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    C (P.add x y) i ∪ C (P.add y x) i = C P i ∧
    (∀ s, (s ∈ C (P.add x y) i ∧ s ∈ C (P.add y x) i) ↔
      s ∈ C P i ∧ ((x ∈ s.1 ∧ y ∈ s.1) ∨ (x ∉ insert s.2 s.1 ∧ y ∉ insert s.2 s.1))) ∧
    (C (P.add x y) i).card + (C (P.add y x) i).card =
      (C P i).card + ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L), (bOf P L + 1)
        + ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), (aOf P A + 1) :=
  ⟨C_union_eq hne, fun _ => mem_C_add_both hne hxy hyx, card_C_add hi hin hne hxy hyx⟩

/-- Obligation 5: the local inequality, and the paper's lower estimate of `ΔPhi*`. -/
theorem local_inequality (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (x y : α) (hne : x ≠ y) (hxy : ¬ P.lt x y) (hyx : ¬ P.lt y x) :
    Phi P i ≤ Phi (P.add x y) i + Phi (P.add y x) i ∧
    ∑ L ∈ (Down P (i - 1)).filter (fun L => x ∈ L ∧ y ∈ L),
        ((2 : ℤ) ^ bOf P L - bOf P L - 1) +
      ∑ A ∈ (Down P i).filter (fun A => x ∉ A ∧ y ∉ A), ((2 : ℤ) ^ aOf P A - aOf P A - 1) ≤
    Phi (P.add x y) i + Phi (P.add y x) i - Phi P i :=
  ⟨local_ineq hi hin hne hxy hyx, delta_Phi_ge hi hin hne hxy hyx⟩

open Classical in
/-- Obligation 6: at a correct leaf the problem is solved (`e` is comparable
with every other label and has exactly `i - 1` known predecessors) and
`FG = FGdual = |C| = Phi* = 1`.  Stated for a leaf that is correct for the
state `P` it is reached in; `children_correct` shows that subtrees of a
correct tree are correct for the child states `P + xy`, `P + yx`. -/
theorem correct_leaf (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (e : α)
    (h : Correct P i (.leaf e)) :
    (∀ u, u ≠ e → P.lt u e ∨ P.lt e u) ∧ (univ.filter fun u => P.lt u e).card = i - 1 ∧
    FG P i = 1 ∧ FGdual P i = 1 ∧ (C P i).card = 1 ∧ Phi P i = 1 := by
  have hs := solved_of_correct_leaf h
  obtain ⟨h1, h2, h3⟩ := counts_of_solved hi hs
  exact ⟨hs.1, hs.2, h1, h2, h3, Phi_eq_one_of_solved hi hs⟩

/-- Obligation 7 (the step of the tree induction): the subtrees of a correct
tree are correct for the states in which they are reached. -/
theorem children_correct (P : SPO α) (i : ℕ) (x y : α) (t₁ t₂ : CTree α)
    (hT : Correct P i (.node x y t₁ t₂)) :
    (P.lt x y → Correct P i t₁) ∧ (P.le y x → Correct P i t₂) ∧
    (¬ P.le y x → Correct (P.add x y) i t₁) ∧ (¬ P.le x y → Correct (P.add y x) i t₂) :=
  ⟨correct_lt_of_lt hT, correct_ge_of_le hT, correct_add_left hT, correct_add_right hT⟩

/-- Deleting the comparisons with known outcomes keeps a tree correct, does not
increase the number of leaves ("removing infeasible paths only decreases the
count"), and makes the depth at most the worst-case number of comparisons. -/
theorem pruning (P : SPO α) (i : ℕ) (T : CTree α) (hT : Correct P i T) :
    Correct P i (prune T P) ∧ (prune T P).leaves ≤ T.leaves ∧
    ∀ d : ℕ, (∀ f : α → ℕ, IsLinExt P f → T.pathLen f ≤ d) → (prune T P).depth ≤ d :=
  ⟨correct_prune hT, leaves_prune_le T P, depth_prune_le T P⟩

/-- `V_i(P)`, defined as the optimal worst-case number of comparisons over
consistent inputs, is also the least depth of a correct tree. -/
theorem V_is_min_depth (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (∃ T : CTree α, Correct P i T ∧ T.depth = V P i) ∧
    ∀ T : CTree α, Correct P i T → V P i ≤ T.depth :=
  V_eq_min_depth P hi hin

/-! ## Theorem `thm:key` -/

/-- Along the path of any input that realizes a compatible solution
`s = (L, e)` (lists `L`, then `e`, then the rest), a correct tree makes at
least `a_s + b_s` comparisons. -/
theorem thm_key_path (P : SPO α) (i : ℕ) (T : CTree α) (hT : Correct P i T)
    (L : Finset α) (e : α) (hs : (L, e) ∈ C P i) (f : α → ℕ) (hf : IsLinExt P f)
    (h1 : ∀ l ∈ L, f l < f e) (h2 : ∀ u, u ∉ L → u ≠ e → f e < f u) :
    aOf P (insert e L) + bOf P L ≤ T.pathLen f :=
  key_le_pathLen hT hs hf h1 h2

/-- `V_i(P) ≥ KEY(P)`: every correct tree makes at least `KEY(P)` comparisons
on some consistent input. -/
theorem thm_key (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    (∀ (T : CTree α), Correct P i T → ∀ d : ℕ,
      (∀ f : α → ℕ, IsLinExt P f → T.pathLen f ≤ d) → KEY P i ≤ d) ∧
    KEY P i ≤ V P i :=
  ⟨fun _ hT d hd => KEY_le_of_cost hT d hd, KEY_le_V P hi hin⟩

/-! ## Corollary `cor:combined` -/

theorem cor_combined (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    max (Nat.clog 2 (Phi P i).toNat) (KEY P i) ≤ V P i ∧
    (C P i).card ≤ FG P i ∧ (C P i).card ≤ FGdual P i ∧
    (FG P i : ℤ) ≤ Phi P i ∧ (FGdual P i : ℤ) ≤ Phi P i ∧
    Nat.clog 2 (C P i).card ≤ Nat.clog 2 (Phi P i).toNat ∧
    Nat.clog 2 (FG P i) ≤ Nat.clog 2 (Phi P i).toNat ∧
    Nat.clog 2 (FGdual P i) ≤ Nat.clog 2 (Phi P i).toNat :=
  ⟨LB_le_V P hi hin, card_C_le_FG P hi hin, card_C_le_FGdual P hi, FG_le_Phi P hi,
    FGdual_le_Phi P hi hin, (clog_bounds_le_clog_Phi P hi hin).1,
    (clog_bounds_le_clog_Phi P hi hin).2.1, (clog_bounds_le_clog_Phi P hi hin).2.2⟩

/-! ## Proposition `prop:phiopt` -/

theorem prop_phiopt (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0)
    (hmin : ∀ n : ℕ, 1 ≤ n →
      Psi F G (antichain (Fin n)) 1 ≤ 2 ^ V (antichain (Fin n)) 1)
    (hmax : ∀ n : ℕ, 1 ≤ n →
      Psi F G (antichain (Fin n)) n ≤ 2 ^ V (antichain (Fin n)) n) :
    (∀ t : ℕ, F t ≤ 2 ^ t - t - 1) ∧ (∀ t : ℕ, G t ≤ 2 ^ t - t - 1) ∧
    (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
      1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ)) :=
  phiopt F G hF0 hG0 hmin hmax

/-- main.tex: "validity is weaker than bounding the number of leaves of every
correct tree, and the latter is weaker than satisfying the local inequality":
a member with `F(0) = G(0) = 0` satisfying the local inequality bounds the
leaves, and any weight bounding the leaves is valid. -/
theorem family_chain (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0) (i : ℕ) (hi : 1 ≤ i)
    (hin : i ≤ Fintype.card α) :
    ((∀ (Q : SPO α) (x y : α), x ≠ y → ¬ Q.lt x y → ¬ Q.lt y x →
        Psi F G Q i ≤ Psi F G (Q.add x y) i + Psi F G (Q.add y x) i) →
      ∀ (P : SPO α) (T : CTree α), Correct P i T → Psi F G P i ≤ T.leaves) ∧
    (∀ P : SPO α, (∀ T : CTree α, Correct P i T → Psi F G P i ≤ T.leaves) →
      Psi F G P i ≤ 2 ^ V P i) :=
  ⟨fun hloc P T hT => Psi_le_leaves_of_local F G hF0 hG0 hi hloc P T hT,
    fun P h => le_two_pow_V_of_le_leaves P hi hin _ h⟩

/-- main.tex: "Thus Phi* is pointwise largest among valid formulas in this
family, including those that bound leaves or satisfy the local inequality":
the conclusion of prop:phiopt under the two stronger hypotheses. -/
theorem prop_phiopt_stronger (F G : ℕ → ℝ) (hF0 : F 0 = 0) (hG0 : G 0 = 0) :
    ((∀ n : ℕ, 1 ≤ n → ∀ T, Correct (antichain (Fin n)) 1 T →
        Psi F G (antichain (Fin n)) 1 ≤ T.leaves) →
      (∀ n : ℕ, 1 ≤ n → ∀ T, Correct (antichain (Fin n)) n T →
        Psi F G (antichain (Fin n)) n ≤ T.leaves) →
      (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
        1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ))) ∧
    ((∀ n : ℕ, 1 ≤ n → ∀ i ∈ ({1, n} : Finset ℕ), ∀ (Q : SPO (Fin n)) (x y : Fin n),
        x ≠ y → ¬ Q.lt x y → ¬ Q.lt y x →
          Psi F G Q i ≤ Psi F G (Q.add x y) i + Psi F G (Q.add y x) i) →
      (∀ (β : Type) [Fintype β] [DecidableEq β] (P : SPO β) (i : ℕ),
        1 ≤ i → i ≤ Fintype.card β → Psi F G P i ≤ (Phi P i : ℝ))) :=
  ⟨fun h1 h2 => (phiopt_of_le_leaves F G hF0 hG0 h1 h2).2.2,
    fun h => (phiopt_of_local F G hF0 hG0 h).2.2⟩

/-- `Phi*` itself belongs to the family (`F = G = 2^t - t - 1`) and is valid. -/
theorem Phi_in_family (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    Psi (fun t => 2 ^ t - t - 1) (fun t => 2 ^ t - t - 1) P i = (Phi P i : ℝ) ∧
    Phi P i ≤ 2 ^ V P i :=
  ⟨Psi_two_pow_eq_Phi P hi hin, Phi_le_two_pow_V P hi hin⟩

/-! ## Non-vacuity: correct trees exist, `V` is attained, and the bound is sharp -/

theorem V_attained (P : SPO α) (i : ℕ) (hi : 1 ≤ i) (hin : i ≤ Fintype.card α) :
    ∃ T : CTree α, Correct P i T ∧ ∀ f : α → ℕ, IsLinExt P f → T.pathLen f ≤ V P i :=
  V_spec P hi hin

/-- For the minimum of `n ≥ 1` unordered labels the linear scan is correct and has
exactly `Phi*` leaves, and `V_1(E_n) = n - 1`. -/
theorem sharp_for_minimum (hn : 1 ≤ Fintype.card α) (m : α) :
    Correct (antichain α) 1 (CTree.scanMin m (univ.erase m).toList) ∧
    ((CTree.scanMin m (univ.erase m).toList).leaves : ℤ) = Phi (antichain α) 1 ∧
    V (antichain α) 1 = Fintype.card α - 1 :=
  ⟨(scanMin_tight hn m).1, (scanMin_tight hn m).2, V_antichain_min hn⟩

end LB
