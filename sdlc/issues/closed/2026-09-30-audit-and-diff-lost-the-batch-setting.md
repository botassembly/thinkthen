# `audit` and `diff` lost the batch setting

Status: closed 2026-09-30 by ticket 0349. Deferred by ticket 0304 slice 2 (ticket, line 84). Owner was ticket 0349, batch B2 of `../../planning/issue-priorities-2026-09-30.md`.

Resolution: paid by ticket 0349. Each detailed record row of `decide`, `filter`, `rank`, `choose`, `tag` and `score` carries `meta.batch_setting`, on the command and on the library's batch rows. `audit` and `diff` read it before an older row's `meta.batch.setting`, so ADR 0085's mixed-setting warnings and `audit --write`'s batch line work on command output again. `tests/backend/batching/audited.rs` proves it over two real runs at `--batch 1` and `--batch max`.

Kind: debt

Pay when: before 0.1, or when 0304 slice 4 or 5 touches the run facts.

Debt: 008

Severity: medium

Paid: 2026-09-30

Keeping it silently drops ADR 0085's batch warning, so a threshold tuned at one batch setting can be reused at another with no warning.

## What happens

`audit` and `diff` read `meta.batch.setting`. Since 0304 slice 2, command rows no longer carry it, so both treat the setting as unknown. `audit --write` leaves `batch` alone, and neither warns. `tests/audit_write.rs` and `tests/diff.rs` pin that behavior.

## What should happen

Give `audit` and `diff` the batch setting another way, then restore ADR 0085's warning.
