# 0096: Build fork recovery

Status: built on `ticket/0096-fork-recovery`; code review pending. Owner: Claude.

Base: main `ba6864d4` with 0085 and 0097, merged at `5d317098`. Code commits `09359a3a` and `147e7f78`.

## Result

`engine/process.rs` holds one private `Guarded<T>`: an owner process ID, a rebuild marker, and an `arc-swap` slot. `current(pid, wait, fresh)` reads the owner and the marker with atomics before it loads the slot.

- When the owner is the caller's process, it returns the published state.
- Otherwise one caller of that process wins the marker by compare-and-swap. It checks the owner again, builds fresh state, swaps it in, leaks the inherited state with `mem::forget`, and then publishes itself as owner with release ordering.
- Other callers of the same process call `wait` between looks at the marker, and never wait on a lock. A marker naming another process belongs to a rebuild that died with a fork, and the caller takes it over.
- A failed or unwinding build clears the marker, so the next call builds.
- An ordinary drop frees the state. Only a replacement leaks.

Two values use it:

- The process width state. `PROCESS_WIDTH` is now a statically initialized `Guarded<&'static Widths>`. Its first use builds the state as a child would. A forked child gets fresh, unselected width state and an empty gate. Each process leaks one width state, just as the static it replaces was never dropped.
- Each engine's `State`: the HTTP pool, the recorder, the counters, and the width its calls follow. `Engine::state(cancel)` is the one door. Every call enters it before key lookup, recording, counting, width acquisition, pool access, dispatch, or scheduler construction, because each of those needs the state it returns. `judge`, `find`, `ask_chunks`, `records`, `groups`, and `usage` go through it. `split` and the pure planning in `recognize` and `relate` touch no state. A child's fresh state gets new counters over the same usage folder, a new recorder, and a new pool whose gate is the child's width state. The engine then selects its width again under ticket 0077's rule. Waiting callers observe the call's cancellation and deadline through `Cancel::wait`.

The engine keeps its immutable settings outside the state: backend, profile, timeout, retries, key reader, the explicit-or-omitted width, the storage folders, the usage folder, and whether a caller-named folder is in use. A child reads only these before its fresh state exists.

The recorder no longer keeps its folder gate between calls. Main held a `Mutex<Option<FolderGate>>` for the recorder's life. The shared folder lock is now opened per operation and held by the `WritePermit` until installation. A replay drops it after the read. This matches `specification/recording.md` ("from its first lookup through replay or installation"). A leaked inherited recorder therefore keeps no directory lock open in the child. The backend identity is still checked once per recorder, behind an `AtomicBool`. The recorder has no lock left.

`Counters` keeps its usage folder as an immutable field. Its mutex now guards only the "still writing" flag across one durable update.

## Dependency

`arc-swap` 1.7.1 enters as the one allowed dependency. It has no dependencies of its own and is licensed MIT or Apache-2.0. `cargo deny check` passes, and `deny.toml` needed no change. The latest release, 1.9.2, pulls in `rustversion`, so the manifest pins `=1.7.1` to keep the tree to one new crate, and the lock file records the same version. Its default hybrid strategy loads through thread-local debt slots and writers pay readers' debts. It takes no userspace lock, and it never waits on another thread to progress, so a debt left by a vanished parent thread cannot block a child's swap. `const_empty` gives the static slot with no lazy initialization. `policy.py` accepts the crate in the direct dependency table. A second reviewer checks it under the repository's dependency rule.

## Tests

Both test files answer the tests-earn-their-place questions. They need no test-only export, flag, or hook in production code. A forked child is its parent's memory under another process ID. So each test builds state as one process ID and then calls as another, and it passes the ID as an argument. Production passes `std::process::id()` from `Engine::new`, `Engine::state`, and `process_width`.

`engine/process/tests.rs` holds the rule as an edge-case table and three more cases. Every call runs on its own thread and fails on `recv_timeout`.

- Table, seven rows: the owner keeps its state, a child builds, a child replaces the parent's rebuild marker, a grandchild replaces an older marker, a failed build is returned, the owner never waits on a marker, and a child waits on its own process's rebuild. Every row checks that the inherited state is never dropped. Each row that builds also checks that the marker is cleared.
- A failed build leaves the next call to build, and then the child owns its state.
- An ordinary drop frees the state.
- Eight racing children publish exactly one state, and all eight get the same one.

`engine/facade/fork_tests.rs` runs two proofs, each alone in a child copy of the test binary, because width state belongs to the process.

