# Ticket 0304 slices 3b, 3c and 3d: preparation

Status: preparation for builders, 2026-09-30. Read against `origin/ticket/0304-s3a-engine-ask-all` at `4d0336943`, which is still changing. Lane 2 owns that branch and ticket 0304. This page changes neither. Line numbers below are from the 3a branch unless a line says `main`. Ian can overturn each recommendation.

Read first: `sdlc/planning/cleanup-2026-09-30.md` "Lessons for builders", ADR 0111 with its 2026-09-30 amendment, and ticket 0304 "Slice 3 split" on the 3a branch.

## What 3a already did that the ticket gave to 3b

The 3a branch now repoints every host test that pinned content cuts, request digests or `*.json` entry counts. Commits `0761f5f02` and `4d0336943` cover the C door, the three SQL hosts and all eleven native hosts. The same branch splits the store's SQLite into two features. `bundled-sqlite` bundles SQLite, and `host-sqlite` bundles nothing (`crates/thinkthen/Cargo.toml` features; `crates/thinkthen/src/lib.rs:13-17` refuses a build with neither). Every binding names `bundled-sqlite` except the SQLite extension, which names `host-sqlite` (`databases/sqlite/Cargo.toml:23`). So 3b starts with green host checks, or 3a does not land. 3b must rerun them all after its own changes. If 3a drops any of these repoints before landing, 3b inherits them.

## 3b: the C door and the SQL hosts

### Batch call sites

No host holds its own packer on the 3a branch. Each calls a public `*_with` method, which runs `Engine::ask_all`.

| Surface | Batch call sites |
| --- | --- |
| C door | `libraries/c/src/call.rs:248` (filter), `:261` (rank), `:320` (annotate); `libraries/c/src/door.rs:128,139` (`decide_many`, from `ffi.rs:396`); `libraries/c/src/call/records.rs:40` (`details_many_with`) |
| SQLite | `databases/sqlite/src/many.rs:274-275` (the `*_many` tables); `databases/sqlite/src/scalars.rs:64,67,312` (one record per call) |
| DuckDB | `databases/duckdb/bridge/src/ffi/scalar/ffi.rs:108,111` (`details_many_recoverable_with`), `:224,259,263,284,298`; `ffi/listed.rs:74-77` (annotate); `ffi/portable_many/ffi.rs:225,228,242`; `ffi/complete_listed/ffi.rs:23`; `ffi/portable/ffi.rs:163,166`; `warm/ffi.rs:66,69` |
| PostgreSQL | `databases/postgresql/src/keyed.rs:25,28` and its `decide_many` (`:52`), `choose_many` (`:92`), `score_many` (`:122`) and `tag_many` (`:145`); `src/lib.rs:44,47,90` (one record per call) |

The eleven native hosts call only C exports. Their batch path is `thinkthen_decide_many*` and `thinkthen_call*` with a filter, rank or annotate request. Their host source needs no change for 3b.

