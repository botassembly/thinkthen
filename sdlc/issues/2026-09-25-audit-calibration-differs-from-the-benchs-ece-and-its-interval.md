# audit's calibration differs from the bench's ECE and its interval

Status: Open. Filed on 2026-09-25 by the Beatles Bench owner for Beatles Bench ticket 0011. That ticket moves grading onto `thinkthen audit` only where audit gives the same numbers. This gap keeps one bench measure in Python.

Ian ruled on 2026-09-25 that everything is in 0.1 (`sdlc/planning/one-line-plan-2026-09-25.md`, commit `82336e9a`). Ticket 0131 (`sdlc/tickets/0131-audit-matches-the-bench.md`) settles this issue after ticket 0125 lands.

## What happens

audit pairs `(p, key is yes)` for `decide` and `(top, pick equals key)` for untied `choose` answers. Its interval is a plain percentile bootstrap with SplitMix64. It prints the error, not the bins.

Beatles Bench `ece.tsv` and `calibration.tsv` pair each answer's confidence in its own answer with whether it was right, over every question of a run together (`scripts/score/score.py`, `confidence`). The interval is a bias-corrected bootstrap seeded with "beatles-bench" (`scripts/score/stats.py`, `ece_interval`). `calibration.tsv` prints each bin's count, right answers, and mean confidence.

On the 2026-09-23 Jev run the bench prints ECE 0.0202 (0.0058 to 0.0368) over 1,501 questions. audit prints 0.0215 (0.0169 to 0.0494) for `choose` and 0.0830 for `decide`, one group each.

## Options

1. An option to pair a `decide` answer by its confidence in its own answer, a per-bin line in the output, and a named interval method.
2. No change. The bench keeps its own calibration table.

Recommendation: option 1. Confidence in the answer given is the common reading of calibration. Ian can overturn it.
