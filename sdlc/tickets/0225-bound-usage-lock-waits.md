---
flow: build
priority: 225
opens: sdlc/issues/2026-09-26-recording-page-omits-the-exit-wait-on-the-usage-lock.md sdlc/records/0225-usage-lock-preflight.md sdlc/records/0225-design-review.md sdlc/planning/adr/0097-bound-advisory-usage-lock-acquisition.md
---

# 0225: Bound the advisory usage-lock wait at exit

Status: design **ACCEPT** at `1e4936a7` by fresh Medium reviewer session `01a0e695-9381-7070-a807-bcc3be1f739f`; coordinator approved routine implementation. Build and code review remain. Owner: Codex. [ADR 0097](../planning/adr/0097-bound-advisory-usage-lock-acquisition.md) proposes the policy change; [the source preflight](../records/0225-usage-lock-preflight.md) confirms register 31 against main `4e0505a9`. The 0163 page fix closed only the separate recording-page issue. This ticket would close register 31's remaining runtime defect after a reviewed build, without touching the paid live ledger.

## Outcome and retained behavior

A completed command does not wait without limit for another process to release the **advisory usage** lock. From `Counters::finish`, its writer shares one one-second monotonic deadline across every remaining usage-lock acquisition. `Drop` uses the same deadline if finish was not called, so a timed-out finish cannot be followed by an unbounded writer join. If lock contention reaches that deadline, the writer abandons unwritten deltas, sets the existing failure flag, and the command prints its existing single usage warning after its result and before final `--facts`. Output, answer, request sends, exit meaning, exact request identity, the in-memory run facts, and successful monthly atomic writes remain as today. A count already written stays; counts since the last completed write can be lost. No later update in that process silently resumes persistence after failure.

This bounds the foreign **lock-acquisition wait** at finalization. It is not a total exit deadline. Filesystem open, validation, sync, rename, a slow network filesystem operation, the read-only `status` shared lock, or another unrelated wait can still delay the process. Do not advertise a general one-second exit guarantee. The separate live ledger retains its authority and must not be read, copied, migrated or written for this ticket.

## Proposed implementation and proof

`engine/usage.rs` currently has `File::lock(&lock)` in `update`, `Condvar::wait_while` in `finish`, and a joining `Drop`. A short private `engine/usage/lock.rs` can hold the monotonic try-lock loop and shared finish deadline; the writer must observe one deadline across a taken group of month deltas. Keep the queue's `failed`/`pending.clear` path and the existing warning in `cli/mod.rs`; do not add a test-only product flag, public error, cache/replay lock change, or paid ledger code. `specification/recording.md:37` must replace its explicit unlimited-wait sentence with the bounded advisory-lock policy after ADR acceptance. Leave ADR 0049's accepted write-behind history intact and cite the amendment.

Use one outside-in functional regression with a scratch usage folder and a separately held `.lock`: complete one loopback-backed command while the holder keeps the lock, assert the answer and unchanged exit code, one existing warning before `--facts`, a missing or unchanged monthly count, and no need to release the lock to let the child exit. A generous child watchdog detects the old hang without a stress campaign or brittle exact elapsed-time threshold. The source invariant proves the one-second shared deadline; the test proves the real command boundary. Reuse current usage-failure and facts cases for atomicity, count semantics and warning order. Focused format, strict affected Clippy, named tests and pages/tickets/diff suffice; no provider call, live ledger operation, broad gate or contention campaign.

## Scope and routing

Prospective source: `crates/thinkthen/src/engine/usage.rs` and one coherent `engine/usage/lock.rs` child; focused compiled regression in `crates/thinkthen/tests/backend/facts.rs` or a small child if its 500-line cap requires it; `specification/recording.md`; and the source ratchet and 0225 records. `cli/mod.rs`, `engine/usage/{attempt,facts,counts}.rs`, public APIs, and the reader's `File::lock_shared` are read-only unless fresh review finds a concrete gap. Source claims precede edits. The current page already documents the unlimited wait, so a page-only change cannot close this runtime item. The coordinator owns plan rows and closure.

## Evidence

- Starts from: register 31 in experiment 284, the closed 0163 page issue, ADR 0049 item 3, recording.md:37, and main `4e0505a9` `engine/usage.rs::{finish,drop,write_behind,update}` plus existing usage/facts tests.
- Keeps: asynchronous count-only monthly persistence, atomic replacement, request independence, original answer/exit/facts order, single warning on failure, and independent live-ledger authority.
- Changes: all writer lock acquisitions remaining after finish share a one-second deadline; a held foreign lock can cause advisory count loss and the existing warning instead of an unbounded wait.
- Proof: one held scratch lock across a complete compiled command, checked output/warning/facts/count, existing failure and accounting cases, source deadline invariant, and focused lint/documentation checks.
- Defers: total filesystem-I/O timing, read-only status lock waits, lock fairness, custom timeout settings and any paid-ledger policy. None is solved by this advisory writer change.

## What the build taught us

Preparation found a second wait in `Counters::drop`: a timeout in `finish` alone would not release exit. It also found that `write_behind` may hold several month deltas, so a fresh deadline per `update` would not give one finalization bound. The accepted design must account for both. This is preparation evidence, not a build result; replace it with measured findings before closure.