| Host | Calls the batch exports | Tests that pin counts or shapes (repointed by 3a) | Check |
| --- | --- | --- | --- |
| C# | `libraries/csharp/src/ThinkThen.cs:38-40,180,204` | `tests/run_matrix.py`, `tests/source/Program.cs` | `sh libraries/csharp/check.sh 0` |
| JVM | `libraries/jvm/door/thinkthen/Door.java:52-54,168,184` | `tests/Matrix.java`, `tests/consumer-run.py`, `tests/installed.py` | `sh libraries/jvm/check.sh 0` |
| Swift | `libraries/swift/Sources/ThinkThen/ThinkThen.swift:193,209` | `Tests/fixtures/portable_batch.py`, `run_matrix.py`, `matrix.swift`; `expected_requests.json` deleted | `sh libraries/swift/check.sh 0` |
| Go | `libraries/go/thinkthen.go:262,311,343` | `portable_batch_test.go`, `fixtures/portable_batch.py`, `fixtures/run_matrix.py`; `accepted_requests.jsonl` and `expected_arrivals.json` trimmed | `sh libraries/go/check.sh 0` |
| C++ | `libraries/cpp/include/thinkthen/door.hpp:130,145` | `fixtures/portable_batch.py`, `fixtures/portable_consumer.cpp`, `fixtures/tests/consumer.cpp` | `sh libraries/cpp/check.sh 0` |
| PHP | `libraries/php/src/ThinkThen.php:25,27,141,150` | `fixtures/portable_batch.py`, `run_matrix.py`, `matrix.php`, `portable_consumer.php`, `accepted_requests.jsonl` | `sh libraries/php/check.sh 0` |
| Ada | `libraries/ada/src/thinkthen_c.ads:34-38` | `checks/portable_batch.py`, `portable_batch.adb`, `package_bulk.adb` | `sh libraries/ada/check.sh 0` |
| Objective-C | `libraries/objective-c/Sources/ThinkThen.m:110,119` | `checks/portable_batch.py`, `portable_batch.m`, `matrix.m` | `sh libraries/objective-c/check.sh 0` |
| COBOL | `libraries/cobol/src/tt_call.cob:42` | `checks/portable_batch.py` | `sh libraries/cobol/check.sh 0` |
| Zig | `libraries/zig/src/thinkthen.zig:153,164` | `Tests/portable_batch.py`, `run_matrix.py`, `matrix.zig`, `installed.py`; `expected_requests.json` deleted | `sh libraries/zig/check.sh 0` |
| Dart | `libraries/dart/lib/src/door.dart:142,160,283,288` | `checks/portable_batch.py`, `checks/consumers/alpha/bin/portable_batch.dart` | `sh libraries/dart/check.sh 0` |
| C door | as above | `libraries/c/tests/door/batching.rs`, `cases.rs`, `main.rs`, `portable.rs`, `settings.rs` | `sh libraries/c/check.sh 0` |
| SQLite | as above | `databases/sqlite/tests/conformance.py`, `test_portable_batch.py`, `test_redesign.py`, `test_settings.py` | `sh databases/sqlite/check.sh 0` |
| DuckDB | as above | `databases/duckdb/tools/conformance.py`, `settings_cases.py`, `settings_suite.py`, `verbs_portable.py`, `verbs_suite.py` | `sh databases/duckdb/check.sh 0` |
| PostgreSQL | as above | `databases/postgresql/tests/batching_cases.py`, `portable_batch.py`, `runner.py`, `settings_cases.py`, `check.sh` | `sh databases/postgresql/check.sh 0` |

Run each check from the repository root under the lane's own lock: `THINKTHEN_HEAVY_LOCK=<lane lock> sh <surface>/check.sh 0`. `sdlc/scripts/surfaces` runs them all the same way (`sdlc/scripts/surfaces:231`). A check that prints "not run" is not a pass.

### What 3b changes

1. **The SQLite store on the host's SQLite.** See the options below.
2. **The static libraries' SQLite symbols.** See the section after the options.
3. **An SQL row with a missing pointer fails alone** (ADR 0111 section 4 and build step 3). The one-record SQL paths already fail one row at a time. The batch paths are the question. DuckDB's listed annotate collects the batch as one `Result` and fails the whole batch on the first bad row (`databases/duckdb/bridge/src/ffi/listed.rs:74-77`). The recoverable detail path already keeps per-row failures (`crates/thinkthen/src/public/native_batch.rs:103-133`), but it takes no `on` pointer. 3b needs a public way to continue past an annotate row's `ItemError`. The pipeline has a `Flow::Continue` for hosts, by ADR 0111 section 4, but the 3a public API may not expose it. Check the landed 3a API first.
4. **The SQL host statics.** Ticket 0304 lists `ACTIVE_THROTTLE`, `SEND_BUDGET`, the engine maps, SQLite's `STORED` and `ENGINE`, and SQLite's budget registry. They are three different kinds:
   - Mirrors of engine send limits: `databases/postgresql/src/call.rs:147` `ACTIVE_THROTTLE`, which `call/settings.rs:223-230` reads to refuse a second throttle; `call.rs:152` `SEND_BUDGET` (used at `:342`); and `databases/duckdb/src/engines.rs:62` `SEND_BUDGET` (used at `:70`). These are send limits, so give them to 3d. See "Order and collisions".
   - Host state with no other home: `databases/postgresql/src/call.rs:151` `ENGINES` and `databases/duckdb/src/engines.rs:138` `ENGINES`, which ADR 0113's flush at exit needs; `databases/sqlite/src/settings.rs:75` `STORED` and `:90` `ENGINE`, the process settings that `thinkthen_configure` sets; and `databases/sqlite/src/question.rs:182,184`, the parsed-question caches.
   - `databases/sqlite/src/budget.rs:19` `REGISTRY` is a per-connection deadline, keyed by connection handle. It is not a send limit.

   Recommendation: 3b keeps the host state and the deadline registry, as 3d's option 1 keeps process state. It removes a static only where it copies engine state. The 3b ticket review confirms the list.
