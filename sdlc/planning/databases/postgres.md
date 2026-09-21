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
- A constant question is parsed once per backend, in a per-backend parse cache. `fn_extra` is replaced: pgrx offers nothing above raw `pg_sys`, and the map is stronger — one parse per backend, not per statement — at about 2 µs a parse saved per row (207).
- Equal pairs of question and text inside one array or one warm pass are asked once.
- `thinkthen_annotate` returns `jsonb`. `jsonb_to_record` names the columns and their types at the call site.
- `thinkthen_score` answers the double — the specification's position from 0 to K−1 — and the nearest level's name rides in `thinkthen_details` (ADR 0017 pick 6). The round-two shape divergence resolves here: one shape rules across the three databases, and this page's double was it.
- The key is a setting only a superuser sets, and no view shows it.
- `pg_cancel_backend` and `statement_timeout` stop a call that waits on the network. Measured (207): with the engine's token, control returns 0.36–0.46 s after the interrupt and traffic freezes within one in-flight round; `statement_timeout 2s` aborted at 2.361 s frozen at 224 requests; round one, without the token, returned control in 80 ms by abandoning the batch. Nothing is billed past one round after the interrupt, and a client that dies leaves the orphan completion, which the engine's cancel promise bounds.

## Anti-goals

- No function takes query text, and no function calls SPI. The extension reads no table. The caller aggregates the column and hands over an array.
- No whole-row argument and no read-ahead over a table's tuple identifiers. `pg-jev` does both.
- No parallel-safe marking on a judging function. `PARALLEL RESTRICTED` everywhere, with its measured price: 1.22 s against 0.33 s for 1,000 rows on a 1.2 ms wire, and four workers would run four times the width without it (207). pgrx 0.17 spells the restricted case.
- No background worker, no queue table, no extension-owned table.
- No runtime, no socket, and no wire touch in `_PG_init` under `shared_preload_libraries`. 207 measured the sharpest form: a shared-preload engine touch does not degrade a backend, it prevents startup outright — the postmaster's own `_PG_init` hung on the wire and the server never finished starting. The shipped shape registers settings and touches nothing else; the engine builds lazily per backend, and its process-ID check rebuilds in any backend that inherited a stamp (ADR 0017 section 2; 211's shared-preload arm answered from a backend whose init pid was the postmaster's).
- No chase after managed services. RDS, Aurora, Cloud SQL, Neon, and Supabase allow only their own lists.

## Where this database wastes time

- **A call per row.** A scalar function runs once per row inside one backend process. A thousand rows at a fifth of a second each is a serial wait of many minutes. The warm pass and the array form are the fix, and the documentation calls the scalar form serial.
- **The fork.** Every connection is its own forked process. Threads do not survive `fork`: only the calling thread lives on, and a lock a vanished thread held stays locked. The engine stamps its process ID and rebuilds the pool and the gate on a mismatch, taking no inherited lock, so a backend answers on its first call whether the state was built in the backend or inherited from the postmaster (ADR 0017 section 2). Nothing is built in the postmaster beyond settings.
- **Parallel workers.** A parallel worker is a separate process with its own memory, so it builds its own state and its own pool. Four workers would run four times the width and break the vendor's rate limit. `PARALLEL RESTRICTED` keeps a judging function in the leader alone and still lets the rest of the plan go parallel, at the measured 1.22 s against 0.33 s for 1,000 rows (207). `PARALLEL UNSAFE` would force the whole query serial for nothing.
- **A question parsed per row.** Without `fn_extra` the shim parses the same constant question a thousand times for a thousand rows.
- **A set delivered a row at a time.** pgrx's `TableIterator` returns one row per call, and a materialized set fills a tuplestore once. Which is cheaper for a thousand rows is unchecked, and whether pgrx 0.17 offers the materialized mode is unchecked. Either way the context a set-returning function enters on each call is cleared between calls, so cross-call state belongs in `multi_call_memory_ctx`.
- **The same call in `WHERE` and in `SELECT`.** A volatile function is evaluated at each appearance. The engine's cache, keyed by the whole request, answers the second for nothing.
- **A pooler in transaction mode.** The engine's pool lives as long as the backend. Such a pooler hands each statement a different backend and pays for a new secure connection every time.

## How little code

**pgrx.** It generates the FFI, the control file, and the SQL script. Version 0.17 gives `TableIterator` for a set of rows, `#[pg_aggregate]` for the warm form, `Array` for input read lazily, `GucSetting` with `GucContext::Suset` and `GucFlags`, and `check_for_interrupts!()`. The aggregate's costs under 0.17 are named and carried: state is TEXT carrying JSON, O(n²) bytes touched, and the transition functions leak as public SQL names (`warm_warm_state` and friends). The real crate needs an internal state type or a varlena `PostgresType`, and a schema or a naming plan for the leaked helpers (207).

