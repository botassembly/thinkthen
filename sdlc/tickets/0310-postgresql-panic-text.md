# 0310: PostgreSQL backend panics reach the client as fixed text

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-3. Branch `ticket/0310-postgresql-panic-text`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Issue: `2026-09-27-panic-diagnostics-can-copy-payloads-across-host-boundaries.md`. Follows 0306's deferred gap.

## Outcome

A Rust panic on the PostgreSQL backend thread reaches the client as one fixed defect sentence. Its payload reaches neither the client nor the server log. pgrx's own errors, a PostgreSQL error, a cancel, and every `ereport!(ERROR)` the extension raises on purpose pass through unchanged.

## Evidence

- Starts from: main `4bfcc9445` with 0306 landed. The worker thread runs under `thinkthen::contained`. The backend thread does not. `databases/postgresql/check.sh` `a_panic_is_an_error` asserts that `the panic probe fired` reaches the client.
- Keeps: SQLSTATE `XX000` for a panic, the session living on to the next statement, every refusal's text and SQLSTATE, cancel and statement timeout, and pgrx's own panic hook.
- Changes: one guard, `call::guarded`, wraps every `#[pg_extern]` body. It passes pgrx payloads through and turns any other payload into the fixed defect `thinkthen defect: the extension panicked (retryable: no)`. `check.sh` pins that sentence and the marker's absence.
- Proof: `a_panic_is_an_error` in `check.sh` fails on main with the new assertions and passes after. `check.sh` also counts one guard per `#[pg_extern]` and one `catch_unwind`. The whole PostgreSQL check passes.
- Defers: macOS and ARM64 installed-package proofs, as the issue says. The two GUC check hooks in `ffi.rs` stay unguarded; each only compares a value and raises on purpose. pgrx's generated argument and return conversion runs outside the guard, so its error text is pgrx's own.

## Design

pgrx 0.17.0 (`pgrx-pg-sys/src/submodules/panic.rs`) raises every SQL error by panicking. `ereport!(ERROR)` and `error!` panic with `ErrorReportWithLevel`. A bare `ErrorReport` panic counts the same. A PostgreSQL `longjmp` comes back as a `CaughtError`. The `#[pg_extern]` wrapper, `pgrx_extern_c_guard`, catches the unwind. It reports those three types as they are. It reports a `&str` or `String` payload as an `XX000` error whose message is the payload. On the backend thread pgrx's panic hook already swallows the message, so the client error and the server log line are the only routes.

`call::guarded` catches the unwind inside the pgrx wrapper. A payload of one of pgrx's three types resumes unwinding untouched, except a `CaughtError::RustPanic`, which carries a panic's payload. Any other payload is forgotten without running its destructor, as `thinkthen::contained` does, and the guard raises the fixed defect through `ereport!`.

`thinkthen::contained` does not fit here. It forgets every payload, so pgrx's own errors would become the fixed defect too. The guard reuses its disposal rule instead.

## Retained behavior

The six error kinds and their SQLSTATEs, every refusal message, row-level failures, cancel and deadline, next-statement recovery, and the worker guard from 0306.

## Build result

Candidate `e9ce4d55b`, built on Linux x86-64 against PostgreSQL 16 with `CARGO_BUILD_JOBS=4`, offline.

| Check | Result |
| --- | --- |
| `a_panic_is_an_error` on main with the new assertions | failed: the client read `ERROR:  XX000: the panic probe fired` |
| `a_panic_is_an_error` after | pass: `XX000`, the fixed sentence in the client error and the server log, no marker in either, next statement answers |
| `one_catch_unwind`, `every_function_is_guarded` | pass; 33 SQL functions, 33 guards |
| Full `databases/postgresql/check.sh` | 82 passed, 1 failed: `a_deliberate_grant_survives` |
| `a_deliberate_grant_survives` after its fix | pass |
| `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --lib` | pass |
| `policy.py`, `sdlc/scripts/tickets`, both ratchets | pass |

`a_deliberate_grant_survives` named `thinkthen_decide(text, text)`. That signature went away when the named settings landed, so the step failed on main with `function "thinkthen_decide(text, text)" does not exist`. The step now names the full signature, as the other grant steps do.

The full run's other 82 steps cover every refusal text and SQLSTATE, cancel, statement timeout, and deadline. They pass unchanged, so pgrx's own payloads and PostgreSQL errors still reach the client as before.

Changes: `call::guarded` in `databases/postgresql/src/call.rs`; one guard call in each `#[pg_extern]` body across `lib.rs`, `scalar.rs`, `keyed.rs`, `find.rs`, `relate.rs` and `removed.rs`. `try_details` lost one closure level so Clippy's nesting limit holds. The PostgreSQL ratchet rises from 2914 to 2991: 24 lines of guard, 53 lines of guard calls.

Code review accepted with two fixes. A rethrown `CaughtError::RustPanic` no longer passes through, and the Defers line names pgrx's own conversion errors. After those fixes and a merge of main, the full PostgreSQL check ran 83 passed, 0 failed, with `CARGO_BUILD_JOBS=3`. `policy.py`, `sdlc/scripts/tickets` and both ratchets pass.

No test was added or deleted in Rust. The end-to-end probe is the proof, and a unit test of the guard would repeat it.

## What the build taught us

- pgrx's panic hook already swallows the message on the backend thread. The only leak was the payload text pgrx copies into the SQL error. The fix belongs at the error, not the hook.
- `thinkthen::contained` cannot serve a host that raises its own errors by panicking. A host-aware guard must sort payloads by type before forgetting them. It still reuses the shared disposal rule.
- pgrx generates each function's wrapper, so no single entry point exists. A count check in `check.sh` keeps a new SQL function from skipping the guard.
- The full PostgreSQL check had rotted on main in one step. A full run before landing catches that; a focused `STEPS` run does not.
