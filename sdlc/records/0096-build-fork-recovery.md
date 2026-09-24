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

`arc-swap` 1.7.1 enters as the one allowed dependency. It has no dependencies of its own and is licensed MIT or Apache-2.0. `cargo deny check` passes, and `deny.toml` needed no change. The latest release, 1.9.2, pulls in `rustversion`, so the manifest pins `1.7.1` to keep the tree to one new crate. Its default hybrid strategy loads through thread-local debt slots and writers pay readers' debts. It takes no userspace lock, and it never waits on another thread to progress, so a debt left by a vanished parent thread cannot block a child's swap. `const_empty` gives the static slot with no lazy initialization. `policy.py` accepts the crate in the direct dependency table. A second reviewer checks it under the repository's dependency rule.

## Tests

Both test files answer the tests-earn-their-place questions. They need no test-only export, flag, or hook in production code. A forked child is its parent's memory under another process ID. So each test builds state as one process ID and then calls as another, and it passes the ID as an argument. Production passes `std::process::id()` from `Engine::new`, `Engine::state`, and `process_width`.

`engine/process/tests.rs` holds the rule as an edge-case table and three cases. Every call runs on its own thread and fails on `recv_timeout`.

- Table, seven rows: the owner keeps its state, a child builds, a child replaces the parent's rebuild marker, a grandchild replaces an older marker, a failed build is returned, the owner never waits on a marker, and a child waits on its own process's rebuild. Every row checks that the inherited state is never dropped. Each row that builds also checks that the marker is cleared.
- A failed build leaves the next call to build, and then the child owns its state.
- An ordinary drop frees the state.
- Eight racing children publish exactly one state, and all eight get the same one.
- A child waiting on its own process's rebuild stops when its call stops, and the builder still publishes.

`engine/facade/fork_tests.rs` runs two proofs, each alone in a child copy of the test binary, because width state belongs to the process.

- Busy parent: the parent engine records to a folder, counts to a usage folder, and has explicit width 1. A parent thread sends through the parent's state and is held by the listener. It holds the only width permit, a live recording permit, and its folder gate. The child's `judge` must reach the listener while that permit is held. Then the parent counts 2 and the child counts 1. The durable total is exactly 3. After the engine drops, an exclusive folder lock succeeds within 5 s.
- Width: the parent selects width 2, and an implicit parent engine follows 2. In the child, the implicit engine follows the fallback 4 and leaves the selection unset. The explicit engine then selects 2. A later width 2 is accepted, and width 3 is refused with `WidthActive`.

`width_tests::in_child_at` now bounds every child run at 60 s. `in_child` delegates to it, so the older width children are bounded too.

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

The first R5-3 run showed an unbounded wait in `a_waiting_child_stops_with_its_call`, which used barriers. `147e7f78` moved it to channels with `recv_timeout`, and every plant then ran to a red result within its bounds.

## Budgets

Measured with `git diff -U0 5d317098 HEAD -- '*.rs'`, nonblank lines:

- Production: 7 files (`cli/schedule.rs`, `engine/facade.rs`, `engine/http.rs`, `engine/mod.rs`, `engine/process.rs`, `engine/recorder.rs`, `engine/usage.rs`) plus the manifest. 265 lines added and 112 deleted. The deleted count includes the 24-line recorder test removed above. The budget is 14 files and 650 lines.
- Tests: 465 lines added and 25 deleted, under the 950-line budget.
- `cli/failure/tests.rs` sits at its 500-line ceiling. Its two `std` imports merged to make room for the new `Client::new` argument.

## What the ticket did not foresee

- The test rule. The ticket asked for a `cfg(test)` fake PID source and a call-entry observer with a counter or phase signal at each boundary. Both are test-only hooks, which the tests-earn-their-place rule of 2026-09-24 rejects. The process ID is an argument to `Guarded::current` and `Engine::built_by`, and the tests pass another value. The order "PID before any state" holds by construction: every retained resource is reachable only through the `Arc<State>` that `Engine::state` returns, and `only_the_one_accessor_reads_the_retained_state` pins that door. Ian can overturn this and ask for the observer.
- Held-lock cases. The ticket named five held locks. The recorder mutex is gone, so it has no case, and the leak check covers the retained gate instead. The counter mutex is private to `usage.rs`. Holding it from a test needs a hook, so the fresh-counter counts cover it: a child that shares the parent's counters fails them. The width gate and a busy scheduler queue are held by a real parent send. The rebuild marker is in the table.
- The process width state is per process, not per engine. It needed its own guarded slot beside each engine's state, and `Client::new` now takes the width state it sends through. Eight test call sites changed with it.
- The recorder's retained folder gate had to move per operation (see Result). Without that, a child kept its parent's shared directory lock for its whole life, and a later `cache prune` waited on it.
- A reused process ID. The owner is a process ID, as ADR 0017 fixes. A grandchild can reuse the ID of an exited grandparent whose engine state its parent inherited but never used. That grandchild would then trust inherited state. Only an at-fork hook closes this, and the ticket excludes one. Recorded here as a known limit.
- `Engine::usage` now returns `Result`, because a child's first `usage` call may have to build state. It stays test-only until 0086 exposes it.

## Ladder

LADDER

## Ratchet

The ceiling rises from 50276 to 50869, 593 lines. About 153 are net production code: the guarded slot, the engine's immutable settings and its one state door, and the per-operation folder gate. The other 440 are tests. Before adding lines, I deleted the recorder mutex, its try-lock loop, its poison path, and the recorder test that held it. `Guarded` serves both the process width state and each engine's state, so the rebuild rule exists once.
