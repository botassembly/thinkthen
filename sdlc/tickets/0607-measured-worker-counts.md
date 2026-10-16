# 0607 — measured-worker-counts

Status: OPEN.

Milestone: 0.3

Depends on: 0589

## Outcome

Compiler and test worker counts are set from a measured sweep, not pinned.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- Checks pin 2 workers on the 16-core Beelink. Nextest spends 360 seconds of test time in 171 seconds of wall time.
- Best measured on the M5 once routine builds move there.
- Cost before: 171 s of nextest wall time at 2 workers. Expected after: the sweep's best count, recorded with its timeouts.

## Scope

- Changes: `.config/nextest.toml`, `sdlc/scripts/test`, and the build-job settings in `sdlc/scripts/`.
- Keeps: every check's coverage.

## Proof

A sweep at 2, 4 and 8 workers records wall time and timeouts, and the chosen count is set in one place. After landing, measure again and record the result here.
