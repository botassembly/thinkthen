# ADR 0080: SQL row errors and budgets

Date: 2026-09-27. Status: accepted 2026-09-27 by the Codex queue owner after fresh review for ticket 0200. Ian can overturn every decision here.

## Context

The three SQL extensions raise a scalar failure as a statement error. A caller cannot keep good rows when one row has a bad question or a refused backend reply. SQL `NULL` already means a missing input or an unresolved answer, so using it for a failure would erase a settled distinction in the result contract. A call deadline starts afresh for each call. PostgreSQL also has native `statement_timeout`; the other hosts need an explicit bound for ThinkThen work across calls. DuckDB keeps at most 16 engine plans. Each engine owns its counters, which the process request total reads. PostgreSQL currently skips a changed throttle value while rebuilding an engine.

## Decisions

### One value form for a row failure

Each extension adds `thinkthen_try_details(question, evidence)` with the same optional per-call deadline as its existing `thinkthen_details` where that overload exists. A non-`NULL` result is a JSON object with exactly one of these shapes:

```json
{"status":"answered","details":{"value":true}}
{"status":"failed","error":{"kind":"backend","message":"the backend refused the call","retryable":true}}
```

`details` is the existing full `thinkthen.result/1` details document, shown abbreviated above. The binding writes the envelope with JSON serialization, never string concatenation. A SQL `NULL` question or evidence returns SQL `NULL` before reading settings, opening a file, parsing a question, or sending. An unresolved answer is an answered envelope whose details value is JSON `null`. A row failure has `status: "failed"` and no `details`. Its error uses the public `ErrorKind`, a deterministic safe advice sentence, and the typed retryable flag. It never copies `Error::detail().message()`, a binding `Refusal` message, a backend body, or a DuckDB exception into the JSON value. Usage advice says to check the row's question and arguments, or to raise the process request total when that typed limit is spent. Local advice says to check the named file and its permissions without repeating its path. Backend advice says the backend did not answer and whether retry is allowed. Neither the error value nor an engine-cap refusal names a key, evidence, raw question, `@file` path, cache path, or backend address. The private `BackendFailureCause` does not become a new public API.

The value form catches per-row usage, backend, and local failures, including argument read and parse failures and a spent process request total. It leaves host interrupt, cancellation, a passed call or query deadline, and a defect as statement errors. These stops and internal faults must not masquerade as a completed query. The existing scalar names and SQLSTATE or SQLite result-code mapping keep raising exactly as before, including their current diagnostic text. `try_details` evaluates each distinct row independently where a host's existing batch would otherwise fail a whole group. It may cost more calls than a batched normal scalar; the existing cache and retry policy still apply. PostgreSQL's existing `Given::read`, `Given::parse`, and `call::read` currently call `or_raise` before the worker; the try path needs typed results from those stages and from the worker. It checks SQL `NULL` first. It never catches a PostgreSQL host error to construct a value.

### A time budget has one owner and one expiry

PostgreSQL keeps native `statement_timeout` as its statement bound and `thinkthen.deadline_ms` as its per-call bound. The extension does not register a second statement timer. A cancellation or timeout remains SQLSTATE `57014`, including inside `try_details`.

SQLite adds a connection-scoped `thinkthen_budget_ms(n)` setting. A caller sets it in a separate SQL statement immediately before the query. A positive `n` starts one monotonic expiry; `0` is spent; `-1` clears it; every other negative or unrepresentable value is usage. Each ThinkThen call on that connection checks the remaining budget before a send and passes the lesser of it and the call's deadline to `CallOptions`. A budget that expires during a blocking send returns a deadline error promptly; the already sent attempt finishes and stays counted. The expiry is not restarted per row, and it stays in force until reset, so later statements share what remains. This is an explicit session scope, not automatic statement detection. It bounds ThinkThen calls within the query; SQLite work after the last ThinkThen call remains the host's responsibility. Connections do not share this state, while the existing engine and request total remain process scoped.

DuckDB's `SET thinkthen_query_budget_ms = n` selects the budget for each statement: `-1` means none, `0` is spent, a positive whole number is milliseconds, and any other negative or a value above ADR 0041's representable deadline is usage. The setting is read when a statement begins, not after each chunk. A statement budget requires one owner shared by every expression and chunk of that statement. Ticket 0201's C++ migration supplies a `ClientContextState` query-begin/query-end owner and proves the first-use and prepared-statement paths. The owner captures one monotonic expiry, sends the remaining time to the Rust call through the existing options interface, and treats expiry as a statement error. A first-use fallback that begins after statement start must be measured and named as such; it cannot claim to bound earlier SQL work. Ticket 0200 fixes this contract and spelling; it does not fake a query timer in the current per-expression scalar state. Ticket 0201 must preserve this ADR's result and budget behavior when it moves the host door.

No new request-count budget is introduced. Existing per-call `max_requests`, process `max_requests_total`, and the 0149/0155 checks before each send and retry remain the count controls. A spent time budget starts no next attempt. A sent attempt can finish after the caller receives a deadline error, as ADR 0017 already states.

### Retire only idle DuckDB engines

The 16-engine resident cap stays. On a seventeenth distinct plan, the registry retires the least recently used engine whose registry `Arc` is its sole owner. It adds a final snapshot of that engine's four counters to a historical process total under the registry lock before dropping the engine. `thinkthen_usage()` and every request-total check sum that historical total and all resident engines. A later use of an evicted plan builds a new engine and starts only that engine's counters at zero. The historical total remains. A detached worker or active scalar state holds an `Arc`, so its engine cannot be retired while its counters can change. If all 16 engines are held, the call refuses with deterministic pathless advice: "16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process". It names no plan dimensions or values. A fork starts its own historical total at zero, matching the engine's current per-process counters.

A message-only change was considered. It would explain the wall but leave a long-lived, idle session unable to change settings. An unconditional least-recently-used eviction was also considered. It would lose prior sends from `thinkthen_usage()` and make `max_requests_total` reusable. Idle-only retirement with a historical total resolves the reported wall after prior callers release their engines without raising the resident cap or weakening the budget.

### Refuse a changed PostgreSQL throttle

PostgreSQL checks an explicit `thinkthen.throttle` against the process's active width before it skips a builder setter. An equal value is accepted; a different value raises usage with the engine's active-width sentence. Unset keeps the environment or configuration value. The first explicit width remains selected for the process under ADR 0017, and ADR 0047's per-loaded-copy scope stays unchanged. A query cannot silently run under a throttle other than the one its setting names.

## Consequences and proof boundary

Ticket 0200 proves an answered, unresolved, failed, and SQL-`NULL` row on each host; a bad inline question and bad `@file` followed by a good row; good rows after a backend failure; no send for a spent time budget; a held send ending at the budget; unchanged retry and process-total counts; 17 idle DuckDB plans with cumulative usage; all 16 held plans refusing safely; and PostgreSQL's equal and changed throttle values. Planted key, evidence, raw question, file path, cache path, and backend address strings must be absent from every failed value and engine-cap refusal. A host interrupt remains fatal in the try form. Tests use the loopback backend and no paid calls. SQLite's explicit scope and DuckDB's deferred query hook are named limits, not claims of automatic full-statement control. Ticket 0201 owns the DuckDB hook proof.
