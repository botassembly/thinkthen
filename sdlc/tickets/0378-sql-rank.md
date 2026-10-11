# 0378: Rank keyed records in all three SQL extensions

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-2. Branch `ticket/0378-sql-rank`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Milestone 0.1.

Ian approved this design for 0.1 on 2026-10-01 ("It's not too hard. We can get it done."). It replaces the first form of this ticket, which only retired DuckDB's `thinkthen_probability`. Ian can overturn any choice below.

## Outcome

DuckDB, SQLite and PostgreSQL each have one table function, `thinkthen_rank(question, keyed_json[, settings])`. It returns one row per keyed record, best first, with columns `key`, `rank` and `probability`. `rank` runs 1, 2, 3 and so on with no gaps. `probability` is the yes probability that orders the rows. All the records go to the engine in one call, which packs them the way `thinkthen_decide_many` does. With no batch setting, records share requests up to the request-size limit. With `{"batch":N}`, requests sent equal the record count divided by N, rounded up, while each request stays under that limit.

DuckDB's per-row `thinkthen_probability` is retired. On all three extensions, calling `thinkthen_probability` fails with the plain message `thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank`, without the retryable suffix. It sends nothing. DuckDB's dead warm bridge code is removed.

## Evidence

- Starts from: the state of main at 8c31cbe80.
  - SQLite and PostgreSQL retired `thinkthen_probability` under ADR 0105. DuckDB kept it as a live vector scalar, because ADR 0105 item 2 says its per-row vector form "has no need of the many row's pairing". The site's rank example, `site/examples/functions/rank/duckdb.sql`, uses it. No database has a rank function, so the three extensions differ.
  - The current refusals read `thinkthen_probability was removed; read the probability column of thinkthen_decide_many` on PostgreSQL (`src/removed.rs`, `check.sh` line 742) and `thinkthen usage: thinkthen_probability was removed; read probability from thinkthen_decide_many or thinkthen_choose_many` on SQLite (`src/scalars.rs`, `tests/test_probability.py`).
  - DuckDB's `thinkthen_probability` returns a `DOUBLE`, the yes probability before the threshold, or `NULL` for a `NULL` question or input. It is kind 1 of the native scalar in `cpp/src/portable.cpp`. Kind 1 runs through `decisions` in `bridge/src/ffi/scalar/ffi.rs` and through `bridge/src/ffi/portable_scalar/ffi.rs`. No other caller reaches `decisions`.
  - The public engine already ranks. `Engine::rank_with(&Question::rank(text)?, records, options)` sends the records as decide judgments and honors the call's batch, context and deadline. It returns `Ranked` rows best first, each with its input index and yes probability, and exact ties keep input order (`public/bulk.rs`, `core/order.rs`). The engine sends a question's own model when it has one (`Engine::asking`). Python, TypeScript, Ruby, R and C call `rank_with`, each with question text only.
  - `thinkthen_find` is the SQL precedent for a question built from text. Its question is literal text, including text that starts with `@` or looks like JSON. Its settings pass `Settings::check(For::Find)`, which allows only `model` among question fields, plus the call keys. DuckDB and PostgreSQL read `model` from the raw settings and build that call's engine with it. SQLite's find accepts `model` and drops it, because its engine is built once per process.
  - Each extension's `thinkthen_decide_many` takes one keyed JSON object of record keys to text: DuckDB `VARCHAR`, SQLite JSON text, PostgreSQL `jsonb`.
    - DuckDB: `cpp/src/portable.cpp` `Many` resolves `@file` questions through the caller's file system, skips a row with a `NULL` question or keyed input, so the table has 0 rows, and refuses repeated keys and non-text values (`portable_many/ffi.rs`).
    - SQLite: `src/many.rs` stamps an `@file` question for slot reuse, raises `the question is required` or `the keyed records is required` for `NULL`, and refuses repeated keys, non-text values and blank text. Eight connection slots reuse one answer set across reopens, and a hidden `lookup_key` column probes it.
    - PostgreSQL: `src/keyed.rs` raises `the question is empty` for a `NULL` question (`files.rs`), returns 0 rows for `NULL` keyed input (`forms.rs`) and refuses non-text values. `jsonb` keeps the last of repeated keys. It reads `jsonb` into a `serde_json` map without `preserve_order`, so keys reach the engine in bytewise key order. None of the three refuses blank text except SQLite. Blank text reaches the engine, which refuses it.
  - Shared conformance cases `15-rank-records`, `16-rank-stable-tie` and `31-usage-rank-blank-question` run on all three runners at batch 1, with one recorded exchange per record and a `{"decide": …}` question. DuckDB runs them through `thinkthen_probability`. SQLite uses `json_extract(thinkthen_details(...))` and PostgreSQL uses `thinkthen_decide_many`'s probability column.
  - DuckDB's `thinkthen_warm` already refuses in `cpp/src/warm.cpp` with `thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many`. The Rust bridge still exports `thinkthen_cpp_warm` (`bridge/src/warm/ffi.rs`, declared in `cpp/src/bridge.hpp`), and nothing calls it. `NOTES.md` still describes the pre-port layout. `sdlc/planning/databases/README.md` line 30 still names `thinkthen_probability` as the rank path.
  - Issue `2026-10-01-sql-names-for-rank-and-filter.md` (Milestone 0.2) asks for named SQL forms for `rank` and `filter`.
