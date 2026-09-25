# Surfaces in one batch

Written 2026-09-25 by Claude. Ian accepted this order the same morning. It replaces the one-surface-at-a-time order in `one-line-plan-2026-09-24.md`, including C before every other surface. Ian can overturn it.

## Why

The frozen `surfaces-wave7` tag holds all nine surfaces, about 63,000 lines. Each links the Rust crate directly, and none needs the C door to build. About 364 lines call the stand-in engine. The port swaps those calls for the public API from 0086 and 0098, and then the tests find where the real engine behaves differently. Porting all surfaces at once finds every such bug in one pass. The heavy lock runs one build at a time, so separate tickets in sequence gain nothing.

## Order

1. Prepare. One agent merges main into every surface ticket branch, renames `width` to `throttle` in the ticket texts, and amends 0110 for experiment 253's findings D2 and D5. A fresh reviewer checks the amendments.
2. Port. One agent per surface ticket swaps the stand-in for the public API on its own branch: 0094 C, 0105 Python, 0106 Python Polars, 0120 Rust Polars, 0107 TypeScript, 0108 R, 0112 Ruby, 0109 SQLite, 0110 and 0118 DuckDB, 0111 PostgreSQL. Code edits run in parallel. A compile check runs under the heavy lock.
3. Build and test. An integration worktree merges every branch. The ladder and every surface `check.sh` run once under the heavy lock, one after another. The result is one failure list.
4. Fix. Each surface agent fixes its own failures on its own branch.
5. Retest once. Fresh reviewers then review each surface in parallel.
6. Land together. One landing agent merges the surfaces and owns the files every surface touches: the workspace members, `Cargo.lock`, the ratchet, `sdlc/surfaces.txt`, and the license exceptions.

0094 runs its one C churn probe at low load and never again. 0119 waits until every surface lands.
