# The named-answers lint fails on three database READMEs

Status: closed after fresh Medium review accepted `9df4e2cc`.

Filed 2026-09-28 by the marketing lead's site builder.

## What fails

`node sdlc/scripts/named-answers.mjs` exits with three findings on main at `ff673422`. The lint rung runs it.

- `databases/duckdb/README.md:34`: `thinkthen_find` has no alias.
- `databases/postgresql/README.md:51`: `thinkthen_choose` has no alias.
- `databases/sqlite/README.md:62`: `thinkthen_choose` has no alias.

The site's own copy of the rule passes. The site does not read these files.

## What would fix it

Give each call an alias named for its meaning, such as `AS team`, and filter or sort on that alias. The rule is in `site/WRITING.md` under "Name each answer".

## Resolution

The DuckDB example names and filters `best_passage`. PostgreSQL and SQLite name `team` before inserting, preserve nullable answers, and filter on the stored name. The complete named-answer check now passes over 225 blocks in 86 pages. No SQL function or website page changed. See [the reviewed record](../../records/qf-prepared-contract-page-gaps.md).
