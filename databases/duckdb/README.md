# The DuckDB surface

A loadable DuckDB v1.5.5 extension that puts the `thinkthen` engine behind SQL. The Linux x86_64, Linux ARM64, Apple Silicon and Intel macOS release routes use DuckDB's C++ API and a Rust bridge to `thinkthen`'s public API (tickets 0201 and 0231). The Intel package has focused installed proof under Rosetta on macOS 26; native Intel hardware, macOS 15 and release-runner proof remain open.

## Functions

| SQL | Returns |
| --- | --- |
| `thinkthen_decide(question, text[, settings])` | `BOOLEAN`; `NULL` is "not sure" |
| `thinkthen_choose(question, text[, options_or_settings[, settings]])` | `VARCHAR`, or `NULL` below the cut or at an exact tie; `options` may be `VARCHAR[]`, and settings are JSON-object `VARCHAR` |
| `thinkthen_find(question, units[, settings])` | a struct with original `index`, `value`, `probability` and ordered `(index, probability)` candidates |
| `thinkthen_score(question, text[, levels_or_settings[, settings]])` | `DOUBLE`, the position from 0 for the first level |
| `thinkthen_tag(question, text[, labels_or_settings[, settings]])` | `VARCHAR[]` |
| `thinkthen_decide_many` / `thinkthen_choose_many(question, keyed_json[, settings])` | table rows `(key, value, probability)` |
| `thinkthen_score_many` / `thinkthen_tag_many(question, keyed_json[, settings])` | table rows `(key, value)` |
| `thinkthen_rank(question, keyed_json[, settings])` | table rows `(key, rank, probability)` as `VARCHAR, BIGINT, DOUBLE`, best first |
| `thinkthen_plan(question, keyed_json[, settings])` | a native struct with records, prepared requests, byte and token estimates, an upper-bound flag, and the first request body |
| `thinkthen_annotate(set, text[, settings])` | `VARCHAR`, the record's values as JSON |
| `thinkthen_details(question, text[, settings])` | the command's `--details` line, as JSON text |
| `thinkthen_try_details(question, text[, settings])` | an answered JSON envelope, or a safe failed envelope for a recoverable row error |
| `thinkthen_recognize(text, kinds[, settings])` | a list of `(text, start, end, length, kind, strength)` |
| `thinkthen_relations(text, file[, settings])` | a list of `(relation, source, source_kind, target, target_kind, probability, either)`; `either` is true for a both-ways rule, whose ends are then in the order found |
| `thinkthen_relate(query, rules[, settings])` | a table of `(relation, source, target, probability, either)`, one row per edge between the query's ids; `either` is true for a both-ways rule, whose ends are then in query order |
| `thinkthen_usage()` | rows `(metric, value)` for `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens` |

`WHERE` provides the filter form. `thinkthen_find` judges one ordered list per call; build a list from rows with `list(unit ORDER BY ordinal)` to preserve caller order. It accepts 2–255 nonblank units, or 2–254 when `{"none":true}` is in settings, and at most 16 MiB of unit text. Equal units keep separate original indexes. A selected none has a non-NULL result with NULL `index` and `value`; a top-level SQL NULL or empty list returns SQL NULL without sending. The question is literal text, including text beginning with `@` or looking like JSON. The earlier four C++ target packages have installed find proof; the Intel result is translated macOS 26 proof, with native Intel and macOS 15 release-runner checks still open. Call settings are one JSON object: `threshold`, question members, `model`, `batch`, `context`, `deadline_ms`, and find's `none` where the verb permits them. `NULL` settings and `{}` mean no call settings. `deadline_ms` replaces positional deadline and context slots; `-1` means no deadline, and `0` is spent. A `NULL` question or text gives a `NULL` row. A failure is an error whose text starts `thinkthen <kind>: `, with one of the six kinds, and never reads as `NULL`.

`thinkthen_rank` orders one keyed JSON object of keys to text by the probability of yes. `rank` runs 1, 2, 3 with no gaps, and `probability` is the yes probability that orders the rows. All the records go to one engine call, which packs them as `thinkthen_decide_many` does, and `{"batch":N}` sets the records per request. The question is literal text, including text beginning with `@` or looking like JSON. Settings take `model`, `batch`, `context` and `deadline_ms`; `threshold`, `true`, `false`, `options`, `levels`, `labels` and `none` are usage errors before any send. Equal probabilities keep the keyed object's member order. An empty object gives no rows, and a `NULL` question or object gives no rows, all without sending. The input follows the `_many` rules: repeated keys and non-text values are usage errors, and blank text fails before any send. Build the object from rows with `json_group_object`, and `ORDER BY rank` again after a join:

