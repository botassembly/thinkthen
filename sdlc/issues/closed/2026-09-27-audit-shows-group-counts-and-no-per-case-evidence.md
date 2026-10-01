# Audit shows group counts and no per-case evidence

Status: closed by ticket0257 after fresh Medium code review accepted `5ce3d270`. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in local experiment 296 and local experiment 297; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

`audit` prints one JSON row per group: counts, agreement with its interval, precision, recall, f1, AUC, calibration, the coverage curve, and the suggested cut. `--table` prints the group lines and the coverage table. Neither view names a case. A reader learns that six of sixty answers were wrong, never which six and never why.

A saved `--details` row can carry `input`, `question`, `answer`, and `threshold`; `meta.usage` is present only when usage is known. `--id` and the key join a row to its label. The missing piece is a view that makes that join and exposes unavailable row usage explicitly.

## Why it matters

The tuning loop's proposer reads per-case failures. The input, what the model said, the truth, and the probability are the entire input to a wording change. Without a per-case view, every consumer of audit restates audit's arithmetic.

Experiments 296 and 297 did exactly that. The experiment's `scoring.py` restates the yes/no cut grid, the right rule, AUC, and the Wilson interval; the evaluator joins rows with the key and builds the proposer's line itself. A restatement can drift from the shipped command, and the shipped `audit` and `diff` become an after-the-fact check instead of the measurement in the loop.

## Measured

- Experiment 296: the shipped `audit` and `diff` agree with the experiment's own scorer on the held-out test split. `diff` reads `gained 3, lost 0 (4 -> 7 right of 15); McNemar p 0.250 on right answers`. The loop still could not use them, because the groups it prints are too coarse for the proposer.
- The whole line of work cost 989 live calls and about $0.042, and a few hundred lines of Python on top of ThinkThen for evidence and scoring.

## What to change

One additive option or subcommand that prints one row per case: id, question text, said, truth, outcome, probabilities, and tokens when the saved row supplies them. The later design must represent unavailable usage as absent or null, never zero or a share inferred from whole-run `--facts`. A `--cases` flag on `audit` is the smallest form. The current group output stays the default, and no exit code changes.

A per-case member on the existing group row is not enough: one group holds many cases, and the row has no place for them.

## Factual preparation, 2026-09-28

At main `e58aceae`, `cli/audit.rs::grade_all` reads details and a separate key, while `core/measure/{answer,key}.rs` supplies identity and grading primitives. The detail row alone does not supply truth; the key join does. The command still emits aggregate groups, while `diff` emits changed cases only. A per-case view remains distinct. Reuse the existing join and refusal rules, and prove labeled, unlabeled, failed and duplicate-identity edges without new sends. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

## Added 2026-09-28: the reading, and what the case row must carry

The follow-up work after `notes/Autorubric cookbook.md` and `~/foss/awesome-evals` showed that a loop needs the shape of each miss, not only right or wrong. Two free measurements from saved answers, in local experiment 297:

- The chained album-year question misses 38 of 60 cases. The model is one year off on fifteen and two years off on eleven, in both directions, and the song's own year equals the album's year on every case. The shape is noisy recall, not a wording problem.
- On the lead-set questions for John, accuracy reads 0.462 while the true positive rate is 0.368: the question holds nineteen yeses in twenty-six, and the model misses most of them.

A per-case view should carry enough to cluster by shape: the answer, the truth, the probabilities, the options in their sent order, and the case's kind. `audit` already prints precision, recall, f1, and both disagreement directions per group. The case list wants the same counts, and the evals literature names the rule: report TPR and TNR separately, because accuracy hides a rare class.

Evidence: local experiment 297's `LESSONS.md` sections 11 to 13, `pipeline.py`, and `runs/pipeline/`.

## Factual preparation refresh, 2026-09-28

At source `9b766cf9`, `cli/audit.rs::grade_all` already joins numbered result lines to the key through `core/measure/{answer,key}.rs`, then emits group rows from `grade::audit`. `diff` emits changed cases only. Accepted ticket 0256 adds a different saved audit output and is still building; it supplies no per-case evidence view. The original additive case-row criterion remains open. Raw details can carry input, ordered options and `meta.usage`, but parsed `Answer` does not retain every raw field, and usage can be absent. `--facts` is whole-run, not per-case tokens. A case view must reuse key truth and existing grading/refusal rules, distinguish failed/unlabeled/tied/unresolved, and prove its row outcomes against the existing aggregate on a small saved fixture with a duplicate identity refusal. The original 989-call/$0.042 account is retained above as history; the corrected final experiment 296+297 ledger in local experiment 297's `LESSONS.md` reports 1,049 calls, 1,019,902 input tokens and about $0.043. Neither total is a per-case cost. See `sdlc/records/2026-09-28-tuning-evidence-refresh.md`.

## Resolution

Ticket0257 adds the read-only `audit --cases` view while preserving the aggregate default. Complete literal rows prove identity, input, question and sent options when saved, answer, keyed truth, probabilities, distinct outcomes, set counts and explicitly scoped available usage. Aggregate parity covers all four yes/no directions. The existing parser/join/refusals and no-send behavior are retained. [The build record](../../records/0257-audit-case-build.md) records exact proof and limits; uncertain-case selection, repeated judgments and new cost estimates remain separate issues.
