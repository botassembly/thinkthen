# audit reports coverage at one rule, not along every confidence

Status: Open. Filed on 2026-09-25 by the Beatles Bench owner for Beatles Bench ticket 0011. That ticket moves grading onto `thinkthen audit` only where audit gives the same numbers. This gap keeps one bench measure in Python.

## What happens

audit's `coverage` is `answered / n` under one rule. Its `--table` prints a grid of cuts in steps of 0.05.

Beatles Bench `coverage.tsv` takes every distinct confidence as a cut, from the highest down, and prints answered and right at each (`scripts/score/stats.py`, `coverage`). A fixed grid cannot rebuild those rows.

## Options

1. An option to print the coverage curve at each distinct confidence as JSON lines.
2. No change. The bench keeps its own curve.

Recommendation: option 1. Ian can overturn it.