The shim holds the signatures in two arities each, the aggregate, array in and a set of rows out, `jsonb` construction, NULL handling, the settings, the per-backend state, and the cancel loop. Rule 1 keeps everything else in the engine. The line ceiling is 299 code lines (207). The measured per-row costs through the layer: decide 0.91 µs, probability 0.85, choose at three options 2.09, score 1.91, tag at three 2.11, annotate at two 5.62, details 3.98, against a 0.15 µs bare statement.

The wait runs on the main thread: poll the future for a short tick, call `check_for_interrupts!()`, poll again. A cancel and a statement timeout both raise through that check, and `pgsql-http` proves the shape with a curl progress callback that reads `QueryCancelPending`. No worker thread ever calls into PostgreSQL.

The key is `thinkthen.api_key`, registered in `_PG_init` under `GucContext::Suset` with `NO_SHOW_ALL`, `SUPERUSER_ONLY`, and `DISALLOW_IN_AUTO_FILE`. Each flag does one job: keep the row out of `SHOW ALL`, `pg_settings`, and the sample configuration; refuse a direct `SHOW`; restrict who sets it; and stop `ALTER SYSTEM` from writing it into `postgresql.auto.conf`.

Four limits grow to six. On PostgreSQL 15 through 17 a typed `SET thinkthen.api_key = '...'` leaves the literal in `pg_stat_statements.query`, and PostgreSQL 18 is the first release that replaces it. `log_statement = 'all'` logs the same line, so the key belongs in `postgresql.conf` or in the connection's startup options. Loading through `shared_preload_libraries` closes the placeholder window — the undefined name any user may set and read before the library loads in a session — but the reload log line remains: accepting that leak, or requiring the environment variable there too, is a policy call (207). Members of `pg_read_all_settings` read it like a superuser, and a superuser reads the backend's memory and the server's files. The last limit: the key channel accepts the setting for the startup line and the environment for the reload.

The cache follows the XDG ruling of ADR 0017 section 5: the platform cache home by default, a folder the user names always winning, the 1 GiB cap and the prune shipping with the location. The server's operating-system user owns the folder. A warm pass in one session and a query in the next are free only through that on-disk cache, which is the engine's to own (207).

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

1. Where does a question get checked, given that PostgreSQL has no bind step? A planner support function reaches a constant question. Everything else fails on the first call, at zero requests, as in the other two databases.
2. Answered by the XDG ruling (ADR 0017 section 5): the cache defaults to the platform cache home, outside the data directory, where `pg_basebackup` does not copy it.
3. Answered by 207: load through `shared_preload_libraries`, register settings only, build nothing on the wire in `_PG_init` — a wire touch there prevents startup outright — and let the engine build lazily per backend.
4. Does the key come only from the setting, or does the server's environment win when both are present? The environment carries the reload line; the policy call is open.
5. Answered by 207's probe: no cast and no rename. A bare `ARRAY[...]` resolves to the array overload and an untyped `NULL` to the scalar. NULL evidence answers NULL ("not sure"), a NULL array element emits its row with NULL, empty-string evidence raises the engine's usage error, and no failure ever reads as NULL.
6. How does a binary reach a user? Trunk is gone: `pgt.dev` answers NXDOMAIN and `github.com/tembo-io/trunk` returns 404, read on 2026-09-20, and the owning organization has no public repository left. pgxman is in the same state, with a site that does not resolve and a last release from August 2024. PGXN ships source and needs a Rust toolchain on the installing machine. The live binary channels are the PGDG repositories, Pigsty, and a Docker image built from a stock base.
7. pgrx's README says how to interact with PostgreSQL from an async context remains unexplored, and it leaves `sigprocmask` unresolved. The engine is blocking now (ADR 0017 section 2), which removes the async half of the question; the cancel loop still proves itself against `check_for_interrupts!()` on the calling thread.
8. Does the array overload stay, now that `thinkthen_warm` carries the bulk story too? **The recommendation is both, with the warm form first.** The aggregate is the only bulk shape SQLite can also spell, so it is the form the three databases get compared on. The array form stays beside it because PostgreSQL's type system offers an array and SQLite's does not, and that is the one kind of difference the shared anti-goal allows. Ian can overturn this and keep the array form alone.
9. Answered by 207: `PARALLEL RESTRICTED`, with the measured price named in the anti-goals.
