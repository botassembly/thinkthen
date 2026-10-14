# The SQLite extension

`thinkthen-sqlite` is a loadable extension over the public Rust engine (ADR 0047 and ADR 0105). Build its release library, copy `libthinkthen0.so` to `thinkthen.so`, and load it with `.load ./thinkthen`. SQLite 3.50.0 or newer is required. Default loading registers volatile, direct-only functions. Objects in main and attached schemas cannot call them to spend requests or read files. Caller-created TEMP objects remain callable, even with `trusted_schema=OFF`.

## Discover registered functions

Run this query after loading the extension:

```sql
SELECT json_extract(value, '$.name') AS name,
       json_extract(value, '$.kind') AS kind,
       json_extract(value, '$.arity') AS arity,
       json_extract(value, '$.min_arity') AS min_arity,
       json_extract(value, '$.max_arity') AS max_arity,
       json_extract(value, '$.description') AS description
FROM json_each(thinkthen_functions())
ORDER BY name, arity;
```

`thinkthen_functions()` returns a JSON array collected from the connection's extension registrations, including the discovery call and removed spellings that still report migration errors. Each scalar overload has its own row; `arity = -1` means variable arguments. Table rows report the minimum required and maximum hidden argument counts as `min_arity` and `max_arity`. Descriptions explain each call's purpose. Loading and discovery construct no engine, read no input files and send no requests. Discovery remains direct-only in both loading modes.

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

## Name keyed call parameters

The keyed judgment and rank tables also accept optional arguments by hidden-column name:

```sql
SELECT key, value, probability
FROM thinkthen_decide_many('Is it red?', '{"7":"A red door."}')
WHERE model = 'judge-model' AND context = 'Describe the visible color.'
  AND batch = 2 AND threshold = '0.3:0.7';

SELECT key, value, probability
FROM thinkthen_choose_many('Which team?', '{"7":"Please track my parcel."}')
WHERE options = '{"shipping":"Parcel questions","billing":"Money questions"}';
```

Named `model`, `context`, `true_reading` and `false_reading` take SQL text. `threshold` takes a SQL number or native text band. `batch` takes a SQL integer or `'max'`; `deadline_ms` takes a SQL integer. `options`, `levels` and `labels` take ordered JSON label arrays or description maps as SQL text; their entries retain the native member grammar and authored order. `none` takes the SQL integers 0 and 1. Each function still applies its own allowed settings grammar. Named NULL values omit that setting. Supplying the same field by name and in the settings object raises usage before sending. Required NULL inputs still return no rows before inspecting these parameters.

Names and positions appear in `pragma_table_xinfo('thinkthen_decide_many')` and the discovery catalog reports the expanded argument range. Named parameters merge into the same native settings object before the connection cache lookup. The effective settings participate in cache identity, so changing context cannot reuse a previous context's rows. Existing positional settings retain their original bytes and behavior.

## Filter file evidence into typed rows

```sql
SELECT file, first_line, last_line, ordinal, value, probability,
       call_id, requests_sent, cache_answers
FROM thinkthen_filter('Is this a contract?', '["contract.txt"]') AS selected_contracts
WHERE unit = 'file' AND files_only = 1 AND batch = 16;
```

`thinkthen_filter(question, paths)` takes an ordered JSON array of file paths and returns selected rows with SQL text, integer and real columns. `files_only = 1` keeps the first passing occurrence for each file in input order. Omission returns every passing occurrence. Repeated paths retain their native occurrence ordinals and contribute to the owning call's record count. `value` is 1 for a selected row; `probability` retains its actual yes probability. The answer and call identities, request and cache counts, optional reported tokens and elapsed seconds come from the same native evaluation.

Named `unit`, `window`, `media` and `format` select the existing native reader. `format` admits `jsonl`, `csv` and `tsv`. `fields` takes an ordered JSON array of evidence pointers; `context_field` selects separate per-record context. The keyed-call settings above also apply through named parameters or `settings`. These calls use the existing Source reader and native admission, including physical row coordinates, file rules and image refusals. An empty path list raises the native Source usage error. A required NULL question or paths value returns no rows before inspecting other parameters. Source failures raise ordinary SQL errors. The same table accepts `request_records` for composed input records and typed receipt projections described below.

## Rank a table once

