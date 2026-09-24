# ADR 0037: The C door serves every language that can call C

- Status: Decided by the library team on 2026-09-22 under Ian's ruling in session. The build team still reviews the door at the merge. Ian can overturn any line.
- Date: 2026-09-22

## Decision

The C surface is the common door. C, C++, Go, and Java consume thinkthen through `contract/include/thinkthen.h` and the C ABI it declares; no separate binding is written for those languages, and the door's shape is chosen for all of them at once. Flat arguments only, so no struct crosses the boundary, and every result of open size crosses as JSON text. That is what lets C++, Go, and Java consume the door without per-language glue.

This supersedes the ownership sentence in `sdlc/planning/library-team-architecture-punch-list.md` ("The architect owns the production Rust API, shared semantics, and integration sequence") for the C ABI design alone: the library team owns the C ABI design. The architect keeps the production Rust API, the shared semantics, and the integration sequence.

Provenance: Ian's ruling in the library-team session on 2026-09-22. His words: "I want to support C, and I was told that C would support C++, it would support Go, and it would support Java. Whatever you need, let's support everything through C." On ownership he assigned the C design to the library team rather than the architect. The design page `libraries/c/DESIGN.md` on branch `surfaces` states the same ruling and answers the punch list's line: "C needs an accepted options/ownership design covering cancellation, deadlines, partial completion, null/length inputs, allocated results, and concurrent callers before its ABI freezes."

## The decided shape

- **The drawn signatures freeze.** The slide's `thinkthen_decide(tt, q, text, text_len, &a)` and its `_many` twin keep compiling as drawn, because the deck is the surface's acceptance sample.
- **Control spellings sit beside them.** `thinkthen_decide_opts`, `thinkthen_decide_many_opts`, `thinkthen_call_opts`, `thinkthen_recognize_opts`, and `thinkthen_relate_opts` carry `unsigned long deadline_ms` and `thinkthen_cancel_token *token` as flat arguments.
- **A plain call is its `_opts` twin** with `THINKTHEN_NO_DEADLINE` (-1) and a null token; 0 stays the spent deadline; the equivalence is proven by test, including the error path.
- **Cancellation is a one-shot token handle** the host fires from any thread; a fired token starts no new request, and requests already sent finish.
- **One memory rule:** the library allocates every returned buffer and takes it back through its free call. The error message belongs to the calling thread and lives until that thread records its next failure.
- **The error code** `thinkthen_error_code` returns the numeric kind, and a panic behind the door comes back as the defect kind instead of aborting the host.

## Proof

Landed on branch `surfaces`: `8c83765` ("Give the C door its cancel token, per-call budget, and error code") and `be8374f` ("Give the C door per-thread error slots, a panic guard, and the checked deadline"). The C gate exits 0: the drawn slide and examples compile unchanged, the `_opts` equivalence and null-matrix tests are green, ASan and LSan are clean including the two-thread error test, and the conformance slice reads 68 green lines. The design page's deferred items are named there with their reasons.

## Amendment, 2026-09-24, by ticket 0094

Ticket 0094 ported the door onto the public API as the crate `thinkthen-c` at `libraries/c`, in its own workspace under ADR 0047. The header moved to `libraries/c/include/thinkthen.h`. The decision above names its old path. The port kept all 19 functions and added none. `libraries/c/DESIGN.md` holds the whole header table. The changes:

- `thinkthen_engine_new` builds through `Engine::from_env`. NULL means the environment settings are invalid. No width argument exists.
- `thinkthen_error_retryable` follows ticket 0089's rule. A retried status earns 1. A transport failure, a 401, and every kind but backend earn 0.
- `deadline_ms` is `int64_t`, as the header already declared, and goes through `CallOptions::deadline_millis` (ADR 0041). The "unsigned long" above is superseded.
- `thinkthen_call` owns a grammar of ten verbs and five envelope keys. Every other key forms the question object. `rank` is the only rank spelling.
- `thinkthen_recognize` takes a version-one spec. Its entities carry `name`, `kind`, `start`, `end`, and `strength`.
- `thinkthen_relate` takes a version-one relate spec and one JSON record per text, at the default `name` and `kind` fields. Its edges take the engine's shape. The 255 cap stays.
- A thread's failure entries leave every engine's table when the thread exits (R3-24).
- The shared library carries the soname `libthinkthen.so.0`, and the crate's library name `thinkthen_c` keeps its files from colliding with the engine's (R2-26).

Ian can overturn any line. The verb spellings and the retryable rule are the ones a host would notice.
