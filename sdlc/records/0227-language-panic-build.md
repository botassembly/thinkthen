# 0227 language panic diagnostics build

Status: four-binding source candidate in progress. High design ACCEPT at `582a2941`; no code review or issue closure yet. TypeScript and R transfers were recorded on main `83770a93`. This record separates selected source children from later installed-package evidence.

## First coherent source slice

Python and Ruby install one delegating hook each, use thread-local RAII depth on the actual caught thread, and forget every opaque `Err(payload)` before leaving that depth. Ruby's worker preparation now runs inside the same catch. Python temporarily suspends the marker for user input conversion, path extraction, caller signal dispatch, Arrow ingress and release. `frame.rs`'s caller and worker handoff remains unchanged. No public fault switch, ABI, engine policy, or host lifetime changed.

The Python selected child checks fixed `DefectError`, string and panic-on-Drop secrecy, later success, unrelated host-thread delivery, and distinct prior-hook markers from user iteration, the real waiting caller's `py.check_signals()`, Arrow ingress, and one gated worker release. The synthetic C callbacks catch their own Rust panic internally. Ruby's selected child checks fixed non-retryable Defect, both owned marker forms, later success and unrelated host-thread delivery. Python and Ruby strict all-feature/all-target Clippy passed after one Ruby test type annotation was simplified. Both selected children passed. These are source children, not installed wheel/gem proofs.

Measured Rust growth is Python 4921→5164 (+243) and Ruby 797→905 (+108) nonblank lines. Python adds one private diagnostic module and one shared selected child for all callback rows; Ruby adds one private diagnostic module and one selected child. The native C pattern is repeated only across separate binding crates; no dependency was added. I checked for repeated per-callback tests and kept one Python child instead. The `arrow/ffi.rs` addition is two net lines; its pre-existing size already exceeded the root crate's 500-line policy, which applies to `crates` and `conformance`, not binding sources. Root Rust source counter is unchanged.

## TypeScript and R source slice

TypeScript's existing `caught` converts every opaque panic into its fixed Node defect envelope while forgetting the payload under the private marker. `guarded` keeps the full envelope conversion in scope. Node `ThreadsafeFunction` completion still happens after the door. R's worker catch stops copying `words(&*panic)` into its returned error, forgets the payload in scope, and returns exact fixed `the call panicked`. Its main-thread `Pending` interrupt check stays outside the marker. R's existing interrupt, closed-channel, and later-success regression now expects that fixed error.

The selected TypeScript and R child each observed fixed non-retryable defect, no owned string/Drop markers on stdout or stderr, later success, and prior-hook delivery for an unrelated host thread. Strict all-feature/all-target Clippy and the selected children passed for both bindings; R's retained worker-edge regression also passed. TypeScript measured 561→665 (+104) and R Rust measured 1120→1218 (+98) nonblank lines. `door.rs` ends at 448 and `calls.rs` at 461, below the 500-line root convention. One private diagnostic child per crate keeps the catch and unrelated-host proof together. I checked for a duplicate panic conversion table and retained each binding's existing conversion point.

## Remaining work

Merge 0212's frozen Call conversions when it lands, review all four binding implementations together, and check every derived ratchet. Then run selected installed-host package/load proofs using each pinned wrapper where the host exists; list unavailable target packages explicitly. Do not treat the old 0226 artifact-loader observation as changed-binary proof.