5. **Conformance on the store.** Every SQL and C conformance case runs with a scratch cache folder, and the checks count answer rows in `thinkthen.sqlite`, as `libraries/c/tests/door/settings.rs` does on the 3a branch.

### The SQLite extension's store: options

The facts:

- `libsqlite3-sys` declares `links = "sqlite3"`, so a Cargo build graph can hold only one copy. Cargo also unifies its features across the graph. The extension needs `loadable_extension` (`databases/sqlite/Cargo.toml:25`). With that feature, every `sqlite3_*` call goes through the host's API table, which `Connection::extension_init2` stores (`databases/sqlite/src/ffi.rs:179-196`). The abort that the 3a re-review found came from `bundled` and `loadable_extension` unifying. 3a's two features fix it.
- The extension already refuses hosts older than 3.50.0 at load (`databases/sqlite/src/ffi.rs:40-55,135-141`). The store's `ON CONFLICT DO UPDATE` (`crates/thinkthen/src/engine/store.rs:360`) needs 3.24. So no new version check is needed.
- The store opens a connection only when `cache`, `record` or `replay` names a folder (`crates/thinkthen/src/engine/pipeline.rs:227-238`, `store.rs:115-141`). Since 0318, the SQL extensions name no folder by default.
- rusqlite already refuses a single-thread SQLite when it opens a connection: `ensure_safe_sqlite_threading_mode` checks `sqlite3_threadsafe()` and the single-thread mutex marker (rusqlite 0.40.2 `src/inner_connection.rs:73,423-450`). The store would then fail with `Error::RecordingStorage` (`store.rs:232`), whose sentence does not name the cause.

| Option | Cost | Gains | Risks |
| --- | --- | --- | --- |
| A. Host SQLite as the store, which the 3a branch already does. Add a named refusal for a single-thread host, but only when a folder is named | Small: one check at `thinkthen_configure` and one fixed sentence | One SQLite in the process, as SQLite's extension rules expect. No second copy, and no size cost | Host builds vary in compile options, heap limits and default VFS. The store's edge tests run only on bundled 3.53.2 |
| B. A private bundled copy for the store inside the extension | Large. A separate store crate does not help alone, because the `links` rule still allows one `libsqlite3-sys`. The build needs either a second -sys crate with renamed symbols and its own `links` key, or an extension with no rusqlite, which means hand-written FFI for every virtual table and function in `ffi.rs` | The store does not depend on the host | A second SQLite in one process. About 1 MB more. Much new unsafe code, against ADR 0047 item 3 |
| C. No store in the extension: `host-sqlite` compiles the store out, and the extension refuses `cache`, `record` and `replay` | Small | The simplest code | Drops ruling 2's one cache for the SQLite surface. Breaks `databases/sqlite/tests/test_settings.py:22,37`, which already use the store |
| A-strict. As A, but refuse any single-thread host at load, as ticket 0304's 3b text says | Small | The failure comes early | Refuses hosts that never name a folder. Those hosts need no background SQLite calls |

Recommendation: A. Put the thread-mode check in `thinkthen_configure` when it names a `cache`, `record` or `replay` folder, with its own fixed sentence. Keep rusqlite's check at open as the backstop. The existing 3.50.0 load floor covers the version. Proof: build a single-thread host from the pinned 3.50.0 amalgamation with `-DSQLITE_THREADSAFE=0` (`databases/sqlite/tests/host_sqlite.sh` builds the multi-thread one). On that host, a call with no folder answers, and a configure that names a folder is refused with the exact sentence and sends nothing, counted on loopback. Also run the store's edge rows (hit, miss, read-only replay, busy wait) through the extension on the 3.50.0 host, because `engine/store/tests.rs` runs them only on bundled 3.53.2. Record A-strict's rejection in ADR 0111's amendment, because ticket 0304's 3b text names A-strict.

### The static libraries' SQLite symbols

