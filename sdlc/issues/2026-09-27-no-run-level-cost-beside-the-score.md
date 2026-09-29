# No run-level cost beside the score

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in the workspace at `experiments/296-gepa-question-tuning/` and `experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`. Deferred past 0.1 by the tuning review of 2026-09-28: the need varies, and existing commands cover useful parts of it. Ian can overturn this placement.

## What happens today

A `--details` row carries `meta.usage` only when the backend reported it; for a batch this is an even allocation of request tokens across records, not a measured per-record bill. `status` shows durable command usage by month and total. Opt-in `--facts` reports whole-command tokens and requests, including work omitted from printed filter or rank rows. The separate `cost` transform already totals printed detail rows at a caller-supplied input-token price. `audit` does not put that printed-row cost beside its score.

## Why it matters

The Optimizer gate weighs accuracy against cost: "A tuned wording grows longer and costs more per call. The gate weighs that cost, and every report shows cost beside the score." GEPA's Pareto search takes cost as a second objective, so a loop reads a run's cost for every candidate it scores. Today it must sum `meta.usage` over the rows itself.

## Measured

- Experiment 296, the chained album-year question: the seed took 450 input tokens a call and the winning wording took 630, 40% more, for four gains and no losses on a held-out fifteen. The gate needs both numbers side by side.
- Experiment 296, the knowledge blob: 3,713 input tokens a call for the seed against 2,794 for a compressed blob that held accuracy, 25% less.
- The original running tally for experiments 296 and 297 was 989 live requests and 993,889 input tokens, about $0.042. The retained final corrected ledger reports 1,049 calls and 1,019,902 input tokens, about $0.043; neither is a product acceptance threshold.

## What to change

A score-adjacent total in machine-readable output: the reported `meta.usage` shares over the rows the run printed, and optionally estimated money at a named user price. The later design must keep absent usage unknown, distinguish cached rows, and avoid counting both row shares and their whole-batch `meta.batch.usage`. A new member on the audit row or a small summary line is additive; the result contract already allows a release to add a member to a row. This is the original printed-row criterion, distinct from whole-run `--facts`.

## Factual preparation, 2026-09-28

The “nothing reports” premise predates accepted ticket 0170. At main `e58aceae`, opt-in `--facts` prints one final `thinkthen.run/1` stderr line with whole-run request and token totals; its record count includes accepted rows omitted by filter or rank. Token members are absent if any live reply lacks usage. The original request for a sum over printed `meta.usage` rows remains a distinct, narrower scope and should be ruled on explicitly: it can be useful for score-aligned rows but cannot represent all live work. Money beside the score is still open. A user-supplied price yields an estimate, not actual billing. The broader every-surface-facts issue remains separate. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

## Factual preparation after typed C facts, 2026-09-28

At main `23371cc9`, `cli/facts.rs` still reports the whole command, while `transforms/cost.jq` already computes a separate input-only estimate from printed detail rows, dividing cached from charged rows and listing rows without usage. Neither puts cost in `audit`'s score row. `core/result.rs::Usage::share` and `specification/result.md` establish that per-row batch usage is allocated evenly; `meta.batch.usage` is the same request's whole usage and must not be added again. Accepted 0256 changed safe audit output publication, not cost. Codex-3 now designs 0257 per-case audit on the same `cli/audit.rs` source, so a score-adjacent cost shape should follow that design rather than colliding with its claim. The smallest future saved-row proof compares printed-row shares with whole-run facts when a filtered/rank-trimmed row is omitted, and covers cached, missing-usage and unequal-size batch cases without pricing unknown tokens. Source/API work and an optional output-token price need a reviewed contract; do not infer either from the experimental price. See `sdlc/records/2026-09-28-accounting-after-c-facts.md`.

## Scope note, 2026-09-28

`audit --cases` now carries each result line's `meta.usage` with `usage_scope`, and `status` totals the month. A
caller can count each saved result line's known usage once; tag and annotation cases can repeat that usage. This gives a saved-row subtotal, not necessarily the whole run, and missing usage stays unknown. The existing `--facts` line reports the command's known totals. The measured spread (450 against 630 tokens a call for the
album-year wording, and 3,713 against 2,794 for the blob) remains the case for a run-level line when a project
needs one.
