# Search code: exact solver and df-pn construction

Two Rust crates with the same structure (standard library only, rustc ≥ 1.75):

* `code/core`: posets on at most 16 elements (u16 bitsets, 128-bit canonical keys).
* `code/core32`: the same code widened to at most 19 elements (u32 bitsets,
  192-bit canonical keys); used for n = 17. It reproduces the n = 16
  certificates of `code/core` byte for byte. `core32/xcheck` compares the two
  crates on n ≤ 16.

Neither the search nor its heuristics are part of the proofs: every certificate
is checked by `code/verifier` and by the Lean development.

## Build

```
cd code/core   && cargo build --release     # likewise in code/core32
```

`.cargo/config.toml` sets `-C target-cpu=native`.

## Binaries

| Binary | Purpose |
|---|---|
| `dfpn N I K [flags] [--out=FILE.rs]` | df-pn search for an algorithm selecting the I-th smallest (1-based) of N within K comparisons; writes a certificate if proved |
| `exact N [threads] [outdir]` | exact V_i(n) for all n ≤ N, plus the table of every reduced problem it solved (`data/groundtruth_table.tsv.xz` was made with `exact 12 6`) |
| `lbcheck dump\|table\|gapfit\|regress\|oracle` | tests of the lower bounds of the paper against exact values; fit of the cost predictor |
| `analyze <table.tsv>` | gap statistics of the lower bounds on the exact table |
| `canon_check` | tests of the canonical form |

`src/lbbest.rs` implements the lower bound LB = max(⌈log2 Φ*⌉, KEY) used for
pruning; `src/count.rs` counts compatible solutions and leaves; `src/cert.rs`
writes certificates.

## df-pn configurations

The defaults are configuration **F1** (`--heur=cs --sigma=0.2 --shift=1 --eps=1`,
weak AND sums with `--and-add=1`). **F0** is F1 with `--and-add=0`; it is better
at loose budgets. Unexplored nodes are initialised from a cost predictor
(V ≈ f(log2|C|), coefficients fitted on the exact table and built into
`src/dfpn.rs`), shifted pessimistically by `--shift`.

Memory: `--tt-bits=B` allocates 2^B × 160 bytes for the transposition table
(20 → 160 MB, 24 → 2.6 GB). All tables are fixed-size. See the header of
`src/bin/dfpn.rs` for all options.

Always verify the result:

```
python3 -I code/verifier/verify_rs_algorithm.py FILE.rs --expect-n N --expect-i I --expect-worst K
```

## Reproducing the certificates

The search is deterministic and single-threaded.

| Certificate | Command | Time |
|---|---|---|
| `V7_16_le28/select_7th_of_16_28cmp.rs` | `core: dfpn 16 7 28` | 5 s |
| `V7_16_le28/select_7th_of_16_28cmp_alt.rs` | `core: dfpn 16 7 28 --shift=1.5` | 5 s |
| `V8_16_le29/select_8th_of_16_29cmp_alt_dfpn.rs` | `core: dfpn 16 8 29` | 5 min |
| `V5_17_le27/select_5th_of_17_27cmp.rs` | `core32: dfpn 17 5 27 --and-add=0` | 103 s |
| `V6_17_le29/select_6th_of_17_29cmp.rs` | `core32: dfpn 17 6 29 --and-add=0` | 89 s |
| `V7_17_le30/select_7th_of_17_30cmp.rs` | `core32: dfpn 17 7 30 --and-add=0` | 8 min |
| `V8_17_le31/select_8th_of_17_31cmp.rs` | `core32: dfpn 17 8 31 --and-add=0` | 19 min |
| `V9_17_le31/select_9th_of_17_31cmp.rs` | `core32: dfpn 17 9 31` | 63 min |

The two V_7(16) commands were re-run for this repository with `--tt-bits=20`,
with both `code/core` and `code/core32`,
and give byte-identical files (SHA-256 in `certificates/V7_16_le28/SHA256SUMS`).
The V_5(17) and V_6(17) commands, re-run with `--tt-bits=21` (the setting of
the original runs), also reproduce their certificates byte for byte.
The 252-function certificate `V8_16_le29/select_8th_of_16_29cmp.rs` was found
by the patched upstream depth-first search, see `code/upstream`.
