# 0316: Close the documentation issues

Status: landed. Lane claude-4. Branch `ticket/0316-docs-issues`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8.

## Outcome

The repo's docs match the landed code for the five open documentation issues. Each issue is closed, or says exactly what remains and who owns it.

## Evidence

- Starts from: main `6fe1d57d8`. Issues `2026-09-29-readme-key-backend-and-overhead-lines`, `2026-09-29-settings-page-says-only-postgresql-find-takes-none`, `2026-09-25-docs-how-tos-and-spec-claims-owed`, `2026-09-29-docs-page-for-the-liquid-d1-backend` and `2026-09-29-docs-page-naming-supported-providers`. ADR 0105 has landed: every SQL call takes one settings JSON, find's `none` sits in it, and SQLite's setters moved into `thinkthen_configure`. Experiment 413's Liquid evidence lives in the null-criteria issue and ticket 0301.
- Keeps: every settings row not named below; the README's key, address and model lines; `site/`, which marketing owns; `spec/` and `conformance/`, which ADR 0111 slice 1 is changing.
- Changes: `specification/settings.md` gives the SQL cells for find's none, context, deadline and batch in the settings JSON, SQLite's engine cells as `thinkthen_configure` keys, and drops stale warm mentions. `README.md` gains one Liquid d1 bullet that claims only the recorded check and links Awesome ThinkThen. The settings issue closes. The README issue stays open for the overhead sentence, which waits on marketing's benchmark. The how-tos issue shrinks to its eight unwritten pages. The Liquid and providers issues stay open with a note for marketing, because their pages belong in `site/`.
- Proof: `sdlc/scripts/settings` with the built command on `PATH` reports 0 failures. Each changed cell was read against `databases/*/README.md` and `databases/*/src`. `sdlc/scripts/lint` and `sdlc/scripts/tickets` pass.
- Defers: the overhead sentence; the Liquid hosted recheck; marketing's Liquid and providers pages; the eight how-to pages; architect review 05, whose open part is code.

## What the build taught us

The settings issue was one symptom of a wider drift. ADR 0105 replaced positional SQL slots and SQLite's twelve setters, and `sdlc/scripts/settings` passed anyway because it checks only that an engine word appears in source. Read each SQL column against the database README after a call-shape change. Code review caught two cells the builder still got wrong: SQLite's cache off is `false`, and DuckDB and PostgreSQL also take `batch` in the settings JSON. Check each spelling in the parser, not only the README.

The "Killed" line in `lint` comes from the `time-limit` self-test in `surfaces --registry`, which kills a planted child that ignores TERM. It is expected and does not mean the machine ran out of memory.
