# 0304: One question cache and one batching path

Status: slice 3a landed; slices 3b, 3c and 3d next. Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 2 to 6.

## Outcome

One engine path serves all ten functions and every surface. It builds backend questions, looks each one up in the cache, packs only the misses into requests, sends them, splits the replies, stores each answer alone with a taken-at time, and returns results in input order with bounded memory.

## Design first

Write ADR 0111. It replaces ADR 0048 item 5 and settles:

- The question key: model, backend address, shared context, and one question as sent. What else changes an answer and so belongs in it.
- How `choose`, `tag`, and `score` quote each record in its own question.
- How `find`, `recognize`, and `relate` map onto question entries.
- The store: one SQLite file or another format. Locking between processes. What happens to per-digest lock files and coalescing.
- Test recordings: the simpler of one store or two, and a converter for old request-level recordings.
- Token and request accounting across only the sent questions.
- Partial replies, refusals that halve a batch, and retries.
- Where the requests-per-minute limit sits.
- Which current code the path deletes: content-cut batching, duplicate schedulers, and the batch paths in `cli`, `public`, and the facade.
- The build order in slices that each land green.

A fresh reviewer returns ACCEPT or findings before any build.

## Slices

ADR 0111 names each slice's proof. Each lands green with `cargo test --workspace`, `policy.py` and a fresh code review.

1. The quoted wire form on every surface. The shared `conformance/` cases and committed recordings are rewritten at the request level. The `spec` gate stops replaying probes. No storage or scheduler change.
2. Question key, SQLite store, sorted JSON Lines fixtures, `cache convert`, and the pipeline for the seven record functions on the command. Proof includes 100 records, then 120, sending only the 20 new questions on the loopback backend.
3. Public Rust API, Polars eager and lazy, the C door and the SQL hosts on `ask_all`.
4. `find`, `recognize` and `relate` on `ask_all`. Their fixtures convert without loss.
5. Remove the old store, locks, marker, request-level prune, both old schedulers and every committed `DIGEST.json` outside probe history.

## Retained behavior

Output order, exit codes, the null-versus-failure rule, key secrecy, no key read on replay.

## Proof

Named in the ADR. At least: a batch of 100 records followed by a run of 120 that includes them sends only the 20 new questions, counted on a loopback backend.

## Deferred gaps

Cache clearing and expiry.

## Evidence

- Starts from: ADR 0048, ADR 0055, and the 2026-09-29 mapping of System One calls.
- Keeps: output order, exit codes, null versus failure, key secrecy, no key on replay.
- Changes: the cache key, the store, and one batching path, per ADR 0111.
- Proof: named per slice in ADR 0111.
- Defers: cache clearing and expiry.

### Slice 2 evidence

- Starts from: slice 1 (d8ac2c30b), ADR 0111 sections 2 to 7 and 9, and rulings 2 to 6 and 8 of `sdlc/planning/cleanup-2026-09-30.md`.
- Keeps: output order, exit codes, null versus failure, key secrecy, no key read on replay, the wire bytes of slice 1, and every demo's printed output. Demos 14, 16 and 27 change only where they show request digests, because `answers[].request` now names a question key. Demo 12 now caches in a scratch copy of its fixture.
- Changes:
  - `core/pack.rs` holds the question key, the packer, the splitter and the even token share.
  - `engine/store.rs` holds the bundled rusqlite store. `engine/store/fixture.rs` reads and writes `thinkthen.jsonl`. `engine/store/convert.rs` and `thinkthen cache convert DIR [--quote]` merge a folder's sources.
  - `engine/pipeline.rs` runs lookup, pack misses, send, split, store and in-order emit for `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` on the command.
  - The annotate group scheduler, its slice batcher and the per-command batch metadata are deleted. `meta.batch` and `meta.batches` leave these rows, and `meta.requests` lists question keys.
  - `policy.py` pins rusqlite and refuses a committed `thinkthen.sqlite`. 42 committed folders gained `thinkthen.jsonl`; their old files stay until slice 5.
  - The loopback listener counts the questions it receives.
  - The recording, result, annotate and backends pages and the changelog describe the store.
  - The crate ratchet rises by 2,615 to 108,286 lines over main at 0324: the key, store, fixture reader, converter and pipeline arrive while the old recorder and schedulers stay for slices 3 to 5. The C library ratchet rises by 7 to 4,293 for the `nm` check.
- Proof:
  - `tests/backend/question_cache.rs`: 100 records, then 120 that include them, send one request holding exactly records 101 to 120, and the second run reports 100 cache answers. A partial reply stores its good answers, and the rerun asks only the failed question. A new tag label sends only its three questions. Two child processes write one store at once, and a third run sends nothing.
  - `tests/backend/question_cache.rs` also: a stored answer that no longer decodes is a miss that re-sends under a cache and a named exit 5 under `--replay`.
  - `tests/audit_write.rs` and `tests/diff.rs`: rows that name no batch setting leave a tuned `batch` alone and warn nothing.
  - `engine/store/tests.rs`: hit, miss, replace, a private new file, a read-only replay that writes nothing, a hot journal under replay, the busy limit and a stop during a wait each ending in under a second, a lookup that waits through another writer's commit, a folder holding both files, a hand-edited fixture, and the merge rule.
  - `tests/cache_convert.rs`: converting twice writes identical bytes, each old form converts to its keys and origins, and demo 14's converted bytes equal its committed fixture. A writer holding the live file makes convert wait, and its committed row reaches the fixture.
  - `tests/backend/batching/too_large.rs`: a 413 makes three attempts and stores both halves; a refused first half sends no second half.
  - `tests/backend/batching.rs` `a_pause_sends_the_open_batch` and `tests/backend/scheduling.rs`: a slow pipe closes a request at the pause, and the window never passes W.
  - `engine/pipeline/tests.rs`: a spent deadline or fired cancel reads no input and sends nothing, and a deadline starts no waiting request. These replace the deleted annotate scheduler's deadline tests.
  - `libraries/c/tests/door/main.rs`: `nm` finds no exported `sqlite3_` symbol. The SQLite extension's check already requires exactly one export.
  - Every green demo replays unchanged apart from the three digest demos named above.
- Defers:
  - Slice 3: the public Rust API, Polars, the C door and the SQL hosts. They keep the request cache, `meta.batch` and the old schedulers. `tests/backend/public_json.rs` leaves `meta.requests` out of its comparison until then. Conformance case 18, annotate with two groups, still holds two requests, so the command's loopback runner skips it until slice 3 rewrites the shared cases.
  - Slice 4: `find`, `recognize` and `relate` keep the old store. The digest-lock and prune tests now run on `find`.
  - Slice 5: the old recorder, locks, marker, request-level prune and every old `DIGEST.json`.
  - `audit` and `diff` read `meta.batch.setting`. Command rows no longer carry it, so both treat such a run's setting as unknown: `audit --write` leaves `batch` alone and neither warns. A later ticket should give them the setting another way.
  - Slice 3: a refused first half fails the untried second half (`engine/pipeline/send.rs`). Revisit when the SQL hosts continue past failures.
  - Slice 3: rusqlite moves out of the `cli` feature.
  - Slice 5: the store's folder-is-a-file check repeats `Recorder::of_private`'s; one goes.
  - Cache clearing and expiry.

