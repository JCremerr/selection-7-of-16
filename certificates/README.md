# Certificates

One folder per bound. Each folder holds the certificate(s), `SHA256SUMS`, the
output of the static checker (`verifier_output*.txt`) and of the compiled
execution test on 2·10^7 random permutations (`randtest_output*.txt`).

| Bound | File | Functions | Worst case | SHA-256 | Found by | Lean theorem |
|---|---|---:|---:|---|---|---|
| V_7(16) ≤ 28 | `V7_16_le28/select_7th_of_16_28cmp.rs` | 329 | 28 | `eb8f064c12bbed768231fe7be883342fd231b24f05f59e6c0cd0b887ff358d72` | df-pn (`code/core`), 5 s | `Sel.V_7_16_le_28` |
| V_7(16) ≤ 28 | `V7_16_le28/select_7th_of_16_28cmp_alt.rs` | 365 | 28 | `ca27a4ac770dda5c266d1243e8eedfc441c2e73920dee0f33271e6f067cd33ce` | df-pn (`code/core`, `--shift=1.5`), 5 s | – |
| V_8(16) ≤ 29 | `V8_16_le29/select_8th_of_16_29cmp_alt_dfpn.rs` | 2937 | 29 | `2e9615ae1d9b1a1a774d5e9d41770bed68aa2511a8b92e6b47990fb888ddca30` | df-pn (`code/core`), 5 min | `Sel.V_8_16_le_29` |
| V_8(16) ≤ 29 | `V8_16_le29/select_8th_of_16_29cmp.rs` | 252 | 29 | `77d3a6bf85f1d02fcb566fabba71b3f090b55e888a62818abe81330efd275449` | patched upstream DFS (`code/upstream`, v3), 2 min | – |
| V_5(17) ≤ 27 | `V5_17_le27/select_5th_of_17_27cmp.rs` | 146 | 27 | `34e26f201b68d4f6eeb6b4adfdee61627dd97b007e805542457c9582e26d9e42` | df-pn (`code/core32`, F0), 103 s | `Sel.V_5_17_le_27` |
| V_6(17) ≤ 29 | `V6_17_le29/select_6th_of_17_29cmp.rs` | 266 | 29 | `ae6d6b42e1c4574d4f1e7be63cc1503e5604b50ec5a9ec2ab2ba702a6a5ea1e9` | df-pn (`code/core32`, F0), 89 s | `Sel.V_6_17_le_29` |
| V_7(17) ≤ 30 | `V7_17_le30/select_7th_of_17_30cmp.rs` | 977 | 30 | `33718071f64abcb08f24b0b1ca3f76fcb35bfeff9713bc87b02a0e51747e2c4f` | df-pn (`code/core32`, F0), 8 min | `Sel.V_7_17_le_30` |
| V_8(17) ≤ 31 | `V8_17_le31/select_8th_of_17_31cmp.rs` | 1077 | 31 | `5925b96967a87e8a6744d349d993ac77f732f4f5eed2ac3068846a6476e12aa0` | df-pn (`code/core32`, F0), 19 min | `Sel.V_8_17_le_31` |
| V_9(17) ≤ 31 | `V9_17_le31/select_9th_of_17_31cmp.rs` | 1493 | 31 | `9ce649f22b2dba1316c7195d0041292306a493ac07c8f29d9361e2e937009d79` | df-pn (`code/core32`, F1), 63 min | `Sel.V_9_17_le_31` |

Search times are single-threaded on an AMD Ryzen 7 5800X. The exact commands
are in `code/core/README.md` and `code/upstream/README.md`.

## Format

Each certificate is a Rust program in the format of the search code of Dörrer
et al. (SEA 2025). Each function solves a remaining subproblem: it may compare
two arguments, discard arguments that cannot be the answer, and call another
function on the remaining ones. A doc comment declares the size, the 0-based
rank and the known relations, for example

```rust
/// n = 3, i = 1, b < c
fn select_12([a, b, c]: [usize; 3]) -> usize {
    if a < b { select_1([b]) } else { select_2([a, c]) }
}
```

The root `select_0` receives all inputs with no known relations. The checker
does not use the relations stated in comments; it recomputes them from the root.

## Checking

From the repository root:

```
(cd certificates/V7_16_le28 && sha256sum -c SHA256SUMS)
python3 -I code/verifier/verify_rs_algorithm.py certificates/V7_16_le28/select_7th_of_16_28cmp.rs \
    --expect-n 16 --expect-i 7 --expect-worst 28        # --expect-i is 1-based
code/randtest/build_and_run.sh certificates/V7_16_le28/select_7th_of_16_28cmp.rs 20000000
```
