# ADR 0116: Release branches are cut at the release candidate

- Status: **Accepted** on Ian's approval of 2026-10-01. Ian can overturn each item.
- Date: 2026-10-01

## Context

Main holds all work today. Ticket 0373 starts Windows support, which is 0.2 groundwork. Release fixes for 0.1 must not wait behind 0.2 work, and 0.2 work must not change 0.1 by accident. The release workflow's rehearse mode refuses any ref but `refs/heads/main` (`sdlc/scripts/release-workflow`, `resolve`).

The coordinator's steps around this rule live in [release-process.md](../release-process.md). This ADR holds only the branching decision.

## Decision

1. **Main is the next version.** Main never stops taking reviewed work. No long-lived 0.2 branch sits beside main.
2. **Before the cut.** Only work that cannot change 0.1 behavior lands on main: Windows stage 0, records and tests. Release fixes for 0.1 land as usual.
3. **The cut.** At 0.1's release candidate, the coordinator cuts `release/0.1` from main. The release candidate means three things hold: the rehearsal is clean, release QA's latest round is clean, and only release fixes remain.
4. **After the cut.** 0.2 work lands on main. The 0.1 release and every 0.1.x release come from `release/0.1`.
5. **Fixes.** Each 0.1.x fix lands on main first. The coordinator then cherry-picks it to `release/0.1` with a `Cherry-picked-from: <sha>` trailer naming the main commit.
6. **Windows stage 1.** It is 0.2 work and waits for the cut. If Ian pulls it into 0.1, it lands on main before the cut, and the cut moves later.
7. **The workflow.** At the cut, rehearse mode must also accept `refs/heads/release/*`. A small ticket makes that change at the cut, not before.

## Consequences

- 0.2 work can start the day the cut happens. Nothing waits for the 0.1 release itself.
- A fix that main has outgrown may not cherry-pick cleanly. Its ticket then names the conflict and the change made on the release branch.
- Until the cut, Windows stage 0 must prove it changes no 0.1 behavior.
