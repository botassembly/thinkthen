# audit tunes its cut on one half and never swaps

Status: Open. Filed on 2026-09-25 by the Beatles Bench owner for Beatles Bench ticket 0011. That ticket moves grading onto `thinkthen audit` only where audit gives the same numbers. This gap keeps one bench measure in Python.

## What happens

`specification/audit.md`, "The suggested cut": audit tunes the cut on one part and checks it on the other. The parts come from the key's `part` or from a seeded split. The held-out result covers one half of the records.

Beatles Bench `decide.tsv` reports "right at a cut tuned on a held-out half" with two-fold cross-fitting (`scripts/score/stats.py`, `cross_cut`). It splits by the SHA-256 of the id, tunes on half 0 and scores half 1, then swaps. Every record is scored once, held out. The candidate cuts are the distinct probabilities plus 1.01, not a grid of 1 to 99. A tie goes to the cut nearest 0.5.

Audit's one-way result cannot rebuild the swapped count.

## Options

1. A `--folds 2` option: tune on each part, score the other, and print both cuts and the summed held-out counts.
2. No change. Callers run audit twice with swapped `part` values in the key and add the counts. This still leaves the candidate grid different.

Recommendation: option 1, with the candidate set named in the output. Ian can overturn it.
