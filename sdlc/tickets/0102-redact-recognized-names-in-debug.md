---
flow: quick-fix
priority: 102
opens: crates/thinkthen/src/core/recognize.rs crates/thinkthen/src/core/records.rs crates/thinkthen/src/cli/recognize.rs crates/thinkthen/src/cli/failure/tests.rs sdlc/ratchet.json sdlc/issues sdlc/records
---

# 0102: Redact recognized names in Debug output

Status: COMPLETE.

Opened as: 2026-10-11. on main after a fresh review accepted it (`sdlc/records/0102-review.md`). Owner: Claude. Record: `sdlc/records/0102-redact-recognized-names-in-debug.md`.

## Outcome

No `Debug` line from `recognize` carries the user's text. This closes `sdlc/issues/closed/2026-09-24-recognized-names-print-their-text-in-debug-output.md`.

## Work

- Give `RecognizedName` a manual `Debug` that withholds the name and keeps the kind, span, and strength. 0088 did the same for `RelationEntity`.
- Cover the sibling types on the same path that derive `Debug` over evidence: `Token`, `TokenInput`, and `Record`.
- Add a recognize test to the shared secrecy suite in `cli/failure/tests.rs` that reads `{:?}` and `{:#?}` of each type.

## Acceptance

The new test fails on main with the planted text in the output and passes after the fix. The full ladder passes with `THINKTHEN_API_KEY` unset.
