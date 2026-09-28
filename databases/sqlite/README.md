# The SQLite extension

A loadable SQLite extension over the public `thinkthen` Rust API. It is the unpublished crate `thinkthen-sqlite` in its own Cargo workspace (ADR 0047). The build makes `target/release/libthinkthen0.so`. Copy it to `thinkthen.so` and load it:

```sql
.load ./thinkthen
SELECT thinkthen_warm('Is this a complaint?', body) FROM reviews;
SELECT id, body FROM (
  SELECT id, body, thinkthen_decide('Is this a complaint?', body) AS is_complaint FROM reviews
) WHERE is_complaint;
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
| `thinkthen_try_details(question, text[, deadline])` | an answered JSON envelope, or a safe failed envelope for a recoverable row error |
| `thinkthen_annotate(questions, text[, deadline])` | a JSON object keyed by question name |
| `thinkthen_warm(question, text)` | an aggregate: judges each distinct pair and returns the count |
| `thinkthen_usage()` | JSON totals: `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens` |
| `thinkthen_recognize(text, kinds)` | a table of `text, start, end, length, kind, strength` |
| `thinkthen_recognize_document(text, spec)` | complete recognize JSON with entities and any relation edges |
| `thinkthen_relate(table, id, name, kind, rule, …)` | a table of `relation, source, target, probability` |

A question is plain text for a decide question, JSON text starting with `{`, or `'@name'` for a question file. A question set for `thinkthen_annotate` takes the same three forms. A banded decide question goes to `thinkthen_decide`, `thinkthen_details`, and `thinkthen_warm` only. `thinkthen_warm` takes decide questions only, and ignores a band, so it fills the answers decide reads with the same question.

A NULL text answers NULL and sends nothing. A BLOB, a number, text holding a NUL byte, or text that is not UTF-8 raises `usage` before any send.

The deadline is milliseconds under ADR 0041. `-1` means none. `0` is already spent and sends nothing. A REAL is accepted when it is finite and whole. Any other value raises `usage` and names the value.

`thinkthen_recognize` takes its kinds as a comma list, a JSON recognize section, or `'@name'`. A spec with relations raises `usage`, because relations come from `thinkthen_relate`. `start`, `end`, and `length` count Unicode characters, as SQLite's `substr` does, so `substr(text, start + 1, length)` returns the name. With no kinds, every name has the kind `ENTITY`.

`thinkthen_recognize_document(text, spec)` takes the original text and a full version-one recognize JSON question file, a bare JSON recognize section, or `'@name'` for either form. It returns the public recognize object as JSON text: complete entities and, when rules were given, a `relations` array with complete source and target entities. The array is empty when no edge reaches the relation cut; it is absent when no rule was given. A SQL NULL argument returns SQL NULL and sends nothing. Invalid SQL types and inline specs raise `usage` before a send; an invalid rule in a file raises `local`. The call may send several requests as recognition finds names, assigns kinds, then tests relations. It remains direct-only and volatile like the other functions.

`thinkthen_relate` reads the named table's id, name, and kind columns with a nested read-only SELECT. Each rule is `NAME`, `NAME=SOURCE:TARGET`, or either one with an `either:` prefix, up to four rules. A single JSON relate section or `'@name'` also works. Rows with the same name and kind count as one entity, and each edge comes back once for every row holding its two ends. `source` and `target` carry the id column's values, so the result joins back to the table. At most 255 distinct name and kind pairs go in one call. A NULL or blank name or kind raises `usage` naming the row's id.

An error reads `thinkthen <kind>: <message>`, with ` (retryable)` after the kind when a retry could succeed. `cancelled` is `SQLITE_INTERRUPT`, `usage` is `SQLITE_CONSTRAINT`, `local` is `SQLITE_CANTOPEN`, and every other kind is `SQLITE_ERROR`.

## Constrain a stored answer

`thinkthen_choose` returns plain text. Put a `CHECK` on the caller's stored answer column:

```sql
CREATE TABLE judged (
  id INTEGER,
  team TEXT CHECK (team IN ('billing', 'shipping'))
);
INSERT INTO judged
SELECT id, thinkthen_choose('{"choose":"Which team owns this?","options":["billing","shipping"]}', body)
FROM tickets;
```

The check rejects another non-`NULL` label. With non-`NULL` arguments, `choose` returns `NULL` when the winner falls below the cut or the top two options tie exactly. A SQL `NULL` argument also propagates `NULL` without a judgment. The check permits either `NULL`. A failed ThinkThen call raises an error; do not turn it into `NULL` to pass the check. The check reads only the stored `team` value. It does not invoke a ThinkThen function inside the schema, which this extension refuses.

`thinkthen_try_details` lets a query keep later good rows after a usage, local, or backend failure. Its JSON is `{"status":"answered","details":...}` or `{"status":"failed","error":{"kind":"usage","message":"check the row's question and arguments, or raise the process request total when it is spent","retryable":false}}`. The answered `details` is the full `thinkthen.result/1` object. A SQL NULL question or text returns SQL NULL. An unresolved answer returns an answered envelope with JSON `null` in its details. Failed values use fixed advice and omit the question, evidence, key, file path, cache path, and backend address. Interrupts, deadlines, and defects still raise SQL errors.

## Run facts

`thinkthen_details(question, text)` returns the command's `--details` line for one text, schema `thinkthen.result/1`. Read a member with `json_extract`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. No call reports cost or time yet.

`thinkthen_usage()` returns this process's running totals of requests sent, cache answers and tokens.

## The engine and its settings

The extension holds one engine for the process, shared by every connection. It builds on the first call that can send, from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE`, as the command reads them. SQL cannot name an address or a key. `thinkthen_usage()` does not build the engine. Before the first call every total reads 0.

Five setting functions change the engine before it builds. Each returns its argument and sends nothing. A bad value raises `usage` at the setting call. A setting after the engine builds raises `usage`.

`thinkthen_budget_ms(n)` is separate from those engine settings. Set it in a separate statement immediately before the query. It starts one monotonic budget on that connection: `-1` clears it, `0` is spent, and a positive whole number sets milliseconds. Every ThinkThen call on that connection uses the remaining time, or its shorter per-call deadline. Expiry during a held send returns a deadline error promptly; the sent attempt stays counted. The budget remains in force across later statements until reset. It bounds ThinkThen work only; SQLite work after the last ThinkThen call remains the host's responsibility. Other connections have independent budgets.

- `thinkthen_throttle(n)`: requests in flight at once, from 1 through 32. The default is 4.
- `thinkthen_max_requests(n)`: the most records one engine call may answer. `NULL` means no limit. Each scalar row is its own one-record call, and each warm flush is one call of up to 256 rows, so this limit does not cap a statement's spending. Use the total below for that.
- `thinkthen_max_requests_total(n)`: the most requests this process may send, summed over every call. It is unset by default, and `NULL` unsets it. Before each call the extension adds up the requests sent so far. Once the total is spent, every call raises `usage` and sends nothing, even a call the cache could answer. Otherwise a warm pass judges at most as many rows as requests remain, then raises `usage` saying it stopped at the remaining total. The cut counts rows, and a cached row costs no request, so a cut warm can spend less than what remained. The judged part of a cut warm stays in the cache, so a later process reads those rows with no send. Settings apply before the first call, so the total is lifted only in a new process. It holds to within one call's retries for the scalars and `thinkthen_warm`. A `thinkthen_recognize`, `thinkthen_recognize_document`, or `thinkthen_relate` call counts as one record but may send several requests, so it can pass the total by that call's own requests as well. Calls running at the same time can each spend what remains, so the total can be exceeded by one call per thread in flight, plus retries. A forked child starts again from zero. `thinkthen status` never sees this spend, because it counts only what the command sends.
- `thinkthen_cache(folder)`: the answer cache's folder. `NULL` turns the cache off.

The throttle holds per loaded copy of the engine. A process that also loads another surface's native package, such as a Python wheel, holds two copies and can run up to twice the throttle (ADR 0047 item 5).

## The cache

Answers go to the engine's disk cache and outlive the process. A warm pass fills the cache, and the queries after it read it. With `thinkthen_cache(NULL)` a warm pass still judges every row, and the queries after it send again.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. `cache prune` is the only thing that removes entries. Turn it off with `thinkthen_cache(NULL)`.

## Authority: who may do what

- **Files.** A call reads only the file its `'@name'` argument names, resolved against the process working directory. The file must be a regular file of at most 1 MiB. It is opened once without blocking, so a fifo or a device refuses. The extension confines nothing and follows symlinks. A parsed file is re-read when its modified time or size changes.
- **Schema.** Every function and both table-valued functions are direct-only. A view, trigger, DEFAULT, CHECK constraint, generated column, or index in a database file cannot call them, whatever `trusted_schema` says. No function is deterministic, so no index expression can hold one.
- **Credentials.** The key comes from `THINKTHEN_API_KEY` and goes only to the address the environment names. No message carries it.
- **Cancellation and threads.** Every call that can send runs on its own worker thread. The calling thread waits and checks `sqlite3_is_interrupted` on its own connection every 50 ms. An interrupt, or Ctrl-C in the CLI, cancels the call and returns `thinkthen cancelled` at once. The detached worker finishes the requests it already sent and starts no new one. This is the known exception to ADR 0017's rule that no thread outlives a call: a detached worker lives until its sent requests end, at most the 30-second request timeout, and holds its throttle permits until then. The extension pins itself in memory, so closing the loading connection never unmaps a worker's code.

## Building and checking

`setup.sh` puts the SQLite 3.50.0 amalgamation under `~/.cache/thinkthen-toolchains/` once. It copies from a folder you name, or fetches the zip from sqlite.org, and checks the two hashes in `amalgamation.sha256`. `tests/host_sqlite.sh` builds the host library and CLI from it. `check.sh` never fetches. Without the toolchain it reports "not run" and exits 77.

`check.sh` runs the formatter, Clippy, and the unit tests, builds the release library with the home path remapped, and checks one exported symbol and one panic guard. It then runs each Python test under a 300-second limit from `sdlc/scripts/time-limit`. Every test child starts its own loopback backend and its own cache, configuration, and usage folders, with the caller's key removed. `tests/conformance.py` runs every shared case in its own child and reports each as pass, FAIL, or not run with its reason.
