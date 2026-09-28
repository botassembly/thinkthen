# ADR 0038: DuckDB relate runs on the caller's database, not the caller's connection

- Status: Decided by the library team on 2026-09-22 on the lane's command-backed evidence. It is input to the build team's C++ fork decision. Ian can overturn.
- Date: 2026-09-22

## Decision

`thinkthen_relate` runs its query on a connection belonging to the caller's database, resolved at bind time through `duckdb_table_function_get_client_context` and `duckdb_client_context_get_catalog` against a per-database registry built at init. The one process-global connection is gone.

The caller's own connection is not reachable through the stable C API: `duckdb_query` requires a `duckdb_connection`; a client context exposes catalogs, config, the file system, and a connection id only; and temporary tables live in per-connection `ClientData`. Temporary-table and open-transaction visibility is therefore a boundary of the stable C API, not a defect this surface can fix. The boundary is pinned by a test, and a query naming a temporary table answers with the boundary in words instead of the raw catalog error. Same-name databases in one process cannot be told apart through the client context and are refused as a defect rather than guessed.

## Why this option

Three options were weighed on the lane's evidence. Option B — changing relate's SQL shape so records cross as values the caller's own SQL builds (a LIST of structs) — would make temporary tables and transactions work, but it changes the settled string form used by the drawn call and every relate conformance case, a contract-level change beyond the lane. Option C — document the defects and leave the wrong-database bug — was refused. Option A removes the wrong-database bug and the stale global without changing the settled shape, so it was ruled.

## Evidence

From `databases/duckdb/NOTES.md` on branch `surfaces`:

- The context accessors expose no connection: catalog, config, file system, connection id.
- Temporary tables live in per-connection `ClientData` (`temporary_objects`); a live temporary-table call answers `Catalog Error: Table with name tt does not exist`.
- Before the fix, two loaded databases shared the last-loaded connection: the two-database probe failed with "no recorded answer for the rule caused_by on these records; the recording covers founded, works_for" — a.db's query ran against b.db's table.
- After the fix: `a.db: 4 edges (want 4), b.db: 11 edges (want 11)`.
- The boundary message: "the relate query names the temporary table tt, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables; materialize it (CREATE TABLE ... AS SELECT) or run the query directly".

## For the build team

This boundary is exactly the one the C++ door would lift. Temporary-table and open-transaction visibility for relate is impossible through the stable C API; the C++ fork under the build team's consideration (the `duckdb-rs` bind-callback question) is what removes it. The lane recorded this as MERGE-NOTE material.

The second review found further defects in this area — two in-memory databases both registering as "memory"; `ATTACH … USE other` counting the main database's table; a saved connection keeping a closed database file locked; relate holding any SQL and committing on its own; a nested relate hanging. Those are filed in `sdlc/issues/closed/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md` and remain open.

## Proof

Landed at `0345a42` ("Route DuckDB's relate to the caller's database, respect its file switch, and contain callback panics"). The fix's suites run from `check.sh`: `tools/two_databases.py` proves each database's relate runs on its own connection, `tools/security_suite.sh` proves the boundary message and the raw-error pass-through, and the conformance slice exits 0 with no stub.

## Amendment of 2026-09-25: the ported extension, part one (ticket 0110)

Ticket 0110 ports the extension onto `thinkthen`'s public API and carries this ADR's rule past relate: every call runs on the caller's database. Ticket 0118 writes part two, for relate. Ian can overturn each point.

