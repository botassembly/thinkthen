# 0096 code review: fork recovery

Reviewer: fresh Claude session (Opus 5.5), read-only. Branch `origin/ticket/0096-fork-recovery` at `e685740b`, which contains main `e7696ca8`. I read the ticket, its design re-review, the 0096 build record, the landed 0078, 0085 and 0097 records, the repo `CLAUDE.md`, and decision `2026-09-24-tests-earn-their-place.md`. I worked in a `git archive` copy at `/tmp/claude-1000/r0096` with its own target folder, built with `-j 4`, and deleted both afterwards. The only change I made to the repo was one `git fetch` of remote refs.

## Verdict

**Findings.** The design is sound, and the production code does what the ticket asks. Two items block landing, and both are small:

- F1: one new test fails intermittently under load.
- F2: the recorder's new per-request folder lock has no test that holds it until the recording is saved.

F3 and F4 should be fixed in the same pass. F5 and F6 are notes to record.

## (1) Correctness against the ticket

`Guarded::current` (`engine/process.rs`) meets the ticket's rule:

- It reads the owner and the rebuild marker with atomics before it loads the slot.
- One caller of a process wins the marker by compare-and-swap.
- The winner checks the owner again. That closes the window where a late thread wins a marker another thread has just cleared.
- The winner builds fresh state, swaps it in, leaks the inherited state with `mem::forget`, and only then publishes the owner with release ordering.
- The `Idle` guard clears the marker only if it still names this process, so a failed or unwinding build leaves the next call free to build.
- Waiters call `rebuild_wait`, which uses `Cancel::wait`. That sleeps and checks the call's stop and deadline, and it takes no lock.

I traced the order of operations by hand and found no race inside one process. The owner is stored only after the swap, and the marker clears only after the owner is stored.

The engine side also holds:

- Every call that uses state enters `Engine::state(cancel)` first: `ask`, `ask_chunks`, `records`, `groups` and `usage`. Key lookup, recording, counting, the width gate, the pool and scheduler construction all sit behind the `Arc<State>` it returns.
- The immutable settings stay outside the state as plain data.
- A child's fresh state gets new counters over the same usage folder, a new recorder, and a new `Client` whose gate is the child's own width state.
- The child then selects its width again under ticket 0077's rule.
- `PROCESS_WIDTH` is statically initialized with `ArcSwapOption::const_empty`, so no lazy mutex is involved.

**`arc-swap` in a forked child.** I read the 1.7.1 source for the default hybrid strategy:

- `Node::get` claims a debt node by compare-and-swap or pushes a new node onto the list. A node that a vanished parent thread left in use or cooling down is skipped. The child never waits for it.
- `Debt::pay_all` and `helping::help` loop only on compare-and-swap. `help` produces a replacement value for a stuck reader and does not wait for that reader.
- The only lock in the crate is the optional `RwLock` strategy, which is not used here.

So the builder's claim holds: a swap in the child cannot block on debts left by parent threads. A replacement handed to a vanished reader's slot leaks one reference to the child state, which does no harm.

## (2) Departures from the ticket

| Departure | Judgment |
| --- | --- |
| Fake process-ID source and call-order observer dropped. The ID is an argument, and the order rests on the one accessor. | Accept. `Engine::built_by(settings, pid)` and `process_width_of(pid, …)` are production code paths. `Engine::new` passes `std::process::id()`, so this is an injected argument and not a test-only hook. The ordering holds because every retained resource is reachable only through the `Arc<State>`. `only_the_one_accessor_reads_the_retained_state` misses `engine.state` in `built_by`, but that is construction only. Ian can overturn this. |
| Recorder folder gate moved from the recorder's lifetime to each request, held by `WritePermit` until the recording is saved. | Accept. It matches `specification/recording.md` line 69, and without it a child would keep its parent's folder lock. It is untested, though: see F2. The one-time backend identity check now sits behind an `AtomicBool`; see F3. |
| The width state gets its own guarded slot, and `Client::new` takes it. | Accept. It is needed because width state belongs to the process and not to an engine. Eight test call sites changed mechanically. |
| A grandchild that reuses a dead grandparent's process ID trusts inherited state. | Accept as a known limit. ADR 0017 fixes the owner as the process ID, and the ticket excludes at-fork hooks. |
| `Engine::usage` now returns `Result`. | Accept. A child's first `usage` call may have to build state. |
| An `Engine` dropped in a child before any call drops the inherited `State` instead of leaking it. The ticket says no code may drop inherited state. The record does not mention this. | See F5. It is harmless today, but it should be recorded. |

