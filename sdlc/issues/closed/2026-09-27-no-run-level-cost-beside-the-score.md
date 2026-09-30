# No cost beside the audit score

Status: Closed on 2026-09-30. Merged into `../2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md`, part 3, deferred past 0.1.

Priority: rank 24 of 25 in `../planning/issue-priorities-2026-09-30.md`. Owner: a future ticket after 0.1.

## What exists

- `--facts` prints whole-run requests and tokens, including rows `filter` drops and `rank --top` cuts (ticket 0170). With caller prices set, it adds `estimated_cost_usd` (ticket 0300, ADR 0108).
- `audit --cases` carries each result line's `meta.usage` with `usage_scope`.
- The `cost` transform totals printed detail rows at a caller-supplied input price.

## What is missing

`audit` puts no cost beside its score. A tuning loop weighs accuracy against cost for every candidate, so it must sum `meta.usage` over the rows itself. The ask is a score-adjacent total in machine-readable output: the reported `meta.usage` shares over the printed rows, and optionally money at a named caller price. The design must keep absent usage unknown, tell cached rows apart, and never add row shares to their whole-batch `meta.batch.usage`. A printed-row subtotal cannot stand for all live work, so it is distinct from `--facts`.

## Measured

- Experiment 296, the chained album-year question: the seed took 450 input tokens a call and the winning wording 630, 40% more, for four gains and no losses on a held-out fifteen.
- Experiment 296, the knowledge blob: 3,713 input tokens a call for the seed against 2,794 for a compressed blob that held accuracy, 25% less.
