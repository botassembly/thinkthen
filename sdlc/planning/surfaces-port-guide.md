# Surfaces port guide

Written 2026-09-24 by Claude for queue items 6, 7, 9, and 10 of `one-line-plan-2026-09-24.md`. Main is the spine. Each surface on `surfaces-wave7` is ported onto the 0086 public Rust API in its own ticket, and the branch is never merged. This page changes no ticket and no code. Ian can overturn any classification or recommendation here.

Sources read: tag `surfaces-wave7-final` (`f6a7faea`), lanes `origin/w7/gate3`, `origin/w7/python4`, and `origin/w7/fast`, ticket branches 0076, 0077, 0078, 0084, 0085, and 0086, ADR 0017 and ADR 0037 on main, and `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. Nothing was built or run.

Port source: tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`). It merges the three lanes into `surfaces-wave7`, and every surface check ran there once. `sdlc/records/surfaces-freeze-2026-09-24.md` on that tag lists each result. DuckDB, Ruby, and PostgreSQL did not run, because they need the network or Docker. Each surface ticket starts from this tag and carries the three follow-ups that record lists.

Shared rules for every surface ticket, set by Claude on 2026-09-24 from the design reviews of 0105 to 0112. Ian can overturn any of them.

- A surface check never depends on Docker. The ladder runs on any developer machine that has the pinned toolchains.
- Toolchains and runtimes live under `~/.cache/thinkthen-toolchains/`, never in the product's answer cache. A one-time download of a public archive is setup, not a gate. It needs a sha256 pinned in the repo and a refusal on mismatch. Gates then run offline.
- Each test gets its own product cache folder and its own loopback backend. The arms beyond 0092 (fixed delay, a held reply that can hold again, one backend per test) belong to ticket 0117.
- Ctrl-C is prompt for single calls and batches alike. The engine waits for requests already sent, so the binding runs every call on a detachable worker and returns `Cancelled` at once. The worker finishes the sent requests.
- No test or plant can reach a paid backend. Surface tests run with the real key removed and a fake key set only beside a loopback address. A plant that changes how the engine is built must be shown to send nothing, counted on the loopback listener. Prove environment seeding through a setting that needs no request, such as the cache folder.
- A deny plant proves the rule it names while offline. A git-sourced dependency fails before deny runs when offline, so use a plant that reaches deny.
- Every surface exposes the engine settings in ADR 0017 section 5, width included, on its engine value, spelled the way that host spells its other settings. The surface builds that engine on `EngineBuilder::from_env()` (0084, amended 2026-09-24), so the address, key, and cache still come from the environment. Tests that need parallel requests set width through that public setting, never a hidden hook. Width is process-wide under 0077, so each such test runs in its own child process.

## 1. The error index, sorted

Every one of the index's 197 rows falls in one of three classes.

- **Binding** (152 rows): a host defect. The row, its test, and its probe move with the surface ticket and re-run against the real engine.
- **Standin-only** (31 rows): the stand-in, the contract crate, the branch gate, or the branch records own the defect. It retires with them.
- **Engine** (14 rows): the spine must satisfy it. Each names the spine ticket that should carry it as an acceptance test.

| Class | closed | open | partial | waive (incl. open/partial waive-candidates) | Total |
|---|---|---|---|---|---|
| Binding | 121 | 12 | 11 | 8 | 152 |
| Standin-only | 23 | 4 | 2 | 2 | 31 |
| Engine | 11 | 2 | 0 | 1 | 14 |

Status is the index's wave-7 probe column, or its wave-6 column where the row was not re-probed. The index header's totals (152 closed) differ by three rows from a per-row count (155 closed). R1-29, R4-11, and R5-42 read "CLOSED" with a stale-note remark, and this page counts them closed. R7-3 reads "fixed on the branch, not verified" and is counted open.

Closed rows are closed against the stand-in only. The one-line plan requires each surface ticket to re-run its rows against the real engine.

The three lanes touch these rows: gate3 (R3-29, R3-32, R5-5, R5-28, R7-4, plus a DuckDB relate queue message), python4 (R5-6, R3-4, R7-2, R7-8 in Python; R7-11 and R7-13 in R), and fast (no row; lint speed only).

### Engine rows (14)

