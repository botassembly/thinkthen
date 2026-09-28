# ADR 0091: Python call results and facts after an early stop

- Status: Proposed for independent 0214 design review. Ian can overturn the Python-specific stop choice.
- Date: 2026-09-27

## Context

Ian requires facts on every library call without another model call. Accepted ADR 0089 proposes `Call<T>` and final facts after a Rust call joins. Python currently runs each call on a detachable worker. Its caller checks signals and cancellation every 50 ms and raises promptly while a held send is still running (`libraries/python/src/worker.rs::wait`; `tests/test_stopping.py`). Waiting for that held request to finish before raising would break the existing under-100-ms stop boundary. A process `usage()` delta cannot replace per-call facts, and an immediate snapshot cannot include replies that arrive during the worker's later join.

## Proposed choice

Every successful Python verb, including `details`, returns `Call[T]` with read-only `value`, `facts`, and `details` fields. `value` retains the current scalar, list, structured, Series or frame value and its null/failure semantics. `facts` is one immutable per-call snapshot using ADR 0089's seven fields. `details` is an ordered collection of completed logical question observations, including filtered-out records and named or staged questions; it is copied from ADR 0089's borrowed observer during this eager Python call. It does not invent a whole-row judgment for `annotate`, `recognize`, `find` or `relate`. The existing separate `details()` and `usage()` functions remain, but neither is needed to learn one new call's facts. This is an intentional Python return-shape break under Ian's already accepted ruling. `Call` has no implicit truth value or list delegation: `if tt.decide(...).value` states whether the answer is true, and a not-sure answer remains `None` inside `value`.

An error after the Rust call finishes carries the same frozen `facts` and completed `details` on its existing Python exception class; an argument or setup refusal before a call starts has neither. `Cancelled` raised by the **outer Python wait before the worker finishes** instead carries a `completion` receipt. The receipt belongs to that call, has no value or provider operation, and can be awaited through `completion.result()` for the final immutable `facts` and completed `details` after the worker exits. Its `done` state is observable without waiting. No immediate final facts are asserted while a request is still held. The exception retains the existing kind, message, `KeyboardInterrupt` inheritance and fast-return behavior. The worker owns its input and finishes cancellation and cleanup even if the receipt is dropped. A signal handler's unrelated `SystemExit` remains unchanged. This is the one proposed Python-specific exception to the immediate frozen-facts spelling; it still returns access to the call's facts without another model call. An outer cancellation before a worker starts has a completed zero-send receipt.

The caller thread may read the receipt after the held request is released. `completion.result()` can block; it cannot make a held network request finish sooner. Documentation and tests must distinguish the immediate stop from final accounting. If Ian requires a frozen final `Cancelled.facts` at raise time, the binding must instead wait for the worker, and the held-call under-100-ms guarantee must be explicitly revised. An unlabelled partial snapshot is rejected because it can undercount started attempts or later replies. No background process-global fact store or optional facts switch is proposed.

## Scope

This decision concerns the Python return carrier and early-stop receipt only. ADR 0089 owns Rust facts, ordered observations, error kinds and joined accounting. ADRs 0048, 0053 and 0055 own request bytes and batching; 0209 owns stable frame columns. J5 owns later Enum/Literal, typed `annotate` rows and optional Pydantic. No source is changed by this proposal.
