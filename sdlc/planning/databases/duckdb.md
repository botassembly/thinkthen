# The DuckDB extension: goals and anti-goals

Shared rules live in [README.md](README.md). That page fixes the nine functions and the eight rules. This page holds what is particular to DuckDB.

## What really good looks like

A user installs one extension, stores a key once, and points a query at a file already on disk. No table is created and no script runs beside the query.

```sql
INSTALL thinkthen FROM community;
LOAD thinkthen;

-- keep the yes rows, straight over a Parquet file
SELECT id, body FROM 'reviews.parquet'
WHERE thinkthen_decide('The reviewer asks for a refund.', body);

-- one request per row, every question at once, unpacked into columns
SELECT id, a.* FROM (
  SELECT id, thinkthen_annotate('triage.json', body) AS a FROM 'inbox.parquet'
);
```

## Goals

- `INSTALL thinkthen FROM community` on every platform DuckDB's CI builds and signs.
- The whole chunk of 2,048 rows crosses into the engine once and runs at the width the engine names.
- Repeated text in one chunk costs one judgment. The shim groups the chunk by the pair of question and text, and writes the one answer into every row holding that pair.
- A bad question or a bad threshold fails before any request, at bind time where DuckDB allows it.
- `thinkthen_annotate` returns a real `STRUCT`, so `a.*` expands into typed columns. `thinkthen_tag` returns a real `LIST`.
- A `NULL` text costs nothing. DuckDB skips the call and writes `NULL` unless the function asks for special handling, and this one does not.
- The key reaches the extension without passing through SQL text.
- `Ctrl-C` on a running query stops the waiting inside the engine.

## Anti-goals

- No function reads a file or a table the caller did not name, and no function takes a whole row.
- No table-producing judgment in the first version. `thinkthen_usage()` is the one table function.
- No extension-level cache. The engine's cache is the only one, and it outlives the session.

## Where this database wastes time

- **A call per row.** DuckDB hands a scalar function a chunk of up to 2,048 rows. The shim reads the whole chunk into one engine call and writes the answers back into the output vector. A per-row loop pays the crossing 2,048 times and judges in series.
- **A repeated value judged again.** DuckDB has constant vectors and dictionary vectors, and C++ reads the vector type and the unified vector format without flattening. The stable C extension API exposes neither. It hands out the data pointer, the validity mask, and the logical type, and no call names the storage form. Grouping the chunk by value in the shim buys the same saving: a constant column becomes one judgment and a dictionary column one judgment for each distinct value. The cost is a hash of each string.
- **A chunk is the ceiling on one call.** 2,048 rows bound how much work one `invoke` starts. The C API could gather more through a table function or an aggregate. It supports both, with bind, init, and local init on the table function. `duckdb-rs` wires up neither, and whether a larger batch beats a chunk is unchecked.
- **Threads.** DuckDB is one process with many threads. The `threads` setting names how many call the function at once, and it defaults to the core count. One engine pool serves all of them, built once on load, so the width stays one number for the process.
- **The same call in `WHERE` and in `SELECT`.** `colliber/duckdb-jev` warns that the query asks twice and does not fix it. Whether DuckDB folds the two appearances is unchecked. The engine's cache is keyed by the whole request and answers the second for free.
- **The volatile marking.** Rule 8 marks the function volatile, and `duckdb_scalar_function_set_volatile()` is the C call. DuckDB issue 13238 constant-folded the first result of a volatile function that took no arguments, and pull request 13241 closed it in version 1.1.0. Every function here takes arguments, so the bug should not reach them.

## How little code

**`duckdb-rs` with the `vscalar` and `vscalar-arrow` features, laid out like `duckdb/extension-template-rs`.** That template builds on the stable C extension API, and the community build expects it. The template's README calls itself experimental, and C++ is still DuckDB's main path.

The shim holds the eight signatures, the question parse, the key lookup, chunk conversion in and out, grouping by value, `STRUCT` and `LIST` construction, NULL handling, and the mapping from an engine failure to a DuckDB error. Rule 1 keeps everything else in the engine.

