# 0680: Make the Polars consumer a workspace member

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0688

## Outcome

- Changes: `libraries/polars/check.sh`, `libraries/polars/consumer/`, `libraries/rust/consumer/Cargo.toml`, `libraries/rust/consumer/src/fixture.rs`, `Cargo.toml`, `Cargo.lock`, `sdlc/scripts/feature-inventory.py`, `sdlc/scripts/feature-inventory-test.py`.
- The consumer is a workspace member, or its lock is generated from the root. `libraries/polars/consumer/Cargo.lock` is deleted, and `feature-inventory.py` stops expecting it.
- The shared fixture is a normal dependency, not a `#[path]` import.
- The installed check points Cargo at the extracted candidate crate through a standard source replacement. The embedded Python rewrite in `check.sh` is deleted.
- The 150 ms sleep in the consumer is replaced by an event.

### Proof

- On the M5 through `pm job run --machine m5 0680 SHA --memory 12 -- sh -c 'sdlc/scripts/package && THINKTHEN_ARTIFACT=$(ls target/package/thinkthen-*.crate) sh libraries/polars/check.sh'`, the installed check builds the consumer against the extracted candidate crate and runs the shared native fixture through a Polars frame.
- `git grep -n '#\[path' libraries/polars` and `git grep -n 'thread::sleep' libraries/polars` find nothing; both find a line on current main.

## Evidence

- **Path rewriting.** `libraries/polars/check.sh` runs an embedded Python block (`RUSTFRAME`) that copies the consumer and rewrites its `Cargo.toml` path and `#[path]` by string replace.
- **Borrowed file.** `libraries/polars/consumer/src/main.rs` imports `libraries/rust/consumer/src/fixture.rs` through `#[path]`.
- **Second lockfile.** It keeps its own 2,362-line `Cargo.lock`, synced with the root by hand (1221af108). `sdlc/scripts/feature-inventory.py` requires that lock for a feature consumer. The root `Cargo.toml` excludes `libraries` from the workspace under ADR 0047; making the consumer a member names it as an exception there.
- **Sleep.** `libraries/polars/consumer/src/main.rs` sleeps 150 ms.
- 0618 already turned the missing-crate exit 77 in `check.sh` into a failure.
