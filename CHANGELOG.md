# Changelog

Every release of every surface shares one version number.

## Unreleased: 0.1.0

The first release. The command, the Rust, C, Python, TypeScript, Ruby, and R libraries, and the SQLite, DuckDB, and PostgreSQL extensions.

The default model is the pinned version `jev-1.13.0`, not the alias `jev-latest`, so a vendor's move of its alias moves no default answer (ticket 0159).

`recognize` finds names in three steps, as ADR 0056 decides. Each name prints `text`, `start`, `end`, `length`, `kind` and `strength` on every surface. The default kinds are gone, so a run with no kinds prints every name as `ENTITY`. `--max-text-bytes` refuses a text over 600,000 bytes before any request (ticket 0147).

### Breaking changes

None. This is the first release.