### Slice 3 split

Slice 3 lands in four parts, one at a time. 3b, 3c and 3d touch disjoint files once 3a lands, so they can build in parallel lanes.

- **3a. The public Rust API on `ask_all`.** Detailed below.
- **3b. The C door and the SQL hosts.** They already call the public `*_with` methods, so 3a moves their batching. The prep note weighed removing the SQL host statics that 0323 named (`ACTIVE_THROTTLE`, `SEND_BUDGET`, the engine maps, SQLite's `STORED`, `ENGINE` and budget registry); 3b kept them, and "Slice 3b evidence" says why. 3b proves an SQL row with a missing pointer fails alone while its neighbours answer, and runs every SQL and C conformance case on the question store. It also fixes the two facts of ADR 0111's 2026-09-30 amendment:
  - The SQLite extension's store runs on the host's SQLite, through `thinkthen`'s `host-sqlite` feature from 3a. The extension's 3.50.0 floor already covers the store's need for 3.24 (`ON CONFLICT DO UPDATE`). The store opens connections on the pipeline's background thread, so 3b refuses a single-thread host when a cache, record or replay folder is named, with its own sentence. rusqlite's thread-mode check at open stays as the backstop. A host that names no folder makes no background SQLite call, so a refusal at load was rejected (`sdlc/planning/0304-slices-3b-3d-prep.md`, option A).
  - `libthinkthen.a` and R's static library export about 285 global `sqlite3_` symbols. 3b localizes them with a partial link before archiving on Linux, hides them in R's package on Linux, and flips `the_static_library_still_exports_sqlite_symbols_until_slice_3b` to require zero. `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md` stays open for macOS.
  - The inverted symbol check is a placeholder, not a gate. It must become a gate that requires zero `sqlite3_` symbols before any release.
- **3c. Polars eager and lazy.** Polars calls the public batch methods, so 3a moves its batching. 3c proves a lazy frame collected twice sends nothing the second time and removes any frame-only batching left.
- **3d. One owner for the send limits.** The engine part moves the throttle, the gates and pacer, and the totals behind one owner, per the decision below. The SQL hosts' own limit copies wait for 3b and land as a follow-up slice, 3e.

The process-wide throttle is an Ian ruling. ADR 0017's amendment from ticket 0077 says one process has one throttle, and a second engine with another throttle is refused. `PROCESS_WIDTH`, `WIDTH_CHILD` and the 1 ms `REBUILD_POLL` exist to keep that ruling safe across a fork. `PROCESS_GATES` (the 429 gate and the pacer) and `process_budget` (the request total) follow the same per-process rule. Cleanup step 4 lists process-wide statics as hacks. The two conflict, so 3a keeps them and 3d waits. Options for Ian:

1. Keep one process throttle, pacer, gate and request total. The statics stay as honest process state, as signals do. 3d only replaces the 1 ms poll with the 50 ms stop check. Cost: small. Recommended, because every engine in a process sends against one account's rate limit.
2. Give each engine its own throttle, pacer, gate and request total, and drop the "already active for this process" refusal. Every static and the fork rebuild for them go. Cost: the settings, records and backends pages, and the Python, TypeScript, PostgreSQL and DuckDB tests that pin the refusal. Two engines in one process could then send twice the rate.

Decision for 3d, 2026-09-30: option 1. One throttle per process stays, per Ian's ruling in ticket 0077, and 3d replaces the 1 ms `REBUILD_POLL` with the pipeline's 50 ms stop check. This is a coordinator default. Ian can overturn it.

### Slice 3a evidence

- Starts from: slice 2 (`1fbbe08e0`) and its Defers; ADR 0111 section 4 (the public Rust API row and the host demand bridge) and section 10; ticket 0323's inventory of engine imports of `public`; `sdlc/issues/closed/2026-09-30-public-batch-holds-one-send-under-a-throttle.md`.
- Keeps: every public name and signature; lazy `Batch` pulls, input order, the stop at the first failed record, `facts()`; `BatchSetting::Records(1)` returning each row before the next pull, as `specification/records.md` and `libraries/rust/README.md` promise; recoverable native rows and the spent rows after a send-budget denial; the wire bytes of slice 1; key secrecy and no key read on replay.
- Changes:
  - `filter`, `decide_many`, `choose_many`, `score_many`, `tag_many`, `rank`, `details_many`, `annotate` and `details_many_recoverable_with` run on `Engine::ask_all`. `ask_all` runs on a background thread. The calling thread is the host: it pulls a record on each ask and returns on each row.
  - Public rows lose `meta.batch` and list question keys in `meta.requests`, as the command's rows did in slice 2. A cached rerun answers from the store.
  - `public/batch/planned*`, `public/batch/annotation.rs`, the planning in `public/native_batch.rs`, `engine/facade/native_batch.rs`, `engine/facade/annotate/batch.rs`, `engine/facade/split.rs` and `engine/annotate_batching.rs` go, unless slice 4's functions still need a piece.
  - The pipeline and the store build without the `cli` feature, so rusqlite leaves it.
  - `SendBudget` and its reservations move from `public` into `engine`, with `public` re-exporting them. `engine` then names nothing from `public`, and `policy.py` refuses an engine reference to `public` as it refuses a core one.
- Proof, offline and counted on the loopback backend:
  - Through the public Rust API, 100 records and then 120 that include them send one request holding exactly records 101 to 120, and the second run reports 100 cache answers.
  - The throttle issue's test passes. Its batch-1 form contradicts the interactive promise above, so the test runs at batch 2 and fills throttle 2, and the interactive test keeps batch 1 at one send.
  - A stop during a pulled batch sends nothing new and joins every worker; rows keep input order with slow and fast replies mixed.
  - `policy.py` refuses a planted engine reference to `public`.
  - `sdlc/scripts/test`, `spec`, workspace clippy, `policy.py`, `tickets` and `lint` pass, with any binding whose build breaks fixed at its call site.
  - After rebasing onto main, every surface check passes, run one at a time: Rust, Polars, C, PHP, C#, JVM, Dart, Swift, Zig, Go, C++, Ada, Objective-C, COBOL, Python, TypeScript, Ruby, R, DuckDB, SQLite and PostgreSQL. Dart runs with the pinned Dart and Flutter under `~/.local/opt` and `PUB_CACHE=$HOME/.pub-cache`, because its source check otherwise reads an empty scratch pub cache. In the first sweep, DuckDB's `b13c_try_details_split_denials` failed once at total 2 with every slot failed, while the machine's load average was near 14. The rerun passed, and the case then passed 90 of 90 runs alone and six at a time. The one failure stays unexplained. Main must stay green on surfaces, so 3b, 3c and 3d can build in parallel lanes. The Ruby check runs now that the pinned Ruby 3.4.11 is installed. The TypeScript check includes `shapes.test.mjs` "details equals the command --details document", which slice 2 turned red on main.
- Defers: 3b, 3c and 3d above; `default_engine`, which holds an engine rather than a limit and which the Python, TypeScript, Ruby and R module functions call, to the binding pass of ticket 0314; slice 4's `find`, `recognize` and `relate`; slice 5's old store.

### Slice 3b evidence

- Starts from: slice 3a (`f4436de26`), ADR 0111 section 4 and its 2026-09-30 amendment, and `sdlc/planning/0304-slices-3b-3d-prep.md`, which a fresh review checked as this slice's ticket review. 3a already moved the C door and the three SQL extensions onto the public `*_with` methods, which run `Engine::ask_all`, and repointed every host test. 3b confirmed that no C door or SQL host holds a packer of its own.
- Keeps: every retained regression the prep note lists for 3b. The C door's header-symbols and soname test, invalid settings, and the refused key stay. The SQLite 3.49.0 load refusal, cancellation, error format, invalid input, and the 0318 folder rules stay. DuckDB's recoverable failures beside an answer and its cancellation suite stay, as do PostgreSQL's panic text and the per-row budget denial of ticket 0240 (`native_denied_retry_keeps_a_later_request_answered_first`). No test was merged or deleted. The inverted static symbol test became the gate below.
- Changes:
  - `libraries/c/localize.sh` writes the release static library. On Linux, `ld -r` joins the members the header's functions need, `objcopy --keep-global-symbol` makes every other global name local, and `ar` archives the one object. `release-pack`, the Go, C++ and Zig checks, and the C door tests use it, so the checks test the archive users get. macOS keeps Cargo's archive; see Defers.
  - The SQLite extension refuses a named cache, record or replay folder on a single-thread host, in `thinkthen_configure` and before the first engine build, which also covers `THINKTHEN_CACHE`. The sentence is "a cache, record or replay folder needs a thread-safe SQLite, and this host's SQLite is single-threaded; name no folder, or load thinkthen into a thread-safe SQLite". The check reads `sqlite3_threadsafe()` and the single-thread mutex marker, as rusqlite does at open, and rusqlite's own check stays as the backstop. Option A of the prep note; the ADR amendment already records the rejected refusal at load.
  - `Engine::annotate_each_with`, hidden from the docs, runs annotate over texts in hand for an SQL host. A record refused before its request fails alone, and every other record is still asked, answered and stored. Other failures end the call, as they end `annotate_with`. More texts than the engine's record limit are refused before any send, as `rank` refuses them, so DuckDB keeps `SET thinkthen_max_requests`. DuckDB's vector `thinkthen_annotate` uses it. This touches `public/bulk/annotation.rs` and not `public/engine.rs`. SQLite and PostgreSQL call annotate one record at a time, so their rows already failed alone.
  - The R package links its archive with `-Wl,--exclude-libs,ALL` on Linux, so its shared object exports no `sqlite3_` name. A build before the change exported 285. R finds its routines through their registration.
  - The Go, Zig and C++ setup steps in each README use `localize.sh`.
  - The SQLite and C door shared cases each run on a scratch cache folder and require one answer row in `thinkthen.sqlite` for each good question key.
  - Host statics stay, as the prep note recommends: the engine maps, SQLite's `STORED` and `ENGINE`, the parsed-question caches, and SQLite's per-connection deadline `REGISTRY` are host state with no other home. The PostgreSQL and DuckDB send-limit copies (`ACTIVE_THROTTLE`, `SEND_BUDGET`) go to slice 3e, as 3d's Defers say.
  - Ratchets: the crate rises by 82 lines for `annotate_each_with` and its two helpers, the C door by 45, SQLite by 35 source and 84 test lines, DuckDB by 2 source and 30 test lines, the Go fixtures by 4, and R by 5.
- Proof, offline and counted on the loopback backend:
  - `libraries/c/tests/door/main.rs` `the_static_library_exports_exactly_the_header_symbols`: the archive's global names equal the header's 30 functions, and none starts with `sqlite3_`. A C program linked the localized archive beside the system's static SQLite. The Go check's `abi.py` asserts the same set on its installed archive.
  - `databases/sqlite/tests/test_thread_mode.py`, on a 3.50.0 library built with `SQLITE_THREADSAFE=0` by `tests/host_sqlite.sh`: a call with no folder answers with one send; a configured cache, record or replay folder and `THINKTHEN_CACHE` are each refused with the exact sentence, create no folder, and send nothing.
  - `databases/duckdb/tools/settings_suite.py` `an_annotate_row_missing_its_part_fails_alone`: at one record a request, a first record missing `/body` fails the statement with "the record holds nothing at `/body`", `thinkthen_usage()` shows five sends for its five neighbours, and a statement without the bad row answers all five from the store with no send. Before the change it showed 0 sends.
  - `settings_suite.py` `annotate_keeps_the_engines_record_limit`: under `SET thinkthen_max_requests = 2`, three texts are refused with "this engine answers at most 2 records in one call" and nothing is sent. The review found the first 3b build sent all three.
  - `databases/sqlite/tests/test_values.py` `test_an_annotate_row_missing_its_part_fails_alone`: a last record missing `/body` fails the statement with the same sentence, its two neighbours send once each, and a rerun without it answers both from the store.
  - `libraries/r/check.sh` requires `nm -D` on the installed package to show no `sqlite3_` name on Linux.
  - The SQLite shared cases (52 pass, 2 not run) and the C door shared cases pass with the store count.
  - Every check the change reaches passed, one at a time: `test`, `spec`, workspace clippy, `policy.py`, `tickets`, `lint` in a clean checkout, the C door tests, and the Polars, C#, JVM, Swift, Go, C++, PHP, Ada, Objective-C, COBOL, Zig, Dart, SQLite, DuckDB, PostgreSQL, Python, TypeScript, Ruby and R checks.
- Defers:
  - The macOS static library still exports SQLite's names. One M5 try of `cc -r -Wl,-exported_symbols_list` wrote an object Apple's `nm` could not read. `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md` stays open for macOS, to the release rehearsal.
  - DuckDB's `TRY()` refuses volatile functions, so SQL cannot keep a failed annotate row beside its neighbours' values in one statement. The neighbours' answers reach the store, and a rerun reads them. This is DuckDB's behavior, not debt.
  - Slice 3e: the SQL hosts' send-limit copies, as 3d's Defers say.
  - `sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md` owns three proof gaps. The DuckDB and PostgreSQL shared case runners do not count store rows. PostgreSQL has no test that pins a missing-pointer row failing alone. The SQLite extension has no read-only replay or busy wait row on the 3.50.0 host.

### Slice 3c evidence

- Starts from: slice 3a (`f4436de26`); `sdlc/planning/0304-slices-3b-3d-prep.md` section 3c, which served as this slice's ticket review; ticket 0307, which dropped `polars/streaming` from the feature. The measurement harness and its logs stay local and unpushed, as experiment 415.
- Keeps: every public name, signature and dtype of the Polars door; null cells asking nothing and returning null in place; input order; the stop at the first failed row with its facts; the `failed` struct and its validity; the lazy expression's per-morsel call, shared token and tally, and per-morsel deadline. Retained regressions, unchanged: `door.rs` `every_refusal_is_pinned_and_sends_nothing` and `a_failed_row_ends_a_series_call`; `lazy.rs` `a_shared_token_stops_a_later_evaluation_without_a_send` and `lazy_matches_eager_values_bodies_and_shared_tally`; `deadline.rs`; `throttle_equality.rs`; `public/frame/lazy.rs` `a_panic_stays_inside_the_polars_compute_boundary`. No test was merged or deleted.
- Changes:
  - No frame-only batching was left after 3a. Every door method calls a public batch method on `Engine::ask_all`.
  - `public/frame/column.rs` reads the text column in place as a `StringChunked`, and `streamed` pulls the call's rows in input order into typed Polars builders as each row arrives. The door no longer collects a `Vec` of rows, and it no longer rebuilds null positions with a `take`. One kept behavior changes on defect paths only: the old door consumed the whole call before any conversion could fail, and now a conversion defect (a kind mismatch, a missing failure marker or a chosen label with no probability) stops the call at that row. `Answers` builds one question's column, and `Failures` builds the `failed` struct row by row.
  - `libraries/polars/README.md` states what a call holds and the measured peaks.
  - The crate ratchet rises by 115 to 105,744: the row builders replace the collect-then-convert helpers for 66 more source lines, and the new lazy test adds 49 nonblank lines.
- Proof:
  - `tests/polars/lazy.rs` `a_cached_lazy_frame_collected_twice_sends_nothing_the_second_time`: with a scratch cache, the first collect of 30 rows sends one request of 30 questions; the second collect sends nothing and equals the first; a cached slice sends nothing; a later 10-row slice of a 40-row frame sends one request of only its 5 new questions, so a slice placed before the expression limits what it asks.
  - Memory and time, release build, 100,000 distinct rows answered from a replay folder, peak resident memory from `VmHWM`, 32 MB before the call on both builds, machine load near 15 to 20:

    | Call | Peak before | Peak after | Seconds before | Seconds after |
    | --- | ---: | ---: | ---: | ---: |
    | `annotate_frame`, decide and choose | 109 MB | 47 MB | 7.3 to 10.9 | 10.6 to 11.0 |
    | `decide_series` | 38 MB | 37 MB | 4.8 to 6.9 | 5.7 to 6.2 |
    | `decide_expr`, lazy collect | 43 MB | 42 MB | 5.5 to 7.7 | 6.0 to 6.7 |

    Time did not change beyond the load's noise. `decide_many_with` alone over the same column takes the same time as `decide_series` (5.3 to 6.9 seconds), so the replay lookups in the engine, not the door, set the time.
  - `sh libraries/polars/check.sh 0` passes: clippy with and without default features, every `polars_*` test, and the doc test.
- Defers:
  - True lazy streaming. The default in-memory engine collects the whole frame and calls the expression once over the whole column. The streaming engine needs Polars' `streaming` feature, which 0307 dropped, so the checks cannot compile it. A user who adds that feature gets one call per morsel, since the expression is elementwise. `LazyFrame::collect_batches` also needs that engine.
  - Engine replay speed: about 50 microseconds per answered question under load. It belongs to the engine, not the Polars door.

### Slice 3d evidence

The engine part only. The SQL hosts' own limit copies wait for 3b and land as a follow-up slice.

- Starts from: slice 3a (`f4436de26`); the "Slice 3 split" decision above (option 1, one throttle per process, Ian's ticket 0077 ruling, overturnable); `sdlc/planning/0304-slices-3b-3d-prep.md` section 3d, which served as this slice's ticket review; ticket 0323's hacks table.
- Keeps: one throttle per process and its "already active for this process" refusal on every surface; the 429 gate and the requests-per-minute pacer per address; the request total and the estimated input cap; a fresh set of limits in a forked child; the 50 ms host polls ticket 0323 keeps.
- Changes:
  - `engine/limits.rs` owns every process send limit: the throttle (`Widths`), the 429 gates and pacer (`Gates`) and the request and estimated input totals (`SendBudget`), in one fork-safe `Guarded` cell, `PROCESS_LIMITS`. One fork rebuild now replaces all three.
  - `PROCESS_WIDTH`, `PROCESS_GATES` and the `process_budget` `OnceLock` go, with their accessors `process_width`, `process_width_of`, `process_gates` and `process_budget`. `limits::of(pid, cancel)` and `limits::process()` replace them.
  - The 1 ms `REBUILD_POLL` goes. A thread that finds another rebuilding waits the 50 ms stop check. `limits::of` is the one production route. The command's `--jobs` registration passes a default stop token, because it holds no call yet; it waits only while another thread of its process builds the limits, which happens on first use or after a fork. `limits::process()` serves tests alone.
  - An engine reads the process total from its state, which a forked child rebuilds from the child's limits. Before, `with_process_budget` saved the total when the engine was built, so the review found that a child could count an inherited engine and its own engine against two totals.
  - `SendBudget` counts through the same `Guarded` cell instead of its own spin-loop reset after a fork. A refund goes to the counts it reserved from. The prep note's open question is settled: the request total already reset in a forked child and still does.
  - `policy.py` holds the one door on `PROCESS_LIMITS` in `engine/limits.rs`, with its plants and control renamed.
  - The crate ratchet rises by 57 lines to 105,801 over main at slice 3c: the owner struct, the rebuild-wait test and the real-fork total proof outweigh the removed statics, accessors and spin reset. `engine/mod.rs` falls from 565 to 338 nonblank lines.
- Proof:
  - `engine/limits.rs` `a_stop_ends_a_rebuild_wait`: a fired stop ends a rebuild wait with `Cancelled`, and a live wait lasts one stop check.
  - `engine/budget.rs` `a_forked_child_counts_from_zero` replaces `inherited_budget_and_reset_marker_do_not_block_a_child`. A parent marker left behind is the `process/tests.rs` row "a child replaces the parent's rebuild".
  - `conformance/consumer/fork-probe` `an_inherited_engine_and_a_child_engine_share_one_request_total`: after a real fork, an engine the parent built and one the child built share one `max_requests_total(Some(1))`; the second is refused before its first send and the backend counts one request. It failed on the first 3d build.
  - Kept unchanged and passing: `cli/schedule/width_tests.rs` `every_live_path_and_every_engine_share_one_cap`; `engine/width_tests.rs`; `engine/process/tests.rs`; `engine/facade/fork_tests.rs`; `engine/backoff.rs` `paced_starts_from_many_threads_keep_one_interval_apart`; the send budget and token cap denials in `engine/http/tests.rs`, `engine/send_budget.rs` and `tests/public_env`; the six other real-fork probe tests.
  - After rebasing onto slice 3c, each passes, run one at a time: `sdlc/scripts/test`, `spec`, workspace clippy with `-D warnings`, `policy.py`, `tickets`, `lint` in a clean checkout, the fork probe, and the C door, Polars, Rust, Python, TypeScript, Go, C#, SQLite, DuckDB and PostgreSQL surface checks against one loopback backend. Python, TypeScript, PostgreSQL and DuckDB pin the "already active for this process" sentence.
- Defers:
  - The SQL hosts' copies: PostgreSQL's `ACTIVE_THROTTLE` and early refusal (`call/settings.rs`) and `SEND_BUDGET`, and DuckDB's `SEND_BUDGET`. They touch the SQL host files 3b reruns, so they wait for 3b and land as slice 3e. That slice needs a public way to reach the process total.
  - `default_engine`, to ticket 0314's binding pass, as 3a recorded.

### Slice 3e evidence

The SQL hosts' own copies of the send limits, which 3d deferred.

- Starts from: slice 3d's engine part (`f2278c900`), whose `engine/limits.rs` owns the throttle, the gates and pacer, and the process request total; slice 3b (`0ee0f2b63`); `sdlc/planning/0304-slices-3b-3d-prep.md` section 3d item 3, which served as this slice's ticket review.
- Keeps: every SQL setting (`SET thinkthen.max_requests_total`, `SET thinkthen_max_requests_total`, `thinkthen.throttle`, `thinkthen_max_requests` and the SQLite `thinkthen_configure` keys) and every refusal sentence and SQLSTATE; one process total that a forked child restarts; the per-row budget denial of ticket 0240; the cancellation, invalid-input and conflict regressions of the three SQL checks. No test was merged or deleted.
- Changes:
  - `CallOptions::max_requests_total(limit)` caps one call against the process total that every engine already counts. A send reserves once, against the tighter of the engine's and the call's limit, so a host passing both never counts a send twice.
  - `process_requests_sent()` reads that total, for a host that cuts its rows before a call.
  - PostgreSQL drops `SEND_BUDGET` and `ACTIVE_THROTTLE` and the early throttle refusal in `call/settings.rs`. Each call passes `thinkthen.max_requests_total` as its call total, and the engine's own refusal at build gives the same "throttle N is already active for this process" sentence and 22023.
  - DuckDB drops `SEND_BUDGET`. Its calls pass `thinkthen_max_requests_total` as their call total, and `within_total` reads `process_requests_sent()` instead of summing the engines' usage counters.
  - SQLite had no copy: `max_requests_total` already reached the engine through `EngineBuilder::max_requests_total`, and its throttle through the builder.
  - The PostgreSQL `single_cancel` check proves order: the cancel ends the statement while the held arm still keeps its only reply. Its 200 ms limit moves to `single_cancel_within_200_ms`, which runs only under the stress profile (`test-stress --run`).
  - Ratchets: the crate rises by 79 lines, for the call total, the read and the real-fork proof. PostgreSQL falls by 17 and DuckDB by 1.
- Proof:
  - `conformance/consumer/fork-probe` `a_call_total_counts_every_engine_of_the_process`: in a forked child, two engines share one total under call totals; a call cap and an engine cap each refuse when they are the tighter, `process_requests_sent()` reads 3, and the backend counts 3.
  - `databases/postgresql/check.sh` `an_annotate_row_missing_its_part_fails_alone`: a record missing `/body` fails with "the record holds nothing at `/body`", its two neighbours send once each, and a rerun answers both from the store with no send. It closes that item of `sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md`.
  - Kept unchanged and passing: PostgreSQL `a_changed_throttle_refuses` and the six `max_requests_total` steps; DuckDB `settings_suite.py`, `verbs_budget.py`, `plan_suite.py` and `relate_suite.py`, which pin the spent sentence, the fork reset and the throttle conflict; SQLite `test_try_budget.py` and `test_settings.py`.
  - Each passes, run one at a time on lane 1's own lock: `sdlc/scripts/test`, `spec`, workspace and consumer clippy with `-D warnings`, `policy.py`, `tickets`, the inventory, `lint` in a clean clone, the fork probe, the C door check, and the SQLite, DuckDB and PostgreSQL checks, with PostgreSQL's stress step alone. No binding reads an SQL host's limits. Two `test` runs at load 22 each failed one test outside this slice, and both are filed as debt: `sdlc/issues/2026-09-30-graded-rank-tests-rewrite-one-question-file-in-place.md` and `sdlc/issues/2026-09-30-ordered-output-test-races-the-next-request-under-load.md`. The third run passed all 1,293.

- Defers:
  - `sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md` keeps two gaps: the DuckDB and PostgreSQL shared case runners count no store rows, which needs a DuckDB harness change, and SQLite has no read-only replay or busy wait row on the 3.50.0 host.

### Added public declarations

```text
const fn CallOptions::max_requests_total(self, Option<u64>) -> CallOptions<'a>
fn process_requests_sent() -> u64
```

### Slice 4 evidence

- Starts from: slice 3d (`f2278c900`); ADR 0111 sections 5 and 6 and build step 4; Ian's ruling 4, which caches these functions' backend questions by context, question and model; `sdlc/planning/after-slice-3-prep.md` section 1, which served as this slice's ticket review; the relate precision issue, whose default keeps relate's answers unchanged here.
- Keeps: every function's wire form, so recognize and relate print the same bytes under replay; the 255-entity refusal and the recognize text-size refusal; relate's partial failure count and exit 6 on a live run; one request per `find` state and per recognize step 1 and 2 window, split only by a profile; the 400-question cap for relate and recognize step 3, or a profile's smaller limit; no key read on replay; a sent single request that finishes after a stop and prints its answer; a stop seen while several relate requests are out, which ends the call as `Cancelled` once they finish.
- Changes:
  - `engine/facade/each.rs` asks planned questions one input each on `ask_all`: `find`, the three recognize steps and relate. Each question is keyed, stored and coalesced as the record functions' are. The pure packer gives dry runs and pre-checks the same requests the pipeline sends with nothing cached.
  - `Packing` gains a question cap and whether requests close at the backend ceiling. `find` and recognize steps 1 and 2 ignore the ceiling, as their old path did.
  - A whole request that fails in a call that does not continue now stops the pipeline sending, as one job did. The relate `at_once` send count caught this. The rule covers the record functions too.
  - One engine reads its replay fixture once per process and answers every later call from that copy. Each `recognize` line and step is its own call, so the five-kind page read the 2.7 MB fixture 600 times and took 16 seconds; it now takes 0.3 seconds with the same bytes.
  - A stop seen while the call had planned several requests ends it as `Cancelled` once they finish. The count assumes nothing is cached, as the chunked sender's did, so a mostly cached call that sends one request still stops.
  - Details list question keys in `meta.requests` and each answer's `request`. Dry runs keep request digests, now from the pure packer.
  - `pair_chunks`, `relation_ceiling`, `ask_chunks_with_plan`, the `PREPARATIONS` counter and `PairPlan`'s re-exports go. `ask_profile` and `PreparedRequest::with_profile` serve tests only.
  - Relate's partial replay on `spec/relate.md` now exits 5 and names the failed question's key, because ADR 0111 section 6 never stores a failed answer. Exit 6 with the good answer kept is proven on loopback. `specification/relate.md`, `recognize.md` and `result.md` say question key where they said request digest, and `CHANGELOG.md` records the change.
  - The shared settings case `request-bytes-splits-relations` turns the cache off in both steps. Its second step asked the same pairs, which the question cache now answers without a send.
  - The ratchet falls by 384 lines to 107,520, from 107,904 on main after 0334. The removed old-store tests, `pair_chunks` and the chunked sender outweigh `each.rs`, the shared fixture and the new proofs.
- Proof, counted on the loopback backend in `tests/backend/question_cache_steps.rs` unless named:
  - Relate over 420 pairs sends a request of 400 questions and one of 20, stores 420 answers, and a rerun sends zero requests and prints the same bytes.
  - Adding a rule to a cached relate run sends only that rule's 3 questions, and the output equals an uncached run.
  - A relate reply with one wrong-kind answer exits 6 and stores the good answer. The rerun sends one request holding one question.
  - A cached recognize rerun sends zero requests across all three steps. Adding a line sends exactly the questions that line asks alone, and the output equals an uncached run.
  - `find.rs` `cache_records_once_and_then_replays_without_a_key_or_second_request` keeps `find`'s zero-send rerun.
  - `relate/ceiling.rs` `one_job_sends_the_planned_requests_past_the_pipeline_window`: with one job, 9,900 pairs go as 24 requests of 400 and one of 300, as the plan counts. Review found the first build closed a request of 192 once the window of 8,192 questions filled. Each step's input cap is now all its questions, so the window holds them all.
  - `batching.rs` `a_failed_request_sends_no_later_request`: `decide --jobs 2` whose line 2 request fails while line 1's is out sends 2 requests and prints line 1. It sent 3 before the stop rule.
  - The shared conformance cases, including `18-find-second`, `19-find-none`, 41 to 50, `51-same-kind-alerts` and `52-cross-kind-staff`, replay converted fixtures through `cache convert` and compare every question key.
- Deleted tests, with their replacements. Each drove only the old recorder through `find`:
  - `cache_locking.rs`, `cache_locking/retained.rs` and `cache_prune_locking.rs`: the busy limit and a stop during a wait (`engine/store/tests.rs` `a_wait_past_the_busy_limit_is_a_storage_failure_and_a_stop_ends_it`), a lookup that waits through a commit (`a_lookup_waits_through_another_writer_and_then_answers`), a read-only replay that writes nothing (`a_new_store_is_private_and_a_read_only_replay_writes_nothing`), a hot journal (`a_read_only_replay_that_meets_an_unfinished_write_is_refused`), and two children writing one store (`question_cache.rs` `two_processes_write_one_store_at_once`). Lock setup failing before a key is the secrecy route "a recording folder that cannot be made".
  - The two padded-response tests in `recording_conflicts.rs`: ADR 0111 removed recording conflicts, and `recording_again_replaces_the_stored_answer` holds the new rule.
  - `default_cache.rs` `prune_waits_for_a_live_partial_and_preserves_its_installed_entry`: prune reads only old files until slice 5.
  - `engine/facade/recognize/tests.rs` `recognition_sends_the_pair_chunks_it_prepared_once`: the pure packer now plans requests, and the dry-run tests compare its request to the sent one.
- Checks, after rebasing onto 0334 and run one at a time: `sdlc/scripts/test`, `spec`, workspace clippy with `-D warnings`, `policy.py`, `tickets` and `lint` in a clean checkout pass. Every surface check passes against one loopback backend: Rust, the C door, PHP, C#, JVM, Swift, Zig, Go, C++, Ada, Objective-C, COBOL, Python, TypeScript, Ruby, R, DuckDB, SQLite, PostgreSQL and Polars. Dart did not run, because its tools are absent. Ada, Objective-C and COBOL take the lane lock themselves, so under `surfaces` they time out on the lock the rung holds; each passed alone.
  - The TypeScript check caught a recognize details test that still looked for a request digest. The conformance runners of the C door, the consumer, Python, TypeScript, Ruby, R, SQLite and PostgreSQL now expect question keys for every verb, where they kept digests for `find`, `recognize` and `relate` until this slice.
- Defers:
  - The command's many-line recognize still runs lines through `schedule::over_records`. Slice 5 moves that runner with the other `Engine::records` callers, per the prep's option (b).
  - `ask_chunks`, `facade::split` and `prepared_request.rs` stay for `check`, the conformance runner and tests until slice 5, though ADR 0111 step 4 lists `ask_chunks`. The facade conformance runner (`cli/conformance_tests/runner.rs`) still sends `recognize` and `relate` through `ask_chunks`; the command conformance test and the loopback cases run the new path.
  - The C door's typed relate rows, which 0314 slice 2 deferred, are not taken here. `sdlc/issues/2026-09-30-c-door-relate-rows-have-no-owner.md` holds them.
  - The site's recognize and relate replay folders stay unconverted, per `sdlc/issues/2026-09-30-site-replay-folders-have-no-fixture.md`.

## What the build taught us

### Slice 1

- `max_evidence_bytes` bounds each quoted record's evidence bytes, as before. It now also counts the shared state, which is the 44-byte fixed sentence or the context. A value below 44 therefore refuses every quoted request. The batcher and the single-record plan each check the record.
- A question written as JSON cannot carry a quote. That record still goes as the state with its question unquoted, and a context with such a question still refuses, as ADR 0111 section 1 says.
- A structured tag description puts its questions in an array. The quote goes at the head of array element 0. The ADR does not cover this, and `specification/tag.md` now says it.
- Two paths quote a JSON record differently. An annotate root group quotes the record's compact text as a JSON string. The batcher quotes the object itself. Slice 2 must pick one form for its question key.
- The annotate slice path hands its questions to a group batcher, which quotes them. Quoting them earlier quoted them twice. The conformance runner hit the same trap through the facade. One quoting point per path avoids it.
- `recognize`, `find` and `relate` keep their own states. The requote script skipped them by their state shape.
- The requote script needed three rules. It parses a string state that looks like JSON as the object the batcher would quote. It treats an instruction as quoted only when a JSON value and ". " follow "The text is ", because some questions start with those words. It merges recordings that collide once the string and structured forms of one record meet, as in demo 06.
- 468 recordings were requoted and each carries `"quoted": true`. Recordings under `site/examples` and `site/recordings` were requoted mechanically. Marketing owns the prose under `site/`; these fixtures changed only by the script.
- The two `portable-frame` batching fixtures became copies of `portable-1` and `portable-2`, so they were removed.
- The `--plan` hint now says each question quotes the evidence it asks about.
- Every surface fake that counted per-record arrivals had to read a one-record quoted request as that record. The fakes still log and compare the true body.
- The full test rung reaches two readers of the state that the shared cases miss: the consumer's annotate parts test and the find-0040 probe self-test. Both now read the quoted form.
- Surface checks not run for slice 1: Dart and the sqlite3 tool are absent, and the Python pandas 2 pin is not in uv's offline cache. The main Python suite passes.

### Slice 2

Where ADR 0111 was silent, the build took the simpler option:

- `cache convert` removes `thinkthen.sqlite` after it writes the fixture, because `--replay` refuses a folder holding both. It holds the file's write lock from read to removal. It must not run beside a live writer, because a process holding the file open keeps writing to the removed file.
- A cache on a folder that holds only the fixture imports it in the new file's schema transaction. A run that stores nothing creates nothing.
- Each path keeps its own quote form: an annotate root group quotes the record's compact text as a JSON string, and the record functions quote the object.
- Coalescing covers keys already on their way within one call. The first waiting row counts the send; a row that joined it shares the answer and counts no attempt.
- A cached answer's model counts toward the run's `model` fact but takes no part in the live model check, by ADR 0111 section 4.
- A stored answer that no longer decodes is a miss under a cache and names its entry under `--replay`.

What the build found:

- rusqlite sets a 5-second busy timeout on every connection. The store turns it off, so its own wait, which checks the stop, is the only one.
- `listener.requests()` drains its list, and a rerun test must reuse the same listener, because the address is part of every key.
- Running demo 12 in place committed a `thinkthen.sqlite`. The demo now caches in a scratch copy, and `policy.py` refuses the file.

### Slice 3a

What the build changed beyond the evidence list:

- An annotate record's groups share one state, so its questions pack into one request, by ADR 0111 section 5. The consumer's parts test and the shared case `18-annotate-two-groups` recorded one request per group. The consumer now reads every question of a request, and it skips case 18 as the command's wire run does.
- A 413 on a stopping stream now sends both halves, by ADR 0111 section 6. The old path stopped after the first half.
- `thinkthen decide --plan` and the public plan count real option counts through `pipeline::options`. The command plan passed 0 before.
- A single public call listed only its first question key in its details. A tag call has one key per label, and `public_json` caught the gap. `keyed` now passes every key through.
- The library now needs rusqlite, so every consumer lock gained it. The SQLite extension also pins rusqlite, so Cargo resolves one rusqlite with both crates' features. `policy.py` now exempts the packages beneath a crate that a binding pins to the root's exact version. Otherwise its lock check read the extension's own packages as drift in thinkthen's tree.
- That unification also joined `bundled` and `loadable_extension` in one `libsqlite3-sys` inside the SQLite extension. ADR 0111 expected two SQLite copies there. The review found the first 3a build broke the extension: the bindings came from the bundled 3.53.2 header, so the load refused the pinned 3.50.0 host, and every call aborted. `thinkthen` now has a `bundled-sqlite` feature, which every binding names, and a `host-sqlite` feature, which only the SQLite extension names; the library refuses to compile with neither, and `policy.py` enforces each binding's choice. The `polars` feature turns on `bundled-sqlite`, and the package and inventory scripts name it beside `--no-default-features`. The store then runs on the host's SQLite. ADR 0111's 2026-09-30 amendment records it. 3b refuses a single-thread host only when a cache, record or replay folder is named, and a refusal at load was rejected. The library also refuses to compile with both features on, because the store runs on one SQLite.
- ADR 0111 coalesces misses only within one call. The fork probe's digest-lock test therefore counts two sends for two concurrent single calls, and it keeps its proof that the parent's call never waits for a forked child.
- A cache on a folder that holds only `thinkthen.jsonl` imports it into `thinkthen.sqlite` beside it. The old folder binding refusal no longer applies to the record functions, because each key names its URL.
- The type corpus lost its three `meta.batch` and `meta.batches` shape cases with the schema definitions they checked.
- The ratchet fell from 108,545 to 105,563 lines, because the old batchers, their tests and the batch receipts went.

What the review fixes found:

- The Polars tests and the C door tests run outside `sdlc/scripts/test`, and both still pinned content cuts, request digests and case 18. They now read question keys, skip case 18 as the consumer does, and the portable fixture's five questions ride one request. The C door's settings case counts answer rows in `thinkthen.sqlite`.
- A call answered wholly from the store lost its `model` fact, because a stored answer told the process usage but not the call's facts. The C door's typed facts test caught it; the stored path now tells both.
- `policy.py`'s lock exemption skipped every package beneath a shared exact pin, so a changed `libsqlite3-sys` would have passed. It now skips only packages whose name the root tree never holds, and what only they reach. Planted locks prove a changed `libsqlite3-sys` or `cc` fails and a changed `hashlink` or an extension-only package passes. `hashlink` is absent from the root tree, so its version has nothing to drift from.
- The batch-1 read-ahead test now pins the real figure: no record read ahead of its rows, one read while the send is held.
- The throttle issue moved to `closed/` with a note that its batch-1 premise was wrong.
- The Python, TypeScript, R and Ruby binding checks pinned the same old shapes. Their portable, conformance, facts and settings tests now read question keys, one portable request and answer rows in the store. A repeated record in one packed call shares its question's answer and usage share, as a cached row does, so row usage can sum past the call's live tokens. The TypeScript key helper reads raw member bytes, because `JSON.parse` moves number-like keys first. R and the pinned Ruby read no SQLite without a new package, so their settings entry checks require the store and no old entry.
- Python's and TypeScript's cache-folder tests also met ADR 0113's usage totals under the cache home. Ticket 0326 fixed that on main, and the rebase kept its assertions.

What the second review found:

- The SQLite extension failed on every call, as above. Its surface check now passes on the pinned host.
- The surface checks over the C door and the SQL hosts still pinned the old counts, so they move in 3a rather than 3b. Each portable check reads one request through `conformance/children/portable.py`, which also holds the shared question key helper. The Go, PHP, C#, JVM, Swift and Zig matrices send one request fewer, because the repeated first/second bulk call reads the question cache the first/second/third call wrote. Every cached bulk replay now counts three cache answers, one per question. The C++ type corpus count fell to 52 with the removed `meta.batch` cases. PHP's omitted-usage proof asks its bulk call another question, so the scalar call's cache entry cannot answer it.
- The DuckDB check caught a real regression. A retry denied by the send budget ended the whole native vector, so a later request that had already answered lost its rows. Ticket 0240 keeps those rows. `details_many_recoverable_with` now fails only the denied request's rows; every later request is denied before it sends. `native_denied_retry_keeps_a_later_request_answered_first` failed before the fix.
- PostgreSQL's two replay-miss tests now expect the store's sentence, "the replay folder holds no answer for this question". Its annotate conformance calls annotate once per record, because ADR 0111 section 5 never stores a failed answer, so a second call resent the failed question.
- `policy.py` refused an engine reference to `public` but not an alias of the crate root, such as `use crate as c; c::public::Error`. It now refuses any alias of the crate root or an ancestor in `engine`, as it does in `core`, with three planted forms.

### Slice 3b

What the build changed beyond the evidence list:

- Slice 3a had already moved the C door and every SQL host onto the one pipeline. No host packer remained. The static library, the single-thread refusal, the store row counts and DuckDB's annotate path were the work.
- The old DuckDB annotate path already stored an answered neighbour at the default batch. A failing test therefore needed batch 1, a bad first row and a send count.
- DuckDB's `TRY()` refuses volatile functions, so a failed annotate row cannot come back as a null through `TRY`. The test reads the store instead.
- On the M5, `cc -r -Wl,-exported_symbols_list` wrote an LLVM bitcode object that Apple's `nm` could not read, because the Homebrew rustc embeds a newer LLVM. The macOS static library keeps its `sqlite3_` names for now.
- A surface check that takes the heavy lock itself deadlocks under a wrapper that already holds it, unless the wrapper sets `THINKTHEN_HEAVY_LOCK_HELD`.
- The PHP, Ada, Objective-C and COBOL checks wait 180 seconds for the heavy lock and ignore `THINKTHEN_HEAVY_LOCK_HELD`. Under a wrapper that holds the lock, they exit 1 or 75 with no test run. They pass when the wrapper leaves the lock to them.

What the review fixes found:

- `annotate_each_with` skipped the engine's record limit, so DuckDB's `thinkthen_annotate` ignored `SET thinkthen_max_requests`. It now refuses before any send, and a DuckDB case pins it.
- The R package's shared object exported 285 `sqlite3_` names. The ticket's slice split named R, and the first build dropped it. After the fix, the package's shared object exports one name, `R_init_thinkthen`, and the R check still passes.
- Lint's recursive-removal rule caught `localize.sh` and `host_sqlite.sh`. Both now use `scratch.sh`, or move a file in place. The release archive self-test packed fake bytes as the static library, so `localize.sh` refused it. The fixture now builds a one-function archive.

### Slice 3c

- The rows the door collected cost more than the columns it built. An annotate record carries its member names, labels and JSON text, so over 100,000 rows the old `annotate_frame` raised its peak by 77 MB and the new one by 15 MB. The decide rows were small, so streaming saved little there.
- The question key names the backend address. A replay under another base URL than the recording finds nothing, so a measurement must replay against the recorded address.
- A slice placed before the expression limits what it asks, so a sliced lazy frame asks only the rows it keeps. The test does not show whether Polars pushes a later slice below the expression. The streaming engine, which the feature no longer compiles, is the only way to bound the frame's own memory.
- Replay speed is the engine's: about 50 microseconds per answered question under load, the same with or without Polars.

### Slice 3d

- The prep note said the request total did not reset in a forked child. `SendBudget` already reset itself with its own spin loop. Reading the code before building found the duplicate, and the budget now reuses the shared fork-safe cell.
- An engine that saves a piece of process state when it is built keeps the parent's copy after a fork. The first build saved the total that way, and review caught it. A simulated fork cannot show the bug, because both engines are built as the child. The real-fork probe can, and its new test failed on that build.
- The surface checks take a loopback port as their argument. `check.sh 0` fails the Rust and Python checks falsely. `heavy-lock` re-runs its caller with the arguments left when it is sourced and only the environment names in `allow-list`, so a wrapper that shifts first or passes its own variables loses them.
- SQLite's `test_one_cached_call_leaves_no_thread` failed once under load 24 with fewer threads after the calls than before, then passed alone. The test counts the process's threads, which other finishing work can change.

### Slice 3e

- A host that passes its own budget beside the engine's would count each send twice, because every public engine already reserves against the process total. A call limit therefore joins the engine's limit in one reservation, the tighter of the two.
- SQLite needed no change. Its `max_requests_total` already went through the builder, so only PostgreSQL and DuckDB held copies.
- DuckDB kept two counts of the same thing: `SEND_BUDGET` for sends and the engines' usage totals for the row cut. Both now read the engine's one total.
- The held arm keeps its reply until release, so a PostgreSQL cancel check that releases after the wait already proves order. Only the latency needed the wall clock.
- Running the fork probe from its own folder left `conformance/consumer/target`, and `policy.py` then counted generated bindings against the 500-line cap. The test rung builds it under `target/consumer`.
- The review's first runs shared the machine with each other. PostgreSQL's `find_proxy_cases` failed once on a refused proxy connection, and DuckDB's `sixteen_held_plans_refuse_without_eviction` failed once and then passed 20 of 20 alone. Neither touches the request total. Both passed in full when run alone.
- After rebasing onto 0314 slice 4a, one PostgreSQL run at load 22 failed `batch_cancel` and `a_small_batch_answers_at_once` on their wall-clock limits, and the next run passed all 88 steps. `sdlc/issues/2026-09-30-postgresql-check-keeps-wall-clock-limits-under-load.md` owns the other timing limits.

### Slice 4

- A key names the backend address, so a partial-reply rerun must ask the same loopback listener. A new listener asks every question again.
- Recognize steps 1 and 2 never obeyed the backend request ceiling. Packing them under it split a 600,000-byte text into 60 requests instead of 2, so `Packing` now says whether a request closes at the ceiling.
- The old chunked sender returned `Cancelled` after a stop only when it ran several requests on workers. One sent request finished and printed. `ask_each` keeps both, because the command's interrupt test and the public relate interrupt test pin each side.
- The old pipeline kept sending after a whole request failed. One job stopped at once. A call that does not continue now stops sending too. A host stops when it reads the failed row in order, so the old count grew only while an earlier request was still out. The record functions had the same gap, and review asked for its test.
- Relate requests arrive in any order at width 2, so a count proof sorts them.
- A replay fixture loaded per call costs nothing for one call over many records, but `recognize` makes one call per line and step. The spec page's 30-second limit caught it.
- Review found a removed test module's `cfg(target_os = "linux")` left in place. It then gated the next module, so the cache trust tests stopped compiling on macOS.
