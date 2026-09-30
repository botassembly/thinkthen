# The PostgreSQL check keeps wall-clock limits that fail under load

Status: closed on 2026-09-30 by ticket 0340. Each routine step keeps its order and count proofs and records its time; a `STEP_within_N_ms` twin checks the limit only under the stress profile.

Kind: debt

Pay when: before the 0.1 release candidate, as a Quick Fix.

Keeping it risks a false red on a busy machine, which teaches builders to rerun reds.

## What happened

At load about 22, one routine run of `databases/postgresql/check.sh` failed `batch_cancel` (256 ms against its 200 ms limit) and `a_small_batch_answers_at_once` (1,263 ms against 90 ms). The next run at load 16 passed all 88 steps. Slice 3e moved `single_cancel`'s limit the same way, which closed `closed/2026-09-30-postgresql-single-cancel-wall-clock-limit-under-load.md`.

## What should happen

Each routine step proves its behavior by order or by request counts. Its time limit becomes a step that runs only under the stress profile, as `single_cancel_within_200_ms` does. The other `within` limits in the check (`single_statement_timeout`, `single_deadline`, `batch_deadline` and the 2,000 ms step) follow the same rule.
