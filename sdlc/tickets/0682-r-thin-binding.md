# 0682: Thin the R binding and remove its interrupt hack

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689

## Outcome

- Changes: `libraries/r/thinkthen/src/rust/`, `libraries/r/thinkthen/R/thinkthen.R`, `libraries/r/thinkthen/R/requests.R`, `libraries/r/thinkthen/R/feeds.R`, `libraries/r/thinkthen/R/complete.R`, `libraries/r/thinkthen/R/extendr-wrappers.R`, `libraries/r/thinkthen/NAMESPACE`, `libraries/r/tests/interrupt.R`, `libraries/r/tests/ordinary_interrupt.R`, `build-tools/results/templates/r.py`, `build-tools/results/registry.py`, `build-tools/results/answer_rules.py`.
- R calls `Engine::call` (the Rust run function from 0629) and builds R values by mapping the 0689 value tree, which carries the final answer, probability, pick and resolution.
- The string state machine in `src/rust/src/calls.rs` is deleted, along with the hand-written result shaping in `native_results.rs` and its private closed, reader and producer error text. The generated `results_generated.rs` keeps only declarations, or is deleted where the tree makes it unused; the R template and registry entries change to match.
- The call runs on a worker thread. The R thread checks R's own interrupt polling while it waits and cancels through the run function's handle. `.tt_interrupt` in `thinkthen.R`, which sends the process SIGINT and sleeps, is deleted.
- The "deadline was renamed deadline_ms" message in `thinkthen.R` is deleted.
- `build-tools/results/templates/r.py` leaves `LEGACY_FILES` in `build-tools/results/answer_rules.py`.
- The public R functions keep their names and arguments. The R packet dumper is 0713.

### Proof

- On the M5: `pm job run --machine m5 0682 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && THINKTHEN_TEST_PROFILE=full sh libraries/r/check.sh'`. The installed package runs the shared cases through `tests/native_case.R`, every named call in `tests/verbs.R`, and the interrupt tests `tests/ordinary_interrupt.R` and `tests/interrupt.R`: an interrupt while the provider is held returns control promptly and the receipt settles `cancelled`.
- `git grep -n 'pskill' libraries/r/thinkthen` finds nothing; on main it finds `.tt_interrupt`.
- `python3 build-tools/results/answer_rules.py` passes with the R template gone from `LEGACY_FILES`.

## Evidence

R has the largest hand-written Rust layer among the direct bridges: about 1,550 lines in `libraries/r/thinkthen/src/rust/src` (`calls.rs`, `ffi.rs`, `ffi/request.rs`, `ffi/request/inputs.rs`, `request.rs`, `native_results.rs`, `lib.rs`), plus the generated `results_generated.rs` (about 3,200 lines) from `build-tools/results/templates/r.py`.

- **Own state machine.** `Call` in `calls.rs` keeps session state as the strings "unused", "claimed" and "terminal" in `claim`, `settle_early`, `drain` and `accept`, and drives `RequestSession` itself.
- **Interrupt hack.** `.tt_interrupt` in `thinkthen/R/thinkthen.R` sends the process SIGINT with `tools::pskill` and sleeps 0.1 seconds to deliver an interrupt that `tt_interrupt_pending` already consumed. `requests.R` cancels on an `interrupt` condition.
- **Compatibility leftover.** `thinkthen.R` still answers "deadline was renamed deadline_ms, in milliseconds". The no-compatibility ruling forbids this.
- **Run function.** 0629 added `Engine::call`, which owns a named call from start to settled result and takes the stop controls.
- **Churn.** R had 253 commits since 2026-09-15, 63 of them fixes, the most of any library here.
- `crates/thinkthen` is unchanged, so `specification/rust-api-0.2.md` does not change. `conformance/library-surface.json` lists R's declarations as `NAMESPACE` and `R/*.R`, which keep their public names.