```sql
SELECT t.id, refund.rank, refund.probability
FROM tickets t
JOIN thinkthen_rank('Does the writer ask for a refund?',
                    (SELECT json_group_object(id, body) FROM tickets)) refund ON refund.key = CAST(t.id AS VARCHAR)
ORDER BY refund.rank;
```

`thinkthen_plan` accepts a keyed JSON object of text values and the same question and settings grammar as `_many`. Its `estimated_input_tokens` struct has `lower` and `upper` members; `first_body` holds the exact first prepared request or SQL NULL for empty input. Counts describe preparation before cache answers, retries or refusals. Planning needs no API key and sends no request; its token band is an estimate, not a provider bill. A positive `thinkthen_max_requests_total` still limits later actual sends.

A question is plain text, `'@path.json'`, or the question file's JSON. Choose, score, and tag take plain text and put their members in the list. `thinkthen_annotate` takes a question set file or its JSON. `start`, `end`, and `length` count code points, as DuckDB's string indexing does.

```sql
LOAD 'build/thinkthen.duckdb_extension';
SELECT id FROM (
  SELECT id, thinkthen_decide('Does the writer ask for a refund?', body) AS asks_refund FROM tickets
) WHERE asks_refund;
SELECT id, thinkthen_choose('Which team owns this?', body, ['billing', 'shipping']) AS team FROM tickets;
SELECT best_passage FROM (
  SELECT thinkthen_find('Which statement matches?', list(body ORDER BY id), '{"none":true}') AS best_passage
  FROM passages
) AS judged WHERE best_passage IS NOT NULL;
```

## Constrain a stored answer

`thinkthen_choose` returns plain `VARCHAR`. Cast its result to a caller-owned DuckDB `ENUM` when storing a fixed label set:

```sql
CREATE TYPE team_label AS ENUM ('billing', 'shipping');
CREATE TABLE judged (id BIGINT, team team_label);
INSERT INTO judged
SELECT id, CAST(thinkthen_choose('Which team owns this?', body, ['billing', 'shipping']) AS team_label)
FROM tickets;
```

The cast refuses a label outside the enum. A stored `NULL` can represent a choice below the cut or an exact tie. The cast and column admit `NULL`; input-`NULL` behavior follows the function's argument rules. A failed ThinkThen call raises its named error and does not produce a row to cast. Do not catch that error and store `NULL` as if it were uncertainty. This is a constraint on the stored answer, not a change to the function's return type.

## Run facts

`thinkthen_details(question, text)` returns the command's scalar `--details` shape as JSON text, schema `thinkthen.result/1`. It omits record `input`, even when the engine packs several texts. Read a member with DuckDB's JSON functions, such as `thinkthen_details(q, t) ->> '$.meta.usage.input_tokens'`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each actual request, including a refused parent before one split; `meta.batch` names a packed member. `meta.url` names the address that answered. A field the backend did not report is absent. No call reports cost or time yet.

The details digest includes a question's saved calibration `profile`. A different runtime `thinkthen_profile` name appears as `meta.profile_warning` with `tuned_for` and `running`; the selected profile checks limits before sending.

`thinkthen_usage()` returns this process's running totals of requests sent, cache answers and tokens.

`thinkthen_try_details` returns `{"status":"answered","details":...}` with the full scalar details object, or `{"status":"failed","error":{"kind":"usage","message":"check the row's question and arguments, or raise the process request total when it is spent","retryable":false}}`. Compatible distinct rows pack under the selected batch setting; a recoverable usage, local, or backend failure affects only its request's members and does not stop later requests. One partly answered reply keeps good and failed members beside each other without another send. SQL NULL question or text returns SQL NULL; NULL settings mean absent. Unresolved answers stay answered with JSON `null` in their details. Failed values omit questions, evidence, keys, paths, and backend addresses. Cancellation, deadlines, and defects still stop the statement.

## Settings