- Keeps: every behavior outside the rank and probability calls.
  - Every other SQL function, column and error.
  - The `_many` functions and their `(key, value, probability)` rows.
  - The `thinkthen_warm` refusals.
  - The engine's ranking rule and the CLI and library `rank` contracts.
  - The six error kinds and their `thinkthen <kind>: <message> (retryable: yes|no)` shape.
  - Removed calls keep plain migration messages without the retryable suffix.
- Changes: one new function on three extensions, one retired function, and cleanup.
  - **Function.** `thinkthen_rank(question, keyed_json[, settings])` returns rows `(key, rank, probability)`. The types are `VARCHAR, BIGINT, DOUBLE` on DuckDB, `TEXT, INTEGER, REAL` on SQLite and `text, bigint, double precision` on PostgreSQL. Its rules:
    - **Question.** The question is literal nonblank text, as in `thinkthen_find`. A question starting with `@` or `{` is sent as text and never read as a file or JSON. DuckDB skips `Resolve` for rank, and SQLite skips the file stamp. A blank question is `usage` before any send, which is conformance case 31.
    - **Keyed input.** It uses each extension's `_many` reader and keeps that reader's rules above for repeated keys, non-text values and blank text.
    - **Empty and `NULL`.** An empty object returns no rows. Each extension returns before calling the engine, so nothing is sent. `NULL` arguments act as in that extension's `thinkthen_decide_many`:
      - DuckDB: 0 rows.
      - SQLite: `the question is required` or `the keyed records is required`.
      - PostgreSQL: `the question is empty` for a `NULL` question, and 0 rows for `NULL` keyed input.
    - **Settings.** Settings accept `model`, `batch`, `context` and `deadline_ms`. `model` becomes the rank question's own model on all three extensions, so SQLite honors it too. `threshold`, `true`, `false`, `options`, `levels`, `labels` and `none` are refused before any send with `the settings key \`NAME\` does not belong to this verb`. A rank orders and never selects, as `rank` takes no `--threshold` in `specification/rank.md`. `Question::rank` has no yes or no meaning, as in every library, so `true` and `false` have nowhere to go. Rank never answers "not sure", so no row is `NULL`.
    - **Ties.** Ties keep the order in which the extension hands records to the engine. DuckDB and SQLite use the keyed object's member order. PostgreSQL uses bytewise key order, because of the sorted map read above. Each README states its rule.
    - **Order.** DuckDB's table macro ends in `ORDER BY rank`. SQLite and PostgreSQL emit rows in rank order. Each README tells callers to `ORDER BY rank` after a join.
  - **Design choices.** The coordinator's outline proposed a list-of-pairs input and a `likelihood` column. This ticket differs in two places.
    - The input is the keyed object, the same as `_many`. That keeps one input form per extension and reuses the existing readers, size limits and refusals. Each README shows how to build the object from rows: `json_group_object` in SQLite, `jsonb_object_agg` in PostgreSQL, and `json_group_object` in DuckDB.
    - The column is `probability`. It is the same number under the same name as the `_many` row, `thinkthen_find`, `Ranked::probability` and the CLI's `rank --details`.
  - **Core** (`crates/thinkthen`):
    - `For` gains `Rank`. `Settings::check(For::Rank)` uses find's check, which allows `model` among question fields. `none` stays find-only.
    - `Settings::model()` returns the validated `model` field.
    - `Question::with_model(self, &str)` sets a question's own model, with the existing `the model is already set` refusal. It lives beside the builders in `public/builders.rs`, because `public/question.rs` holds 489 nonblank lines.
    - `core/settings.rs` holds 476 nonblank lines and grows by about 8, which stays under the cap of 500.
    - `Engine::rank_with` checks every record's text before the first send. Before this, a blank record in a later batch failed only after the earlier batches had sent. `tests/public_batches/ranks.rs` pins the zero-send refusal and the question's own model in the body.
  - **DuckDB:**
    - The bridge's keyed parser takes a rank kind, 8. It builds `Question::rank(text)?.with_model(...)` after `check(For::Rank)`. The keyed run returns before the engine for an empty object, and otherwise calls `rank_with` and writes `{key, rank, probability}` rows best first.
    - `cpp/src/portable.cpp`:
      - `Many` skips `Resolve` for kind 8 and passes the question as text.
      - A `thinkthen_rank` table macro over `thinkthen_native_many` ends in `ORDER BY rank`.
      - Kind 1 leaves the native scalar loop, and the scalar decoder keeps only the details-text path.
    - `cpp/src/warm.cpp` and `warm.hpp` become `cpp/src/removed.cpp` and `removed.hpp`, and `cpp/src/thinkthen.cpp` and `CMakeLists.txt` follow. One native refusal function backs `thinkthen_probability(question, input, settings := NULL)` as a macro, so the named form still binds and reaches the refusal. `thinkthen_warm` keeps its overloads.
    - `decisions` and the kind 1 branches leave the bridge.
    - `bridge/src/warm/`, its `lib.rs` module line and its `bridge.hpp` declaration leave. `NOTES.md` says what the extension has today.
  - **SQLite:**
    - `src/many.rs` gains a `thinkthen_rank` virtual table with its own schema: `key TEXT, rank INTEGER, probability REAL`, and hidden `question`, `keyed_json`, `settings` and `lookup_key`. It uses the same connection slots, keyed reader and `lookup_key` probe. Its rank kind takes the question as text, with no file stamp, and its rows come from `rank_with`. Registration goes in `src/ffi.rs`.
    - The `thinkthen_probability` refusal takes the new sentence.
  - **PostgreSQL:**
    - `src/keyed.rs` gains a `thinkthen_rank` `TableIterator` on `forms::keyed`, `forms::controls` and `call.within`. Its question comes from `Question::rank`, and its rows come from `rank_with`. It is extension-owned and private like the other keyed functions.
    - `src/removed.rs` takes the new sentence on both overloads.
    - `src/forms.rs` adds `For::Rank` to its three matches. The first two refuse it as they refuse `For::Find`. The third maps it to `rank`. These arms land in slice A, because slice A's core change breaks the build without them.
  - **Conformance runners.** Rank cases 15 and 16 call `thinkthen_rank` on all three runners. Each passes the case's `question.decide` as text, keeps batch 1 and pins 3 requests and the exact `ranking`. Packing proof lives in each extension's own suite. DuckDB's runner and suites stop calling `thinkthen_probability` except to prove the refusal.
  - **Documentation:**
    - Each README's function table gets the new row, and its old-call paragraph names `thinkthen_rank`.
    - `specification/rank.md` gains a short "SQL" section.
    - `specification/settings.md` lists no SQL rank or probability function today, so it needs no row.
    - ADR 0105 gets a dated amendment under Ian's ruling. Probability now lives on the `_many` row and on `thinkthen_rank` on all three extensions. The per-row DuckDB scalar is retired, because a keyed rank batches on every database and the per-row form could not batch on SQLite and PostgreSQL.
    - `sdlc/planning/databases/README.md` line 30 names `thinkthen_rank`.
    - `sdlc/planning/milestones.md` lists this ticket under 0.1.
    - The rank-and-filter issue records that 0378 shipped the rank half in 0.1. Only `thinkthen_filter` stays a 0.2 idea for Ian.
  - **Ratchets.** Each extension's line ratchets move by the measured amount, and the build record explains the growth.
