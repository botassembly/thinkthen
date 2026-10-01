# Test harness and review leftovers

Status: Closed on 2026-09-28 after fresh independent Medium code review of `dcec83f0`. The reviewed command no-connection correction settles the last concrete item 1 proof gap. The old blanket raw-socket removal criterion is a non-issue: the retained sites protect distinct raw-wire, synchronous no-connection, arrival and held-reply behavior. [The closeout assessment](../../records/0119-harness-closeout-preflight.md) and [final build/review record](../../records/qf-command-no-connection-proof.md) give the evidence. The wider 0119 engine audit remains separately open.

Prior settled work: Ticket 0127 settles items 2, 3, and 10. It lands the item 4 fix. Item 4's sweep passed 50 runs of 50 at normal load, recorded in `sdlc/records/0127-build-test-harness-fixes.md`. The old high-load repeat demand is historical and separately opt-in under Ian's later functional-gate ruling; it is not a pending ordinary gate or a claim of a high-load pass. Ticket 0133 settles items 6, 11, 12, and 13 on 2026-09-26. Ticket 0138 settles item 5 on 2026-09-26. Quick Fix qf-h1-h3-h6 settles items 7, 8, and 9 on 2026-09-26.

Ian's ruling, 2026-09-25, on item 10: nobody asks the rusqlite maintainers for a fix. Keep the hand-extended API table and add a test.

This issue merges six files: `2026-09-25-test-harness-and-review-leftovers.md`, `2026-09-25-test-harness-and-review-leftovers.md`, `2026-09-25-test-harness-and-review-leftovers.md`, `2026-09-25-test-harness-and-review-leftovers.md`, `2026-09-25-test-harness-and-review-leftovers.md`, and `2026-09-25-test-harness-and-review-leftovers.md`. Each item is small, and each came out of a review or a test run. Most touch the test harness: the shared loopback backend, the secrecy sweep, and the demo checks. The rest are review leftovers that still stand on main. Every item below was checked against main on 2026-09-25. Ticket 0119, the mutation audit of the engine tests, runs after the surfaces land. It should take item 1 alone. Items 2 to 10 are separate work, and items 2, 3, and 4 should land before 0119 starts, because 0119 leans on the secrecy sweep and the shared backend.

## 1. Hand-rolled loopback listeners duplicate the shared backend

Kind: cleanup. Owner: ticket 0119.

The bounded `engine/width_tests.rs::serving` helper replacement landed at reviewed source `b89c58f7`. Its two permit/replay tests remain and passed before and after. The remaining sites retain distinct functional assertions under the reviewed closeout assessment. The command form/probe/runner paths now use synchronous destination-matched socket checks at reviewed source `dcec83f0`; their prior asynchronous zero count could miss a queued connection. See `sdlc/records/0119-listener-build.md`.

Preparation update, 2026-09-27: this item describes a wider listener-helper cleanup, not an automatic deletion under 0119's proposed first deadline-test slice. Its connection-count and arrival-order assertions need exact replacement proofs, and `conformance/backend` needs a separate file claim before edits. The historical line references below may have moved on current main.

### Original investigation and requested remedy (superseded)

Ticket 0092 moved the loopback listener into `conformance/backend`, so every test can share one backend. Eleven test helpers still open their own `TcpListener`:

- `crates/thinkthen/src/cli/conformance_tests/command.rs:377` `serve_once` answers one request. `Listener::serving` gives that.
- `crates/thinkthen/src/cli/conformance_tests/command.rs:199` `counting` counts connections and answers none.
- `crates/thinkthen/src/engine/deadline_tests.rs:120` `serve` scripts a 503 with a wait, an answer, a hold, and an arrival signal. `Canned::status(..).asking(..)`, `Canned::after_release`, and `Listener::answering_with_events` cover it.
- `crates/thinkthen/src/engine/deadline_tests.rs:191` and `:214` open listeners nothing may reach.
- `crates/thinkthen/src/cli/edge/deadline_tests.rs:40` `spent` opens a listener nothing may reach.
- `crates/thinkthen/src/engine/width_tests.rs:236` `silent` opens a listener nothing may reach.
- `crates/thinkthen/src/engine/width_tests.rs:329` `serving` writes raw responses in order and announces each.
- `crates/thinkthen/src/engine/host_signal_tests.rs:26` `held_reply` holds a reply until released. `Canned::after_release` covers it.
- `crates/thinkthen/tests/audit_refusals.rs:339` and `crates/thinkthen/tests/transform.rs:324` open listeners nothing may reach.

