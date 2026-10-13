# Exact values of small selection problems

`groundtruth_table.tsv.xz` (unpack with `xz -dk`): 1,663,508 reduced selection
problems with at most 12 elements and their exact values, produced by
`code/core` `exact 12 6` (all values were made exact in a second pass). It is
the corpus used in the paper for the gap statistics of the lower bounds and
for fitting the cost predictor of the df-pn search. It contains the problems
the solver visited, not all problems up to 12 elements.

Columns (tab-separated): n, i (1-based), V, the poset as comma-separated hex
masks of the elements above each element, |C| (number of compatible
solutions), ceil(log2 |C|), `exact`, phase (1 or 2).
