Status: Closed by ticket 0316. ADR 0105 landed first, so all three databases now take find's `none` in the settings object, and the row says so. The same ticket fixed the stale SQL cells for context, deadline, batch and SQLite's engine setters, which moved into `thinkthen_configure`. Filed 2026-09-29 by the marketing lead from wave 4 release QA preparation (workspace experiment 218).

# The settings page says only PostgreSQL's find takes `none`

`specification/settings.md` line 57, the "None option" row, gives the SQLite and DuckDB cells as "not on this surface". It gives PostgreSQL an "optional final `none boolean`".

The landed signatures disagree:

- `databases/sqlite/README.md:29`: `thinkthen_find(question, units_json[, none[, deadline_ms]])`
- `databases/duckdb/README.md:12`: `thinkthen_find(question, units[, none[, deadline_ms]])`
- `databases/postgresql/README.md:15`
- `sdlc/records/0224-sql-find-preflight.md:13` and `sdlc/records/0224-duckdb-find-preflight.md:9` record the `none` slot on SQLite and DuckDB.

On SQLite and DuckDB the `none` slot is not final, because `deadline_ms` follows it. The page is settled and feeds the site's generated Settings page, so users read the wrong row. `sdlc/scripts/settings` checks fixed engine cells only (lines 41 to 54 and 144 to 164), so no gate catches this row.

ADR 0105, when accepted, moves `none` into the settings object on all three databases. The row then changes again.

## Done when

The "None option" row matches each database's landed `thinkthen_find` signature. If ADR 0105 lands first, the row matches its settings form.
