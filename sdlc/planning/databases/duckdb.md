# The DuckDB extension: goals and anti-goals

Shared rules live in [README.md](README.md). That page fixes the nine functions and the eight rules. This page holds what is particular to DuckDB. The C API experiment and ticket 0110 paragraphs below are historical evidence. Ticket 0201's Linux x86_64 candidate uses a pinned C++ host extension and Rust bridge; its wider release target choice remains open.

## What really good looks like

A user installs one extension, stores a key once, and points a query at a file already on disk. No table is created and no script runs beside the query.

```sql
INSTALL thinkthen FROM community;
LOAD thinkthen;

-- keep the yes rows, straight over a Parquet file
SELECT id, body FROM 'reviews.parquet'
WHERE thinkthen_decide('The reviewer asks for a refund.', body);

-- one request per row, every question at once, unpacked into columns
SELECT id, thinkthen_annotate('triage.json', body) AS triage_json FROM 'inbox.parquet';
```

## Goals

- `INSTALL thinkthen FROM community` on every platform DuckDB's CI builds and signs. The Linux x86_64 candidate uses the pinned C++ v1.5.5 host API; its extension loads in stock v1.5.5 CLI and Python and refuses stock v1.5.4. The retired C API path required `USE_UNSTABLE_C_API=1` (207). Other release targets still need a C++ package decision.
- The whole chunk of 2,048 rows crosses into the engine once and runs at the throttle the engine names.
- Repeated text in one chunk costs one judgment. The shim groups the chunk by the pair of question and text, and writes the one answer into every row holding that pair.
- A bad question or a bad threshold fails before any paid request. The C++ candidate validates ordinary foldable questions at bind and nonconstant chunks before their first send. `thinkthen_try_details` instead retains recoverable failures as safe row values. The C API bind crash in experiment 207 is the reason for the migration.
- `thinkthen_annotate` returns JSON text under the retained SQL contract. `thinkthen_tag` returns a real `LIST`.
- A `NULL` text costs nothing. DuckDB skips the call and writes `NULL` unless the function asks for special handling, and this one does not.
- The key stays in the environment variable. The stable C API registers no secret type and exposes no secret call, and a `SET` would put the key into SQL text anyway (207).
- `Ctrl-C` on a running query stops the waiting inside the engine.

## Anti-goals

- No function reads a file or a table the caller did not name, and no function takes a whole row.
- No table-producing judgment in the first release. `thinkthen_usage()` is the one table function.
- No extension-level cache. The engine's cache is the only one, and it outlives the session.

## Where the C API experiment found costs (history)

- **A call per row.** DuckDB hands a scalar function a chunk of up to 2,048 rows. The shim reads the whole chunk into one engine call and writes the answers back into the output vector. A per-row loop pays the crossing 2,048 times and judges in series.
- **A repeated value judged again.** DuckDB has constant vectors and dictionary vectors, and C++ reads the vector type and the unified vector format without flattening. The stable C extension API exposes neither. It hands out the data pointer, the validity mask, and the logical type, and no call names the storage form. Grouping the chunk by value in the shim buys the same saving: a constant column becomes one judgment and a dictionary column one judgment for each distinct value. The cost is a hash of each string.
- **A chunk is the ceiling on one call.** 2,048 rows bound how much work one `invoke` starts. The C API could gather more through a table function or an aggregate. It supports both, with bind, init, and local init on the table function. The port registers the aggregate `thinkthen_warm` through it, and whether a larger batch beats a chunk is unchecked.
- **Threads.** DuckDB is one process with many threads. The `threads` setting names how many call the function at once, and it defaults to the core count. The engine's gate is one process-wide throttle, not one batch throttle: 207 measured 100 concurrent single-row calls arriving 85 wide on 101 connections under a per-batch gate, and the process gate of ADR 0017 section 2 closes that.
- **The same call in `WHERE` and in `SELECT`.** `colliber/duckdb-jev` warns that the query asks twice and does not fix it. Whether DuckDB folds the two appearances is unchecked. The engine's cache is keyed by the whole request and answers the second for free.
- **The volatile marking.** Rule 8 marks the function volatile, and `duckdb_scalar_function_set_volatile()` is the C call. DuckDB issue 13238 constant-folded the first result of a volatile function that took no arguments, and pull request 13241 closed it in version 1.1.0. Every function here takes arguments, so the bug should not reach them.

## C API experiment architecture (history)

**The raw C API through `libduckdb-sys`, with no wrapper crate.** Ticket 0110 (2026-09-25) replaced `duckdb-rs` and its `vscalar` feature. Every scalar needs an init callback, which reads the caller's client context and file system, and `VScalar` registers none. The usage table and the warm aggregate already went through the raw C API. The `duckdb` crate's `arrow` and `hashlink` trees leave with it. Experiment 207's choice of `vscalar` stands as history.