## (3) Size and whether the tests are scaffolding

I measured nonblank lines of `.rs` changes from `e7696ca8` to `e685740b`: production +265 and −112, tests +465 and −26. These agree with the record. About 153 net production lines for the slot, the one state door, and the recorder restructure is lean, and `Guarded` serves both the width state and each engine's state. The 592-line ratchet rise is proportionate, and the file and line budgets are met.

Tests against the four kinds the decision allows:

- The `process/tests.rs` table is an edge-case table and earns its place.
- `racing_children_publish_one_state` protects the ticket's race rule. My plant P1 turned it red.
- `a_failed_build_leaves_the_next_call_to_build` earns its place. It caught P1, and the table row for a failed build did not. It could become a two-call table row.
- `a_waiting_child_stops_with_its_call` overlaps table row 7 ("a child waits on its own rebuild"). Its only extra assertion is that the builder still publishes. Fold it into the table or delete it.
- `an_ordinary_drop_frees_the_state` protects the ticket's line "an ordinary engine drop does not leak". It is flaky: see F1.
- `fork_tests.rs` holds two behavior proofs. They drive the engine through private internals (`engine.state.current(parent_pid())`, `engine.transport`) because the crate cannot fork. No test-only hook in production code enables them. Once 0086's real-fork proofs land, audit them: the busy-parent proof will then test one contract at two layers. The width proof also covers ticket 0077's rule for a child, so keep it.

The record says "three cases" beside the table. There are four.

## (4) Bugs I planted

Each plant was reverted after its run.

| Plant | Result |
| --- | --- |
| P1: publish the owner before building, so racing callers get the inherited state | Red: `racing_children_publish_one_state`, `a_failed_build_leaves_the_next_call_to_build`, and `a_waiting_child_stops_with_its_call` |
| P2: the child's state skips `Widths::select` and uses the explicit width or the fallback | Red: `child_width_child` and the older `facade_width_child` |
| P3: the identity check is marked done before it runs, so a mismatched folder fails only the first call on an engine | **Green in all `thinkthen` targets** (`--all-targets --all-features`). See F3. |
| P4: the folder gate is dropped as soon as the save is prepared, before the recording is saved | **Green in all `thinkthen` targets.** See F2. |

The builder's six plants cover the rule itself. P3 and P4 show that the recorder restructure has no test of its own.

## (5) Load and paid backends

- The load average was between 10 and 11.6 during my runs.
- I ran `fork_tests process::` 5 times in a row, then 36 times with 12 copies running at once. Every fork proof passed. `an_ordinary_drop_frees_the_state` failed in 2 of the 36 runs.
- I then ran `process::tests::` 60 times with 12 copies at once. It failed 6 times, always at `tests.rs:149`.
- With the one-line fix from F1, it passed 96 times out of 96 under the same load.
- Nothing reaches a paid backend. The fork proofs use the loopback `conformance_backend` or `127.0.0.1:1`, and the key is the test value `sk-test-value`. The child test binary gets a test key and temporary `XDG_*` homes. The process tests make no network call. I ran every test with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset.

## (6) Merging with 0086

There is no conflict today. The 0086 worktree (`ticket/0086-public-rust-api`) already merged this whole branch at `e685740b` (merge `e38f0296`). Its HEAD contains `147e7f78`, `138c1def` and `e685740b`. Its uncommitted work builds on 0096's shape:

