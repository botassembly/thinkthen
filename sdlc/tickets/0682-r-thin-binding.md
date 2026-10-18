# 0682: Thin the R binding and remove its interrupt hack

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689

## Outcome

- R calls the Rust run function from 0629 and builds R values by mapping the 0689 value tree, which carries the final answer, probability, pick and resolution.
- The string state machine is deleted, along with any hand-written result shaping and its private closed, reader and producer error text.
- The call runs on a worker thread. The R thread checks R's own interrupt polling while it waits and cancels through the run function's handle. The self-SIGINT and sleep are deleted.
- The renamed-argument message is deleted.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed package runs all ten named calls against a recorded backend, and its packet dump matches the 0642 oracle. An interrupt while the provider is held returns control promptly, with no signal sent to the process.

## Evidence

R has the largest hand-written Rust layer in this group, about 1,500 lines in `libraries/r/thinkthen/src/rust` (`calls.rs`, `ffi.rs`, `ffi/request.rs`, `request.rs`, `native_results.rs`, `lib.rs`, `inputs.rs`).

- **Own state machine.** `calls.rs:44-57` keeps session state as strings: "unused", "claimed", "terminal".
- **Interrupt hack.** To deliver an interrupt, the package sends itself SIGINT and sleeps (`thinkthen/R/thinkthen.R:67-74`).
- **Compatibility leftover.** `thinkthen.R:56` still answers "deadline was renamed deadline_ms". The no-compatibility ruling forbids this.
- **Churn.** R had 253 commits since 2026-09-15, 63 of them fixes, the most of any library here.
