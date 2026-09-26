# The DuckDB surface

A loadable DuckDB v1.5.5 extension that puts the `thinkthen` engine behind SQL. It reaches the engine only through `thinkthen`'s public Rust API and DuckDB only through its C API (ADR 0047, ticket 0110).

## Functions

| SQL | Returns |
| --- | --- |
| `thinkthen_decide(question, text)` | `BOOLEAN`; `NULL` is "not sure" |
| `thinkthen_probability(question, text)` | `DOUBLE`, the yes probability |
| `thinkthen_choose(question, text, options)` | `VARCHAR`, or `NULL` when no option clears the cut |
| `thinkthen_score(question, text, levels)` | `DOUBLE`, the position from 0 for the first level |
| `thinkthen_tag(question, text, labels)` | `VARCHAR[]` |
| `thinkthen_annotate(set, text)` | `VARCHAR`, the record's values as JSON |
| `thinkthen_details(question, text)` | a struct: `probability`, `answer`, `value`, `nearest`, `model`, `question_sha256`, `requests_sent`, `cached` |
| `thinkthen_recognize(text, kinds)` | a list of `(name, kind, start, end, strength)` |
| `thinkthen_relations(text, file)` | a list of `(relation, source, source_kind, target, target_kind, probability)` |
| `thinkthen_relate(query, rules)` | a table of `(relation, source, target, probability)`, one row per edge between the query's ids |
| `thinkthen_warm(question, text)` | an aggregate: asks each distinct text once and returns how many |
| `thinkthen_usage()` | rows `(metric, value)` for `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens` |

`WHERE`, `ORDER BY`, and `LIMIT` are the filter, rank, and find verbs. Every scalar except `thinkthen_recognize` also takes a last `BIGINT` deadline in milliseconds. A `NULL` in any argument gives a `NULL` row. A failure is an error whose text starts `thinkthen <kind>: `, with one of the six kinds, and never reads as `NULL`.

A question is plain text, `'@path.json'`, or the question file's JSON. Choose, score, and tag take plain text and put their members in the list. `thinkthen_annotate` takes a question set file or its JSON. A member a verb lacks reads `NULL` in `thinkthen_details`, never 0. `start` and `end` count code points, as DuckDB's string indexing does.

```sql
LOAD 'build/thinkthen.duckdb_extension';
SELECT id FROM tickets WHERE thinkthen_decide('Does the writer ask for a refund?', body);
SELECT id, thinkthen_choose('Which team owns this?', body, ['billing', 'shipping']) FROM tickets;
```

## Settings