```sql
SELECT s.title, abbey_road.rank, abbey_road.probability
FROM songs AS s
JOIN thinkthen_rank(
  'The text is the title of a song by the Beatles. It appears on the album Abbey Road.',
  (SELECT json_group_object(id, title) FROM songs)) AS abbey_road ON abbey_road.key = CAST(s.id AS TEXT)
ORDER BY abbey_road.rank;
```

`thinkthen_rank(question, keyed_json[, settings])` returns `(key, rank, probability)` as `TEXT, INTEGER, REAL`, best first. `rank` runs 1, 2, 3 with no gaps, and `probability` is the yes probability that orders the rows. It takes the same keyed object as `thinkthen_decide_many`, under the same input rules, connection slots and `key =` probe, and packs the records into one engine call. `{"batch":N}` sets the records per request. The question is literal text, including text beginning with `@` or looking like JSON. Settings take `model`, `batch`, `context` and `deadline_ms`; `threshold`, `true`, `false`, `options`, `levels`, `labels` and `none` are usage errors before any send. Equal probabilities keep the keyed object's member order. An empty object gives no rows without sending, and a NULL question or object gives no rows, as `thinkthen_decide_many` does. `ORDER BY rank` again after a join.

## Calls and settings

| Call | Return |
| --- | --- |
| `thinkthen_decide(question, input[, settings])` | 1, 0, or SQL NULL for unsure |
| `thinkthen_choose(question, input[, settings])` | chosen label or SQL NULL |
| `thinkthen_score(question, input[, settings])` | position on the named levels |
| `thinkthen_tag(question, input[, settings])` | JSON array text of labels |
| `thinkthen_details(question, input[, settings])` as a table | native answer, confidence, answer identity, model and request count; full typed receipt projections |
| `thinkthen_try_details` as a scalar with the same slots | answered/failed recovery envelope |
| `thinkthen_find(question, units_json[, settings])` | selected index, value and probabilities as JSON text |
| `thinkthen_annotate(questions, input[, settings])` | named judgments as JSON text |
| `thinkthen_recognize(input, kinds[, settings])` | rows of names, offsets, kinds and strengths |
| `thinkthen_relations(input, spec)` | complete recognition JSON, including relations |
| `thinkthen_relate(query, rules[, settings])` | rows of relation, source id, target id, probability, and `either`, 1 for an edge of a both-ways rule (its ends then in query order) and 0 otherwise |
| `thinkthen_plan(question, keyed_json[, settings])` | JSON text with planned records, requests, bytes, input token band and the first exact request body as `first_body_utf8` |
| `thinkthen_usage()` | cumulative request, cache-answer and reported token totals; the process also adds them to the command's usage totals when it exits, so `thinkthen status` shows them (ADR 0113) |
| `thinkthen_usage_status()` | live JSON persistence observation; builds no engine and sends no request |
| `thinkthen_configure(json)` | the selected engine settings object, before engine build |

`thinkthen_usage_status()` returns `{"state":"disabled"}` before an engine is built. The native engine can report `pending`, `written` or latched `failed`. Failed adds only `"advice":"check the usage folder permissions and free space"`. Observation does not wait for the writer. Written covers this engine's current deltas, not future calls or other processes. A persistence failure leaves good answers and in-memory `thinkthen_usage()` counters intact.

The optional settings slot is JSON text in the shared `thinkthen.settings/1` grammar. Question fields such as `threshold`, `options`, `levels`, `labels` and `model` affect question bytes; `context`, `batch` and `deadline_ms` control the call. Choose, score and tag can take a plain question when their members are in settings. Duplicate fields in a complete question and settings refuse before sending. `deadline_ms` is an integer: `-1` removes a call deadline, `0` is already spent, and positive values are milliseconds. The parser checks its range before a worker starts. Find alone accepts `none: true`. Aggregate verbs take call controls, not judgment fields, in their settings object.

`thinkthen_plan` validates the keyed input and settings and makes no backend request. Its `requests` count is prepared requests before cache answers, refusal splits or retries. `first_body_utf8` is the exact first planned request body, or JSON null for empty input; it can contain question and evidence text, so handle the plan as carefully as a request. It contains no key.

