# 0306: Panic payload secrecy through one shared guard

Status: landed. Lane claude-4. Branch `ticket/0306-panic-payload-secrecy`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Issues: `2026-09-27-panic-diagnostics-can-copy-payloads-across-host-boundaries.md`, `2026-09-28-binding-panic-hooks-can-print-caught-payloads.md`, `2026-09-28-r-worker-panic-can-copy-payload-text.md`.

## Outcome

A Rust panic never copies its payload to standard output, standard error, or a host error. One public helper in the `thinkthen` crate catches a panic, keeps it from the panic hook, and forgets the payload without running its destructor. The engine door and every binding guard call that helper. No port keeps its own panic hook.

## Evidence

- Starts from: main `a233dd30c`, tickets 0226, 0227 and 0254, and the three open panic issues.
- Keeps: each surface's fixed failure message and the host's own hook for unrelated panics.
- Changes: one shared guard, `thinkthen::contained`, replaces eight copied hooks; the PostgreSQL worker is guarded.
- Proof: marker tests per surface; the drop-panic case fails on the old door.
- Defers: the PostgreSQL backend thread under pgrx; macOS and ARM64 package proofs.

Main `a233dd30c` was checked on 2026-09-29.

Fixed in source by tickets 0226, 0227 and 0254, each with its own copy of one scoped hook:

- C: `libraries/c/src/failures.rs:183-228` (hook and depth), `:343-370` (guards).
- SQLite: `databases/sqlite/src/lib.rs:37-75`, `:179-189`.
- DuckDB bridge: `databases/duckdb/bridge/src/ffi/panic.rs:1-64`.
- Python: `libraries/python/src/diagnostics.rs:1-51`, `libraries/python/src/lib.rs:154-169`.
- Ruby: `libraries/ruby/src/diagnostics.rs:1-41`, `libraries/ruby/src/lib.rs:86-97`.
- TypeScript: `libraries/typescript/src/door/diagnostics.rs:1-41`, `libraries/typescript/src/door.rs:140-152`.
- R: `libraries/r/thinkthen/src/rust/src/calls/diagnostics.rs:1-41`, `calls/worker.rs:136-152`.
- Engine: `crates/thinkthen/src/engine/workers.rs:13-62`.

That is eight copies of the same hook. Each surface installs its own, so one process chains two hooks.

Not fixed:

- The engine door. `crates/thinkthen/src/public/options.rs:452-455` drops a caught payload outside the marked scope. A payload whose destructor panics prints its marker through the host hook and unwinds out of the door. A new case in `public/options/tests.rs` shows `prior hook: drop key evidence secret` on stderr. Every surface calls this door.
- PostgreSQL worker. `databases/postgresql/src/call.rs:252-254` runs the worker body with no guard. A panic there reaches the default hook and the server log.
- PostgreSQL backend thread. pgrx turns a panic into an `XX000` error carrying the payload. `databases/postgresql/check.sh:1083-1089` asserts that `the panic probe fired` reaches the client.

## Changes

- Add `thinkthen::contained(body) -> Option<T>` and `thinkthen::uncontained(body) -> T` in `public/panic.rs`. The first marks the thread, catches, and forgets the payload inside the mark. The second lets host callbacks run under the host's previous hook. Both reuse the engine's one hook in `engine/workers.rs`.
- The engine door `guarded` calls `contained`.
- C, SQLite, DuckDB bridge, Python, Ruby, TypeScript and R guards call `contained`. Delete their hook code. The Ruby, TypeScript and R `diagnostics.rs` files keep only their child tests. Python's host helpers call `uncontained`.
- The PostgreSQL worker body runs under `contained`. A panic there becomes the fixed defect.
- Per-binding checks that counted one `catch_unwind` site now count none. The DuckDB source check does the same.
- The engine's depth mark uses `try_with`, so a thread past its local-storage teardown still runs the guarded body.
- Fix one argument in `libraries/python/src/frame.rs` `_arrow_probe`. The `probe` feature did not compile on main, and it holds the Python panic child.

