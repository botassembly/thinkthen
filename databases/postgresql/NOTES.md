# Notes on the PostgreSQL port

Ticket 0111 ported this surface from tag `surfaces-wave7-frozen-2026-09-24b` onto the public `thinkthen` API. The tag's `NOTES.md` stays at the tag as history. `sdlc/records/0111-build-postgresql-surface.md` holds the measurements.

## What the port found

- Each `Engine` counts its own sends. The ticket assumed one process total. The binding keeps every engine a backend built and adds their counters in `thinkthen_usage()` with the crate's `Sum` for `Counters` (ticket 0347), so the totals carry across a rebuild.
- `decide_many` streams. A batch over `max_requests` sends its first records before the engine refuses. The binding holds the records, so it checks the count before any send and refuses with the engine's sentence.
- A request total per backend (Ian's ruling of 2026-09-25) needs the counters of every engine, so the binding keeps one engine per settings plan and records it right after the build. A cancelled call's sends then still count.
- The binding raises its own refusals as `thinkthen::Error`, built with `Error::new` (ticket 0347). The `Refusal` copy is gone.
- `relate.rs` parses inline relate rules with the command's parser, `RelationRule::parse_inline` (ticket 0347), and keeps its own refusal sentence.
- `cargo pgrx package` names the extension after the one `.control` file in the folder. The package `thinkthen-postgresql` still ships `thinkthen.so` and `thinkthen--0.1.0.sql`.
- pgrx cannot read a set's columns through a type alias. A `TableIterator` return type is spelled out in each `#[pg_extern]` signature.
- A `SET` of a `thinkthen.*` name before the library loads makes a placeholder, and PostgreSQL checks no range then. The range test loads the library first.
- The loopback backend answers about 41 ms after each request on a reused connection. `curl` sees the same delay, so it sits in the backend, not the engine. A 20,000-row warm at throttle 32 takes 46 to 52 s, over the ticket's 30 s bound. A second warm over the same 20,000 cached rows takes 2.5 s, so the aggregate's text state is not the cause. Main's Quick Fix at `365fc938` sends one write per loopback reply, and a reused-connection reply fell under 0.1 ms. With it, the same warm took 30.05 s, 50 ms over the bound. The engine holds 32 requests in flight at throttle 32. A 2,000-record array batch at throttle 32 sends about 1,140 requests a second, and 200 records at throttle 1 take about 12 ms each. Quick Fix `qf-flaky-gate-tests` retired the 30 s bound. The step now counts 20,000 sends for the cold pass and none for a second pass, and the second pass takes at most half the first (record `sdlc/records/qf-flaky-gate-tests.md`).
- A 20,000-row scalar decide over cached answers takes about 1.05 s, about 52 µs a row, under the ticket's 100 µs stop.

## How the check runs

- `runtime.sh` checks the pinned server package, extracts it once, and compares its version with `/usr/bin/pg_config`. A mismatch or a missing tool reports "not run" with the fetch command.
- Every test gets a fresh server start, a fresh loopback backend, and a fresh cache folder. The start refuses any address whose host is not 127.0.0.1 and gives the server a fake key.
- Each step runs in a subshell under errexit, so its first failing line fails it.
- The legacy conformance runner restarts the server for each case on that case's arm. Filter and rank cases retain indexed SQL projections; find has named SQL functions and runs its cases. Its three declared exclusions remain: `18-annotate-two-groups` records separate requests for one record's groups, which conflicts with [ADR 0111 section 5](../../sdlc/planning/adr/0111-question-cache-and-one-batching-path.md); `23-cancelled-fault` has no SQL token and the host cancellation checks cover it; `25-defect-fault` has no outside input that reaches an invariant defect and the panic probe covers SQLSTATE XX000. The separate installed complete suite checks all ten named functions with typed SQL fields, including annotation groups, cancellation and failure envelopes.

Executing judgment routes share Request admission and native results. Result/1 projections reuse retained native serializers and keep actual batch/context metadata and requested attempt receipts. `thinkthen_plan` retains the shared zero-send diagnostic planner. Saved question authority and the client evidence-reader workaround remain PostgreSQL's existing rules; the Request migration grants no server evidence-file access.
