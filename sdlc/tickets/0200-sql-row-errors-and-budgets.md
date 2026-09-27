---
flow: build
priority: 200
opens: sdlc/tickets/0200-sql-row-errors-and-budgets.md sdlc/planning/adr/0080-sql-row-errors-and-budgets.md databases/sqlite databases/duckdb databases/postgresql specification/settings.md CHANGELOG.md
---

# 0200: Keep good SQL rows and bound their calls

Status: ready for fresh read-only design review. Owner: Codex. ADR 0080 is proposed with this ticket. Ian can overturn its routine choices.

## Outcome and authority

A query can ask for an error value on a bad ThinkThen row, count or route that failure, and still read later good rows. A budget set for SQLite bounds ThinkThen calls across rows without restarting each row's clock. PostgreSQL's native statement timeout remains its query bound. DuckDB can retire idle engine plans while keeping a 16-engine resident cap and cumulative request accounting. PostgreSQL refuses a changed throttle instead of silently ignoring it. This is I2 in `sdlc/planning/work-plan-2026-09-27.md`, authorized ahead of its former ordering on 2026-09-27.

## Evidence

- Starts from: local experiment 284 files 71, 72, 74, and 75, derived from experiment 273 report 04; the 2026-09-27 work plan; ADRs 0017, 0041, 0043, and 0047; and the code read on `origin/main` at `9b8bf076`. No live call was made.
- Keeps: ordinary scalar failures raising; SQL `NULL` as missing input or unresolved answer; the six public error kinds and safe messages; PostgreSQL `statement_timeout` and `thinkthen.deadline_ms`; the 16 resident DuckDB engine cap; one selected throttle per process and one scope per loaded copy; cumulative usage and the 0149/0155 checks before each send and retry.
- Changes: adds one JSON error-as-value form to each SQL extension; adds an explicit connection budget for SQLite; specifies DuckDB's future shared query owner; safely retires idle DuckDB engines without losing their counts; and makes PostgreSQL reject a changed throttle.
- Proof: loopback SQL cases distinguish answered, unresolved, failed, and SQL-`NULL` rows; count later good rows and requests; cross a SQLite budget during a held send; run 17 idle DuckDB plans and 16 held plans; check usage and totals after retirement and fork; and check PostgreSQL's changed throttle, cancel, and native timeout. Run the five gate rungs after implementation.
- Defers: DuckDB's `SET thinkthen_query_budget_ms` implementation, statement-lifetime hook, and stock-host proof to ticket 0201/ADR 0081; full SQLite VM time outside ThinkThen callbacks to the host; and a general error-as-value family for recognize and relate until a use case asks for it.

## Current behavior and the exact defects

- SQLite's scalar guard maps a `Failure` into a SQLite error. PostgreSQL's `call::run` raises a `Refusal` with SQLSTATE. DuckDB's scalar callback sets an error for the chunk. None gives a row failure as a value. DuckDB's existing grouping can let a failed group stop a whole chunk. The existing details scalar gives a JSON `null` value for an unresolved answer, so returning SQL `NULL` on failure would be ambiguous.
- SQLite and DuckDB pass a relative deadline to each call. Each later row starts again. PostgreSQL has `statement_timeout` and a separate call deadline. The current SQLite extension has no statement-begin callback; the current DuckDB C scalar state is per expression, so it cannot own a query-wide clock.
- `databases/duckdb/src/engines.rs` refuses the seventeenth distinct settings plan. `usage_totals()` sums only resident engines, and `within_total()` reads that sum. Dropping an engine without retaining its final counters would make prior sends disappear from the total. A live or detached caller can still change an engine's counters.
- `databases/postgresql/src/call.rs::apply` passes an explicit throttle only when no active throttle exists. A later different value is skipped. Other bindings refuse the conflict through the public engine's active-width check.

## Build sequence and boundaries

