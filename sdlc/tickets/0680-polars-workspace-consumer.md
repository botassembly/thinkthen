# 0680: Make the Polars consumer a workspace member

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0740

## Outcome

- Changes: `libraries/polars/check.sh`, `libraries/polars/consumer/`, `libraries/rust/consumer/Cargo.toml`, `libraries/rust/consumer/src/fixture.rs`, `Cargo.toml`, `Cargo.lock`, `sdlc/scripts/feature-inventory.py`, `sdlc/scripts/feature-inventory-test.py`, `sdlc/tools/polars.json`, `conformance/parity.py`.
- The consumer is a workspace member, or its lock is generated from the root. `libraries/polars/consumer/Cargo.lock` is deleted, and `feature-inventory.py` stops expecting it.
- The shared fixture is a normal dependency, not a `#[path]` import.
- The installed check points Cargo at the extracted candidate crate through a standard source replacement. The embedded Python rewrite in `check.sh` is deleted.
- The 150 ms sleep in the consumer is replaced by an event.
- Polars's Linux tools are declared in `sdlc/tools/polars.json` and join the Linux image through 0740; a missing tool fails the check, never skips.
- Polars's parity consumer joins the 0621 scheduling table in `conformance/parity.py` with its measured peak memory and time.

### Proof

- On the M5 through `pm job run --machine m5 0680 SHA --memory 12 -- sdlc/scripts/job sh -c 'sdlc/scripts/package && THINKTHEN_ARTIFACT=$(ls target/package/thinkthen-*.crate) sh libraries/polars/check.sh'`, the installed check builds the consumer against the extracted candidate crate and runs the shared native fixture through a Polars frame.
- Linux arm64: `pm job run --machine m5 0680 SHA --memory 16 -- sdlc/scripts/linux-check surface libraries/polars`; Linux x86_64: the same command with `--machine gator --memory 7`.
- `git grep -n '#\[path' libraries/polars` and `git grep -n 'thread::sleep' libraries/polars` find nothing; both find a line on current main.

## Evidence

- **Path rewriting.** `libraries/polars/check.sh` runs an embedded Python block (`RUSTFRAME`) that copies the consumer and rewrites its `Cargo.toml` path and `#[path]` by string replace.
- **Borrowed file.** `libraries/polars/consumer/src/main.rs` imports `libraries/rust/consumer/src/fixture.rs` through `#[path]`.
- **Second lockfile.** It keeps its own 2,362-line `Cargo.lock`, synced with the root by hand (1221af108). `sdlc/scripts/feature-inventory.py` requires that lock for a feature consumer. The root `Cargo.toml` excludes `libraries` from the workspace under ADR 0047; making the consumer a member names it as an exception there.
- **Sleep.** `libraries/polars/consumer/src/main.rs` sleeps 150 ms.
- 0618 already turned the missing-crate exit 77 in `check.sh` into a failure.