`libthinkthen.a` exports about 285 global `sqlite3_` symbols. The test `libraries/c/tests/door/main.rs:249` `the_static_library_still_exports_sqlite_symbols_until_slice_3b` pins the leak. `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md` asks for a partial link that localizes every symbol except the header's `thinkthen_` names. Neither hidden visibility nor compile-time flags help, because a hidden global in an archive still takes part in a static link.

Where the archive leaves the build:

- `sdlc/scripts/release-pack:136` copies `libthinkthen_c.a` to `lib/libthinkthen.a`.
- `libraries/go/check.sh:83`, `libraries/cpp/check.sh:70,78` and `libraries/zig/check.sh:57` copy or compare Cargo's output directly. `libraries/swift/check.sh:35` and `libraries/zig/check.sh:36` compare the artifact's archive.

So one helper must produce the localized archive, and every one of those places must use it. Otherwise the checks test an archive that users never get. Keep the helper beside the C door, run it from `libraries/c/check.sh` and `release-pack`, and point the four consumer checks at its output. On Linux: `ld -r` over the archive's members, then `objcopy --keep-global-symbol` for each `thinkthen_` export, then `ar`. This also hides Rust's own runtime symbols, which clash when a program links two Rust static libraries. macOS needs a different command (`ld -r -exported_symbols_list`), and the M5 lane must prove it. Then flip the test to require zero and move the issue to `closed/`.

Two related leaks:

- R links `libthinkthen_r.a` into the package's shared object and then deletes the archive (`libraries/r/thinkthen/src/Makevars`). So R's exposure is the package's dynamic exports, not an archive. Adding `-Wl,--exclude-libs,ALL` to `PKG_LIBS` on Linux may be enough. Check it with `nm -D`.
- DuckDB passes `-Wl,--exclude-libs,ALL` on Linux only (`databases/duckdb/cpp/CMakeLists.txt:69-70`). Since 3a, the bridge bundles SQLite, so the macOS extension may export `sqlite3_` symbols. Check with `nm -gU` on M5.

## 3c: Polars eager and lazy

On the 3a branch, the Polars door has no packer of its own. `crates/thinkthen/src/public/frame.rs` calls `details_many_with` (`:297`), `decide_many_with` (`:341`) and `annotate_with` (`:408,464`), and `completed` (`:475-476`) collects one call's rows. 3a changes only the tests (`crates/thinkthen/tests/polars/batching.rs`, `cases.rs`). The source under `public/frame*` is untouched.

What 3c changes:

