# The macOS artifact evidence

Copied 2026-09-22 from the build lanes' ignored `dist/` folders, so the
evidence is committed rather than living only on the guest and in the
packaging notes (`sdlc/records/surfaces-notes/NOTES-packaging.md`).

Provenance, stated because it matters: these logs were captured on the
Mac against branch commit `c3616dc`, the tip when the macOS lanes ran —
NOT the current tip, and the third review correctly flagged the
hand-written summary for saying so nowhere. The artifacts prove the
darwin-arm64 build path of that commit; the current tip's Mac story is
re-proven by `scripts/gate-macos.md` when a Mac is next available. The
`ruby.log` here is the later, successful visit's log.

- `typescript.log` — the TypeScript tarball built on macOS 26.4 arm64,
  installed into a scratch app, five examples green.
- `ruby.log` — the Ruby gem built on the same guest (Ruby 3.4.10 built in
  the scratch), full suite green.
- `duckdb.log` — the DuckDB extension built as `osx_arm64` and loaded by
  the official v1.5.5 macOS CLI.

The SQLite and PostgreSQL macOS artifacts were verified on the guest with
their commands and outputs recorded in the packaging notes; no separate
log file was kept for them.
