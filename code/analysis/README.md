# Analysis scripts

Python 3, standard library only. Run from this directory.

| Script | Paper |
|---|---|
| `test_lb.py local 7 fgsum` | local inequality of Theorem "Φ*" on all 149,598 cases with at most 7 elements (isomorphism classes, unsolved ranks, all incomparable pairs); `test_lb.py values NMAX` checks LB ≤ V on all problems up to NMAX |
| `empty_order.py` | at the empty order, ⌈log2 Φ*⌉ beats the one-sided bound by one comparison for fifteen pairs (n, i), n ≤ 16; it gives only 23 for (16, 8) |
| `corpus/b_gain.py TABLE` | average gap V − LB and share of exact rows (1.13, 10.0%) for LB = max(⌈log2 Φ*⌉, KEY) on the whole corpus (`data/groundtruth_table.tsv`, unpacked; about 8 min); output in `corpus/b_gain.txt`. It also reports a conjectural bound ("C1") that is not part of the paper. The gap 2.62 of ⌈log2 \|C\|⌉ follows directly from the table: `awk -F'\t' '$1>=2 {n++; g+=$3-$6} END {print g/n}' TABLE` |
| `heuristic_accuracy.py` | how often the top-ranked comparison of an ordering (Φ*, compatible solutions, random) is optimal, on exact data with 7 to 10 elements (needs `data/groundtruth_table.tsv`, unpacked) |
| `structure/analyze.py > report.txt` | structure of the certificates: the 15-comparison opening R16, worst paths and tree leaves (248,381,440 for the 252-function V_8(16) certificate) |
| `structure/r16_reach.py` | which certificates pass through R16 |
| `structure/r16_check.py > r16_check.txt` | lower bounds for (R16, i), i = 1..8; rules out ranks 1 to 3 (needs the upstream repository cloned into `external/selection_generator` at the repository root, for the published optimal algorithms) |
| `opening.py` | prints the shape of the active poset along one path of a certificate |

Helper modules: `posets.py` (posets, reduction, canonical forms),
`exact_small.py` (exact solver for small problems), `lb_candidates.py` (the
lower bounds FG, FGδ, Φ*, KEY and LB). `structure/cert.py` is an own parser for
certificates that does not import the verifier. `structure/report.txt`
also covers two older certificates that are not part of this repository.

See the help text at the top of each script for options.
