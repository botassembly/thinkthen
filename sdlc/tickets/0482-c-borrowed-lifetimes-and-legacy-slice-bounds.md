# 0482 — c-borrowed-lifetimes-and-legacy-slice-bounds

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision da9ce6e04d32f7f16341b033304919529662002e, accept

## Outcome

The C header states borrowed failure-pointer lifetimes. Legacy counted inputs reject impossible byte extents before unsafe slice construction.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, asks 4 and 6; thinkthen.h omits engine-free invalidation; ffi.rs legacy text/texts call from_raw_parts without the extent guard used by current/read/ffi.rs and typed_facts/ffi.rs.
- Keeps: Null/zero inputs, normal legacy calls, owned result lifetimes, thread-local failure slots, C ABI/layout and all request counts. Invalid caller pointers remain outside the safety contract; this change catches representationally impossible extents without dereferencing them.
- Changes: Document next-failure, engine-free and thread-exit invalidation consistently with DESIGN.md and Rust failure comments, including the distinct null-engine slot. Reuse the typed-read extent pattern for byte text and both pointer/length arrays. Return the established usage error before constructing a slice or allocating results. Correct the separately confirmed formatting-only main failure in libraries/c/tests/door/current.rs while preserving its assertions.
- Proof: One-byte owned pointer with SIZE_MAX length/count through legacy decide, recognize, many and relate refuses with untouched outputs and zero sends. Normal calls, null/zero and existing typed-facts bounds still pass. Fresh memory-safety ticket/code review; applicable public C consumer checks.
- Defers: New C ABI, generated bindings and unrelated ownership redesign. Do not execute undefined behavior to reproduce the missing guard.
