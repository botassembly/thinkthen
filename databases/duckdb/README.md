# The DuckDB surface

Unsigned loadable extensions for DuckDB v1.5.5 and v1.5.4 that put the `thinkthen` engine behind SQL. The Linux x86_64, Linux ARM64, Apple Silicon and Intel macOS release routes use DuckDB's C++ API and a Rust bridge to `thinkthen`'s public API (tickets 0201 and 0231). The Intel package has focused installed proof under Rosetta on macOS 26; native Intel hardware, macOS 15 and release-runner proof remain open.

The archive uses DuckDB's repository layout: `v1.5.5/<platform>/thinkthen.duckdb_extension` and `v1.5.4/<platform>/thinkthen.duckdb_extension`. From its unpacked directory, run DuckDB with unsigned extensions allowed at startup:

```sql
INSTALL thinkthen FROM './';
LOAD thinkthen;
```

To load directly, use `LOAD 'v1.5.5/linux_amd64/thinkthen.duckdb_extension';` on the matching Linux x86-64 host, or the matching version/platform path on another supported host.

Use dbt v1 with `duckdb` 1.5.5 for the documented dbt route. dbt v2 requires signed extensions; this unsigned archive does not enable ThinkThen in dbt v2. Community listing and signing are in progress under ticket 0415. No community install is available until DuckDB accepts and builds the submission. A signed build for another DuckDB version does not establish dbt v2 compatibility. The [public install page](https://thinkthen.dev/install/duckdb/#use-the-release-from-dbt-v1) provides the pinned dbt v1 packages and complete startup profile.

## Discover function purposes

DuckDB's catalog describes every registered ThinkThen function, including SQL macros, native scalar overloads, table functions, image and file helpers, usage observation and removed calls that still refuse with migration advice:

```sql
SELECT function_name, function_type, parameter_types, return_type, description
FROM duckdb_functions()
WHERE starts_with(function_name, 'thinkthen_')
ORDER BY function_name, function_type, parameter_types;
```

Discovery creates no engine, reads no input files and sends no requests.

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
| `thinkthen_read_files(path[, reader_options])` | rows `(ordinal, record, file, first_line, last_line)`; path is text or a list of explicit paths |
| `thinkthen_span_lines(record, first_line, start, end)` | struct `(first_line, last_line)` mapped from native Unicode scalar offsets |
| `thinkthen_usage()` | rows `(metric, value)` for `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens` |
| `thinkthen_usage_status()` | live JSON persistence observation across resident engines and failures retained at eviction |

`thinkthen_usage_status()` returns a VARCHAR JSON object with `state`: `disabled`, `pending`, `written` or `failed`. Failed adds only the native fixed advice, `check the usage folder permissions and free space`. Observation builds no engine, sends no request and never waits on the registry or writer. Failure takes precedence, then pending work, then written current deltas; an empty registry is disabled. A busy registry reports pending observation. Written does not promise future calls or other processes. Eviction retains failure discovered by the existing retirement finalization; successful answers and historical `thinkthen_usage()` totals remain available.

Required SQL NULL operands return SQL NULL from scalar calls and zero rows from table or set calls before inspecting partner operands, session settings or files. Optional NULL settings mean omitted settings. NULL members inside a supplied list retain that function’s existing validation.

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

## Files and folders

`thinkthen_read_files` explicitly selects local files and recursively sorted folders. It defaults to physical lines. Reader options are JSON text: `{"unit":"file"}` retains whole documents and all original endings; `{"unit":"window","window":4}` selects nonoverlapping windows of four physical lines. Line/window units drop only the final LF or CRLF ending, skip whitespace units after size validation, and keep one-based inclusive physical positions. Operand lists retain their order and duplicate occurrences. Hidden regular files are included; descendant symlinks are skipped. Explicit file symlinks keep DuckDB's host policy. A manifest and each record have a 16 MiB bound. Unsupported entries and enumeration failures refuse before content admission; content opens and reads occur on demand and later failures stop the scan.

The executing DuckDB filesystem authorizes every operand and descendant, then C++ inspects only local metadata because the pinned DuckDB listing API omits unsupported entries and listing failures. All content opens and reads use DuckDB-authorized handles and honor `enable_external_access`, `allowed_paths`, `allowed_directories` and disabled filesystems. Prepared executions recheck permission. This strict reader requires the stock local filesystem with no custom filesystem registrations; remote/custom routes are refused. Rust frames the authorized handle and never opens a DuckDB path.

[All ten SQL examples](examples/files.sql) use the same [document folder](../../specification/fixtures/files/documents) and [question set](../../specification/fixtures/files/questions.json) as CLI and Python. Run them from the repository root after loading the matching extension. Their committed `documents` table lets relate's separate read-only connection see the complete source set. Row judgments retain source columns outside provider evidence and cache identity. Find maps its selected ordered index to the actual reader row; recognize calls the shared native span mapper; relate joins both endpoint ids to actual source rows. Document endpoints retain complete document ranges.

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

## Price and elapsed time

SQL plans report an estimated input-token band before cache hits, retries or refusal splits. They do not predict output tokens, total dollars or future duration. SQL usage reports cumulative process totals, not the facts of one isolated call. Packed request metadata can appear on more than one result row; summing those rows counts the same request more than once.

`thinkthen_plan` previews one judgment question, not an annotate question set or a whole SQL pipeline. `thinkthen annotate --plan` previews CLI question-set packing; match the question set, backend, model and record framing, and treat that preview as a guide for SQL calls whose packing can differ. Reduce the planned cohort before calls when the configured allowance cannot admit the pipeline. Estimates neither authorize a larger budget nor predict the provider's bill.

Measure wall time around the SQL statement in the client. This includes database and client work and is not engine-only time. For an external price estimate, apply a known input/output tariff to complete provider-reported usage with decimal arithmetic. The provider's invoice determines actual charges. Missing rates or incomplete attempt usage mean unknown cost, not zero. Cumulative SQL counters do not prove usage completeness for failed attempts, and subtracting shared counters cannot isolate concurrent calls.

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
- `SET thinkthen_profile = '{...}'` applies one inline version-one backend limits profile. `SET thinkthen_record = '/absolute/folder'` writes every exchange. `SET thinkthen_replay = '/absolute/folder'` reads saved exchanges without a send. SQL record and replay folders follow the caller's current file permissions. Recognize, nested relations and explicit relate can replay without a key when `thinkthen_max_requests_total` is zero or already spent. Their replay/cache lookup precedes actual-send quota reservation; a strict replay miss is a local error and sends nothing, even under a spent total. Input, file-access and cancellation guards still apply.
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

`tools/setup.sh --fetch` is the networked setup step. It verifies each supported version's stock CLI, Python module, pinned C++ source and static archives. `tools/setup.sh` alone checks those inputs and installs the pinned Python test requirements offline. `check.sh PORT` builds the C++ extension and Rust bridge, runs source and dependency checks, loads the extension in the stock CLI, and runs the loopback suites with fake keys. The old raw C API entry and its `libduckdb-sys` dependency are retired under ADR 0081; the Rust files `bridge/src/engines.rs` and `bridge/src/signal.rs` belong to the C++ bridge. With `THINKTHEN_ARTIFACT` set to the release archive, the check unpacks the package and checks the installed binary without invoking a Rust build. The `surfaces` rung runs the check at its named integration checkpoint.

## Explicit images (0.2 development)

`thinkthen_image(bytes BLOB, mime VARCHAR)` returns `STRUCT(media VARCHAR, data BLOB, file VARCHAR)`. Pass an ordered list of these structs. MIME is exactly `image/png` or `image/jpeg`; the native reader/decoder validates original compressed pixels. Each question takes 1–8 images and at most 24 MiB of compressed bytes. Order and duplicates are preserved. Source file names stay outside model evidence and cache identity.

`thinkthen_decide_images(question, images, text := NULL, settings := NULL)`, `thinkthen_choose_images`, `thinkthen_score_images` and `thinkthen_details_images` use the existing native image engine. The first three return their ordinary scalar value; the last returns native detailed JSON. Questions and settings use the existing text/JSON/question-file grammar. Choose options and score levels belong in question JSON or settings. Ancillary text is optional.

NULL constructor operands return NULL. NULL question or collection returns NULL without sending; NULL ancillary text means absent text and NULL settings means defaults. A NULL member inside a collection, empty collection, invalid media/pixels/tag or exceeded limit is Usage before sending. Unsure decide/choose is NULL; failures remain errors. Arbitrary BLOB/bytea, text, paths, URLs and generic JSON never become images implicitly. Other functions retain their text input contract.

`thinkthen_image_file(path)` explicitly reads one local regular image with DuckDB filesystem permissions and the native handle reader. Its `file` field retains the source name; a byte constructor has NULL `file`. No text line positions are invented. `thinkthen_recognize` also accepts the existing native recognize JSON/question-file grammar with described kinds in addition to its retained name list.


`thinkthen_rank_set(questions, keyed_json[, settings])` is the explicit additive
rank-set route. It takes the native version-one ordered set of named decide
questions, returns `key, rank, probability, question_name, facts`, and preserves
each original keyed identity even when several keys have equal text. Each
member sorts with the host's existing input-order ties; native turns visits
each depth in authored member order, consumes duplicate visits, and emits
each original once. `probability` and `question_name` belong to the selecting
member. Use `ORDER BY rank` after a join and `LIMIT` for the merged prefix;
every member still judges every record. The literal `thinkthen_rank` route
retains its existing signatures and behavior.

Settings take `batch`, `context` and `deadline_ms`; model/backend selection
uses the existing host configuration. Per-call model, cuts, pointers and
score members refuse before sending. One-member sets preserve the plain
question's wire/cache identity. Recording each member individually supports
strict set replay with zero sends. Described decide members retain their
authored true/false meanings. The additive `thinkthen_rank_complete` route also accepts described decide and
saved score questions through the native complete rank API.

`facts` comes directly from the same completed native call, counts original
records and combined member requests, and repeats on every output row. It
is the compatibility count-facts shape. Use `thinkthen_rank_complete` for full
observations, call/answer IDs and started-failure envelopes. An empty object yields
no rows. Set JSON and files use native RankSet admission, preserving member
order and rejecting duplicates, authored thresholds/on and non-decide kinds.

Questions/keyed input and facts are VARCHAR JSON text. Questions may name an
explicit `@file` through the existing DuckDB-authorized reader. NULL questions
or keyed input yields no rows without file IO or sends; NULL settings uses
defaults.

## Complete native calls (0.2 development)

All ten `thinkthen_FUNCTION_complete(question TEXT, inputs TEXT, settings TEXT :=
NULL)` calls expose the owning native result/2 envelope and final facts, including
started-failure facts, full probabilities, authors, call/answer/observation IDs,
structured stops, requested attempts and optional exact caller-priced cost.
Ordinary scalar/table signatures and cumulative usage remain compatible. See the
[shared SQL complete API](../sqlite/README.md#complete-native-calls-02-development)
for explicit records, context, reading, options, images, rank sets, file controls,
error envelopes and native cache/record/replay behavior.

`thinkthen_decide_complete(question, images, settings := NULL)` also accepts a native list of `thinkthen_image(blob, media)` or `thinkthen_image_file(path)` values. It evaluates the ordered collection as one image-only record, preserving duplicates and native BLOB payloads without a JSON byte array. Its text descriptor form remains available. Both forms use the same Request admission and result facts.

All ten complete calls admit a shared native Request before constructing the
engine and execute through the same Request owner as SQLite and PostgreSQL.
Generated SQL results retain the native envelope and its presence distinctions.
DuckDB returns JSON text. Explicit native file inputs read DuckDB-authorized
handles. Complete question `@reference` and `@@NAME` use native metadata
selection, then the executing session authorizes the selected path and reads
capped UTF-8 through DuckDB. The same retained selection parses the original
bytes; Rust never opens that question content. Engine options add
`thinkthen_base_url`, paired price strings and
`thinkthen_refresh_cache` (0/1); the same native engine owns cache, replay and
usage. No complete call creates another SQL scheduler or cumulative facts ledger.
