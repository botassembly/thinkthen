---
flow: build
priority: 94
opens: Cargo.toml libraries/c sdlc/planning/adr/0037-the-c-door-serves-every-language-that-can-call-c.md sdlc/planning/libraries/c.md sdlc/ratchet.json
---

# 0094: Port the C interface

Status: draft, design not reviewed. Owner: Claude.

## Outcome and authority

Port the C door from the `surfaces` branch (tag `surfaces-wave7-final`, `f6a7faea`) onto the public API as the crate `thinkthen-c` at `libraries/c`. ADR 0037 (Ian: "Whatever you need, let's support everything through C") fixes the door's shape. Draft ADR 0047 fixes the crate's place, and ticket 0093 sets the workspace pattern. Queue item 9 of `sdlc/planning/one-line-plan-2026-09-24.md`.

## Port

- Carry `contract/include/thinkthen.h` (19 symbols) to `libraries/c/include/thinkthen.h` unchanged in meaning, and `libraries/c/DESIGN.md` with it. Fix DESIGN section 2, which names a conversion function the code never calls.
- `thinkthen_engine_new` builds an `Engine` through `EngineBuilder`. A width outside 1–32 is the usage kind (R4-12).
- `deadline_ms` goes through `CallOptions::deadline_millis`, so C follows branch ADR 0041 exactly (R2-10). The cancel handle wraps `CancelToken`. The header gains no interrupt symbol: a C host fires the token from any thread.
- `thinkthen_call` parses with `Question::from_json`, `QuestionSet::from_json`, `Recognize::from_json`, and `Relate::from_json`, and answers with the 0095 `to_json` methods. It holds no parser or serializer of its own.
- Keep `guard()` (a panic becomes the defect kind), the per-thread per-engine error slot, and `thinkthen_error_code` numbering from the header, driven by `ErrorKind`.
- A check compares exported symbols with header declarations, so neither drifts alone.

## Acceptance

- The drawn slide and C examples compile unchanged. The `_opts` equivalence, null matrix, two-thread error, and fork tests pass. ASan and LSan are clean.
- The shared cases pass through the door against the 0092 loopback backend. C rows R1-9, R2-7, R2-16, R2-26, and R3-24 re-run against the real engine.
- The branch's churn probe runs at least 104 times through the door with zero crashes (R7-1, G3). A crash stops the ticket with an issue carrying the cores.
- `thinkthen` keeps `forbid(unsafe_code)`. Only this crate relaxes it, at its FFI edge.
- Production code stays under 1,100 nonblank Rust lines. The branch door is the ceiling, less the stand-in glue it no longer needs.

## Exclusions

Release archives, installers, and checksums (queue item 11). C++, Go, or Java wrappers. Any change to `thinkthen`.

## Dependencies

After 0093 and review of ADR 0047. The ADR 0037 amendment that moves the header path lands here.

## Complexity

Contract 3; state and timing 3; reach 3; proof 3; cost of error 3; total 15. Final level: 3. A frozen ABI, per-thread memory, and panics near a host process.

## Review

- Design review: pending.
- Code review: pending.