- Busy parent: the parent engine records to a folder, counts to a usage folder, and has explicit width 1. A parent thread sends through the parent's state and is held by the listener. It holds the only width permit, a live recording permit, and its folder gate. The child's `judge` must reach the listener while that permit is held. Then the parent counts 2 and the child counts 1. The durable total is exactly 3. After the engine drops, an exclusive folder lock succeeds within 5 s.
- Width: the parent selects width 2, and an implicit parent engine follows 2. In the child, the implicit engine follows the fallback 4 and leaves the selection unset. The explicit engine then selects 2. A later width 2 is accepted, and width 3 is refused with `WidthActive`.

`width_tests::in_child_at` runs a child test by its full path and waits through `test_deadline::finish`, which main's Quick Fix `qf-test-deadlines` added. `in_child` delegates to it, so the older width children get the same 60 s bound.

Removed: `recorder::tests::a_held_recorder_gate_ends_as_the_deadline_without_the_owner`. It held the recorder mutex, which no longer exists. `deadline_tests::a_held_folder_ends_as_the_deadline_without_the_owner` and `request::tests::a_held_folder_cancels_through_the_prepared_request_path` already hold the folder's exclusive lock and cover the same wait.

## Planted bugs

Each plant ran alone on `147e7f78` with `cargo test --lib -- fork_tests process::` and was reverted. Every one turned red.

| Plant | Result |
| --- | --- |
| R5-3: `Guarded::current` returns the published state before it compares the process ID, so the child's first call takes the parent's pool and its full width gate | red in `busy_parent_child`: "the child sent past the parent's full gate", 1 request where 2 were expected. Also red in the width child and in all five process cases |
| A child's fresh state reuses the parent's counters | red in `busy_parent_child`: counts (3, 3) where (2, 1) were expected |
| The inherited state is dropped in place of leaked | red in the table: strong count 1 where 2 was expected |
| Any rebuild marker makes a caller wait, including one a fork left behind | red in the table: "a child replaces the parent's rebuild" waited in place of building |
| The recorder keeps its folder gate between calls, as on main | red in `busy_parent_child`: the exclusive folder lock timed out after 5 s |
| The process width state is never rebuilt, because every process shares one owner ID | red in `child_width_child`: the implicit engine followed width 2 where 4 was expected. Also red in `busy_parent_child`: 1 request where 2 were expected |

The first R5-3 run showed an unbounded wait in `a_waiting_child_stops_with_its_call`, which used barriers. `147e7f78` moved it to channels with `recv_timeout`, and every plant then ran to a red result within its bounds. The code review later deleted that case, because table row 7 already covers a child waiting on its own rebuild.

## Code review fixes

The code review (`/tmp` review of `e685740b`, reply relayed by the coordinator) returned F1 to F6.

- F1: `an_ordinary_drop_frees_the_state` failed 6 times in 60 under load. The thread in `call` sent its answer before dropping its clone of the guarded state, so the test could still see two owners. `call` now drops the clone before it sends.
- F2: nothing held the per-request folder lock until the recording is saved. While the parent's send is held, `busy_parent_child` now checks that a non-blocking exclusive lock on the folder is refused. Plant: the write permit drops its folder lock once the write is prepared. Red at the new assertion.
- F3: `contract_tests::a_folder_of_another_backend_refuses_each_call` records to a folder with one backend, then asks twice through a long-lived engine for another backend. Both calls return `RecordingBackendMismatch`, and the listener reads only the first backend's request. Plant: the identity check is marked done before it runs. Red on the second call.
- F4: the manifest pins `arc-swap = "=1.7.1"`.
- F5 and F6: known limits below. F6 is also a dated note on the 0086 ticket.
- `a_waiting_child_stops_with_its_call` is deleted, because table row 7 covers it.

## Budgets

Measured with `git diff -U0 5d317098 HEAD -- '*.rs'`, nonblank lines:

- Production: 7 files (`cli/schedule.rs`, `engine/facade.rs`, `engine/http.rs`, `engine/mod.rs`, `engine/process.rs`, `engine/recorder.rs`, `engine/usage.rs`) plus the manifest. 265 lines added and 112 deleted. The deleted count includes the 24-line recorder test removed above. The budget is 14 files and 650 lines.
- Tests: 465 lines added and 25 deleted, under the 950-line budget. The code review fixes then added 34 test lines and removed 23, net 11.
- `cli/failure/tests.rs` sits at its 500-line ceiling. Its two `std` imports merged to make room for the new `Client::new` argument.

## Known limits