- **SIGINT at LOAD.** LOAD takes SIGINT through `sigaction` and chains to the host's own action with the signature its flags name. A host that ignores SIGINT keeps ignoring it, and a default action is restored and re-raised, so it still ends the process. The host's action is recorded before ours goes in.
- **The interrupt predicate and the worker.** Every engine call runs on a detachable worker with its own cancel token. The DuckDB thread waits in 50 ms ticks and reads a predicate: a signal after the invoke began, or within 10 ms before it. On a stop it cancels the token and raises `thinkthen cancelled: ` at once. No token outlives a call, so a signal never reaches the next query.
- **The `con.interrupt()` limit (R5-23).** DuckDB gives a scalar no view of its own interrupt, so `con.interrupt()` does not stop a held batch before its replies arrive. A SIGINT does, within 100 ms. `sdlc/issues/2026-09-23-duckdb-scalars-through-the-c-api-for-a-query-hook.md` names the lever.
- **Access through the caller's own file system.** Each scalar registers through the raw C API with an init callback. The init reads the caller's client context and file system. `@file` opens through that file system, so the caller's own settings decide each read. A cache folder set from SQL must be an absolute local path with no scheme, and a probe open through the same file system must not be refused. Over experiment 253's 35 cases, this agreed with DuckDB's own `COPY` and `read_text` in every case (`sdlc/records/0110-port-the-duckdb-surface.md`).
- **The four engine settings.** `thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_cache`, and `thinkthen_cache_bytes` reach `EngineBuilder`'s own setters on `EngineBuilder::from_env()`. Each init checks every set value before it reads the engine map. The process keeps one engine per throttle, request limit, and cache folder, at most 16. The first throttle wins for the process, as main's `build` rules.
- **Volatile scalars.** Every scalar registers as volatile, so the planner never folds a constant call into a send.
- **Warm.** The aggregate has no client context, so `thinkthen_warm` refuses `@file` and runs on the engine the environment describes.
- **Licenses.** The binding uses `libduckdb-sys` alone, so the `duckdb` crate's `arrow` and `hashlink` trees leave. `foldhash` and `tiny-keccak` leave with them. One exception remains: `zlib-rs` (Zlib), a build-time dependency of `libduckdb-sys`.

## Amendment of 2026-09-25: the ported extension, part two (ticket 0118)

Ticket 0118 ports relate onto `thinkthen`'s public API and completes this amendment. `databases/duckdb/tools/source_checks.py` reads each sentence in bold below, in both parts, and fails when one leaves (R2-29). Ian can overturn each point.

- **Relate from rows.** **The relate query returns `id, name, kind` or `id, name`, rows with the same name and kind become one entity, and each edge returns one row per pair of their ids.** The rules read as a list of command-grammar rules, a rules file's JSON, or `@file` through the caller's own file system.
- **The row cap and the time limit.** **Relate reads at most 255 rows, under `LIMIT 256`, and more is `usage`.** **`SET thinkthen_relate_seconds` bounds the query and the engine call, and `memory_limit` stays the hard bound for a step that holds its input.** A timer interrupts the kept connection at the limit, and the plan-size guard refuses a large holding step before the query runs.
- **The identity source.** **Each kept connection attaches an in-memory probe database named from 128 bits of `/dev/urandom`, and a caller matches only the probe its own context resolves.** No setting names it, and a database that cannot attach one is never routed to. The reaper closes a kept connection once its database has no other, and a counted guard keeps a bound relate's connection across a pass.
- **The bridge's per-process pipe.** **The SIGINT handler writes one byte to a pipe only when the pipe's recorded process id equals `getpid()`, and a bridge thread interrupts each busy kept connection.** The process id and the write end share one atomic, so the handler never pairs one process's id with another's descriptor. A forked child builds its own pipe on first use.

## Amendment of 2026-09-25: warm reads `@file` (ticket 0129)

This replaces part one's warm point. `source_checks.py` reads the sentence in bold. Ian can overturn it.

- **Warm.** **The aggregate reads `@file` through the kept connection of the database that registered it, under that database's gate, and runs on the engine the environment describes.** LOAD gives each kept connection a process-wide serial, and the aggregate carries it. DuckDB's database-wide file settings decide each read. Warm refuses `'@~'` paths, because `home_directory` is a session setting the kept connection does not share. It refuses at once while a relate query runs on the kept connection, since a warm inside that query would wait on the gate its own query holds.
