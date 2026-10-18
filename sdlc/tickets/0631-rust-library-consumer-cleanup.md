# 0631: Run the Rust library's outside consumer without pins or shims

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- Changes: `libraries/rust/check.py`, `libraries/rust/check.sh`, `sdlc/scripts/coverage_installed_extensions.py`, `sdlc/scripts/package`.
- The installed-crate check unpacks the crate and points the consumer at it through a `[patch]` entry or a generated manifest, not a string replace.
- The consumer builds with the toolchain the crate's `rust-version` declares, with no inline launcher script.
- The coverage path remaps leave this gate. Coverage does not gate a release, so the coverage script sets them only if it still needs them.

### Proof

On the M5 through pm job, the gate unpacks the packaged crate, builds the consumer against it, and passes the shared cases.

## Evidence

`libraries/rust` is examples plus an outside consumer program over `crates/thinkthen`, and it is the healthiest library. Its gate still carries special cases:
- It pins the consumer to the root compiler through an inline Python launcher that reads `rust-toolchain.toml` and sets `RUSTUP_TOOLCHAIN` (`libraries/rust/check.sh:20-32`; commits 89ead698f and 4fe00b9e8).
- It rewrites the consumer manifest path with a string replace (`check.sh:42-47`).
- It sets coverage path remaps through `CARGO_ENCODED_RUSTFLAGS` (`check.sh:48-52`).

The consumer's own projection code goes in 0612, and its packet dump comes with 0642.

## Progress

- 2026-10-18 started
