# The PostgreSQL extension: goals and anti-goals

Shared rules live in [README.md](README.md). That page fixes the nine functions and the eight rules. This page holds only what is particular to PostgreSQL.

## What really good looks like

A person who runs their own server puts the key in the configuration, runs one `CREATE EXTENSION`, and queries a table that is already there.

```sql
-- postgresql.conf, read at startup. Never typed into a session.
-- thinkthen.api_key = 'sk-...'
CREATE EXTENSION thinkthen;

-- the scalar form: one judgment per row, in series. IS NULL finds the unsure ones
SELECT id, body FROM reviews
WHERE thinkthen_decide('The reviewer asks for a refund.', body);

-- the warm form: one pass at full width fills the cache, and the query reads it back
SELECT thinkthen_warm('The reviewer asks for a refund.', body) FROM reviews;

-- the array form: one array in, one row per element out, position kept
SELECT d.i, d.decided
FROM (SELECT array_agg(body ORDER BY id) AS bodies FROM reviews) b,
     thinkthen_decide('The reviewer asks for a refund.', b.bodies) AS d(i, decided);

-- one request per row, every question at once, unpacked into columns
SELECT i.id, a.spam, a.folder FROM inbox i,
LATERAL jsonb_to_record(thinkthen_annotate('triage.json', i.body)) AS a(spam boolean, folder text);
```

## Goals

- `CREATE EXTENSION thinkthen` from a prebuilt package, with no Rust toolchain on the server.
- `thinkthen_warm(question, text)` is an aggregate. It runs the column at the engine's width, fills the cache on disk, and returns the count. The name and the meaning carry to the other two databases.
- The array form overloads the same eight names on an array argument. No name beyond `thinkthen_warm` enters the surface. The set emits the position itself, so the join back needs no `WITH ORDINALITY`.
- One array crosses into the engine once as pgrx's `Array<'_, &str>`. That type reads an element only when the iterator reaches it. `Vec<Option<String>>` detoasts and copies the whole array on entry.
- A constant question is parsed once per statement. `flinfo->fn_extra` holds the parsed value, allocated in `fn_mcxt`. That context lives as long as the statement's `FmgrInfo`.
- Equal pairs of question and text inside one array or one warm pass are asked once.
- `thinkthen_annotate` returns `jsonb`. `jsonb_to_record` names the columns and their types at the call site.
- The key is a setting only a superuser sets, and no view shows it.
- `pg_cancel_backend` and `statement_timeout` stop a call that waits on the network.

## Anti-goals

- No function takes query text, and no function calls SPI. The extension reads no table. The caller aggregates the column and hands over an array.
- No whole-row argument and no read-ahead over a table's tuple identifiers. `pg-jev` does both.
- No parallel-safe marking on a judging function. A worker would open its own width.
- No background worker, no queue table, no extension-owned table.
- No runtime, no socket, and no thread built in `_PG_init` under `shared_preload_libraries`.
- No chase after managed services. RDS, Aurora, Cloud SQL, Neon, and Supabase allow only their own lists.

## Where this database wastes time

- **A call per row.** A scalar function runs once per row inside one backend process. A thousand rows at a fifth of a second each is a serial wait of many minutes. The warm pass and the array form are the fix, and the documentation calls the scalar form serial.
- **The fork.** Every connection is its own forked process. Threads do not survive `fork`: only the calling thread lives on, and a lock a vanished thread held stays locked. A runtime built in the postmaster hands every backend a dead worker pool. Build it backend-local, on first use, after the fork.
- **Parallel workers.** A parallel worker is a separate process with its own memory, so it builds its own runtime and its own pool. Four workers would run four times the width and break the vendor's rate limit. `PARALLEL RESTRICTED` keeps a judging function in the leader alone and still lets the rest of the plan go parallel. `PARALLEL UNSAFE` would force the whole query serial for nothing. pgrx spells `parallel_safe` on `#[pg_extern]`; whether 0.17 spells the restricted case is unchecked.
- **A question parsed per row.** Without `fn_extra` the shim parses the same constant question a thousand times for a thousand rows.
- **A set delivered a row at a time.** pgrx's `TableIterator` returns one row per call, and a materialized set fills a tuplestore once. Which is cheaper for a thousand rows is unchecked, and whether pgrx 0.17 offers the materialized mode is unchecked. Either way the context a set-returning function enters on each call is cleared between calls, so cross-call state belongs in `multi_call_memory_ctx`.
- **The same call in `WHERE` and in `SELECT`.** A volatile function is evaluated at each appearance. The engine's cache, keyed by the whole request, answers the second for nothing.
- **A pooler in transaction mode.** The engine's pool lives as long as the backend. Such a pooler hands each statement a different backend and pays for a new secure connection every time.

