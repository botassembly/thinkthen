# Audit shows group counts and no per-case evidence

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in `~/workspace/experiments/296-gepa-question-tuning/` and `~/workspace/experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

`audit` prints one JSON row per group: counts, agreement with its interval, precision, recall, f1, AUC, calibration, the coverage curve, and the suggested cut. `--table` prints the group lines and the coverage table. Neither view names a case. A reader learns that six of sixty answers were wrong, never which six and never why.

A `--details` row already carries everything needed: `input`, `question`, `answer`, `threshold`, and `meta.usage`. `--id` and the key join a row to its label. The missing piece is a view that makes that join.

## Why it matters

The tuning loop's proposer reads per-case failures. The input, what the model said, the truth, and the probability are the entire input to a wording change. Without a per-case view, every consumer of audit restates audit's arithmetic.

Experiments 296 and 297 did exactly that. The experiment's `scoring.py` restates the yes/no cut grid, the right rule, AUC, and the Wilson interval; the evaluator joins rows with the key and builds the proposer's line itself. A restatement can drift from the shipped command, and the shipped `audit` and `diff` become an after-the-fact check instead of the measurement in the loop.

## Measured

- Experiment 296: the shipped `audit` and `diff` agree with the experiment's own scorer on the held-out test split. `diff` reads `gained 3, lost 0 (4 -> 7 right of 15); McNemar p 0.250 on right answers`. The loop still could not use them, because the groups it prints are too coarse for the proposer.
- The whole line of work cost 989 live calls and about $0.042, and a few hundred lines of Python on top of ThinkThen for evidence and scoring.

## What to change

One additive option or subcommand that prints one row per case: id, question text, said, truth, right or wrong, the probabilities, and the tokens. A `--cases` flag on `audit` is the smallest form. The current group output stays the default, and no exit code changes.

A per-case member on the existing group row is not enough: one group holds many cases, and the row has no place for them.
