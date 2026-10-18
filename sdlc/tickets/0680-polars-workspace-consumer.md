# 0680: Make the Polars consumer a workspace member

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0688

## Outcome

- The consumer is a workspace member, or its lock is generated from the root. The separate lockfile is deleted.
- The shared fixture is a normal dependency, not a `#[path]` import.
- The installed check points Cargo at the extracted candidate crate through a standard source replacement. The Python rewrite is deleted.
- The sleep is replaced by an event.

### Proof

On the M5 through pm job, the installed check builds the consumer against the extracted candidate crate and runs the shared native fixture through a Polars frame.

## Evidence

- **Path rewriting.** `libraries/polars/check.sh:43-56` runs embedded Python that copies the consumer and rewrites its `Cargo.toml` path and `#[path]` by string replace.
- **Borrowed file.** The consumer imports another library's file with `#[path = "../../../rust/consumer/src/fixture.rs"]` (`consumer/src/main.rs:10`).
- **Second lockfile.** It keeps its own 2,362-line `Cargo.lock`, synced with the root by hand (1221af108).
- **Silent skip.** `check.sh:16-20` exits 77 when the crate cache lacks locked crates; 0618 makes that a failure.
- **Sleep.** `consumer/src/main.rs:77` sleeps 150 ms.
