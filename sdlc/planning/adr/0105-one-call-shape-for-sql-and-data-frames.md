# ADR 0105: One call shape for SQL and data frames

- Status: **accepted** 2026-09-29 by Ian's approval relayed by the coordinator, after the review history in experiment 2038's `review.md` (rounds 1-7 fresh reviewers, Opus rounds 8-9, the marketing lead's round 10 with its diff-checked fix) and Ian's recorded rulings in that folder's `questions.md` and `feedback-round-1.md`. Proposed 2026-09-29, revised the same day after Ian's feedback round 1 (`feedback-round-1.md` in experiment 2038; `analysis.md`, `questions.md` and `probes/` in the same folder hold the analysis, the rulings and the measurements). Nothing here is built.
- Amends: ADR 0041 (the deadline gains one name and one unit, `deadline_ms`, everywhere), ADR 0007 (whose rename of `--plan` to `--dry-run` this ADR reverses), ADR 0096's find signature table (`none` and the deadline move into settings; its selection contract is untouched), the SQL READMEs' argument tables, and the command's asking-verb `--dry-run` flag (replaced by `--plan`). Leaves ADR 0082's return types, ADR 0047's `Userset` key rule, and ADRs 0048, 0051, 0053, 0055, 0087's batching, ceilings and cache-identity rules untouched.
- Ian's rulings of 2026-09-29 (`questions.md`) stand, sharpened by the feedback: shared native arguments in one order with one portable JSON settings form (1C); **a clean break with no compatibility section** — 0.1 is unreleased, every function takes the same arguments in the same order and nothing keeps an old positional slot (2A, feedback 1); SQLite and PostgreSQL get the packed form and examples use it, plain scalars stay per-row with the count stated, the per-record cache key waits for its own ADR (3B); module verbs were this ADR's base — Ian overturned ruling 4A on 2026-09-29 and the functional and lazy interfaces are in 0.1, designed in ADR 0107, which shares this ADR's T1 core settings schema; decide and choose probability, now carried on the same row and the same `Call` instead of a companion function (5A, feedback 3); a no-send preview and one process request cap on **every surface, Rust included**, and one preview word (6B, feedback 6–7); the default single cut stays and a band is one short argument (7A); keep the prefixes, unify usage names not shapes (8).

## Context

The three SQL extensions and the three data frame doors spell one question six ways. The survey, proposals and criteria are `sdlc/issues/2026-09-28-sql-interface-usability-before-0-1.md`; ticket 0259 landed its two bounded fixes. MotherDuck's `prompt_jev()` sets the comparison bar.

Probes (scripts and raw outputs in `probes/`, worktree at `060feb92` unless named):

- **DuckDB v1.5.5 named arguments split by kind** (`duckdb-named-args`, `duckdb-macro-named-args`, each case run alone): scalar functions accept `:=` but bind **positionally** — a wrong name is silently ignored and out-of-order names fail to bind — while SQL-level macros with named defaults bind **by name** and refuse unknown names with a Binder error. The pinned v1.5.5 source exposes `CreateMacroInfo` and `Catalog::CreateFunction`, so the extension **can** register such a macro (`probes/duckdb-macro-registration`); T4 adopts macro named arguments as DuckDB's daily form, with the installed package proving the built artifact.
- **PostgreSQL 16.15 named notation binds by name**: out-of-order `scratch_two(hi => 'a', lo => 'b')` answers `ba`, and an unknown parameter name fails resolution with a clean error (`pg-named-args`, pinned server package). A parameter named `true` or `false` is impossible — reserved words — so those two settings stay in the settings object on PostgreSQL.
- **A paired decide and probability read one request.** On the SQLite surface, probability after decide on the same question and text answered from cache: 1 request sent, 1 cache answer (measured). The many form returns both from one request by construction.
- **Engine shape for 100,000 distinct records**: `filter` at default settings sent **154 requests**, ran **0.61 s** against the loopback conformance backend, and peaked at **15 MB** resident; the empty-run baseline is 6.5 MB and one record 7.4 MB (measured).
- **SQLite today**: `thinkthen_warm` over 1,000 distinct rows at default `max` sent **4 requests**; ten scalar calls after that packed warm sent **10 more** (measured) — the whole-request cache digest cannot serve a packed entry to a singleton call, which is why the many form replaces warm instead of feeding it.
- **The keyed join works**: SQLite matches a TEXT object key against an INTEGER column by numeric affinity, and `json_group_object(id, title)` builds the keyed input (measured, `sqlite-keyed-join`).

## Decision

### 1. One order, one settings form, one parser

Every function on every surface takes the same arguments in the same order:

```
question, input, members, settings
```

- **question**: plain text, JSON question, or `'@file'`, everywhere.
- **input**: the evidence — one text, one column, or, for the many form, one keyed JSON object.
- **members**: the verb's list — `options`, `levels`, `labels` — natively: `VARCHAR[]` on DuckDB, `text[]` on PostgreSQL, a list or vector on the frames. Absent on decide; carried inside settings on SQLite and in the many form.
- **settings**: a JSON object, schema `thinkthen.settings/1`:

```json
{"threshold": 0.7,
 "true": "What a yes means", "false": "What a no means",
 "options|labels|levels": ["billing", "shipping"] | {"billing": "Payments, invoices, refunds"},
 "model": "jev-1.13.0",
 "context": "One shared reference text per request",
 "batch": "max",
 "deadline_ms": 5000,
 "none": true}
```

Rules, identical everywhere:

- **The schema and its parser live in the Rust core.** The command, the three SQL extensions, Python and R call it from T1 to T8; **T9 moves every remaining language door to it** — `deadline_ms` in each door's idiom, call settings through the one parser, and the process cap — so the "every surface" of ruling 6B is true on main when T9 lands, not before.
- Keys equal the question-file keys wherever both exist (`threshold`, `true`, `false`, `options`, `labels`, `levels`, `model`, `batch`) and the command's flag names where they overlap (`--threshold`, `--true`, `--false`, `--option`, `--label`, `--model`, `--batch`). Two differences, with reasons: the CLI's `--context FILE` reads a file while settings and keywords carry literal text, because SQL and the frames have no file-reading argument; and `deadline_ms` has no flag today.
- **One name and one unit for the deadline: `deadline_ms`, milliseconds, everywhere** — settings key, keyword argument, SQL parameter, and Rust `CallOptions` in its `Duration` idiom, with milliseconds named wherever a number crosses a boundary — amending ADR 0041's seconds/milliseconds split.
- **One preview word: `plan`.** The command's asking-verb `--dry-run` flag is replaced by `--plan`, which prints the first request's body (the disclosure `--dry-run` gives today) **and** a whole-input count line; it reads the whole input for the count and sends nothing, proved by counting loopback requests; `check --dry-run` is renamed `check --plan` under the same rule, and `cache prune --dry-run` stays. SQL has `thinkthen_plan`, the frames `tt.plan` / `tt_plan`, and Rust the `Engine` plan. A plan's `requests` counts **planned requests before cache answers, refusal splits and retries**; chunked hosts (DuckDB vectors, Polars morsels) can add one partial request per chunk.
- A key the verb cannot take is `usage`; an unknown key is `usage`; a settings key repeating what the question sets is `usage`; a **named parameter and the same key in `settings`** is `usage` (PostgreSQL); a members list beside a settings members key is `usage` (a `NULL` members argument with members named in settings is accepted).
- `none` (for find), `deadline_ms` and `context` are call keys; the rest are question keys under the question-file grammar. SQL `NULL` settings and `{}` mean absent. The settings object changes the wire exactly as the same values would through a question file; `threshold`, `deadline_ms` and `none` never ride the wire or enter a digest.

On the frames the same names are keyword arguments: `threshold=`, `true=`, `false=`, `options=`, `levels=`, `labels=`, `context=`, `batch=`, `deadline_ms=`, `none=` on find, and `token=` in Python (R keeps completion handles). A per-call `model` rides the question as today.

### 2. Exact spelling per surface

**SQLite** (settings text is the third argument; one `thinkthen_configure`; the many form is keyed):

| Call | Spelling |
| --- | --- |
| decide | `thinkthen_decide(question, input[, settings])` → 1, 0, NULL |
| choose / score / tag | `thinkthen_choose|score|tag(question, input[, settings])` → TEXT / REAL / JSON array text; members live in settings |
| many form | `thinkthen_decide_many|choose_many(question, keyed_json[, settings])` → rows `(key TEXT, value, probability REAL)`; `thinkthen_score_many|tag_many(question, keyed_json[, settings])` → rows `(key TEXT, value)` — ruling 5A withholds their probability; see item 4 |
| find | `thinkthen_find(question, units_json[, settings])`; `none` rides settings |
| annotate | `thinkthen_annotate(questions, input[, settings])` |
| recognize | `thinkthen_recognize(input, kinds[, settings])` → table rows, unchanged shape |
| relations | `thinkthen_relations(input, spec)` — `thinkthen_recognize_document` renamed to the name the other two databases and the site sample already use |
| relate | `thinkthen_relate(query, rules[, settings])` — the same read-only `SELECT` input as DuckDB and PostgreSQL, replacing the table-and-columns form |
| details, try_details | `thinkthen_details|try_details(question, input[, settings])` |
| plan | `thinkthen_plan(question, keyed_json[, settings])` → JSON text |
| usage | `thinkthen_usage()` unchanged |
| configure | `thinkthen_configure(json)` replaces the twelve setting functions; keys `model`, `batch`, `cache`, `throttle`, `timeout`, `max_retries`, `max_request_bytes`, `max_requests`, `max_requests_total`, `profile`, `record`, `replay`, with today's value rules and the same before-engine-build rule; unknown key `usage`; returns the object it set |

**DuckDB** (members stay a native third list; the vector is the daily form; `_many` exists so the shared packed line runs here too):

| Call | Spelling |
| --- | --- |
| decide | `thinkthen_decide(question, input[, settings])` → BOOLEAN |
| probability | `thinkthen_probability(question, input[, settings])` → DOUBLE — kept on DuckDB only, because its idiomatic form is the per-row vector and it has no need of the many row's pairing; every other surface carries probability on the many row or the `Call` |
| choose / score / tag | `thinkthen_choose|score|tag(question, input[, members[, settings]])`; a JSON-object `VARCHAR` in the members slot is accepted through a type-distinguished overload, so the portable settings-only call runs here too |
| named arguments | scalar functions bind `:=` positionally only (probe); macros bind by name and the extension can register them (probes), so T4 ships the daily form `threshold := '0.3:0.7'` and the installed package proves it |
| many form | `thinkthen_decide_many|choose_many(question, keyed_json[, settings])` → rows `(key VARCHAR, value, probability DOUBLE)`; `thinkthen_score_many|tag_many` → rows `(key VARCHAR, value)`; DuckDB's choose probability rides `choose_many` |
| find | `thinkthen_find(question, units[, settings])`; `none` rides settings |
| annotate, recognize, relations, relate, details, try_details, usage | unchanged shapes; judgment calls take `[, settings]`; warm is removed |
| plan | `thinkthen_plan(question, keyed_json[, settings])` → STRUCT |

**PostgreSQL** (native named parameters, probed; the many form replaces array overloads):

| Call | Spelling |
| --- | --- |
| decide | `thinkthen_decide(question text, input text[, settings json DEFAULT NULL, threshold text DEFAULT NULL, context text DEFAULT NULL, model text DEFAULT NULL, batch text DEFAULT NULL, deadline_ms bigint DEFAULT NULL])` → boolean — `settings` follows `input` and the named conveniences follow it, keeping the positional order shared with SQLite and DuckDB; decide has no members |
| choose / score / tag | the same defaulted parameters with `members text[]` between `input` and `settings`; an untyped literal cannot select `json` over `text[]`, so the portable settings-only choose uses `settings => '{…}'` — PostgreSQL's one dialect limit, named here |
| many form | `thinkthen_decide_many|choose_many(question, keyed_json jsonb[, settings json])` → rows `(key text, value, probability float8)`; `thinkthen_score_many|tag_many` → rows `(key text, value)`; the existing `text[]` array overloads are removed |
| find | `thinkthen_find(question, units text[][, settings json])`; `none` rides settings |
| annotate, recognize, relations, relate, details, try_details, usage | unchanged shapes; judgment calls take `[, settings]`; warm is removed |
| plan | `thinkthen_plan(question, keyed_json jsonb[, settings json])` → jsonb |

The daily PostgreSQL form is named notation: `thinkthen_decide(q, title, threshold => '0.3:0.7')` (probe: binds by name, out-of-order works, unknown names fail cleanly). `true` and `false` are reserved words and cannot be parameter names, so those two ride the settings object — **one call may mix named arguments with `settings => JSON`**, for example `thinkthen_decide(q, title, threshold => '0.7', settings => '{"true": "A yes means a refund request"}')` — and every other name stays a named argument. Ian chose this over renaming the pair, which would change a concept everywhere to fix one database's limit. The JSON settings object stays the portable form that runs byte-identical on all three databases. pgrx 0.17 declares defaults, and each argument's Rust identifier reaches the function entity (`probes/pgrx-argument-names`, crate source cited); parameter names come from the Rust signatures and T3 proves them on the installed host.

**pandas and Polars** (the module verbs are this ADR's base; ADR 0107 adds judges, streams, the Polars `.tt` namespace and the pandas accessor in 0.1): every verb returns `Call` with `.value`, `.probability`, `.facts`, `.details`. `tt.decide(q, text_or_column, *, threshold=, true=, false=, options=, levels=, labels=, context=, batch=, deadline_ms=, token=)`; the same keywords on every verb; `tt.plan(judge, column_or_list)` — the judge is ADR 0107's, so every keyword that changes bytes rides it; plan adds only `batch=` and `context=`. `.probability` is the yes probability for decide and the winner's probability (NULL when not sure) for choose — one call, no second request. `tt.question` renames `true_`/`false_` to `true`/`false`.

**R**: `tt_decide(q, text, threshold =, true =, false =, context =, batch =, deadline_ms =)` and the same on every verb; the call list gains `$probability` beside `$value`; `tt_plan` lands with ADR 0107's F6, when R judges exist; arguments and docs rename `evidence` to `input`.

**Rust core**: `EngineBuilder::max_requests_total`, `Engine` plan, and `CallOptions`' deadline in its `Duration` idiom with the `deadline_ms` name at every boundary a number crosses; the Rust Polars door gains `probability` beside its five methods, plan over a column, and call-level `threshold`, `true` and `false`.

### 3. Return types, not-sure, and probability on the same row

ADR 0082's plain types stand. NULL stays not-sure and never failure; choose keeps "NULL when no option wins above the cut or an exact tie", and its probability is NULL in exactly those cases. Input NULL answers NULL without a send; for column and expression forms the rule is null-in, null-out, so a later null in a stream or morsel never fails a query after earlier rows were paid for (T5, T7, F4, F7 tests). The default decide cut stays 0.5; a band is the settings `threshold` string, and the first example on the page shows not-sure.

**Probability belongs to decide and choose only** (ruling 5A). The many row returns `key, value, probability` for them; `score_many` and `tag_many` return `(key, value)`; `Call.probability` is `None` (Python) or `NULL` (R) on score and tag; `probability=True` on score or tag is `usage` on every surface (judges, streams, `.tt`, `_expr`). There is no `thinkthen_probability_many` and no second join. The scalar `thinkthen_probability` survives only on DuckDB, whose vector idiom is per-row (item 2 states why). `details` and `try_details` keep their contracts and the `thinkthen.result/1` shape.

### 4. The many form: keyed input, one join

The many form takes one JSON object mapping the caller's own keys to texts and returns each key with its answer and probability:

```sql
-- SQLite
JOIN thinkthen_decide_many(q, (SELECT json_group_object(id, title) FROM songs), '{"threshold": "0.3:0.7"}') d ON d.key = CAST(s.id AS TEXT)
-- PostgreSQL
JOIN thinkthen_decide_many(q, (SELECT jsonb_object_agg(id, title) FROM songs), '{"threshold": "0.3:0.7"}') d ON d.key = CAST(s.id AS TEXT)
```

Keys are JSON object keys, so `key` is text; `d.key = CAST(s.id AS TEXT)` runs on all three databases (probed on DuckDB as well), so the join spelling is shared and only the aggregate name differs — `json_group_object` on SQLite and DuckDB, `jsonb_object_agg` on PostgreSQL, named as the one dialect limit. Repeated keys are `usage` where the host keeps them, and `jsonb`'s roughly 256 MB limit is where the page's chunk recipe applies. **SQLite opens and filters a table function once per outer row for some shapes** — `LEFT JOIN` re-filters one cursor, and a correlated scalar subquery or `EXISTS` opens a new cursor per outer row (measured: 1,000 opens for 1,000 rows). So the many table object keeps **a small map of answer slots on the database connection** — eight slots keyed by an argument digest, evicting the least recently used, each slot bounded by its input and answers (an estimated 15–20 MB at 100,000 short rows) — so a statement that joins two packed calls, such as `decide_many` beside `choose_many`, alternates without evicting each other; the map is freed at disconnect. Nothing that changes an answer can change on a live connection: `thinkthen_configure` works only before the engine builds, and the settings are part of the argument bytes, so a slot's answers never go stale. The per-call check compares length and then raw bytes before any copy or UTF-8 validation, and a full parse of a 100,000-pair keyed object costs a measured ~16 ms on the pinned host (`probes/sqlite-json-parse-cost`), so one parse per distinct argument — not one per row — is the contract; 100,000 answered short rows hold an estimated 15–20 MB. The pages lead with the shapes the probe measured as single calls — `IN (SELECT key FROM many(...))` and a `MATERIALIZED` CTE — beside the join. PostgreSQL's function scan already keeps its rows in a tuplestore, and DuckDB's vector evaluates once. No positional arithmetic — gaps and non-1 starts are safe. Members ride settings. The same packed line — name, keyed input, settings — runs unchanged on SQLite and PostgreSQL, and DuckDB registers the same `_many` names so it runs on all three; DuckDB's own daily form stays the per-row vector. The frames' column calls are the frames' many form. Each packs under the engine planner at the current `batch` setting.

Plain scalars on SQLite and PostgreSQL stay one request per row, and each README's table states it. The goal-5 matrix for distinct short records (~40 bytes of evidence); every cell is **estimated** from measured rates except those marked measured (round trips 135–151 ms, throttle 4, the measured engine run, and the measured 6.5 MB / 7.4 MB baseline in `probes/engine-baseline-memory`):

| Surface | 10 rows | 1,000 rows | 100,000 rows |
| --- | --- | --- | --- |
| Engine core (loopback, `filter`) | 1 req · <0.05 s · ~6 MB | 2 req · ~0.05 s · ~6 MB | **154 req · 0.61 s · 15 MB, all measured** |
| Any packed surface, live RTT (`_many` everywhere, DuckDB vector, frames) | 1 req · ~0.14 s · ~6–10 MB | 2 req · ~0.3 s · ~6–10 MB | 154 req · ~5.5 s · 6–30 MB by surface |
| SQLite or PostgreSQL scalar | 10 req · ~0.4 s | 1,000 req · ~35 s | ~100,000 req · ~58 min; ~25M input tokens |

Memory detail: the keyed input is one JSON text (about the size of the column, 3.8 MB measured input at 100,000 short rows) held by the many call and streamed back row by row; DuckDB never buffers the whole column (2,048-row chunks under `memory_limit`); PostgreSQL materializes the aggregate and result, linear in text size (an estimated ~300 MB at 250-word cards; the page shows chunked iteration); pandas 2 makes one transient copy, pandas 3 Arrow and Polars cross zero-copy; R holds the column resident. T2 through T8 pin exact wall and peak per surface at 1,000 and 100,000 rows in installed-host proofs, and `plan` pins exact request counts for any real input, so the gate, not this table, is the standing authority for estimated cells. Counts scale with record size through the 96,000-byte ceiling and 4,096-member cap (ADRs 0048/0051/0053).

### 5. Process safety (goal 6, one answer per item)

- **Threads.** SQLite: worker thread per send; 50 ms interrupt poll; detached workers end within the 30 s attempt timeout; the extension stays pinned in memory. DuckDB: host threads; statement-owner interruption and SIGINT stop a call within 100 ms; the first throttle holds for the process. PostgreSQL: signal-blocking worker per call; PARALLEL RESTRICTED. Frames: worker thread per call; 50 ms in Python, R's 100 ms tick, completion receipts in both.
- **Forks.** PostgreSQL builds the engine per backend after the fork. Everywhere: counters restart in a child, but no process may fork while a ThinkThen call is in flight; engines build lazily, so forking before the first call is clean. The Polars warm-pool spawn rule and the R `mcparallel` refusal stay in the docs. Removing warm removes its 256-row flush buffers.
- **Cancelling and deadlines.** Every surface stops within one 50–100 ms tick; a request on the wire completes and is billed; PostgreSQL caches its answer. One deadline spelling: settings/parameter/keyword `deadline_ms`, milliseconds (ADR 0041 amended). PostgreSQL keeps `thinkthen.deadline_ms` and `statement_timeout`; SQLite keeps `thinkthen_budget_ms`; DuckDB keeps `SET thinkthen_query_budget_ms`.
- **Panics.** ADR 0098's per-binding catch scopes a Rust panic to a fixed `defect`; ADR 0081 covers the root and C++ bridge hooks. No payload crosses a boundary; `defect` is never NULL.
- **Signals.** Worker threads block host signals (PostgreSQL) or poll an interrupt flag; DuckDB's LOAD chains SIGINT to the host's action.
- **Cache and usage files.** Platform folder 0700, bound to the backend address, digest-named entries with a per-digest lock; partial replies never installed (ADR 0053 item 6); PostgreSQL's per-role `ALTER ROLE … SET thinkthen.cache` recipe stays the stated answer for role sharing. Usage files hold counts only.
- **The key.** `THINKTHEN_API_KEY` from the environment on every surface; no SQL spelling of the key is read. `SET thinkthen.api_key` **always succeeds and immediately emits a `WARNING`** — "thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment" — and the next call still refuses (Ian's ruling of 2026-09-29, reversing feedback round 1's fail-at-once ask after the Opus review showed a failing `SET` writes the statement, key included, to the server log under `log_min_error_statement = error`). This keeps ADR 0047's accepted `Userset` rule: a `SET` never fails and never logs its statement.

### 6. Cost caps and preview (ruling 6B, now including Rust)

- **Preview.** `plan` on every surface, one word: `thinkthen_plan(question, keyed_json[, settings])` on the three databases, `tt.plan` / `tt_plan` and the `Engine` plan on the frames and Rust, and the command's `--plan` flag replacing the asking verbs' `--dry-run`. All return `requests` (**planned requests before cache answers, refusal splits and retries**, from the pure batch planner; on `recognize` and `relate` an **upper bound**, marked as one, because later requests depend on earlier answers), `records`, `estimated_bytes` and `estimated_input_tokens` as a band at the measured 0.516 and 0.908 tokens-per-byte rates. No key, no network, no send — `--plan` keeps `--dry-run`'s zero-send behavior, proved by counting loopback requests.
- **Caps.** `max_requests_total` (atomic process reservation) on every surface: the three SQL extensions today, new on Python `Engine(...)`, R `tt_engine(...)` and Rust `EngineBuilder` (T7), **the command's `--max-requests-total` flag in T7**, and the remaining doors in T9. Per-call `max_requests` stays. A token cap is the recorded first follow-up.

### 7. Configuration

| Tier | Spelling |
| --- | --- |
| Per call | settings JSON / named parameters / keyword arguments (item 1) |
| Per session or connection | `SET thinkthen_*` (DuckDB), `thinkthen.*` GUCs (PostgreSQL), `thinkthen_configure(json)` once (SQLite, before the engine builds), `tt.Engine(...)`, `tt_engine(...)` |
| Per process | `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`, `THINKTHEN_BATCH`, and the process totals |

### 8. Errors in one form

Every SQL failure reads `thinkthen <kind>: <message> (retryable: yes|no)` — PostgreSQL's spelling, adopted by SQLite and DuckDB. The six kinds and the SQLSTATE and SQLite code maps are unchanged. Every removed call fails with a plain message naming its replacement, for example `thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many`.

### 9. Usage

The four names `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens` are the contract on every surface; each host keeps its native shape. No `rows_judged` in 0.1.

### 10. Removals — one rule, no compatibility section

Every function takes `question, input[, members][, settings]`, and the bare `(question, input)` call is simply that order with the optional arguments omitted. Nothing else survives from today:

| Removed | Replaced by |
| --- | --- |
| SQLite's positional deadline and context slots; DuckDB's trailing `BIGINT` deadline and literal context slots; PostgreSQL's trailing literal context overloads | the settings object / named parameters |
| `find`'s positional `none` and `deadline_ms` on all three; SQLite `annotate`'s deadline slot | settings keys `none`, `deadline_ms` |
| `thinkthen_warm` on all three, and the `thinkthen_batch(1)` warm-then-scalar recipe | the many form |
| `thinkthen_probability` on SQLite and PostgreSQL (kept on DuckDB only) | probability on the many row and on `Call` |
| PostgreSQL's `text[]` array overloads | the shared `_many` keyed form |
| SQLite's `thinkthen_recognize_document` | `thinkthen_relations`, the shared name |
| SQLite's twelve setting functions | `thinkthen_configure(json)` |
| SQLite `relate`'s table-and-columns form | the shared `(query, rules)` form, on the same read-only `SELECT` machinery |
| The command's asking-verb `--dry-run` flag (`cache prune --dry-run` stays) | the `--plan` flag |
| Python `true_`/`false_`; the `evidence` argument name in PostgreSQL and R docs | `true`/`false`; `input` |
| Python's `decide_many`, `choose_many`, `score_many` and `tag_many` module functions and Engine methods | judge application over lists — removed by ADR 0107's F1 |
| Python `deadline=` in seconds; R `deadline =` in seconds | `deadline_ms` in milliseconds. In R, `deadline_ms` sits after `...`; every name caught by `...` is `usage`, and `deadline =` fails with "deadline was renamed deadline_ms, in milliseconds", because partial matching would otherwise read 30 seconds as 30 ms |
| Python's `descriptions=` keyword on choose, score and tag | the members map in `options`/`labels`/`levels`; `recognize` keeps its own `descriptions=` |
| `check --dry-run` | `check --plan` |
| The `--dry-run` flags on `find`, `annotate`, `recognize` and `relate` | `--plan` on each; `recognize` and `relate` print the count as a marked upper bound |
| Reading `.value` on ADR 0107's `Stream` | `Stream.value` raises "a stream has no value; iterate it, or pass a list" |

Each removal gets a test proving the old call now fails with the plain message of item 8. Nothing is deprecated with a lifetime: 0.1 is the first published release, and marketing rewrites the deck, site and Beatles Bench samples after T8.

### 11. Six examples, six surfaces

`q` below is `The text is the title of a song by the Beatles. It appears on the album Abbey Road.`

**E1 — the Abbey Road deck query**, one join, both answers:

```sql
-- SQLite
SELECT s.title, d.value AS on_abbey_road, d.probability AS p
FROM songs s JOIN thinkthen_decide_many(q,
  (SELECT json_group_object(id, title) FROM songs),
  '{"threshold": "0.3:0.7"}') d ON d.key = CAST(s.id AS TEXT)
ORDER BY d.probability DESC;
-- PostgreSQL (scalar calls use named notation; see E3)
SELECT s.title, d.value AS on_abbey_road, d.probability AS p
FROM songs s JOIN thinkthen_decide_many(q,
  (SELECT jsonb_object_agg(id, title) FROM songs),
  '{"threshold": "0.3:0.7"}') d ON d.key = CAST(s.id AS TEXT)
ORDER BY d.probability DESC;
-- DuckDB (the vector is its daily form; the keyed _many line builds its input with to_json(map_from_entries(list({...} ORDER BY id))), probed)
SELECT title, thinkthen_decide(q, title, '{"threshold": "0.3:0.7"}') AS on_abbey_road,
       thinkthen_probability(q, title) AS p
FROM songs ORDER BY p DESC;
```

```python
call = tt.decide(q, songs["title"], threshold="0.3:0.7")        # pandas
songs["on_abbey_road"], songs["p"] = call.value, call.probability
songs = songs.sort_values("p", ascending=False)

call = tt.decide(q, songs["title"], threshold="0.3:0.7")        # Polars
songs = songs.with_columns(on_abbey_road=call.value, p=call.probability).sort("p", descending=True)
```

```r
ans <- tt_decide(q, songs$title, threshold = "0.3:0.7")
songs |> mutate(on_abbey_road = ans$value, p = ans$probability) |> arrange(desc(p))
```

**E2 — filter**, the same many line with `WHERE d.value` on SQLite and PostgreSQL, `WHERE thinkthen_decide('Does this ask for money back?', body)` on DuckDB; pandas `tickets[tt.decide("Does this ask for money back?", tickets["body"]).value]`; Polars `tickets.filter(tt.decide("Does this ask for money back?", tickets["body"]).value)` — the eager mask; a lazy frame uses ADR 0107's `with_columns` form; R `tickets |> filter(tt_decide("Does this ask for money back?", body)$value)`.

**E3 — choose with descriptions**, packed (members and descriptions in settings):

```sql
-- SQLite and PostgreSQL, the same call (PostgreSQL builds the input with jsonb_object_agg; both join with CAST)
SELECT s.id, d.value AS team, d.probability
FROM tickets s JOIN thinkthen_choose_many('Which team owns this?',
  (SELECT json_group_object(id, body) FROM tickets),
  '{"options": {"billing": "Payments, invoices, refunds", "shipping": "Parcels and delivery"}}') d
  ON d.key = CAST(s.id AS TEXT);
-- DuckDB daily form
SELECT id, thinkthen_choose('Which team owns this?', body,
  '{"options": {"billing": "Payments, invoices, refunds", "shipping": "Parcels and delivery"}}') AS team
FROM tickets;
```

Frames: `tt.choose("Which team owns this?", tickets["body"], options={"billing": "…", "shipping": "…"})` — `.value` and `.probability`; R: `tt_choose(..., options = c(billing = "…", shipping = "…"))` — `$value`, `$probability`. PostgreSQL's scalar daily form names its arguments: `thinkthen_choose('Which team owns this?', body, members => ARRAY['billing','shipping'], threshold => '0.7')`. The mixed named-plus-settings form of section 2 belongs to a decide call, whose keys `true` and `false` are: `thinkthen_decide('Does this ask for money back?', body, threshold => '0.7', settings => '{"true": "A yes means a refund request"}')`.

**E4 — score**: `thinkthen_score_many('How frustrated is the customer?', (SELECT json_group_object(id, message) FROM messages), '{"levels": ["calm", "frustrated", "angry"]}')` (SQLite; PostgreSQL the same with `jsonb_object_agg`); DuckDB `thinkthen_score('How frustrated is the customer?', message, ['calm', 'frustrated', 'angry'])`; frames `tt.score("How frustrated…", df["message"], levels=[...])` and `tt_score(..., levels = c(...))`.

**E5 — tag**: `thinkthen_tag_many('What does this mention?', (SELECT json_group_object(id, body) FROM tickets), '{"labels": ["refund", "shipping"]}')`; DuckDB `thinkthen_tag('What does this mention?', body, ['refund', 'shipping'])`; frames `tt.tag(..., labels=[...])`, `tt_tag(..., labels = c(...))`.

**E6 — find**, one set, `none` in settings: `thinkthen_find('Which passage answers?', units, '{"none": true}')` with each host's ordered collection constructor; `tt.find(q, units, none=True).value`; `tt_find(q, units, none = TRUE)$value`.

Request counts, which ruling 3 requires the page to state and T8's replayed tests pin: E1's six songs send **one** request on every surface — the many row carries both answers, and DuckDB's paired vectors send once with the second reading the cache, which assumes the default cache is on (measured shape); E2–E5 over the examples' small tables send one packed request per call on every surface, and a full table's count is the item 4 matrix's; a scalar call on SQLite or PostgreSQL sends once per row, which is why the packed forms lead; E6 sends one request for its whole set.

### 12. Test plan

- The settings grammar and its parser are built once in the Rust core (T1) with the full valid/invalid corpus; each host's proof runs a small table instead — one valid object, one invalid object, and that host's own conversion (PostgreSQL's named-argument merge, DuckDB's overload, SQLite's text) — because the full corpus at every layer tests the same contract twice. No test derives expectations from the implementation's own parser.
- One shared example corpus under `specification/fixtures/examples/`: E1–E6 with fixed inputs (the six-song Beatles Bench `songs.tsv` slice pinned at `ed8ffe49`, **with id gaps**, proving the keyed join), expected outputs, and pinned request counts; ADR 0107 adds E7–E9 (`pipe` and `take` over a generator, a Polars streaming scan to sink, R `mutate` with dbplyr). Each SQL host replays its examples through its replay settings from recordings captured once against the conformance backend; frames replay through `Engine(replay=...)`. No key, no network, in every gate.
- Removal tests: every removed call in item 10 fails with its plain replacement message, zero sends.
- Named-argument proofs: PostgreSQL named notation (including `threshold => '0.3:0.7'`, the mixed named-plus-`settings =>` form, and an unknown-name refusal); DuckDB's scalar positional limit asserted in docs and, if T4 registers macros, one macro named-argument proof.
- Pinned counts per surface: E1 one send everywhere; packed `_many` counts (the probe numbers as expectations); the per-row scalar count stated in each README and pinned by a replay.

### 13. Tickets, in landing order

| Ticket | Scope | Proof |
| --- | --- | --- |
| **T1 core settings schema** (M) | `thinkthen.settings/1` schema and parser in the Rust core, beside one **core engine-settings schema** shared with the C door's `thinkthen_engine_new_with` (ADR 0037), gaining `max_requests_total`; the shared corpus; `deadline_ms` unification; the `--plan` flag replacing the asking verbs' `--dry-run`, with the `spec/` pages, the `specification/` pages and fixtures, the `specification/settings.md` Dry-run row, every gated demo block that uses the asking-verb `--dry-run` (demos 03, 06, 14, 15, 16, 21 and 45), the probe self-tests that pass it (`probes/probability-total-0038`, `probes/find-0040`), and the remaining prose (`demos/27-test-with-no-network/README.md`, `demos/FINDINGS.md`, `AGENTS.md`) moved in the same commit | corpus self-test green through the core; every invalid object refuses before a send; `--plan` prints the plan and sends nothing, proved by counting loopback requests; every gate stays green |
| **T2 SQLite** (M) | settings slot; `_many` keyed forms with statement-scoped row reuse keyed by argument values (the `LEFT JOIN` rule of item 4) and the optional `key =` lookup; `thinkthen_relations` rename; relate's `(query, rules)` form; `thinkthen_configure`; removals | installed pinned-host replay: E1 one send; a 100,000-row `LEFT JOIN`, a correlated scalar subquery, an `EXISTS` query, and **two packed calls in one 100,000-row `LEFT JOIN`** under replay each call the engine once and send the planned count, with their `EXPLAIN QUERY PLAN` pinned and the wall time pinned against the per-row byte comparison; warm removal fails with the plain message; the score/tag no-probability refusal; one valid and one invalid settings object through SQLite's own conversion |
| **T3 PostgreSQL** (M) | defaulted named parameters (probed names) with `settings` directly after `input`/`members`; `_many` keyed form; the `api_key` warning; array-overload and warm removals | installed host: E1 replay with request capture; `threshold => '0.3:0.7'`, the named-plus-settings duplicate refusal, and unknown-name proofs; overload resolution for members/settings; the key absent from the server log under `log_min_error_statement = error` with the default `log_statement`, the warning emitted only for a non-empty value from an interactive source, and the recorded limits restated: `log_statement = 'all'` and `pg_stat_statements` do log the `SET` line |
| **T4 DuckDB** (M) | settings `VARCHAR` beside the members list (type-distinguished overload); `_many` registration; deadline/context slot removals; macro named arguments registered through `CreateMacroInfo` as the daily form, beside the scalar positional limit documented | installed v1.5.5 package: vector and `_many` counts, E1 replay; `threshold := '0.3:0.7'` through a registered macro; the `VARCHAR`-vs-`VARCHAR[]` overload resolution proof; the keyed `_many` line with its constructor, `to_json(map_from_entries(list({'key': CAST(id AS VARCHAR), 'value': title} ORDER BY id)))` (probed; `json_group_object` is a macro, not an aggregate, on DuckDB) |
| **T5 Python** (M) | keywords on every verb; `.probability` on `Call`; `Engine(max_requests_total=)`; the rename. `tt.plan` lands with ADR 0107's F1, when judges exist. It leaves the four `*_many` functions and Engine methods alone — ADR 0107's F1 deletes them | wheel tests under pandas 2.3.3 and 3.0.6 and Polars, replayed, counts pinned; the score/tag `probability=True` refusal; null-in, null-out on columns |
| **T6 R** (S) | `$probability`, uniform keywords, the `input` rename, `tt_plan`, `tt_engine(max_requests_total =)` | package tests replayed, counts pinned |
| **T7 Rust core and Polars door** (M) | `EngineBuilder::max_requests_total` and the command's `--max-requests-total` flag; `Engine` plan; the **core `Tally`** (an `Arc`-shared facts sum, which ADR 0107's Python `tt.Tally` wraps); the door's probability column (decide and choose), plan, call-level `threshold`/`true`/`false`. Rust keeps `CallOptions`' `Duration`; milliseconds are named only where a number crosses a boundary | core and door tests replayed; door returns value and probability from one request; the core tally equals the listener count; the score/tag probability refusal |
| **T8 examples and pages** (S) | the six examples on six surfaces in the corpus and every gate; DuckDB `examples.json`; all three SQL README tables; the settings page cells, including the PostgreSQL note that the key comes only from the server environment and that a `SET thinkthen.api_key` line reaches the log under `log_statement = 'all'` or `pg_stat_statements` | gates green; replayed outputs byte-pinned; marketing rewrites the site, deck, how-tos and Beatles Bench samples and any non-gated prose (demo 41's table row, `followup-performance.md`) after this lands — the gated demo blocks moved with T1, so the repo gates are green before marketing starts |

T2, T3, T4 and T7 run in parallel after T1; T5 and T6 follow; then ADR 0107's F1, F2 and F3, F4, F5 and F7 (F7 after T7), F6; then **T9 other doors**; T8 is last, with marketing starting after it, so the pages are rewritten once, after every surface — including E7–E9's — has landed. Each host ticket carries its own removals with the failing-with-plain-message tests of item 10.

| **T9 other doors** (L) | every language door besides Python, R, Rust and the three SQL extensions: `deadline_ms` in the door's idiom, call settings through the one core parser, `max_requests_total`, and the score/tag probability refusal | each door's replayed tests pin the rename, one valid and one invalid settings object through its own conversion, and the cap |

## What Ian can overturn

Every item above. Recorded limits he may want changed: PostgreSQL spells `true`/`false` only in the settings object (reserved words), mixed with named arguments in one call, as he chose over renaming the pair; DuckDB's scalar functions bind `:=` positionally only (probe), with macro named arguments possible if T4 proves registration; the keyed join casts the id to text on every database, because JSON object keys are text.

## Relations

- Issue `2026-09-28-sql-interface-usability-before-0-1.md` is the parent, criterion by criterion: 2, 3, 5, 6, 7 and 10 are satisfied by this design. Criterion 4 is met in the amended spelling: the deck query runs with a band and no `json_object`, by named notation on PostgreSQL (probed) and by the settings argument elsewhere; DuckDB's scalar `:=` binds positionally on pinned v1.5.5 (probe), and macro named arguments are a T4 gate, not a promise. Criterion 1's updated survey tables are T8's README rewrites. Criterion 9's one-row-four-column usage shape is rejected by Ian's ruling 8: names unify, native shapes stay. Criterion 11's three sample repairs belong to marketing under the site-samples issue, with T8 fixing `examples.json` and the database READMEs. Criterion 8 is met by T8's page without new code: SQLite and PostgreSQL already run recognize as a joined row source, and the page names DuckDB's scalar-plus-`unnest` form as its dialect limit, which the criterion's own `or` branch accepts. Criterion 12 is this ADR.
- The run-facts issue keeps its scope: `Call.facts` stays on every frame call and gains `.probability` beside `.value`; ADR 0107 adds `Stream.facts` and the tally to the same promise.
- ADR 0082 keeps its types; ADR 0096 keeps find's selection contract (only `none` and the deadline move into settings); the batching ADRs keep their rules. This ADR moves argument spelling, adds the keyed many form and `plan`, fixes names, and removes the calls in item 10.

## Amendment, 2026-10-01: `thinkthen_rank` replaces DuckDB's probability scalar

Ian ruled on 2026-10-01 that the three SQL extensions rank the same way, and ticket 0378 builds it. Probability now lives on the decide and choose `_many` rows and on the new `thinkthen_rank(question, keyed_json[, settings])` table on DuckDB, SQLite and PostgreSQL. That table returns `(key, rank, probability)` best first from one packed engine call. DuckDB's per-row `thinkthen_probability` scalar is retired, which overrides the DuckDB exception in items 2, 3 and 10. A keyed rank batches on every database, and the per-row form could not batch on SQLite and PostgreSQL. All three extensions refuse `thinkthen_probability` with `thinkthen_probability was removed; order records with thinkthen_rank`. Ian can overturn this amendment.
