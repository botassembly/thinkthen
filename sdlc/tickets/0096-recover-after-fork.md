---
flow: build
priority: 96
opens: Cargo.lock deny.toml crates/thinkthen/Cargo.toml crates/thinkthen/src/engine crates/thinkthen/src/cli sdlc/scripts/policy.py sdlc/ratchet.json sdlc/planning/adr/0017-libraries-over-one-bound-core.md
---

# 0096: Recover after fork

Status: revised after re-review; confirming. Owner: Claude.

Split from 0078 on 2026-09-24 ("0078b" in the spine review). 0078 keeps host signals. This ticket lands after 0085, which builds the retained engine state this ticket guards, and before 0086.

## Outcome and authority

Every engine call first proves that its private runtime state belongs to the current process. A forked child never touches an inherited mutex, condition variable, width waiter, connection pool, recorder coordinator, counter lock, or worker queue. On a process-ID mismatch it publishes fresh child state through an atomic replacement path. The parent keeps its state. ADR 0017 section 2 fixes the PID-before-lock rule, a fresh child pool and gate, the lock-free replacement path, and leaking inherited state instead of tearing it down. Ian can overturn the child counter reset, the leak per fork, the one-dependency allowance, and the budgets.

## Design

These two sections are copied whole from the earlier 0078 draft (commit `01d82781` on `ticket/0078-fork-and-host-signals`). Ticket 0085 now owns the retained HTTP pool, and 0078 now owns host signals only.

### One PID guard before runtime state

Keep immutable engine settings outside the replaceable runtime state: resolved backend and model settings, timeout and retry settings, cache and recording paths, usage path, and an engine's explicit-or-omitted width choice. Put every fork-unsafe mutable resource below one private `ProcessState`: ticket 0077's selected-width state and attempt gate, every reusable HTTP pool, process counters and their persistent-update mutex, and recorder/cache coordinators that retain a userspace lock or open handle between calls.

Every direct and bulk call enters one `current_process_state` choke point before key lookup, cache or recording preparation, counters, width registration or acquisition, pool access, input dispatch, scheduler construction, or worker creation. Read the current PID and the published owner PID using atomics before loading the state slot. When they match, acquire the published state and continue. When they differ, no code may inspect, lock, clone, drop, or call through the inherited state first.

The replacement protocol uses only atomics until fresh child state exists. A PID-tagged rebuilding marker elects one child thread. The winner constructs complete fresh state from immutable settings, atomically replaces the slot, deliberately leaks the inherited state in the child, then publishes the child PID with release ordering. Other child callers wait on the atomic rebuilding marker with the existing bounded poll and observe cancellation or ticket 0076's deadline. They never wait on an inherited mutex. A child forked again while replacement is in progress can replace the inherited marker because it names another PID.

Use one safe atomic-`Arc` slot implementation. The repository continues to forbid unsafe code. This ticket may add the narrowly scoped `arc-swap` dependency if the post-0077 tree has no equivalent reviewed safe primitive. The PID word and slot must be statically initialized without a lazy mutex that can itself be inherited mid-initialization. Stop for redesign if the selected primitive takes a userspace lock before the PID mismatch is known, runs inherited-state destruction during replacement, or cannot publish the complete state before the child PID.

Retired inherited state is leaked only in the forked child. The parent owns and eventually drops its state normally. A same-process width change or ordinary engine drop does not leak. The ticket records the bounded cost: one retired runtime state per fork generation that later uses the engine. No background reaper, at-fork callback, process scan, or inherited teardown enters.

### Fresh child meaning

The child width selection starts unset. Its first call applies ticket 0077 exactly: an explicit engine width sets the child process cap, an implicit engine leaves the selection unset and uses fallback 4, a matching later explicit width succeeds, and only a conflicting later explicit width fails. Parent selection, permits, and waiters do not cross. The fresh gate has no permit held.

The child opens a new HTTP pool sized from its child effective width. It reuses no inherited idle socket, resolver state, connection object, or pool synchronization. A warm parent therefore gives the child no transport shortcut. A busy parent gives the child no inherited capacity debt. The parent request count and active sockets remain unchanged by child rebuilding.

The child creates new process counters at zero and a new persistent-update mutex around the configured usage path. Durable monthly files remain cross-process and keep their operating-system locking. The child adds only operations that begin in the child. A pre-fork parent count is visible in the durable total once, not copied into the child's process snapshot or added again.

The child creates new recorder and cache coordinators from immutable folder settings. It does not inspect or lock an inherited recorder mutex or reuse an inherited `FolderGate`. New child operations open their own directory and digest locks and follow the existing second-read rule. Stored entries, backend identity, privacy checks, immutable-entry behavior, and cache-answer meaning do not change. Restructure any process-retained recorder handle that would keep an inherited filesystem lock alive after replacement; request-owned operating-system locks remain request-owned and are never treated as reusable process state.

Each child call creates new scoped worker queues after state recovery. No scheduler, input request, pending annotation group, completed parent result, cancellation observation, or deadline instant transfers from the parent. Caller-supplied cancellation remains caller-supplied; this ticket does not silently reset an explicit token. A new call computes its own ticket 0076 deadline from its own budget.

## How it is proven without `fork()` in the crate

The crate forbids `unsafe`, and `fork()` is unsafe. So the in-crate proofs use a private PID source with a `cfg(test)` fake:

- Red-first unit tests hold each inherited lock in another thread: the recorder mutex, the rebuilding marker, the gate, a queue, and the counter mutex. They fake a PID change and prove replacement never touches those locks. Each held-lock case runs on its own spawned thread, and the test fails on `recv_timeout`, the pattern at `crates/thinkthen/src/engine/annotate_schedule.rs:465`. A lock-first bug therefore fails the case and never hangs the gate.
- A call-entry observer proves the PID comparison precedes width, pool, recorder and cache work, counters, dispatch, queue construction, key lookup, accounting, connection, and send. Each boundary has a counter or phase signal.
- Races: two callers after a faked change publish exactly one fresh state. A second faked change during rebuilding also recovers. Conflicting explicit widths follow 0077's rule.
- Counters: nonzero parent counts, a faked change, a zero child snapshot, and exact durable totals.

The real-fork proofs (warm parent, busy parent, warm cache) live in 0086's outside consumer crate. That crate gets a reviewed allowance for one `unsafe` fork call in its test code. The Python surface's `os.fork` check repeats them later.

## Acceptance

- The proofs above, red first.
- This ticket carries error-index row R5-3 (a lock taken on a child's first call after fork), moved from 0078.
- Planted-bug proof: for R5-3, the record plants a lock taken before the PID check and shows the fake-PID test turning red.
- Existing 0076, 0077, 0078, and 0085 tests stay green. Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass. The coordinator runs `install`, `lint`, `test`, and `spec` in order.

## Scope

At most fourteen production files and 650 nonblank production lines; at most 950 test lines. At most one new dependency, for the atomic slot only. Its `Cargo.lock` and `deny.toml` changes are in scope, and a second reviewer checks it under the repo's dependency rule. Excluded: public types, bindings, signals (0078), `pthread_atfork`, resident threads, cross-process width, duplicate-image coordination (ADR 0047 item 5), and paid calls.

## Dependencies

After 0085 and 0097, since both edit call entry. Before 0086.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Review

- Design review: `sdlc/records/2026-09-24-rereview-engine.md` asked for a timeout per held-lock case, the design written here whole, the lock and deny files, and the order after 0097. All applied; confirmation pending.
- Code review: pending.
