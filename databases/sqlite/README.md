# The SQLite extension

`thinkthen-sqlite` is a loadable extension over the public Rust engine (ADR 0047 and ADR 0105). Build its release library, copy `libthinkthen0.so` to `thinkthen.so`, and load it with `.load ./thinkthen`. SQLite 3.50.0 or newer is required. Default loading registers volatile, direct-only functions. Objects in main and attached schemas cannot call them to spend requests or read files. Caller-created TEMP objects remain callable, even with `trusted_schema=OFF`.

## Use reviewed views and ingestion triggers

On a fresh connection, load once through the explicit trusted entry point:

```sql
.load ./thinkthen sqlite3_thinkthen_trusted_init
PRAGMA trusted_schema=ON;
```

A native host passes `sqlite3_thinkthen_trusted_init` as the entry-point argument to `sqlite3_load_extension`. Default loading uses `sqlite3_thinkthen_init`. [entry-points.txt](entry-points.txt) declares the extension's exports. Switch modes by closing the connection and loading once on a fresh connection.

This choice permits reviewed schema SQL to spend paid requests and read permitted question files. Trust covers main, every attached schema and later attachments while `trusted_schema` is ON. It provides no per-view or per-file trust isolation. Keep downloaded or unreviewed databases on separate default-loaded connections.

Trusted registration permits judgment scalars, `thinkthen_plan` and judgment tables in views and triggers. SQLite may also evaluate judgments in CHECK and DEFAULT expressions. Configuration, usage, budget setters and removed spellings retain DIRECTONLY and remain unavailable from main and attached schema objects. Judgments retain neither INNOCUOUS nor DETERMINISTIC flags; SQLite still refuses non-deterministic generated columns and indexes. Loading changes neither `trusted_schema` nor the host's authorizer. With `trusted_schema=OFF`, judgments from main and attached schema objects refuse before reading a question file or sending a request. Top-level calls remain available. SQLite treats caller-created TEMP objects as caller SQL: TEMP views and triggers can call judgments, plans, controls and judgment tables in both loading modes, even with `trusted_schema=OFF`. Such calls can spend paid requests and read permitted question files. This flag does not revoke TEMP calls. The ordinary permissions, budgets, cancellation and errors still apply.

For example, a reviewed ingestion trigger can store a judgment:

```sql
CREATE TABLE messages(body TEXT, is_red INTEGER);
CREATE TRIGGER judge_message AFTER INSERT ON messages BEGIN
  UPDATE messages SET is_red=(SELECT thinkthen_decide('Is it red?', NEW.body) AS is_red)
  WHERE rowid=NEW.rowid;
END;
```

## Judge a table once

```sql
.load ./thinkthen
SELECT s.title, d.value AS on_abbey_road, d.probability AS p
FROM songs AS s
JOIN thinkthen_decide_many(
  'The text is the title of a song by the Beatles. It appears on the album Abbey Road.',
  (SELECT json_group_object(id, title) FROM songs),
  '{"threshold":"0.3:0.7"}') AS d ON d.key = CAST(s.id AS TEXT)
ORDER BY d.probability DESC;
```

The keyed input is a JSON object of original row keys to nonblank text; repeated decoded keys are usage errors. One table call packs the records in input order; `key` joins each answer back to its row. `thinkthen_decide_many` and `thinkthen_choose_many` return `(key, value, probability)`. `thinkthen_score_many` and `thinkthen_tag_many` return `(key, value)`; their probability column does not exist. A `key =` condition probes the already judged result and is excluded from its argument identity. SQLite still applies its own comparison, including NULL, TEXT affinity and collation; the host uses the fast lookup only for binary text equality. The connection holds eight most recently used packed answer sets, shares rows across scans, and frees them when it closes. Each slot retains the three argument byte strings, one key-to-row map and the answer rows, so its memory grows with that packed input; eight slots bound the number of retained sets, not their total bytes. An identical table call on that connection can reuse its result without another engine call; a changed question, keyed object or settings object occupies another slot. A changed named question file invalidates its matching slot.

Each SQL call still needs the caller's judgment about whether to spend. Correlated scalar calls can send once per source row. For a large join, aggregate its keyed JSON once, then join the table result. `tests/slide.sql` is a runnable small example of that shape.

## Rank a table once

```sql
SELECT s.title, abbey_road.rank, abbey_road.probability
FROM songs AS s
JOIN thinkthen_rank(
  'The text is the title of a song by the Beatles. It appears on the album Abbey Road.',
  (SELECT json_group_object(id, title) FROM songs)) AS abbey_road ON abbey_road.key = CAST(s.id AS TEXT)
ORDER BY abbey_road.rank;
```

