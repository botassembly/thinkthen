# SQLite: `thinkthen_max_requests` caps nothing per statement

Status: Open. Ian decides, because it concerns spending.

Filed 2026-09-25 by the ticket 0109 build, from its code review.

## The problem

`thinkthen_max_requests(n)` sets the engine's `max_requests`. The engine applies that limit to one engine call. In SQLite each scalar row is its own one-record call. Each `thinkthen_warm` flush is one call of up to 256 rows. So `WHERE thinkthen_decide(…)` over a million rows sends a million requests under any limit of 1 or more. A warm pass over a large table sends every flush under any limit of 256 or more. A user who reads the setting as a spending cap is wrong. The README and ADR 0047's SQLite section now say so. DuckDB and PostgreSQL run scalar rows the same way and likely share the gap.

## Options

1. **Keep it per call, and say so.** No code. The setting stays a guard on one batch, as in the Rust API. Cost: a user has no spending cap in SQL.
2. **A cap per statement.** The extension counts sends per SQL statement and refuses the row that would cross the limit. SQLite gives a function no clean statement boundary. A count keyed by `sqlite3_stmt` through `sqlite3_context_db_handle` and statement state is possible but fragile. Cost: about 60 lines and a new test per surface. It still leaves a script of many statements uncapped.
3. **A cap per process.** The extension counts every request its engine sends and refuses once the total reaches the limit. It reuses `Engine::usage().requests_sent()`, which the engine already keeps. Cost: about 20 lines in `settings.rs` and one test. The limit then means "this process spends at most n requests". It is simple to explain and holds across statements and connections. The cache answers do not count, so a warmed query costs nothing.

## Recommendation

Option 3. It is the only one that bounds money in plain words, and it is the smallest change. The name would change to say what it does, such as `thinkthen_max_requests_total`, and the per-call meaning would drop from SQL. The same rule should go to DuckDB and PostgreSQL. Ian can choose option 1 to keep the surfaces matched with the Rust API for 0.1.
