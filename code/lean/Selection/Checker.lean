import Selection.Spec

/-! The executable checker (NOT trusted: `Sound.lean` proves that `check = true`
    implies `SelectsWithin`).  Same local rules as code/verifier (README sec. 5),
    but the preconditions are supplied as hints and only CHECKED here:

    * every function f has a hint: precondition poset P_f (bitset), a height
      (strictly decreasing along calls: termination within the fuel) and a cost
      (bound on the number of comparisons from f);
    * knowledge after the comparison x < y is closure(P_f + {x < y}) (`addRel`);
    * comparison: x, y in range, x != y, incomparable in P_f (the last is the
      verifier's "no wasted comparison" policy; soundness does not need it);
    * return p: p has exactly r elements below and m-1-r above in the knowledge;
    * call g args: |args| = arity g, positions distinct and in range; every
      dropped position e is certified "below" (>= m-r elements above e) or
      "above" (>= r+1 below e); rank g + #below = r; the knowledge implies
      P_g on the callee's positions; height and cost decrease. -/

namespace Sel

structure Hint where
  pre    : Nat   -- bit (32*u + v) set  <->  precondition contains x_u < x_v
                 -- (stride 32: injective for arities <= 32; soundness does not
                 -- depend on the encoding, only completeness does)
  height : Nat
  cost   : Nat
  deriving Repr

/-- A relation on positions (true = "known to be smaller"). -/
abbrev Rel := Nat → Nat → Bool

def preRel (pre : Nat) : Rel := fun u v => pre.testBit (32 * u + v)

/-- closure(Q + {lo < hi}), for transitively closed Q. -/
def addRel (Q : Rel) (lo hi : Nat) : Rel :=
  fun u v => Q u v || ((u == lo || Q u lo) && (hi == v || Q hi v))

/-- number of positions below p / above p in Q, among positions 0..m-1 -/
def countBelow (Q : Rel) (m p : Nat) : Nat := (List.range m).countP (fun u => Q u p)
def countAbove (Q : Rel) (m p : Nat) : Nat := (List.range m).countP (fun v => Q p v)

def nodupB : List Nat → Bool
  | [] => true
  | a :: l => !l.contains a && nodupB l

/-- dropped position e is certainly below the answer (rank(e) <= r-1) -/
def certBelow (Q : Rel) (m r e : Nat) : Bool := m ≤ countAbove Q m e + r
/-- dropped position e is certainly above the answer (rank(e) >= r+1) -/
def certAbove (Q : Rel) (m r e : Nat) : Bool := r + 1 ≤ countBelow Q m e

def dropped (m : Nat) (args : List Nat) : List Nat :=
  (List.range m).filter (fun e => !args.contains e)

/-- Check one statement of a function with arity m, rank r and hint h,
    executed with knowledge Q after c comparisons (c = 0 or 1). -/
def checkStmt (P : Program) (H : Array Hint) (m r : Nat) (h : Hint) (Q : Rel) (c : Nat) :
    Stmt → Bool
  | .ret p => p < m && countBelow Q m p == r && countAbove Q m p + r + 1 == m && c ≤ h.cost
  | .call g args =>
    match P.funs[g]?, H[g]? with
    | some G, some hg =>
      args.length == G.arity && args.all (· < m) && nodupB args &&
      hg.height < h.height && c + hg.cost ≤ h.cost &&
      (dropped m args).all (fun e => certBelow Q m r e || certAbove Q m r e) &&
      G.rank + (dropped m args).countP (certBelow Q m r) == r &&
      (List.range args.length).all (fun j => (List.range args.length).all (fun l =>
        !preRel hg.pre j l || Q (args.getD j 0) (args.getD l 0)))
    | _, _ => false

def checkFn (P : Program) (H : Array Hint) (f : Nat) : Bool :=
  match P.funs[f]?, H[f]? with
  | some F, some h =>
    F.rank < F.arity &&
    match F.body with
    | .stmt s => checkStmt P H F.arity F.rank h (preRel h.pre) 0 s
    | .cmp x y lt ge =>
      x < F.arity && y < F.arity && x != y &&
      !preRel h.pre x y && !preRel h.pre y x &&
      checkStmt P H F.arity F.rank h (addRel (preRel h.pre) x y) 1 lt &&
      checkStmt P H F.arity F.rank h (addRel (preRel h.pre) y x) 1 ge
  | _, _ => false

/-- The whole check: every function passes, and the root has arity n, rank r,
    empty precondition, cost <= w and height < number of functions. -/
def check (P : Program) (H : Array Hint) (n r w : Nat) : Bool :=
  (List.range P.funs.size).all (checkFn P H) &&
  match P.funs[P.root]?, H[P.root]? with
  | some F, some h =>
    F.arity == n && F.rank == r && h.pre == 0 && h.cost ≤ w && h.height < P.funs.size
  | _, _ => false

end Sel
