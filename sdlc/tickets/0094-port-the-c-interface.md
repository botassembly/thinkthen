---
flow: build
priority: 94
opens: libraries/c probes/c-churn sdlc/planning/adr/0037-the-c-door-serves-every-language-that-can-call-c.md sdlc/planning/libraries/c.md sdlc/scripts
---

# 0094: Port the C interface

Status: revised after re-review; confirming. Owner: Claude.

## Outcome and authority

Port the C door from the `surfaces` branch (tag `surfaces-wave7-final`, `f6a7faea`) onto the public API as the crate `thinkthen-c` at `libraries/c`, in its own Cargo workspace. ADR 0037 (Ian: "Whatever you need, let's support everything through C") fixes the door's shape, and the library team owns the C ABI under that ruling. Draft ADR 0047 fixes the crate's place, and ticket 0093 sets the workspace pattern. Queue item 9 of `sdlc/planning/one-line-plan-2026-09-24.md`.

## Header table

The tag header (`contract/include/thinkthen.h` at `f6a7faea`) declares 19 functions. Each row says what the new door keeps, changes, or drops. The builder copies this table into `libraries/c/DESIGN.md`, an ADR 0037 amendment records it, the header comments match it, and the examples' expected output follows it.

| Symbol | Verdict | Change |
|---|---|---|
| `thinkthen_engine_new` | changed | Builds through 0084's environment constructor, the one `default_engine` uses. It reads the variables that constructor reads, and the comment lists them from there. The churn probe's `THINKTHEN_BASE_URL`, `THINKTHEN_TIMEOUT_SECS`, and `THINKTHEN_MAX_RETRIES` must keep their meaning. "The connector refuses" becomes "the environment settings are invalid". No width argument and no width variable, so the door has no width refusal. |
| `thinkthen_engine_free` | kept | |
| `thinkthen_error_message` | kept | Messages come from main's `Error` display text. |
| `thinkthen_error_code` | kept | Codes 1 to 6 map from `ErrorKind` in the header's order. |
| `thinkthen_error_retryable` | changed | Returns 0084's retryable signal under 0089's rule: a retried status earns 1, and a transport failure, 401, and every non-backend kind earn 0. "Busy and slow earn 1, refused and untrusted earn 0" leaves the comment. |
| `thinkthen_cancel_token_new`, `thinkthen_cancel`, `thinkthen_cancel_token_free` | kept | The handle wraps `CancelToken` and fires from any thread. |
| `thinkthen_decide`, `thinkthen_decide_opts` | kept | `question_json` goes through `Question::from_json`. `deadline_ms` stays `int64_t` and goes through `CallOptions::deadline_millis` under ADR 0041, ported by 0099 (R2-10). |
| `thinkthen_decide_many`, `thinkthen_decide_many_opts` | kept | A cancelled or expired call still returns no rows, since a row count would change a frozen argument list. |
| `thinkthen_call`, `thinkthen_call_opts` | changed | The envelope grammar below replaces "the eight verbs". |
| `thinkthen_recognize`, `thinkthen_recognize_opts` | changed | Entities take main's `{name, kind, start, end, strength}` through `Recognized::to_json`. "A text the recordings do not hold is refused" leaves, since it described stand-in replay. Recognize follows the engine's cache and replay settings like every call. |
| `thinkthen_relate`, `thinkthen_relate_opts` | changed | `spec_json` is main's relate section, including `fields`. Each of `texts` is one JSON record, and the door resolves its name and kind with the `fields` pointers (default `/name` and `/kind`), as the command does for JSONL. `kind_field` becomes `fields.kind`, and bare-name records leave. Edges take main's shape through `Edge::to_json`: `{"relation", "source":{name,kind}, "target":{name,kind}, "probability"}`. The 255 cap stays. |
| `thinkthen_free_string` | kept | |

No symbol is dropped, and the header gains no symbol. The version macros equal the `thinkthen-c` crate version, and the symbol check compares them. DESIGN section 2's reference to an unused conversion function is fixed, and the "deferred until the engine exposes them" line goes. A later ADR 0037 amendment may add a checked width constructor if a host needs one.

## Port

- The header moves to `libraries/c/include/thinkthen.h`. The cancel handle wraps `CancelToken`, and a C host fires it from any thread. The header gains no interrupt symbol.
- `thinkthen_call` owns its envelope grammar. A request names one verb of the ten: `decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `recognize`, or `relate`. Beside it the door reads exactly seven keys: `evidence`, `records`, `units`, `details`, `usage`, `rank`, and `annotate`. Any other key is `usage`. Question and spec text go through `Question`, `QuestionSet`, `Recognize`, and `Relate` `from_json`. Details, annotate, recognize, and relate results go through the 0095 JSON methods. The door writes the remaining bare values (`filter`, `rank`, `find`, scalar `choose`, `score`, `tag`, and `usage`) with a small serializer checked byte for byte against the command's output on the shared cases. The envelope and the serializer stay under 350 nonblank lines together, inside the 1,100.
- Keep `guard()` (an engine panic becomes the defect kind), the per-thread per-engine error slot, and `thinkthen_error_code` numbering from the header, driven by `ErrorKind`.
- Library name `thinkthen_c`, crate types `cdylib` and `staticlib`, soname `libthinkthen.so.0` (ADR 0047 item 6). A check compares exported symbols with header declarations.
- Split the branch's 1,473-line `src/lib.rs` into files under 500 lines each.

## Acceptance

- Red first: a clean build shows no output-filename collision warning, and `readelf -d` shows the soname (R2-26).
- The drawn slide and C examples compile unchanged where the table keeps a symbol. The `_opts` equivalence, null matrix, two-thread error, and fork tests pass. ASan and LSan are clean. C rows R1-9, R2-7, R2-16, R2-26, and R3-24 re-run against the real engine, each with a planted bug shown turning its test red.
- The shared cases pass through the door against the 0092 loopback backend.
- R7-1 and G3: the tag's churn probe is committed verbatim at `probes/c-churn/churn.c`. It runs with NT=32, ITERS=20000, 70 engines, and a refused port, in 8 parallel runs under heavy load (the crash appeared at load average 300 to 360). On the tag build it runs up to 300 times and must crash at least once. Then it runs 300 times through the new door with zero crashes. The record keeps both counts. This ticket closes R7-1. A crash on the new door stops the ticket with an issue carrying the cores.
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

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-engine.md`) found that the header cannot keep its meaning, that the JSON door owns some grammar, a library-name collision, a weak churn count, and stale deadline text. All applied. The re-review (`sdlc/records/2026-09-24-rereview-engine.md`) asked for the whole header table, relate's record input, the seven door keys, and the committed churn probe; all applied. Confirmation pending.
- Code review: pending.
