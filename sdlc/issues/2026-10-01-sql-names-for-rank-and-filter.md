# SQL named forms `thinkthen_rank` and `thinkthen_filter`

Status: open for `thinkthen_filter` only. Filed 2026-10-01 from the site owner's mailroom message "SQL names for rank and filter". Owner: the queue owner, in 0.2. Ian decides.
Kind: idea
When: 0.2 work starts on main
Milestone: 0.2

Named SQL forms would let a SQL reader find all ten functions by name.

DuckDB, SQLite and PostgreSQL carry a `thinkthen_` function for eight of the ten functions. `filter` and `rank` have none. The READMEs teach `WHERE` and `ORDER BY` over decide's probability, which works. A SQL reader still looks for the ten by name. A deck card for "Top N per group" first reached for DuckDB's `thinkthen_probability`, and Ian asked why it did not say `rank`.

The idea: add `thinkthen_rank` and `thinkthen_filter` to each extension as thin forms over decide's probability.

The cost: two more functions in each of three extensions, each with its tests and docs. Both would duplicate what `WHERE` and `ORDER BY` already do.

## Rank shipped in 0.1

Ian approved `thinkthen_rank` for 0.1 on 2026-10-01. Ticket 0378 added it to all three extensions as a keyed table of `(key, rank, probability)` rows and retired DuckDB's `thinkthen_probability`. The rank half of this idea is done.

Only `thinkthen_filter` stays a 0.2 idea for Ian. Until he decides, the docs keep teaching `WHERE` over decide for filter.
