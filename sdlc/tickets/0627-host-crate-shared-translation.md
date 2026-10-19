# 0627: Make the host crate a plain shared translation layer

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0626, 0643

## Outcome

- The host crate holds only translations shared by two or more hosts (the C library and the three database extensions), each with docs and `Debug`.
- Legacy and frozen entry points are deleted. Code that only one host needs moves into that host.
- The crate-wide lint allows and the host-specific dead-code exceptions are gone.
- The crate shares the root workspace lock, unless a documented build reason forces a separate one.

### Proof

- On the M5 through pm job, the installed C library and the installed SQLite, DuckDB and PostgreSQL extensions pass their routine checks against the loopback backend.
- `sdlc/scripts/lint` passes for the host crate with warnings denied.

## Evidence

`crates/thinkthen-host/src/lib.rs:1-7` describes the crate as "private compatibility transports" with "legacy entry points" and a "frozen C translation". It turns off missing-docs and missing-Debug lints on that basis, which the no-compatibility ruling no longer supports.

Its dead-code exceptions are tied to particular hosts:

- `crates/thinkthen-host/src/complete_native/settings.rs:54,83`: "used by PostgreSQL and DuckDB".
- `crates/thinkthen-host/src/complete_native/observations.rs:90`: "DuckDB owns native session snapshots; SQLite retains borrowed observation callbacks".
- `crates/thinkthen-host/src/complete_native/file_format.rs:21`: "only PostgreSQL and DuckDB transport native reader descriptors".

The crate keeps its own workspace and lock file (`crates/thinkthen-host/Cargo.toml`, `crates/thinkthen-host/Cargo.lock`), and it pins serde versions apart from the core.

## Progress

- 2026-10-19 started
