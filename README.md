# Selecting the 7th smallest of 16 elements takes exactly 28 comparisons

How many comparisons does it take to find the $i$-th smallest of $n$ elements
in the worst case? For 16 elements, the answer was known for every rank except
the 7th and the 8th. This repository contains comparison algorithms that find
the 7th smallest of 16 elements with 28 comparisons and the 8th smallest with 29.
Together with a published lower bound, this settles $V_7(16) = 28$. It also
contains new bounds for 17 elements. Each algorithm is an explicit decision
graph (a *certificate*), and every bound is checked three ways: by an
independent static checker, by exhaustive and randomized execution, and by a
formal proof in Lean 4.

> **Paper:** Jonathan Cremer, *Selecting the 7th Smallest of 16 Elements Takes
> Exactly 28 Comparisons*, 2026. [link to be added]

## Results

$V_i(n)$ denotes the minimum worst-case number of comparisons needed to find
the $i$-th smallest of $n$ distinct elements. Since $V_i(n) = V_{n+1-i}(n)$,
the tables list $i \le \lceil n/2 \rceil$ only.

| Theorem | Statement | Previously |
|:--|:--|:--|
| 1 | $V_7(16) = V_{10}(16) = 28$ | 28–30 |
| 2 | $V_8(16) = V_9(16) \le 29$, so $V_8(16) \in \lbrace 28, 29\rbrace$ | 28–31 |
| 3 | $V_5(17) = V_{13}(17) = 27$ | 27–28 |
| 3 | $V_6(17) \le 29,\ V_7(17) \le 30,\ V_8(17) \le 31,\ V_9(17) \le 31$ | 31, 33, 34, 35 |

**Row $n = 16$**

| $i$ | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| before | 15 | 18 | 21 | 24 | 26 | 27 | 28–30 | 28–31 |
| **now** | 15 | 18 | 21 | 24 | 26 | 27 | **28** | **28–29** |

**Row $n = 17$**

| $i$ | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| before | 16 | 20 | 22 | 25 | 27–28 | 28–31 | 29–33 | 29–34 | 29–35 |
| **now** | 16 | 20 | 22 | 25 | **27** | 28–**29** | 29–**30** | 29–**31** | 29–**31** |

> [!NOTE]
> This repository proves the **upper bounds**. The lower bound
> $V_7(16) \ge 28$ is the computational result of Dörrer et al.,
> *Exact Lower Bounds for the Number of Comparisons in Selection*, SEA 2025
> ([doi:10.4230/LIPIcs.SEA.2025.16](https://doi.org/10.4230/LIPIcs.SEA.2025.16)).
> $V_8(16) \ge 28$ and $V_5(17) \ge 27$ follow from their values
> $V_8(15) = 27$ and $V_5(16) = 26$ via $V_i(n+1) \ge V_i(n) + 1$
> (their Lemma 1). These lower bounds are not recomputed here.

### How the bounds are verified

| Check | What it establishes | Where |
|:--|:--|:--|
| Static checker | Each certificate selects the right element on every input within the stated worst case. Written from the definitions; shares no code with the search. | [`code/verifier`](code/verifier) |
| Exact path oracle | An independent re-implementation follows every path and gives the same worst case. | [`code/independent_checks`](code/independent_checks) |
| Random execution | Each certificate compiled with `rustc` and run on $2\cdot10^7$ random permutations. | [`code/randtest`](code/randtest) |
| Lean 4 proof | `∃ P, SelectsWithin P n r w` for all seven bounds, checked by the kernel and by `leanchecker`; standard axioms only. | [`code/lean`](code/lean) |

## Quick start

The first three checks need only Python 3 and take a few seconds:

```bash
# 1. checksums of the certificates
for d in certificates/*/; do (cd "$d" && sha256sum -c SHA256SUMS); done

# 2. static checker (--expect-i is 1-based)
python3 -I code/verifier/verify_rs_algorithm.py \
    certificates/V7_16_le28/select_7th_of_16_28cmp.rs \
    --expect-n 16 --expect-i 7 --expect-worst 28

# 3. independent exact path oracle, all nine certificates
(cd code/independent_checks/checker && python3 -I pathcheck.py ../../../certificates/*/*.rs)
```

With Rust and Lean installed (see below):

```bash
# 4. compiled execution on 2*10^7 random permutations (needs rustc)
code/randtest/build_and_run.sh certificates/V7_16_le28/select_7th_of_16_28cmp.rs 20000000

# 5. Lean proofs of all seven bounds (about 3 min, 1.8 GB RAM)
cd code/lean && lake build && lake env lean tests/Axioms.lean

# 6. rediscover the 28-comparison algorithm (about 5 s, byte-identical to the certificate)
cd code/core && cargo build --release && target/release/dfpn 16 7 28 --out=found.rs
```

## Installation

| Tool | Needed for | Install |
|:--|:--|:--|
| Python 3 (tested with 3.12) | checker, oracle, analysis | standard library only, no packages |
| Rust ≥ 1.75 (`cargo`, `rustc`) | random execution, search | [rustup.rs](https://rustup.rs) |
| Lean 4 via `elan` | formal proofs | [elan](https://github.com/leanprover/elan); `lake build` fetches the pinned toolchain v4.34.1 automatically, no Mathlib |
| g++ | exhaustive check of the lower bounds | any C++17 compiler |

The Rust crates have no dependencies outside the standard library and are
built with `-C target-cpu=native`. Some optional scripts need the published
algorithms of Dörrer et al.: clone
[JGDoerrer/selection_generator](https://github.com/JGDoerrer/selection_generator)
into `external/selection_generator` (ignored by git).

## Repository layout

| Path | Contents |
|:--|:--|
| [`certificates/`](certificates) | the 9 certificates (7 bounds; two have a second certificate), checksums and check outputs |
| [`code/verifier/`](code/verifier) | static checker `verify_rs_algorithm.py`, its soundness proof (README) and test suite |
| [`code/randtest/`](code/randtest) | compiled execution test on random permutations |
| [`code/lean/`](code/lean) | Lean 4 development proving all seven bounds |
| [`code/core/`](code/core), [`code/core32/`](code/core32) | search: df-pn construction, exact solver, lower bounds ($n \le 16$ and $n \le 19$) |
| [`code/upstream/`](code/upstream) | patches to the search code of Dörrer et al. (fixed budget, information-greedy ordering) |
| [`code/analysis/`](code/analysis) | scripts behind the lower-bound evaluation and the structure of the 28-comparison algorithm |
| [`code/independent_checks/`](code/independent_checks) | independent path oracle and mutation tests for the checker; exhaustive check of the lower bounds for $n \le 9$ |
| [`data/`](data) | exact values of 1,663,508 small selection problems (corpus used in the paper) |

Each folder has its own README with details and the exact commands.

## License

[CC BY 4.0](LICENSE). The patch files in `code/upstream/` modify MIT-licensed
code by Josua Doerrer; see [`UPSTREAM_LICENSE`](code/upstream/UPSTREAM_LICENSE).

## Citation

If you use this work, please cite the paper (reference to be added).

## Acknowledgments

We thank J. Dörrer, K. Gendle, J. Betz, J. von Smercek, A. Steding and
F. Stober for publishing their search code and optimal algorithms.
