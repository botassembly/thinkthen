# SQL error-format follow-up

Source-read preparation at main `3af779c8f`. No new runtime proof, independent preparation review or issue closure is claimed. ADR 0105 section 8 already settles the outcome. The [SQL usability issue](../issues/2026-09-28-sql-interface-usability-before-0-1.md) retains this criterion; register 117's catalog follows the runtime format.

Ordinary ThinkThen SQL failures use `thinkthen <kind>: <message> (retryable: yes|no)`. The shared corpus's exact plain removal messages remain exceptions. Host-native SQL syntax and binding diagnostics remain the host's errors. Keep the existing six kinds, SQLSTATE and SQLite code maps, structured try-details fields, and secret-safe messages.

## Ownership and actual paths

DuckDB 0286 owns its correction now. Review found new plan and settings errors without the suffix. `bridge/src/errors.rs` has typed kind and retryability; direct C++ product exceptions need the same disposition. Reuse the existing failure boundary and preserve the exact removal stubs. The builder must prove one ordinary usage refusal, representative backend retryability, and a plain removal through the installed host.

PostgreSQL 0285 must retain its current formatter. `databases/postgresql/src/call.rs::Refusal::message` already prints the accepted suffix, and its existing six-kind table preserves SQLSTATE behavior. Do not copy DuckDB's implementation into it. New removal stubs need the explicitly accepted plain form.

SQLite needs a separately claimed Quick Fix after its landed 0284 settings/keyed-call work. `databases/sqlite/src/lib.rs::Failure` already carries kind, message and retryability. `From<Failure> for rusqlite::Error` currently inserts ` (retryable)` before the colon only when true; false has no suffix. This is the actual formatting boundary. The existing kind/code table and retryable-backend assertion in the same file provide a compact place to pin the exact format. Keep `Failure::value`'s structured JSON fields unchanged. Give removal stubs an explicit plain-message path; do not infer that exception by searching human prose for words such as “removed.” Claim the actual removal producers under `src/scalars.rs`, its subordinate modules, and `src/tables.rs` as needed after locating them.

SQLite's selected tests with exact strings include `test_settings.py`, `test_values.py`, `test_try_budget.py`, and the new keyed/plan cases. Update real contract expectations without weakening their kind, code, send-count or canary checks. Reuse the existing `Backend`, child isolation and pinned SQLite 3.50 host. A small host table should pin usage/no retry, one retryable backend failure, and unchanged plain removal with bounded exact arrivals. Retain the pure six-kind/code table; do not run a full package or duplicate it in every test file. Regenerate measured Rust/Python counters and run focused format, Clippy and policy before fresh review.

The last SQLite source artifact was checked with 0299's shared constructor. Any subsequent error-format receipt must name its new source and artifact; do not relabel the earlier archive or settings messages as current. This follow-up is a ready scope for the next SQL author, not an active file claim.

Follow-up from DuckDB High review `902dd512a`: a caller's query error forged an internal-looking prefix and retryability suffix. SQLite already owns typed kind/retryability in `Failure`; format from those fields. Never infer either field from `message` or treat a matching suffix as trusted. Preserve the explicit plain-removal exception. See the [friction record](2026-09-27-ticket-friction-since-1300.md#duckdb-retryability-must-carry-provenance).


## Current Quick Fix refresh

The author confirmed the formatter defect on main369488d5b. A separate genuine removal producer is `src/question.rs::call_settings`, which refuses the old numeric deadline slot. The coordinator explicitly includes it in the formatting claim. `tests/test_deadline.py` still exercises that superseded positional API and a strict0.3-second elapsed bound; current `test_settings.py` already proves its plain migration sentence. The same Quick Fix updates the deadline fixture to accepted settings while retaining zero-budget refusal, a held-call deadline and exact bounded retry counts. Remove the brittle speed threshold; use owned held work and a bounded harness to prove the functional stop. This is a test-contract correction alongside the formatter, not a new deadline API or a performance campaign. Keep the original failed receipt and avoid duplicating the existing migration proof.


Fresh Medium review of8eda286ae passed the installed format and deadline witnesses but found `tests/conformance.py` still parsing the former prefix retry marker. It interprets new yes, new no and absent suffix as false, so the shared checker no longer proves retryability or format. The coordinator includes that reader in the same correction. Inventory consumers of changed error text, including shared conformance helpers, rather than only tests with exact expected strings. Preserve the kind and actual send assertions, distinguish yes/no explicitly, and refuse a missing required suffix. Use small planted parser cases or selected shared error cases; the installed runtime is unchanged.
