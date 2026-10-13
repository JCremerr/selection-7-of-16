import Selection.Checker

/-! Fast table lookup for kernel evaluation of big certificates (NOT trusted:
    everything here is proved).

    `checkFn P H f` reads the program only through `P.funs[g]?` and `H[g]?`.
    For an array literal with thousands of entries, each such lookup is a walk
    of up to thousands of `List.get` steps in the kernel.  `checkFnL` is the
    same checker with the two lookups as parameters (`checkFnL_eq`: equal to
    `checkFn` for the plain lookups), and `look2 K parts` is a two-level
    lookup into data stored as `parts.flatten` with parts of length K
    (`look2_eq`).  Kernel cost per lookup: <= #parts + K steps. -/

namespace Sel

/-- `checkStmt` with the table lookups as parameters (same code). -/
def checkStmtL (lf : Nat → Option Fn) (lh : Nat → Option Hint) (m r : Nat) (h : Hint)
    (Q : Rel) (c : Nat) : Stmt → Bool
  | .ret p => p < m && countBelow Q m p == r && countAbove Q m p + r + 1 == m && c ≤ h.cost
  | .call g args =>
    match lf g, lh g with
    | some G, some hg =>
      args.length == G.arity && args.all (· < m) && nodupB args &&
      hg.height < h.height && c + hg.cost ≤ h.cost &&
      (dropped m args).all (fun e => certBelow Q m r e || certAbove Q m r e) &&
      G.rank + (dropped m args).countP (certBelow Q m r) == r &&
      (List.range args.length).all (fun j => (List.range args.length).all (fun l =>
        !preRel hg.pre j l || Q (args.getD j 0) (args.getD l 0)))
    | _, _ => false

/-- `checkFn` with the table lookups as parameters (same code). -/
def checkFnL (lf : Nat → Option Fn) (lh : Nat → Option Hint) (f : Nat) : Bool :=
  match lf f, lh f with
  | some F, some h =>
    F.rank < F.arity &&
    match F.body with
    | .stmt s => checkStmtL lf lh F.arity F.rank h (preRel h.pre) 0 s
    | .cmp x y lt ge =>
      x < F.arity && y < F.arity && x != y &&
      !preRel h.pre x y && !preRel h.pre y x &&
      checkStmtL lf lh F.arity F.rank h (addRel (preRel h.pre) x y) 1 lt &&
      checkStmtL lf lh F.arity F.rank h (addRel (preRel h.pre) y x) 1 ge
  | _, _ => false

theorem checkStmtL_eq (P : Program) (H : Array Hint) (m r : Nat) (h : Hint) (Q : Rel)
    (c : Nat) (s : Stmt) :
    checkStmtL (fun g => P.funs[g]?) (fun g => H[g]?) m r h Q c s = checkStmt P H m r h Q c s := by
  cases s <;> rfl

theorem checkFnL_eq (P : Program) (H : Array Hint) (f : Nat) :
    checkFnL (fun g => P.funs[g]?) (fun g => H[g]?) f = checkFn P H f := by
  unfold checkFnL checkFn
  dsimp only
  cases P.funs[f]? with
  | none => rfl
  | some F =>
    cases H[f]? with
    | none => rfl
    | some h =>
      obtain ⟨ar, rk, body⟩ := F
      cases body <;> simp only [checkStmtL_eq]

/-- Two-level lookup: `parts` = consecutive blocks of the table, K entries each. -/
def look2 {α : Type} (K : Nat) : List (List α) → Nat → Option α
  | [], _ => none
  | l :: rest, g => if g < K then l[g]? else look2 K rest (g - K)

/-- All blocks have exactly K entries, except the last, which has at most K. -/
def uniform {α : Type} (K : Nat) : List (List α) → Bool
  | [] => true
  | [l] => l.length ≤ K
  | l :: rest => l.length == K && uniform K rest

theorem look2_eq {α : Type} (K : Nat) :
    ∀ (ls : List (List α)), uniform K ls = true → ∀ g, look2 K ls g = ls.flatten[g]?
  | [], _, g => by simp [look2]
  | [l], hu, g => by
    simp only [uniform, decide_eq_true_eq] at hu
    simp only [look2, List.flatten_cons, List.flatten_nil, List.append_nil]
    split
    · rfl
    · rw [List.getElem?_eq_none (by omega)]
  | l :: l2 :: rest, hu, g => by
    simp only [uniform, Bool.and_eq_true, beq_iff_eq] at hu
    have ih := look2_eq K (l2 :: rest) hu.2
    rw [look2, List.flatten_cons]
    split
    · rw [List.getElem?_append_left (by omega)]
    · rw [List.getElem?_append_right (by omega), hu.1, ih]

/-- The checker run with fast lookups proves the plain `(List.range N).all
    (checkFn P H)` when the program and hints are stored as `parts.flatten`. -/
theorem all_checkFn_of_fast (P : Program) (H : Array Hint) (K : Nat)
    (fp : List (List Fn)) (hp : List (List Hint))
    (hP : P.funs = fp.flatten.toArray) (hH : H = hp.flatten.toArray)
    (hfp : uniform K fp = true) (hhp : uniform K hp = true) (N : Nat)
    (hall : (List.range N).all (checkFnL (look2 K fp) (look2 K hp)) = true) :
    (List.range N).all (checkFn P H) = true := by
  have e1 : look2 K fp = fun g => P.funs[g]? := by
    funext g; rw [look2_eq K fp hfp g, hP, List.getElem?_toArray]
  have e2 : look2 K hp = fun g => H[g]? := by
    funext g; rw [look2_eq K hp hhp g, hH, List.getElem?_toArray]
  rw [e1, e2] at hall
  have e3 : checkFnL (fun g => P.funs[g]?) (fun g => H[g]?) = checkFn P H := by
    funext f; exact checkFnL_eq P H f
  rw [e3] at hall
  exact hall

end Sel