`thinkthen_configure` replaces the individual setters. Its closed object supports `backend`, `model`, `batch`, `cache`, `throttle`, `timeout`, `max_retries`, `max_request_bytes`, `max_requests`, `max_requests_total`, `profile`, `record` and `replay`. The whole object is checked before it replaces the previous selection, and the engine must not already have built. `thinkthen_usage()` does not build it; a plan does build the engine so configure before planning. `"backend":"typesafe"` selects a built-in or configured name. Its address and key variable come from Rust configuration. SQL accepts no address or key. Omit `backend` to retain environment selection. Configure validation reads the environment but retains no credentials. The first engine build captures them. Every connection then uses that process engine. A failed build allows a later configuration and build to recover. `max_requests_total` reserves each live attempt against the shared process count, including later packed requests and retries. Cache hits and plans send nothing.

The old `thinkthen_warm`, `thinkthen_probability`, `thinkthen_recognize_document`, individual setters and positional deadline/context forms refuse with migration guidance. Read probability from the same decide or choose `_many` row as its value, and order records with `thinkthen_rank`. A required NULL scalar operand returns SQL NULL; a required NULL table or set operand produces no rows. These calls return before inspecting other operands, reading files or sending. A NULL optional setting means the setting is omitted. Malformed types and settings raise `thinkthen usage` before a send. Ordinary errors read `thinkthen <kind>: <message> (retryable: yes|no)` with kinds `usage`, `local`, `backend`, `cancelled`, `deadline` and `defect`; removed calls keep plain migration messages. `thinkthen_try_details` returns fixed safe advice for a recoverable row failure; cancellation and deadlines still raise errors.

`thinkthen_find` accepts an ordered JSON array of 2–255 nonblank text units, or 2–254 with `{"none":true}`. Duplicates retain separate zero-based positions. Empty input and SQL NULL return SQL NULL. `thinkthen_recognize` keeps the `text, start, end, length, kind, strength` row shape, with character offsets matching SQLite `substr`. `thinkthen_relations` accepts a full or bare recognize spec or an `@file`.

For `thinkthen_recognize`, pass one kind as `'person'`, or comma-separated names as `'person,organisation'`. To supply descriptions, pass a recognize JSON object such as `'{"kinds":{"person":"A human name."}}'`, a full versioned recognize spec, or an `@file` containing that spec. A bare JSON array string such as `'["person"]'` raises a usage error before any request. Use `'person'` to request the person kind.

```sql
SELECT text, kind FROM thinkthen_recognize('Maria Chen called.', 'person') AS recognized_names;
```

`thinkthen_relate` runs a caller-supplied read-only `SELECT` yielding `id, name` or `id, name, kind` on the same connection. `rules` is one inline rule, a JSON array of rules, a JSON relate spec or `@file`. Every form uses the [native relation grammar](../../specification/relate.md); empty and duplicate rule sets raise usage before a send. At most 255 distinct name/kind pairs enter a call. Equal pairs share one entity, and each answer edge expands to the ids that held its endpoints. Blank names/kinds and a 256th pair raise usage before a send.

## Price and elapsed time

SQL plans report an estimated input-token band before cache hits, retries or refusal splits. They do not predict output tokens, total dollars or future duration. SQL usage reports cumulative process totals, not the facts of one isolated call. Packed request metadata can appear on more than one result row; summing those rows counts the same request more than once.

`thinkthen_plan` previews one judgment question, not an annotate question set or a whole SQL pipeline. `thinkthen annotate --plan` previews CLI question-set packing; match the question set, backend, model and record framing, and treat that preview as a guide for SQL calls whose packing can differ. Reduce the planned cohort before calls when the configured allowance cannot admit the pipeline. Estimates neither authorize a larger budget nor predict the provider's bill.

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
       thinkthen_decide('Does this document contain a support contract?', record) AS has_support_contract
