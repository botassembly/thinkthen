# ADR 0116: Release branches are cut at the release candidate

- Status: **Accepted** on Ian's approval of 2026-10-01. The amendment of 2026-10-04 freezes release/0.1. Ian can overturn each item.
- Date: 2026-10-01

## Context

Main holds all work today. Ticket 0373 starts Windows support, which is 0.2 groundwork. Release fixes for 0.1 must not wait behind 0.2 work, and 0.2 work must not change 0.1 by accident. Before ticket 0376, the release workflow's rehearse mode refused any ref but `refs/heads/main` (`sdlc/scripts/release-workflow`, `resolve`).

The coordinator's steps around this rule live in [release-process.md](../release-process.md). This ADR holds only the branching decision.

## Decision

1. **Main is the next version.** Main never stops taking reviewed work. No long-lived 0.2 branch sits beside main.
2. **Before the cut.** Only work that cannot change 0.1 behavior lands on main: Windows stage 0, records and tests. Release fixes for 0.1 land as usual.
3. **The cut.** At 0.1's release candidate, the coordinator cuts `release/0.1` from main. The release candidate means three things hold: the rehearsal is clean, release QA's latest round is clean, and only release fixes remain.
4. **After the cut.** 0.2 work lands on main. The 0.1 release and every 0.1.x release come from `release/0.1`.
5. **Fixes.** Each 0.1.x fix lands on main first. The coordinator then cherry-picks it to `release/0.1` with a `Cherry-picked-from: <sha>` trailer naming the main commit.
6. **Windows stage 1.** It is 0.2 work and waits for the cut. If Ian pulls it into 0.1, it lands on main before the cut, and the cut moves later.
7. **The workflow.** Rehearse mode also accepts a release branch `refs/heads/release/X.Y`, such as `release/0.1`, and refuses every other branch. Ticket 0376 made the change before the cut on the coordinator's direction, so the cut does not wait on a ticket. Until a release branch exists, the rule allows nothing new. This item first said `refs/heads/release/*` and a change at the cut. Ian can overturn the timing and the narrower pattern.

## Consequences

- 0.2 work can start the day the cut happens. Nothing waits for the 0.1 release itself.
- A fix that main has outgrown may not cherry-pick cleanly. Its ticket then names the conflict and the change made on the release branch.
- Until the cut, Windows stage 0 must prove it changes no 0.1 behavior.

## Amendment, 2026-10-04: no 0.1.x releases

Ian ruled that there are no 0.1.x patch releases. `release/0.1` is frozen, and nothing is cherry-picked to it. Items 4 and 5 no longer apply to 0.1. Every fix lands on main and ships in 0.2. Ticket 0397 moves main to 0.2.0 at once. Main moves to the next version immediately after each release, while public install text keeps naming the latest published release.

Items 1 to 3 and 7 stand for the 0.2 cut. The two release approvals remain safety gates. No standing patch-release approval is wanted. Ian can overturn this ruling.