| ID | Surface | Status | Spine ticket | Defect |
|---|---|---|---|---|
| R1-10 | contract | closed | 0086 (no panic crosses a public method); binding keeps its own FFI guard | Panics cross into host processes; no C-facing function catches a panic. |
| R1-11 | contract | closed | 0086: `deadline_after(huge)` must not panic; binding keeps checked host conversion | Large deadlines crash Node, abort Ruby, raise uncatchably in Python; one unchecked… |
| R1-23 | standin | closed | 0076 (retry waits observe deadline); cancel side landed in 0073 | Stand-in retry sleeps up to 60s ignore cancel and deadline. |
| R2-9 | standin | closed | 0077 (one process width cap) | Stand-in keyed wrongly: two engines with differing settings share state; a width-1 engine… |
| R2-21 | contract | closed | 0085 (retry classification; 0072 landed fail-fast) | Transport errors are misclassified as retryable; a refused connection wastes 3s. |
| R4-12 | contract | partial/waive | 0077 (width range) and 0086 (checked deadline add); `.expect` sites retire | Width has no upper bound: width 100,000 panics to exit code 6 at 322MB; the JSON door… |
| R4-24 | standin | closed | 0086 (builder base_url feeds the digest; environment capture) | Stand-in request_digest reads the base URL from the environment, so an engine configured… |
| R5-3 | standin | closed | 0078 (PID before any lock) | retain() takes a lock on every publish, including a child's first call after fork, which… |
| R5-4 | standin | closed | no ticket, gap G2 (resend after delivery) | A POST is resent after its bytes already left the process, on a connection reset or a… |
| R5-19 | postgresql | closed | 0085 (workers joined, no leftover send after a cancelled batch) and G2 | After a batch is cancelled by statement_timeout, the next single decide call on that… |
| R6-3 | standin | closed | 0078 (engine owns no signal; a host signal is not a cancel) | A harmless signal landing on the worker thread failed the whole call as "cancelled," even… |
| R6-4 | standin | closed | 0076 (spent deadline on every path including empty input; budget printed exactly) | Duration::MAX printed incorrectly through a float conversion, and annotate skipped its… |
| R6-15 | contract | open | no ticket, gap G2 | The same resend-on-transport-failure behavior found in the surfaces branch (see R5-4)… |
| R7-1 | standin | open | no ticket, gap G3 (main links the same ureq 3.4.2 resolver) | A churn probe crashes the stand-in through the C door: 2 of 104 runs SIGSEGV in ureq's… |


Engine rows by spine ticket: 0076 carries R1-23 and R6-4. 0077 carries R2-9 and the width half of R4-12. 0078 carries R6-3. 0096 carries R5-3. 0089 (landed) fixed R5-4 and R6-15 (G2), and 0085 re-proves them on every facade path, with R2-21 and R5-19. 0086 carries R1-10, R1-11, R4-24, and the deadline half of R4-12, and runs a Rust churn probe toward R7-1. 0094 closes R7-1.

### Standin-only rows (31)

| ID | Surface | Status | Why it retires |
|---|---|---|---|
| R1-7 | standin | closed | fixture in stand-in product code |
| R1-8 | standin | closed | stand-in settings; real builder takes base_url and width (0086) |
| R1-26 | gate | partial | branch is never merged |
| R1-27 | contract | closed | connector retires; 0084 bans it |
| R1-32 | gate | closed | branch file retires; union ticket owns coverage |
| R2-20 | contract | closed | 0086 uses core's one parser |
| R2-30 | records | closed | branch process |
| R3-14 | standin | closed | null backend is stand-in only |
| R3-27 | gate | closed | surfaces ratchet retires; main's ratchet counts ported code |
| R3-33 | records | closed | branch records |
| R3-34 | gate | closed | branch merge retires |
| R4-1 | standin | partial | stand-in state table retires; see R7-1 for the part that survives |
| R4-11 | records | closed | branch records |
| R4-20 | records | partial/waive | branch history |
| R5-1 | standin | closed | stand-in eviction path |
| R5-2 | standin | closed | stand-in probes |
| R5-5 | standin | closed | stand-in settings-keyed state table |
| R5-27 | gate | closed | surfaces ratchet |
| R5-28 | gate | open | surfaces ratchet; gate3 lane |
| R5-33 | records | open/waive | branch history |
| R5-36 | gate | closed | wire stub in experiments retires; loopback tests use main's harness |
| R5-39 | packaging | closed | branch notes file |
| R5-40 | gate | closed | branch checker; main has its own |
| R5-41 | gate | closed | branch gate output |
| R5-42 | gate | closed | branch document |
| R5-43 | records | closed | MERGE-NOTE retires |
| R6-1 | gate | closed | branch gate process |
| R6-2 | gate | closed | check_surfaces.sh retires; first surface ticket makes main's rung refuse a missing surface |
| R7-4 | gate | open | surfaces ratchet; gate3 lane |
| R7-5 | gate | open | branch artifact script; release ticket (queue 11) owns artifacts |
| R7-12 | records | open | stale branch records; TS DIVERGENCES rewritten at port |


### Binding rows not closed (31)

