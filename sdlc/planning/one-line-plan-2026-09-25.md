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

## Ian's ruling on SQL spending, 2026-09-25

In SQL each row is its own engine call, so `max_requests` caps nothing on a large query (`sdlc/issues/closed/2026-09-25-sqlite-max-requests-caps-nothing-per-statement.md` on ticket 0109). Ian chose a cap per process. SQLite, DuckDB, and PostgreSQL each add a total setting (`thinkthen_max_requests_total`, spelled the host's way), unset by default. Before each engine call the extension takes the requests its engines in this process have sent, refuses as `usage` with zero sends once the total is spent, and otherwise passes the remaining budget as that call's `max_requests`. The cap then holds to within the call's retries. A forked child starts from zero, and each README says so.

## Ian's ruling on pandas, 2026-09-25

Ian ruled that pandas is supported fully in 0.1. This overturns the 2026-09-21 line "pandas leaves the surface" in ADR 0017. Full support means four things:

- A pandas column goes to `decide`, `choose`, `score`, and `tag`, and a pandas column with the caller's index comes back.
- A pandas frame goes to `annotate` and `recognize` with `on=`, and the frame comes back with the new answer columns.
- pandas 2 and pandas 3 both work. A pandas 2 object column crosses at list speed, and the page says so.
- `import thinkthen` still imports neither pandas nor Polars.

Claude offered a partial version with columns only and frames refused, and recommended against it. A pandas user's first try is a whole frame, and a bare list back invites the lost-index bug R3-19 found. Ticket 0122 carries the work. It starts after 0106 lands, and it touches only `libraries/python`. It reuses the 2026-09-21 checks in `sdlc/issues/closed/2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md` as its starting proof.

## Landed, 2026-09-25

Seven surfaces landed together on main at `eb3fae21`: C 0094, Rust Polars 0120, TypeScript 0107, SQLite 0109, Ruby 0112, R 0108, and PostgreSQL 0111. The full ladder passed once on the batch, and `sdlc/records/surface-batch-integration.md` holds the run. Claude landed these seven without waiting for Python and DuckDB, and Ian can overturn that. Python 0105 and 0106 are accepted and wait on Ian's word on the blocked merge. DuckDB 0110 and 0118 are still being fixed. Both follow as a second batch. 0094's one C churn probe is still owed.

## Ian's ruling on landing, 2026-09-25

Ian ruled: "You decide everything. Whatever's most efficient, you can merge whatever you want to merge. Whatever is ready, you do it." Claude lands each reviewed, checked surface as soon as it is ready and needs no batch. Python 0105 and 0106 land next, with `libraries/python/deny.toml` carrying the `target-lexicon` exception Ian approved the same day.

## The 0.1 queue after the surfaces, 2026-09-25

All ten surfaces are on main at `534b4eb5`. Claude works section A of `sdlc/planning/issue-backlog-2026-09-25.md` with one owner per ticket. Tickets that touch different files run in parallel, and their builds take turns under the heavy lock.

- 0123: relate splits requests to fit the backend (backlog A3).
- 0124: the answer cache's three fixes (A2).
- 0125: audit is complete, and diff warns when nothing pairs (A6 and A4).
- 0126: command wording, help, and the wrong doc claims (A7 and the first part of A8). It also states in the docs that `status` counts only command spend today (A5, the docs half).
- 0127: test harness fixes before 0119 (A9). Test children get an allow-listed environment and never inherit the shell.
- 0128: release and install (A10), with publishing through GitHub Actions under Ian's ruling.
- 0129: warm takes the question file decide uses and ignores its not-sure range (`2026-09-25-warm-refuses-the-question-file-decide-uses.md`).

After pandas 0122 lands: the Python Polars cells fix (A1), spend recorded by the libraries (A5), removing settings that do nothing, and Polars as an optional feature of `thinkthen`. They touch the Python code or every surface.

Packing is in 0.1, following the marketing request and Ian's "everything is in 0.1". Its design ticket starts when experiment 261 reports. It adds pack as a setting on every surface, and a design record picks the default. The default stays at one row per request until that record rules. Ian can overturn this.

0119 runs after 0127 lands. The release build runs last.