`thinkthen_rank(question, keyed_json[, settings])` returns `(key, rank, probability)` as `TEXT, INTEGER, REAL`, best first. `rank` runs 1, 2, 3 with no gaps, and `probability` is the yes probability that orders the rows. It takes the same keyed object as `thinkthen_decide_many`, under the same input rules, connection slots and `key =` probe, and packs the records into one engine call. `{"batch":N}` sets the records per request. The question is literal text, including text beginning with `@` or looking like JSON. Settings take `model`, `batch`, `context` and `deadline_ms`; `threshold`, `true`, `false`, `options`, `levels`, `labels` and `none` are usage errors before any send. Equal probabilities keep the keyed object's member order. An empty object gives no rows without sending, and a NULL question or object raises `thinkthen usage`, as `thinkthen_decide_many` does. `ORDER BY rank` again after a join.

## Calls and settings

| Call | Return |
| --- | --- |
| `thinkthen_decide(question, input[, settings])` | 1, 0, or SQL NULL for unsure |
| `thinkthen_choose(question, input[, settings])` | chosen label or SQL NULL |
| `thinkthen_score(question, input[, settings])` | position on the named levels |
| `thinkthen_tag(question, input[, settings])` | JSON array text of labels |
| `thinkthen_details`, `thinkthen_try_details` with the same slots | result JSON, or an answered/failed envelope |
| `thinkthen_find(question, units_json[, settings])` | selected index, value and probabilities as JSON text |
| `thinkthen_annotate(questions, input[, settings])` | named judgments as JSON text |
| `thinkthen_recognize(input, kinds[, settings])` | rows of names, offsets, kinds and strengths |
| `thinkthen_relations(input, spec)` | complete recognition JSON, including relations |
| `thinkthen_relate(query, rules[, settings])` | rows of relation, source id, target id, probability, and `either`, 1 for an edge of a both-ways rule (its ends then in query order) and 0 otherwise |
| `thinkthen_plan(question, keyed_json[, settings])` | JSON text with planned records, requests, bytes, input token band and the first exact request body as `first_body_utf8` |
| `thinkthen_usage()` | cumulative request, cache-answer and reported token totals; the process also adds them to the command's usage totals when it exits, so `thinkthen status` shows them (ADR 0113) |
| `thinkthen_configure(json)` | the selected engine settings object, before engine build |

The optional settings slot is JSON text in the shared `thinkthen.settings/1` grammar. Question fields such as `threshold`, `options`, `levels`, `labels` and `model` affect question bytes; `context`, `batch` and `deadline_ms` control the call. Choose, score and tag can take a plain question when their members are in settings. Duplicate fields in a complete question and settings refuse before sending. `deadline_ms` is an integer: `-1` removes a call deadline, `0` is already spent, and positive values are milliseconds. The parser checks its range before a worker starts. Find alone accepts `none: true`. Aggregate verbs take call controls, not judgment fields, in their settings object.

`thinkthen_plan` validates the keyed input and settings and makes no backend request. Its `requests` count is prepared requests before cache answers, refusal splits or retries. `first_body_utf8` is the exact first planned request body, or JSON null for empty input; it can contain question and evidence text, so handle the plan as carefully as a request. It contains no key.

`thinkthen_configure` replaces the individual setters. Its closed object supports `backend`, `model`, `batch`, `cache`, `throttle`, `timeout`, `max_retries`, `max_request_bytes`, `max_requests`, `max_requests_total`, `profile`, `record` and `replay`. The whole object is checked before it replaces the previous selection, and the engine must not already have built. `thinkthen_usage()` does not build it; a plan does build the engine so configure before planning. `"backend":"typesafe"` selects a built-in or configured name. Its address and key variable come from Rust configuration. SQL accepts no address or key. Omit `backend` to retain environment selection. Configure validation reads the environment but retains no credentials. The first engine build captures them. Every connection then uses that process engine. A failed build allows a later configuration and build to recover. `max_requests_total` reserves each live attempt against the shared process count, including later packed requests and retries. Cache hits and plans send nothing.

The old `thinkthen_warm`, `thinkthen_probability`, `thinkthen_recognize_document`, individual setters and positional deadline/context forms refuse with migration guidance. Read probability from the same decide or choose `_many` row as its value, and order records with `thinkthen_rank`. A NULL question or input returns SQL NULL; malformed types and settings raise `thinkthen usage` before a send. Ordinary errors read `thinkthen <kind>: <message> (retryable: yes|no)` with kinds `usage`, `local`, `backend`, `cancelled`, `deadline` and `defect`; removed calls keep plain migration messages. `thinkthen_try_details` returns fixed safe advice for a recoverable row failure; cancellation and deadlines still raise errors.

`thinkthen_find` accepts an ordered JSON array of 2–255 nonblank text units, or 2–254 with `{"none":true}`. Duplicates retain separate zero-based positions. Empty input and SQL NULL return SQL NULL. `thinkthen_recognize` keeps the `text, start, end, length, kind, strength` row shape, with character offsets matching SQLite `substr`. `thinkthen_relations` accepts a full or bare recognize spec or an `@file`.

