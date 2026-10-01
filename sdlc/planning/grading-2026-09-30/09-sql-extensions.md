# Area 9: SQL extensions

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Three loadable extensions put the engine's verbs into SQL: SQLite (a Rust extension on the host's SQLite), DuckDB (a Rust library under a C++ extension over a C bridge) and PostgreSQL (a pgrx extension). Each calls the public Rust API, keeps its own settings surface, and has its own guard against a panic.

Paths are under `databases/` unless they start with `sdlc/` or `specification/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, SQLite | `sqlite/src/` (2,838; largest `scalars.rs` 435, `tables.rs` 404, `many.rs` 330), `sqlite/build.rs`, `sqlite/setup.sh` |
| Code, DuckDB | `duckdb/src/` (934), `duckdb/bridge/src/` (3,273; largest `ffi.rs` 473, `ffi/scalar/ffi.rs` 459), `duckdb/cpp/src/` (2,198 C++; `portable.cpp` 434), `duckdb/cpp/build.sh` and `CMakeLists.txt` |
| Code, PostgreSQL | `postgresql/src/` (3,052; `call.rs` 451, `call/settings.rs` 409, `files.rs` 404, `ffi.rs` 329) |
| Tests | SQLite: 81 Python tests in 19 files (2,319 lines), 23 Rust unit tests across the three hosts. DuckDB: 3,793 lines of Python suites under `duckdb/tools/` plus `duckdb/proof/0201`. PostgreSQL: 6 Python files (657) and `postgresql/check.sh`, 1,317 lines holding 105 shell test functions |
| Ratchets | SQLite 2,838 Rust, 2,319 Python, 22 SQL. DuckDB 4,207 Rust, 1,752 C++, 3,977 Python. PostgreSQL 3,052 Rust, 657 Python. The 1,317-line shell check is under no ceiling |
| Contract | `specification/settings.md`; `sdlc/planning/databases/{README,sqlite,duckdb,postgres}.md`; ADRs 0038, 0043, 0047, 0080, 0081, 0096, 0105, 0111 (2026-09-30 amendment: `host-sqlite` versus `bundled-sqlite`, ADR line 266), 0113; tickets 0304, 0310, 0318, 0336 |

## Complexity: 5 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 5 | About 11,850 nonblank lines: SQLite 2,838, DuckDB 6,405 (Rust 4,207 plus C++ 2,198), PostgreSQL 3,052 |
| States and concurrency | 5 | Detachable worker threads with a 50 ms interrupt poll and a library kept mapped until exit (SQLite, `sqlite/README.md` "Runtime boundaries"), a SIGINT handler (`duckdb/src/signal.rs`), a process-wide engine list behind a mutex (`postgresql/src/call.rs:147`), one PostgreSQL backend process per connection with panics caught at the boundary (`postgresql/src/call.rs:115`), and a question store opened on a background thread, which a single-thread SQLite forbids (`sqlite/src/settings.rs:31-46`) |
| Rules and refusals | 5 | Well over 50: six error kinds, the `thinkthen <kind>: <message> (retryable: yes\|no)` shape, 2 to 255 find units (254 with `none`), 255 relate pairs, the 1 MiB file cap and 16 MiB find cap, privilege gates, file confinement, removed-call migration messages, and a fixed refusal for each schema position (CHECK, index, generated column) |
| Surfaces touched | 2 | 3 of 22, the three SQL extensions |
| Settings | 5 | SQLite `thinkthen_configure` reads twelve keys (`model`, `batch`, `cache`, `throttle`, `timeout`, `max_retries`, `max_request_bytes`, `max_requests`, `max_requests_total`, `profile`, `record`, `replay`). PostgreSQL has the same settings as `thinkthen.*` names, plus `thinkthen.file_directory` (`postgresql/README.md:76-87`) |
| Contract weight | 5 | 9 ADRs, 4 planning pages, `settings.md` and the shared result and find pages, at least 14 in all |
| Churn and debt | 5 | 335 commits on the three folders since 2026-09-23 (SQLite 138, DuckDB 209, PostgreSQL 121, with overlap), about 37 of them fixes. Four open issues: store proofs (Debt 010), the macOS DuckDB export names (Debt 026), the HTTP/1.0 reuse flake (Debt 020), and the release host setup skipping the bridge crates |

Mean 4.6, rounded to 5.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | Primary behavior matches the contract. The single-thread SQLite refusal exists with its own sentence and fires only when a folder is named (`sqlite/src/settings.rs:31-46`, tested by `sqlite/tests/test_thread_mode.py`). The PostgreSQL panic path is caught, freed without running the payload's destructor, and raised as the defect text with no payload in the log (`postgresql/src/call.rs:105-123`, pinned by `postgresql/check.sh:1208-1218` and a unit test at `postgresql/src/call.rs:441`). The shipped package must not hold the probe function (`postgresql/check.sh:124-133`). Named-file reads are confined with `openat2` on Linux, one message for every unreadable cause, and a 1 MiB regular-file rule (`postgresql/src/files.rs:1-60`, `postgresql/src/ffi.rs:235-250`). Drift is in the builder notes: all three `NOTES.md` still say no SQL find function exists (`sqlite/NOTES.md`, `duckdb/NOTES.md`, `postgresql/NOTES.md`) while each README lists `thinkthen_find` (`postgresql/README.md:18`, `duckdb/README.md:12`, `sqlite/README.md` calls table), and two of them say `18-annotate-two-groups` runs in SQL while the runners skip it (`databases/postgresql/tests/runner.py:31`, `databases/sqlite/tests/conformance.py:32`). SQLite's file door follows symlinks while PostgreSQL refuses them (`sqlite/README.md` "Runtime boundaries" versus `postgresql/README.md:138`); both are documented, but the difference is a contract choice nobody recorded as a choice |
| Reliability | C | Failure paths are counted: the PostgreSQL suite counts backend requests per case (`bcount`, `postgresql/check.sh:1205`), SQLite counts sends per test, and the store tests cover a denied retry and a missing annotate part (`an_annotate_row_missing_its_part_fails_alone`). The problems are stability and proof. About 37 fix commits landed in 7 days. The HTTP/1.0 reuse flake failed one DuckDB case 11 of 200 times at load 16 to 20, and only `duckdb/tools/verbs_budget.py:53` works around it (`sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`). The DuckDB and PostgreSQL runners do not count rows in `thinkthen.sqlite`, and no test covers a read-only replay folder or a busy wait on the SQLite host (`sdlc/issues/2026-09-30-sql-host-store-proofs-are-partial.md`). `duckdb/cpp/CMakeLists.txt:69-70` hides the bundled SQLite names on Linux only, so a macOS extension may export `sqlite3_` names (`sdlc/issues/2026-09-30-duckdb-macos-extension-may-export-sqlite-names.md`, unconfirmed until a macOS build is inspected) |
| Maintainability | C | The same engine logic is copied across hosts, which ticket 0347 is about to pay (Debt 018). PostgreSQL copies `inline_rule` (`postgresql/src/relate.rs:35`, from `crates/thinkthen/src/core/relate_file.rs:300`) and keeps its own list of engines (`postgresql/src/call.rs:147`). The 1 MiB file cap is written twice (`sqlite/src/question.rs:19`, `postgresql/src/files.rs:13`) and the 16 MiB find cap twice (`sqlite/src/scalars/find.rs:10`, `duckdb/bridge/src/ffi/find.rs:8`). `postgresql/check.sh` is 1,317 lines with 105 functions and no ceiling, because the 500-line cap and the ratchets count Rust and Python only. The three hosts carry 33 `allow` or `expect` attributes (measured with `rg`; reasons not all read). DuckDB is three layers (Rust, a C bridge, C++). Debt is filed and owned (tickets 0347 and 0348) |

## Strengths

- The SQLite extension runs its question store on the host's SQLite and refuses a named folder on a single-thread host with a dedicated sentence, leaving no-folder use untouched (`sqlite/src/settings.rs:31-46`, ADR 0111 amendment).
- PostgreSQL's panic guard separates pgrx's own errors from Rust panics, forgets the payload instead of running its destructor, and raises one fixed defect message (`postgresql/src/call.rs:105-123`).
- File confinement is judged before any open, opens beneath the folder with `openat2`, and gives one message for every unreadable cause (`postgresql/src/files.rs:1-60`).
- The SQLite extension registers volatile, direct-only functions, so an untrusted schema cannot spend requests or read files; each schema position has a pinned refusal sentence (`sqlite/README.md` opening, `sqlite/NOTES.md` "Schema messages").
- Every suite starts its own loopback backend and counts sends, with a fake key and loopback address only (`sqlite/NOTES.md` "The shared port", `postgresql/NOTES.md` "How the check runs").

## Cleanup

1. **Count stored answer rows in the DuckDB and PostgreSQL runners, and add SQLite's read-only replay and busy-store tests.** Where: `duckdb/tools/harness.py`, `duckdb/tools/conformance.py`, `postgresql/tests/runner.py`, `sqlite/tests/`. Why: Debt 010 says a store regression in either host has no counting check. Ticket 0348 is in flight. Size: M. Blocks 0.1: yes (the issue says "before 0.1").
2. **Inspect the macOS DuckDB extension for `sqlite3_` names and add the exported-symbol list.** Where: `duckdb/cpp/CMakeLists.txt:69-70`, `duckdb/cpp/build.sh`. Why: only Linux hides the bundled SQLite's names, and a macOS extension could bind to another extension's SQLite. The next rehearsal run should confirm it, but the release host setup bug (area 10, item 1) keeps the job from reaching the build. Size: M. Blocks 0.1: yes (severity medium, "pay before 0.1 if the rehearsal shows a name").
3. **Replace the copied engine code with public API calls.** Where: `postgresql/src/relate.rs:35`, `postgresql/src/call.rs:147`, the plan writers in `sqlite/src/scalars/plan.rs` and `postgresql/src/keyed.rs:166`, the `LoadedQuestion` matches in `sqlite/src/many.rs:274`. Why: each host owns code the crate could supply. Ticket 0347 is in flight. Size: M. Blocks 0.1: no.
4. **Split `postgresql/check.sh` by topic and put the shell under a ceiling.** Where: `postgresql/check.sh` (1,317 lines, 105 functions), `sdlc/scripts/ratchet.mjs`. Why: a new person changes one giant script, and growth goes unmeasured. Size: M. Blocks 0.1: no.
5. **Send `Connection: close` from the SQL loopback fixtures, or pool-proof the engine.** Where: `databases/sqlite/tests/conditional_backend.py`, the other Python backends that reply over HTTP/1.0. Why: DuckDB hit the flake 11 of 200 runs, and only one harness works around it. Size: M. Blocks 0.1: no.
6. **Rewrite the three `NOTES.md` where they contradict the READMEs.** Where: `sqlite/NOTES.md` ("Not-run forms", "Measurements"), `duckdb/NOTES.md` ("Limits"), `postgresql/NOTES.md` ("How the check runs"). Why: they say find has no SQL function and that case 18 runs. Size: S. Blocks 0.1: no.
7. **Record one rule for symlinks in named files.** Where: `sqlite/README.md` "Runtime boundaries", `postgresql/README.md:138`, `sdlc/planning/databases/README.md`. Why: SQLite follows them and PostgreSQL refuses them, and neither choice is written as a decision. The SQLite host reads any file its process can read and sends the text to the backend (unconfirmed severity; direct-only calls limit who can ask). Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: the three READMEs and NOTES, `sqlite/check.sh` and the head of `postgresql/check.sh`, `sqlite/src/settings.rs`, `postgresql/src/call.rs` guard, `postgresql/src/files.rs` head, `duckdb/cpp/CMakeLists.txt` link lines, ADR 0111 item 1, tickets 0347 and 0348, the four open issues the brief names, and the source sizes.

Not checked: no build or test ran, so the DuckDB macOS export finding and the ureq flake come from the issues, not from my measurement. I did not read the DuckDB C++ and bridge source, the PostgreSQL relate, keyed and scalar modules, the DuckDB `proof/0201` folder, or ticket 0310, 0318, 0336 and 0304's slice records beyond the ADR text. Whether the PostgreSQL confinement tests cover symlinks and `..` beyond the code's own checks is unconfirmed (only 2 `file_directory` hits in `postgresql/check.sh`; the cases may sit in `tests/settings_cases.py`). Tickets 0347 and 0348 are landing and will change items 1, 3 and 6.
