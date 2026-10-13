/-
  TRUSTED SPECIFICATION of the Section 5 lower bounds
  (selection/paper/short.tex, Section "Lower bounds from a partially ordered
  state"; expanded proofs in main.tex Section 5).

  A referee checks THIS FILE and the theorem statements in LB/Main.lean.
  Everything else is proved and checked by the Lean kernel.  The definitions
  follow the paper literally; the comments quote the paper's wording.

    paper                          Lean
    -----------------------------  -------------------------------------------
    labels Omega, n = |Omega|      a finite type `α`, n = `Fintype.card α`
    known partial order P          `P : SPO α` (irreflexive, transitive)
    rank i, 1 <= i <= n            `i : ℕ` with hypotheses `1 ≤ i`, `i ≤ card α`
    input / total order extending P
                                   `f : α → ℕ` with `IsLinExt P f` (injective,
                                   every known relation u < v has f u < f v);
                                   u comes before v iff f u < f v
    compatible solution (L,e,U)    `(L, e) ∈ C P i`, U = the remaining labels
    down-set, Down_j(P)            `IsDownSet P S`, `Down P j`
    b(L), a(A)                     `bOf P L`, `aOf P A`
    FG, FGdual, Phi*               `FG P i`, `FGdual P i`, `Phi P i` (in ℤ)
    KEY                            `KEY P i`
    comparison decision tree       `T : CTree α` (compares two labels per node)
    correct tree for (P, i)        `Correct P i T`
    comparisons on an input        `T.pathLen f`;  leaves `T.leaves`;
                                   depth `T.depth`
    V_i(P)                         `V P i`
    Psi_{F,G}, antichain E_n       `Psi F G P i`, `antichain α`
-/
import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Data.Fintype.Powerset
import Mathlib.Data.Fintype.Prod
import Mathlib.Order.Lattice.Nat
import Mathlib.Basic.Real.Basic

namespace LB

open Finset

/-- A strict partial order on the labels: the comparison results known so far
(closed under transitivity). -/
structure SPO (α : Type*) where
  /-- `lt u v`: it is known that `u` is smaller than `v`. -/
  lt : α → α → Prop
  irrefl : ∀ a, ¬ lt a a
  trans : ∀ {a b c}, lt a b → lt b c → lt a c

/-- The empty order (no comparisons known), `E_n` in the paper. -/
def antichain (α : Type*) : SPO α where
  lt _ _ := False
  irrefl _ h := h
  trans h _ := h

/-- A comparison decision tree over the ORIGINAL labels.  `node x y t₁ t₂`
compares the input values of labels `x` and `y` and continues with `t₁` if
`x < y`, otherwise with `t₂`; `leaf e` returns the label `e`.  There is no
restriction on the compared pairs (they may be already comparable, or even
equal; such comparisons are simply wasted). -/
inductive CTree (α : Type*) where
  | leaf (e : α)
  | node (x y : α) (lt ge : CTree α)

namespace CTree

variable {α : Type*}

/-- The label returned on input `f`. -/
def eval : CTree α → (α → ℕ) → α
  | leaf e, _ => e
  | node x y t₁ t₂, f => if f x < f y then t₁.eval f else t₂.eval f

/-- The number of comparisons made on input `f` (length of its root-leaf path). -/
def pathLen : CTree α → (α → ℕ) → ℕ
  | leaf _, _ => 0
  | node x y t₁ t₂, f => 1 + if f x < f y then t₁.pathLen f else t₂.pathLen f

/-- Number of leaves (all leaves, reachable or not). -/
def leaves : CTree α → ℕ
  | leaf _ => 1
  | node _ _ t₁ t₂ => t₁.leaves + t₂.leaves

/-- Depth (longest root-leaf path, reachable or not). -/
def depth : CTree α → ℕ
  | leaf _ => 0
  | node _ _ t₁ t₂ => 1 + max t₁.depth t₂.depth

end CTree

variable {α : Type*} [Fintype α] [DecidableEq α]

/-- An input consistent with `P`: pairwise distinct values for the labels such
that every known relation `u < v` of `P` holds.  Equivalently, the total order
"`u` before `v` iff `f u < f v`" is a total order extending `P` (every strict
total order on a finite set arises from such an `f`). -/
def IsLinExt (P : SPO α) (f : α → ℕ) : Prop :=
  Function.Injective f ∧ ∀ u v, P.lt u v → f u < f v

/-- Compatible solutions `C(P)`: "a triple `(L, e, U)` with `|L| = i - 1` such
that some total order extending `P` lists `L`, then `e`, then `U`".  We store
the pair `(L, e)`; `U` is the set of all other labels.  (The condition forces
`e ∉ L`, because `f e < f e` is impossible.) -/
noncomputable def C (P : SPO α) (i : ℕ) : Finset (Finset α × α) := by
  classical
  exact univ.filter fun s => s.1.card = i - 1 ∧
    ∃ f, IsLinExt P f ∧ (∀ l ∈ s.1, f l < f s.2) ∧ (∀ u, u ∉ s.1 → u ≠ s.2 → f s.2 < f u)

/-- "A down-set contains all known predecessors of its members." -/
def IsDownSet (P : SPO α) (S : Finset α) : Prop :=
  ∀ u v, P.lt u v → v ∈ S → u ∈ S

/-- `Down_j(P)`: the down-sets of size `j`. -/
noncomputable def Down (P : SPO α) (j : ℕ) : Finset (Finset α) := by
  classical
  exact univ.filter fun S => IsDownSet P S ∧ S.card = j

/-- Minimal elements of `S` under `P` restricted to `S`. -/
noncomputable def minimals (P : SPO α) (S : Finset α) : Finset α := by
  classical
  exact S.filter fun v => ∀ u ∈ S, ¬ P.lt u v

/-- Maximal elements of `S` under `P` restricted to `S`. -/
noncomputable def maximals (P : SPO α) (S : Finset α) : Finset α := by
  classical
  exact S.filter fun v => ∀ u ∈ S, ¬ P.lt v u

/-- `b(L)` = "the number of minimal elements of its complement minus one".
This is natural-number subtraction; it is exact (no truncation) for every
`L ∈ Down P (i-1)` with `1 ≤ i ≤ n`, because the complement is then nonempty and
has a minimal element (proved: `LB.bOf_add_one`). -/
noncomputable def bOf (P : SPO α) (L : Finset α) : ℕ := (minimals P Lᶜ).card - 1

/-- `a(A)` = "the number of maximal elements of `A` minus one"; exact for every
`A ∈ Down P i` with `1 ≤ i` (proved: `LB.aOf_add_one`). -/
noncomputable def aOf (P : SPO α) (A : Finset α) : ℕ := (maximals P A).card - 1

/-- `FG(P) = ∑_{L ∈ Down_{i-1}(P)} 2^{b(L)}`. -/
noncomputable def FG (P : SPO α) (i : ℕ) : ℕ := ∑ L ∈ Down P (i - 1), 2 ^ bOf P L

/-- `FGdual(P) = ∑_{A ∈ Down_i(P)} 2^{a(A)}`. -/
noncomputable def FGdual (P : SPO α) (i : ℕ) : ℕ := ∑ A ∈ Down P i, 2 ^ aOf P A

/-- `Phi*(P) = FG(P) + FGdual(P) - |C(P)|`, computed in `ℤ` so that no
truncated subtraction occurs.  (It is `≥ 1` for valid ranks: `LB.one_le_Phi`.) -/
noncomputable def Phi (P : SPO α) (i : ℕ) : ℤ :=
  (FG P i : ℤ) + (FGdual P i : ℤ) - ((C P i).card : ℤ)

/-- `KEY(P) = max_{s ∈ C(P)} (a_s + b_s)` with `a_s = a(L ∪ {e})`, `b_s = b(L)`
(the maximum over the empty set is `0`; `C P i` is nonempty for valid ranks). -/
noncomputable def KEY (P : SPO α) (i : ℕ) : ℕ :=
  (C P i).sup fun s => aOf P (insert s.2 s.1) + bOf P s.1

/-- `T` is a correct decision tree for `(P, i)`: on every input consistent
with `P` it returns the `i`-th smallest label, i.e. a label with exactly
`i - 1` smaller values. -/
def Correct (P : SPO α) (i : ℕ) (T : CTree α) : Prop :=
  ∀ f, IsLinExt P f → (univ.filter fun u => f u < f (T.eval f)).card = i - 1

/-- `V_i(P)`: the least `d` such that some correct tree for `(P, i)` makes at
most `d` comparisons on every input consistent with `P` (worst case over the
inputs, not the syntactic depth).  A correct tree always exists for valid
ranks (`LB.exists_correct`), so the infimum is attained. -/
noncomputable def V (P : SPO α) (i : ℕ) : ℕ :=
  sInf {d | ∃ T : CTree α, Correct P i T ∧ ∀ f, IsLinExt P f → T.pathLen f ≤ d}

/-- The family `Psi_{F,G} = |C| + ∑_L F(b(L)) + ∑_A G(a(A))` of Proposition
`prop:phiopt` (main.tex: `F, G` real-valued on the nonnegative integers). -/
noncomputable def Psi (F G : ℕ → ℝ) (P : SPO α) (i : ℕ) : ℝ :=
  ((C P i).card : ℝ) + ∑ L ∈ Down P (i - 1), F (bOf P L) + ∑ A ∈ Down P i, G (aOf P A)

end LB
