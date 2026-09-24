# Answers still print record labels under Debug

Status: open. Found 2026-09-24 by the review of Quick Fix `qf-debug-withholds` (`sdlc/records/qf-debug-withholds.md`). Owner: Claude.

`choose --options` reads its labels from the record, and a record is evidence. That Quick Fix withheld the labels in `Labels`, `Question`, and `Request`. The answer still carries them.

- `core/answer.rs`: `Answer`, its inner `Shape` (`pick`, `level`), `Distribution`, `TagProbabilities`, and `Value::Choice` derive `Debug`. Each holds the picked label or every label with its odds.
- `core/result.rs`: `DecisionResult` derives `Debug` and holds the same names.

No log or error prints these today, and no secrecy test reads them. The fix withholds the names under `Debug` and adds each type to `no_record_label_or_request_debug_line_shows_the_evidence` in `crates/thinkthen/src/cli/failure/tests.rs`.
