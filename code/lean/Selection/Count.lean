import Selection.Spec

/-! Counting lemmas about ranks in lists of naturals (not trusted: checked by Lean).
    `l.countP (· < v)` is the number of elements of `l` below `v`. -/

namespace Sel

/-- Counting over a list = counting over its positions. -/
theorem countP_range_getD (xs : List Nat) (p : Nat → Bool) :
    (List.range xs.length).countP (fun i => p (xs.getD i 0)) = xs.countP p := by
  induction xs with
  | nil => simp
  | cons a l ih =>
    rw [List.length_cons, List.range_succ_eq_map, List.countP_cons, List.countP_map]
    simp only [Function.comp_def, List.getD_cons_succ, List.getD_cons_zero]
    rw [ih, List.countP_cons]

theorem count_lt_add_gt_of_ne (l : List Nat) (v : Nat) (h : ∀ z ∈ l, z ≠ v) :
    l.countP (· < v) + l.countP (v < ·) = l.length := by
  induction l with
  | nil => simp
  | cons a l ih =>
    have ha := h a (by simp)
    have := ih (fun z hz => h z (by simp [hz]))
    simp only [List.countP_cons, List.length_cons, decide_eq_true_eq]
    split <;> split <;> omega

/-- For distinct elements: #below v + #above v + 1 = length. -/
theorem count_lt_add_gt (l : List Nat) (v : Nat) (hnd : l.Nodup) (hv : v ∈ l) :
    l.countP (· < v) + l.countP (v < ·) + 1 = l.length := by
  induction l with
  | nil => simp at hv
  | cons a l ih =>
    rw [List.nodup_cons] at hnd
    simp only [List.countP_cons, List.length_cons, decide_eq_true_eq]
    by_cases hav : a = v
    · subst hav
      have := count_lt_add_gt_of_ne l a (fun z hz h => hnd.1 (h ▸ hz))
      simp; omega
    · have hv' : v ∈ l := by
        rcases List.mem_cons.mp hv with h | h
        · exact absurd h.symm hav
        · exact h
      have := ih hnd.2 hv'
      split <;> split <;> omega

theorem count_lt_mono (l : List Nat) {a b : Nat} (h : a ≤ b) :
    l.countP (· < a) ≤ l.countP (· < b) :=
  List.countP_mono_left (fun x _ hx => by simp at hx ⊢; omega)

theorem count_lt_strict (l : List Nat) {a b : Nat} (ha : a ∈ l) (h : a < b) :
    l.countP (· < a) < l.countP (· < b) := by
  induction l with
  | nil => simp at ha
  | cons x l ih =>
    simp only [List.countP_cons, decide_eq_true_eq]
    rcases List.mem_cons.mp ha with hx | hx
    · subst hx
      have := count_lt_mono l (Nat.le_of_lt h)
      simp [h]; omega
    · have := ih hx
      split <;> split <;> omega

/-- In a list of distinct naturals every rank r < length is attained. -/
theorem exists_rank (l : List Nat) (hnd : l.Nodup) :
    ∀ r, r < l.length → ∃ z ∈ l, l.countP (· < z) = r := by
  induction l with
  | nil => intro r h; simp at h
  | cons a l ih =>
    intro r hr
    rw [List.nodup_cons] at hnd
    simp only [List.length_cons] at hr
    have hs : l.countP (· < a) ≤ l.length := List.countP_le_length
    by_cases h1 : r < l.countP (· < a)
    · obtain ⟨z, hz, hzr⟩ := ih hnd.2 r (by omega)
      have hza : z < a := by
        apply Nat.lt_of_not_le; intro hle
        have := count_lt_mono l hle; omega
      refine ⟨z, List.mem_cons_of_mem _ hz, ?_⟩
      simp only [List.countP_cons, decide_eq_true_eq]
      have : ¬ a < z := by omega
      simp [this, hzr]
    · by_cases h2 : r = l.countP (· < a)
      · refine ⟨a, List.mem_cons_self, ?_⟩
        simp [h2]
      · obtain ⟨z, hz, hzr⟩ := ih hnd.2 (r - 1) (by omega)
        have hza : a < z := by
          rcases Nat.lt_trichotomy a z with h | h | h
          · exact h
          · subst h; exact absurd hz hnd.1
          · have := count_lt_strict l hz h; omega
        refine ⟨z, List.mem_cons_of_mem _ hz, ?_⟩
        simp only [List.countP_cons, decide_eq_true_eq]
        simp [hza, hzr]; omega

/-- Different elements have different ranks. -/
theorem rank_inj (l : List Nat) {y z : Nat} (hy : y ∈ l) (hz : z ∈ l)
    (h : l.countP (· < y) = l.countP (· < z)) : y = z := by
  rcases Nat.lt_trichotomy y z with hl | he | hg
  · have := count_lt_strict l hy hl; omega
  · exact he
  · have := count_lt_strict l hz hg; omega

/-- Splitting a count into the selected positions `args` and the dropped ones. -/
theorem countP_split (xs args : List Nat) (p : Nat → Bool)
    (hnd : args.Nodup) (hr : ∀ a ∈ args, a < xs.length) :
    xs.countP p = (args.map (fun a => xs.getD a 0)).countP p +
      ((List.range xs.length).filter (fun e => !args.contains e)).countP
        (fun e => p (xs.getD e 0)) := by
  rw [← countP_range_getD xs p,
    List.countP_eq_countP_filter_add _ _ (fun e => args.contains e), List.countP_map]
  congr 1
  apply List.Perm.countP_eq
  rw [List.perm_ext_iff_of_nodup (List.Pairwise.filter _ List.nodup_range) hnd]
  intro e
  simp only [List.mem_filter, List.mem_range, List.contains_iff_mem]
  constructor
  · exact fun h => h.2
  · exact fun h => ⟨hr e h, h⟩

end Sel
