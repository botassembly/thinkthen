ACCEPT

# Review of ThinkThen Quick Fix 0116: deterministic global-queue test

Commit reviewed: `7b156f08` on `ticket/0116-deterministic-global-queue-test`. Reviewer: fresh read-only session (Opus 5.5). I did not write the work. The worktree has no uncommitted changes.

## 1. The test still proves the bound and the fill

The listener counts a request in flight and raises its peak before it calls the reply closure (`conformance/backend/src/listener.rs`, `serve_kept`). A request that queues on the `Gathering` mutex already counts. `assert_eq!(peak, min(jobs, 6))` covers the old `peak <= jobs` and `peak > 1` checks and adds the full fill.

I planted both bugs in a scratch copy from `git archive 7b156f08` with its own target folder. The plants went in `crates/thinkthen/src/engine/annotate_schedule.rs`:

| Plant | Result |
| --- | --- |
| `jobs + 1` workers and `in_flight <= jobs` | red at `document at 1`, left 2, right 1, in 0.22 s |
| same, with the loop narrowed to `[4, 32]` to check width 4 | red 5 of 5, `document at 4`, left 5, right 4 |
| 1 worker and `in_flight < 1` | red at `document at 4`, left 1, right 4, in 31.04 s |

## 2. It cannot hang

Each of the first `wanted` requests waits at most `FAILSAFE` (10 s) through `wait_timeout_while`. A request after `wanted` does not wait. The one-at-a-time plant finished in 31 s with a plain `left: 1, right: 4` message. A queue two wide at 4 jobs would take about 10 s. At 32 jobs the worst case is five 10-second waits, but the run fails at 4 jobs first. Each hold stays under the tool's 30-second request timeout, so no retry adds requests.

## 3. Repeat runs

20 of 20 passes in the worktree. The one-minute load was 7.0 at the start and 9.2 at the end. I did not run `stress-ng`.

## 4. Scope, ratchet, merge

- The only non-`sdlc/` file changed is `crates/thinkthen/tests/backend/annotate/scheduling.rs`. No production code changed.
- `lint` reports `ratchet: crates + conformance 47007/47007`, which matches `sdlc/ratchet.json`.
- `origin/main` is one commit ahead (`cb1ccb7b`, `sdlc/` only). `git merge-tree` against it is clean.
- `origin/ticket/0077-process-width-cap` has only its ticket file so far. `git merge-tree` against it is clean. 0077 opens `tests/backend` and `sdlc/ratchet.json`, so whichever lands second rebases the one-line ratchet number.

## 5. Gates

`THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` were unset. I waited until the one-minute load fell below 10 each time.

- `sdlc/scripts/lint`: exit 0 at a load of 8.94.
- `sdlc/scripts/test`: exit 0 at a load of 7.30. There were 18 result lines with 767 passed, 0 failed, and 2 ignored, plus `live-test: all cases passed`.
- `sdlc/scripts/live` did not run. I deleted the scratch copy and its target folder.

## Notes (no action required)

- At 1 job the hold is only the 50 ms sleep. A too-wide queue at 1 job could slip past under heavy load. The 4-job pass still catches it, because the fifth request goes out while the first three are held. The old test had the same timing dependence at 25 ms.
- The record's status line says the review has not run yet. The author updates it when landing.
