# A run cannot be repeated on purpose, so a loop cannot measure its own noise

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in `~/workspace/experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

The cache answers a repeated request for free, so asking the same question again through the cache returns the same bytes. `--no-cache` forces every request live, but no command repeats a record a fixed number of times, and no command reports how the answers varied. `audit` reads two saved runs or one run at two cuts; it has no repeat input.

## Why it matters

Jev's probabilities move between identical requests. A loop that compares one run against another cannot separate a wording effect from rerun noise, and a model's lean can look like a wording win. Experiment 249 met this first and paired every variant with a fresh control run by hand. Experiment 297 did the same with a custom script.

## Measured

Local experiment 297, `runs/noise-summary.json`. Fifty `multi-hop` same-month yes/no cases, three fresh answers each, 150 live requests:

- One case changed its answer across the three runs, and it sat at p 0.49 to 0.52.
- Probability spread across repeats: median 0.020, 90th percentile 0.040, maximum 0.070.
- Mean accuracy at the run cut per repeat: 0.527.

The answers are stable and the scale moves a couple of points. The only real wobble is a case sitting on the cut, which is also the case a label buys the most.

## What to change

`--repeat N` on `decide`, `filter`, `rank`, and `choose`, default 1. Each repeat sends a fresh request and prints its own row or its own share, so a script can count flips and see the spread. Repeats must bypass the cache or the option measures nothing.

A smaller alternative is a stability section in `audit` over a repeated run. The option is the more direct form, it is additive, and a default of 1 changes no existing run.