- Proof: every check runs on loopback, with fake keys and no network.
  - **Exact values.** On each extension, `thinkthen_rank` over the three keyed records of conformance case `15-rank-records` returns the exact `(key, rank, probability)` rows against its fixed probabilities, best first.
  - **Ties.**
    - DuckDB and SQLite: three equal probabilities keep keyed member order, including a key order that is not sorted.
    - PostgreSQL: the same input comes back in bytewise key order. A test pins this.
  - **Batching**, by loopback request count:
    - Seven records with no batch setting send exactly 1 request.
    - Seven records at `{"batch":3}` send exactly 3 requests.
    - Seven records at `{"batch":1}` send exactly 7 requests.
  - **SQLite reuse.** A `JOIN` of a seven-row table to `thinkthen_rank` on `key` sends exactly 1 request, and a `lookup_key` probe returns the one matching row.
  - **Literal question.** A question `'@nofile'` sends 1 request whose body holds `@nofile` as the question, on all three extensions.
  - **Empty and `NULL`.**
    - `{}` returns 0 rows and 0 requests.
    - Each `NULL` case gives the per-extension result above, with 0 requests.
  - **Refusals.** Each case below is pinned to its whole message, with 0 requests.
    - `threshold`, `true`, `none` and `options` in settings.
    - A blank question.
    - Repeated keys on DuckDB and SQLite.
    - A non-text value.
    - One blank value among seven records: DuckDB and PostgreSQL give the engine's whole message `thinkthen usage: evidence is text, not white space (retryable: no)`, and SQLite gives its reader's message, all with 0 requests.
    - `thinkthen_probability` with two and three arguments, and DuckDB's `settings :=` form. All give `thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank`.
    - `thinkthen_warm`, which keeps its message.
  - **Settings flow.**
    - `model` reaches the request body on each extension, including SQLite.
    - `context` is sent once per request as shared evidence.
    - `deadline_ms: 0` raises `deadline` with 0 requests.
  - **Secrecy.** The fake key appears in no error, details row or `Debug` line on the new paths. Each extension's existing secrecy sweep gets the new function.
  - **Gates.**
    - `databases/duckdb/check.sh`, including its installed-file mode on a locally packed extension.
    - `databases/sqlite/check.sh`.
    - `databases/postgresql/check.sh` and its installed-file mode where this host has the pinned runtime.
    - DuckDB macOS on the M5 when its shared lock is free.
    - `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/lint` in full (including the public API inventory), `sdlc/scripts/tickets` and the core settings tests.
