# Quick Fix qf-rust-195: move the Rust toolchain from 1.93.1 to 1.95.0

Status: prepared on `ticket/qf-rust-195` from main `665975c5`. It lands right after 0086, per `sdlc/planning/one-line-plan-2026-09-24.md`. Before landing, main merges in, any new 1.95 lint in 0086's code gets fixed, and the ladder reruns. A fresh read-only Opus review is in `sdlc/records/qf-rust-195-review.md`. Ian can overturn the move.

## Why

Ticket 0120 builds Rust Polars on `polars` 0.55. That release does not build on 1.93.1. `polars-compute` 0.55.2 uses `array_windows`, and `sysinfo` 0.39 requires 1.95 (spike logs 01 and 06). The 0120 reviewer asked for the real move to raise each package's `rust-version`, run clippy against it, and run the full ladder.

## Evidence

Spike 257, local experiment 257, copied main and changed only the channel in `rust-toolchain.toml` to 1.95 (`scripts/bump-1.95.sh`). Under rustc 1.95.0 (`59807616e`), deny, format, clippy, docs, tests, and doc tests passed (`logs/07-bump-*.log`). The spike left `rust-version` at 1.93.1 and did not run `install`, `spec`, or the demos.

## Change

- `rust-toolchain.toml` pins channel `1.95.0`. `policy.py` requires an exact three-part release.
- The root `Cargo.toml` sets `[workspace.package] rust-version = "1.95.0"`. `crates/thinkthen` and `conformance/backend` inherit it. Clippy now reads 1.95.0 as each package's minimum Rust version. `policy.py` checks it equals the channel.
- `sdlc/planning/libraries/rust.md` names the new pin.

Nothing else pins the version on main. `.github/workflows/gate.yml` installs whatever `rust-toolchain.toml` names. `sdlc/scripts/package` reads the channel from that file. `Cargo.lock` did not change. Dated records and issues that name 1.93.1 stay as written. The Ruby Dockerfile pin (R7-10) lives on the surfaces branch, and its port picks up the new pin.

No code changed. Clippy and rustc 1.95.0 raised no new warning.

## Checks

On rustc 1.95.0, with the heavy lock the scripts take:

- `sdlc/scripts/install` exited 0.
- `sdlc/scripts/lint` exited 0: policies, ratchet, format, clippy `-D warnings`, docs, doc tests, the package check, and `cargo deny` (advisories, bans, and licenses ok).
- `sdlc/scripts/test` exited 0: 839 passed, 0 failed, 10 ignored.
- `sdlc/scripts/spec` exited 0: the spec pages and 21 demos green, 0 red.

`sdlc/scripts/live` did not run.

## Landing

Landed 2026-09-24 ahead of 0086. On the main checkout, `lint` failed after the merge. The private-surface probe in `sdlc/scripts/package` took the first `libthinkthen-*.rlib` it found, which was the 1.93.1 build, and rustc refused it (E0514). The fix takes the newest build. A clean worktree never showed the failure, so the reviewed ladder still holds. After the fix, lint passed on the main checkout. The coordinator reviewed the two-line fix alone. Neither reviewer saw it.
