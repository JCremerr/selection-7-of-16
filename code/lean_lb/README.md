# Lean proofs of the lower-bound theory

This development formalizes the universal analytical results of Section 5 of
the paper for every finite strict partial order and every valid rank
`1 ≤ i ≤ n`. It is separate from the seven numerical upper-bound proofs in
[`../lean`](../lean).

Start with [`LB/Spec.lean`](LB/Spec.lean) for the definitions and
[`LB/Main.lean`](LB/Main.lean) for the final theorem statements.

## Claims and formal statements

| Paper claim | Theorem in `LB/Main.lean` |
|---|---|
| Every correct tree has at least Phi leaves | `thm_phi_leaves` |
| Phi ≤ 2^d for a bound d on comparisons on every consistent input | `thm_phi_cost` |
| Ceiling log₂(Phi) ≤ V | `thm_phi_V` |
| Two compatible-solution counting identities | `count_identities` |
| Adding a comparison gives the transitive closure | `add_is_closure` |
| One-set comparison lemma, both orientations | `lem_world_L`, `lem_world_A` |
| Exact accounting of duplicated solutions | `duplicate_accounting` |
| Local inequality for Phi | `local_inequality` |
| Correct leaves have Phi = 1 | `correct_leaf` |
| KEY lower bound, including the bound along a realizing input | `thm_key`, `thm_key_path` |
| Combined lower bound and dominance over the one-sided weights | `cor_combined` |
| Optimality within the specified separable family | `prop_phiopt`, `Phi_in_family` |
| Removing redundant comparisons and relating cost to depth | `pruning`, `V_is_min_depth` |
| The optimal comparison cost is attained | `V_attained` |

`LB/Examples.lean` includes the minimum of four, the median-of-three example,
and negative controls for incorrect decision trees.

## Model and trust

Inputs assign distinct natural-number values to the original labels and
satisfy every relation of the known partial order. Every finite total order
has such a rank representation. Trees can compare any pair of labels,
including already ordered pairs, and must return the requested rank on every
consistent input. Every executed comparison counts. `V` minimizes the actual
worst-case path length; existence and attainment are proved.

Compatible solutions are defined using total orders extending the partial
order. `Phi` is the integer-valued sum of the two weights minus the solution
count. Its positivity is proved before conversion to natural numbers in the
ceiling-log theorem. The bounds hold for arbitrary finite sizes.

The specification, final statements, Lean kernel, relevant Mathlib definitions,
and standard axioms `propext`, `Classical.choice`, and `Quot.sound` form the
trust basis. The development contains no unfinished proofs or new mathematical
axioms and does not use `native_decide`. The reported verification included a
clean build, axiom inspection, per-module kernel replay, and a fresh replay
including imports. The definitions and statements also received separate
AI source reviews; these are not human referee reviews.

## Build and check

Lean and Mathlib are pinned to v4.34.1; the dependency revisions are recorded
in `lake-manifest.json`. Install Lean through elan and run these commands
from this directory. Unlike `../lean`, this development requires Mathlib.
Allow several GB for dependencies and build artifacts. First-time dependency
setup is additional to the approximately 33-second build measured with
prebuilt Mathlib dependencies.

```sh
lake build
lake env lean tests/Axioms.lean
lake env lean tests/Statements.lean
tools/leancheck_all.sh --fresh
```

On Linux with a systemd user session, the optional portable wrapper applies a
memory limit and runs from the project directory:

```sh
tools/capped.sh 6G lake build
tools/capped.sh 4G lake env lean tests/Axioms.lean
tools/capped.sh 4G lake env lean tests/Statements.lean
tools/capped.sh 8G tools/leancheck_all.sh --fresh
```

Use the pinned toolchain inside this project. The sources import selected
Mathlib modules; importing all of Mathlib substantially increases memory use.
The `.lake` dependency and build directories are not distributed here.

## Scope

These proofs cover the analytical lower-bound theory. They do not formalize
the evaluation statistics, the certificate count of 248,381,440 leaves,
statements about prior literature or upstream code, the search implementation,
or the published lemma equating the cost of a state and its reduction.
The numerical upper bounds have their own development in `../lean` and do
not depend on this theory. The published lower bounds used to establish the
exact numerical values are credited in the repository's main README.
