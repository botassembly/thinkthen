# 0702 — SQLite triggers and views call ThinkThen by default

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `databases/sqlite/src/`, `databases/sqlite/README.md`, `databases/sqlite/entry-points.txt`, `databases/sqlite/tests/`, `site/src/pages/databases/`.
- Loading the SQLite extension the normal way (`.load ./thinkthen`) registers every ThinkThen function so that triggers, views, CHECK constraints and column defaults can call it. A developer labels records as they arrive with an `AFTER INSERT` trigger, and later queries filter the saved columns with plain SQL and no model call.
- One registration mode remains. The separate trusted entry point and the direct-only default are deleted, with no compatibility alias (no compatibility before 1.0).
- SQLite's own `trusted_schema` setting still applies: a host that opens database files it did not create sets `PRAGMA trusted_schema=OFF`, and SQLite then refuses the functions inside that file's schema. The SQLite guide states this in two sentences, and nothing else changes.

### Proof

- An installed-extension test creates a table and an `AFTER INSERT` trigger that calls `thinkthen_choose` against the loopback backend, inserts a row, and reads the saved label. It fails before the change with `unsafe use of thinkthen_choose()`.
- The same test with `PRAGMA trusted_schema=OFF` gets SQLite's refusal and sends no request, counted at the loopback.

## Evidence

The extension registers its functions with `SQLITE_DIRECTONLY` (`databases/sqlite/src/scalars.rs`, `budget.rs`, `catalog.rs`, `files.rs`), so SQLite refuses them in triggers, views, CHECK constraints and defaults. A user building a demo hit `unsafe use of thinkthen_choose()` on an `AFTER INSERT` trigger. The guard protects someone who opens a database file another person wrote, so it blocks the main use, labeling records ahead of time, to protect a rare one, and SQLite's standard `trusted_schema` setting already covers that rare case. Ian, 2026-10-19, asked for schema use to work by default. An opt-in trusted entry point exists on main since 0422 but needs a special load call nobody finds.

PostgreSQL and DuckDB get the same check once their own trigger behavior is measured.
