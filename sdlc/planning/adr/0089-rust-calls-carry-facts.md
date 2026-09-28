# ADR 0089: Rust calls carry their own run facts

- Status: Proposed for 0212 design review. No implementation or public API change has landed. Ian can overturn the proposed shape.
- Date: 2026-09-27

## Context

ADR 0017's Rust API and ticket 0084 return bare single values, `Result<Vec<_>, Error>` for eager bulk operations, and lazy `Batch<T>` iterators. Ian later ruled in `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md` that every library result carries facts with no second call. The batching issue fixes the fact names and per-call meaning. ADR 0083 fixes the command's final stop facts and explicitly leaves the library return shape to B12a. A process `Counters` snapshot cannot isolate one Rust call while another call uses the same engine.

## Proposed decision

1. Eager Rust methods and matching root free functions return `Result<Call<T>, Error>`. `Call<T>` owns one value and one immutable `Facts`, with `value()`, `into_value()` and `facts()` accessors. This is an intentional source-breaking public signature change; the native doors must adapt when their B12 tickets land. It does not change the bare command output or SQL scalar shape.
2. Lazy methods keep `Batch<T>: Iterator<Item = Result<T, Error>>`. After exhaustion or a terminal error, `Batch::facts() -> Option<&Facts>` returns final facts; before completion it returns `None`. Reading facts sends nothing. Dropping an unfinished batch cancels and joins its work as before, but a dropped value cannot return a final report. A first-error row still appears once and then the iterator ends.
3. `Facts` exposes `records()`, `requests_sent()`, `cache_answers()`, optional `input_tokens()` and `output_tokens()`, `seconds()`, and optional `model()`. The names and presence rules come from the batching design and run-facts issue. They count all work of that call, including a filtered-away row and every attempted retry. They are collected in call-scoped state at request and completion boundaries, not by subtracting shared process counters. Provider-absent tokens remain `None`. No price, server duration or vendor request ID is guessed.
4. Keep the six public `ErrorKind` variants. A post-start `Error` may carry safe frozen facts through `Error::facts() -> Option<&Facts>`; validation and engine-build errors have none. A batch terminal error freezes its final facts after workers join. Cancellation, deadline, replay miss and Backend failure keep their existing kind and retryable meaning. Facts contain counts, model and request digests where separately reviewed, never raw evidence, context, key or backend reply text.
5. B12a must review a bounded representation for the accepted full-detail reach across all verbs before implementation claims that this ADR settles that part. Existing `Details` works only for one decide/choose/tag/score text; a second request to fetch details is forbidden. In particular, do not retain every hidden `filter` row only to populate a final object. The 0212 design review can amend this proposed shape or return it for another review; no decision is silently inferred from the summary fields alone.

## Existing decisions retained

ADRs 0048, 0053 and 0055 continue to fix batch shape, limits and shares. ADR 0056 continues to fix recognize's separate three-step planner. ADR 0083 remains the command's stop and count contract. ADR 0017's blocking engine, shared throttle, fork rebuild, six error kinds and cancellation rules remain. This proposal amends only the public Rust return shape and call-scoped facts access once reviewed and accepted.
