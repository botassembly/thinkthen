# 0094: Build the C interface (progress note)

Status 2026-09-24: paused, not done. Ian asked for the work to wrap up, so the builder stopped at a clean point and pushed the work in progress to `ticket/0094-c-interface`. Nothing here is reviewed or landed.

## Done

- The branch merged `ticket/0093-first-binding-crate` and `ticket/0098-binding-members`, the latter last at `9c0fc2cb`, which brought Rust 1.95.0. The crate names `rust-version = "1.95.0"` to match the root. 0094 needs no 0098 member beyond what 0098 builds, and it changes none.
- `libraries/c` holds the crate `thinkthen-c`, in its own workspace under ADR 0047. Its library is named `thinkthen_c`, its crate types are `cdylib` and `staticlib`, and `build.rs` sets the soname `libthinkthen.so.0`.
- `include/thinkthen.h` follows the ticket's header table. It keeps all 19 functions.
- `src/` splits the door into `ffi.rs` (the only file that allows `unsafe`), `call.rs` (the envelope and the serializer), `door.rs` (the typed doors), and `failures.rs` (the per-thread, per-engine failure table, its thread-exit cleanup, and the panic guard).
- `examples/slide.c` keeps the drawn lines. `examples/functions.c` makes one call per function in the new grammar.
- `tests/c/` holds the C rows: `nulls.c`, `opts.c`, `threads.c` (R1-9), `engines.c` (R2-16), `atexit.c` (R2-7), `fork.c`, and `driver.c`. `tests/door/main.rs` runs them under AddressSanitizer and checks the soname and the exported symbols (R2-26). `tests/door/cases.rs` runs the shared cases through the driver. `tests/door/bytes.rs` holds the door's bare values to the command's bytes.
- `DESIGN.md`, `README.md`, and `check.sh` are written. The ADR 0037 amendment is written. `sdlc/planning/libraries/c.md` points at DESIGN.md. `sdlc/surfaces.txt` names `libraries/c` landed. An issue records the site's stale C samples.
- Measured sizes: production is 1,071 nonblank Rust lines in `src/` plus 11 in `build.rs`, 1,082 in all, under the 1,100 budget. That count includes the unit tests in `failures.rs`. The envelope and serializer file `call.rs` is 222 lines, under 350. `ratchet.json` holds 1,789 and `ratchet.c.json` holds 636, the measured totals.

## Not done

- The crate has not compiled since its last edits. The first Clippy run failed in `call.rs`. The builder fixed those errors and wrote `cases.rs` and `bytes.rs`, but the machine's load stayed near 30, so no later build ran. Expect compile and Clippy fixes.
- `examples/functions.txt` is not pinned. `tests/c/opts.c` expects a score of `0.1`, which no run has checked.
- `libraries/c/Cargo.lock` was copied from the root lock and is not yet checked against it.
- No planted bug has been shown for R1-9, R2-7, R2-16, R2-26, or R3-24.
- Churn (R7-1, G3): not run. `probes/c-churn/churn.c` is committed. The runner planned for it runs 8 at a time under the heavy lock. The tag's library build was queued and then stopped at the wrap-up request, so no count exists on either side. Ian ruled on 2026-09-24 that the churn probe is a one-time measurement. 0094 runs the C-door churn once to close R7-1, only when the load is low and under the heavy lock. It never runs in the ladder, `check.sh`, or a review.
- The ladder has not run. Until the crate builds and its ratchets hold, `lint`'s registry check and the surface rung fail on this branch.
- No code review, and no merge of `origin/main`.