FROM thinkthen_read_files('specification/fixtures/files/documents', '{"unit":"file"}');
```

Files use the caller's native filesystem permissions. The reader remains DIRECTONLY even under `sqlite3_thinkthen_trusted_init`: stored main/attached views and triggers cannot read files. Caller-created TEMP objects retain SQLite's availability under either trusted_schema setting. A reader's source coordinates stay outside judging payloads and cache keys. `thinkthen_span_lines(record, first_line, start, end)` returns JSON with physical `first_line` and `last_line` from the shared native Unicode scalar mapper; retain the local recognize offsets alongside it. Relate joins both returned endpoint ids to their actual reader rows and admits at most 255 source rows before deduplication. PostgreSQL server paths remain deferred.

## Explicit images (0.2 development)

`thinkthen_image(bytes BLOB, mime TEXT)` returns a versioned tagged BLOB. `thinkthen_images(image, ...)` packs 1–8 values in authored order into another persistent tagged BLOB. Tags survive storage; every judgment revalidates them. MIME is exactly `image/png` or `image/jpeg`; the native reader/decoder validates original compressed pixels. Each question takes 1–8 images and at most 24 MiB of compressed bytes. Order and duplicates are preserved. Source file names stay outside model evidence and cache identity.

`thinkthen_decide_images(question, images, text, settings)`, `thinkthen_choose_images`, `thinkthen_score_images` and `thinkthen_details_images` use the existing native image engine. The first three return their ordinary scalar value; `thinkthen_details_images` returns typed table rows with the same receipt projections as text details. Questions and settings use the existing text/JSON/question-file grammar. Choose options and score levels belong in question JSON or settings. Ancillary text is optional. Typed image receipt projections retain the original evidence and source metadata.

NULL constructor operands return NULL. NULL question or collection returns NULL without sending; NULL ancillary text means absent text and NULL settings means defaults. A NULL member inside a collection, empty collection, invalid media/pixels/tag or exceeded limit is Usage before sending. Unsure decide/choose is NULL; failures remain errors. Arbitrary BLOB/bytea, text, paths, URLs and generic JSON never become images implicitly. Other functions retain their text input contract.

SQLite accepts image judgments with 2, 3 or 4 arguments; omitted trailing arguments mean NULL. `thinkthen_image_file(path)` explicitly selects one local regular image through the native reader. `thinkthen_image_file_name(image)` returns its retained source name, or NULL for a byte constructor. No text line positions are invented.


## Rank an ordered question set

`thinkthen_rank_set(questions, keyed_json[, settings])` takes the native version-one ordered set of named decide questions. `SELECT *` returns `key, rank, probability, question_name`. Equal text keeps each original key. Native turns visit each depth in authored member order and emit each original once. The selecting member supplies probability and question name. `ORDER BY rank` and `LIMIT` operate on the merged result; every member still judges every record.

Settings take batch, context and deadline. Engine configuration supplies model/backend selection. Described decide members retain authored true/false meanings. An empty keyed object and required NULL operands return no rows before sending. Question files use the existing native reader. Typed `facts_*` columns replace the previous JSON facts column and repeat the owning call facts on each answer row. Full member judgments and observations use the same typed projections below.

## Read typed answers and receipts

Each of the ten ordinary names also has a table form: `thinkthen_decide`, `thinkthen_choose`, `thinkthen_tag`, `thinkthen_score`, `thinkthen_filter`, `thinkthen_rank`, `thinkthen_find`, `thinkthen_annotate`, `thinkthen_recognize` and `thinkthen_relate`. Existing scalar answer calls and existing table answer columns remain available. `thinkthen_details` and `thinkthen_details_images` are typed tables. The retired `_complete` functions are absent.

For one ordinary receipt:

```sql
SELECT value, confidence, answer_id, answer_model, requests_sent,
       question_detail_usage_index, facts_call_id
FROM thinkthen_details('Does this ask for a refund?', 'Refund me.');
```

For an ordered collection, use named inputs:

```sql
SELECT decision_index, decision_value, decision_value_type,
       decision_answer_id, facts_requests_sent, error_kind
FROM thinkthen_decide
WHERE request_question = '{"decide":"Does this ask for a refund?"}'
  AND request_records = '[{"text":"Refund me."},{"text":"Track my parcel."}]'
  AND batch = 2 AND attempts = 1;
