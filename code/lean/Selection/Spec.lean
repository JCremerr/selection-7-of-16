/-
  TRUSTED SPECIFICATION  (this file + the statement in Main.lean is all a
  referee has to read; everything else is checked by Lean's kernel.  Only to
  identify the Lean data `Cert.prog` with the Rust certificate one also needs
  Cert.lean, generated and cross-checked, see README.txt).

  A selection program is an array of functions `funs[0], funs[1], ...`.
  Function `f` is called with a list `xs` of `arity` numbers.  Its body
  makes AT MOST ONE comparison `xs[x] < xs[y]` of two argument positions and
  then executes one statement:
    * `ret p`         return the argument at position p, or
    * `call g args`   call function g on the list [xs[args[0]], xs[args[1]], ...]
                      and return what g returns.  Positions of xs that are not
                      in `args` are dropped ("pass-through" on a sub-array);
                      `args` may also reorder the arguments.
  A body without comparison is `stmt s`.  The `rank` field is an annotation
  for the checker only; the semantics `exec` does not read it.

  Example (Rust syntax of the certificate):
      fn select_3([a, b, c]: [usize; 3]) -> usize {
          if a < b { select_2([a, c]) } else { select_2([b, c]) } }
  is   { arity := 3, rank := 0, body := .cmp 0 1 (.call 2 [0, 2]) (.call 2 [1, 2]) }.
-/

namespace Sel

/-- A statement.  Positions refer to the current function's argument list. -/
inductive Stmt where
  | ret  (p : Nat)                    -- return argument p
  | call (g : Nat) (args : List Nat)  -- return g(xs[args[0]], xs[args[1]], ...)
  deriving Repr

/-- A function body: one statement, or one comparison followed by a statement. -/
inductive Body where
  | stmt (s : Stmt)                           -- no comparison
  | cmp (x y : Nat) (lt : Stmt) (ge : Stmt)   -- if xs[x] < xs[y] then lt else ge
  deriving Repr

structure Fn where
  arity : Nat
  rank  : Nat      -- annotation only (0-based rank this function claims to return)
  body  : Body
  deriving Repr

structure Program where
  funs : Array Fn
  root : Nat
  deriving Repr

/-- The comparison step of a body on input `xs`: the statement to execute next,
    and the number of comparisons made (0 or 1).  `none` = position out of range. -/
def Body.step (b : Body) (xs : List Nat) : Option (Stmt × Nat) :=
  match b with
  | .stmt s => some (s, 0)
  | .cmp x y lt ge =>
    match xs[x]?, xs[y]? with
    | some a, some b => some (if a < b then lt else ge, 1)
    | _, _ => none

/-- Run function `f` of `P` on the argument list `xs`.  Result: `some (v, c)` =
    the returned value `v` and the number `c` of comparisons made, or `none` if
    the fuel (maximum call depth) runs out or the program is ill-formed
    (unknown function, wrong number of arguments, position out of range). -/
def exec (P : Program) : (fuel : Nat) → (f : Nat) → (xs : List Nat) → Option (Nat × Nat)
  | 0, _, _ => none
  | fuel + 1, f, xs =>
    match P.funs[f]? with
    | none => none
    | some F =>
      if xs.length ≠ F.arity then none else
      match F.body.step xs with
      | none => none
      | some (.ret p, c) => xs[p]?.map (fun v => (v, c))
      | some (.call g args, c) =>
        match args.mapM (fun a => xs[a]?) with
        | none => none
        | some ys => (exec P fuel g ys).map (fun r => (r.1, c + r.2))

/-- `P` finds the element of 0-based rank `r` (i.e. the (r+1)-th smallest) among
    any `n` distinct inputs, using at most `w` comparisons.  Fuel = number of
    functions (an acyclic program never nests deeper than that). -/
def SelectsWithin (P : Program) (n r w : Nat) : Prop :=
  ∀ xs : List Nat, xs.length = n → xs.Nodup →
    ∃ v c, exec P P.funs.size P.root xs = some (v, c) ∧
      v ∈ xs ∧ (xs.filter (· < v)).length = r ∧ c ≤ w

end Sel
