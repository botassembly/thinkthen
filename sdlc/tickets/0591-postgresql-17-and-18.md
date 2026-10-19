# 0591: Prove the PostgreSQL extension on PostgreSQL 16, 17 and 18

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

Reviews: revision f9cefb6, accept

## Outcome

- Changes: `databases/postgresql/README.md`.
- The PostgreSQL extension ships and passes its installed checks on PostgreSQL 16, 17 and 18, one release archive per major and target.
- The README's check paragraph matches the code: the routine `surfaces` run checks all three majors, not only 16 and 18.
- Keeps every PostgreSQL function, row shape and error sentence across majors. The site's `smoke-sql` replay and the SQLite complete-suite PostgreSQL host stay on 16.

### Proof

- At the 0619 container checkpoint, in the arm64 container on the M5 and the x86_64 container on Yellow: `sh databases/postgresql/check.sh 16`, `17` and `18` pass.
- The release rehearsal in 0687 produces `thinkthen-postgresql16-…`, `thinkthen-postgresql17-…` and `thinkthen-postgresql18-…` archives per target, `release-smoke` installs each, and `install-check postgresql` runs all three majors. macOS majors are proven there too.

## Evidence

- Ian, 2026-10-15: "definitely Postgres 17 and 18." PostgreSQL 18 was released in September 2025; 17 in September 2024.
- PM decisions of 2026-10-15 stand: one extension source built per major through pgrx features `pg16`, `pg17` and `pg18`; archives named `thinkthen-postgresql16-…`, `17-…` and `18-…`; the current patch release of each major pinned in each route. Server majors are minimum host versions, which the 2026-10-19 no-version ruling keeps.
- The build, packaging and pins are already on main, landed 2026-10-15 in `e53689fd7`, `1a65417f8` and `6f9f78c10`:
  - `databases/postgresql/Cargo.toml` declares `pg16`, `pg17` and `pg18` on `pgrx = "=0.17.0"`.
  - `check.sh MAJOR`, `package.sh MAJOR`, `runtime.sh` and `setup.sh` take the major; `runtime.url` and `runtime.sha256` pin Ubuntu 16.15 and PGDG 17.11 and 18.6 debs; `runtime-darwin.env` pins Homebrew bottles per major.
  - `sdlc/scripts/surfaces` loops `majors='16 17 18'`. `sdlc/scripts/release-pack`, `release-container`, `release-smoke`, `publish-builds`, `registry-readmes.py` and `.github/workflows/release.yml` handle each major.
  - `sdlc/scripts/install_check_channels.py` `postgresql` installs all three majors, with `install_check_tools.py` `postgres_tools` reading the pins.
  - `databases/postgresql/README-PUBLIC.md` and `site/src/data/catalog.mjs` name all three archives.
- What remains is the proof above and one stale README sentence: `databases/postgresql/README.md` says "The routine surfaces run checks 16 and 18; the release suite checks all three."
- The pinned PostgreSQL servers run in the Linux container, so the check runs at the 0619 checkpoint and not natively on the M5.
