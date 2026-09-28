# No run-level cost beside the score

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in `~/workspace/experiments/296-gepa-question-tuning/` and `~/workspace/experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

Every `--details` row carries `meta.usage` with the input and output tokens for that answer. `status` counts the current UTC month and the total for the machine. Nothing reports what one run cost, and `audit`'s row is about answers rather than spend.

## Why it matters

The Optimizer gate weighs accuracy against cost: "A tuned wording grows longer and costs more per call. The gate weighs that cost, and every report shows cost beside the score." GEPA's Pareto search takes cost as a second objective, so a loop reads a run's cost for every candidate it scores. Today it must sum `meta.usage` over the rows itself.

## Measured

- Experiment 296, the chained album-year question: the seed took 450 input tokens a call and the winning wording took 630, 40% more, for four gains and no losses on a held-out fifteen. The gate needs both numbers side by side.
- Experiment 296, the knowledge blob: 3,713 input tokens a call for the seed against 2,794 for a compressed blob that held accuracy, 25% less.
- Experiments 296 and 297 together: 989 live requests and 993,889 input tokens, about $0.042.

## What to change

A run total in the machine-readable output: the sum of `meta.usage` over the rows the run printed, and optionally the money at a named price. A new member on the audit row or a small summary line. Additive, and the result contract already allows a release to add a member to a row.