- Defers: six gaps stay open.
  - Graded rank from a saved `score` question. `Ranked::probability` is a yes probability, so SQL rank stays yes/no, as in the libraries.
  - Yes and no meanings (`true`, `false`). `Question::rank` has none, as in the libraries.
  - A `--top` equivalent. SQL `LIMIT` serves it, and every record is judged either way.
  - `@file` and JSON questions. These would need a public way to turn a saved `decide` file into a rank question. No library has one.
  - SQLite's `thinkthen_find` still drops `model`. An issue records it.
  - The site page. The site owner gets a message with the three new forms, because the site belongs to marketing.

### Added public declarations

```text
For::Rank
fn Question::with_model(self, &str) -> Result<Question, Error>
fn Settings::model(&self) -> Option<&str>
```

## Size

These are estimates. The build record gives the measured numbers.

| Part | Files | Code lines | Test lines |
| --- | --- | --- | --- |
| Core | `core/settings.rs`, `public/builders.rs`, `tests/backend/settings_cases.rs` | about 25 added | about 15 |
| DuckDB | `bridge/src/ffi/portable_many/ffi.rs`, `scalar/ffi.rs`, `portable_scalar/ffi.rs`, `bridge/src/lib.rs`, `bridge/src/warm/` removed, `cpp/src/portable.cpp`, `cpp/src/removed.cpp` and `removed.hpp` (renamed), `cpp/src/thinkthen.cpp`, `cpp/src/bridge.hpp`, `CMakeLists.txt`, README, NOTES | about 70 added, about 150 removed | about 90 changed across `verbs_suite.py`, `portable_suite.py`, `signal_suite.py`, `conformance.py`, `cpp/verify_decide.py`, and `check.sh`'s installed test names |
| SQLite | `src/many.rs`, `src/ffi.rs`, `src/scalars.rs`, README | about 70 added | about 90: `tests/test_rank.py`, `test_probability.py`, `tests/conformance.py` |
| PostgreSQL | `src/keyed.rs`, `src/removed.rs`, `src/forms.rs`, README | about 50 added | about 70: `check.sh`, `tests/runner.py` |
| Records | `specification/rank.md`, ADR 0105, `sdlc/planning/databases/README.md`, `milestones.md`, the rank-and-filter issue, a SQLite find issue, this ticket | about 50 lines of prose | none |