1. **Proof of a lazy frame collected twice.** Collect the lazy frame once, then again; the second collect sends nothing, counted on loopback. `tests/polars/lazy.rs:44-49` builds its engine with `no_cache()`, so the new test needs `cache_at` on a scratch folder. Add it to `lazy.rs` or a new file under `tests/polars/`, and keep every file under 500 lines.
2. **Remove frame-only batching.** None is left on the 3a branch. 3c confirms this and says so in the ticket.
3. **Lazy frames and low-memory batching (Ian's subject 4).**
   - The lazy expression uses `Expr::map` (`public/frame/lazy.rs:189-192`). In Polars 0.55.2, `map` marks the function elementwise and optionally re-entrant (`polars-plan-0.55.2/src/dsl/mod.rs:490-491`). So the engine may split a column into morsels and call the function once per morsel, possibly on several threads at once. Each morsel is one `ask_all` call with its own store connection. The process throttle bounds the sends. SQLite writes serialize through `BEGIN IMMEDIATE` with the 30-second busy limit.
   - Memory per morsel: `nullable` (`public/frame/column.rs:10`) borrows each cell into a `Vec<Option<&str>>`, the pipeline holds at most W unemitted inputs (ADR 0111 section 4), and `completed` holds one row per cell until the column is built. A call's memory therefore grows with the morsel, not the frame. The in-memory engine treats the whole column as one morsel.
   - 0307 dropped `polars/streaming` from the feature, so the streaming engine is the user's opt-in. The checks cannot compile it. A chunked column test already exists (`tests/polars/door.rs:278` `a_chunked_and_sliced_column_answers_each_row`). 3c can add a test that evaluates the expression twice on slices of one frame and checks that the second slice sends only its new questions.
   - Open question for the 3c ticket: should `completed` stream rows into the output builder instead of collecting a `Vec` first? That would cut peak memory per morsel by one copy. Measure it before building.
4. **Check:** `sh libraries/polars/check.sh 0`. It runs clippy with and without default features, `cargo test --features polars --test 'polars_*'`, and the doc tests. The stress profile adds `polars_throttle two_hundred_series_records_match_the_slice`.

## 3d: one owner for the send limits

Decision: keep one throttle per process (ticket 0304 "Slice 3 split", option 1, from Ian's ticket 0077 ruling). Ian can overturn it.

Where the limits live now:

| Limit | State | Readers |
| --- | --- | --- |
| Throttle (width) | `crates/thinkthen/src/engine/mod.rs:374-430` `Widths`; `:493` `PROCESS_WIDTH`; `:496` `REBUILD_POLL` (1 ms); `:499-504` `process_width`, which sleeps 1 ms in its loop; `:508` `process_width_of`; `:520-524` `rebuild_wait`; `:529` `WIDTH_CHILD` (tests only); `:532` `client_width`; the refusal sentence at `:364` | `cli/schedule.rs:283`, `engine/http.rs:170`, `engine/facade.rs:199` |
| 429 gate and requests-per-minute pacer | `engine/backoff.rs:12` `Gates`, `:24` `per_minute`, `:36` `interval`, `:80` `pace`, `:275` `PROCESS_GATES`, `:278` `process_gates` | `engine/http.rs:258` |
| Process request total | `engine/budget.rs:135-138` `process_budget` (a `OnceLock`); `SendBudget::reserve` at `:99`, `reserve_estimated` at `:67` | `engine/facade.rs:129-135`, `cli/asking.rs:99`, `engine/request.rs:113`, `engine/pipeline/send.rs:114`, `public/engine.rs:150` |
| Estimated input token cap | `engine/send_budget.rs:37` (`THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL`), `:59` `with_process_budget`, `:75` `reserve_send` | `public/settings/environment.rs:62`, `cli/edge.rs:128` |
| SQL host copies | `databases/postgresql/src/call.rs:147` `ACTIVE_THROTTLE` and `:152` `SEND_BUDGET`; `databases/duckdb/src/engines.rs:62` `SEND_BUDGET` | Their own calls |
| Default engine | `public/mod.rs:73-74` | Deferred to ticket 0314's binding pass by 3a |

`PROCESS_WIDTH` and `PROCESS_GATES` share the fork-safe `engine/process.rs` `Guarded` cell. `process_budget` uses a plain `OnceLock`, so a forked child inherits its parent's count.

What moves:

1. Replace the 1 ms `REBUILD_POLL` with the pipeline's 50 ms stop check, as the ticket says. `process_width` (`mod.rs:499-504`) has no cancel token and sleeps in a loop. Give its caller, `cli/schedule.rs:283`, a token, or let it wait on the rebuild without polling.
2. Put the width state, the gates and the pacer, and the request total behind one process owner, such as one `Guarded` value in a new `engine/limits.rs`. Then one fork rebuild covers all three, and the request total gains the fork rule that the others have. Whether the total should reset in a forked child is an open question for the 3d ticket. Today it does not.
3. The SQL hosts drop their copies. PostgreSQL's early refusal (`call/settings.rs:223-230`) repeats the engine's sentence (`engine/mod.rs:364`). The engine's own refusal at build can replace it, if the PostgreSQL test pins the same sentence and exit. The two `SEND_BUDGET`s give way to the process total. That needs a public way to reach the process total, which is a 3d API choice.
4. Do not touch the 50 ms host polls that ticket 0323 keeps (0323 table: `engine/workers.rs`, `engine/backoff.rs:171`, `databases/sqlite/src/worker.rs:123`, `databases/postgresql/src/call.rs:267`).

Tests that pin these limits: `crates/thinkthen/src/cli/schedule/width_tests.rs` (`:145`, `:299` `every_live_path_and_every_engine_share_one_cap`); `crates/thinkthen/tests/public_env.rs` and `public_env/cache_budget.rs`; `engine/backoff.rs:198` `paced_starts_from_many_threads_keep_one_interval_apart`; `engine/process/tests.rs`; `conformance/consumer/fork-probe/tests/fork.rs`; the "already active for this process" sentence in `libraries/python/tests/test_engine.py`, `libraries/typescript/tests/settings.test.mjs`, `databases/postgresql/check.sh`, `databases/duckdb/tools/settings_suite.py` and `relate_suite.py`.

Checks: `sdlc/scripts/test`, workspace clippy, `policy.py`, the fork probe, and the Python, TypeScript, PostgreSQL and DuckDB checks.

## Order and collisions

Files each slice touches:

- 3b: `libraries/c/**` and a new archive helper, `sdlc/scripts/release-pack`, the static-archive lines of the Go, C++, Zig and Swift checks, `libraries/r/thinkthen/src/Makevars*`, `databases/sqlite/src/{ffi,settings}.rs`, `databases/duckdb/bridge/src/ffi/listed.rs`, `databases/duckdb/cpp/CMakeLists.txt`, the SQL test folders, and possibly `crates/thinkthen/src/public/native_batch.rs` for per-row annotate.
- 3c: `crates/thinkthen/src/public/frame*.rs`, `crates/thinkthen/tests/polars/*`, `libraries/polars/README.md`.
- 3d, part 1: `crates/thinkthen/src/engine/{mod,backoff,budget,send_budget,process,http,facade}.rs`, `cli/schedule.rs`, `cli/asking.rs`, `public/engine.rs`, their tests.
- 3d, part 2: `databases/postgresql/src/call.rs`, `call/settings.rs`, `databases/duckdb/src/engines.rs`.

Plan:

- 3b, 3c and 3d part 1 run at once in three lanes. Their code files do not overlap.
- 3d part 2 waits for 3b to land, because both reach the SQL host call files and 3b's reruns of the SQL checks would go stale.
- Every slice changes `sdlc/ratchet.json`, `CHANGELOG.md` and ticket 0304's evidence, so landings go one at a time. Each lander rebases, measures the ratchet again, and reruns its checks.
- If 3b needs a public per-row annotate API, it touches `public/`, but not `public/engine.rs`. Say so in its ticket, so 3d's lane knows.

## Retained regressions (CLAUDE.md Proof)

Each slice keeps these and names any test it merges or deletes.

- **All slices:** key secrecy on every command, failure path and `Debug` line; "sends nothing" proved by counting loopback requests; exact sentences, row counts and exit codes; no key read on replay; the null-versus-failure rule.
- **3b:** the C door's header-symbols and soname test (`libraries/c/tests/door/main.rs:193`), with the static symbol test flipped to a zero gate (`:249`); `settings.rs:307` (invalid settings); `cases.rs:151` (a refused key is not retryable); the SQLite 3.49.0 load refusal and its exact sentence (`ffi.rs:46-55`, with the load probes from `host_sqlite.sh`); `databases/sqlite/tests/test_interrupt.py` (cancellation, including `test_a_keyed_call_is_cancelled_mid_batch`); `test_error_format.py`; `test_files.py` (invalid input); the SQL cache-folder rules from 0318; DuckDB `portable_suite.py:202` (recoverable failures beside an answer) and `signal_suite.py` (cancellation); the PostgreSQL panic-text regression from 0310; the per-row budget denial of ticket 0240 (`public/native_batch.rs:94-97`).
- **3c:** `tests/polars/door.rs:77` `every_refusal_is_pinned_and_sends_nothing`; `:236` `a_failed_row_ends_a_series_call`; `lazy.rs:282` `a_shared_token_stops_a_later_evaluation_without_a_send`; `deadline.rs`; `lazy.rs:107` `lazy_matches_eager_values_bodies_and_shared_tally`; the panic containment test in `public/frame/lazy.rs` (`a_panic_stays_inside_the_polars_compute_boundary`); `throttle_equality.rs`.
- **3d:** the throttle conflict sentence and its refusal on every surface that pins it; `width_tests.rs:299` (one cap across every path and engine); the fork probe; `process/tests.rs` (`racing_children_publish_one_state`, `a_failed_build_leaves_the_next_call_to_build`); a stop during a rebuild wait; the pacer interval test; the send budget and token cap denials (0311).

## What depends on 3a details that may still change

- The feature names `bundled-sqlite` and `host-sqlite`, and `policy.py`'s enforcement of them.
- Whether 3a keeps every host repoint listed above.
- The name of the static symbol test and the issue file.
- Whether the public API exposes a way to continue past a failed row (`Flow::Continue`) for annotate.
- `SendBudget`'s move to `crates/thinkthen/src/engine/budget.rs` and its re-export from `public`.
- The ADR 0111 amendment text, which names A-strict for 3b.
- All line numbers on this page.
