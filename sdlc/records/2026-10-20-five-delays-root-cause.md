# Root cause: the five largest delays on 2026-10-20

Ian asked on 2026-10-20 for a root-cause analysis of the problems causing the greatest delays. At 16:00 nothing had landed for two and a half hours, five lanes waited on one dependency repair, and 54 phase 3 tickets waited on the core freeze (0688). Ian approved every fix below the same afternoon; the rules are in [the freeze decision](../decisions/2026-10-20-freeze-names-the-contract.md).

## 1. One barrier holds all language work

Severity: medium. It is the largest delay.

- Cases: 0688 waited on a passing three-platform checkpoint run. The 52a2e2c48 run failed four ways (0743 worker thread count, 0744 TLS advisory, 0745 closed-pipe race, nine Yellow routine deadlines). None changed the header, packet schema, run function or value tree. Each became a ticket that must land before another multi-hour run.
- Why: rule 1 of [the 0.2.1 ruling](../decisions/2026-10-18-0-2-1-rebuilds-the-basis.md) needs a named commit for the contract. 0688's proof added the full checkpoint run to that. The contract tickets landed at 13:38; the checkpoint run tests fixtures, packaging and machines as well.
- Root cause: one ticket tied "the contract is settled" to "the whole suite passes everywhere", so test-fixture problems held contract-stable work.
- Fix: the freeze names the contract after the landing gate and native smoke and package pass. The full checkpoint run continues alongside and gates qualification (0687).

## 2. The checkpoint finds what landing gates never see

Severity: medium.

- Cases: 0743 (arm64 package checkpoint), 0745 (configured gate, first seen at checkpoint), the nine Yellow deadlines, and the x86 failures 0621 fixed earlier.
- Why: a ticket's landing gate runs lint and routine tests from the M5. The checkpoint adds packaging, both containers and x86. Problems that only show there surface once per checkpoint, one at a time, each costing a full cycle.
- Root cause: the landing gate is narrower than the checkpoint, and x86 meets each change for the first time at the checkpoint.
- Fix: each landing batch also runs the x86 container gate on Gator in parallel with its configured gate. When Gator is held, the batch lands on the configured gate alone and the next checkpoint covers x86. Jarvis is asked whether pm's gate can run both.

## 3. Tests that depend on timing

Severity: medium, repeated.

- Cases: the closed-pipe race across five fixtures (0745, first seen in 0740's gate); the libtest thread counted by 0743; nine Yellow deadlines (`sdlc/records/0688-yellow-checkpoint-routine-deadlines.md` on the 0688 branch); earlier routine timeouts (0605) and Linux parity timeouts (0621).
- Why: tests use wall-clock waits and scheduler behavior for synchronization, put fixture setup and durable writes inside the five-second limit, and took their headroom from the arm64 M5.
- Root cause: deadlines and waits stand in for explicit synchronization, measured on the fastest machine.
- Fix: 0605 moves to 0.2.1 and takes the Yellow cases: explicit signals instead of waits, setup outside the timed part, limits checked on the slowest supported machine.

## 4. A live advisory database turned every gate red

Severity: medium.

- Cases: one TLS advisory stopped the gates of 0743, 0745 and 0740 and the Gator checkpoint within an hour, without a commit. Every lane then waited on standalone 0744.
- Why: lint's `cargo deny` step checks advisories against a freshly fetched database on every gate, so main's gate changes when the database changes.
- Root cause: every ticket's gate checks a live outside database regardless of what the ticket changed.
- Fix: 0746. A ticket's gate checks advisories only when it changes a lock; checkpoint and release checks still check every run, and a finding gets a repair ticket ordered first.

## 5. Runs that fail on setup or invocation

Severity: medium, falling.

- Cases: about twelve of sixteen failed runs earlier in the day ([earlier record](2026-10-20-setup-failures-root-cause.md)); then M5 receipt 1792523999973-37872, where a brief gave 0744 native `sdlc/scripts/job sdlc/scripts/lint` on macOS, and generation needs a Linux-only GCC provider.
- Why: the standard-commands rule copies a command into the brief, but nothing says which machine the command runs on, and lint is Linux-only.
- Root cause: the test machinery is still large and tied to particular machines; a copied command helps only when it fits the machine.
- Fix: every ticket command names its machine, and Linux-only checks (lint, generation, `cargo deny`) run only through `linux-check` or the configured gate.

## Reach

Fix 2 and fix 4 concern habits other Rust repos share: a gate that checks a live advisory database, and a single-machine gate. Both went to Jarvis.
