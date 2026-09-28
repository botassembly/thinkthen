# Audit writes the tuned bar in place, so a search cannot keep the incumbent

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in `~/workspace/experiments/296-gepa-question-tuning/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

`audit --write QUESTIONS` resolves the named question file or set, checks that every graded line carries that file's digest, and writes the steady bar into the file it names. A second `--write` from the old results refuses, because the digest moved with the threshold. The write is in place, and the file it names is the file the run came from.

## Why it matters

A tuning loop compares a candidate against the incumbent, and the incumbent file must survive the round. An in-place write destroys the baseline, and a candidate cannot be written beside it at all. A loop also wants to write several candidates and compare them, which the in-place form cannot express.

## Measured

Experiments 296 and 297. The runner wrote every candidate to a content-addressed file under `work/questions/` and kept the seed and the winner under `runs/<arm>/`. `audit --write` was never called, so the tuned bar was applied with a `--threshold` value at scoring time instead.

## What to change

A flag beside `--write` that writes the tuned file to a named path and leaves the named question file alone. `--write` keeps its present meaning. Additive, and the digest rule stays as it is.

## Factual preparation, 2026-09-28

At main `e58aceae`, `cli/audit/write.rs::bars` checks applicable saved question digests and splices tuned text before `std::fs::write(path, new)`. The original report describes deliberate in-place overwrite, not an observed crash loss. The separate-destination criterion remains open. A design should preserve the incumbent bytes and examine destination aliasing and partial-write failure. This preparation has not chosen an overwrite policy or changed `--write`. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.
