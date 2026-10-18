# 0591 — postgresql-17-and-18

Status: OPEN.

Milestone: 0.2.1

Depends on: 0688

Reviews: revision f9cefb6, accept

## Outcome

The PostgreSQL extension ships and passes its installed checks on PostgreSQL 16, 17 and 18.

## Evidence

- Ian, 2026-10-15: "definitely Postgres 17 and 18." PostgreSQL 18 was released in September 2025; 17 in September 2024.
- Starts from `ticket/0543-final`. `databases/postgresql/Cargo.toml` builds only `pg16` (the default feature) on `pgrx = "=0.17.0"`; pgrx 0.17.0 already offers `pg17` and `pg18` features, so no dependency change is needed.
- The 16.15 pins live in four routes, not in `check.sh` alone:
  - Local Linux check: `databases/postgresql/runtime.url`, `runtime.sha256`, `runtime.sh` and `setup.sh` fetch Ubuntu 24.04's `postgresql-16_16.15-0ubuntu0.24.04.1` deb from Launchpad. Ubuntu 24.04 ships no 17 or 18 server, so take those from apt.postgresql.org's pgdg24.04 debs.
  - Local macOS check and macOS release build: `databases/postgresql/runtime-darwin.env` pins the Homebrew `postgresql@16` bottles. `runtime.sh`, `package.sh`, `sdlc/scripts/release-workflow` and `.github/workflows/release.yml` name `postgresql@16`.
  - Linux release build: `sdlc/scripts/release-container` pins PGDG RHEL 8 `postgresql16` RPMs and writes a pgrx config with only `pg16`. PGDG publishes RHEL 8 trees for 17 and 18.
  - Published install check: `sdlc/scripts/install_check_tools.py` pins PGDG pgdg24.04 debs for 16.15. `sdlc/scripts/install_check_channels.py` unpacks `thinkthen-postgresql16-VERSION-TARGET.tar.gz` and runs `/16/` paths.
- `check.sh`, `package.sh` and `databases/sqlite/tests/complete/postgresql_runtime.sh` read the pgrx output `target/release/thinkthen-pg16`; `check.sh` refuses any `pg_config` that is not 16.
- `sdlc/scripts/release-pack` packs `thinkthen-postgresql16-$V-$TARGET.tar.gz`. `release-smoke`, `release-smoke-source-packages-test`, `publish-builds`, `registry-readmes.py`, `conformance/parity.py` and `conformance/test_parity.py` match that name.
- The site's PostgreSQL page renders from `site/src/data/catalog.mjs` (the archive table) and `site/src/components/SurfacePage.astro` ("built and tested for PostgreSQL 16"). `databases/postgresql/README-PUBLIC.md` ships in the archive and says PostgreSQL 16.
- 0543 depends on this ticket.

## Decisions

- PM decision, 2026-10-15: one extension source, built once per major version through pgrx features `pg16`, `pg17` and `pg18`, with one release archive per major version and target. No code branches per version unless pgrx requires one.
- PM decision, 2026-10-15: name the archives `thinkthen-postgresql16-…`, `thinkthen-postgresql17-…` and `thinkthen-postgresql18-…`, following the existing 16 name.
- PM decision, 2026-10-15: pin the current patch release of 17 and 18 the same way 16.15 is pinned, in each of the four routes above.

## Scope

- Changes: `databases/postgresql/Cargo.toml`, `databases/postgresql/check.sh`, `databases/postgresql/runtime.sh`, `databases/postgresql/runtime.url`, `databases/postgresql/runtime.sha256`, `databases/postgresql/runtime-darwin.env`, `databases/postgresql/setup.sh`, `databases/postgresql/package.sh`, `databases/postgresql/README.md`, `databases/postgresql/README-PUBLIC.md`, `.github/workflows/release.yml`, `sdlc/scripts/release-container`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-pack`, `sdlc/scripts/release-smoke`, `sdlc/scripts/release-smoke-source-packages-test`, `sdlc/scripts/publish-builds`, `sdlc/scripts/registry-readmes.py`, `sdlc/scripts/install_check_channels.py`, `sdlc/scripts/install_check_tools.py`, `sdlc/scripts/install_check_postgresql.py`, `conformance/parity.py`, `conformance/test_parity.py`, `site/src/data/catalog.mjs` and `site/src/components/SurfacePage.astro`.
- Checklist:
  1. Build each feature with `--no-default-features --features pgNN`; `check.sh` and `package.sh` take the major version and read `thinkthen-pgNN`.
  2. Run the existing installed check once per major version; the routine `surfaces` run covers 16 and 18, the release suite covers all three.
  3. Pack, smoke and install-check one archive per major version; the README, the archive README and the site say which archive matches which server.
- Keeps: every PostgreSQL function, row shape and error sentence, byte for byte across versions. The site's `smoke-sql` replay and the SQLite complete-suite PostgreSQL host stay on 16.

## Proof

`databases/postgresql/check.sh` passes against pinned PostgreSQL 16, 17 and 18 servers on Linux and macOS, and the release rehearsal produces and install-checks three PostgreSQL archives per target.

## Defers

The site `smoke-sql` replay and the SQLite complete suite keep one PostgreSQL version, 16; nothing else is deferred.
