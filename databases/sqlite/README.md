# The SQLite extension

`thinkthen-sqlite` is a loadable extension over the public Rust engine (ADR 0047 and ADR 0105). Build its release library, copy `libthinkthen0.so` to `thinkthen.so`, and load it with `.load ./thinkthen`. SQLite 3.50.0 or newer is required. The extension registers volatile, direct-only functions: an untrusted schema cannot call them to spend requests or read files.

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
| `thinkthen_relate(query, rules[, settings])` | rows of relation, source id, target id and probability |
| `thinkthen_plan(question, keyed_json[, settings])` | JSON text with planned records, requests, bytes, input token band and the first exact request body as `first_body_utf8` |
| `thinkthen_usage()` | cumulative request, cache-answer and reported token totals |
| `thinkthen_configure(json)` | the selected engine settings object, before engine build |

The optional settings slot is JSON text in the shared `thinkthen.settings/1` grammar. Question fields such as `threshold`, `options`, `levels`, `labels` and `model` affect question bytes; `context`, `batch` and `deadline_ms` control the call. Choose, score and tag can take a plain question when their members are in settings. Duplicate fields in a complete question and settings refuse before sending. `deadline_ms` is an integer: `-1` removes a call deadline, `0` is already spent, and positive values are milliseconds. The parser checks its range before a worker starts. Find alone accepts `none: true`. Aggregate verbs take call controls, not judgment fields, in their settings object.

`thinkthen_plan` validates the keyed input and settings and makes no backend request. Its `requests` count is prepared requests before cache answers, refusal splits or retries. `first_body_utf8` is the exact first planned request body, or JSON null for empty input; it can contain question and evidence text, so handle the plan as carefully as a request. It contains no key.

`thinkthen_configure` replaces twelve individual setters. Its closed object supports `model`, `batch`, `cache`, `throttle`, `timeout`, `max_retries`, `max_request_bytes`, `max_requests`, `max_requests_total`, `profile`, `record` and `replay`. The whole object is checked before it replaces the previous selection, and the engine must not already have built. `thinkthen_usage()` does not build it; a plan does build the engine so configure before planning. The address and key remain environment-controlled through `THINKTHEN_BASE_URL` and `THINKTHEN_API_KEY`; SQL cannot supply either. `max_requests_total` reserves each live attempt against the shared process count, including later packed requests and retries. Cache hits and plans send nothing.

The old `thinkthen_warm`, `thinkthen_probability`, `thinkthen_recognize_document`, individual setters and positional deadline/context forms refuse with migration guidance. Read probability from the same decide or choose `_many` row as its value. A NULL question or input returns SQL NULL; malformed types and settings raise `thinkthen usage` before a send. Error prefixes are `usage`, `local`, `backend`, `cancelled`, `deadline` and `defect`. `thinkthen_try_details` returns fixed safe advice for a recoverable row failure; cancellation and deadlines still raise errors.

`thinkthen_find` accepts an ordered JSON array of 2–255 nonblank text units, or 2–254 with `{"none":true}`. Duplicates retain separate zero-based positions. Empty input and SQL NULL return SQL NULL. `thinkthen_recognize` keeps the `text, start, end, length, kind, strength` row shape, with character offsets matching SQLite `substr`. `thinkthen_relations` accepts a full or bare recognize spec or an `@file`.

`thinkthen_relate` runs a caller-supplied read-only `SELECT` yielding `id, name` or `id, name, kind` on the same connection. `rules` is one inline rule, a JSON array of rules, a JSON relate spec or `@file`. At most 255 distinct name/kind pairs enter a call. Equal pairs share one entity, and each answer edge expands to the ids that held its endpoints. Blank names/kinds and a 256th pair raise usage before a send.

## Runtime boundaries

The engine is shared by connections in this loaded copy. `thinkthen_budget_ms(n)` is separate from engine configuration and belongs to one connection: `0` spends it, `-1` clears it, and positive milliseconds run from that statement onward. Calls use the shorter remaining connection budget or call deadline. Every sending call runs on a detachable worker; the SQLite thread checks interruption every 50 ms and cancels promptly. A sent request remains counted. The library stays mapped until process exit so a detached worker can finish safely.

Questions may be plain decide text, inline JSON or a named `@file`; question sets, recognition and relation specs use their documented JSON/file forms. The file door follows symlinks, opens a regular file nonblocking and refuses files above 1 MiB. Parsed named files are re-read when modification time or size changes. Answer cache and recordings contain question and evidence text; keep their folders private. Recordings store bodies, never headers. Strict replay misses send nothing. The cache is not a pricing or authorization ledger.

`setup.sh` installs the pinned SQLite 3.50.0 amalgamation into the local toolchain cache once. `check.sh` builds a matching extension offline, verifies one exported entry point and the panic guard, then runs fixtures on the pinned host. `tests/helper.py` supplies isolated loopback children with clean environments; `tests/conformance.py` selects cases with an absolute `THINKTHEN_CONFORMANCE_IDS` file. A source test, a copied installed package and the release runner are separate qualifications.
