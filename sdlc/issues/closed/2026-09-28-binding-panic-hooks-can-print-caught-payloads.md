# Python, Ruby and TypeScript panic hooks can print caught payloads

Status: closed 2026-09-30. Fixed by ticket 0306 (`4de0f8116`): Python, Ruby and TypeScript call `thinkthen::contained` and keep no hook. Non-Linux package proofs move to the release issue.

## Current boundaries

| Binding | Source and remaining gap |
| --- | --- |
| Python | `libraries/python/src/lib.rs::caught` calls `catch_unwind(...).ok()` and `guard` maps failure to a fixed `DefectError`. There is no local scoped hook for a panic in the wrapper itself. The existing module-edge test uses `resume_unwind`, which skips the hook and therefore cannot prove stderr secrecy. |
| Ruby | `libraries/ruby/src/lib.rs::guarded` catches the binding body and returns a fixed `Fault`. It does not suppress the earlier hook for binding-owned work. The existing marker-panic test checks the returned error, not captured stdout/stderr or unrelated host-hook delivery. |
| TypeScript | `libraries/typescript/src/door.rs::caught` catches the body and returns fixed Node defect text. Its guard test checks the error envelope and later success but not the earlier hook or unrelated host diagnostics. |

A Rust panic hook runs before `catch_unwind` returns. The landed engine scopes cover engine-owned work, not every possible panic in these wrappers. The default hook can therefore print a binding panic payload even though its returned error is fixed. These catches also dispose of an arbitrary opaque payload outside a reviewed disposal policy; a payload destructor may panic again. Trace that path in the design instead of assuming all payloads are strings.

## Outcome and focused proof

Keep the existing six error kinds, non-retryable Defect, ordinary host exceptions, cancellation, caller and worker ownership, next-call recovery and public APIs. Use the reviewed private scope and disposal lessons from ADRs 0081 and 0098 where each host permits them. Preserve unrelated host diagnostics; do not swap the global hook around every call. Record any retained-allocation cost and verify actual artifact linkage before imposing a new host-visible unload policy.

Replace or extend the closest private child boundary with one synthetic string payload and one payload whose destructor panics. Check the fixed returned error, absence of both markers in stdout/stderr, a later successful call and prior-hook delivery for an unrelated host thread. Cover separate worker entry only where the host actually uses one. No public fault switch, repeated campaign or provider call is needed. A `resume_unwind` case may still prove containment but does not exercise the hook.

## Routing

The R worker payload copy has its own issue, `2026-09-28-r-worker-panic-can-copy-payload-text.md`. Ticket 0226 covers C, SQLite and retained DuckDB C API only. This family follows its reviewed implementation lessons. Coordinate exact files with 0212's TypeScript adapter conversion and subsequent port batching tickets before building. Keep per-host source and installed-package proof distinct and leave unavailable targets open.

## Reviewed implementation and remaining proof

Ticket 0227 source and the evidenced Linux package subset landed from corrected candidate `e52c1604` after fresh High code review. The source children prove fixed non-retryable Defect, both synthetic payload forms, later success and unrelated prior-hook delivery. Installed Linux packages separately prove loading and no-fault later use. Python's reviewed correction also covers implicit reference cleanup. See [the build evidence](../../records/0227-language-panic-build.md) and [code acceptance](../../records/0227-code-review.md). Other target packages and the later 0212 Call conversion integration remain open; this issue is not counted complete from the Linux subset alone.

## Ticket 0306

Ticket 0306 removes this binding's own panic hook. Its guard now calls the shared `thinkthen::contained`, which forgets the payload inside the marked scope. The existing child test passes unchanged. The target-package proofs above remain open.