The engine starts from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE`. SQL cannot name a backend or a key. Four session settings reach the engine's own setters, and a fifth caps the whole process:

- `SET thinkthen_throttle = N` caps live requests in flight at N, from 1 through 32. Live requests are capped by the active throttle, or by 4 when none is set. The first throttle holds for the life of the process, and a different one reads `thinkthen usage: throttle 8 is already active for this process; use throttle 8 or drop the throttle argument`.
- `SET thinkthen_max_requests = N` caps one call's requests. One call covers one chunk of at most 2,048 rows, so the cap does not bound a whole query. `sdlc/issues/2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md` names the lever.
- `SET thinkthen_cache = '/absolute/folder'` moves the cache. The folder must be an absolute local path with no scheme, and the calling database's own file settings must allow it. Each call checks both before it asks anything.
- `SET thinkthen_cache_bytes = N` caps the cache's size.
- `SET thinkthen_max_requests_total = N` caps the requests this process sends. Before each call the extension adds up what its engines have sent. A spent total reads `thinkthen usage: this process has spent its request total of N; raise SET thinkthen_max_requests_total or RESET it` and sends nothing. A call with more texts than remain sends only the first ones that fit, then raises the same sentence. Calls running at the same time can each spend what remains, so the total can be passed by one call per thread in flight, plus retries. A forked child starts from zero. `thinkthen_warm` reads no session setting, so the total does not bind it. `thinkthen status` never sees this spend, because it counts only what the command sends.

A bad value's `SET` succeeds, since DuckDB has no check step for an extension setting, and the next call refuses it. The refusal uses the setter's own sentence, so a bad `SET` never wraps into an accepted one. The extension keeps one engine per distinct throttle, request limit, and cache folder, at most 16.

## Relate

`thinkthen_relate(query, rules)` runs `query` on the calling database and asks the engine how its rows relate. The query returns `id, name, kind` or `id, name`. Rows with the same name and kind become one entity, and each edge returns one row for every pair of their ids. A two-column query reads every kind as `*`, so each rule must be bare or `*:*`. The rules are a list such as `['works_for=person:organization', 'same_as']`, a rules file's JSON, or `'@rules.json'`, read through the caller's own file system.

```sql
SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM staff', ['works_for=person:organization']);
```

- The query runs read-only as one `SELECT`. A statement that writes, attaches, loads, or changes a setting reads `thinkthen usage: the relate query must be a SELECT; relate reads records, it does not write files, attach databases, change settings, or load extensions`.
- Relate reads at most 255 rows. More reads `thinkthen usage: the relate query returned more than 255 rows, and relate reads at most 255; add a WHERE or a LIMIT`. This cap is stricter than the engine's own cap on unique pairs.
- `SET thinkthen_relate_seconds = N` bounds the query, 60 by default and 0 for none. A query past it reads `thinkthen deadline: the relate query ran past its N-second limit and was stopped; filter the rows first or raise SET thinkthen_relate_seconds (0 turns the limit off)`. The engine call shares the same limit. `memory_limit` stays the hard bound for a step that holds its whole input, such as a sort or a join build.
- `SET thinkthen_relate_holding_rows = N` refuses, before the query runs, a plan whose estimate feeds more than N rows into a sorting, grouping, windowing, or joining step, 1,000,000 by default.
- Relates on one database run one at a time. A relate that waits past its limit reads `thinkthen deadline: the relate query waited past its N-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off)`.
- A relate inside a relate's query reads `thinkthen usage: the relate query calls thinkthen_relate while its own query is running; nested relate cannot run, because the outer query waits on the connection the inner one needs`.
- A table function cannot run SQL on its caller's connection, so LOAD opens one connection of its own on each database (ADR 0038). Relate therefore sees committed tables, not the caller's temporary tables or open transaction. A temporary table reads `thinkthen local: the relate query names the temporary table NAME, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables`.
- Each loaded database carries an in-memory probe database named `thinkthen_instance_` and 32 random hex characters. Relate finds the caller's own database through it, never through a database name. A read-only database cannot carry one, so relate refuses there. The extension closes its connection once the database has no other, so a closed file database frees its lock.
- A Ctrl-C stops a running relate query and its engine call within 100 ms.

## Files and access

An `'@file'` question opens through the calling database's own file system, so `enable_external_access`, `allowed_directories`, `allowed_paths`, and `disabled_filesystems` decide every read, and the extension copies none of them. A cache folder set from SQL passes the same check. `thinkthen_warm` reads `'@file'` through its database's own connection, so the same database-wide file settings decide, and it takes a banded question and ignores the band. It refuses a `'@~'` path, because `home_directory` is a session setting it cannot see, and it refuses while a relate query runs on the database. Warm reads no session setting. It takes its cache folder from `THINKTHEN_CACHE` alone and runs at the process's throttle, so set `THINKTHEN_CACHE` before the process starts. An `'@file'` read stops at 1 MiB and reads `thinkthen local: the question file PATH was not read: it holds more than 1 MiB`.

## Interrupts

LOAD takes SIGINT and chains to the host's own action. A Ctrl-C stops the running query's engine call within 100 ms, and the next query answers normally. A host that ignores SIGINT keeps ignoring it. DuckDB's own `con.interrupt()` does not stop a held batch before its replies arrive; a SIGINT does, within 100 ms. The C API gives a scalar no view of DuckDB's interrupt, and `sdlc/issues/2026-09-23-duckdb-scalars-through-the-c-api-for-a-query-hook.md` names the lever.

## Build and check

`tools/setup.sh --fetch` is the one networked step: it fetches the stock v1.5.5 CLI into `~/.cache/thinkthen-toolchains/duckdb/v1.5.5/` and checks its sha256 against `tools/version.env`. `tools/setup.sh` alone makes the test venv offline. `check.sh PORT` then builds the shipped extension and the `test-hooks` build, runs the source checks and deny, loads the extension in the stock CLI, and runs the suites under `tools/`, each on its own loopback backend with a fake key. The `surfaces` rung runs it.