A listener nothing may reach needs only `Listener::connections`. `serve_script` does not count connections today (`conformance/backend/src/listener.rs:318`), so `Listener::serving` needs that count first. The transport tests in `crates/thinkthen/src/engine/http.rs:431` keep their raw sockets, because they test the bytes on the wire. `crates/thinkthen/tests/public_batches.rs:313` only takes a free port and keeps it. A new `Canned` builder edits `conformance/backend`, so the ticket must list that folder in its `opens`.

Fix: move each helper above onto `conformance_backend::Listener` or `Canned`. Each moved test must still turn red on its old plant. 0119 rewrites these same engine test files, so it takes this item.

Retired criterion: `grep -rn TcpListener crates` finds only `engine/http.rs`, `public_batches.rs`, and the clippy rule.

## 2. The loopback reset reply has two silent edges

Settled by ticket 0127. `conformance/backend/tests/listener.rs` pins both edges.

Kind: real test gap.

Found by the 0089 code review (`sdlc/records/0089-code-review.md`, FU3). The code now lives in `conformance/backend/src/listener.rs`. No test hits either edge.

- A client that sends part of a request and closes is never seen as closed. `peek_request` (`listener.rs:419`) peeks, finds no whole request, sleeps a millisecond, and peeks the same bytes again (`listener.rs:421` to `:434`). The serving thread spins until the test process exits.
- A request of 32 KiB or more is read in full instead of peeked (`listener.rs:430`), and `used` comes back 0. A `Canned::reset()` reply at `listener.rs:329` or `:391` then drops a drained stream. The client sees a plain close instead of a reset.

Fix: make the reset path panic when `used == 0`. Treat an unchanged `seen` across polls, followed by a zero-length `read`, as end of file.

Done when: a backend test sends half a request and closes, and a test resets a 32 KiB request. Both fail loudly on the old code.

## 3. The secrecy sweep marks only the first entity

Settled by ticket 0127. The sweep marks the second entity's name and kind.

Kind: real test gap.

Found by the final review of ticket 0088 (`sdlc/records/0088-review-final.md`, F2). `evidence` in `crates/thinkthen/tests/backend/secrecy.rs:62` builds every relate input from the marker entity and a plain `Acme` of kind `record` (lines 66, 70, 72, 73, 74). `nothing_leaked` (`secrecy.rs:143`) looks only for `EVIDENCE`. `secrecy_relate.rs:26` does the same with `Ada`. A relate command that prints the second entity's name or kind passes every secrecy test. A planted `eprintln!` of `Acme` passed all five secrecy tests at 0088.

Fix: give the second entity its own marker name and a marked kind. Make `nothing_leaked` refuse both markers in every diagnostic.

Done when: a planted print of the second entity's name or kind fails the secrecy tests.

## 4. The secrecy relate case fails under load

Fix landed by ticket 0127. The scripted listener keeps its port and counts connections, and the sweep's failure names what the listener saw.

Kind: flaky test.

Filed by Quick Fix `qf-heavy-lock` (`sdlc/records/qf-heavy-lock.md`). `no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` (`secrecy.rs:318`) failed once for case `a-record-run-relate---lines []`. The command exited 4 with "the backend refused the connection". The status check sits at `secrecy.rs:235` today. The one-minute load was 120. Nobody has reproduced it on an idle machine.

