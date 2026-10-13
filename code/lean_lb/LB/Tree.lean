/-
  Obligations 6 and 7: Phi = 1 at every correct leaf, and the induction over
  an arbitrary comparison tree.

  * A correct leaf `e` for `(Q, i)` (every input consistent with `Q` has `e`
    at rank `i`) forces `Q` to be SOLVED at `e`: `e` is comparable with every
    other label and has exactly `i - 1` known predecessors (the paper's
    argument: place `e` directly after its predecessors in one consistent
    order and directly before its successors in another).
  * At a solved order, `FG = FGdual = |C| = 1`, so `Phi = 1`.
  * Tree induction.  At a node comparing `x` and `y` there are three cases:
      known `x < y`   every consistent input takes the first branch;
      `x = y` or known `y < x`   every consistent input takes the second;
      incomparable   the inputs reaching the branches are exactly those
                     consistent with `P + xy` and `P + yx`; use `local_ineq`.
    So the trees need no restriction on the compared pairs.
-/
import LB.Local

set_option linter.unusedSectionVars false

namespace LB

open Finset

variable {α : Type*} [Fintype α] [DecidableEq α]

/-! ### Ranks in an input -/

/-- In an input, at most one label has a given number of smaller values. -/
lemma eq_of_card_below_eq {f : α → ℕ} (hf : Function.Injective f) {e e' : α}
    (h : (univ.filter fun u => f u < f e).card = (univ.filter fun u => f u < f e').card) :
    e = e' := by
  by_contra hne
  have key : ∀ a b : α, f a < f b →
      (univ.filter fun u => f u < f a).card < (univ.filter fun u => f u < f b).card := by
    intro a b hab
    apply Finset.card_lt_card
    rw [Finset.ssubset_iff_of_subset]
    · exact ⟨a, by simp [hab], by simp⟩
    · intro u hu
      simp only [Finset.mem_filter, Finset.mem_univ, true_and] at hu ⊢
      omega
  rcases lt_or_gt_of_ne (hf.ne hne) with hlt | hlt
  · have := key _ _ hlt; omega
  · have := key _ _ hlt; omega

/-- The labels below `e` in an input that lists `L`, then `e`, then the rest. -/
lemma below_eq_of_realize {f : α → ℕ} {L : Finset α} {e : α} (h1 : ∀ l ∈ L, f l < f e)
    (h2 : ∀ u, u ∉ L → u ≠ e → f e < f u) : univ.filter (fun u => f u < f e) = L := by
  ext u
  simp only [Finset.mem_filter, Finset.mem_univ, true_and]
  constructor
  · intro hu
    by_contra huL
    by_cases hue : u = e
    · subst hue; exact lt_irrefl _ hu
    · have := h2 u huL hue; omega
  · exact h1 u

/-! ### Solved orders -/

open Classical in
/-- `P` is solved at `e` for rank `i`: `e` is comparable with every other label
and has exactly `i - 1` known predecessors. -/
def Solved (P : SPO α) (i : ℕ) (e : α) : Prop :=
  (∀ u, u ≠ e → P.lt u e ∨ P.lt e u) ∧ (univ.filter fun u => P.lt u e).card = i - 1

/-- **A correct leaf is a solved state.** -/
theorem solved_of_correct_leaf {P : SPO α} {i : ℕ} {e : α} (h : Correct P i (.leaf e)) :
    Solved P i e := by
  classical
  -- D = known predecessors of e;  L₂ = labels neither equal to nor above e
  set D := univ.filter fun u => P.lt u e with hD
  set L₂ := univ.filter fun u => u ≠ e ∧ ¬ P.lt e u with hL₂
  have memD : ∀ u, u ∈ D ↔ P.lt u e := fun u => by simp [hD]
  have memL₂ : ∀ u, u ∈ L₂ ↔ u ≠ e ∧ ¬ P.lt e u := fun u => by simp [hL₂]
  -- e directly after its predecessors
  have hDdown : IsDownSet P D := fun u v huv hv => (memD u).2 (P.trans huv ((memD v).1 hv))
  have heD : e ∈ minimals P Dᶜ := by
    refine mem_minimals.2 ⟨Finset.mem_compl.2 fun he => P.irrefl e ((memD e).1 he), ?_⟩
    intro u hu hue
    exact (Finset.mem_compl.1 hu) ((memD u).2 hue)
  obtain ⟨f₁, hf₁, h1₁, h2₁⟩ := realize hDdown heD
  have hcD : D.card = i - 1 := by
    have : (univ.filter fun u => f₁ u < f₁ e).card = i - 1 := h f₁ hf₁
    rwa [below_eq_of_realize h1₁ h2₁] at this
  -- e directly before its successors
  have hL₂down : IsDownSet P L₂ := by
    intro u v huv hv
    obtain ⟨hve, hev⟩ := (memL₂ v).1 hv
    refine (memL₂ u).2 ⟨?_, fun heu => hev (P.trans heu huv)⟩
    rintro rfl
    exact hev huv
  have heL₂ : e ∈ minimals P L₂ᶜ := by
    refine mem_minimals.2 ⟨Finset.mem_compl.2 fun he => ((memL₂ e).1 he).1 rfl, ?_⟩
    intro u hu hue
    have hu' : ¬ (u ≠ e ∧ ¬ P.lt e u) := fun h' => (Finset.mem_compl.1 hu) ((memL₂ u).2 h')
    by_cases hue' : u = e
    · subst hue'; exact P.irrefl u hue
    · have heu : P.lt e u := by
        by_contra hneg; exact hu' ⟨hue', hneg⟩
      exact P.irrefl e (P.trans heu hue)
  obtain ⟨f₂, hf₂, h1₂, h2₂⟩ := realize hL₂down heL₂
  have hcL₂ : L₂.card = i - 1 := by
    have : (univ.filter fun u => f₂ u < f₂ e).card = i - 1 := h f₂ hf₂
    rwa [below_eq_of_realize h1₂ h2₂] at this
  -- D ⊆ L₂ and equal sizes, so every label other than e is comparable with e
  have hsub : D ⊆ L₂ := by
    intro u hu
    have hue := (memD u).1 hu
    exact (memL₂ u).2 ⟨P.ne_of_lt hue, P.asymm hue⟩
  have heq : D = L₂ := Finset.eq_of_subset_of_card_le hsub (by omega)
  refine ⟨fun u hue => ?_, hcD⟩
  by_cases heu : P.lt e u
  · exact Or.inr heu
  · exact Or.inl ((memD u).1 (heq ▸ (memL₂ u).2 ⟨hue, heu⟩))

/-- **At a solved state `FG = FGdual = |C| = 1`.** -/
theorem counts_of_solved {P : SPO α} {i : ℕ} {e : α} (hi : 1 ≤ i) (hs : Solved P i e) :
    FG P i = 1 ∧ FGdual P i = 1 ∧ (C P i).card = 1 := by
  classical
  obtain ⟨hcomp, hcard⟩ := hs
  set D := univ.filter fun u => P.lt u e with hD
  have memD : ∀ u, u ∈ D ↔ P.lt u e := fun u => by simp [hD]
  have hDdown : IsDownSet P D := fun u v huv hv => (memD u).2 (P.trans huv ((memD v).1 hv))
  have heD : e ∉ D := fun he => P.irrefl e ((memD e).1 he)
  -- any down-set containing e contains insert e D
  have hins : ∀ S : Finset α, IsDownSet P S → e ∈ S → insert e D ⊆ S := by
    intro S hS heS u hu
    rw [Finset.mem_insert] at hu
    rcases hu with rfl | hu
    · exact heS
    · exact hS u e ((memD u).1 hu) heS
  have hcins : (insert e D).card = i := by
    rw [Finset.card_insert_of_notMem heD, hcard]; omega
  -- the unique lower set is D
  have hDown1 : Down P (i - 1) = {D} := by
    ext L
    rw [mem_Down, Finset.mem_singleton]
    constructor
    · rintro ⟨hL, hc⟩
      have heL : e ∉ L := by
        intro heL
        have := Finset.card_le_card (hins L hL heL)
        omega
      have hLD : L ⊆ D := by
        intro u hu
        have hue : u ≠ e := fun h => heL (h ▸ hu)
        rcases hcomp u hue with h | h
        · exact (memD u).2 h
        · exact absurd (hL e u h hu) heL
      exact Finset.eq_of_subset_of_card_le hLD (by omega)
    · rintro rfl; exact ⟨hDdown, hcard⟩
  -- the unique initial set is insert e D
  have hDown2 : Down P i = {insert e D} := by
    ext A
    rw [mem_Down, Finset.mem_singleton]
    constructor
    · rintro ⟨hA, hc⟩
      have heA : e ∈ A := by
        by_contra heA
        have hAD : A ⊆ D := by
          intro u hu
          have hue : u ≠ e := fun h => heA (h ▸ hu)
          rcases hcomp u hue with h | h
          · exact (memD u).2 h
          · exact absurd (hA e u h hu) heA
        have := Finset.card_le_card hAD
        omega
      exact (Finset.eq_of_subset_of_card_le (hins A hA heA) (by omega)).symm
    · rintro rfl
      refine ⟨?_, hcins⟩
      intro u v huv hv
      rw [Finset.mem_insert] at hv ⊢
      rcases hv with rfl | hv
      · exact Or.inr ((memD u).2 huv)
      · exact Or.inr ((memD u).2 (P.trans huv ((memD v).1 hv)))
  -- the candidates
  have hmin : minimals P Dᶜ = {e} := by
    ext v
    rw [mem_minimals, Finset.mem_singleton]
    constructor
    · rintro ⟨hv, hvmin⟩
      by_contra hve
      rcases hcomp v hve with h | h
      · exact (Finset.mem_compl.1 hv) ((memD v).2 h)
      · exact hvmin e (Finset.mem_compl.2 heD) h
    · rintro rfl
      exact ⟨Finset.mem_compl.2 heD, fun u hu hue => (Finset.mem_compl.1 hu) ((memD u).2 hue)⟩
  have hmax : maximals P (insert e D) = {e} := by
    ext v
    rw [mem_maximals, Finset.mem_singleton]
    constructor
    · rintro ⟨hv, hvmax⟩
      rw [Finset.mem_insert] at hv
      rcases hv with rfl | hv
      · rfl
      · exact absurd ((memD v).1 hv) (hvmax e (Finset.mem_insert_self _ _))
    · rintro rfl
      refine ⟨Finset.mem_insert_self _ _, fun u hu hvu => ?_⟩
      rw [Finset.mem_insert] at hu
      rcases hu with rfl | hu
      · exact P.irrefl _ hvu
      · exact P.irrefl _ (P.trans hvu ((memD u).1 hu))
  refine ⟨?_, ?_, ?_⟩
  · simp [FG, hDown1, bOf, hmin]
  · simp [FGdual, hDown2, aOf, hmax]
  · have hC : C P i = {(D, e)} := by
      ext s
      rw [mem_C, hDown1, Finset.mem_singleton, Finset.mem_singleton]
      constructor
      · rintro ⟨hs1, hs2⟩
        rw [hs1, hmin, Finset.mem_singleton] at hs2
        exact Prod.ext hs1 hs2
      · rintro rfl
        exact ⟨rfl, by rw [hmin]; exact Finset.mem_singleton_self e⟩
    rw [hC, Finset.card_singleton]

theorem Phi_eq_one_of_solved {P : SPO α} {i : ℕ} {e : α} (hi : 1 ≤ i) (hs : Solved P i e) :
    Phi P i = 1 := by
  obtain ⟨h1, h2, h3⟩ := counts_of_solved hi hs
  unfold Phi; rw [h1, h2, h3]; norm_num

/-- **Obligation 6: `Phi = 1` at every correct leaf.** -/
theorem Phi_eq_one_of_correct_leaf {P : SPO α} {i : ℕ} {e : α} (hi : 1 ≤ i)
    (h : Correct P i (.leaf e)) : Phi P i = 1 :=
  Phi_eq_one_of_solved hi (solved_of_correct_leaf h)

/-! ### Correctness of the subtrees -/

section Children

variable {P : SPO α} {i : ℕ} {x y : α} {t₁ t₂ : CTree α}

lemma correct_lt_of_lt (hT : Correct P i (.node x y t₁ t₂)) (hxy : P.lt x y) :
    Correct P i t₁ := by
  intro f hf
  have := hT f hf
  simp only [CTree.eval, ite_eq_left (hf.2 x y hxy)] at this
  exact this

lemma correct_ge_of_le (hT : Correct P i (.node x y t₁ t₂)) (hyx : P.le y x) :
    Correct P i t₂ := by
  intro f hf
  have := hT f hf
  have hn : ¬ f x < f y := by
    rcases hyx with rfl | hyx
    · exact lt_irrefl _
    · have := hf.2 y x hyx; omega
  simp only [CTree.eval, ite_eq_right hn] at this
  exact this

lemma correct_add_left (hT : Correct P i (.node x y t₁ t₂)) (h : ¬ P.le y x) :
    Correct (P.add x y) i t₁ := by
  intro f hf
  have := hT f hf.of_add
  simp only [CTree.eval, ite_eq_left (hf.lt_of_add h)] at this
  exact this

lemma correct_add_right (hT : Correct P i (.node x y t₁ t₂)) (h : ¬ P.le x y) :
    Correct (P.add y x) i t₂ := by
  intro f hf
  have := hT f hf.of_add
  have hn : ¬ f x < f y := by have := hf.lt_of_add h; omega
  simp only [CTree.eval, ite_eq_right hn] at this
  exact this

end Children

/-! ### The tree induction (obligation 7) -/

/-- **Theorem `thm:phi`, leaf form.**  Every correct decision tree for `(P, i)`
has at least `Phi*(P)` leaves. -/
theorem Phi_le_leaves (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) : Phi P i ≤ T.leaves := by
  induction T generalizing P with
  | leaf e =>
    rw [Phi_eq_one_of_correct_leaf hi hT]; simp [CTree.leaves]
  | node x y t₁ t₂ ih₁ ih₂ =>
    simp only [CTree.leaves, Nat.cast_add]
    have h₁ : (0 : ℤ) ≤ t₁.leaves := Nat.cast_nonneg _
    have h₂ : (0 : ℤ) ≤ t₂.leaves := Nat.cast_nonneg _
    by_cases hxy : P.lt x y
    · have := ih₁ P (correct_lt_of_lt hT hxy); omega
    · by_cases hyx : P.le y x
      · have := ih₂ P (correct_ge_of_le hT hyx); omega
      · have hne : x ≠ y := fun h => hyx (Or.inl h.symm)
        have hyx' : ¬ P.lt y x := fun h => hyx (Or.inr h)
        have hxy' : ¬ P.le x y := not_le_of_incomp hne.symm hxy
        have e₁ := ih₁ (P.add x y) (correct_add_left hT hyx)
        have e₂ := ih₂ (P.add y x) (correct_add_right hT hxy')
        have := local_ineq hi hin hne hxy hyx'
        omega

/-- **Theorem `thm:phi`, depth form.**  If a correct tree for `(P, i)` makes at
most `d` comparisons on every input consistent with `P`, then `Phi*(P) ≤ 2^d`.
(Comparisons are counted along the paths actually taken by consistent inputs.) -/
theorem Phi_le_two_pow_of_cost (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) (d : ℕ)
    (hd : ∀ f, IsLinExt P f → T.pathLen f ≤ d) : Phi P i ≤ 2 ^ d := by
  induction T generalizing P d with
  | leaf e =>
    rw [Phi_eq_one_of_correct_leaf hi hT]
    exact_mod_cast Nat.one_le_two_pow
  | node x y t₁ t₂ ih₁ ih₂ =>
    -- some consistent input exists and makes at least one comparison, so `d ≥ 1`
    obtain ⟨f₀, hf₀⟩ := exists_linExt P
    have hd0 := hd f₀ hf₀
    simp only [CTree.pathLen] at hd0
    have hd1 : 1 ≤ d := by omega
    have hpow : (2 : ℤ) ^ d = 2 ^ (d - 1) + 2 ^ (d - 1) := by
      rw [← two_mul, ← pow_succ']; congr 1; omega
    have hpos : (0 : ℤ) ≤ 2 ^ (d - 1) := pow_nonneg (by norm_num) _
    by_cases hxy : P.lt x y
    · have := ih₁ P (correct_lt_of_lt hT hxy) (d - 1) fun f hf => by
        have := hd f hf
        simp only [CTree.pathLen, ite_eq_left (hf.2 x y hxy)] at this
        omega
      omega
    · by_cases hyx : P.le y x
      · have := ih₂ P (correct_ge_of_le hT hyx) (d - 1) fun f hf => by
          have := hd f hf
          have hn : ¬ f x < f y := by
            rcases hyx with rfl | hyx
            · exact lt_irrefl _
            · have := hf.2 y x hyx; omega
          simp only [CTree.pathLen, ite_eq_right hn] at this
          omega
        omega
      · have hne : x ≠ y := fun h => hyx (Or.inl h.symm)
        have hyx' : ¬ P.lt y x := fun h => hyx (Or.inr h)
        have hxy' : ¬ P.le x y := not_le_of_incomp hne.symm hxy
        have e₁ := ih₁ (P.add x y) (correct_add_left hT hyx) (d - 1) fun f hf => by
          have := hd f hf.of_add
          simp only [CTree.pathLen, ite_eq_left (hf.lt_of_add hyx)] at this
          omega
        have e₂ := ih₂ (P.add y x) (correct_add_right hT hxy') (d - 1) fun f hf => by
          have := hd f hf.of_add
          have hn : ¬ f x < f y := by have := hf.lt_of_add hxy'; omega
          simp only [CTree.pathLen, ite_eq_right hn] at this
          omega
        have := local_ineq hi hin hne hxy hyx'
        omega

lemma CTree.pathLen_le_depth (T : CTree α) (f : α → ℕ) : T.pathLen f ≤ T.depth := by
  induction T with
  | leaf e => simp [CTree.pathLen, CTree.depth]
  | node x y t₁ t₂ ih₁ ih₂ =>
    simp only [CTree.pathLen, CTree.depth]
    split_ifs <;> omega

/-- "A binary tree of depth `d` has at most `2^d` leaves." -/
lemma CTree.leaves_le_two_pow_depth (T : CTree α) : T.leaves ≤ 2 ^ T.depth := by
  induction T with
  | leaf e => simp [CTree.leaves, CTree.depth]
  | node x y t₁ t₂ ih₁ ih₂ =>
    simp only [CTree.leaves, CTree.depth]
    have h₁ : 2 ^ t₁.depth ≤ 2 ^ max t₁.depth t₂.depth :=
      Nat.pow_le_pow_right (by norm_num) (le_max_left _ _)
    have h₂ : 2 ^ t₂.depth ≤ 2 ^ max t₁.depth t₂.depth :=
      Nat.pow_le_pow_right (by norm_num) (le_max_right _ _)
    rw [Nat.add_comm 1, Nat.pow_succ]
    omega

/-- `Phi*(P) ≤ 2^{depth}` for every correct tree. -/
theorem Phi_le_two_pow_depth (P : SPO α) {i : ℕ} (hi : 1 ≤ i) (hin : i ≤ Fintype.card α)
    (T : CTree α) (hT : Correct P i T) : Phi P i ≤ 2 ^ T.depth :=
  Phi_le_two_pow_of_cost P hi hin T hT T.depth fun f _ => T.pathLen_le_depth f

end LB
