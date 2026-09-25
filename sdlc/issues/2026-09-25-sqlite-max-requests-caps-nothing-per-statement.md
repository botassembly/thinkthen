# SQLite: `thinkthen_max_requests` caps nothing per statement

Status: Decided 2026-09-25. Ian chose option 3 in the exact form below, recorded in `sdlc/planning/one-line-plan-2026-09-25.md` on main at `f78421be`. Ticket 0109 decision 17 builds it for SQLite.

Filed 2026-09-25 by the ticket 0109 build, from its code review.

## The problem

`thinkthen_max_requests(n)` sets the engine's `max_requests`. The engine applies that limit to one engine call. In SQLite each scalar row is its own one-record call. Each `thinkthen_warm` flush is one call of up to 256 rows. So `WHERE thinkthen_decide(…)` over a million rows sends a million requests under any limit of 1 or more. A warm pass over a large table sends every flush under any limit of 256 or more. A user who reads the setting as a spending cap is wrong. The README and ADR 0047's SQLite section now say so. DuckDB and PostgreSQL run scalar rows the same way and likely share the gap.

## Options

1. **Keep it per call, and say so.** No code. The setting stays a guard on one batch, as in the Rust API. Cost: a user has no spending cap in SQL.
2. **A cap per statement.** The extension counts sends per SQL statement and refuses the row that would cross the limit. SQLite gives a function no clean statement boundary. A count keyed by `sqlite3_stmt` through `sqlite3_context_db_handle` and statement state is possible but fragile. Cost: about 60 lines and a new test per surface. It still leaves a script of many statements uncapped.
3. **A cap per process.** The extension counts every request its engines send and refuses once the total is spent. It reuses `Engine::usage().requests_sent()`, which the engine already keeps. It holds across statements and connections. The cache answers do not count, so a warmed query costs nothing. As first written, this option overstated the bound: checking the count only before a call lets a warm flush of 256 rows run past the total, and retries add more.

## Ian's ruling

A cap per process, in this exact form. Each database surface adds a total setting, `thinkthen_max_requests_total` in SQLite, unset by default. Before each engine call the extension sums the requests its engines in this process have sent. Once the total is spent, it refuses as `usage` with zero sends. Otherwise it passes the remaining total as that call's `max_requests`, together with any smaller `thinkthen_max_requests`.

## The bound as built

- A scalar row or a warm flush can pass the total only by that call's retries. The engine retries a failed send twice (`max_retries: 2`), so one call can add at most two sends per record past its limit.
- A `thinkthen_recognize` or `thinkthen_relate` call counts as one record but may send several requests. It can pass the total by that call's own requests and their retries.
- A forked child starts again from zero. A probe on ticket 0109 measured it: the parent sent 1 under a total of 2, and the child still sent 2.
- A spent total refuses every later call, even one the cache could answer.

## The recommendation as filed

Option 3. It is the only one that bounds money in plain words, and it is the smallest change. The name would change to say what it does, such as `thinkthen_max_requests_total`, and the per-call meaning would drop from SQL. The same rule should go to DuckDB and PostgreSQL. Ian can choose option 1 to keep the surfaces matched with the Rust API for 0.1.