- F5: an engine dropped in a forked child before any call drops the inherited `State` and does not leak it. The ticket forbids dropping inherited state. The code review checked what that drop runs. `ureq` 3.4.2 has no `Drop` impls, dropping a `std` mutex does not lock it, the width state is a leaked `'static`, the recorder and counters have no `Drop`, and closing the child's copy of an idle socket leaves the parent's socket open. So nothing takes a lock or writes to a socket today. A later `Drop` impl on any of these would change that. Making `Guarded`'s drop leak when another process owns the state would close it.
- F6: request-owned file locks after a real fork. `cache_lock` releases a lock by closing its file and never calls `unlock`. A forked child keeps copies of the parent's open lock files for requests in flight. Under flock(2) such a lock stays held until every copy is closed or one copy unlocks. So a busy parent's digest lock or shared folder lock outlives the parent's release until the child exits. The ticket keeps request-owned locks request-owned, so 0096 does not change it. 0086's real-fork busy-parent proof may meet it. The fix is an explicit `File::unlock` before close. It is a dated note on the 0086 ticket.

## What the ticket did not foresee

- The test rule. The ticket asked for a `cfg(test)` fake PID source and a call-entry observer with a counter or phase signal at each boundary. Both are test-only hooks, which the tests-earn-their-place rule of 2026-09-24 rejects. The process ID is an argument to `Guarded::current` and `Engine::built_by`, and the tests pass another value. The order "PID before any state" holds by construction: every retained resource is reachable only through the `Arc<State>` that `Engine::state` returns, and `only_the_one_accessor_reads_the_retained_state` pins that door. Ian can overturn this and ask for the observer.
- Held-lock cases. The ticket named five held locks. The recorder mutex is gone, so it has no case, and the leak check covers the retained gate instead. The counter mutex is private to `usage.rs`. Holding it from a test needs a hook, so the fresh-counter counts cover it: a child that shares the parent's counters fails them. The width gate and a busy scheduler queue are held by a real parent send. The rebuild marker is in the table.
- The process width state is per process, not per engine. It needed its own guarded slot beside each engine's state, and `Client::new` now takes the width state it sends through. Eight test call sites changed with it.
- The recorder's retained folder gate had to move per operation (see Result). Without that, a child kept its parent's shared directory lock for its whole life, and a later `cache prune` waited on it.
- A reused process ID. The owner is a process ID, as ADR 0017 fixes. A grandchild can reuse the ID of an exited grandparent whose engine state its parent inherited but never used. That grandchild would then trust inherited state. Only an at-fork hook closes this, and the ticket excludes one. Recorded here as a known limit.
- `Engine::usage` now returns `Result`, because a child's first `usage` call may have to build state. It stays test-only until 0086 exposes it.

## Ladder

At `138c1def`, which merges main `5783815b` (with 0097 and `qf-test-deadlines`), the rungs ran one after another with the key and base-address variables unset. The one-minute load stood at 3.16 at the start. The observed exits were `install` 0, `lint` 0, `test` 0 (817 passed, 0 failed), and `spec` 0 (demos 21 green, 0 red). `ratchet.mjs` read 50970/50970. Main then moved to `e7696ca8`, which changes only `sdlc/planning/one-line-plan-2026-09-24.md`. The later merge and this record add no code.

After the code review fixes, the rungs ran again at `45e66de7`. That commit merges main `a2e5f1fa`, which has the throttle rename and the ticket evidence rule. The one-minute load stood at 9.92 at the start. The observed exits were `install` 0, `lint` 0, `test` 0 (817 passed, 0 failed), and `spec` 0 (demos 21 green, 0 red). `ratchet.mjs` read 50981/50981. Before that ladder, the four `process` tests ran 30 times, 10 copies at once, beside 8 busy loops at a load of 12.17. All 30 runs passed, including `an_ordinary_drop_frees_the_state`.

## Ratchet

The ceiling rises from 50276 to 50869, 593 lines. About 153 are net production code: the guarded slot, the engine's immutable settings and its one state door, and the per-operation folder gate. The other 440 are tests. Merging main at `5783815b` (ceiling 50378) gave 50970: main plus this ticket's 592 lines. The code review fixes added 11 net test lines, and main at `a2e5f1fa` changed no Rust source, so the ceiling is now 50981: main plus 603. Moving the child wait onto main's `test_deadline::finish` saved one line. Before adding lines, I deleted the recorder mutex, its try-lock loop, its poison path, and the recorder test that held it. `Guarded` serves both the process width state and each engine's state, so the rebuild rule exists once.
