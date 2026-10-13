import Selection.Cert5of17
import Selection.Hints5of17
import Selection.Cert6of17
import Selection.Hints6of17
import Selection.Cert7of17
import Selection.Hints7of17
import Selection.Cert8of17
import Selection.Hints8of17
import Selection.Cert9of17
import Selection.Hints9of17

/-! n = 17 certificates: negative controls for the checker (evaluation, not
    part of the proof): accepted at (17, r, w); w-1, r-1, r+1, n = 16
    rejected; every single-function mutation rejected by the local check of
    that function; the full `check` rejects mutated programs. -/

open Sel

def swapBranches : Body → Body
  | .cmp x y a b => .cmp x y b a
  | b => b

def swapCmp : Body → Body
  | .cmp x y a b => .cmp y x a b
  | b => b

def mutFn (P : Program) (k : Nat) (g : Fn → Fn) : Program :=
  { P with funs := P.funs.modify k g }

def mRank : Fn → Fn := fun F => { F with rank := F.rank + 1 }
def mBr : Fn → Fn := fun F => { F with body := swapBranches F.body }
def mCmp : Fn → Fn := fun F => { F with body := swapCmp F.body }

/-- functions k still accepted (by checkFn at k) after mutating function k -/
def accepted (P : Program) (H : Array Hint) (g : Fn → Fn) : List Nat :=
  (List.range P.funs.size).filter (fun k => checkFn (mutFn P k g) H k)

def report (name : String) (P : Program) (H : Array Hint) (n r w : Nat) : IO Unit := do
  IO.println s!"{name}: {P.funs.size} functions, root {P.root}"
  IO.println s!"  check n r w   = {check P H n r w}   (expected true)"
  IO.println s!"  check w-1     = {check P H n r (w-1)}"
  IO.println s!"  check r-1     = {check P H n (r-1) w}"
  IO.println s!"  check r+1     = {check P H n (r+1) w}"
  IO.println s!"  check n=16    = {check P H 16 r w}"
  IO.println s!"  check n=18    = {check P H 18 r w}"
  let noCmp := (List.range P.funs.size).filter (fun k =>
    match P.funs[k]? with | some { body := .stmt _, .. } => true | _ => false)
  IO.println s!"  functions without comparison: {noCmp}"
  IO.println s!"  accepted after rank+1:         {accepted P H mRank}"
  IO.println s!"  accepted after swap branches:  {accepted P H mBr}"
  IO.println s!"  accepted after swap operands:  {accepted P H mCmp}"
  IO.println s!"  accepted after cost hint-1:    {(List.range P.funs.size).filter (fun k =>
    checkFn P (H.modify k (fun h => { h with cost := h.cost - 1 })) k)}"
  -- full check of mutated programs, a few functions (root, early, middle, last)
  let ks := [P.root, 2, 3, P.funs.size / 2, P.funs.size - 1]
  let full := ks.map (fun k => (k, check (mutFn P k mRank) H n r w,
    check (mutFn P k mBr) H n r w, check (mutFn P k mCmp) H n r w))
  IO.println s!"  full check of mutants (k, rank+1, swapBr, swapCmp): {full}"

#eval report "V5(17)" Cert5of17.prog Cert5of17.hints 17 4 27
#eval report "V6(17)" Cert6of17.prog Cert6of17.hints 17 5 29
#eval report "V7(17)" Cert7of17.prog Cert7of17.hints 17 6 30
#eval report "V8(17)" Cert8of17.prog Cert8of17.hints 17 7 31
#eval report "V9(17)" Cert9of17.prog Cert9of17.hints 17 8 31
