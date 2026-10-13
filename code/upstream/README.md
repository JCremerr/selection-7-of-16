# Patches to the search code of Dörrer et al. (SEA 2025)

The patches apply to [JGDoerrer/selection_generator](https://github.com/JGDoerrer/selection_generator)
at commit `afd67a4`. That code is MIT-licensed, © 2024 Josua Doerrer; its
license is reproduced in `UPSTREAM_LICENSE`. The patch files contain context
lines from it.

* `first_solution_v3.patch`: fixed-budget, first-solution forward search (accept
  the first comparison whose two outcomes are both solvable within the
  remaining budget), with lower bounds kept separately from cached upper
  bounds. Environment variables: `SELGEN_OR_ORDER=cs` orders comparisons by the
  larger compatible-solution count of their two outcomes (information-greedy
  ordering); `SELGEN_AND_HARD_FIRST=1` searches the harder outcome first.
  This version found `certificates/V8_16_le29/select_8th_of_16_29cmp.rs`
  (budget 29, 3 GB cache, about 2 minutes).
* `first_solution_v9.patch`: the last version, used for the later
  depth-first comparisons in the paper. Additional variables:
  `SELGEN_OR_ORDER=phi` (order by Φ*), `SELGEN_WIDTH`, `SELGEN_SEED`,
  `SELGEN_NOISE`, `SELGEN_START` (start from a given poset), `SELGEN_TABLE` and
  `SELGEN_TABLE_MAXN` (exact table as endgame oracle). See the `[patch]`
  comments in the patch.

## Use

```
git clone https://github.com/JGDoerrer/selection_generator && cd selection_generator
git checkout afd67a4
git apply /path/to/first_solution_v3.patch
cargo build --release

SELGEN_BIN=$PWD/target/release/selection_generator \
SELGEN_OR_ORDER=cs SELGEN_AND_HARD_FIRST=1 \
  /path/to/run_budget.sh 16 8 29 3 OUTDIR      # N, I (1-based), budget, cache GB, output dir
```

The algorithm is written to `OUTDIR/n16_i8_k29/algorithms/`. Verify it with
`code/verifier/verify_rs_algorithm.py`.
