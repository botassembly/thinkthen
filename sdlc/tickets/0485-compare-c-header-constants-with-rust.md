# 0485 — compare-c-header-constants-with-rust

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

## Outcome

The existing C export check fails when real header constants disagree with production Rust values.

## Evidence

- Starts from: architecture/test-sediment PM message of 2026-10-08, ask 2; check-c-exports.py compares a changed temporary header with the real header but reads no independent Rust discriminant table.
- Keeps: Existing header layout/prototype checks and planted comparator case. No public ABI or semantic enum change.
- Changes: Emit the C-visible values from actual production Rust definitions through the existing check/test route. Compare that table with compiler-read header values in check-c-exports.py. Do not create a second hand-written expected-value table or a public introspection API.
- Proof: A planted Rust value change with a fixed header fails; a real-header value change with fixed Rust fails. Normal table and existing comparator/layout cases pass. Use one small check addition, no receipt framework.
- Defers: Whole-header/binding generation and wider architecture work to the PM's later backlog.