- `state: Arc<Guarded<State>>`, so `with_model` shares one guarded state between two engines. That is correct: whichever engine calls first rebuilds the shared state once in a child.
- `key: KeyReader`, which changes the `settings()` helper in `fork_tests.rs`, `ask_as_parent`, and `cli/failure/tests.rs`.

If 0096 lands unchanged, 0086's next merge of main brings nothing new for these files.

The fixes below touch `process/tests.rs` and `fork_tests.rs`, and F3 adds a test in the recorder area. The merge plan:

1. Land 0096's fix commits on this branch, then on main.
2. 0086 commits its work in progress, then merges `origin/main`.
3. Any conflict will be limited to `fork_tests.rs`. Keep 0096's new assertions and 0086's `Arc::new(|| …)` key and `|| (engine.key)()` call.

`Client::new(timeout, secure, widths)` and `Engine::state(cancel)` do not conflict. 0086 already calls both in their 0096 form.

## Findings

**F1 (blocking): a flaky test.** `process/tests.rs::call` sends its answer before its thread drops that thread's `Arc<Guarded>` clone. `an_ordinary_drop_frees_the_state` then drops only the main thread's clone and can still see a strong count of 2. It failed 6 times in 60 runs at load 10 or more. The fix is one line in `call`, and it passed 96 of 96 runs:

```rust
        );
        drop(guarded);
        let _sent = sent.send((answer, builds.into_inner()));
```

**F2 (blocking): the per-request folder lock is untested.** This ticket moved the recorder's shared folder lock into each request. The spec requires the lock from the first lookup through replay or saving, so `cache prune` cannot run while a request uses the folder. Plant P4, which drops the lock before the recording is saved, survives every `thinkthen` test. `busy_parent_child` already holds a live recording at the listener. While it is held, add one assertion that an exclusive folder lock is not granted within a short bound, using the try form in `cache_lock` if one fits. The existing check after the engine drops then shows the lock is released.

**F3 (should fix): the identity check's order is unguarded.** The new `checked` flag must be set only after `identity::check` succeeds. If it is set first (plant P3), a folder that belongs to another backend refuses only the first call on a long-lived engine. It also lets a concurrent call write into that folder before the first call fails. Add one edge case: a mismatched folder refuses a second call on the same engine. That matters once 0086 exposes long-lived engines.

**F4 (record fix): "pinned" is not accurate.** `crates/thinkthen/Cargo.toml` says `arc-swap = "1.7.1"`, which allows any later 1.x release. Only `Cargo.lock` and the ladder's `--locked` pin 1.7.1. I confirmed that 1.9.2 depends on `rustversion`. Any build without this lock file would take the second crate, for example an outside consumer crate or a binding with its own lock. Either write `=1.7.1` or change the record to say the lock file pins it. Also fix "three cases" to "four".

**F5 (record as a known limit): an engine dropped in a child.** If an engine is dropped in a child before any call, `Guarded` drops the inherited `State`. The ticket forbids that. I checked what the drop would run:

- `ureq` 3.4.2 has no `Drop` impls.
- Dropping a `std::sync::Mutex` does not lock it.
- The width state is a leaked `'static` and is never dropped.
- The recorder and the counters have no `Drop`.
- Closing the child's copy of an idle socket does not affect the parent.

So nothing takes a lock or writes to a socket today. Record this in the build record as a known limit, or make `Guarded`'s drop leak when the owner is another process.

**F6 (note for 0086): request-owned file locks after a real fork.** `cache_lock` releases its locks by closing the file. It never calls `unlock`. After a real fork, the child keeps copies of the parent's open lock files for requests in flight. Per flock(2), such a lock stays held until every copy is closed or one copy calls unlock. A busy parent's digest lock or shared folder lock therefore outlives the parent's release until the child exits. Meanwhile other waiters for that digest, or `cache prune`, keep waiting. The ticket explicitly keeps request-owned OS locks request-owned, so this does not block 0096. 0086's busy-parent real-fork proof may hit it, though. A few lines would fix it: have the lock types call `File::unlock` explicitly before closing. File it in 0086 or in a small follow-up.

## Dependency check (second reviewer under the dependency rule)

