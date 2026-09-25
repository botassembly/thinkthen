# The SQLite extension

A loadable SQLite extension over the public `thinkthen` Rust API. It is the unpublished crate `thinkthen-sqlite` in its own Cargo workspace (ADR 0047). The build makes `target/release/libthinkthen0.so`. Copy it to `thinkthen.so` and load it:

```sql
.load ./thinkthen
SELECT thinkthen_warm('Is this a complaint?', body) FROM reviews;
SELECT id, body FROM reviews WHERE thinkthen_decide('Is this a complaint?', body);
```

The extension needs SQLite 3.50.0 or newer. Below 3.50.0 a CHECK constraint in a database file from somewhere else can reach a volatile function, so the load refuses and names the host's version.

## The functions

| Function | Answers |
|---|---|
| `thinkthen_decide(question, text[, deadline])` | 1, 0, or NULL for unsure |
| `thinkthen_choose(question, text[, deadline])` | the chosen label, or NULL |
| `thinkthen_score(question, text[, deadline])` | the position from 0 to K−1, as REAL |
| `thinkthen_tag(question, text[, deadline])` | a JSON array of labels |
| `thinkthen_details(question, text[, deadline])` | the command's `--details` JSON document |
| `thinkthen_annotate(questions, text[, deadline])` | a JSON object keyed by question name |
| `thinkthen_warm(question, text)` | an aggregate: judges each distinct pair and returns the count |
| `thinkthen_usage()` | JSON totals: `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens` |
| `thinkthen_recognize(text, kinds)` | a table of `name, kind, start, end, strength` |
| `thinkthen_relate(table, id, name, kind, rule, …)` | a table of `relation, source, target, probability` |

A question is plain text for a decide question, JSON text starting with `{`, or `'@name'` for a question file. A question set for `thinkthen_annotate` takes the same three forms. A banded decide question goes to `thinkthen_decide` and `thinkthen_details` only. `thinkthen_warm` takes decide questions only.

A NULL text answers NULL and sends nothing. A BLOB, a number, text holding a NUL byte, or text that is not UTF-8 raises `usage` before any send.

The deadline is milliseconds under ADR 0041. `-1` means none. `0` is already spent and sends nothing. A REAL is accepted when it is finite and whole. Any other value raises `usage` and names the value.

`thinkthen_recognize` takes its kinds as a comma list, a JSON recognize section, or `'@name'`. A spec with relations raises `usage`, because relations come from `thinkthen_relate`. Offsets count Unicode characters, as SQLite's `substr` does, so `substr(text, start + 1, "end" - start)` returns the name.

`thinkthen_relate` reads the named table's id, name, and kind columns with a nested read-only SELECT. Each rule is `NAME`, `NAME=SOURCE:TARGET`, or either one with an `either:` prefix, up to four rules. A single JSON relate section or `'@name'` also works. Rows with the same name and kind count as one entity, and each edge comes back once for every row holding its two ends. `source` and `target` carry the id column's values, so the result joins back to the table. At most 255 distinct name and kind pairs go in one call. A NULL or blank name or kind raises `usage` naming the row's id.

An error reads `thinkthen <kind>: <message>`, with ` (retryable)` after the kind when a retry could succeed. `cancelled` is `SQLITE_INTERRUPT`, `usage` is `SQLITE_CONSTRAINT`, `local` is `SQLITE_CANTOPEN`, and every other kind is `SQLITE_ERROR`.

## The engine and its settings

The extension holds one engine for the process, shared by every connection. It builds on the first call that can send, from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE`, as the command reads them. SQL cannot name an address or a key. `thinkthen_usage()` does not build the engine. Before the first call every total reads 0.

Four setting functions change the engine before it builds. Each returns its argument and sends nothing. A bad value raises `usage` at the setting call. A setting after the engine builds raises `usage`.

- `thinkthen_throttle(n)`: requests in flight at once, from 1 through 32. The default is 4.
- `thinkthen_max_requests(n)`: the most records one call may answer. `NULL` means no limit.
- `thinkthen_cache(folder)`: the answer cache's folder. `NULL` turns the cache off.
- `thinkthen_cache_bytes(n)`: the cache's size cap in bytes.

The throttle holds per loaded copy of the engine. A process that also loads another surface's native package, such as a Python wheel, holds two copies and can run up to twice the throttle (ADR 0047 item 5).

## The cache

Answers go to the engine's disk cache and outlive the process. A warm pass fills the cache, and the queries after it read it. Raise `thinkthen_cache_bytes` before `thinkthen_warm` runs over a table larger than the cap. With `thinkthen_cache(NULL)` a warm pass still judges every row, and the queries after it send again.

## Authority: who may do what

- **Files.** A call reads only the file its `'@name'` argument names, resolved against the process working directory. The file must be a regular file of at most 1 MiB. It is opened once without blocking, so a fifo or a device refuses. The extension confines nothing and follows symlinks. A parsed file is re-read when its modified time or size changes.
- **Schema.** Every function and both table-valued functions are direct-only. A view, trigger, DEFAULT, CHECK constraint, generated column, or index in a database file cannot call them, whatever `trusted_schema` says. No function is deterministic, so no index expression can hold one.
- **Credentials.** The key comes from `THINKTHEN_API_KEY` and goes only to the address the environment names. No message carries it.
- **Cancellation and threads.** Every call that can send runs on its own worker thread. The calling thread waits and checks `sqlite3_is_interrupted` on its own connection every 50 ms. An interrupt, or Ctrl-C in the CLI, cancels the call and returns `thinkthen cancelled` at once. The detached worker finishes the requests it already sent and starts no new one. This is the known exception to ADR 0017's rule that no thread outlives a call: a detached worker lives until its sent requests end, at most the 30-second request timeout, and holds its throttle permits until then. The extension pins itself in memory, so closing the loading connection never unmaps a worker's code.

## Building and checking

`setup.sh` puts the SQLite 3.50.0 amalgamation under `~/.cache/thinkthen-toolchains/` once. It copies from a folder you name, or fetches the zip from sqlite.org, and checks the two hashes in `amalgamation.sha256`. `tests/host_sqlite.sh` builds the host library and CLI from it. `check.sh` never fetches. Without the toolchain it reports "not run" and exits 77.

`check.sh` runs the formatter, Clippy, and the unit tests, builds the release library with the home path remapped, and checks one exported symbol and one panic guard. It then runs each Python test under `timeout 300`. Every test child starts its own loopback backend and its own cache, configuration, and usage folders, with the caller's key removed. `tests/conformance.py` runs every shared case in its own child and reports each as pass, FAIL, or not run with its reason.
