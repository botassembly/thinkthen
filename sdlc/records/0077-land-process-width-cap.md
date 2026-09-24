# Land: ticket 0077, one process width cap

Status: landed 2026-09-24. Owner: Claude.

## What landed

Branch `ticket/0077-process-width-cap`. The code is `a2097e53`. The build record is `1f4db620` (`sdlc/records/0077-build-process-width-cap.md`). A fresh read-only Claude review accepted `1f4db620` (`sdlc/records/0077-code-review.md`).

## Merges

The first merge of origin/main at `d783ab6b` brought in 0116 and later surface commits. It conflicted only in `sdlc/ratchet.json`. `ratchet.mjs` measured 47990, and the ceiling was set to that.

Ticket 0115 landed on main at `46950c84` before this ladder finished. The second merge brought it in. It again conflicted only in `sdlc/ratchet.json`. `ratchet.mjs` measured 48140, and the ceiling was set to that. The review's trial merge predicted the same two totals.

The gate ladder ran at the merge commit `7fafa5e2`.

## Checks

With no `THINKTHEN_` variable set. The one-minute load was 3.02 at the start.

- `install`: exit 0.
- `lint`: exit 0, `ratchet: crates + conformance 48140/48140`.
- `test`: exit 0, 781 passed, 0 failed, 6 ignored across 18 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

## Follow-up

The review found the default `--jobs` width question outside this ticket. The issue `sdlc/issues/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` now carries it as a Quick Fix for Claude.

The review offered a deterministic replacement for the 150 ms release window in `shared_cap_child`. It does not block, and the build record discloses the departure.