The old issue guessed the listener was not yet accepting. That guess is wrong. `Listener::serving` binds before it returns (`listener.rs:197`), and the command spawns after (`secrecy.rs:180`). A transport failure is never retried since commit `3686414f`, so the command made one connection. `serve_script` answers the first connection from anyone and returns when its script runs out, which closes the port (`listener.rs:318` to `:337`). A stray connection from another test to a reused port would take the one scripted reply. The case's own command would then find the port closed. This is the likely cause, and it is unproven.

Fix: have `serve_script` keep the port open after its script ends and record each extra connection. Make the sweep's failure message print the requests and connections the listener saw.

Historical 2026-09-25 completion demand: the sweep passes 50 runs in a row under the heavy lock at high load, or a failure names the stray connection. Under Ian's later ruling, that repeated high-load campaign is opt-in. Ticket 0127's functional listener fix and the retained secrecy sweep are the ordinary proof; this issue does not claim a high-load pass or close the umbrella.

## 5. `--field` and `--options` echo a pointer with control characters

Kind: real test gap. The code lacks the check as well.

`Failure::Pointer` prints the pointer as typed: `crates/thinkthen/src/cli/failure.rs:216` formats `{option} \`{typed}\``. The callers are `cli/find.rs:35`, `cli/annotate.rs:181`, and `cli/judge.rs:281`. A pointer that holds a terminal escape reaches standard error unchanged. Labels, options, and table headers already refuse control characters (`core/question.rs:38` to `:44`, `cli/table.rs:220`).

Fix: refuse a pointer that holds a control character before it is echoed, with the same check the labels use.

Done when: `--field $'/a\e[31m'` exits 2 and standard error holds no escape byte.

## 6. `EntryError::Unwritable` cannot fire

Kind: cleanup.

`crates/thinkthen/src/core/recording.rs:45` declares `Unwritable`, and `recording.rs:159` maps a `serde_json::to_string_pretty` failure to it. The entry holds only strings and raw JSON, so that call cannot fail. The other half of the old item is fixed: see Already fixed.

Fix: remove the variant if a caller can prove the call infallible without an `unwrap`. Otherwise record in `recording.rs` why it stays.

Done when: the variant is gone, or a comment at `recording.rs:45` says why it stays.

## 7. The demos script cuts a `--replay` folder name at a space

Settled by Quick Fix qf-h1-h3-h6 on 2026-09-26. `sdlc/scripts/demos` refuses a `--replay` folder name with any character outside letters, digits, and `. _ / -`, and names the text. `demos-self-test` case `replay-spaced` pins it.

## 8. Demo 27 copies `triage.sh` into a block nothing checks

Settled by Quick Fix qf-h1-h3-h6 on 2026-09-26. A `bash` block prints `triage.sh` from `set -eu` down and pins every line, so an edit to the script alone turns the demo red.

## 9. A `set +e` demo block can hide an earlier failure

Settled by Quick Fix qf-h1-h3-h6 on 2026-09-26. `sdlc/scripts/demos` refuses `set +e` in any demo `bash` block, and `demos-self-test` case `set-e-off` pins it. Demos 19 and 27 keep `set -e` and capture each code with `&& rc=0 || rc=$?`. The pages under `spec/` still use `set +e` and sit outside this check.

## 10. rusqlite's loadable bindings stop at SQLite 3.34

Settled by ticket 0127. `databases/sqlite/tests/test_interrupt.py` records Ian's ruling and fails on a wrong tail count.

Kind: cleanup. The ruling is Ian's.

