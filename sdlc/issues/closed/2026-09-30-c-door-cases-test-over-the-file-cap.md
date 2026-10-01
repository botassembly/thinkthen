# The C door's cases test is over the 500-line file cap

Status: closed by a quick fix. Found in the ticket 0346 code review.

Kind: debt

Pay when: the next ticket that edits `libraries/c/tests/door/cases.rs`, or before 0.1.

Debt: 027

Severity: low

`libraries/c/tests/door/cases.rs` holds 768 nonblank lines, over the 500-line cap in `CLAUDE.md`. `policy.py` checks the cap only under `crates/` and `conformance/`, so no gate caught it. Keeping it lets the file grow past the size a reviewer can read at once. Split it by case family, and extend the cap check to the bindings' Rust files.

Resolution: quick fix. `policy.py` now holds every tracked Rust file under `libraries/` and `databases/` to the 500-line cap. The cap found three files. `libraries/c/tests/door/cases.rs` split into the runner (`cases.rs`), how each case asks and what its replies hold (`cases/plan.rs`), and the reply, member and digest helpers (`cases/wire.rs`). Python's `src/worker.rs` and `src/arrow/ffi.rs` moved their unit tests into `worker/tests.rs` and `arrow/ffi/tests.rs`. The C ceiling rose by 21 lines for the two module headers and their imports; the Python ceiling fell by 3.
