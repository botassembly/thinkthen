---
flow: build
priority: 109
opens: databases/sqlite sdlc/scripts sdlc/planning/databases/sqlite.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0109: Port the SQLite surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the SQLite extension from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-sqlite` at `databases/sqlite`, in its own Cargo workspace. `.load ./thinkthen` keeps the eight functions (`thinkthen_decide`, `_choose`, `_score`, `_tag`, `_annotate`, `_details`, `_usage`, and the `_warm` aggregate) and the two table-valued functions (`thinkthen_recognize`, `thinkthen_relate`). Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists SQLite among the surfaces after C.

Draft ADR 0047 fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls. Ticket 0105 is the accepted template for a surface port. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ian can overturn every decision below.

## Design and decisions

1. **The Rust API.** The extension links `thinkthen` by path and calls the public Rust API through `rusqlite`, as the tag does. It does not load `libthinkthen.so` from ticket 0094. The C header gains no interrupt symbol (0094), so a warm pass behind the C door could not hear `sqlite3_is_interrupted` within one tick. The C door would also add a second shared library to every install and a JSON round trip to every row. This ticket builds on 0094 as the reference FFI edge (ADR 0047 item 7): one panic guard, one error-kind table, one `unsafe` module, and an exported-symbol check. 0094's churn probe covers R7-1 for the engine both crates link.
2. **Engine and cache.** Every call uses `thinkthen::default_engine()`, configured by the environment variables `Engine::from_env` reads. SQL cannot name a backend or a key. The tag's saved-answer map (`Saved`, `SavedAnswers`, and the hit counter) retires. The engine's bounded disk cache (0062) replaces it, so warmed answers now outlive the process, as `sdlc/planning/databases/sqlite.md` asks. `thinkthen_usage()` returns `Engine::usage()` as `{"requests_sent","cache_answers","input_tokens","output_tokens"}`. The `'reset'` spelling keeps its refusal sentence. With the cache off, a warm pass still judges every row and the queries after it send again. The README says so.
3. **Questions.** Plain text builds `Question::decide(text)?.cut()`. Text that starts with `{` goes to `Question::from_json`. `'@name'` reads the named file through the tag's `named_file` (one `O_NONBLOCK` open, a regular file of at most 1 MiB, symlinks followed, one refusal sentence for every cause) and then calls `from_json`. A broken rule in a file maps to `local`, and in an argument to `usage` (0095, Q16). The tag's question and question-set caches stay: 4,096 entries each, oldest out first, re-read when a file's modified time or size changes.
4. **Results.** `decide` returns 1, 0, or NULL. `choose` and `tag` call `details` and read `value()` with no added send (G5). `choose` returns the label or NULL, and `tag` returns a JSON array. `score` returns the position as REAL. `annotate` returns `AnnotatedRecord::value_json`, with a failed member as `{"failed":{"kind":…,"cause":…}}`. `details` returns `Details::to_json`, equal to the command's `--details` document. A SQL NULL text returns NULL and sends nothing (branch case 81). A BLOB argument, or text with a NUL byte or invalid UTF-8, raises `usage` before any send (G11).
5. **Single calls are promptly interruptible.** This follows 0105 decision 6. The engine's interrupt check does not run during one blocking send, and a send may hold for the 30-second request timeout. The CLI's Ctrl-C calls `sqlite3_interrupt`, so a user would wait for the reply. Design: `decide`, `choose`, `score`, `tag`, `annotate`, `details`, and `recognize` each run on a spawned worker that never touches SQLite. The worker owns an `Arc` of the parsed question, the text as a `String`, and a clone of an internal `CancelToken`. It builds its own `CallOptions` with that token and the call's deadline. The calling thread waits on a channel with a 50 ms timeout, so a finished call returns at once. On each tick it reads `sqlite3_is_interrupted` on the calling connection, and it checks the deadline instant. On an interrupt it cancels the internal token, detaches the worker, and returns `thinkthen cancelled`. On a passed deadline it does the same and returns `thinkthen deadline`. The detached send finishes under 0073 and starts no retry. The worker ignores a closed channel. A channel that closes with no result returns `defect`. The extension keeps the tag's `RTLD_NODELETE` pin, so a detached worker's code stays mapped after the loading connection closes. Cost: about 60 lines and one thread spawn per single call. A detached send holds one width permit until it ends. This design retires the tag's shared watcher thread, its registry, the `pthread_atfork` handler, and the fork generation. No thread outlives its call.
6. **Warm and relate use the interrupt check.** `thinkthen_warm(question, text)` groups rows by question in first-seen order with the tag's hash index. It flushes each group at 256 rows through `decide_many_with`, and it passes one `CallOptions::interrupt` check that reads `sqlite3_is_interrupted` for the calling connection. The engine runs it on the calling thread within one 50 ms tick (0095). The connection handle is not `Sync`, so the FFI module wraps it (0095). Equal pairs in one pass are asked once. Warm takes a decide question only. Any other kind raises `usage` naming `thinkthen_decide`. It returns the count of distinct pairs judged. `thinkthen_relate` passes the same check.
7. **Deadlines.** Each scalar keeps the three-argument spelling: the last argument is milliseconds, and an INTEGER goes to `CallOptions::deadline_millis` (ADR 0041). `-1` is none, `0` is spent and sends nothing, and another negative or a value above 4,294,967,295,000 is `usage`. A REAL is accepted only when it is finite and whole. TEXT, BLOB, and NULL are `usage`. The message names the value.
8. **Recognize.** `thinkthen_recognize(text, kinds)` keeps its eponymous table-valued shape and hidden arguments. `kinds` is a comma list, a JSON recognize section, or `'@name'`. The binding composes the recognize JSON and calls `Recognize::from_json` (0095), so it adds no grammar. A spec with relation rules raises `usage` that names `thinkthen_relate`. The columns become main's `(name, kind, start, end, strength)`. The tag's `text` column is renamed `name`. Offsets count Unicode scalar values, and SQLite's `substr` counts the same way (case 68).
9. **Relate reads entities.** Main's relate takes entities with a name and a kind (`specification/relate.md`). The tag's call over body text has no meaning there. The new call follows the database relate issue (`sdlc/issues/2026-09-23-relate-in-a-database-kinds-new-rows-and-the-cache.md`, item 1) and ADR 0047 item 9: `thinkthen_relate(table, id_column, name_column, kind_column, rule, …)` with one to four rules. Each rule is `NAME`, `NAME=SOURCE:TARGET`, `either:NAME`, or `either:NAME=SOURCE:TARGET`, or a single JSON relate section or `'@name'`. The binding composes the version-one relate file and calls `Relate::from_json`. The nested read-only `SELECT` of the tag's `read_records` stays, with quoted identifiers. It dedupes rows by name and kind in first-seen order. It stops reading at the 256th unique pair and raises `usage` naming the 255 limit. A NULL or blank name or kind raises `usage` naming the row's id. The output is `(relation, source, target, probability)`, and `source` and `target` carry the id column's values. Each edge is emitted once for every pair of rows holding its two endpoints, so the result joins back to the table by id. The tag's `name` output column is renamed `relation`, as in `Edge`.
10. **Direct-only and volatile.** Every function keeps `SQLITE_DIRECTONLY`, and both modules keep `VTabConfig::DirectOnly`. None carries `SQLITE_DETERMINISTIC`. The load-time check keeps the 3.50.0 floor and its refusal sentence. Below 3.50.0 a CHECK constraint in an untrusted database reaches a volatile function. The host's `is_interrupted` comes from its own API table through the tag's `ApiRoutines` tail, since `libsqlite3-sys` 0.38.2's loadable bindings stop at 3.34.
11. **Errors and panics.** One exhaustive `match` maps `ErrorKind::name` to the message prefix `thinkthen <kind>:` and a SQLite result code. It keeps the tag's codes: `cancelled` is `SQLITE_INTERRUPT`, `usage` is `SQLITE_CONSTRAINT`, `local` is `SQLITE_CANTOPEN`, and the rest are `SQLITE_ERROR`. A retryable failure keeps ` (retryable)` after the kind. One guard wraps every function body and vtab callback and turns a binding panic into `defect`. `thinkthen` already stops engine panics at its public methods (0086).

## What moves from the tag

- `src/lib.rs`, split into files under 500 nonblank lines. Every `unsafe` block, the entry point, the API-table tail, the pin, and the `unsafe impl VTab` blocks go into one FFI module (ADR 0047 item 3). `Saved`, the hub, `InterruptWatch`, `watch_forks`, `forked_child`, and the stand-in imports leave.
- `Cargo.toml` with the crate name, lib name `thinkthen0`, `crate-type = ["cdylib"]`, and the entry-point rule. The contract and stand-in path dependencies and the `synthetic-partial` feature leave.
- `README.md` and its authority section, rewritten for the engine, the worker, the cache, and the new relate call. `tests/slide.sql`, unchanged. `examples.json` and `tests/examples.py`, with each value re-derived against 0092's generic arm.
- The tests keep their assertions where the behavior stays: `schema_refusal.py`, `single_row_cancel.py`, `two_connections.py`, `cancel_fast.py`, the loopback parts of `wire_suite.py` and `null_suite.py`, `tvf_suite.py` (rewritten for the new relate and recognize columns), `tools/file_door.py`, and `tools/question_cache_probe.py`. They stay stdlib Python. The builder regroups them by topic into at most eight files beside one shared helper.
- `tests/conformance_driver.py`, rewritten onto main's `conformance/cases.json`. `test_conformance_driver_fail.py` becomes the runner's planted-failure test.
- `tests/host_sqlite.sh`, grown to build the CLI too (see the check section).

These retire: `package.sh` (the release ticket, queue item 4, owns packaging), `tools/swap_hammer.py` (it could not fail, pre-fix or post-fix, per the tag's NOTES), the stock-CLI download, the wire stub on port 8218, `ENGINE_NULL`, and `THINKTHEN_NULL`. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 100 lines. `sdlc/planning/databases/sqlite.md` is the design page, and this ticket updates it.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 10 SQLite rows, and the port guide classes all 10 as binding. This ticket re-proves all 10. It also re-proves the SQLite halves of 14 cross-surface rows and the host halves of two engine rows. Each re-proof runs against the real engine through the 0092 loopback backend unless marked as a unit test. "Counted" means the backend's `count` line. The record plants each bug and shows its test turning red, then green once the bug is removed. Each timed test sends its signal only once the `count` line reads the stated number, and each held test releases the arm or cancels before it ends.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-14 | closed | Two connections load the extension. A held `thinkthen_decide` on the second is interrupted through the second once the count reads 1, and it returns `thinkthen cancelled` within 100 ms. Closing the first connection first changes nothing. | Read the interrupt flag through the handle of the first connection that loaded the extension. The second connection's interrupt is never heard, and the 100 ms assertion turns red. |
| R1-17 | closed | On the 3.50.0 host, with `trusted_schema` on and off, a view, trigger, DEFAULT, CHECK constraint, crafted generated column, crafted index expression, crafted partial index, and a view over `thinkthen_recognize` inside an attached file each refuse. Top-level SQL still answers. Zero counted requests. | Drop `SQLITE_DIRECTONLY` from `thinkthen_score`. The view that calls it answers, and the count turns nonzero. |
| R2-1 | closed | The stock 3.45.1 library on this machine refuses the load with the pinned floor sentence. A unit test refuses 3.45.1 and 3.49.0 and passes 3.50.0. | Lower the floor to 3.45.0. The stock host loads, and the pinned-sentence assertion turns red. |
| R3-2b | closed | `'@fifo'`, `'@/dev/zero'`, and a 1,048,577-byte file each refuse with the pinned sentence and zero counted requests. Each case runs in a child under a 5 s timeout. | Drop the size cap. The over-cap file reads whole and fails at parse with another sentence. |
| R3-22 | closed | The watcher retires, and its promise stays. 200 `thinkthen_decide` calls answered from the engine cache finish in under 2 s in total. After them the process holds no more threads than before them. | Sleep one full tick before reading the channel. The 200 calls take at least 10 s. |
| R4-17 | partial | Recognize and relate carry the interrupt: a held `thinkthen_recognize` returns `thinkthen cancelled` within 100 ms of the interrupt, and a relate held at its first send counts no new request after the interrupt. A unit test inserts 5,000 questions and finds 4,096 held. The tests find the built library by glob. The saved-answer half retires with the map. | Pass empty `CallOptions` to recognize. The call waits for the release, and the 100 ms assertion turns red. Drop eviction, and the unit test finds 5,000. |
| R4-21 | closed | A parent answers one call, then forks. The child opens its own connection, holds one call, and interrupts it once the count reads 2. `Cancelled` arrives within 100 ms. The parent reaps the child under a 20 s alarm. | Read interrupts from a helper thread that the parent starts on its first call. The child has no such thread, and the 100 ms assertion turns red. |
| R5-18 | closed | Unit test: `WarmState` groups 200,000 rows under 200,000 distinct question keys in under 1 s. | Find a row's group by linear search. The test runs past 1 s. |
| R5-20 | closed | A symlink to a valid question file answers. The README says the surface follows symlinks and confines nothing. | Open with `O_NOFOLLOW`. The symlink refuses. |
| R6-9 | closed | The R4-21 test runs again one generation down: the child forks a grandchild, and the grandchild's held call returns `Cancelled` within 100 ms. No fork generation exists to key on. | The R4-21 plant. |
| R2-22 SQLite half | closed | `thinkthen_warm` over `(q1, t)` and `(q2, t)` counts 2. `thinkthen_decide(q2, t)` then counts no new request. | Key warm groups by text alone. The second question's pair is never judged, and the decide sends. |
| R2-24 SQLite half | closed | A held single call is interrupted through `Connection.interrupt` and returns `thinkthen cancelled` within 100 ms. The test releases the reply, and the count stays at 1. The CLI built from the amalgamation gets `SIGINT` during a held call and prints the cancelled sentence within 100 ms. On the 503 arm a 150 ms deadline returns `thinkthen deadline` within 300 ms. | Run single calls on the calling thread. The 100 ms assertion turns red. |
| R2-10 SQLite half | partial | `-1` answers. `0` returns `thinkthen deadline` with zero counted requests. `-2`, `4294967295001`, `1.5`, `'100'`, and NULL raise `usage`. On the held arm a 200 ms deadline returns `thinkthen deadline` within 300 ms with one counted send. | Treat any negative as none. `-2` answers. Drop the tick's deadline check, and the held call waits for the release. |
| R4-6 SQLite half | closed | A fifo and a symlink to a fifo each refuse at the door within 1 s. A symlink to a regular file reads its target. | Open without `O_NONBLOCK`. The fifo open blocks, and the 5 s child timeout fires. |
| R6-14 SQLite half | closed | The map retires, and the guard stays. Warm under model A, then `thinkthen_decide` under model B, counts one new request. `thinkthen_annotate` with field `kind`, then the same question under field `topic`, returns an object keyed `topic`. | Add a binding map keyed by question text and evidence. Model B is served model A's answer. |
| R3-30 SQLite half | not re-probed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with the reason, and the three counts sum to the file's count. `rank` and `find` report "not run: no SQL form". | Skip one case silently. The sum check fails. |
| R1-30 and R3-31 SQLite half | closed | `check.sh` builds with `--remap-path-prefix` for `$HOME`, and `strings` finds no home path in the built library. | Drop the remap. The count turns nonzero. |
| R3-29, R4-19, and R5-34 SQLite half | open, closed, closed | Every `cargo` call in `check.sh` passes `--locked` and `--offline`. A `check.sh` step reads the script and fails on a call without them. | Drop `--offline` from the build. The step fails. |
| R1-31 | waive | One kind table. A Rust unit test maps each `ErrorKind` to its prefix and result code. | Map `Deadline` to `backend`. |
| R2-31 | partial | One panic guard. A `check.sh` step counts one `catch_unwind` site in `src`. | Add a second guard. |
| R2-29 SQLite half | open | The rulings that live only in the tag's `NOTES.md` land in a short SQLite section of ADR 0047: the 3.50.0 floor and why, volatile and direct-only, `'@name'` resolved against the process directory with symlinks followed, warm as decide-only, relate from rows, and the `foldhash` license exception. | Not code. The design reviewer checks the section. |
| R1-10 host half | engine | Unit test: the guard runs a closure that panics and returns `thinkthen defect:` with the panic text. A second guarded call then answers. | Remove the guard. The panic fails the test. |
| R1-11 host half | engine | `9223372036854775807` and `1e300` as the deadline raise `usage` with zero counted requests. | Add the milliseconds to `Instant::now()` in the shim. The call panics, and the guard reports `defect`. |

Handed on or retired, with reasons: the SQLite half of R1-29 (the wire stub under `tools/`) retires with 0092's loopback backend. The SQLite halves of R5-37 and the `package.sh` half of R4-19 go to the release ticket with `package.sh`. R5-42 and R6-2 are standin-only in the port guide. Here the check's not-run line names the one fetch, and 0093's rung refuses a surface with no check. The macOS rows R3-32 and R4-10 wait for a Mac run, which this ticket excludes.

Not closed here: G9 (a Python wheel and this extension in one process) waits on ADR 0047 item 5. The SQLite page states whichever answer Ian gives.

## Other acceptance

- Red first: the ported tests fail against an empty `databases/sqlite` workspace for the stated reason, then pass.
- The slide runs as drawn in the CLI built from the amalgamation: warm 5, the WHERE keeps the complaint rows, and the count agrees. The query after the warm counts no new request.
- The volatile check stays: `pragma_function_list` shows eight names, fourteen registrations, and none with flag `0x800`. An index expression over `thinkthen_decide` refuses with `unsafe use of thinkthen_decide()`.
- Case 18 (cancel mid-batch): a 200-row warm at width 8 on the held arm is interrupted once the count reads 8. After 300 ms the count still reads 8. The test releases the arm, and the statement ends with `thinkthen cancelled`.
- `cancel_fast.py` keeps its stated limit: on a fast backend SQLite's own step loop may stop first. That test asserts only the stop time.
- The worker's two edges each get a Rust unit test: a send on a closed channel is ignored without a panic, and a channel closed with no result returns `defect`.
- The conformance runner runs every applicable case through the 0092 case arm in one child process per case, since the default engine reads its address once per process. Digests are recomputed. No backend arm is added.
- Case 68 (an accent and an emoji) returns the same `start` and `end` through `thinkthen_recognize` as through Rust, and `substr(text, start + 1, end - start)` returns the name.
- A relate test proves the id mapping: two rows share one name and kind, and each edge appears once for each of them.
- The built library exports exactly one dynamic symbol, `sqlite3_thinkthen_init`, per `nm -D --defined-only`. Planted: a second `#[unsafe(no_mangle)]` function fails the step.
- `cargo test --lib --locked --offline` in `databases/sqlite` passes.
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset. A secrecy test reads every error message from every function for the key and for the base URL's credentials.

## Where the check runs

The tag's check ran and passed at the freeze, on its second run. It needed a SQLite amalgamation and the stock `sqlite3` CLI, both copied from a local worktree with no download (`sdlc/records/surfaces-freeze-2026-09-24.md` at the tag). Nothing in it needs Docker.

The port runs on Beelink, this Linux machine (Ubuntu 24.04, x86_64, stock SQLite 3.45.1, Python 3.12.3, rustc 1.93.1, gcc), offline and with no container. Its one outside input is the SQLite 3.50.0 amalgamation. `check.sh` reads it from `$SQLITE_AMALGAMATION`, or from the gitignored `databases/sqlite/.runtimes/sqlite-amalgamation-3500000` when that is unset. It checks two pinned SHA-256 values: `sqlite3.c` `76d2bc5ce0038e5805f420f4ce6abde81ab880aac122b930923f4411dd84aaec` and `shell.c` `ab201c205a3adf299faec65732f18815174df64e398e6c376e42ca7954509cb1`. Five local worktrees hold byte-identical copies, `worktrees/thinkthen-surfaces` among them. The zip on sqlite.org is the one fetch for a networked machine.

`tests/host_sqlite.sh` builds two things once into `.runtimes/host`, keyed by those hashes. `libsqlite3.so.0` serves Python through `LD_LIBRARY_PATH`. `sqlite3` is the CLI built from `shell.c`, and it replaces the downloaded 3.53.4 tools bundle. Measured here on 2026-09-24: the library built in 18.7 s and the CLI in 23.3 s. Python's `sqlite3` then reported 3.50.0 and allowed extension loading. The CLI reported 3.50.0. Both hosts sit at the floor, which is the version the schema proofs need.

A missing or mismatched amalgamation, a missing `cc`, or Python below 3.10 reports "not run" and never "pass" (R6-2). The line names the copy or fetch to run. A cargo cache that lacks a locked crate reports the same way. The floor-refusal half of R2-1 needs a host below 3.50.0. The stock 3.45.1 library serves here. A machine without one reports that half as not run.

## The check it adds to the gate ladder

- `databases/sqlite/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047, the fifth rung after `spec`) runs it with the 0092 loopback port. Each Python script runs under `timeout 300`, and the whole check finishes within 10 minutes here from a warm cargo cache.
- `lint` runs on `databases/sqlite`: the ADR 0047 manifest, lock, lint-table, and profile checks; deny as `cargo deny --offline --manifest-path databases/sqlite/Cargo.toml check --config databases/sqlite/deny.toml advisories bans licenses sources`, planted with a git-sourced dependency that `[sources] unknown-git = "deny"` refuses; `ratchet.mjs` on `ratchet.json` for Rust, `ratchet.py.json` for the Python tests, and `ratchet.sql.json` for SQL; and the registry check.
- The binding needs its own deny file. Measured here with cargo-deny 0.19.4: `rusqlite` 0.40.2 pulls `hashlink` 0.12.2, then `hashbrown` 0.17.1, then `foldhash` 0.2.0. Its license is `Zlib`. The root `deny.toml` does not allow `Zlib`. The root cannot add it either, because `unused-allowed-license = "deny"` refuses a license its own tree never uses. `databases/sqlite/deny.toml` equals the root file plus one exception: `foldhash` may use `Zlib`. A `lint` check compares the two files and refuses any other difference. Zlib is OSI-approved and permissive, with no copyleft term. This is a decision Ian can overturn. The lever is dropping `rusqlite` for raw `libsqlite3-sys` calls, which would cost the aggregate and vtab glue the tag relies on.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, and `unsafe_code`, and it denies `expect_used`, `unwrap_used`, `indexing_slicing`, and `panic`. A local `allow` cannot lift a forbid. `rusqlite` has no procedural macros, so every trip is in hand-written code. The vtab and cursor types get hand-written `Debug`, since the `sqlite3_vtab` base has none. The tag's `unwrap` on locks and `expect` on thread spawn get rewritten. If a forbid-level lint still cannot be met, the builder stops and records the case for an ADR 0047 amendment.
- `check.sh` drops `ENGINE_NULL`, `THINKTHEN_NULL`, the `synthetic-partial` feature, and the stub probe on port 8218.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off. `rusqlite` `0.40.2` with `loadable_extension`, `functions`, and `vtab`. It brings `libsqlite3-sys` `0.38.2`. `libc` `0.2.189` for the pin and the file door. `serde_json` `1.0.151` to compose the recognize and relate JSON. `libc` and `serde_json` must match the root lock (ADR 0047 item 1). Each is the tag's version and sits in this machine's cargo cache.
- New to main through `rusqlite`: `hashlink` 0.12.2, `hashbrown` 0.17.1, `foldhash` 0.2.0, `fallible-iterator` 0.3.0, `fallible-streaming-iterator` 0.1.9, `smallvec` 1.16.1, and `pkg-config` 0.3.34, with wasm-only crates in the lock.
- Build tools: `cc` for the host library and CLI. Tests: Python 3.10 or later, standard library only. No `pytest`, `uv`, or `maturin`.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, deny's result, and the `foldhash` exception, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust: at most seven files and 1,500 nonblank lines, each file under 500. The tag's `src/lib.rs` has 2,089 nonblank lines, and its production part ends before line 1,835. The map, the hub, and the stand-in glue leave. The worker wait, the relate entity reader, and the lint rewrites come in.
- Rust unit tests: at most 350 nonblank lines.
- Python tests: at most nine files beside one helper, and 2,000 nonblank lines with the helper. The conformance runner is at most 300 more. The tag's test and tool scripts measure 2,248 together before `swap_hammer.py` and the stand-in cases leave.
- Scripts: `check.sh` and `tests/host_sqlite.sh` together at most 220 nonblank lines. The tag measures 144. The CLI build, the hash checks, the symbol, guard-count, flag, and home-path steps, and the not-run lines come in. Gate changes under `sdlc/scripts` at most 40 nonblank lines.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/databases/sqlite.md`, and the ADR 0047 section, at most 300 net nonblank lines.
- Ratchet: the three binding ratchet files each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen`, or adding a backend arm.

## Exclusions

Packaging, `package.sh`, release artifacts, and installs (queue item 4). A macOS run. Any change to `thinkthen`. SQL forms of `filter`, `rank`, and `find` beyond the `WHERE thinkthen_decide` pattern. A relate call over query text. A setting function for the cache. The site's example data. Any live or paid call.

## Dependencies

After 0086 lands. Also after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), 0096 (fork recovery), and 0094, since the plan puts C before every other surface and this ticket copies its FFI edge. 0092 and 0099 have landed. No other surface ticket blocks this one.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. An interrupt crosses SQLite's connection and a detached worker. A schema in someone else's file must never spend money.

## Review

- Design review: pending.
- Code review: pending.