For `thinkthen_recognize`, pass one kind as `'person'`, or comma-separated names as `'person,organisation'`. To supply descriptions, pass a recognize JSON object such as `'{"kinds":{"person":"A human name."}}'`, a full versioned recognize spec, or an `@file` containing that spec. A bare JSON array string such as `'["person"]'` currently names one literal kind, `["person"]`. Use `'person'` to request the person kind.

```sql
SELECT text, kind FROM thinkthen_recognize('Maria Chen called.', 'person') AS recognized_names;
```

`thinkthen_relate` runs a caller-supplied read-only `SELECT` yielding `id, name` or `id, name, kind` on the same connection. `rules` is one inline rule, a JSON array of rules, a JSON relate spec or `@file`. At most 255 distinct name/kind pairs enter a call. Equal pairs share one entity, and each answer edge expands to the ids that held its endpoints. Blank names/kinds and a 256th pair raise usage before a send.

## Price and elapsed time

SQL plans report an estimated input-token band before cache hits, retries or refusal splits. They do not predict output tokens, total dollars or future duration. SQL usage reports cumulative process totals, not the facts of one isolated call. Packed request metadata can appear on more than one result row; summing those rows counts the same request more than once.

Measure wall time around the SQL statement in the client. This includes database and client work and is not engine-only time. For an external price estimate, apply a known input/output tariff to complete provider-reported usage with decimal arithmetic. The provider's invoice determines actual charges. Missing rates or incomplete attempt usage mean unknown cost, not zero. Cumulative SQL counters do not prove usage completeness for failed attempts, and subtracting shared counters cannot isolate concurrent calls.

## Runtime boundaries

The engine is shared by connections in this loaded copy. `thinkthen_budget_ms(n)` is separate from engine configuration and belongs to one connection: `0` spends it, `-1` clears it, and positive milliseconds run from that statement onward. Calls use the shorter remaining connection budget or call deadline. Every sending call runs on a detachable worker; the SQLite thread checks interruption every 50 ms and cancels promptly. A sent request remains counted. The library stays mapped until process exit so a detached worker can finish safely.

Questions may be plain decide text, inline JSON or a named `@file`; question sets, recognition and relation specs use their documented JSON/file forms. The file door follows symlinks, opens a regular file nonblocking and refuses files above 1 MiB. Parsed named files are re-read when modification time or size changes. The answer cache is off unless `THINKTHEN_CACHE` or the `"cache"` setting names a folder; the platform cache folder is never used (ticket 0318). Answer cache and recordings contain question and evidence text, in plain text, with no expiry. Whoever can write such a folder controls its answers, so a call refuses, before any request, a named cache, record or replay folder that another user owns or others can write; keep it at mode 0700. Recordings store bodies, never headers. Strict replay misses send nothing. The cache is not a pricing or authorization ledger.

`setup.sh` installs the pinned SQLite 3.50.0 amalgamation into the local toolchain cache once. `check.sh` builds a matching extension offline, verifies the declared exported entry points and the panic guard, then runs fixtures on the pinned host. `tests/helper.py` supplies isolated loopback children with clean environments; `tests/conformance.py` selects cases with an absolute `THINKTHEN_CONFORMANCE_IDS` file. A source test, a copied installed package and the release runner are separate qualifications.

## Explicit files and folders

`thinkthen_read_files(path[, reader_options])` returns `ordinal, record, file, first_line, last_line`. A path is explicit text or a JSON array of paths; strings passed to judging functions remain ordinary evidence. Reader options are JSON text and default to line units. Use `{"unit":"window","window":4}` for nonoverlapping physical line windows or `{"unit":"file"}` for exact whole documents. Positions are one-based and inclusive. Folder descendants sort by relative path, include hidden regular files and skip descendant symlinks. Explicit file symlinks retain the native open policy. Operand order and duplicate occurrences are preserved. A manifest and each record have a 16 MiB bound. Invalid options, unsupported operands and enumeration failures refuse before admission; one active handle reads content on demand and a later content failure stops the scan.

```sql
-- Run from the repository root after loading the SQLite extension.
SELECT ordinal, record, file, first_line, last_line,
       thinkthen_decide('Does this document contain a support contract?', record) AS value
FROM thinkthen_read_files('specification/fixtures/files/documents', '{"unit":"file"}');
```

Files use the caller's native filesystem permissions. The reader remains DIRECTONLY even under `sqlite3_thinkthen_trusted_init`: stored main/attached views and triggers cannot read files. Caller-created TEMP objects retain SQLite's availability under either trusted_schema setting. A reader's source coordinates stay outside judging payloads and cache keys. `thinkthen_span_lines(record, first_line, start, end)` returns JSON with physical `first_line` and `last_line` from the shared native Unicode scalar mapper; retain the local recognize offsets alongside it. Relate joins both returned endpoint ids to their actual reader rows and admits at most 255 source rows before deduplication. PostgreSQL server paths remain deferred.