Two gaps sit in the Rust binding rather than in DuckDB. The `VScalar` trait requires `State`, `invoke`, and `signatures`, and offers `volatile` with a default. It has no bind step, while the C API already carries `duckdb_scalar_function_set_bind` and the calls around it. A DuckDB discussion opened on 2026-02-16 asks for that wiring and has no answer. Second, no C API function has "secret" in its name, so a stable-C-API extension registers no secret type. `colliber/duckdb-jev` registers one from C++, and Rust does not reach that path.

## Tests only this surface needs

- A 2,048-row chunk shows the stub's highest in-flight count near the engine's width and a wall near one round of the stub's delay.
- A chunk holding one distinct text leaves one request on the stub. A chunk holding 64 distinct texts leaves 64.
- The same call in `WHERE` and in `SELECT` leaves one request on the stub's counter.
- A bad threshold in a constant question fails with zero requests.
- `Ctrl-C` during a delayed query returns control, and the stub sees no further requests.
- The extension loads in a stock DuckDB binary with no Rust toolchain present.
- A `NULL` text returns `NULL` with no request. A backend failure raises an error rather than `NULL`.
- `a.*` over `thinkthen_annotate` gives one typed column per question.

## Open questions for the ADR

1. Rule 4 asks for a bind-time check and `duckdb-rs` gives no bind step. Does the extension drop to the C API, push the wiring upstream, or soften rule 4 to a first-row check?
2. Does the question come only as a constant, or does a per-row question column stay legal with the check skipped?
3. Does `thinkthen_annotate` take a file path, a `STRUCT` of question text, or both, and how is its return type declared without a bind step?
4. Does the chunk cross as Arrow through `vscalar-arrow` or as DuckDB vectors through `vscalar`? The Arrow path hands `invoke` a `RecordBatch` and takes one Arrow array back.
5. Where does the key live, given that the stable C API registers no secret type? A setting and an environment variable are the two candidates, and rule 3 forbids an argument.
6. What does a cancel reach inside the engine? `duckdb_interrupt` sits on a connection. A poll from inside a scalar function is unchecked.
7. Does the extension ship signed through the community repository first, or unsigned from its own releases while the pull request waits?
8. Does `thinkthen_warm(question, text)` exist here? **The recommendation is yes, for sameness.** SQLite and PostgreSQL both take the aggregate as their first bulk form, and a script that runs on all three should not fork on the name. It buys DuckDB no speed, because the chunk already runs at full width. The cost is the C aggregate path, since `duckdb-rs` exposes no aggregate. If that path is blocked, the page drops the name and the shared README records a forced difference.

## Ctrl-C inside a Python process, 2026-09-21

Job 2 of the experiment team proved the SIGINT-at-LOAD handler inside Python (duckdb 1.5.5, extension loaded in-process, stub at 300 ms, 2,048-row query at 32 in flight). Three findings, each with the baseline that separates DuckDB's own behavior from the extension's:

- Ctrl-C during a thinkthen query stops the query within one 32-wide round (measured 0.11 s past the signal; the stub's counter frozen at 320 requests through a 3 s settle) and surfaces as DuckDB Python's own `RuntimeError: Query interrupted` — the same exception pure DuckDB raises with no extension loaded, verified against a native 19 s query. `KeyboardInterrupt` propagates only when no query is running; the chain preserves Python's default handler exactly. The earlier CLI proof stands; inside Python, the exception the host sees is DuckDB's translation, not `KeyboardInterrupt`.
- A host that installs its own SIGINT handler after `LOAD` keeps it — the extension's chaining does not break it — but the install replaces the extension's handler at the OS level, so the automatic stop is lost, and a Python handler cannot run while the main thread is blocked in the query. Measured deaf: 16.42 s of paid work past the signal.
- The working shape for a host with its own handler: run long queries on a worker thread, keep the main thread free, and call `thinkthen_cancel()` from a second connection when the handler fires. Measured 0.05 s from signal to stop, requests frozen, and the engine's `cancelled` kind carries in DuckDB's error message.

The full commands and output are in `experiments/207-thinkthen-db/duckdb/NOTES.md` under "Job 2, 2026-09-21".
