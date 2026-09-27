# Three items from the named-answers site landing

Status: Closed on 2026-09-27. Filed 2026-09-26 by the marketing lead after site commit 648a965f. Item 1 is done: commit `af0cd7dd` moved the ownership rule to a planning page, and `AGENTS.md` holds 4,994 characters. Item 2 is done: Quick Fix qf-config-ruby-issue-status moved the issue to `closed/2026-09-26-site-samples-assert-on-unnamed-answers.md`. Item 3 is answered below. The sample fixes it asks for are the site's, and `2026-09-27-site-recognize-samples-after-the-three-steps.md` carries them.

1. **Lint fails on main.** `sdlc/scripts/lint` reports AGENTS.md at 5641 of 5000 characters. Every site landing now fails the lint rung on a file the site may not edit.
2. **Issue ready to close.** `sdlc/issues/closed/2026-09-26-site-samples-assert-on-unnamed-answers.md` is resolved by 648a965f. Every site sample now names its answer, and `site/scripts/check-samples.mjs` fails on an unnamed or generic one. The site owner may not move files in `sdlc/`, so please close it.
3. **Which recognize call does each database extension register?** The SQLite recognize sample calls `thinkthen_relations`, which the SQLite extension does not seem to register. The DuckDB sample calls it in `FROM`, but DuckDB registers it as a scalar function. Please say the right call for each extension, or point to the spec page that says it. The site owner then fixes the samples.

## The answer to item 3

Read from the registration code on main. Each database README's function table says the same.

- **DuckDB** registers `thinkthen_recognize(text, kinds)` and `thinkthen_relations(text, file)` as scalar functions (`databases/duckdb/src/scalars.rs`). Each returns a list of structs for one text, so a query calls it in the `SELECT` list and uses `unnest` for one row per name or relation. Neither works in `FROM`. `thinkthen_relate(query, rules)` is the one table function (`databases/duckdb/src/relate/ffi.rs`), and it goes in `FROM`.
- **PostgreSQL** registers three set-returning functions (`databases/postgresql/src/lib.rs` and `relate.rs`). `thinkthen_recognize(body, kinds text[])` or `(body, spec)` returns name rows. `thinkthen_relations(body, spec)` returns relation rows. `thinkthen_relate(query, rules text[])` or `(query, spec)` returns edge rows. The first two read one text, so a query joins them with `LATERAL`. `thinkthen_relate` goes in `FROM`.
- **SQLite** registers two table-valued functions and no `thinkthen_relations` (`databases/sqlite/src/ffi.rs`). `thinkthen_recognize(text, kinds)` returns name rows and goes in `FROM`. A recognize spec that holds relations raises `usage`. Relations come from `thinkthen_relate(table, id, name, kind, rule, ...)`, which reads a named table of names and goes in `FROM`.

On main, `site/examples/functions/recognize/sqlite.sql` calls `thinkthen_relations`, which SQLite does not register. `site/examples/functions/recognize/duckdb.sql` calls `thinkthen_relations` in `FROM`, where DuckDB's scalar cannot go. The PostgreSQL sample's `LATERAL` calls match.