`SET thinkthen_backend = 'typesafe'` selects a built-in or configured backend for the calling session. `RESET thinkthen_backend` restores captured environment selection. An explicit empty name refuses. SQL accepts no address or key. The Rust engine captures the selected key from the process environment when it builds. Connections with different backend names retain separate engines. The existing limit of 16 resident engines, held-plan refusal, idle eviction and process request totals still apply.

The engine starts from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE`. SQL cannot name a backend address or a key. DuckDB reads these caller-session settings before each call:

- `SET thinkthen_batch = 'max'` fills each compatible vector group up to the backend's limits. `SET thinkthen_batch = '1'` restores one distinct text per request and its old no-context wire identity; another positive decimal sets a member cap. `RESET thinkthen_batch` returns to `THINKTHEN_BATCH`, then a loaded question's `batch`, then `max`. Invalid text is refused before transport. A literal nonblank context in call settings is shared across one request and changes `meta.context_sha256` and request digests, not the question digest.
- `SET thinkthen_throttle = N` caps live requests in flight at N, from 1 through 32. Live requests are capped by the active throttle, or by 8 when none is set. The first throttle holds for the life of the process, and a different one reads `thinkthen usage: throttle 8 is already active for this process; use throttle 8 or drop the throttle argument`.
- `SET thinkthen_max_requests = N` caps one call's requests. One call covers one chunk of at most 2,048 rows, so the cap does not bound a whole query. `sdlc/issues/2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md` names the lever.
- `SET thinkthen_max_request_bytes = N` sets a positive request-byte ceiling for relation plans. `RESET` keeps the environment value. A single unsplittable question still goes alone.
- `SET thinkthen_cache = '/absolute/folder'` moves the cache. The folder must be an absolute local path with no scheme, and the calling database's own file settings must allow it. Each call checks both before it asks anything.
- `SET thinkthen_cache = 'off'` disables answer caching for that session. `RESET thinkthen_cache` restores the environment's cache choice: `THINKTHEN_CACHE`, or no cache when it is unset.
- `SET thinkthen_model = 'name'` names a model for questions that do not name one. `SET thinkthen_timeout = N` bounds each live attempt to a positive whole number of seconds. `SET thinkthen_max_retries = N` permits a whole number of retries from zero upward.
- `SET thinkthen_profile = '{...}'` applies one inline version-one backend limits profile. `SET thinkthen_record = '/absolute/folder'` writes every exchange. `SET thinkthen_replay = '/absolute/folder'` reads saved exchanges without a send. SQL record and replay folders follow the caller's current file permissions. A replay miss is a local error and sends nothing.
- `SET thinkthen_max_requests_total = N` caps live sends across all engines and sessions in this process. `N` may be zero. Each actual attempt, including a retry or split half, reserves one unit immediately before transport. A spent total reads `thinkthen usage: this process has spent its request total of N; raise SET thinkthen_max_requests_total or RESET it` and sends nothing further. A packed call may fit several rows in one attempt. When a later attempt is denied, `thinkthen_try_details` keeps completed answers at their SQL positions and returns safe failed Usage values for denied or unstarted rows. Ordinary scalar and vector calls still raise. A forked child starts from zero. The process adds its sends to the command's count-only usage totals when it exits, so `thinkthen status` shows them (ADR 0113).

The answer cache is off unless `THINKTHEN_CACHE` or `thinkthen_cache` names a folder; the platform cache folder is never used (ticket 0318). Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it. So a call refuses, before any request, a named cache, record or replay folder that another user owns or others can write; keep it at mode 0700. `cache prune` is the only thing that removes entries. Vector and keyed calls in one session share the selected folder and model; another session keeps its own settings.

A bad value's `SET` succeeds, since DuckDB has no check step for an extension setting, and the next call refuses it. The refusal uses the setter's own sentence, so a bad `SET` never wraps into an accepted one. The extension keeps at most 16 resident engines, keyed by all settings that affect an engine. A new plan retires the least recently used idle engine and keeps its requests and tokens in the process total. When all 16 plans are held, the new plan refuses: `16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process`.

The C++ statement owner shares one `SET thinkthen_query_budget_ms = N` clock across expressions and chunks. `-1` turns the budget off, `0` is already spent, and a positive value sets milliseconds. A spent budget refuses before another model send.

## Relate

`thinkthen_relate(query, rules)` runs `query` on the calling database and asks the engine how its rows relate. The query returns `id, name, kind` or `id, name`. Rows with the same name and kind become one entity, and each edge returns one row for every pair of their ids. A two-column query reads every kind as `*`, so each rule must be bare or `*:*`. The rules are a list such as `['works_for=person:organization', 'same_as']`, a rules file's JSON, or `'@rules.json'`, read through the caller's own file system.

```sql
SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM staff', ['works_for=person:organization']) AS works_for;
```

- The query runs read-only as one `SELECT`. A statement that writes, attaches, loads, or changes a setting reads `thinkthen usage: the relate query must be a SELECT; relate reads records, it does not write files, attach databases, change settings, or load extensions`.
- Relate reads at most 255 rows. More reads `thinkthen usage: the relate query returned more than 255 rows, and relate reads at most 255; add a WHERE or a LIMIT`. This cap is stricter than the engine's own cap on unique pairs.
- `SET thinkthen_relate_seconds = N` bounds the query, 60 by default and 0 for none. A query past it reads `thinkthen deadline: the relate query ran past its N-second limit and was stopped; filter the rows first or raise SET thinkthen_relate_seconds (0 turns the limit off)`. The engine call shares the same limit. `memory_limit` stays the hard bound for a step that holds its whole input, such as a sort or a join build.
- `SET thinkthen_relate_holding_rows = N` refuses, before the query runs, a plan whose estimate feeds more than N rows into a sorting, grouping, windowing, or joining step, 1,000,000 by default.
- Relates on one database run one at a time. A relate that waits past its limit reads `thinkthen deadline: the relate query waited past its N-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off)`.
- A relate inside a relate's query reads `thinkthen usage: the relate query calls thinkthen_relate while its own query is running; nested relate cannot run, because the outer query waits on the connection the inner one needs`.
- Relate runs its SQL on a separate connection to the caller's database (ADR 0038). It therefore sees committed tables, not the caller's temporary tables or open transaction. A temporary table reads `thinkthen local: the relate query names the temporary table NAME, and relate runs on a separate connection, so it cannot see temporary tables`. Another missing-table error keeps DuckDB's catalog wording and adds `relate reads only committed tables on its separate connection; if you created this table in an open transaction, commit it before retrying`. The catalog error alone cannot tell a typo from a table the caller has not committed.
- The C++ bind selects the caller's database directly. Relate can read committed tables from a read-only file database through its separate read-only connection. It still cannot see the caller's temporary tables or uncommitted rows, and it never runs a mutating query. Closing the caller frees a file database's lock. ADR 0038 records the retired C API identity probe.
- A Ctrl-C stops a running relate query and its engine call within 100 ms.

## Files and access

An `'@file'` question opens through the calling database's own file system, so `enable_external_access`, `allowed_directories`, `allowed_paths`, and `disabled_filesystems` decide every read, and the extension copies none of them. SQL cache, record and replay folders pass the same check at execution. `thinkthen_warm` now refuses with `thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many`, and `thinkthen_probability` refuses with `thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank`, both without sending. An `'@file'` read stops at 1 MiB and reads `thinkthen local: the question file PATH was not read: it holds more than 1 MiB`.

## Interrupts

LOAD takes SIGINT and chains to the host's own action. A Ctrl-C stops the running query's engine call within 100 ms, and the next query answers normally. A host that ignores SIGINT keeps ignoring it. Before ticket 0201, DuckDB's own `con.interrupt()` does not stop a held batch before its replies arrive; a SIGINT does, within 100 ms. The C++ statement owner now observes DuckDB query interruption; a direct `con.interrupt()` held-call proof remains separate from the retained SIGINT check.

## Build and check

`tools/setup.sh --fetch` is the networked setup step. It verifies the stock v1.5.5 CLI, the pinned DuckDB C++ source and static archives, and the stock v1.5.4 CLI used for version refusal. `tools/setup.sh` alone checks those inputs and installs the pinned Python test requirements offline. `check.sh PORT` builds the C++ extension and Rust bridge, runs source and dependency checks, loads the extension in the stock CLI, and runs the loopback suites with fake keys. The old raw C API entry and its `libduckdb-sys` dependency are retired under ADR 0081; the Rust files `src/engines.rs` and `src/signal.rs` remain source imports of the C++ bridge. With `THINKTHEN_ARTIFACT` set to the release archive, the check unpacks the package and checks the installed binary without invoking a Rust build. The `surfaces` rung runs the check at its named integration checkpoint.