## Retained behavior

The six error kinds, the fixed defect messages, non-retryable defect, next-call recovery, cancellation, host interrupt callbacks and unrelated host panic hooks. The engine's worker scopes stay as they are.

## Proof

- Core: the existing diagnostic child gains a destructor-panic payload. It fails on main and passes after. It also proves `contained` directly.
- Each binding's existing child test (C, SQLite, Python, Ruby, TypeScript, R, DuckDB bridge) passes with the shared helper. Each asserts both markers absent from stdout and stderr, the fixed defect, a later call, and an unrelated host panic on the prior hook.
- PostgreSQL: a unit test drives `deliver`'s worker path with a marker panic, if pgrx builds offline here.
- `cargo test` for the core, `policy.py`.

## Deferred gaps

- PostgreSQL backend-thread panics. pgrx raises errors by panicking, so a generic catch there must pass pgrx's own payloads through. That needs its own design. The issue records it.
- macOS and Linux ARM64 installed-package proofs stay open, as the issues say.

## Build result

All surface tests below ran locally with `-j4`, offline. Every toolchain was present: Python 3.12 with libpython, Ruby 3.4.11 from the toolchain cache, Node 22, R, pgrx for PostgreSQL 16, and the DuckDB bridge's Rust crate.

| Surface | Test | Result |
| --- | --- | --- |
| Core door | `public::options::tests::engine_diagnostics_hide_worker_payloads_and_preserve_host_hook` | failed on main with `prior hook: drop key evidence secret`; passes |
| C | `failures::tests::a_caught_payload_never_reaches_native_diagnostics`, door suite | pass |
| SQLite | `worker::tests` native panic child, lib suite | pass |
| DuckDB bridge | `ffi::tests::caught_payloads_stay_in_bridge_scope` | pass |
| Python | `arrow::probe::diagnostics_tests::a_caught_python_panic_delegates_each_host_callback` (`--features probe`) | pass |
| Ruby | `diagnostics::a_caught_panic_stays_out_of_ruby_diagnostics` | pass |
| TypeScript | `door::diagnostics::a_caught_panic_stays_out_of_node_diagnostics` | pass |
| R | `calls::diagnostics::a_caught_panic_stays_out_of_r_diagnostics` | pass |
| PostgreSQL worker | `call::tests::a_worker_panic_stays_out_of_the_server_log` | failed with the old `deliver`; passes |

Clippy with `-D warnings` is clean for the core, C, SQLite, DuckDB bridge, Python (with and without `probe`), Ruby, R and PostgreSQL. TypeScript clippy fails on main at `src/door/result.rs:38` and on the moved child test with `result_large_err`; this ticket does not change that. `policy.py` passes. The core `--lib` suite has one failure, `engine::deadline_tests::a_held_folder_ends_as_the_deadline_without_the_owner`, which ticket 0303 already names.

Line counts: the core grows 72 and PostgreSQL 48. The seven bindings shrink by 310.

## What the build taught us

- The copies hid a bug in the original. Every binding forgot its payload, but the engine door dropped it. A payload whose destructor panicked escaped the door and printed. One shared guard fixes that for every surface at once.
- A private helper copied per port drifts. A public helper in the core crate keeps ports thin and gives one place to test.
- A surface guard needs no marked scope around its own fallback. Only the body needs marking.
- The core grows by 72 lines and PostgreSQL by 48, after review added the guard's limits and a host-hook check to the PostgreSQL test. The new public API is `thinkthen::contained` and `thinkthen::uncontained`, in `public/panic.rs`. Their docs state the limits: a host that replaces the hook later displaces it, a thread past local-storage teardown runs unmarked, and a first call from a thread already panicking aborts.
- Package gates on main have rotted: the Python `probe` feature did not compile, TypeScript clippy fails, and C sources on main are not `cargo fmt` clean. Issue `2026-09-29-nine-package-gates-fail-from-clean-checkouts.md` owns the last two.
