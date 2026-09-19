# ADR 0002: Strict gates and a pure core from the first commit

- Status: Accepted
- Date: 2026-09-18

Ian can overturn this cheaply. It applies his Rust ideal state and copies the strictest repository in his workspace.

## Context

Ian writes no Rust and cannot review it. Gates stand in for him. A survey of ten Rust repositories found one strict lint posture and several weaker copies. It found size ratchets that pass quietly because no ceiling was declared. It found `expect` calls standing in for the `unwrap` calls a lint had denied. Every repository that adopted strictness late carries that debt.

## Decision

**The strict lint table applies from the first commit.** `unwrap`, `expect`, `panic`, indexing, printing, `todo`, and `dbg` are denied. Unsafe code and unreasoned `allow` attributes are forbidden. `lint` compares the tables against accepted copies.

**The size ceiling is declared from the first commit** and equals the measured total. No file passes 500 non-blank lines.

**The workspace has two crates.** `thinkn-core` is pure and its `clippy.toml` bans files, the environment, sockets, clocks, processes, and dynamic JSON. `thinkn` is the binary and owns every edge. The split follows a real dependency direction.

**The starting dependencies are `serde`, `serde_json`, `thiserror`, `clap`, and `ureq`.** Requests run in parallel on threads. No async runtime enters without a measurement.

**No gate touches the network.** Tests replay recorded responses.

`rust-standards.md` holds the full rule list and names each enforcing tool.

## Consequences

Early slices move slower. Each new line edits the ceiling, and each dependency needs a second reader. The repository never has to pay for a late cleanup.
