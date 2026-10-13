import Selection.Cert
import Selection.Hints

/-! Negative controls for the checker (evaluation, not part of the proof):
    the certificate is accepted at w = 28, and every mutation is rejected. -/

open Sel Sel.Cert

def swapBranches : Body → Body
  | .cmp x y a b => .cmp x y b a
  | b => b

def swapCmp : Body → Body
  | .cmp x y a b => .cmp y x a b
  | b => b

def mutFn (k : Nat) (g : Fn → Fn) : Program :=
  { prog with funs := prog.funs.modify k g }

/-- the functions k still accepted after mutating function k (expected: [1] only,
    select_1 = `ret 0` has no comparison and cost 0, so there the mutation is
    the identity) -/
def accepted (mk : Nat → Program) : List Nat :=
  (List.range prog.funs.size).filter (fun k => checkFn (mk k) hints k)

#eval check prog hints 16 6 28   -- true
#eval check prog hints 16 6 27   -- false (budget)
#eval check prog hints 16 5 28   -- false (wrong rank)
#eval check prog hints 15 6 28   -- false (wrong n)
-- swap the two branches of every comparison (one function at a time): accepted
#eval accepted (fun k => mutFn k (fun F => { F with body := swapBranches F.body }))
-- swap the operands of every comparison: accepted
#eval accepted (fun k => mutFn k (fun F => { F with body := swapCmp F.body }))
-- claim a different rank: accepted
#eval accepted (fun k => mutFn k (fun F => { F with rank := F.rank + 1 }))
-- lower one cost hint by 1: accepted
#eval (List.range prog.funs.size).filter (fun k =>
  checkFn prog (hints.modify k (fun h => { h with cost := h.cost - 1 })) k)
