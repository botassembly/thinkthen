# DuckDB try_details raises on a spent request total

Status: confirmed on the installed 0222 Linux artifact after a bounded independent re-audit. Severity 2; resolve before public 0.1. [Ticket 0240](../tickets/0240-duckdb-try-budget-values.md) and its [preflight](../records/0240-duckdb-try-budget-preflight.md) now propose the partial-outcome correction for fresh High design review. No runtime fix or new installed artifact is claimed yet.

## Expected behavior

Accepted ADR 0080 says `thinkthen_try_details` converts a spent process request total into a safe failed Usage value. Ticket 0149 maps both `BeforeFirstSend` and `BeforeRetry` markers to that host spent-total form and preserves its try-value shape. Cancellation, deadline and defect remain statement errors. Ordinary scalar and warm calls still raise on a spent total. No path may start an extra transport attempt.

## Observed regression and evidence

The 0222 follow-up review incorrectly required every tagged denial to remain fatal at the SQL try boundary; its reviewer withdrew that fatal-budget acceptance after the re-audit. Accepted source `e7d57bc0`, landed at `73932ef4`, adds `send_budget_denial().is_none()` to the recoverable outer-error branch in `databases/duckdb/bridge/src/ffi/scalar/ffi.rs`. A denial now reaches the exception path. ADR 0094 lists cancellation, deadline and defect as fatal and does not explicitly amend ADR 0080's spent-total rule. A new test and a review finding do not amend that accepted rule.

The original `tools/verbs_suite.py::try_details_keeps_unresolved_and_spent_total_distinct`, introduced by `02782ef3` and unchanged, expects an answered unresolved value followed by safe failed Usage with exactly one listener send. The reviewer reran it against the final 0222 Linux artifact: it fails with the spent-total exception. The same test passes against intermediate artifact `8546ad51`, before the fatal-guard correction. The selected 0222 acceptance cases had omitted this older contract test. The README now contains both a recoverable Usage promise and a denied-vector exception promise. Preserve the historical evidence and correct the conflicting final record.

## Correction boundaries

Map both typed denial variants to safe Usage at the explicit try boundary; never let `BeforeRetry` become an ordinary safe Backend failure simply because its underlying public kind is Backend. Preserve actual-attempt enforcement and joined accounting. Keep cancellation, deadlines and defects fatal, and preserve normal scalar/warm refusal. Reconcile packed results too: the current native outer error can discard already answered members in the same call. A blanket replacement of those completed answers with failed values is not a sufficient correction without satisfying ADR 0094's per-request/member preservation. The design must name the smallest way to retain completed answers and represent denied or unstarted rows while preventing new attempts.

Use the original safe-value test, one bounded packed prior-success case and one denied-retry case. Count actual listener requests and pin returned JSON, safe wording and facts. Reuse selected installed Linux and Apple Silicon proof; do not rerun a full matrix or provider campaign. Record source and artifact identities in 0240's preflight. B13c's other platform gaps remain open.
