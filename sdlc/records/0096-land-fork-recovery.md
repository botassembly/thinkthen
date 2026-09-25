# Land: ticket 0096, recover after fork

Status: landed 2026-09-24. Owner: Claude.

## What landed

Branch `ticket/0096-fork-recovery`. The build and its review fixes end at `4d2945e6`, with code at `45e66de7`. The build record is `sdlc/records/0096-build-fork-recovery.md`. A fresh read-only Claude review returned six findings at `e685740b` and accepted `4d2945e6` on re-review (`sdlc/records/0096-code-review.md`). The same review checked the one new dependency, `arc-swap` 1.7.1, under the dependency rule.

## Merges

The branch merged main at `a2e5f1fa`, which brought in the throttle rename and the ticket evidence rule. `ratchet.mjs` measured 50981, and the ceiling was set to that. The full ladder ran at `45e66de7`, which contains that merge. The one-minute load was 9.92 at the start.

- `install`: exit 0.
- `lint`: exit 0.
- `test`: exit 0, 817 passed and 0 failed.
- `spec`: exit 0, `demos: 21 green, 0 red`.

At landing, main was still `a2e5f1fa`. This landing adds only Markdown: the review copy, this record, and the status lines. `lint` ran again on the landing tree.

`sdlc/scripts/live` did not run, and no test used a paid backend.

## Follow-up

- F6, request-owned file locks after a real fork, is a dated note on the 0086 ticket (`57026289` on `ticket/0086-public-rust-api`).
- Once 0086's real-fork proofs land, audit `engine/facade/fork_tests.rs::busy_parent_child`, so one contract is tested at one layer.