The shim holds the signatures, the question parse, the key lookup, chunk conversion in and out, grouping by value, `STRUCT` and `LIST` construction, NULL handling, and the mapping from an engine failure to a DuckDB error. Rule 1 keeps everything else in the engine. The line ceiling for this surface is about 400 code lines for the provable functions, 748 with the mechanisms the page itself demands (207).

Two gaps sit in the Rust binding rather than in DuckDB. The `VScalar` trait requires `State`, `invoke`, and `signatures`, and offers `volatile` with a default. It has no bind step, while the C API already carries `duckdb_scalar_function_set_bind` and the calls around it. A DuckDB discussion opened on 2026-02-16 asks for that wiring and has no answer. Second, no C API function has "secret" in its name, so a stable-C-API extension registers no secret type. `colliber/duckdb-jev` registers one from C++, and Rust does not reach that path.

## Tests only this surface needs

- A 2,048-row chunk shows the stub's highest in-flight count near the engine's throttle and a wall near one round of the stub's delay.
- A chunk holding one distinct text leaves one request on the stub. A chunk holding 64 distinct texts leaves 64.
- The same call in `WHERE` and in `SELECT` leaves one request on the stub's counter.
- A bad threshold in a constant question fails with zero requests.
- `Ctrl-C` during a delayed query returns control, and the stub sees no further requests.
- The extension loads in a stock DuckDB binary with no Rust toolchain present.
- A `NULL` text returns `NULL` with no request. A backend failure raises an error rather than `NULL`.
- `a.*` over `thinkthen_annotate` gives one typed column per question.
- Test shapes force evaluation: `count(*)` over a subquery projection elided the volatile call entirely, zero requests observed, so the count tests use shapes that cannot elide (207).

## Relate on the caller's database, 2026-09-25

Ticket 0118 originally routed `thinkthen_relate(query, rules)` through a random C API probe, as ADR 0038 records. The ticket 0201 C++ bind now selects the caller's database directly. Its separate read-only connection reads committed tables even when the caller opened the file read-only; it cannot see the caller's temporary or uncommitted rows. It retains the 255-row cap, plan guard, time limit, caller file settings, and cancellation. The focused read-only regression counts zero sends for a mutating query and a caller-denied rules file. The wider C++ release target choice remains pending.

## Historical questions from experiment 207

1. Answered by 207: drop the bind-time check for the first release. The C API cannot support it, the first-row parse already fails before any request at zero cost, and rule 4 keeps the aspiration with the citation.
2. Does the question come only as a constant, or does a per-row question column stay legal with the check skipped?
3. Answered by 207: the annotate return type is fixed at registration, so the member names are part of the function's declared type — a fixed struct per installed set — or annotate waits for a bind step the C API cannot spell. The experiment shipped the fixed `struct(a, b)` and `a.*` works.
4. Answered by 207: stay on `vscalar`. Arrow measured the same at 10k rows.
5. Answered by 207: the key stays in the environment. No secret call and no setting registration exist on this API, and a `SET` puts the key in SQL text anyway.
6. Answered on 2026-09-21, in the Python section below and local experiment 207's `duckdb/NOTES.md`: Ctrl-C stops the query inside one round, DuckDB Python surfaces its own `Query interrupted`, and a host handler survives with the worker-thread-plus-`thinkthen_cancel` shape.
7. Does the extension ship signed through the community repository first, or unsigned from its own releases while the pull request waits?
8. Answered by 207 and ADR 0017 pick 10: `thinkthen_warm` exists, costs little, buys nothing on distinct data and 4x on repeats when the cache is absent, and stays for the shared name on all three databases. DuckDB's chunk already carries the throttle.

## Ctrl-C inside a Python process, 2026-09-21

Job 2 of the experiment team proved the SIGINT-at-LOAD handler inside Python (duckdb 1.5.5, extension loaded in-process, stub at 300 ms, 2,048-row query at 32 in flight). Three findings, each with the baseline that separates DuckDB's own behavior from the extension's:

- Ctrl-C during a thinkthen query stops the query within one 32-wide round (measured 0.11 s past the signal; the stub's counter frozen at 320 requests through a 3 s settle) and surfaces as DuckDB Python's own `RuntimeError: Query interrupted` — the same exception pure DuckDB raises with no extension loaded, verified against a native 19 s query. `KeyboardInterrupt` propagates only when no query is running; the chain preserves Python's default handler exactly. The earlier CLI proof stands; inside Python, the exception the host sees is DuckDB's translation, not `KeyboardInterrupt`.
- A host that installs its own SIGINT handler after `LOAD` keeps it — the extension's chaining does not break it — but the install replaces the extension's handler at the OS level, so the automatic stop is lost, and a Python handler cannot run while the main thread is blocked in the query. Measured deaf: 16.42 s of paid work past the signal.
- The working shape for a host with its own handler: run long queries on a worker thread, keep the main thread free, and call `thinkthen_cancel()` from a second connection when the handler fires. Measured 0.05 s from signal to stop, requests frozen, and the engine's `cancelled` kind carries in DuckDB's error message.

The full commands and output are in local experiment 207's `duckdb/NOTES.md` under "Job 2, 2026-09-21".
