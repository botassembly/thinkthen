# 0227 language panic diagnostics build

Status: Python/Ruby source checkpoint in progress. High design ACCEPT at `582a2941`; no code review or issue closure yet. TypeScript and R remain pending exact 0212 file transfer. This record separates selected source children from later installed-package evidence.

## First coherent source slice

Python and Ruby install one delegating hook each, use thread-local RAII depth on the actual caught thread, and forget every opaque `Err(payload)` before leaving that depth. Ruby's worker preparation now runs inside the same catch. Python temporarily suspends the marker for user input conversion, path extraction, caller signal dispatch, Arrow ingress and release. `frame.rs`'s caller and worker handoff remains unchanged. No public fault switch, ABI, engine policy, or host lifetime changed.

The Python selected child checks fixed `DefectError`, string and panic-on-Drop secrecy, later success, unrelated host-thread delivery, and distinct prior-hook markers from user iteration, `py.check_signals()`, Arrow ingress, and one gated worker release. The synthetic C callbacks catch their own Rust panic internally. Ruby's selected child checks fixed non-retryable Defect, both owned marker forms, later success and unrelated host-thread delivery. Python and Ruby strict all-feature/all-target Clippy passed after one Ruby test type annotation was simplified. Both selected children passed. These are source children, not installed wheel/gem proofs.

Measured Rust growth at this checkpoint is Python 4921→5160 (+239) and Ruby 797→905 (+108) nonblank lines. Python adds one private diagnostic module and one shared selected child for all callback rows; Ruby adds one private diagnostic module and one selected child. The native C pattern is repeated only across separate binding crates; no dependency was added. I checked for repeated per-callback tests and kept one Python child instead. The `arrow/ffi.rs` addition is two net lines; its pre-existing size already exceeded the root crate's 500-line policy, which applies to `crates` and `conformance`, not binding sources. Root Rust source counter is unchanged.

## Remaining work

Finish TypeScript and R after 0212 transfers their accepted source, review all four binding implementations together, and check exact derived counters. Then run selected installed-host package/load proofs using each pinned wrapper where the host exists; list unavailable target packages explicitly. Do not treat the old 0226 artifact-loader observation as changed-binary proof.
