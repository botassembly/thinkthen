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

`thinkthen_decide_images(question, images, text, settings)`, `thinkthen_choose_images`, `thinkthen_score_images` and `thinkthen_details_images` use the existing native image engine. The first three return their ordinary scalar value; the last returns native detailed JSON. Questions and settings use the existing text/JSON/question-file grammar. Choose options and score levels belong in question JSON or settings. Ancillary text is optional.

NULL constructor operands return NULL. NULL question or collection returns NULL without sending; NULL ancillary text means absent text and NULL settings means defaults. A NULL member inside a collection, empty collection, invalid media/pixels/tag or exceeded limit is Usage before sending. Unsure decide/choose is NULL; failures remain errors. Arbitrary BLOB/bytea, text, paths, URLs and generic JSON never become images implicitly. Other functions retain their text input contract.

SQLite accepts image judgments with 2, 3 or 4 arguments; omitted trailing arguments mean NULL. `thinkthen_image_file(path)` explicitly selects one local regular image through the native reader. `thinkthen_image_file_name(image)` returns its retained source name, or NULL for a byte constructor. No text line positions are invented.


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

Questions/keyed input and facts are TEXT JSON. Questions may name an explicit
`@file` through the existing question reader. NULL questions or keyed input
raises required-argument Usage without sends; NULL settings uses defaults.

## Complete native calls (0.2 development)

`thinkthen_decide_complete`, `thinkthen_choose_complete`,
`thinkthen_tag_complete`, `thinkthen_score_complete`,
`thinkthen_filter_complete`, `thinkthen_rank_complete`,
`thinkthen_find_complete`, `thinkthen_annotate_complete`,
`thinkthen_recognize_complete` and `thinkthen_relate_complete` take
`(question TEXT, inputs TEXT, settings TEXT := NULL)` and return JSON text.
These are additive doors; existing scalar/table calls and usage totals keep
their signatures. PostgreSQL returns `json` for the same three arguments;
DuckDB returns JSON text. Each complete invocation evaluates once.

Use native saved-question JSON, including authored descriptions, declarations,
metadata and reading, or plain wording for an atomic question. Rank accepts
native saved decide/score questions or an ordered rank set. Annotation takes a
native question set; find, recognize and relate take their native saved grammar.
`@reference` selects native reference loading and `@@NAME` explicitly selects
named loading. DuckDB retains the native selection while its executing session
authorizes and reads the selected path. PostgreSQL authorizes before resolution
and checks the same opened descriptor against its allowed directory and the
captured named root before reading content. Native parsing receives the original
bytes; resolution grants no read permission.

The ordered input descriptor is explicit:

```sql
SELECT thinkthen_decide_complete(
  'Does this ask for a refund?',
  '{"records":[{"text":"Refund me."}],"attempts":true}');
```

A text/JSON record contains one of `text`, `json`, `json_text` or `document`, optionally
`context`, `options`, `images` and `source`. `json` retains the original authored
JSON. `json_text` admits a JSON line when the native iterator consumes it;
`document` uses annotation document semantics. Options are an ordered array of
names or `{ "name": ..., "description": ... }` objects. Images are an explicit
ordered array of `{ "media": "image/png", "bytes": [137,80,...] }` or JPEG with
its complete original compressed bytes. Image-only records contain only the explicit image collection. Paths, strings and
JSON never imply images. Native media, pixel, count, size, profile and function admission apply.
A source carries `file` and paired `first_line`/`last_line`; locations stay outside
model evidence and answer-cache identity.

`reading` contains native `fields`, `context`/`options` pointers and optional
`context_schema`. SQLite also admits per-record `seed_spans` and `examples`, with
matching projection pointers under `reading`. Null lists refuse; an explicit empty
list clears the question fallback. Other database adapters retain their own admitted grammar.
Per-record context and options retain false, empty and missing
distinctions. `incremental: true` selects native fallible admission; row-producing
calls retain their completed prefix on failure. Aggregate calls use native whole
set admission. `cancelled: true` pre-cancels a native token. NULL question or inputs
returns NULL; NULL settings uses defaults. Empty records use native semantics.

