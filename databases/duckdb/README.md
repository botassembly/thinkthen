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
- `SET thinkthen_max_requests_total = N` caps the requests this process sends. Before each call the extension adds up what its engines have sent. A spent total reads `thinkthen usage: this process has spent its request total of N; raise SET thinkthen_max_requests_total or RESET it` and sends nothing. A call with more texts than remain sends only the first ones that fit, then raises the same sentence. Calls running at the same time can each spend what remains, so the total can be passed by one call per thread in flight, plus retries. A forked child starts from zero. `thinkthen_warm` reads no session setting, so the total does not bind it.

A bad value's `SET` succeeds, since DuckDB has no check step for an extension setting, and the next call refuses it. The refusal uses the setter's own sentence, so a bad `SET` never wraps into an accepted one. The extension keeps one engine per distinct throttle, request limit, and cache folder, at most 16.

## Files and access

An `'@file'` question opens through the calling database's own file system, so `enable_external_access`, `allowed_directories`, `allowed_paths`, and `disabled_filesystems` decide every read, and the extension copies none of them. A cache folder set from SQL passes the same check. `thinkthen_warm` cannot see a caller's settings, so it refuses `'@file'`; pass the file's text from DuckDB's own `read_text` instead. Warm reads no session setting. It takes its cache folder from `THINKTHEN_CACHE` alone and runs at the process's throttle, so set `THINKTHEN_CACHE` before the process starts. An `'@file'` read stops at 1 MiB and reads `thinkthen local: the question file PATH was not read: it holds more than 1 MiB`.

## Interrupts

LOAD takes SIGINT and chains to the host's own action. A Ctrl-C stops the running query's engine call within 100 ms, and the next query answers normally. A host that ignores SIGINT keeps ignoring it. DuckDB's own `con.interrupt()` does not stop a held batch before its replies arrive; a SIGINT does, within 100 ms. The C API gives a scalar no view of DuckDB's interrupt, and `sdlc/issues/2026-09-23-duckdb-scalars-through-the-c-api-for-a-query-hook.md` names the lever.

## Build and check

`tools/setup.sh --fetch` is the one networked step: it fetches the stock v1.5.5 CLI into `~/.cache/thinkthen-toolchains/duckdb/v1.5.5/` and checks its sha256 against `tools/version.env`. `tools/setup.sh` alone makes the test venv offline. `check.sh PORT` then builds the shipped extension and the `test-hooks` build, runs the source checks and deny, loads the extension in the stock CLI, and runs the suites under `tools/`, each on its own loopback backend with a fake key. The `surfaces` rung runs it.
