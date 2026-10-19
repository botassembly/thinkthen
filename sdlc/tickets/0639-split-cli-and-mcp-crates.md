# 0639: Move the command-line tool and MCP server out of the core crate

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0625, 0629

## Outcome

- The library crate holds only the library. The command-line tool and the MCP server live in their own crate (or crates) and use only the library's public API. Any private access they need becomes public API or moves with them.
- The `cli` feature and the `cfg(feature = "cli")` blocks in the library crate are deleted.
- The MCP server executes tools through the Rust run function from 0629, with no session loop of its own.
- The released binary name, install path and behavior do not change.

### Proof

- On the M5 through pm job, the packaged `thinkthen` binary from the release suite (`package`) passes `conformance/check-cli.sh` and the spec page replays (`sdlc/scripts/spec`). The MCP stdio checks pass against the installed binary.
- The installed C library and the Rust library consumer pass their routine checks.

## Evidence

`crates/thinkthen` holds the library, the command-line tool (`src/cli`, `src/main.rs`) and the MCP server (`src/mcp`), switched by the `cli` feature (`crates/thinkthen/Cargo.toml`, `src/lib.rs:52-67`). The crate is about 107,000 source lines and exports about 216 public type names. Every binding builds it with default features off and depends on the feature switch to leave the CLI out. `lib.rs:61-64` pulls CLI modules into the crate root (`args`, `asking`, `edge`, `failure`, `judge`, `profile`), so CLI and library code can reach into each other.

The CLI is among the most churned code since 2026-09-15: `cli/args.rs` 68 commits, `cli/args/command.rs` 62, `cli/asking.rs` 59, `cli/mod.rs` 57, `cli/failure.rs` 51. Each of those changes rebuilds and re-reviews the library crate that every binding uses.

## Progress

- 2026-10-19 started
