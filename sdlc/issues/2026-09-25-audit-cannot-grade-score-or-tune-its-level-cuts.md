# audit cannot grade score or tune its level cuts

Status: Open

Found 2026-09-25 while comparing audit with DSPy 3.4's ReAnchor optimizer for the talk. The comparison is Ian's inbox report `notes/reports/2026-09-25-dspy-jev-support-vs-audit-and-diff.md` in the workspace.

## What happens

Here `score` means the ThinkThen function that rates a text on a ladder, such as low, medium, high. It does not mean a metric such as F1. Jev returns a probability for each rung. A level cut is the point where one rung ends and the next begins. It works like the yes/no bar, with one bar between each pair of rungs.


`audit` grades `decide` and `choose` answers and refuses any other verb ([audit.md](../../specification/audit.md), "The verb"). For `decide` it suggests one cut. For `choose` it suggests a cut that reaches a target agreement. It has no answer for `score`, and no way to lean a `choose` toward one option.

ReAnchor fits three settings from a labeled key with no new model calls: a yes/no cut, the cuts between `score` levels, and a weight on each `choose` option. On a three-level email task, moving the level cuts from 0.5/1.5 to 0.1/1.1 raised right answers from 239 to 294 of 336, per the DSPy post the report cites. ThinkThen has no way to find or apply level cuts.

## Why it matters

A `score` user with a labeled key has no tool to check it or tune it. The talk shows audit tuning a yes/no bar. A viewer who uses `score` will ask for the same.

## Options

1. `audit` grades `score` answers and suggests level cuts over the expected level, using the same tune and held parts. The question file carries the cuts beside the yes/no cut (question-file rule 8).
2. Also suggest per-option weights for `choose`. This changes how `choose` picks and needs an ADR.
3. Leave `score` out and say so in `audit --help`.

The recommendation is 1, after the two open audit issues of 2026-09-25 land, since it reuses their metric choice and spread report. Ian can overturn this.
