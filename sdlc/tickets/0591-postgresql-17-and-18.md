# 0591 — postgresql-17-and-18

Status: OPEN.

Milestone: 0.2

## Outcome

The PostgreSQL extension ships and passes its installed checks on PostgreSQL 16, 17 and 18.

## Evidence

- Ian, 2026-10-15: "definitely Postgres 17 and 18." PostgreSQL 18 was released in September 2025; 17 in September 2024.
- Starts from `ticket/0543-final`: `databases/postgresql/Cargo.toml` builds only the `pg16` feature on `pgrx = "=0.17.0"`, `databases/postgresql/check.sh` pins PostgreSQL 16.15 headers and server and names the output `thinkthen-pg16`, and `.github/workflows/release.yml` packs one PostgreSQL archive per target.
- 0543 depends on this ticket.

## Decisions

- PM decision, 2026-10-15: one extension source, built once per major version through pgrx features `pg16`, `pg17` and `pg18`, with one release archive per major version and target. No code branches per version unless pgrx requires one.
- PM decision, 2026-10-15: pin the current patch release of 17 and 18 the same way 16.15 is pinned, on Linux and on the macOS Homebrew route.

## Scope

- Changes: `databases/postgresql/Cargo.toml`, `databases/postgresql/check.sh`, `databases/postgresql/README.md`, the PostgreSQL runtime pins beside `check.sh`, `.github/workflows/release.yml`, `.github/workflows/install-check.yml`, `sdlc/scripts/release-pack`, `sdlc/scripts/install_check_channels.py`, `release-notes/0.2.0-known-gaps.md` and the PostgreSQL site page.
- Checklist:
  1. Add the `pg17` and `pg18` features and build each.
  2. Run the existing installed check once per major version; the routine profile runs 16 and 18, the release suite runs all three.
  3. Pack and name one archive per major version; the README and site say which archive matches which server.
- Keeps: every PostgreSQL function, row shape and error sentence, byte for byte across versions.

## Proof

`databases/postgresql/check.sh` passes against PostgreSQL 16, 17 and 18 servers, and the release rehearsal produces three PostgreSQL archives per target.

## Defers

Nothing.
