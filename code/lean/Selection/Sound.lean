import Selection.Checker
import Selection.Count

/-! Soundness of the checker:  `check P H n r w = true → SelectsWithin P n r w`.
    Invariant (induction on the fuel): the input of function f satisfies the
    hint precondition P_f; then f returns the element of rank `rank f` with at
    most `cost f` comparisons.  Proof outline: code/verifier/README.txt sec. 6. -/

namespace Sel

/-- `xs` satisfies the relation `Q` on positions. -/
def Sat (Q : Rel) (xs : List Nat) : Prop :=
  ∀ u v, u < xs.length → v < xs.length → Q u v = true → xs.getD u 0 < xs.getD v 0

theorem sat_addRel {Q : Rel} {xs : List Nat} {lo hi : Nat} (h : Sat Q xs)
    (hlo : lo < xs.length) (hhi : hi < xs.length) (hlt : xs.getD lo 0 < xs.getD hi 0) :
    Sat (addRel Q lo hi) xs := by
  intro u v hu hv huv
  simp only [addRel, Bool.or_eq_true, Bool.and_eq_true, beq_iff_eq] at huv
  rcases huv with h1 | ⟨h2, h3⟩
  · exact h u v hu hv h1
  · have a : xs.getD u 0 ≤ xs.getD lo 0 := by
      rcases h2 with h2 | h2
      · subst h2; exact Nat.le_refl _
      · exact Nat.le_of_lt (h u lo hu hlo h2)
    have b : xs.getD hi 0 ≤ xs.getD v 0 := by
      rcases h3 with h3 | h3
      · subst h3; exact Nat.le_refl _
      · exact Nat.le_of_lt (h hi v hhi hv h3)
    omega

theorem getD_mem {xs : List Nat} {p : Nat} (h : p < xs.length) : xs.getD p 0 ∈ xs := by
  rw [List.getD_eq_getElem?_getD, List.getElem?_eq_getElem h]
  exact List.getElem_mem h

theorem getD_eq_some {xs : List Nat} {p : Nat} (h : p < xs.length) :
    xs[p]? = some (xs.getD p 0) := by
  rw [List.getD_eq_getElem?_getD, List.getElem?_eq_getElem h]; rfl

theorem countBelow_le {Q : Rel} {xs : List Nat} {p : Nat} (h : Sat Q xs) (hp : p < xs.length) :
    countBelow Q xs.length p ≤ xs.countP (· < xs.getD p 0) := by
  rw [← countP_range_getD xs]
  apply List.countP_mono_left
  intro u hu hq
  simp only [List.mem_range] at hu
  simpa using h u p hu hp hq

theorem countAbove_le {Q : Rel} {xs : List Nat} {p : Nat} (h : Sat Q xs) (hp : p < xs.length) :
    countAbove Q xs.length p ≤ xs.countP (xs.getD p 0 < ·) := by
  rw [← countP_range_getD xs]
  apply List.countP_mono_left
  intro v hv hq
  simp only [List.mem_range] at hv
  simpa using h p v hp hv hq

/-- Rule R3 (return). -/
theorem leaf_rank {Q : Rel} {xs : List Nat} {p r : Nat} (hnd : xs.Nodup) (h : Sat Q xs)
    (hp : p < xs.length) (hb : countBelow Q xs.length p = r)
    (ha : countAbove Q xs.length p + r + 1 = xs.length) :
    xs.countP (· < xs.getD p 0) = r := by
  have h1 := countBelow_le h hp
  have h2 := countAbove_le h hp
  have h3 := count_lt_add_gt xs _ hnd (getD_mem hp)
  omega

theorem certBelow_rank {Q : Rel} {xs : List Nat} {r e : Nat} (hnd : xs.Nodup) (h : Sat Q xs)
    (he : e < xs.length) (hc : certBelow Q xs.length r e = true) :
    xs.countP (· < xs.getD e 0) < r := by
  simp only [certBelow, decide_eq_true_eq] at hc
  have h2 := countAbove_le h he
  have h3 := count_lt_add_gt xs _ hnd (getD_mem he)
  omega

theorem certAbove_rank {Q : Rel} {xs : List Nat} {r e : Nat} (h : Sat Q xs)
    (he : e < xs.length) (hc : certAbove Q xs.length r e = true) :
    r < xs.countP (· < xs.getD e 0) := by
  simp only [certAbove, decide_eq_true_eq] at hc
  have h1 := countBelow_le h he
  omega

/-- Rule R2 (call with dropped elements): if the callee returns the element of
    rank `rg` among the arguments, it is the element of rank `r` of `xs`. -/