Total: about 215 lines of code added and 150 removed, and about 265 lines of tests.

## Slicing

- **Slice A**: core, DuckDB rank, the DuckDB probability retirement and warm cleanup, PostgreSQL's three `forms.rs` arms, the DuckDB conformance runner, the ADR 0105 amendment and the `rank.md` SQL section for DuckDB.
- **Slice B**: SQLite and PostgreSQL rank, the shared refusal sentence on both, their runners, their READMEs, `rank.md` for all three, the planning README, and both issues.

Each slice lands whole after its own code review. The build finished both slices before the first code review, so they land together in one commit after one review.

## Risks

- **PostgreSQL tie order** differs from the other two. It is documented and pinned, not hidden. Making it match would mean taking `json` instead of `jsonb`, which breaks consistency with `_many`.
- **Build time.** DuckDB's C++ build and PostgreSQL's pgrx build are the slow steps. They wait while load is above 16.
- **macOS DuckDB proof** depends on the M5's shared lock. If the lock is busy, the next rehearsal proves the change on macOS.
- **Site.** The site's DuckDB rank example breaks at the next site build until the site owner changes it.
- **Engine request limit.** `rank_with` refuses more records than an engine's request limit. The SQL extensions set no such limit today, so this applies only to a future limit.

## What the build taught us

- Measured size against the base 8c31cbe80: core 95 lines added and 3 removed, DuckDB 304 added and 222 removed, SQLite 267 added and 49 removed, PostgreSQL 162 added and 19 removed, across 57 files with the records. The estimate was about 215 code lines added, 150 removed and 265 test lines. Tests and docs took most of the extra lines. The ratchets moved by the measured amounts: crates and conformance +85, DuckDB `src` and `bridge/src` -77, DuckDB C++ +10, DuckDB tools +117, SQLite `src` +76, SQLite tests +102, PostgreSQL `src` +68 and PostgreSQL tests +2. The search for code to slim found the dead `decisions` path and the warm bridge, both removed here.
- The engine checked rank records lazily, so a blank record in a later batch failed only after earlier batches had sent. `Engine::rank_with` now checks every record first, and a core test pins zero sends. The bug would have shown on every surface that ranks.
- SQLite's keyed reader refuses blank text with its own message, `keyed records are invalid: each keyed record is nonblank text at line 1 column N`, before the engine sees it. DuckDB and PostgreSQL pass blank text to the engine, which says `evidence is text, not white space`.
- PostgreSQL's tie order is bytewise because `forms::keyed` reads `jsonb` into a sorted map. `jsonb` alone would sort shorter keys first. The test uses keys of different lengths to tell the two apart.
- PostgreSQL's check sets `THINKTHEN_BATCH=1` for its retained fixtures, so its default-batch proof sets `thinkthen.batch = 'max'`, which is the default everywhere else.
- Editing a shell check while bash runs it breaks the run, because bash reads the script as it goes. Wait for the run to finish before editing.
- The macOS DuckDB proof did not run on the M5 for this ticket. The next release rehearsal covers it, as Risks says.
- SQLite's `thinkthen_find` drops `model`. Issue `2026-10-01-sqlite-find-drops-its-model-setting.md` records it for 0.2.
