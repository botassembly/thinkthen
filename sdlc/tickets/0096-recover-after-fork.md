---
flow: build
priority: 96
opens: crates/thinkthen/Cargo.toml crates/thinkthen/src/engine crates/thinkthen/src/cli sdlc/scripts/policy.py sdlc/ratchet.json sdlc/planning/adr/0017-libraries-over-one-bound-core.md
---

# 0096: Recover after fork

Status: draft, revised after design review; needs re-review. Owner: Claude.

Split from 0078 on 2026-09-24 ("0078b" in the spine review). 0078 keeps host signals. This ticket lands after 0085, which builds the retained engine state this ticket guards, and before 0086.

## Outcome and authority

Every engine call first proves that its private runtime state belongs to the current process. A forked child never touches an inherited mutex, condition variable, width waiter, connection pool, recorder coordinator, counter lock, or worker queue. On a process-ID mismatch it publishes fresh child state through an atomic replacement path. The parent keeps its state. ADR 0017 section 2 fixes the PID-before-lock rule, a fresh child pool and gate, the lock-free replacement path, and leaking inherited state instead of tearing it down. Ian can overturn the child counter reset, the leak per fork, the one-dependency allowance, and the budgets.

## Design

The design of the earlier 0078 draft carries over unchanged. See its text at `origin/ticket/0078-fork-and-host-signals` commit `01d82781`, sections "One PID guard before runtime state" and "Fresh child meaning". In short:

- Immutable settings stay outside one private `ProcessState`. That state holds 0077's width selection and gate, 0085's retained HTTP pool, process counters, and recorder and cache coordinators.
- One `current_process_state` choke point runs before key lookup, cache or recording work, counters, width, pool access, dispatch, or worker creation. It compares PIDs with atomics before it loads the slot.
- One child thread wins an atomic PID-tagged rebuilding marker. It builds fresh state from settings, swaps the slot, leaks the inherited state, and publishes the child PID. Other child callers poll the marker under cancellation and 0076's deadline.
- One safe atomic-`Arc` slot. `arc-swap` is allowed if the tree has no reviewed equal. The repository keeps forbidding `unsafe`.
- The child starts with its width unset, a fresh pool, counters at zero, and fresh recorder and cache coordination around the same folders. Durable totals stay cross-process, and the child adds only its own deltas.

## How it is proven without `fork()` in the crate

The crate forbids `unsafe`, and `fork()` is unsafe. So the in-crate proofs use a private PID source with a `cfg(test)` fake:

- Red-first unit tests hold each inherited lock in another thread: the recorder mutex, the rebuilding marker, the gate, a queue, and the counter mutex. They fake a PID change and prove replacement never touches those locks. If the code locks first, the test hangs and the watchdog fails it.
- A call-entry observer proves the PID comparison precedes width, pool, recorder and cache work, counters, dispatch, queue construction, key lookup, accounting, connection, and send. Each boundary has a counter or phase signal.
- Races: two callers after a faked change publish exactly one fresh state. A second faked change during rebuilding also recovers. Conflicting explicit widths follow 0077's rule.
- Counters: nonzero parent counts, a faked change, a zero child snapshot, and exact durable totals.

The real-fork proofs (warm parent, busy parent, warm cache) live in 0086's outside consumer crate. That crate gets a reviewed allowance for one `unsafe` fork call in its test code. The Python surface's `os.fork` check repeats them later.

## Acceptance

- The proofs above, red first.
- Planted-bug proof: for error-index row R5-3, the record plants a lock taken before the PID check and shows the fake-PID test turning red.
- Existing 0076, 0077, 0078, and 0085 tests stay green. Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass. The coordinator runs `install`, `lint`, `test`, and `spec` in order.

## Scope

At most fourteen production files and 650 nonblank production lines; at most 950 test lines. At most one new dependency, for the atomic slot only. Excluded: public types, bindings, signals (0078), `pthread_atfork`, resident threads, cross-process width, duplicate-image coordination (ADR 0047 item 5), and paid calls.

## Dependencies

After 0085. Before 0086.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Review

- Design review: pending.
- Code review: pending.