theorem call_rank {Q : Rel} {xs args : List Nat} {r rg y : Nat}
    (hnd : xs.Nodup) (hsat : Sat Q xs) (hr : r < xs.length)
    (hargs : ∀ a ∈ args, a < xs.length) (hargnd : args.Nodup)
    (hcert : ∀ e ∈ dropped xs.length args,
      certBelow Q xs.length r e = true ∨ certAbove Q xs.length r e = true)
    (hrank : rg + (dropped xs.length args).countP (certBelow Q xs.length r) = r)
    (hy : y ∈ args.map (fun a => xs.getD a 0))
    (hyr : (args.map (fun a => xs.getD a 0)).countP (· < y) = rg) :
    xs.countP (· < y) = r := by
  obtain ⟨z, hz, hzr⟩ := exists_rank xs hnd r hr
  have hdrop : ∀ e, e ∈ dropped xs.length args ↔ e < xs.length ∧ e ∉ args := by
    intro e; simp [dropped, List.mem_filter, List.mem_range]
  -- z is not dropped, so z is an argument
  have hzarg : z ∈ args.map (fun a => xs.getD a 0) := by
    obtain ⟨e, he, hze⟩ := List.mem_iff_getElem.mp hz
    have hze' : xs.getD e 0 = z := by rw [List.getD_eq_getElem?_getD, List.getElem?_eq_getElem he]; exact hze
    by_cases hea : e ∈ args
    · exact List.mem_map.mpr ⟨e, hea, hze'⟩
    · rcases hcert e ((hdrop e).mpr ⟨he, hea⟩) with hc | hc
      · have := certBelow_rank hnd hsat he hc; rw [hze'] at this; omega
      · have := certAbove_rank hsat he hc; rw [hze'] at this; omega
  -- the dropped elements below z are exactly the ones certified "below"
  have hcnt : (dropped xs.length args).countP (fun e => decide (xs.getD e 0 < z)) =
      (dropped xs.length args).countP (certBelow Q xs.length r) := by
    apply List.countP_congr
    intro e hed
    have he := ((hdrop e).mp hed).1
    simp only [decide_eq_true_eq]
    constructor
    · intro hlt
      rcases hcert e hed with hc | hc
      · exact hc
      · have h1 := certAbove_rank hsat he hc
        have h2 := count_lt_strict xs (getD_mem he) hlt
        omega
    · intro hc
      have h1 := certBelow_rank hnd hsat he hc
      apply Nat.lt_of_not_le; intro hle
      have := count_lt_mono xs hle; omega
  have hsplit := countP_split xs args (fun v => decide (v < z)) hargnd hargs
  have hzr' : (args.map (fun a => xs.getD a 0)).countP (· < z) = rg := by
    have : (dropped xs.length args).countP (fun e => decide (xs.getD e 0 < z)) =
      ((List.range xs.length).filter (fun e => !args.contains e)).countP
        (fun e => decide (xs.getD e 0 < z)) := rfl
    omega
  have hyz : y = z := rank_inj _ hy hzarg (by rw [hyr, hzr'])
  rw [hyz, hzr]

theorem nodupB_nodup : ∀ {l : List Nat}, nodupB l = true → l.Nodup
  | [], _ => List.nodup_nil
  | a :: l, h => by
    simp only [nodupB, Bool.and_eq_true, Bool.not_eq_true'] at h
    rw [List.nodup_cons]
    refine ⟨?_, nodupB_nodup h.2⟩
    intro hm
    have : (List.contains l a) = true := List.contains_iff_mem.mpr hm
    rw [this] at h; exact Bool.noConfusion h.1

theorem mapM_getElem {xs : List Nat} : ∀ {args : List Nat}, (∀ a ∈ args, a < xs.length) →
    args.mapM (fun a => xs[a]?) = some (args.map (fun a => xs.getD a 0))
  | [], _ => rfl
  | a :: l, h => by
    have ha : a < xs.length := h a (List.mem_cons_self)
    have hl := mapM_getElem (args := l) (fun b hb => h b (List.mem_cons_of_mem _ hb))
    simp only [List.mapM_cons, getD_eq_some ha, hl, List.map_cons]
    rfl

theorem getD_map_lt {xs args : List Nat} {j : Nat} (hj : j < args.length) :
    (args.map (fun a => xs.getD a 0)).getD j 0 = xs.getD (args.getD j 0) 0 := by
  rw [List.getD_eq_getElem?_getD, List.getElem?_map, List.getElem?_eq_getElem hj,
    List.getD_eq_getElem?_getD (l := args), List.getElem?_eq_getElem hj]
  rfl

/-- The callee's input: distinct, and it satisfies the callee's precondition. -/
theorem callee_input {Q : Rel} {xs args : List Nat} {pre : Nat}
    (hnd : xs.Nodup) (hsat : Sat Q xs)
    (hargs : ∀ a ∈ args, a < xs.length) (hargnd : args.Nodup)
    (himp : ∀ j, j < args.length → ∀ l, l < args.length →
      (!preRel pre j l || Q (args.getD j 0) (args.getD l 0)) = true) :
    (args.map (fun a => xs.getD a 0)).Nodup ∧ Sat (preRel pre) (args.map (fun a => xs.getD a 0)) := by
  constructor
  · unfold List.Nodup
    rw [List.pairwise_map]
    apply List.Pairwise.imp_of_mem _ hargnd
    intro a b ha hb hab heq
    exact hab ((List.Nodup.getD_inj (hargs a ha) (hargs b hb) hnd).mp heq)
  · intro j l hj hl hp
    simp only [List.length_map] at hj hl
    have hq := himp j hj l hl
    simp only [hp, Bool.not_true, Bool.false_or] at hq
    have hmj : args.getD j 0 ∈ args := getD_mem hj
    have hml : args.getD l 0 ∈ args := getD_mem hl
    rw [getD_map_lt hj, getD_map_lt hl]
    exact hsat _ _ (hargs _ hmj) (hargs _ hml) hq

/-- Execution of the statement chosen by a body (mirrors the end of `exec`). -/
def runStmt (P : Program) (fuel : Nat) (xs : List Nat) (s : Stmt) (c : Nat) :
    Option (Nat × Nat) :=
  match s with
  | .ret p => xs[p]?.map (fun v => (v, c))
  | .call g args =>
    match args.mapM (fun a => xs[a]?) with
    | none => none
    | some ys => (exec P fuel g ys).map (fun r => (r.1, c + r.2))

theorem exec_succ {P : Program} {fuel f : Nat} {xs : List Nat} {F : Fn} {s : Stmt} {c : Nat}
    (hF : P.funs[f]? = some F) (hlen : xs.length = F.arity)
    (hstep : F.body.step xs = some (s, c)) :
    exec P (fuel + 1) f xs = runStmt P fuel xs s c := by
  simp only [exec, hF, hlen, ne_eq, not_true_eq_false, ite_false, hstep]
  cases s <;> rfl

/-- The induction hypothesis = the specification of every function at a given fuel. -/
def FnSpec (P : Program) (H : Array Hint) (fuel : Nat) : Prop :=
  ∀ g G hg, P.funs[g]? = some G → H[g]? = some hg → hg.height < fuel →
  ∀ ys : List Nat, ys.length = G.arity → ys.Nodup → Sat (preRel hg.pre) ys →
  ∃ v c, exec P fuel g ys = some (v, c) ∧ v ∈ ys ∧ ys.countP (· < v) = G.rank ∧ c ≤ hg.cost

theorem stmt_ok {P : Program} {H : Array Hint} {fuel r c : Nat} {h : Hint} {Q : Rel}
    {xs : List Nat} {s : Stmt} (IH : FnSpec P H fuel)
    (hcs : checkStmt P H xs.length r h Q c s = true) (hr : r < xs.length)
    (hnd : xs.Nodup) (hsat : Sat Q xs) (hh : h.height ≤ fuel) :
    ∃ v c', runStmt P fuel xs s c = some (v, c') ∧ v ∈ xs ∧ xs.countP (· < v) = r ∧
      c' ≤ h.cost := by
  cases s with
  | ret p =>
    simp only [checkStmt, Bool.and_eq_true, decide_eq_true_eq, beq_iff_eq] at hcs
    obtain ⟨⟨⟨hp, hb⟩, ha⟩, hc⟩ := hcs
    refine ⟨xs.getD p 0, c, ?_, getD_mem hp, leaf_rank hnd hsat hp hb ha, hc⟩
    simp only [runStmt, getD_eq_some hp]
    rfl
  | call g args =>
    simp only [checkStmt] at hcs
    split at hcs
    · rename_i G hg hG hH
      simp only [Bool.and_eq_true, decide_eq_true_eq, beq_iff_eq, List.all_eq_true,
        List.mem_range] at hcs
      obtain ⟨⟨⟨⟨⟨⟨⟨hk, hin⟩, hnodup⟩, hhg⟩, hcost⟩, hcert⟩, hrank⟩, himp⟩ := hcs
      have hargnd := nodupB_nodup hnodup
      have hargs : ∀ a ∈ args, a < xs.length := fun a ha => by simpa using hin a ha
      obtain ⟨hynd, hysat⟩ := callee_input (pre := hg.pre) hnd hsat hargs hargnd
        (fun j hj l hl => himp j hj l hl)
      obtain ⟨v, c', hex, hv, hvr, hc'⟩ :=
        IH g G hg hG hH (by omega) _ (by simp [hk]) hynd hysat
      refine ⟨v, c + c', ?_, ?_, ?_, by omega⟩
      · simp only [runStmt, mapM_getElem hargs, hex]
        rfl
      · obtain ⟨a, ha, rfl⟩ := List.mem_map.mp hv
        exact getD_mem (hargs a ha)
      · exact call_rank hnd hsat hr hargs hargnd
          (fun e he => by simpa [Bool.or_eq_true] using hcert e he) hrank hv hvr
    · simp at hcs

theorem fn_ok (P : Program) (H : Array Hint)
    (hall : ∀ f, f < P.funs.size → checkFn P H f = true) : ∀ fuel, FnSpec P H fuel := by
  intro fuel
  induction fuel with
  | zero => intro f F h _ _ hh; omega
  | succ fuel ih =>
    intro f F h hF hH hh xs hlen hnd hsat
    have hf : f < P.funs.size := by
      rcases Array.getElem?_eq_some_iff.mp hF with ⟨hf, _⟩; exact hf
    have hc := hall f hf
    simp only [checkFn, hF, hH, Bool.and_eq_true, decide_eq_true_eq] at hc
    obtain ⟨hrank, hbody⟩ := hc
    rw [← hlen] at hrank hbody
    cases hb : F.body with
    | stmt s =>
      rw [hb] at hbody
      have hstep : F.body.step xs = some (s, 0) := by rw [hb]; rfl
      rw [exec_succ hF hlen hstep]
      exact stmt_ok ih hbody hrank hnd hsat (by omega)
    | cmp x y lt ge =>
      rw [hb] at hbody
      simp only [Bool.and_eq_true, decide_eq_true_eq, bne_iff_ne, ne_eq] at hbody
      obtain ⟨⟨⟨⟨⟨⟨hx, hy⟩, hxy⟩, _⟩, _⟩, hlt⟩, hge⟩ := hbody
      by_cases hab : xs.getD x 0 < xs.getD y 0
      · have hstep : F.body.step xs = some (lt, 1) := by
          rw [hb]; simp only [Body.step, getD_eq_some hx, getD_eq_some hy, hab, ↓reduceIte]
        rw [exec_succ hF hlen hstep]
        exact stmt_ok ih hlt hrank hnd (sat_addRel hsat hx hy hab) (by omega)
      · have hne : xs.getD x 0 ≠ xs.getD y 0 := fun he =>
          hxy ((List.Nodup.getD_inj hx hy hnd).mp he)
        have hba : xs.getD y 0 < xs.getD x 0 := by omega
        have hstep : F.body.step xs = some (ge, 1) := by
          rw [hb]; simp only [Body.step, getD_eq_some hx, getD_eq_some hy, hab, ↓reduceIte]
        rw [exec_succ hF hlen hstep]
        exact stmt_ok ih hge hrank hnd (sat_addRel hsat hy hx hba) (by omega)

/-- SOUNDNESS: a successful check proves the specification. -/
theorem sound (P : Program) (H : Array Hint) (n r w : Nat) (hc : check P H n r w = true) :
    SelectsWithin P n r w := by
  simp only [check, Bool.and_eq_true, List.all_eq_true, List.mem_range] at hc
  obtain ⟨hall, hroot⟩ := hc
  split at hroot
  · rename_i F h hF hH
    simp only [Bool.and_eq_true, decide_eq_true_eq, beq_iff_eq] at hroot
    obtain ⟨⟨⟨⟨harity, hrank⟩, hpre⟩, hcost⟩, hheight⟩ := hroot
    intro xs hlen hnd
    have hsat : Sat (preRel h.pre) xs := by
      intro u v _ _ huv; simp [preRel, hpre] at huv
    obtain ⟨v, c, hex, hv, hvr, hc⟩ :=
      fn_ok P H hall P.funs.size P.root F h hF hH hheight xs (by omega) hnd hsat
    exact ⟨v, c, hex, hv, by rw [← List.countP_eq_length_filter, hvr, hrank], by omega⟩
  · simp at hroot

end Sel