- **License:** MIT OR Apache-2.0, from the 1.7.1 `Cargo.toml`. Both are already allowed in `deny.toml` and `policy.py`.
- **`cargo deny`:** `cargo deny --offline check licenses bans sources` passes, and `check advisories` passes against the local database. `deny.toml` is unchanged.
- **What the lock file pulls in:** one `[[package]]` entry for `arc-swap` 1.7.1 with checksum `69f7f8c3…`, plus the new edge from `thinkthen`. `cargo tree -i arc-swap` shows only `thinkthen` depends on it. Its only optional dependency, `serde`, is not enabled. `policy.py` adds it to the accepted direct dependencies.
- **Whether a std-only slot would do:** no. Every safe std option either locks or takes the lazy lock the ticket forbids:
  - `Mutex<Arc<T>>` and `RwLock` take a lock.
  - `OnceLock` and `Once` are exactly the lazy initialization lock the ticket forbids, because a child can inherit one halfway through initialization.
  - `AtomicPtr` needs `unsafe` to read through, and the crate forbids `unsafe`.
- **Pin:** see F4. The pin comes from the lock file, not from the manifest.

## Re-review of the fixes at `4d2945e6` (code at `45e66de7`)

**Verdict: ACCEPT.**

Since `e685740b` the branch adds three commits of its own and one merge of main `a2e5f1fa`:

- `dd3c9855` makes the fixes.
- `45e66de7` raises the ratchet from 50970 to 50981.
- `4d2945e6` records the ladder.

The merge brings in only `qf-throttle` and `qf-ticket-evidence`. They change docs, `sdlc/`, the site and the spec, and no `crates/` source. The ticket's own diff touches only `Cargo.toml`, `fork_tests.rs`, `contract_tests.rs`, `process/tests.rs`, the build record, and `ratchet.json`. No production `.rs` file changed: the diff from `e685740b` to `4d2945e6` over `process.rs`, `facade.rs`, `recorder.rs`, `mod.rs`, `usage.rs` and `http.rs` is empty. `Cargo.lock` is unchanged.

| Item | Checked |
| --- | --- |
| F1 | `call` now drops its `Arc<Guarded>` copy before it sends. `process::tests::` ran 30 times, 10 copies at once, at a load average of 10.4 or more with 12 busy loops added: 30 of 30 passed. It also ran 73 more times at loads of 6 to 7: all passed. |
| F2 | `busy_parent_child` now opens the folder while the parent's send is held and asserts that `try_lock` returns `WouldBlock`. The check does not block. I planted the early gate drop again, and it went red at `fork_tests.rs:126`. |
| F3 | New `a_folder_of_another_backend_refuses_each_call`: a second engine against a folder recorded for another backend is refused with `RecordingBackendMismatch` on two calls, and it sends nothing. I planted the flag-first order again, and it went red at `contract_tests.rs:187`. The production code was already correct, so a test was the right fix. |
| F4 | The manifest says `arc-swap = "=1.7.1"`, and the lock file is unchanged. `cargo deny --offline check bans licenses sources` passes. The record says "pins `=1.7.1`". |
| F5, F6 | Both are recorded as known limits in the build record. F6 is a dated note on the 0086 ticket at `57026289` on `origin/ticket/0086-public-rust-api`. It states the `File::unlock` fix. |
| Waiting-child test | Deleted. Table row 7 covers the same wait. |
| Ratchet | +11 net test lines (the two new checks less the deleted test), so 50970 + 11 = 50981, as recorded. |

On this tree, `cargo test --lib --all-features` passed 336 tests, and `cargo fmt --check` passed. I did not rerun the whole ladder. The record reports it at `45e66de7`: 817 passed, and the spec demos gave 21 green.

Nothing reaches a paid backend. The new test uses the local `Listener`, and I ran every test with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset.

The 0086 merge plan is unchanged. 0086 merges `origin/main` after 0096 lands. The fixes touch only test files, and any conflict will be limited to `fork_tests.rs` and `contract_tests.rs`. Keep both sides.
