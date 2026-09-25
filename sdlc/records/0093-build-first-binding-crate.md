# 0093 build: the Rust examples as the first binding crate

Built 2026-09-24 by Claude (Opus) on `ticket/0093-first-binding-crate`, over a merge of `origin/ticket/0086-public-rust-api` at `4603df17`. Code review: a fresh read-only Claude (Opus) session. The first pass rejected on one finding. The second pass, at `f52f1ee4`, accepted.

## What landed

- `libraries/rust` is its own Cargo workspace. The package `rust-examples` depends on `thinkthen` by path with default features off. It holds one program per function (`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `recognize`, `relate`) plus the deck's slide, each with a `.txt` file that pins its output. `tests/examples.rs` runs each built program in its own process. The process gets a cleared environment, a loopback base, a fake key, and a fresh cache. The test compares stdout with the pinned file and requires at least one backend request. The stand-in forwarder, the Polars door, the conformance runner, and the testkit tests are gone. The shared cases stay in `conformance/consumer`.
- The root `Cargo.toml` excludes `libraries` and `databases` and sets `default-members = ["crates/thinkthen"]`. `policy.py` checks both.
- `policy.py` checks every `libraries/*/Cargo.toml` and `databases/*/Cargo.toml` for these rules:
  - `publish = false`, and the root edition and rust-version;
  - one `thinkthen` dependency, by path, with default features off, including a renamed one;
  - no dependency on another binding through any table: package, target, workspace, or patch;
  - the root lint table with `unsafe_code = "deny"`, and the shared `clippy.toml`;
  - the root release profile;
  - thinkthen's resolved tree held to the root lock's versions;
  - `unsafe` only in `ffi.rs`;
  - no `#[ignore]`, and no test that returns before its first assertion.
- `ratchet.mjs` takes an optional config path. With no argument it reads `sdlc/ratchet.json` as before. It now skips folders named `target`.
- `sdlc/surfaces.txt` lists the nine surfaces. `sdlc/scripts/surfaces` is rung 4. It starts one loopback backend and runs each landed `check.sh` with the port. Exit 77 reports "not run", and any other failure fails the rung. `surfaces --registry` runs from `lint`. It refuses a landed surface with no `check.sh` and a binding folder with no landed line. It also runs and plants each `ratchet*.json` file, and runs `cargo deny` over each landed lock.
- ADR 0047's surface checklist now lists the eleven items this crate follows.

## Proof

- Ladder at `f52f1ee4`: `install`, `lint`, `test`, `spec`, and `surfaces` each exited 0, under the heavy lock, at load below 10.
- A plain root `cargo build --message-format=json` compiled only `crates/thinkthen` (its library and binary) among local packages.
- A bare `cargo test` in `libraries/rust` ran 1 test and passed. No test is skipped.
- Red first: `tests/examples.rs` failed with "tag.txt pins what tag prints" before the pins existed. With `decide.txt`'s first line changed to `No`, it failed on the exact text.
- Each pinned output follows the generic arm's rule: the first option, level, label, or yes gets 0.9. `decide` and the banded slide answer `Yes`, `choose` answers `billing`, and `score` answers 0.15 (0.05 × 1 + 0.05 × 2). `tag` holds every label, `filter` keeps every record, `rank` keeps input order at 0.90, and `find` picks the first unit.
- Every plant fails `lint` for its own reason. The reviewer confirmed this with a separate harness.
  - `policy.py`: publish = true; default features on; another binding; a renamed thinkthen with default features; a patch toward a binding; unsafe in `src/lib.rs`; overflow-checks = false; ureq at another version in the lock; a test that prints "skipped" and returns; an `#[ignore]` test.
  - `surfaces --registry`: a ceiling one above the count; a landed entry with no `check.sh`.
- By hand: a registry entry whose `check.sh` is missing fails the full rung with exit 1. A check that exits 77 prints "not run". An unregistered `libraries/zz` folder fails the registry. No backend process remains after a rung.

## Budget

- Rust: 261 nonblank lines, under 600. `libraries/rust/ratchet.json` holds 261.
- Scripts: 245 nonblank lines, under 300: policy.py +135, ratchet.mjs +15, lint +2, surfaces 93. `check.sh` adds 16 more.
- No dependency was added to `thinkthen`.
- `sdlc/ratchet.json` holds the measured 60290. The 0086 branch records 60306. That count included 16 lines of build output (`private.rs`) under `conformance/consumer/target`, which the test rung creates. The reader now skips `target`, so the root count no longer depends on which rung ran last.

## Decisions Ian can overturn

- A "not run" surface does not fail the rung. The release checklist (ticket 0111) counts it as a failure.
- The CI workflow runs the first four rungs. `lint` there still runs the registry, ratchet, and deny checks. The surface rung runs by hand until a ticket adds it to CI.
- The ratchet reader skips `target` folders. The other repositories' copies still count them.
- The pinned outputs show the loopback arm's answers, not a model's.

## Deferred

- The test check misses `cfg_attr(…, ignore)`, a skip branch that never returns, and test macros other than `#[test]`. ADR 0047 item 6 records the limits.
- Nothing times out a hung `check.sh`. Add a timeout when the Docker checks arrive.
- The site's Rust samples predate the public API: `sdlc/issues/2026-09-24-site-rust-samples-predate-the-public-api.md`.

## Merge after 0086 landed

`origin/main` at `1b2e9df7` merged in after 0086 landed. `sdlc/README.md` kept main's `flock -o` sentence and names `surfaces` among the heavy rungs. The root ceiling is the measured 60351, which equals main's value with build output skipped. `libraries/rust` moved to `rust-version = "1.95.0"`, and its lock still resolves offline under `--locked`. Ladder at `3deda370`: `install`, `lint`, `test`, `spec`, and `surfaces` each exited 0 at load 10 or below.
