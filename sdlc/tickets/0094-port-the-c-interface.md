---
flow: build
priority: 94
opens: libraries/c sdlc/planning/adr/0037-the-c-door-serves-every-language-that-can-call-c.md sdlc/planning/libraries/c.md sdlc/scripts
---

# 0094: Port the C interface

Status: draft, revised after design review; needs re-review. Owner: Claude.

## Outcome and authority

Port the C door from the `surfaces` branch (tag `surfaces-wave7-final`, `f6a7faea`) onto the public API as the crate `thinkthen-c` at `libraries/c`, in its own Cargo workspace. ADR 0037 (Ian: "Whatever you need, let's support everything through C") fixes the door's shape, and the library team owns the C ABI under that ruling. Draft ADR 0047 fixes the crate's place, and ticket 0093 sets the workspace pattern. Queue item 9 of `sdlc/planning/one-line-plan-2026-09-24.md`.

## Header changes

Main's engine cannot keep every branch meaning. The first step writes a symbol-by-symbol table of the 19 symbols (kept, changed, dropped) into `libraries/c/DESIGN.md`. An ADR 0037 amendment records it, the header comments match it, and the examples' expected output follows it. Known changes:

- Relate edges become main's shape, `{"relation", "source":{name,kind}, "target":{name,kind}, "probability"}` (`specification/relate.md`). The relate spec takes entities with name and kind. The C caller maps its records to entities, and `kind_field` and bare names leave.
- Recognize answers carry main's `{name, kind, start, end, strength}` entities.
- `thinkthen_engine_new(void)` keeps no width argument and reads no width variable, so the door has no width refusal. If a host needs width, a later ADR 0037 amendment adds a checked constructor.
- ADR 0037's `unsigned long deadline_ms` becomes the header's `int64_t`, read through `CallOptions::deadline_millis` under ADR 0041 (ported by 0099) (R2-10).
- DESIGN section 3 keeps "no rows" for a cancelled or expired call, since changing it needs a count argument in a frozen ABI. The "deferred until the engine exposes them" line goes. DESIGN section 2's reference to an unused conversion function is fixed.

## Port

- The header moves to `libraries/c/include/thinkthen.h`. The cancel handle wraps `CancelToken`, and a C host fires it from any thread. The header gains no interrupt symbol.
- `thinkthen_call` owns its envelope grammar (`decide`, `evidence`, `records`, `annotate`, `details`, `usage`, `rank`, and the rest). Question and spec text go through `Question`, `QuestionSet`, `Recognize`, and `Relate` `from_json`. Details, annotate, recognize, and relate results go through the 0095 JSON methods. The door writes the remaining bare values (`filter`, `rank`, `find`, scalar `choose`, `score`, `tag`, and `usage`) with a small serializer checked byte for byte against the command's output on the shared cases.
- Keep `guard()` (an engine panic becomes the defect kind), the per-thread per-engine error slot, and `thinkthen_error_code` numbering from the header, driven by `ErrorKind`.
- Library name `thinkthen_c`, crate types `cdylib` and `staticlib`, soname `libthinkthen.so.0` (ADR 0047 item 6). A check compares exported symbols with header declarations.
- Split the branch's 1,473-line `src/lib.rs` into files under 500 lines each.

## Acceptance

- Red first: a clean build shows no output-filename collision warning, and `readelf -d` shows the soname (R2-26).
- The drawn slide and C examples compile unchanged where the table keeps a symbol. The `_opts` equivalence, null matrix, two-thread error, and fork tests pass. ASan and LSan are clean. C rows R1-9, R2-7, R2-16, R2-26, and R3-24 re-run against the real engine, each with a planted bug shown turning its test red.
- The shared cases pass through the door against the 0092 loopback backend.
- R7-1 and G3: the branch churn probe (`t7a/churn/R4-1-churn.c` or its port, same parameters) first reproduces the crash on the tag build, then runs at least 300 times through the new door with zero crashes. The record keeps both counts. A crash stops the ticket with an issue carrying the cores.
- Only this crate's FFI module allows `unsafe` (ADR 0047 item 3).
- Production code stays under 1,100 nonblank Rust lines once the stand-in glue goes.

## Exclusions

Release archives, installers, and checksums (queue item 11). C++, Go, or Java wrappers. Any change to `thinkthen`.

## Dependencies

After 0086, 0092, 0098, 0099, and 0093, and after ADR 0047 is reviewed. The ADR 0037 amendment lands here.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 3; state and timing 3; reach 3; proof 3; cost of error 3; total 15. Final level: 3. A frozen ABI, per-thread memory, and panics near a host process.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-engine.md`) found that the header cannot keep its meaning, that the JSON door owns some grammar, a library-name collision, a weak churn count, and stale deadline text. All applied; re-review pending.
- Code review: pending.
