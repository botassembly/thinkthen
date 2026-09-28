# Binding diagnostic policy Quick Fix

Status: Candidate for fresh code review on `ticket/qf-binding-diagnostic-policy`. This changes test dispatch and the location of one CPython FFI operation; it changes no runtime contract, policy rule, subprocess isolation or public test hook.

## Cause and correction

On main `3b96e720`, `python3 sdlc/scripts/policy.py` exited 1 with six findings: the Python diagnostic child test held `unsafe` outside a binding FFI file, four binding diagnostic tests returned before their first assertion, and `crates/thinkthen/src/core/batch/groups.rs` used a prohibited glob import. The core finding belongs to 0216 and is outside this claim.

The Python test now asks the existing `probe/ffi.rs` module to set CPython's pending interrupt through one private `#[cfg(test)]` helper. Its safety explanation and the actual `PyErr_SetInterrupt` operation moved together. Python, R, Ruby and TypeScript diagnostic tests now use an `if child { child() } else { inspect child process }` dispatch. Each still runs the same exact isolated child, checks the same parent output and keeps every child secrecy, defect and host-callback assertion. No assertion or test was deleted or consolidated, and no dummy assertion was introduced.

The Python Rust source counter rises from 5,302 to exactly 5,306 nonblank lines for the four net lines needed to place the FFI operation at its real boundary. Existing probe FFI code was reused; no extra module, copied checker or runtime branch was added. R (1,221), Ruby (907) and TypeScript (667) source totals are unchanged. A fresh independent code review must judge the raised Python ceiling.

## Focused proof

All four existing isolated Rust diagnostic tests passed, one test selected per binding:

- Python: `arrow::probe::diagnostics_tests::a_caught_python_panic_delegates_each_host_callback`, with `--no-default-features --features probe` and linked host Python.
- R: `calls::diagnostics::tests::a_caught_panic_stays_out_of_r_diagnostics`.
- Ruby: `diagnostics::tests::a_caught_panic_stays_out_of_ruby_diagnostics`, using the pinned Ruby 3.4.11 prefix and its libclang.
- TypeScript: `door::diagnostics::tests::a_caught_panic_stays_out_of_node_diagnostics`.

Strict `cargo clippy --locked --offline --all-targets -- -D warnings` passed for all four binding crates (Python also used `--no-default-features --features probe`). `rustfmt --check --edition 2024` passed for all five changed Rust files, `git diff --check` passed, and all four affected source counters equal their measured totals. After the fix, `python3 sdlc/scripts/policy.py` exits 1 **only** on the separately owned `core/batch/groups.rs` glob import. There was no full surface, paid, stress or provider run.
