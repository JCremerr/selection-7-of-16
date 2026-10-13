import Selection.Cert8
import Selection.Hints8

/-! V_8(16) certificate: negative controls for the checker (evaluation, not part
    of the proof): accepted at (16, 7, 29); wrong budget/rank/n rejected; every
    single-function mutation rejected. -/

open Sel Sel.Cert8

def swapBranches : Body → Body
  | .cmp x y a b => .cmp x y b a
  | b => b

def swapCmp : Body → Body
  | .cmp x y a b => .cmp y x a b
  | b => b

def mutFn (k : Nat) (g : Fn → Fn) : Program :=
  { prog with funs := prog.funs.modify k g }

/-- functions k still accepted after mutating function k -/
def accepted (mk : Nat → Program) : List Nat :=
  (List.range prog.funs.size).filter (fun k => checkFn (mk k) hints k)

#eval prog.funs.size              -- 2937
#eval check prog hints 16 7 29    -- true
#eval check prog hints 16 7 28    -- false (budget)
#eval check prog hints 16 6 29    -- false (wrong rank)
#eval check prog hints 16 8 29    -- false (wrong rank)
#eval check prog hints 15 7 29    -- false (wrong n)
-- functions with no comparison (there the branch/operand mutations are the identity)
#eval (List.range prog.funs.size).filter (fun k =>
  match prog.funs[k]? with | some { body := .stmt _, .. } => true | _ => false)
#eval accepted (fun k => mutFn k (fun F => { F with body := swapBranches F.body }))
#eval accepted (fun k => mutFn k (fun F => { F with body := swapCmp F.body }))
#eval accepted (fun k => mutFn k (fun F => { F with rank := F.rank + 1 }))
#eval (List.range prog.funs.size).filter (fun k =>
  checkFn prog (hints.modify k (fun h => { h with cost := h.cost - 1 })) k)