```

`request_question` takes plain atomic wording or the native saved grammar for the selected function. It accepts explicit `@reference` and named `@@NAME` selectors through the authorized SQLite reader. `request_records` takes an ordered JSON array of native record DATA. A record contains one of text, json, json_text or document, with optional context, options, images and source. JSON arrays/maps carry collection data because SQLite has no array type. They do not contain a request or response envelope. Original member order, duplicate occurrences and source coordinates are preserved. The name `request_records` avoids the existing `thinkthen_filter.records` answer column.

Use `request_paths` instead of `request_records` for an ordered JSON path array. `request_unit`, `request_window`, `request_media` and `request_format` select the native reader; format admits jsonl, csv or tsv. `request_files_only` keeps the first passing occurrence of each file. Existing filter input names retain their positions and meanings.

`record_fields` takes an ordered JSON array of evidence pointers. `record_context_field`, `record_options_field`, `record_seed_spans_field` and `record_examples_field` select per-record controls. `record_context_schema` takes a native JSON context declaration for composed records or file Feed input. Explicit Source selection uses the canonical Request option grammar, which rejects a caller context-schema override; a saved question can supply its context declaration. These fields replace question fallback reading. Explicit empty lists retain their native clearing behavior. Additional named request controls are top, mode, snippet_pieces, relation_threshold, seed_spans, examples, max_requests_total and stage_context. Integers and booleans use SQL integer values; booleans admit 0 and 1. Recognition seeds/examples and stage context use native collection DATA. The shared Request owner applies every function's option grammar and admission rules.

Exactly one collection is admitted before question resolution. Invalid known inputs refuse before a send; SQLite rejects unknown SQL parameter names. `incremental = 1` permits native prefix retention for decide, choose, tag, score, filter and annotate. Other functions admit their full input set before sending. `cancelled = 1` pre-cancels. `attempts = 1` retains native attempt facts. Required NULL legacy operands return no rows before inspecting partners; NULL optional settings use defaults.

Default table queries return ordinary answer columns plus explicitly selected facts/error columns. All receipt outputs are hidden, so `SELECT *` keeps each ordinary answer shape. Inspect the schema with `pragma_table_xinfo('thinkthen_decide')`. True input columns appear before hidden receipt outputs. Extra positional arguments that reach receipt output positions are ordinary SQLite data predicates; they do not change the admitted call or its identity.

Select `projection` to read a nested receipt collection from the same retained evaluation:

```sql
SELECT question_detail_model, question_detail_requests_sent,
       question_detail_question_index, question_detail_usage_index
FROM thinkthen_details('Does this ask for a refund?', 'Refund me.')
WHERE projection = 'question_detail';

SELECT usage_input_tokens, usage_output_tokens
FROM thinkthen_details('Does this ask for a refund?', 'Refund me.')
WHERE projection = 'usage';
```

Caller-facing projections include facts, error, question_detail, question, usage, event, record_order and the native result kinds decision, choice, tags, score, filtered, ranked, found, annotation, recognized and related. Their columns follow canonical semantic field names. The question's name field is `question_definition_name`, with matching presence column, to preserve rank_set's `question_name` answer column.

Each projection has its own `<projection>_receipt_index`. Object references use `<field>_index`; arrays and maps use `<field>_first_index` and `<field>_count` into the named child projection. `<field>_present` distinguishes omission from explicit null. Open answer DATA adds `<field>_type`; its primitive values are native SQL values and its arrays/objects are JSON collection data. Unsigned counts above SQLite's signed integer range use exact decimal text. Every canonical receipt field remains accessible, including probabilities, authored questions, input sources, attempts, pricing, diagnostic facts and failure stops.

Projection reads reuse the connection's retained evaluation and send nothing extra. Ordinary details/details_images default invocations preserve their fresh-call behavior, with the engine's own cache/record/replay counters. Following projection reads see that retained invocation. Different arguments select different retained receipts; changed question/source files invalidate their result. The connection retains eight most recently used results and releases them at close. Output predicates only filter projected host data and do not change call identity. Failure projections contain canonical safe errors, final facts and completed prefixes; facts are absent when admission failed before a call started. SQL interruption and ordinary errors keep their host behavior.

For tagged stored images:

```sql
SELECT value, answer_id, facts_requests_sent
FROM thinkthen_details_images(
  'Is red visible?', (SELECT thinkthen_images(image) FROM photos));
```

The native tagged BLOB preserves original image order, duplicates, compressed bytes and source metadata. Images never follow implicitly from text or paths.

All table forms execute through shared native Request admission and the same evaluation owns answers, observer details and facts. SQLite keeps registration, typed projection, file authority and interruption. Installed checks cover all ten function families, complete typed receipts, mixed reading, caller order, settings, source changes, failures and absent retired registrations.