SQLite and DuckDB additionally accept `files` instead of `records`, with `paths`,
native reader `options` and optional `format: "jsonl"`. DuckDB opens only handles
authorized by its own filesystem settings. PostgreSQL accepts only client-read
records. Its native client reader transports actual Local/Usage read failures as
a terminal `{ "read_error": <native Error::complete envelope> }` record. This
bounded descriptor admits no caller facts, IDs or started stops. The consuming
native SQL iterator supplies its own terminal facts. PostgreSQL question files
retain the privileged descriptor loader and all confinement/link/size rules.

SQLite accepts exactly one input collection, `records` or `files`. Supplying both or neither refuses before question-file resolution and sends nothing. A `reading` descriptor composes the selected collection once; it does not add another input collection. Unknown input fields and nonboolean flags also refuse before resolution.

| SQLite input combination | Admission and execution |
| --- | --- |
| `records` with optional `reading` | Compose explicit records and preserve their originals and locations. |
| `files` with optional `reading` | Read selected files with native reader options; preserve file and line locations. |
| `files` with `format: "jsonl"` | Require text media and compose each JSON line through the selected reading. |
| Text or JSON records with explicit `images` | Admit actual image media, route and function before sending. |
| Image-only records or image file media | Admit native image inputs; images never follow implicitly from text or JSON. |
| Either collection with `incremental: true` | Row-producing calls retain the completed prefix; whole-set calls still admit the complete set. |

The `native` member contains the strict native `Call::complete()` envelope: `value` contains complete
result/2 rows (or the native find/relation object), and `facts` contains the owning
call ID, records, requests, cache answers, reported tokens, elapsed time, optional
caller-priced cost and requested attempts. Result rows retain full probabilities,
authors, answer IDs, reading, question sources and observation identities. The
separate ordered `observations` array snapshots that same native observer. `ordinals`
uses native occurrence getters; find adds its native `selection`. No SQL
wrapper subtracts cumulative usage or evaluates again to obtain details.

On failure, `native` contains strict `Error::complete()`, with all six kinds, safe diagnostic,
retryability and structured stopping information. Facts are absent before the
native call starts and final after its workers join. Incremental row failures add
actual canonical `completed` rows and their ordinals outside `native`. SQL interrupts keep host
cancellation behavior. Existing ordinary calls continue raising their errors.

Per-call settings use native context, batch and deadline controls and atomic
literal fields. Duplicate authorship/settings refuse before transport. Rank/find
accept model selection; set/recognize/relation model selection uses existing host
engine settings. Engine settings also admit `base_url`, `refresh_cache` and the
paired decimal strings `usd_per_million_input`/`usd_per_million_output`. Costs
use exact native combined rounding and are estimates, never provider bills.
Missing/overflowed usage omits cost; cache and replay have zero sends and cost.
The existing cache/record/replay and backend/key selection remain native.

SQLite translates these named complete calls directly into Rust `Request` values.
The native feed preserves composed originals and locations. Eager input validates
every row before sending; incremental input retains its actual completed prefix.
Caller-known image descriptors require route and function admission before reader
access. An omitted image declaration never waives validation of actual image rows.
Authored projections use the admitted native reading before declaration validation.
Annotation declarations validate each member's selected evidence. Complete filter
calls retain both passing and rejected observations and their original ordinals,
including completed rows before an incremental failure. Ordinary native Request
filtering retains its passing-row behavior; SQL explicitly selects all results.
Top-level input descriptors validate before SQL resolves a saved question. SQLite retains its authorized file reader. Its legacy scalar and table judgments also execute through Request. PostgreSQL imports the shared SQL preparation and Request helpers; DuckDB retains the compatibility dispatcher until its Request migration.

The existing surface checks execute all applicable shared cases through these
named calls against counted owned loopback, check known JSON fields using SQL
types, and repeat them against the installed extension package. They also retain
ordinary compatibility, secrecy, cancellation and wrong-model regressions.