| ID | Surface | Status | Defect | Note |
|---|---|---|---|---|
| R1-31 | packaging | waive | Heavy duplication: ~13,100 lines of hand glue, error-kind names… | dedupe at port: one error-kind table per binding |
| R2-10 | contract | partial | Deadline sentinel spellings differ: any negative in C means none,… | one deadline rule per host; see G7 |
| R2-23 | contract | open/waive | Row-at-a-time requests (DuckDB and R verbs); host-side batching waits… | bulk forms exist in 0084; choose/score/tag bulk via one-question `annotate` (see G4) |
| R2-27 | contract | partial/waive | Shapes: Python and TypeScript response shapes differ; Node holds one… |  |
| R2-28 | rust | partial | A bare cargo test in libraries/rust still fails 17 tests (phase 3 was… | rust examples ticket |
| R2-29 | records | open | Rulings lack durable records (NOTES-only or nowhere): DuckDB relate… | surface ticket writes its ADR (DuckDB relate option A, R residual window, PG deadline ruling, C header) |
| R2-31 | packaging | partial | Duplication grew: the panic guard written seven ways, panic-to-text… | dedupe at port |
| R3-4 | python | partial | Python crash (exit 139): malformed Arrow string-view column,… | python4 lane |
| R3-25 | typescript | open/waive | Node ties up one libuv worker per call. |  |
| R3-29 | gate | partial | Hermeticity overstated: duckdb unpinned + git+https in the DuckDB… | gate3 lane |
| R3-32 | packaging | partial | Mac: experimental scripts hard-code ss, pg_config, timeout, GNU sed… | gate3 lane |
| R4-10 | packaging | partial | Mac-labelled scripts keep ss, /usr/bin/pg_config, timeout, GNU sed… |  |
| R4-15 | python | open | Moved output children have a no-op release; output offsets can wrap… |  |
| R4-16 | duckdb | partial | DuckDB leaks logical types, uses the deprecated result API 9 times,… |  |
| R4-17 | sqlite | partial | SQLite recognize and relate pass empty options; its question cache… | SQLite shim answer map retires at the swap; question cache bound stays |
| R4-18 | gate | partial | Tests behind PASS rows cannot fail: settings_state.rs is… | Ruby GC test and deadline_fast move; settings_state.rs retires |
| R5-6 | python | partial/waive | A string-view or UTF-8 offset that sits under the 4 MiB cap but names… | python4 lane |
| R5-17 | postgresql | waive | The per-backend saved-answer table has no bound; 270,000 warmed… | per-backend map; retires if warm moves onto the engine cache |
| R5-22 | duckdb | waive | DuckDB's row cap only bounds streaming row count; blocking operators… |  |
| R5-23 | duckdb | open/waive | con.interrupt() does not cancel a running DuckDB call at all; only… | needs G1 plus a DuckDB statement hook |
| R5-24 | duckdb | open | The vendored conformance deck fixture has no mechanical check that it… |  |
| R5-37 | packaging | partial | The Mac build recipe is not runnable as written: Ruby's build always… |  |
| R7-2 | python | open | A malformed Arrow string-view column crashes Python at the tip (exit… | python4 lane |
| R7-3 | ruby | open (fixed on branch, unverified) | Ruby's check.sh runs cargo test inside a networked container without… |  |
| R7-6 | postgresql | open | PostgreSQL's saved-answer budget counts only key bytes; real backend… |  |
| R7-7 | duckdb | open | DuckDB's signal handler sets HOST_ACTION after installing sigaction,… |  |
| R7-8 | python | open | NOTES.md:847-848 overstates the sizes-buffer fix: it claims a short… | python4 lane |
| R7-9 | ruby | open | tests/test_tick_gc.rb:29 cannot fail: the tick stays rooted in… |  |
| R7-10 | ruby | open | Ruby's Dockerfile:11 hard-codes rustc 1.93.1 apart from… |  |
| R7-11 | r | open | R's tt_annotate and related calls refuse an integer deadline… | python4 lane |
| R7-13 | r | open | An uncaught interrupt during an R call skips the options(error=) hook… | python4 lane |

### Binding rows closed on the branch (121)

Each moves with its surface ticket and re-runs against the real engine. Rows marked * also need gap G1 (the host interrupt check) or the worker-thread pattern.

| Surface | Rows |
|---|---|
| c | R1-9, R2-7, R2-16, R2-26, R3-24 |
| contract | R2-24*, R2-25 |
| duckdb | R1-1, R1-15, R1-16, R1-21*, R2-2, R2-3, R2-6, R2-13, R2-14, R2-18, R2-22, R3-1, R3-6, R3-7, R3-11, R3-12, R3-13, R3-23, R4-3, R4-4, R4-5, R4-22, R5-21, R5-25, R5-26, R6-5, R6-6 |
| gate | R1-2, R1-28, R1-29, R3-28, R3-30, R4-19, R5-29, R5-30, R5-31, R5-32, R5-34, R5-35, R6-7, R6-12 |
| packaging | R1-30, R1-33, R1-34, R3-31, R5-38 |
| postgresql | R1-18, R1-19, R1-22*, R2-4, R2-19, R3-2, R3-8, R3-9, R3-10, R3-21, R4-6, R5-15, R5-16, R6-8, R6-14 |
| python | R1-3, R1-4, R1-6, R1-12, R1-24*, R2-11, R2-12, R2-17, R3-18, R3-19, R4-14, R4-23*, R5-7, R5-8, R6-11, R6-13 |
| r | R1-5, R1-13*, R2-5, R3-3, R3-15, R3-17, R4-7, R5-10, R5-11, R5-12, R6-10 |
| ruby | R1-20*, R2-8, R2-15, R3-5, R3-15b, R3-16, R4-2, R4-8, R4-9, R5-9*, R5-13, R5-14 |
| rust | R2-32 |
| sqlite | R1-14, R1-17, R2-1, R3-2b, R3-22, R4-21, R5-18, R5-20, R6-9 |
| typescript | R1-25, R3-20, R3-26, R4-13 |

## 2. The gaps

Each gap is an engine need that no spine ticket covers today. Each line names the smallest ticket change that would cover it.

| Gap | Need | Evidence on the branch | Smallest change |
|---|---|---|---|
| G1 | A host interrupt check that runs on the calling thread while the engine waits. ADR 0017 §2 and §4 give the poll callback to the binding. 0085 keeps host polling internal, and 0086 says bindings add it over the private facade. A binding crate cannot reach private code. | Python bulk, SQLite warm, and the Rust door pass `poll: Option<&mut dyn FnMut()>`. MERGE-NOTE §6 rules the tick shape: within one tick, including the answer arms. | Plan item 7: amend 0084 with one `CallOptions` member such as `interrupt(self, &'a (dyn Fn() -> bool + Sync))`. `CallOptions` stays `Copy`. Add a 0085 acceptance test: a counted listener proves the check runs on the calling thread within one tick and that nothing starts after it returns true. |
| G2 | Stop resending a delivered request after a transport failure. | R5-4 fixed on the branch at `dd8a383`; R6-15 open on main. | Plan item 2: open the ticket. Add to 0085 acceptance: a close after the body left causes zero resends and returns the backend kind. |
| G3 | Survive engine churn inside ureq 3.4.2's resolver. Main links the same version. | R7-1: 2 of 104 churn runs crashed in `DefaultResolver::resolve_async` at `pthread_detach`. | Add a churn acceptance test to 0086: build and drop engines from many threads against loopback, many runs, zero crashes. File an issue on main with the cores. |
| G4 | A bulk probability and bulk choose, score, and tag. 0084's `decide_many` returns `Row<T, Answer>` with no probability. | DuckDB `thinkthen_probability` reads `Judgment.probability` from `decide_many`. R2-23 keeps R's choose, score, and tag at one request per row. | Amend 0084: give `Row` a `probability()` or return `Row<T, Details>`. Confirm in 0084 that a one-question `annotate` serves bulk choose, score, and tag. |
| G5 | Choices whose labels arrive at run time. `ChooseQuestion<C>` and `TagQuestion<C>` need `C::labels() -> &'static [&'static str]`. | Every dynamic host builds choose and tag from runtime strings. | Record in 0084 that `details(&Question)` and `annotate` accept a loaded, unbound choose or tag question and return `Judgment::Choice` or `Judgment::Tags`. 0086 adds a compile-pass fixture and a conformance run for that path. |
| G6 | Reading specs and inspecting questions. 0084 has no `Recognize::from_json`, `Relate::from_json`, question kind getter, or question-set names and kinds. | Ruby, SQLite, PostgreSQL, DuckDB, and C read specs as JSON text. R1-5 shows a column must be typed before its first row. | Amend 0084 with `Recognize::from_json`, `Relate::from_json`, `Question::kind`, and `QuestionSet::members`, each delegating to the 0080 and 0081 parsers. Without them each binding writes a second parser. |
| G7 | One rule for host deadline numbers: -1 none, 0 spent, above 4,294,967,295 seconds refused. 0084 has only `deadline_at` and an infallible `deadline_after`. | Five bindings call `deadline_from_seconds` or `deadline_from_millis`. R2-10 and R7-11 show the spellings drift. | Add to 0086 acceptance: `deadline_after(Duration::MAX)` neither panics nor wraps. Put the host number rule in one specification page with one conformance case per binding. A checked `CallOptions::deadline_seconds(f64) -> Result` in 0084 is the stronger fix. |
| G8 | Result JSON for the JSON doors. 0084 drops `to_json`, `edges_json`, and `rows_json`. | The C door, TypeScript, R, and PostgreSQL cross results as JSON text. | Decide in the 0086 design review: a public serializer, or one specification page each JSON door follows. The C ticket owns the first serializer if the review declines a public one. |
| G9 | Two engine images in one host process. 0077 and 0078 leave this to "surface integration", and no ticket owns it. | A Python wheel and a DuckDB extension each link their own engine. Each gets its own width cap. | The first surface ticket (Python) states the rule: prevent it, or qualify the one-process width claim on every surface page. |
| G10 | An offline test backend for bindings. The real engine has no null backend and no public fault hook. | Every `check.sh` runs on `ENGINE_NULL=1` or `THINKTHEN_NULL=1`. Case 74 and several suites build `synthetic-partial`. | Add to 0086 acceptance: an external crate runs the shared cases offline from a recording folder, and a loopback reply produces each `FailureCause`. The conformance union (plan item 6) ships that runner once for every surface. |
| G11 | A binding record type that can refuse. `Evidence::evidence` returns `Result<&str, Error>`, and no binding can construct an `Error`. | Bindings refuse NULL, bad UTF-8, and NUL bytes per row. | Record in 0084 that bindings validate before the call and implement `Evidence` as infallible. Otherwise add one public usage constructor. |


## 3. Port map

### 3.1 Stand-in contract to the 0084 public API

The left column is `contract/src/lib.rs` at the tag. The right column is the 0084 inventory. "Binding" means the binding owns the code after the port.

| Stand-in | 0084 public API | Port note |
|---|---|---|
| `Connector`, `StandinConnector.connect(&EngineConfig::from_env())`, `Arc<dyn Engine>` | `Engine::from_env()`, `Engine::builder()…build()`, `default_engine()` | 0084 bans a connector trait. Each surface replaces its one connector line and its engine static. `Engine` is `Clone + Send + Sync`. |
| `EngineConfig` / `Settings` (model, address, width) | `EngineBuilder::base_url`, `api_key`, `model`, `width(u8)`, `max_requests`, `cache_at`, `no_cache`, `cache_bytes` | Omitted width stays omitted (0077). PostgreSQL's refused `thinkthen.api_key` becomes `api_key`. |
| Trait methods `decide_opts`, `choose_opts`, and the rest | `Engine::decide_with` and the other `_with` forms, plus free functions | Suffix `_opts` becomes `_with`. The stand-in's `decide_with(q, e, Option<&Cancel>)` becomes `decide_with(q, e, CallOptions)`. |
| `Options::new().cancel(&t)`, `maybe_cancel`, `deadline(Instant)`, `deadline_in(Duration)` | `CallOptions::new().cancel(&t)`, `deadline_at`, `deadline_after` | `maybe_cancel` becomes a conditional builder step. `passed`, `seconds`, `remaining`, and `cancel_token` getters are gone. A binding that reads the budget keeps its own copy. |
| `with_deadline_seconds`, `with_deadline_millis`, `deadline_from_*`, `NO_DEADLINE`, `MAX_DEADLINE_SECONDS` | none | Binding, under one rule (G7). |
| `Cancel` (new, cancel, is_cancelled) | `CancelToken` (same three) | Still one-shot. DuckDB keeps its per-call re-arm logic. |
| `poll: Option<&mut dyn FnMut()>` on bulk calls | none | G1. Until then a binding runs the engine on a worker thread and fires the token from the host thread, as Ruby, R, and PostgreSQL batches already do. |
| `Error { kind, retryable, message }`, `Error::usage(..)`, `local`, `defect`, `deadline`, `cancelled`, `guard` | `enum Error { Usage(ErrorDetail), … }`, `kind()`, `retryable()`, `detail().message()` | No public constructor. A binding raises its own six-kind host error for its own refusals (G11). `guard` goes: the engine refuses a spent deadline itself. |
| `catch_panic`, `panic_text` | none | 0086 stops panics at public methods. Each binding keeps one guard at its own FFI edge. R2-31 asks for one guard per binding. |
| `Answer` (`value() -> Option<bool>`) | `Answer { Yes, No, Unsure }` | The binding matches the enum. |
| `Judgment { probability, answer }` from `decide_many` | `Row<T, Answer>` (`input`, `value`, `into_parts`) | Probability lost (G4). |
| `Scored { value, nearest }` | `f64`; `Details::nearest()` | `score` and `Annotated::Score` return the position only. The nearest level needs `details` (one judgment, a cache hit if repeated). |
| `Details` public fields: `probability`, `answer`, `nearest`, `model`, `digest`, `sends: u32`, `requests`, `failed_questions: u32` | accessors: `value() -> &Judgment`, `probabilities() -> &Probabilities`, `nearest`, `model`, `question_sha256`, `requests`, `requests_sent() -> u64`, `cached`, `usage`, `failed_questions() -> usize` | `digest` becomes `question_sha256`. `sends` becomes `requests_sent`. A yes probability comes from `Probabilities::YesNo { yes }`. `cached` and `usage` are new. |
| `Usage { requests, cache_answers, tokens }`, "since the last reset" | `Counters { requests_sent, cache_answers, input_tokens, output_tokens }` | No reset. Tokens split in two. Case 17 changes. |
| `Ranked { index, probability }` | `Ranked<T> { input(), probability(), into_input() }` | Input-carrying. A binding wraps each row as its own `Evidence` type holding the row number. |
| `Found { index: Option<usize>, probability }` | `Found<T> { selected(), candidates(), into_selected() }` | The winner's probability comes from its candidate. `none` is a candidate with `is_none()`. |
| `filter -> Vec<usize>` | `filter -> Batch<T>` of passing records | Streaming, one `Result` per item. The binding decides whether a mid-stream error raises after partial output. |
| `decide_many -> Vec<Judgment>`, `annotate -> Vec<Vec<(String, Annotated)>>` | `Batch<Row<T, Answer>>`, `Batch<AnnotatedRecord<T>>` with `values() -> &[NamedAnnotation]` | Streaming and ordered. |
| `Annotated::{Decision, Choice, Score(Scored), Tags, Failed}` | `Annotated::{Decision, Choice, Score(f64), Tags, Failed}` | Score loses `nearest`. |
| `Failed { kind: FailureKind, cause: Cause }`, `failed_questions(rows)` | `Failed { kind() -> ErrorKind, cause() -> FailureCause }` | Same six causes. The binding counts failures itself. |
| `Question::decide(t).cut(m)`, `.band(l, h)` | `Question::decide(t)?.cut()`, `.cut_at(m)?`, `.band(l, h)? -> BandedQuestion` | A band is its own type. `filter(&BandedQuestion)` does not compile. |
| `Question::choose(t, &[&str])`, `tag(t, &[&str])` | `Question::choose::<C>(t)?.option(C, desc)?`, `Question::tag::<C>(t)?.label(..)?` | Typed labels. Dynamic hosts use a loaded, unbound question (G5). |
| `Question::score(t, &[&str])` | `Question::score(t)?.level(name, desc)?.build()?` | |
| `Question::from_json`, `from_file`, `kind()`, `text()`, `members()`, `model()`, `digest()` | `Question::from_json -> LoadedQuestion`, `load`, `into_choose::<C>`, `into_tag::<C>` | The binding matches `LoadedQuestion::{Question, Banded}`. No getters (G6). |
| `QuestionKind` | none | G6. |
| `QuestionSet::from_json`, names | `QuestionSet::from_json`, `load`, `builder().question(..)` | No names or kinds accessor (G6). |
| `Kind::Any`, `Kind::Named(s)` | `&str` kind; `"*"` for any | |
| `RelationRule::new(name, from, to)` | `RelationRule::one_way(name, source, target)?` | |
| `RelationRule::new(..).either(true)` | `RelationRule::both_ways(name, kind)?` | `both_ways` takes one kind for both ends. A stand-in rule with two different kinds and `either` has no equivalent. |
| `Recognize::new().kinds(..).relation(n, a, b)` | `Recognize::builder().kind(Kind::new(n, desc)?)?.relation(rule)?` | Zero kinds means person, organization, place. |
| `Recognize::from_json`, `Relate::from_json` | none | G6. |
| `Recognized` with entity `{id, text, …}` and relation ids | `RecognizedEntity { name, kind, start, end, strength }`, `Relation { relation, source(), target(), probability }` | Offsets count Unicode scalar values. TypeScript must convert to UTF-16 code units. |
| `Relate::new().relation(..).either(..).kind_field(ptr)`, `relate_checked(engine, ask, &[&str], opts)`, `MAX_RELATE_RECORDS` | `Relate::builder().relation(rule)?.threshold(..)?.build()?`, `relate_with(ask, entities, opts)` | Input is `Entity::new(name, kind)`. The binding extracts the kind from its row and maps edges back to row ids by `(name, kind)`. The engine refuses more than 255 entities or an exact duplicate before any send. The binding still stops reading at 256 rows (R3-12). |
| `Edge` with record indexes | `Edge { relation(), source() -> &Entity, target(), probability() }` | |
| `rows_json`, `edges_json`, `Recognized::to_json` | none | G8. |
| `usage()` | `Engine::usage() -> Counters`, free `usage() -> Result<Counters>` | |

### 3.2 Per surface

Common to every surface: replace the connector line, the engine static, and the Cargo path dependencies on `contract/` and `standin/` with a path dependency on `crates/thinkthen` built without default features. Bring the workspace under main's lint, deny, policy, and ratchet (R1-28, R3-28). Rewrite the conformance runner onto main's `cases.json` (section 4). Move off `ENGINE_NULL` and `THINKTHEN_NULL` onto recording-folder replay and loopback listeners (G10). Tests that call the binding keep their assertions. Tests that name the stand-in, the wire stub under `experiments/`, or `synthetic-partial` are rewritten.

| Surface | What changes | Carries over untouched | Risks | Needs from the lanes |
|---|---|---|---|---|
| Python, with Polars | The pandas door leaves: Ian ruled on 2026-09-21 that Python's data frame is Polars (ADR 0017, "the data frame is Polars"). `src/lib.rs` imports and engine static. `bulk()` drops the `poll` tick, or runs on a worker like `step()` (G1). Choose and tag go through a loaded question (G5). `score` returns the position; `nearest` moves to `details`. `rank` and `find` rebuild index and probability from input-carrying results. Recognize and relate specs move to builders or wait for G6. | `src/arrow.rs` (Arrow C data import and export), `thinkthen/__init__.py` dispatch, `step()` and `TokenBridge`, the six exception classes, `check.sh` shape, NOTES, and tests that import `thinkthen`. | The null backend underlies most suites. Bulk interrupt latency regresses without G1 (R1-24, R4-23). R5-6 and R4-15 crash shapes stay open in the Arrow layer. | python4 (8 commits): the file-mapping bound, one map snapshot per call, and pointer-table checks for R5-6, with 3 tests. Take `origin/w7/python4:libraries/python` as the port base. |
| Polars | The Python Polars door rides the Arrow layer and ports with Python. `libraries/rust/src/polars.rs` is a feature-gated Series door over the stand-in. | The Arrow path and `test_polars_door`. | 0084 and 0086 allow one crate and exclude a Polars door. The Rust Series door has no home until a review rules on a second crate or a feature on `thinkthen`. | none |
| TypeScript | `addon/src/lib.rs` imports and engine static. `deadline_of` keeps its own rule (G7). The JSON envelope needs a result serializer (G8). Recognize offsets convert from scalar values to UTF-16 code units. | The napi `AsyncTask`, `AbortSignal` bridge, `index.js`, `index.mjs`, `index.d.ts`, `loader.cjs`, and the can-fail conformance probe. | One libuv worker per call stays a waiver (R2-27, R3-25). `DIVERGENCES.md` is stale (R7-12). | none |
| Ruby | `src/lib.rs` imports and engine value. `Recognize`, `Relate`, and `Question` from JSON need G6. | `cross()` (worker thread, `pthread_sigmask`, GVL release, `rb_thread_check_ints`), the watchdog, the six classes, and the Docker build. | The build image and the toolchain pin (R7-3, R7-10). `test_tick_gc` cannot fail (R7-9). The conformance runner has no rank or find arms. | none |
| R | `lib.rs` imports and the `LazyLock` engine. `deadline_of` keeps its rule. Choose, score, and tag can move to one-question `annotate` and close R2-23 (G4). The error string `kind␟retryable␟message` stays. | `interrupt_pending()` under `R_ToplevelExec`, the worker plus channel wait, `.tt_call` error-hook handling, text and encoding checks, and `make-tarball.sh` (retarget from `thinkthen-core` to `crates/thinkthen`). | The tarball must vendor `crates/thinkthen` offline (R4-7, R5-38). `tt_details` on choose and tag was never run. | python4 R commits: `I(5)` as a deadline (R7-11) and the R7-13 text. gate3's R NOTES line describes the method-H relate bar and is obsolete under main's relate. |
| Rust examples | `libraries/rust` is a forwarder that re-exports the contract. It retires. The slide, examples, and tests become external-crate examples over `thinkthen`. `DecideBuilder::cut()` gives the default cut the NOTES asked for. | The examples' intent and the defect and deadline test ideas. | Tests built on `thinkthen_standin::testkit` are rewritten whole. R2-28's skip-only passes must not return. | none |
| C | Queue item 9. `thinkthen_engine_new` builds an `Engine`. `control()` keeps the flat `deadline_ms` rule. The JSON door `thinkthen_call` needs G6 to parse and G8 to answer. | `contract/include/thinkthen.h` (19 symbols, ADR 0037), `libraries/c/DESIGN.md`, `guard()`, the per-thread per-engine failure table, the null matrix, ASan tests, and the C examples. | The door is a second crate producing a `.so`. ADR 0017's one-crate rule needs an explicit ruling for an unpublished C crate. Two images in one process (G9). DESIGN §2 names a conversion function the code never calls. | gate3: one NOTES line, obsolete with the relate conflict. |
| DuckDB | `lib.rs` imports and the engine static. `thinkthen_probability` needs G4. Relate turns `(id, body)` rows into `Entity` values and maps edges back. `details` on score re-maps (R3-11). | `connections.rs` routing, the identity probe, the reaper, read-only relate under a timer, `@file` checks, the chained SIGINT bridge, `guard.rs`, and every security suite. | Largest host layer and most rows (32). Connection-level cancel stays a documented limit (R5-23). Deck fixture drift (R5-24). Deprecated result API (R4-16). | gate3: the relate queue-wait message and its test. Port it. |
| SQLite | Imports and engine static. The saved-answer map `Saved` retires; the engine cache replaces it. Warm drops the `poll` hook or waits for G1. | DIRECTONLY registration and the 3.50 floor, the interrupt watcher, the `pthread_atfork` generation, `named_file`, the question cache, and `read_records`. | The host floor fetches SQLite over the network (R5-42 names it). | gate3: one NOTES line. |
| PostgreSQL | Imports and the lazy per-backend engine. `api_key` delivery through the builder. The per-backend saved-answer map may retire in favour of the engine's disk cache (R5-17, R7-6). | Extension SQL with the PUBLIC revoke, `@file` confinement through `open_beneath`, SPI relate capped at 256 rows, `run_batch` with `QueryCancelPending` polling, and the SQLSTATE map. | The 894-line Docker `check.sh`. The engine is built after the postmaster fork and relies on 0078. Single-row calls can be cancelled only with G1. `serde_cbor` stays under pgrx. | gate3: one NOTES line. |

The fast lane changes only the lint rung's changed-only mode and records its measurements. No surface needs it. Port it only if main's lint rung grows the same per-workspace loop.


## 4. Conformance

The branch holds two files. Its `conformance/cases.json`, `backend-profiles.json`, and `record-values.json` are byte-identical to main's. Its own `conformance/conformance.json` holds 83 cases (01 to 84, no 72) in a looser format. Plan item 6 folds the useful ones into main's `cases.json` (27 cases) before 0085, because 0085 runs every case.

### 4.1 Format

Both files use schema `thinkthen.conformance/1` and the six error kinds.

| Branch field | Main field |
|---|---|
| `expect.answer` | `expect.success.answers[].bare` |
| flat `expect.details` (`kind`, `probability`, `model`, `question_sha256`) | `details.answer.{kind,…}`, `details.model`, `details.question_sha256`, `details.requests` |
| `records` with one exchange list | one exchange per record, each with `evidence` |
| `indexes`, `ranking`, `none` | `expect.success.operation` |
| `set` | `question_set` with `version: 1` |
| `captured: stub`, `shaped-to-contract`, `synthesized` | `provenance.kind: synthetic_contract`; `captured` only with a stable recording `path` |
| `request` as an object | `request` as exact bytes |
| `budget_ms: 0`, `cancel_after_replies` | `operation.injection` (`expired_deadline`, `cancel_token`) |

Main has no success kind for `decide_many`, `recognize`, or `relate`, and no `usage` or `details` verb. The union ticket adds the kinds it needs. Recognize already has a main fixture: `crates/thinkthen/tests/fixtures/recognize-225/cases.json`.

### 4.2 Case by case

| Group | Branch cases | Main |
|---|---|---|
| Covered on main (same behaviour, other inputs; branch → main) | 01→01, 02→02, 03→03, 04→04, 05→13, 07→21, 11→06, 12→07, 13→11, 14→09, 15→17-annotate-mixed, 16→`requests`, 20→21, 21→16, 24→18-find-second, 26→22, 73→`requests`, 74→17-annotate-partial, 75→12, 77→10, 78→09 | keep main's |
| New and portable after re-encoding | 06 (filter over an empty list), 19 and 80 (need a decide-many kind), 23 (blank rank question), 76 (score tie-break), 79 (tag threshold excludes), 82, 83, 84 (annotate over repeated texts), 68 (recognize offsets past an accent and an emoji) | add |
| Recognize cases already in main's fixture | 35 of 28–67 (C05, C15, and C22 need `MISC` mapped to `other`) | fixture; refresh digests, since 0 of 40 branch digests match |
| Main's fixture keeps as known divergences | C06, C11, C13, C14, C16 | tests only |
| Binding-only | 17 (usage, cache, reset), 18 (cancel after two replies), 27 (spent budget), 81 (SQL NULL passes through), and the `jobs` field on 05, 19, 80 | each surface's own tests; 18 and 27 map to main's injections |
| Conflicts | 08, 09, 10, 22, 24, 25, 36, 69, 70, 71, and every recognize case with relations | below |

### 4.3 Conflicts with main

| Topic | Branch | Main today | Who settles it |
|---|---|---|---|
| Relate method (69, 70, 71) | The superseded all-H bake-off (`relate-h/arms.py`): one yes-or-no per pair over bare text records with kind `*`, a per-rule phrase, edges as `{name, source: index, target: index}` ordered by pair, and a stray `".thinkthen-backend"` digest. | `relate-design.md` and 0081/0088: same-kind pairs ask yes-or-no, cross-kind asks a choice, entities carry name and kind, edges carry full endpoints ordered by relation. | Re-record under main's planner. None of the probabilities or digests carry over. The branch's `form: "yes-no"` is not a public key. The open issue `2026-09-24-a-target-side-choice-asks-the-reversed-relation.md` must close first. |
| Recognize shape (all 41) | Entities `{id, text, …}` and relations by id. | `{name, kind, start, end, strength}`; relations with full endpoints. | Main's shape. 36-C09 also differs in content: the branch keeps "Karst and Vellum" as one name; main splits it under its connector-word rule. 0081 changed the relation request bytes, so the 9 cases with relations re-record. |
| Score detail (13, 75, 76) | `details.nearest_level` | `answer.level` | Main's name. The values agree. |
| Unsure word (03, 12, 15, file header) | `unsure: true`, `public_word_for_unsure: "unsure"` | The specification says "unresolved"; 0084 says `Answer::Unsure`. | The union ticket picks one word for cases and records it. |
| Find (24, 25) | A bare-string question, an index answer, `details.kind: "find"`, and `null` when nothing fits. | `{find, none, units}` questions, `bare: "u002"`, `cases.json` says kind `choice` while `result.md` says `find`, and case 19 says `bare: "none"` while `find.md` says null. | Main settles its own two contradictions first. |
| Invalid rule inside a JSON question (08, 09, 10, 22) | `usage` | `question-file.md`: a file that breaks a rule is `local` (exit 5); `usage` is for values typed on the command line. | Rule how a binding's JSON question maps. A question built from a binding's arguments reads as typed, which argues for `usage`. |
| `tuned_for` vs `calibrated` | No case touches it. | Ticket 0090 renamed the key to `tuned_for` in `backend-profiles.json`, `specification/result.md`, and the tool, per Ian's 2026-09-23 ruling. The case reader refuses the old `calibrated` key. | Each surface ticket reads `tuned_for` from main. No conflict with the branch. |

### 4.4 The skip table and divergences

The branch skip table holds 24 entries, all `skip`.

- Retire with the stand-in: the null backend's `backend` skips (Ruby, R, Rust, C, SQLite), the find-with-`none` skip, and the eight `usage` skips for the missing disk cache. `DIVERGENCES.md`'s run record and its 17 shaped-to-contract exchanges are history.
- Real runner gaps: `18-cancel-mid-batch` is skipped on all nine surfaces. Each surface ticket either adds a mid-batch cancel arm or proves it in its own tests and says so.
- Binding shape that stays: `local` on the six libraries, `deadline` on SQLite and DuckDB, rank and find on DuckDB, SQLite, C, and Ruby, filter, rank, and find on PostgreSQL, the filter band error on DuckDB and SQLite, `null` records on the six libraries and SQLite, and 77 and 79 on DuckDB. Each ported runner reports a skipped case as not run (R5-32).
- Retire with the relate conflict: PostgreSQL's `69-relate-alerts` skip.


## 5. Questions the 0085 and 0086 design reviews must answer

Each surface can bind once these have answers on record.

1. Interrupt check (G1). What is the public shape, which thread runs it, how often, and does it run from the answer arms within one tick? Until it lands, is the worker-thread pattern the ruled binding shape?
2. Bulk probability and bulk choose, score, and tag (G4). Does `decide_many` carry a probability? Is a one-question `annotate` the ruled bulk form for the other three?
3. Runtime labels (G5). Do `details(&Question)` and `annotate` accept a loaded, unbound choose or tag question, and at what cost?
4. Spec text and introspection (G6). Do `Recognize::from_json`, `Relate::from_json`, `Question::kind`, and `QuestionSet` names and kinds join the inventory?
5. Host deadline numbers (G7). What does `deadline_after(Duration::MAX)` do? Where does the -1, 0, and ceiling rule live?
6. Refusals in bindings (G11). Must bindings validate before a call? May a binding produce a `thinkthen::Error`, or does each host own a parallel six-kind error?
7. Result JSON (G8). One public serializer, or one specification page every JSON door follows?
8. Binding crates. May `libraries/*` and `databases/*` live on main as unpublished workspace members with a path dependency on `crates/thinkthen`? Is the C door crate a permitted second crate under ADR 0017? Where does the Rust Polars Series door go?
9. Two engine images in one process (G9). Prevent, or qualify the one-process width claim?
10. Offline tests for bindings (G10). Which fixture replaces the null backend, and which loopback reply stands in for `synthetic-partial`?
11. Counters. With no reset, what does conformance case 17 assert on each surface?
12. Score. Does `Annotated::Score` or a bulk form carry the nearest level, or does every surface call `details`?
13. Offsets. Is UTF-16 conversion a TypeScript binding duty, and does conformance carry a case for it?
14. Relate from rows. How does a database map row ids to entities? What happens with two rows of the same name and kind? Does `both_ways` need two kinds?
15. Engine lifetime. What does cloning an `Engine` cost? Is `default_engine()` safe under PostgreSQL's postmaster fork and Python's prefork servers through 0078's guard?
16. Words and shapes that main contradicts itself on: unsure against unresolved, find's `none` against null, and a JSON question's rule failure as `usage` or `local`.
