# Lean 4 proofs of the upper bounds

All seven upper bounds of the paper are theorems of this Lean 4 development.
It uses only Lean's core library (no Mathlib). Toolchain: `leanprover/lean4:v4.34.1`
(see `lean-toolchain`; install with [elan](https://github.com/leanprover/elan)).

| Bound | Theorem | File | Certificate |
|---|---|---|---|
| V_7(16) ≤ 28 | `Sel.V_7_16_le_28` | `Selection/Main.lean` | `V7_16_le28/select_7th_of_16_28cmp.rs` |
| V_8(16) ≤ 29 | `Sel.V_8_16_le_29` | `Selection/Main8.lean` | `V8_16_le29/select_8th_of_16_29cmp_alt_dfpn.rs` |
| V_5(17) ≤ 27 | `Sel.V_5_17_le_27` | `Selection/Main17.lean` | `V5_17_le27/select_5th_of_17_27cmp.rs` |
| V_6(17) ≤ 29 | `Sel.V_6_17_le_29` | `Selection/Main17.lean` | `V6_17_le29/select_6th_of_17_29cmp.rs` |
| V_7(17) ≤ 30 | `Sel.V_7_17_le_30` | `Selection/Main17.lean` | `V7_17_le30/select_7th_of_17_30cmp.rs` |
| V_8(17) ≤ 31 | `Sel.V_8_17_le_31` | `Selection/Main17.lean` | `V8_17_le31/select_8th_of_17_31cmp.rs` |
| V_9(17) ≤ 31 | `Sel.V_9_17_le_31` | `Selection/Main17.lean` | `V9_17_le31/select_9th_of_17_31cmp.rs` |

Each statement has the form `∃ P : Program, SelectsWithin P n r w` (rank `r` is
0-based): on every list of `n` distinct natural numbers, `P` returns an input
with exactly `r` smaller inputs, using at most `w` comparisons. Each `Main*.lean`
file also states the concrete version for the translated certificate
(`select_7th_of_16_le_28` etc.).

## What has to be trusted

* `Selection/Spec.lean`: program format, semantics `exec` with a comparison
  counter, and the predicate `SelectsWithin`.
* The theorem statements in `Selection/Main.lean`, `Main8.lean`, `Main17.lean`.
* The Lean kernel and the three standard axioms `propext`, `Classical.choice`,
  `Quot.sound` (no `native_decide`).

Everything else is checked by Lean: `Checker.lean`, `Count.lean`, `Fast.lean`
and `Sound.lean` prove that acceptance by the checker implies `SelectsWithin`;
`Cert*.lean`, `Hints*.lean` and `Kernel*.lean` are generated from the Rust
certificates by `tools/rs2lean.py` and `tools/gen_kernel.py` and evaluated with
`decide +kernel`. The hints only guide the checker and cannot affect soundness.
The translator only matters for the further claim that the Lean programs are
the archived `.rs` files; this is cross-checked by `tools/run_crosscheck*.sh`.

## Build and check

```
lake build                          # all modules; about 3 min on 8 cores, peak 1.8 GB
lake env lean tests/Axioms.lean     # axioms of the V_7(16) theorems
lake env lean tests/Axioms8.lean    # V_8(16)
lake env lean tests/Axioms17.lean   # n = 17
tools/leancheck_all.sh [MODULES]    # replay in the independent kernel checker leanchecker
lake env lean tests/Mutations.lean  # negative controls (also Mutations8, Mutations17)
```

Translator cross-checks (structural comparison of the Lean data with an
independent parse of the `.rs` file, and Lean `exec` against the reference
interpreter of `code/verifier` on 20,002 inputs). Scratch files go to `../../out/lean`.

```
tools/run_crosscheck.sh             # V_7(16)
tools/run_crosscheck8.sh            # V_8(16)
tools/run_crosscheck17.sh           # n = 17, see the header of the script
```

## Regenerating the data

Only needed if a certificate changes. Re-running reproduces the files byte for
byte; the SHA-256 of the source `.rs` is recorded in the generated files.

```
python3 -I tools/rs2lean.py ../../certificates/V7_16_le28/select_7th_of_16_28cmp.rs \
    --cert Selection/Cert.lean --hints Selection/Hints.lean
python3 -I tools/gen_kernel.py 329 16 6 28        # NFUNS N RANK W [--chunk C]

python3 -I tools/rs2lean.py ../../certificates/V8_16_le29/select_8th_of_16_29cmp_alt_dfpn.rs \
    --cert Selection/Cert8.lean --hints Selection/Hints8.lean --ns Cert8 --parts 100
python3 -I tools/gen_kernel.py 2937 16 7 29 --ns Cert8 --cert-module Selection.Cert8 \
    --hints-module Selection.Hints8 --fast 100 --out Selection/Kernel8.lean
```

For n = 17 and rank K (files `CertKof17.lean`, `HintsKof17.lean`, `KernelKof17.lean`):
`rs2lean.py FILE --ns CertKof17 --parts 100 ...` and
`gen_kernel.py NFUNS 17 K-1 W --ns CertKof17 --cert-module Selection.CertKof17
--hints-module Selection.HintsKof17 --fast 100 --out Selection/KernelKof17.lean`.