Found by experiment 207 (local experiment 207's `sqlite/NOTES.md`). rusqlite 0.40.2 with `loadable_extension` routes SQLite calls through `libsqlite3-sys`, whose API table ends at SQLite 3.34. `sqlite3_is_interrupted` arrived in 3.41.0. The SQLite surface polls it every 50 ms while a call waits (`databases/sqlite/src/worker.rs:7`).

The landed surface does not link the host library, which the old issue expected. `databases/sqlite/src/ffi.rs:21` to `:30` extends the API table by hand with thirteen opaque tail pointers and reads `is_interrupted` from the host's own table. The floor is SQLite 3.50.0 (`ffi.rs:37`). Every later call needs the same hand extension, and a wrong tail count reads the wrong pointer.

Options for Ian:

- Keep the hand extension. It costs nothing now. Each new post-3.34 call adds a field and a count to check.
- Ask rusqlite to refresh the loadable bindings. This is outward-facing and spends a maintainer's time. It removes the hand table once a release ships.
- Both. Keep the extension and ask upstream.

Recommendation: keep the hand extension and add a test that loads the extension on the floor version and reads `is_interrupted` through it. Ask upstream only if a second post-3.34 call is needed. Nobody opens an upstream issue without Ian's word.

Done when: Ian rules, and the choice is recorded in `databases/sqlite` or an ADR.

## 11. The interrupt handler ships a failure switch in the release binary

Kind: cleanup. Found by experiment 218, wave 2, at main `20e9b8d4`.

`crates/thinkthen/src/cli/interrupt.rs:181` calls `Self::start_with(cancel, path, [false; 4])`. `start_with` takes `failures: [bool; 4]`, which it unpacks as block, spawn, readiness, and restore failures, and `restore` at `:261` takes an `injected_failure` flag. Only tests pass `true`. The switches compile into every release build. The engine's own `Injection` sits behind `#[cfg(test)]`, so the command breaks a rule the engine keeps: no test fixture in product code. A later edit that passes a wrong array would fail a user's Ctrl-C silently.

Fix: move the four switches behind `#[cfg(test)]`, or test the failures at the real boundary and drop the switches.

Done when: a release build of `interrupt.rs` holds no failure switch, and the interrupt tests still cover the four failure paths or name the boundary that does.

## 12. Three surface checks break under an absolute `CARGO_TARGET_DIR`

Kind: cleanup. Found by experiment 218, wave 2, at main `8391153d`.

With `CARGO_TARGET_DIR` set to an absolute folder, the `surfaces` rung fails Ruby, DuckDB, and PostgreSQL for path reasons alone. `libraries/ruby/check.sh:98` builds `$repo/${CARGO_TARGET_DIR:-target}/debug/conformance-backend`, which doubles an absolute path. `databases/duckdb/check.sh:50` packages `target/release/libthinkthen_duckdb.so`, and `databases/postgresql/check.sh:29` reads `target/release/thinkthen-pg16`. Both ignore the variable while cargo builds elsewhere. The same rung with the variable unset passes all three. A contributor with a shared target folder sees three red surfaces and no product fault.

Fix: resolve the target folder once, from `cargo metadata --format-version 1 --no-deps` or from the variable when it is absolute, and use it in all three scripts.

Done when: the `surfaces` rung passes with `CARGO_TARGET_DIR` unset, relative, and absolute.

## 13. The children check keeps pending entries for landed tickets

Kind: cleanup. Found by experiment 218, wave 2, at main `3f7b51ac`.

`sdlc/scripts/children` holds five `PENDING` entries. One names tickets 0123 and 0126 (`crates/thinkthen/tests/relate_edge.rs`, a spawn of `relate --help`). Four name ticket 0129 (`databases/postgresql/src/files.rs`, `databases/duckdb/tools/databases_suite.py`, and two in `databases/duckdb/tools/source_checks.py`). All three tickets have landed. Every named line is still there, so each child still inherits the whole environment, and the check stays green. Ticket 0127 line 290 said "0129 clears its pending entries, or a Quick Fix does after it lands." Neither happened. A pending entry with no open owner hides the leak the check exists to catch.

Fix: give each of the five spawns an allow-listed environment through its language's child helper, and delete the entries. Or move an entry to `EXEMPT` with a reason that still holds.

Done when: `PENDING` names no landed ticket, and `children` stays green.

## 14. The host-signal test failed once beside other signal tests

Status: fixed by Quick Fix `qf/isolate-host-signal-tests` on 2026-09-27. At HEAD `22332a38` with the uncommitted 0169 command changes, `cargo test --locked -p thinkthen --lib signal -- --nocapture` selected six tests: four passed, one failed, and one was ignored. `engine::host_signal_tests::a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole` failed at its `host_signal_tests.rs:98` assertion with `the worker kept SIGUSR1 blocked`. Running that exact test once in isolation passed in 0.10 seconds. No engine or worker source had changed in that lane.

Neither run redirected a raw log. The builder transcribed the tool results and their provenance into `target/codex-builds/0169/host-signal-observation.md` in the `thinkthen-codex-1` worktree; the 0169 build record also names the observation. At that point, it was not a clean-main reproduction and the isolated pass did not establish a root cause. Parallel signal interference and a timing-sensitive assertion remained hypotheses.

Source inspection identified a concrete interference path: `engine/facade_tests.rs::a_host_signal_on_the_calling_thread_never_fails_a_single_send` registers a process-wide SIGUSR1 flag and sends SIGUSR1 to an unblocked calling thread. `engine/host_signal_tests.rs::a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole` registers another process-wide flag for the same signal, then asserts its flag stayed false because its own worker blocked SIGUSR1. The installed signal-hook 0.4.4 `flag::register` registers a callback that sets the flag whenever that signal is handled; it does not filter by sending test or target thread. If these tests overlap, the caller test can set the worker test's flag even when the worker mask is correct. The bounded proof below confirmed that interference path.

The Quick Fix ran one coordinated proof: the engine worker had SIGUSR1 blocked, yet a signal sent to an unblocked caller set both registered flags. It then removed that temporary proof, ran the ordinary worker test in a bounded child process, and kept the worker mask, caller mask, one request, and good reply assertions. Both affected tests passed individually and together once. The build record gives the exact checks. This closes item 14 only; the other cleanup items remain open. The older 50-run high-load request in item 4 remains outside the current functional gate under Ian's opt-in ruling.

## Already fixed

- rusqlite's second trap, the workspace feature clash with `load_extension`: ADR 0047 gives each binding its own workspace. The root `Cargo.toml:4` to `:6` excludes `databases`.
- Early leftover 1, decode errors carrying a wire name `String`: `DecodeError` now carries the place number and renders `wire_name` in its message (`core/adapters/systemone.rs:94` to `:126`).
- Early leftover 2, public core items with no outside caller: `core` is a private module (`src/lib.rs:13`) and holds no `pub` item.
- Early leftovers 3, 4, 5, and 6, the `Reply` triple, the owned encode strings, `EncodeError`, and the newtype repetition: each item recorded a choice to keep the code as it is. No change is wanted.
- Early leftover 7, no cap on standard input: `cli/edge.rs:25` and `:201` read at most `MAX_RECORD_BYTES` plus two bytes per record.
- Early leftovers 8 and 22, retries of a bad body and of a refused header: commit `3686414f` stops every retry after a transport failure (`engine/http.rs:3`).
- Early leftover 9, a defect and a render failure sharing exit 70: `specification/channels.md:59` makes 70 the defect code, and `cli/failure.rs:264` reports a render failure as a defect. The code matches the specification.
- Early leftover 11, second half: `EntryError::Schema` no longer echoes the file's schema (`core/recording.rs:30` to `:37`). The first half stays open as item 6.
- Early leftover 12, `record_and_replay.rs` repetition: commit `a96ce4f0` folded that file into `tests/backend/`. The wrong-assertion fixture is now five lines (`tests/fixtures/demos-wrong/README.md`).
- Early leftover 13, `backend.rs` at 411 lines with a two-job doc line: `core/backend.rs` holds 227 lines of source before its tests, and its first doc line names one job.
- Early leftovers 14, 15, 16, and 17, the ledger variable, the backtick guard, the harness suppression, and the 0006 record numbers: commit `a96ce4f0`.
- Early leftover 18, `--url ""`: `core/backend.rs:129` refuses a blank address, and the test at `core/backend.rs:430` pins it.
