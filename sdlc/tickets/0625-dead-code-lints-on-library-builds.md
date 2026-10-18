# 0625: Turn dead-code warnings back on for library builds and drop stale excuses

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- The blanket allow in `lib.rs` is gone. Code reached only by the CLI sits behind the `cli` feature, so the library build reports no dead code.
- Each stale `allow(dead_code)` or `expect(dead_code)` in `crates/thinkthen/src` either goes with the code it covers, or its reason names the current caller.
- The lint gate builds the core both with and without default features.

### Proof

- On the M5 through pm job, `sdlc/scripts/lint` passes for the core with default features and with `--no-default-features --features bundled-sqlite`, with warnings denied.
- The installed C library and the Rust library consumer still pass their routine checks.

## Evidence

`crates/thinkthen/src/lib.rs:3-10` sets `allow(dead_code, unused_imports)` whenever the `cli` feature is off. Every binding and database extension builds the core with `default-features = false` (`libraries/c/Cargo.toml`, `libraries/python/Cargo.toml` and others), so dead library code is never reported.

Dead-code exceptions also point at work that already landed:

- `crates/thinkthen/src/engine/error.rs:24-44,145-147,249-251` keeps error kinds "before public engine errors land".
- `crates/thinkthen/src/engine/mod.rs:30,203,228` and `crates/thinkthen/src/engine/facade.rs:326` say "tickets 0084 through 0086 expose it". Ticket 0086 landed on 2026-09-24.
- `crates/thinkthen/src/core/answer.rs:42` keeps a "generic default for later adapters".
- `crates/thinkthen/src/public/request/input.rs:314` expects dead code.

## Progress

- 2026-10-18 started
