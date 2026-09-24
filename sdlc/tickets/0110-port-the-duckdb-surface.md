---
flow: build
priority: 110
opens: databases/duckdb sdlc/scripts sdlc/issues sdlc/planning/databases/duckdb.md sdlc/planning/adr/0038-duckdb-relate-runs-on-the-callers-database.md
---

# 0110: Port the DuckDB surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the DuckDB extension from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-duckdb` at `databases/duckdb`, in its own Cargo workspace. The loaded extension keeps its SQL names: `thinkthen_decide`, `thinkthen_probability`, `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`, `thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`, `thinkthen_warm`, `thinkthen_recognize`, `thinkthen_relations`, and `thinkthen_relate`. `WHERE`, `ORDER BY`, and `LIMIT` stay the filter, rank, and find verbs. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists DuckDB after Python and TypeScript.

Draft ADR 0047 (on the 0093 branch until 0093 lands) fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. ADR 0038 on main records relate on the caller's database. Accepted ticket 0105 is the template for this ticket's shape. Ian can overturn every decision here.

## Design and decisions

1. **No C door underneath.** The extension links `thinkthen` by path and never links `thinkthen-c` (ADR 0047 item 1: no binding depends on another binding). DuckDB loads a Rust `cdylib` through `duckdb-rs`, and a C door would add a JSON hop per chunk. Ticket 0094 serves as the reference for the FFI edge only: one panic guard, one error-kind table, and one module that holds `unsafe`.
2. **One `unsafe` module.** The tag spreads about 300 `unsafe` sites over six files. The port moves every raw DuckDB C API call, the signal handler, and the extension entry point into `src/ffi/`, and only `mod ffi` carries `#[allow(unsafe_code, reason = "…")]` (ADR 0047 item 3). The rest of the crate holds safe code over typed wrappers.
3. **Engine.** Every call uses `thinkthen::default_engine()`. LOAD registers the functions and the SIGINT handler and touches no wire. SQL cannot name a backend. `Engine::from_env` reads the environment.
4. **Chunks go in bulk.** Each scalar invoke reads its chunk, drops NULL rows, dedupes texts in first-seen order, and makes one streaming call. Decide and probability use `decide_many_with`, and `thinkthen_probability` reads `Row::probability` (G4). Choose, score, and tag use `annotate_with` over a one-question set (0095, closing R2-23). Choose and tag with list members build through `Question::choose_labels` and `tag_labels` (G5). Answers map back by text, so a repeated text or a NULL never shifts a row (R1-1).
5. **Single sends stay promptly interruptible.** `thinkthen_details`, `thinkthen_recognize`, and `thinkthen_relations` make one call per distinct text. Each runs on a spawned worker that owns clones of its inputs, as in 0105 decision 6. The DuckDB thread waits on a channel in 50 ms ticks and reads the interrupt predicate of decision 6. A stop cancels the worker's internal token, detaches the worker, and raises at once. Batches use `CallOptions::interrupt` with the same predicate, and the engine runs it within one 50 ms tick (0095).
6. **SIGINT.** LOAD takes SIGINT through `sigaction` and keeps the host's action for the chain, as at the tag. The handler bumps an atomic signal count, stores the signal time, writes one byte to a per-process pipe, and chains. It never calls into `thinkthen`. 0084 does not promise that `CancelToken::cancel` is async-signal-safe. Each call builds its own `CancelToken`. The tag's rules in `current_cancel` (signal inside this invoke, another call in flight, an active burst, a start within one burst window) become one predicate over those atomics, and that predicate is the call's interrupt check. The one-shot token and its re-arm logic leave, so no token outlives a call (R1-21, R2-14). The bridge thread still reads the pipe and interrupts busy relate connections. The install records the host's action before `sigaction` installs ours (R7-7).
7. **Deadlines.** The ruled third argument stays milliseconds and goes through `CallOptions::deadline_millis` (ADR 0041). A NULL deadline gives a NULL row, as at the tag. `thinkthen_warm` and `thinkthen_relate` take no deadline argument. Relate passes the remaining `thinkthen_relate_seconds` budget to the engine as its deadline.
8. **Question arguments.** A question is plain text, `'@file.json'`, or the file grammar's JSON. The binding keeps the tag's access check against `enable_external_access`, `allowed_directories`, and `disabled_filesystems`, reads the file itself, and calls `Question::from_json` (R1-15, R2-3, R4-5). A `Usage` from a file's text maps to the `local` word, per 0095. The per-statement cache keyed by path and modification time stays (R2-18).
9. **Relate from rows.** Main's relate takes entities with a name and a kind (`specification/relate.md`). The tag's relate was the superseded all-yes-or-no design over bare text. `thinkthen_relate(query, rules)` now reads a query whose columns are id, name, and kind. A two-column query of id and name reads kind `*` and accepts only bare rules, as the command's `--lines` does. `rules` is `'@file.json'`, the version-one relate file as JSON text through `Relate::from_json`, or a `LIST` of the command's inline rule strings. Per ADR 0047 item 9, the binding dedupes rows by name and kind in first-seen order, calls `relate_with` once, and returns one row `(relation, source, target, probability)` per matching id pair. Ids come back as text. More than 255 rows is `usage`, read under `LIMIT 256` (R3-12). This row cap is stricter than the engine's unique-pair cap, and the README says so. The read-only single-`SELECT` rule, the plan-size guard, the time limit, the kept connection, and the reaper carry over unchanged. The tag's `{"either": [...]}` rule list leaves, and `either` rides in the relate file. The 2026-09-23 issue on relate in a database (item 1) asks for this shape. Its items 2 to 4 are out of scope.
10. **Recognize.** `thinkthen_recognize(body, kinds)` builds through `Recognize::builder` and returns a `LIST` of `(name, kind, start, end, strength)`. `start` and `end` count code points. DuckDB's string indexing counts the same way. `thinkthen_relations(body, spec)` reads `Recognize::from_json` or the file and returns `(relation, source, source_kind, target, target_kind, probability)`.
11. **Results.** `thinkthen_score` returns the position. `thinkthen_details` returns one struct from `Details` accessors: the yes probability, the answer word, `nearest`, `model`, `question_sha256`, `requests_sent`, and `cached`. A member the verb lacks reads NULL and never 0 (R3-11). Choose and tag now fill `thinkthen_details`, because main serves them (G5). `thinkthen_annotate` returns `AnnotatedRecord::value_json`. `thinkthen_usage()` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`. Counters have no reset, and tests assert differences around a call (0095).
12. **Errors and panics.** One exhaustive `match` maps `ErrorKind::name` to the message prefix `thinkthen <kind>: `. One guard function contains every callback. The tag's three wrappers in `guard.rs` fold into it (R2-31). `thinkthen` already stops engine panics at its public methods (0086).
13. **The drawn calls.** The tag vendored the deck's calls into `tools/drawn-calls/` and compared them with a private deck when one was on the machine. The port drops the copy. The check runs every `site/src/data/examples/*__duckdb.json` code block in this repository (R5-24). The relate and recognize examples draw the tag's shapes. Decisions 9 and 10 change those shapes. The check runs the ported replacements for those two and names both divergences in its output. The builder files an issue for the deck's owner in the deck's repository with the new shapes.

## What moves from the tag

- `src/` becomes files under 500 nonblank lines each: `connections.rs` and `relate.rs` split, and `lib.rs` splits into registration, scalars, questions, and signals. `src/ffi/` holds every `unsafe` line.
- `README.md`, rewritten for decisions 5 to 11. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 150 lines.
- The suites under `tools/`: `null_suite.sh` (renamed `verbs_suite.sh`), `mapping_suite.sh`, `security_suite.sh`, `relate_guard_suite.sh`, `relate_wait_timer.py`, `atfile_suite.sh`, `rearm_suite.py`, `two_databases.py`, `review3_duckdb.py`, `panic_suite.sh`, `cancel_fast.sh`, `host_signal.py`, the `review4_*.py` probes, `examples.py`, `conformance.py`, `conformance_selftest.sh`, `check_entry_selftest.sh`, `deprecated_api_check.py`, and `wire_suite.sh`. Every suite moves off `ENGINE_NULL`, `ENGINE_BASE_URL`, and the stub on port 8217 onto a 0092 backend it starts itself. The builder regroups them into at most 26 files.
- The tag's gate3 lane message for a relate that waits in the queue, with its test (`relate_wait_timer.py`).
- `extension-ci-tools/scripts/append_extension_metadata.py` and `configure_helper.py`, with DuckDB's MIT license beside them in `vendor/`. The two vendored makefiles, the `Makefile`, and `.cargo/config.toml` leave. `check.sh` calls `cargo` and the metadata script directly.
- The issue `2026-09-23-duckdb-scalars-through-the-c-api-for-a-query-hook.md` moves from the tag to main's `sdlc/issues/`. R5-23 names it as its lever.

These stay at the tag or retire: `package.sh` (the release ticket, queue item 4), `tools/drawn-calls/`, `drawn_calls_drift.py`, `run_slide.sh` and `slide.sql` (replaced by decision 13), `run_recognize.sh` (replaced by the ported relate and recognize suite), the `synthetic-partial` feature, and the stand-in-only check that greps for `thinkthen_standin`.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 32 DuckDB rows, and this ticket re-proves all 32. It also re-proves the DuckDB halves of fourteen cross-surface rows and of two engine rows. Each re-proof runs against the real engine through a 0092 loopback backend in the stock v1.5.5 CLI or the venv's `duckdb` module, unless marked as a unit test. The record plants each bug below and shows its test turning red, then green once the bug is removed. "Counted" means the backend's `count` line. "Held" means 0092's held arm. Every held test releases or cancels after it reads its count. Every subprocess runs under `timeout` with a stated bound.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-1 | closed | Case 28's texts, shuffled, with two NULL rows, through `/case/28-decide-many-repeated-texts`: `thinkthen_decide` and `thinkthen_probability` put each case answer on its own row and NULL on the NULL rows. Score does the same over case 35's texts. | Map answers back by distinct position. Rows swap. |
| R1-15 | closed | With `enable_external_access=false`, `'@q.json'` refuses with the pinned sentence, and `strace` counts zero opens of `q.json`. | Drop the access check. The open count reads 1. |
| R2-3 | closed | `allowed_directories` without the folder and `disabled_filesystems='LocalFileSystem'` each refuse `@file` in decide, annotate, and relations. A missing file and a refused file give the same message. | Check only `enable_external_access`. The `allowed_directories` case runs. |
| R4-5 | closed | A test hook marks eight live databases retired. `@file` still refuses with access off. | Skip the check for a retired database. The file is read. |
| R2-18 | closed | 20,000 rows over `'@q.json'` open the file once per statement under `strace`. | Read the file per row. The count reads 20,000. |
| R1-16 | closed | Two file databases in one process each relate over their own table, with 4 and 11 edges. A temporary table answers the ADR 0038 boundary sentence. | Route relate to the last-loaded database. The counts cross. |
| R2-13 | closed | Two `:memory:` databases each relate. `ATTACH … USE` counts the used database's table. A closed file database reopens, since no kept connection holds its lock. | Key the registry by database name. Both in-memory relates fail. |
| R2-2 | closed | A relate query holding a `DELETE` is refused before it runs, and the table keeps its rows after the caller's `ROLLBACK`. | Skip the statement-kind check. The row count drops. |
| R3-7 | closed | `COPY … TO`, `EXPORT DATABASE`, `ATTACH`, `SET GLOBAL`, `LOAD`, and `SET VARIABLE` as the relate query each refuse, and no file appears. | Check table writes only. `COPY` writes its file. |
| R2-6 | closed | A relate nested in a relate query refuses within 1 s under `timeout 30`. | Drop the nesting guard. The timeout fires. |
| R3-1 | closed | 200 rounds: a prepared relate is held while the reaper runs a forced pass, then executes and answers, with exit 0. | Drop the counted handle. The run exits 139. |
| R3-6 | closed | No `thinkthen_instance_token` setting exists. Database B's relate cannot read database A's tables. | Resolve identity from a setting. B reads A. |
| R4-3 | closed | A closes, the reaper drops A, B loads, and A reconnects. A's relate reads A's table and still refuses `/etc/hostname` with access off. | Reuse a freed registry slot. A reads B. |
| R4-4 | closed | A read-only database gets its probe. Detaching the probe and attaching `:memory:` under its name is refused. | Fall back to the name when the probe is missing. The forged read succeeds. |
| R5-26 | closed | Two processes produce distinct identity names of 32 hex characters from `getrandom`. | Seed the hash with a constant. The names match. |
| R3-23 | closed | 500 idle databases cost under 5 percent of one core in CPU time over 10 s, read by `getrusage`. | Poll every database every 200 ms with no backoff. |
| R3-12 | closed | Relate over `generate_series(1, 8000000)` refuses with the pinned 255 sentence within 2 s and under 200 MB of peak growth. Another database's relate runs meanwhile. | Read every row before counting. The memory bound fails. |
| R5-22 | waive | With `SET thinkthen_relate_seconds = 2`, the `unnest` and misjudged-join shapes stop by 3 s with the `deadline` word. The README pins the sentence that `memory_limit` is the hard bound. | Drop the timer. The `timeout 30` bound fires. |
| R1-21 | closed | After one SIGINT during a held batch, the next query answers normally. 50 rounds. | Latch the stop: any past signal cancels. The next query reads `cancelled`. |
| R2-14 | closed | A SIGINT that lands within 250 ms before a call leaves the next query answering. 50 rounds. | Treat a signal in the last 250 ms as live for every new call. |
| R3-13 | closed | An `SA_SIGINFO` host handler gets the real signal number and sender. An ignored SIGINT stays ignored, and ours never installs. A default action re-raises and ends the process. | Install with `libc::signal`. The siginfo assertion turns red. |
| R4-22 | closed | One SIGINT stops two and then four concurrent held queries, relate among them, 20 of 20 runs, each within 100 ms of the signal. The relate query runs once. | Stop only the thread that saw the signal. Another query finishes. |
| R6-6 | closed | With a chained host handler, 20 of 20 SIGINTs stop the running query. A query reading its chunk before its engine call counts as running. | Count engine calls only. A signal during the chunk read is missed. |
| R5-21 | closed | A source check over the handler in `src/ffi/signal.rs` refuses a lock, an allocation, or a call into `thinkthen`. 10,000 SIGINTs while four threads allocate end under `timeout 60`. | Add `Mutex::lock` to the handler. The source check fails. |
| R7-7 | open | A test-only `test-hooks` feature raises SIGINT between the host-action record and our `sigaction`. The host handler's count reads 1. | Record the host action after the install. The count reads 0. |
| R6-5 | closed | A forked child's SIGINT leaves the parent's held relate running. The parent's relate answers after release. | Share the parent's pipe with the child. The parent's relate reads `cancelled`. |
| R2-22 | closed | `thinkthen_warm` judges under the first-seen question. A second distinct question in one group is `usage`. A backend failure raises and never reads as 0. | Keep the last-seen question and swallow errors. |
| R3-11 | closed | Details on score reads NULL probability, a named `nearest`, and the engine's `requests_sent`. Choose and tag fill `value`. No absent member reads 0. | Fill a missing probability with 0.0. |
| R4-16 | partial | `deprecated_api_check.py` finds zero deprecated result-API calls in `src`. Every created logical type has its destroy. Clippy passes with `-D warnings`. The shipped extension holds no `ENGINE_TEST_PANIC` string. The handler takes no lock (R5-21). | Add one `duckdb_value_varchar` call. The API check fails. |
| R5-23 | open/waive | Documented limit, pinned. `con.interrupt()` from Python does not stop a held batch before release. A SIGINT does, within 100 ms. The README pins the limit sentence and names the query-hook issue as the lever. | Delete the README sentence. The pinned-sentence check fails. |
| R5-24 | open | The check runs each `site/src/data/examples/*__duckdb.json` block. `databases/duckdb` holds no copy of those calls. | Change a site example to call `thinkthen_decidex`. The check fails. |
| R5-25 | closed | `databases/duckdb/deny.toml` equals the root `deny.toml` plus one `[[licenses.exceptions]]` entry per named crate. A `lint` step compares them. | Add `Zlib` to the global `allow` list. The comparison fails. |
| R1-2, R5-31 DuckDB halves | closed | `conformance.py` exits nonzero on any failed case, and a lookup miss refuses. `conformance_selftest.sh` plants a wrong expected answer and sees the run fail. | End the runner with `exit 0`. The self-test fails. |
| R3-30, R5-32 DuckDB halves | closed | The runner has no local skip list. It reports pass, fail, or not run with a reason from one closed list in the runner, and the three counts sum to the cases in `cases.json`. | Skip one case silently. The sum check fails. |
| R6-7 DuckDB half | closed | `check_entry_selftest.sh` runs `check.sh` from `/` and from `sdlc/`. Setup passes both times. | Read `tools/version.env` by a relative path after the `cd`. |
| R3-29 DuckDB half | partial | Setup installs only `duckdb==1.5.5`, offline from the uv cache, with no `git+https` and no unpinned package. Setup refuses a requirements line without `==`. | Add an unpinned `packaging` line. Setup still passes, and its refusal test turns red. |
| R4-19, R5-34 DuckDB halves | closed | Every `cargo` call in `check.sh` passes `--locked` and `--offline`. A `check.sh` step reads the script and fails on a call without both. | Drop `--locked` from the release build. |
| R3-28, R1-28 DuckDB halves | closed | The binding's lint table equals the root's except `unsafe_code = "deny"`. `policy.py` refuses `unsafe` outside `src/ffi/`. | Put one `unsafe` block in `src/scalars.rs`. `lint` fails. |
| R1-33 DuckDB half | closed | `vendor/` holds the two used scripts and DuckDB's license. A `check.sh` step fails on a vendored file that no script names. | Add an unused vendored file. |
| R2-10 DuckDB half | partial | `-1` runs with no deadline. `0` reads `deadline` with zero counted requests. `-2` and `4294967296000` read `usage`. | Treat any negative as none. `-2` runs. |
| R2-23 DuckDB half | open/waive | 64 distinct texts through `thinkthen_choose` on the held arm at width 8 hold 8 counted requests in flight. | Loop per row. The count reads 1. |
| R2-31, R1-31 DuckDB halves | partial, waive | `check.sh` counts one `catch_unwind` site in `src`. A Rust unit test maps every `ErrorKind` to its word. | Add a second guard, or map `Deadline` to `backend`. |
| R1-10 host half | engine | In the `test-panic` build, a panic in a scalar, a table bind, init, and function, an aggregate, and the relate scan each reads `defect`, and the CLI answers the next query. | Remove the guard at one boundary. The process aborts. |
| R1-11 host half | engine | A deadline of `9223372036854775807` reads `usage` with zero counted requests. | Convert with an unchecked `Duration` add in the shim. The guard turns the panic into `defect`. |

R2-29's DuckDB half lands as a DuckDB amendment to ADR 0038. It records these rulings from the tag's `NOTES.md`: SIGINT at LOAD with the chain, the interrupt predicate of decision 6, the relate time limit (R5-22), the `con.interrupt()` limit (R5-23), relate from rows (decision 9), and the license exceptions.

R5-37's DuckDB half (the package script on Darwin and its `--dry-run`) moves with `package.sh` to the release ticket. R3-32 and R4-10 have no DuckDB hit at the tag. On Darwin, `check.sh` reports "not run" at 0.1.

Rows that retire here: R1-27 (the connector), R1-29 and R5-42 (the private deck and the branch hermeticity page), R5-36 (the wire stub), and R6-1 (the online setup). Setup is now offline. The freeze record's three review follow-ups name no DuckDB code.

Not closed here: G9 waits on ADR 0047 item 5. The venv's `duckdb` module loading this extension beside a Python `thinkthen` wheel is the plainest case of two engine images in one process. The DuckDB page states whichever answer Ian gives.

## Other acceptance

- Red first: the ported suites fail against an empty `databases/duckdb` workspace for the stated reason, then pass.
- `cargo test --locked --offline --lib` in `databases/duckdb` passes.
- The conformance runner runs every applicable case in main's `cases.json` through the case arm with recomputed digests. Case 23 (cancel) uses the held arm and a SIGINT. Case 24 (deadline) now runs. DuckDB has a deadline argument. Cases 51 and 52 run through `thinkthen_relate`. No backend arm is added.
- Case 41 returns the same `start` and `end` through `thinkthen_recognize` as through Rust.
- `thinkthen_probability` equals the yes probability `thinkthen_details` returns for the same text, with no added counted send.
- A fork test loads the extension in Python, warms it, calls `os.fork`, and gets an answer in the child. The parent's `thinkthen_usage` does not move (0096).
- Nothing reaches a non-loopback address. A secrecy test sets a sentinel `THINKTHEN_API_KEY` and a base URL with credentials, then reads every error message and every `thinkthen_details` row for both.
- The worker of decision 5 survives a closed channel, and a channel that closes with no result raises `defect`. A Rust unit test covers each edge.

## The check it adds to the gate ladder

- `databases/duckdb/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047) runs it with the 0092 port. A missing tool reports "not run" and never "pass" (R6-2), and the line names the one fetch to run on a networked machine.
- **Where it runs.** On Beelink, the Linux machine that owns this repository, with no network and no Docker. The freeze record says DuckDB's check did not run there. `check.sh:36` stopped because `make configure` had never run in the freeze worktree. Docker was never involved. Observed by command on 2026-09-24:
  - `cargo check --offline --locked` on the tag's DuckDB crate exits 0. The cargo cache holds `duckdb`, `libduckdb-sys`, and `duckdb-loadable-macros` 1.10505.0 and the rest of the tree. The `loadable-extension` feature compiles no DuckDB source.
  - `uv venv --offline --python 3.13` plus `uv pip install --offline duckdb==1.5.5` succeeds from the uv cache, with CPython 3.13.5 from uv. The same install under Python 3.12 fails, because the cache holds only the `cp313` wheel.
  - The stock DuckDB v1.5.5 CLI (`v1.5.5 (Variegata) d8cdaa33fd`, sha256 `3d33b1df037cb049155c393778df7853fafb23e9d49d7c9cacdde4dd67155788`) exists on this machine only as a gitignored file in an old worktree. The CLI on `PATH` is v1.1.3 and cannot load what this build makes.
  - `strace`, `timeout`, and `cargo-deny` 0.19.4 are installed. `rustc` is 1.93.1, main's pin.
- **Setup.** `check.sh` runs its own offline setup in the gitignored `databases/duckdb/.setup/`. It creates the venv as above. It writes `platform.txt` through `configure_helper.py` and `extension_version.txt` from the crate version. It finds the CLI at `$THINKTHEN_DUCKDB_CLI`, or else at `~/.cache/thinkthen-dev/duckdb/v1.5.5/duckdb`, and checks the pinned sha256 from `tools/version.env`. A missing or mismatched CLI reports "not run" with the fetch command: the release zip `duckdb_cli-linux-amd64.zip` for v1.5.5 from DuckDB's GitHub releases. The builder seeds that path on Beelink by copying the existing binary with no download, and the record says so.
- Every `cargo` step passes `--locked` and `--offline`. `check.sh` drops `ENGINE_NULL`, `ENGINE_BASE_URL`, the `synthetic-partial` build, and the stub on 8217.
- `lint` runs on `databases/duckdb`: the ADR 0047 manifest, lock, lint-table, and profile checks; the registry check; `ratchet.mjs` on `databases/duckdb/ratchet.json` for Rust and `ratchet.py.json` for Python; and deny as `cargo deny --offline --manifest-path databases/duckdb/Cargo.toml check --config databases/duckdb/deny.toml advisories bans licenses sources`. The deny plant is a git-sourced dependency. `[sources] unknown-git = "deny"` refuses it.
- **Licenses.** Main's `deny.toml` refuses three crates in the tag's DuckDB tree, observed with that command against the root config: `foldhash` 0.1.5 (Zlib, through `hashlink` under `duckdb`), `zlib-rs` 0.6.8 (Zlib, through `zip` in `libduckdb-sys`'s build dependencies), and `tiny-keccak` 2.0.2 (CC0-1.0, through `const-random` under `arrow-select`). Advisories, bans, and sources pass. Decision: the binding's `deny.toml` copies the root file and adds one per-crate exception for each, as R5-25 asked. The root list stays untouched. Both licenses are permissive. The builder first drops the `vscalar-arrow` feature if the build allows it, and the exception list names only crates in the final lock. Ian can overturn this.
- Lints. The root table forbids `missing_debug_implementations`, `unreachable_pub`, and `unsafe_code`, and denies `expect_used`, `unwrap_used`, `indexing_slicing`, `panic`, `print_stdout`, and `print_stderr`. A local `allow` cannot lift a forbid. The extension entry point macro sits in `src/ffi/`. If `duckdb-rs` generated code trips a forbid-level lint, the builder stops and records the case for an ADR 0047 amendment.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off. `duckdb` `1.10505.0` with `loadable-extension` and `vscalar`, and `vscalar-arrow` only if needed. That brings `libduckdb-sys` and `duckdb-loadable-macros` 1.10505.0. `libc` `0.2.189` and `serde_json` `1.0.151` match the root lock.
- Test and build tools: Python 3.13 from uv, `duckdb==1.5.5` from the uv cache, and the DuckDB v1.5.5 CLI pinned by sha256. The two vendored scripts come from DuckDB's `extension-ci-tools` under MIT.
- `duckdb-rs`, its tree, the Python `duckdb` package, and the CLI enter main for the first time. The code reviewer checks each entry, the binding lock, the deny exceptions, and deny's result, and the review record says so (repo `CLAUDE.md`).

## Budgets

The tag measures 4,457 nonblank production Rust lines in eight files. Its largest are `lib.rs` (1,631), `connections.rs` (1,185), and `relate.rs` (953). The test modules hold about 390 more. The tools hold 2,745 nonblank Python and shell lines in 30 files. `check.sh` is 162 and the `Makefile` 25.

- Production Rust: at most sixteen files and 4,300 nonblank lines, each file under 500. The stand-in glue, the token re-arm code, and the deprecated calls leave. The worker and `src/ffi/` come in.
- Tests: at most 26 Python and shell test files and 3,000 nonblank lines, at most 450 nonblank Rust unit-test lines, and the conformance runner at most 400.
- Scripts: `check.sh` and its setup at most 260 nonblank lines. Gate changes under `sdlc/scripts` at most 40.
- Vendored: the two scripts at 131 nonblank lines, unchanged, counted by `ratchet.py.json`.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/databases/duckdb.md`, and the ADR 0038 amendment, at most 360 net nonblank lines.
- Ratchet: `databases/duckdb/ratchet.json` and `ratchet.py.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, or widening the root `deny.toml`.

## Exclusions

Release archives, community-repository signing, and `package.sh` (queue item 4). New relate forms for new rows against old rows, or links to a known entity table. A connection-level cancel (R5-23 stays a documented limit). Any change to `thinkthen`. Any live or paid call. Changing the site examples or the deck.

## Dependencies

After 0086. Also after 0098 (labels, spec readers, `Row::probability`, JSON methods, and `ErrorKind::name`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 and 0099 have landed. 0105 and the TypeScript ticket come first in the queue. This ticket needs no code from either.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 4; reach 2; proof 3; cost of error 3; total 14. Final level: 3. Signal handling, a reaper, and routing on the caller's database sit inside another program's process, and a wrong rule loses a user's Ctrl-C or reads another database's tables.

## Review

- Design review: pending.
- Code review: pending.
