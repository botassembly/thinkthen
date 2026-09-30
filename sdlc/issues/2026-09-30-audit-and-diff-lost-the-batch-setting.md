# `audit` and `diff` lost the batch setting

Status: open. Deferred by ticket 0304 slice 2 (ticket, line 84). Owner: ticket 0304 slice 5, which rewrites the run facts; otherwise its own Quick Fix after 0304 slice 4.

Kind: debt

Pay when: before 0.1, or when 0304 slice 4 or 5 touches the run facts.

Keeping it silently drops ADR 0085's batch warning, so a threshold tuned at one batch setting can be reused at another with no warning.

## What happens

`audit` and `diff` read `meta.batch.setting`. Since 0304 slice 2, command rows no longer carry it, so both treat the setting as unknown. `audit --write` leaves `batch` alone, and neither warns. `tests/audit_write.rs` and `tests/diff.rs` pin that behavior.

## What should happen

Give `audit` and `diff` the batch setting another way, then restore ADR 0085's warning.
