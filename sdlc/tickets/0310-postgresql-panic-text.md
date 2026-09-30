# 0310: PostgreSQL backend panics reach the client as fixed text

Status: in build. Lane claude-3. Branch `ticket/0310-postgresql-panic-text`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Issue: `2026-09-27-panic-diagnostics-can-copy-payloads-across-host-boundaries.md`. Follows 0306's deferred gap.

## Outcome

A Rust panic on the PostgreSQL backend thread reaches the client as one fixed defect sentence. Its payload reaches neither the client nor the server log. pgrx's own errors, a PostgreSQL error, a cancel, and every `ereport!(ERROR)` the extension raises on purpose pass through unchanged.

## Evidence

- Starts from: main `4bfcc9445` with 0306 landed. The worker thread runs under `thinkthen::contained`. The backend thread does not. `databases/postgresql/check.sh` `a_panic_is_an_error` asserts that `the panic probe fired` reaches the client.
- Keeps: SQLSTATE `XX000` for a panic, the session living on to the next statement, every refusal's text and SQLSTATE, cancel and statement timeout, and pgrx's own panic hook.
- Changes: one guard, `call::guarded`, wraps every `#[pg_extern]` body. It passes pgrx payloads through and turns any other payload into the fixed defect `thinkthen defect: the extension panicked (retryable: no)`. `check.sh` pins that sentence and the marker's absence.
- Proof: `a_panic_is_an_error` in `check.sh` fails on main with the new assertions and passes after. `check.sh` also counts one guard per `#[pg_extern]`. The whole PostgreSQL check passes.
- Defers: macOS and ARM64 installed-package proofs, as the issue says. The two GUC check hooks in `ffi.rs` stay unguarded; each only compares a value and raises on purpose.

## Design

pgrx 0.17.0 (`pgrx-pg-sys/src/submodules/panic.rs`) raises every SQL error by panicking. `ereport!(ERROR)` and `error!` panic with `ErrorReportWithLevel`. A bare `ErrorReport` panic counts the same. A PostgreSQL `longjmp` comes back as a `CaughtError`. The `#[pg_extern]` wrapper, `pgrx_extern_c_guard`, catches the unwind. It reports those three types as they are. It reports a `&str` or `String` payload as an `XX000` error whose message is the payload. On the backend thread pgrx's panic hook already swallows the message, so the client error and the server log line are the only routes.

`call::guarded` catches the unwind inside the pgrx wrapper. A payload of one of pgrx's three types resumes unwinding untouched. Any other payload is forgotten without running its destructor, as `thinkthen::contained` does, and the guard raises the fixed defect through `ereport!`.

`thinkthen::contained` does not fit here. It forgets every payload, so pgrx's own errors would become the fixed defect too. The guard reuses its disposal rule instead.

## Retained behavior

The six error kinds and their SQLSTATEs, every refusal message, row-level failures, cancel and deadline, next-statement recovery, and the worker guard from 0306.
