# 0485: Keep C header values consistent with Rust definitions

Status: COMPLETE.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision 443b593f47216b3e88351beb7f5135b4245212aa, accept

## Outcome

The existing C export check fails when real header constants disagree with production Rust values.

## Evidence

- Starts from: architecture/test-sediment PM message of 2026-10-08, ask 2; check-c-exports.py compares a changed temporary header with the real header but reads no independent Rust discriminant table.
- Keeps: Existing header layout/prototype checks and planted comparator case. No public ABI or semantic enum change.
- Changes: Generated header ticket 0492 absorbs this outcome and its comparison checks. Claim `libraries/c/include/thinkthen.h`, `libraries/c/src/**` and `scripts/check-c-exports.py`. Do not run a separate hand-maintained values project. Once 0492 lands, retain its acceptance evidence in the normal 0485 record and close 0485 through pm ticket land --record-only.
- Proof: A planted Rust value change with a fixed header fails; a real-header value change with fixed Rust fails. Normal table and existing comparator/layout cases pass. Use one small check addition, no receipt framework.
- Defers: Whole-header/binding generation and wider architecture work to the PM's later backlog.
