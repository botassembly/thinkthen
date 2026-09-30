# 0349: `audit` and `diff` see the batch setting again

Status: ready. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B2. Pays Debt 008, `sdlc/issues/2026-09-30-audit-and-diff-lost-the-batch-setting.md`. Starts after ticket 0344 lands, because both edit `specification/result.schema.json`.

## Outcome

Each detailed row of `decide`, `filter`, `rank`, `choose`, `tag` and `score` carries `meta.batch_setting`: the run's resolved batch setting, `1`, a positive count, or `"max"`. `audit` and `diff` read it, so ADR 0085's mixed-setting warnings and `audit --write`'s batch line work on command output again. The batch receipt that ADR 0111 section 7 removed stays removed.

## Evidence

- Starts from: the debt issue; ADR 0085 items 2 and 3; ADR 0111 section 7, which dropped `meta.batch` from command rows because a row's request depends on which neighbours missed; `specification/result.md`, which still describes `batch` as "library surfaces only, until ADR 0111 slice 3"; `core/result/batch_warning.rs` `BatchSetting::in_results`, which reads `meta.batch.setting`; `cli/asking/row.rs`, which writes `batch_warning`; `crates/thinkthen/tests/backend/audit_write.rs` and `diff.rs`, which plant `meta.batch.setting` by hand because the command no longer writes it. Checked on main `d8018dd96`.
- Keeps: question keys, request bodies, cached answers and the bare output of every verb. `meta.batch_warning` and its stderr line. ADR 0085's rules: a threshold file with no `batch` was tuned at 1, a line with no setting counts as 1 only when no line names another, and mixed settings warn once in ascending order with `max` last. Saved results that carry `meta.batch.setting` still read.
- Changes: `meta.batch_setting` on the six verbs' detailed rows, written before `batch_warning`. `BatchSetting::in_results` reads `meta.batch_setting`, then `meta.batch.setting`. The generated `result.schema.json`, `specification/result.md` (the `batch_setting` row and a corrected `batch` row), ADR 0085 amended in one line, CHANGELOG, ratchet. The tests plant rows the command really prints. The shared result writer adds the member (`public/results/member.rs` builds the same `Run` that carries `batch_warning`), so every surface that returns detailed rows carries `meta.batch_setting`, and the conformance and type expectations that pin full `meta` change on every runner. The builder checks whether any row still writes `meta.batch`, and states the answer in `result.md`.
- Proof: an outside-in test runs the compiled command twice against a counted loopback backend, once at `--batch 1` and once at `--batch max`, saves both `--details` outputs, and pins `audit`'s and `diff`'s exact warning sentences and `audit --write`'s batch line over them. A run at one setting prints no warning. Request bodies and `question_sha256` equal main's for the same run. The schema drift test, `policy.py`, `sdlc/scripts/test`, workspace clippy with `-D warnings`, every conformance runner, `tickets`, and `lint` in a clean checkout.
- Defers: a batch setting in `annotate`, `find`, `recognize` and `relate` rows, which ADR 0085 does not cover.

## What the build taught us
