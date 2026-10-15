# 0593 — site-version-support

Status: OPEN.

Milestone: 0.2

Reviews: revision 522c859, accept

## Outcome

Every language, database and command page on the documentation site shows a "Supported versions" line, and the languages and databases index pages show one table of every surface's supported versions and platforms.

## Evidence

- Ian, 2026-10-15: the site must include version support per surface, library and database.
- Starts from `ticket/0543-final`: `site/src/data/catalog.mjs` holds one entry per surface. Versions appear today only inside some `install` notes ("Python 3.10 or later.", "Ruby 3.4.", "Needs JDK 22 or later.", "Needs Zig 0.15.2."), many surfaces name none, and DuckDB alone has `supportedVersions`. The pages render through `site/src/components/SurfacePage.astro`, `site/src/pages/languages/[surface].astro`, `site/src/pages/databases/[surface].astro`, `site/src/pages/languages/index.astro` and `site/src/pages/databases/index.astro`.
- The floors the packages declare on 2026-10-15: Python 3.10 (`requires-python`), Node 22 (`engines`), Ruby 3.4 and 4.0 after 0592, R 4.2, Go 1.22, Rust 1.95, JDK 22, .NET 8, Swift 6 on macOS 15, Dart 3.10 below 4.0, PHP 8.3, Zig 0.15.2, C++17, GnuCOBOL 4, GNAT 13.3 with Ada 2022, DuckDB 1.5.6 and 2.0, PostgreSQL 16, 17 and 18 after 0591, SQLite as built from 3.50.0. Native packages ship for Linux x86-64 and ARM64, macOS 15 or later on Apple silicon and Intel, and Windows x86-64 where a channel ships Windows.
- 0543 depends on this ticket; 0591 and 0592 change two rows.

## Decisions

- PM decision, 2026-10-15: add one `versions` field to each catalog entry, holding the runtime floor, any upper limit and the platforms, and render it in one place in `SurfacePage.astro` and in the two index tables. Remove the version words from the `install` notes so each fact lives once.
- PM decision, 2026-10-15: each value is copied from the package's own declaration, the files listed in Evidence. Add no new checking script; the developer verifies each row against its package file, and the reviewer re-checks the rows.

## Scope

- Changes: `site/src/data/catalog.mjs`, `site/src/components/SurfacePage.astro`, `site/src/pages/languages/index.astro`, `site/src/pages/databases/index.astro`.
- Checklist:
  1. Fill `versions` for every surface, including the command, MCP and Polars.
  2. Render the line on each surface page and the table on both index pages.
  3. Take the 0591 and 0592 rows from their merged state.
- Keeps: every other page's text and the site build's existing checks.

## Proof

The site builds, and each surface page and both index pages show the versions that match their package declarations.

## Defers

Nothing.
