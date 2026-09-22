# The macOS artifact evidence

Copied 2026-09-22 from the build lanes' ignored `dist/` folders, so the
evidence is committed rather than living only on the guest and in the
packaging notes (`sdlc/records/surfaces-notes/NOTES-packaging.md`).

- `typescript.log` — the TypeScript tarball built on macOS 26.4 arm64,
  installed into a scratch app, five examples green.
- `ruby.log` — the Ruby gem built on the same guest (Ruby 3.4.10 built in
  the scratch), full suite green.
- `duckdb.log` — the DuckDB extension built as `osx_arm64` and loaded by
  the official v1.5.5 macOS CLI.

The SQLite and PostgreSQL macOS artifacts were verified on the guest with
their commands and outputs recorded in the packaging notes; no separate
log file was kept for them.