1. Review this ticket and ADR before any runtime edit. The design-only commit changes these two files. Expand the lane's file claim for implementation after the review accepts it.
2. Add `thinkthen_try_details` at the three existing SQL doors. Reuse the existing details result and the public `ErrorKind`, message, and retryable signal. Write JSON through a serializer. Preserve the raising scalar functions. Keep host cancel, deadline, and defect fatal. On DuckDB, isolate each distinct row for the try form so one failed batch group does not fail good rows. On PostgreSQL, return ordinary engine errors from an internal worker path while host interrupts still raise; do not catch PostgreSQL errors or turn them into values. A bad row must not run a second query to diagnose itself.
3. Add the SQLite connection budget described by ADR 0080. Set it in a separate statement and share one monotonic expiry with every ThinkThen scalar on that connection. Pass the remaining budget into each call and retain the existing per-call deadline as the tighter bound. Expiry before an attempt sends nothing. Expiry during an attempt returns promptly while that sent attempt remains counted. Document the explicit scope accurately.
4. Make DuckDB's engine registry track recency and historical counters. Retire only an idle least-recently-used `Arc<Engine>` and transfer its counters before dropping it. Make both `usage_totals()` and `within_total()` include historical counts. Do not retire a plan owned by a worker or a scalar state. Keep the cap and provide an actionable refusal when all plans are held. Preserve the existing per-process fork reset.
5. In PostgreSQL, compare a requested throttle with the active width before the builder skips its setter. Accept unset and equal values; refuse a different one using the public active-width sentence. Run this check even if a cached plan exists or an engine rebuild races.
6. Update the SQL pages, `specification/settings.md`, and `CHANGELOG.md` in the implementation commit. Ticket 0201 implements DuckDB's `SET thinkthen_query_budget_ms` and query-begin/query-end hook, then proves the contract in ADR 0080. This ticket must not claim that the current C scalar callback owns a whole DuckDB query.

Ticket 0149's settings work and 0155's per-send retry checks are dependencies only where implementation actually reads their code. If either remains unlanded, stage the code against the current API and merge the later setting or retry work before claiming its behavior. Do not add this scope silently to ticket 0148.

## Cases that must distinguish the behavior

| Case | Expected result |
| --- | --- |
| Good row, bad question row, later good row through `thinkthen_try_details` | Both good rows answer. The middle row has `status: failed`, `kind: usage`. The statement completes. |
| A refused backend reply on one row | That row is failed with `kind: backend` and its retryable flag; later good rows answer. The listener count matches attempts. |
| SQL `NULL` input and an unresolved answer | Input returns SQL `NULL` without a send. Unresolved answer returns an answered JSON object whose details value is JSON `null`. |
| Cancel, passed deadline, or internal defect in the try form | The statement raises the existing host error; no completed failure envelope is returned. |
| SQLite budget of zero, then a positive budget across several rows | Zero sends nothing. The positive clock begins once, does not restart per row, and stops a held second call with a deadline error. The first sent attempt remains counted. A second connection has an independent budget. |
| Seventeen distinct DuckDB plans after each caller releases its state | The seventeenth works, the resident count stays at most 16, and `thinkthen_usage()` still includes all earlier sends and tokens. A request total already spent stays spent. |
| Sixteen DuckDB plans held by active callers | A new plan refuses without eviction or send and names a recovery path. When a caller releases a plan, a new plan can use its slot. |
| PostgreSQL throttle unset, equal, then different | Unset preserves the seeded value. Equal works. Different raises usage naming the active width. `statement_timeout` still raises SQLSTATE `57014`. |

Each new test must protect an outside-in SQL behavior or a count regression that fails before the fix. Do not add test-only exports or a copied implementation inventory. The final implementation runs `sdlc/scripts/{install,lint,test,spec,surfaces}` under the canonical heavy lock. A cold Cargo command outside a rung uses `flock -o`. No paid or live backend call is authorized.

## Stop rules

- Stop if safe retirement needs a new public engine counter API, cannot prove idle ownership, or loses a count after fork. Bring the narrower cap and refusal alternative back to design review; do not land an unsafe LRU.
- Stop if a SQL host cannot give the try form typed failures without parsing an error sentence. Keep that host raising until the typed path is designed.
- Stop if the SQLite budget requires replacing the host's progress handler or promises to stop SQL execution after the last ThinkThen call. Its explicit scope may only promise to bound ThinkThen work.
- Stop if DuckDB 0201 cannot identify one owner shared across expressions and chunks. A per-chunk timer does not meet ADR 0080.
- Stop if a proposed check reruns a paid call, changes the settled ordinary scalar result, or erases requests from the process total.