## How little code

**pgrx.** It generates the FFI, the control file, and the SQL script. Version 0.17 gives `TableIterator` for a set of rows, `#[pg_aggregate]` for the warm form, `Array` for input read lazily, `GucSetting` with `GucContext::Suset` and `GucFlags`, and `check_for_interrupts!()`. The aggregate trait asks for a state type, a state function, a finalize function, and a combine function, and the extension leaves its parallel option unset.

The shim holds the eight signatures in two arities each, the aggregate, array in and a set of rows out, `jsonb` construction, NULL handling, the settings, the backend-local runtime, and the cancel loop. Rule 1 keeps everything else in the engine.

The wait runs on the main thread: poll the future for a short tick, call `check_for_interrupts!()`, poll again. A cancel and a statement timeout both raise through that check, and `pgsql-http` proves the shape with a curl progress callback that reads `QueryCancelPending`. No worker thread ever calls into PostgreSQL.

The key is `thinkthen.api_key`, registered in `_PG_init` under `GucContext::Suset` with `NO_SHOW_ALL`, `SUPERUSER_ONLY`, and `DISALLOW_IN_AUTO_FILE`. Each flag does one job: keep the row out of `SHOW ALL`, `pg_settings`, and the sample configuration; refuse a direct `SHOW`; restrict who sets it; and stop `ALTER SYSTEM` from writing it into `postgresql.auto.conf`.

Four limits remain. On PostgreSQL 15 through 17 a typed `SET thinkthen.api_key = '...'` leaves the literal in `pg_stat_statements.query`, and PostgreSQL 18 is the first release that replaces it. `log_statement = 'all'` logs the same line, so the key belongs in `postgresql.conf` or in the connection's startup options. Before the extension loads, the name is an undefined placeholder any user may set and read. Members of `pg_read_all_settings` read it like a superuser, and a superuser reads the backend's memory and the server's files.

The cache is a folder owned by the operating system user the server runs as. `thinkthen.cache_dir` moves it.

## Tests only this surface needs

- A 1,000-element array shows the stub's highest in-flight count near the engine's width and a wall near one round of the stub's delay. The warm aggregate over 1,000 rows shows the same pair.
- A query after a warm pass makes zero requests, counted on the stub.
- A thousand rows holding one hundred distinct texts leave one hundred requests.
- A plan with `max_parallel_workers_per_gather` above zero does not raise the stub's highest in-flight count.
- The same scalar call in `WHERE` and in `SELECT` leaves one request on the stub's counter.
- `pg_cancel_backend` from a second session returns control during a delayed call, and `statement_timeout` does the same.
- A runtime built after the fork serves a fresh connection. A runtime built in the postmaster is proven to fail, and the code refuses to build one there.
- `thinkthen.api_key` appears in no row of `pg_settings`, no line of the server log, and no row of `pg_stat_statements`.
- The built extension installs into a stock PostgreSQL image that has no Rust toolchain.

## Open questions for the ADR

1. Where does a question get checked, given that PostgreSQL has no bind step? A planner support function reaches a constant question. Everything else fails on the first call.
2. Does the cache default under the data directory, where `pg_basebackup` copies it, or outside it, where nothing guarantees the server user can write?
3. Does the extension need `shared_preload_libraries` at all, given that `_PG_init` there may only register settings?
4. Does the key come only from the setting, or does the server's environment win when both are present?
5. An untyped `ARRAY[...]` literal is `unknown` until PostgreSQL resolves it, so a call may sit between the scalar and the array overload. Does the extension document a cast, or rename the array form?
6. How does a binary reach a user? Trunk is gone: `pgt.dev` answers NXDOMAIN and `github.com/tembo-io/trunk` returns 404, read on 2026-09-20, and the owning organization has no public repository left. pgxman is in the same state, with a site that does not resolve and a last release from August 2024. PGXN ships source and needs a Rust toolchain on the installing machine. The live binary channels are the PGDG repositories, Pigsty, and a Docker image built from a stock base.
7. pgrx's README says how to interact with PostgreSQL from an async context remains unexplored, and it leaves `sigprocmask` unresolved. What does the cancel loop have to prove before that is safe?
8. Does the array overload stay, now that `thinkthen_warm` carries the bulk story too? **The recommendation is both, with the warm form first.** The aggregate is the only bulk shape SQLite can also spell, so it is the form the three databases get compared on. The array form stays beside it because PostgreSQL's type system offers an array and SQLite's does not, and that is the one kind of difference the shared anti-goal allows. The cost is a ninth name on all three surfaces and a second bulk path here. Ian can overturn this and keep the array form alone.
9. Is `PARALLEL RESTRICTED` the right marking? It holds the width to one number and costs a parallel scan of the judged column. The experiment measures both markings.
