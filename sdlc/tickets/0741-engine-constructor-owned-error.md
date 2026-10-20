# 0741: Report engine construction failures through an owned C error

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision d721b6d, accept

## Outcome

- Changes: `libraries/c/src/ffi.rs`, `libraries/c/src/ffi/`, `libraries/c/src/failures.rs`, `libraries/c/include/thinkthen.h`, `libraries/c/tests/`, `libraries/c/README.md`, `libraries/BINDING-AUTHOR.md`, `build-tools/results/operations.py`, `build-tools/results/templates/operations.py`, `sdlc/scripts/install_check_consumers.py`, `libraries/c/examples/session.c`, `libraries/c/README-PUBLIC.md`.
- `thinkthen_engine_new_with` takes a `struct thinkthen_call_error **error` out-parameter, the same owned error `thinkthen_run_call` returns. On failure it returns NULL and sets `*error` with the code, message, retryability and facts; on success it leaves `*error` NULL. A NULL `error` pointer is allowed and reports nothing.
- `thinkthen_engine_new` and `thinkthen_engine_new_typed` follow the same pattern, or are deleted if 0690 deletes them.
- After this ticket, no caller needs a per-thread error accessor to learn why an engine was not built, so 0690 can delete every `thinkthen_error_*` accessor.
- `libraries/BINDING-AUTHOR.md` says hosts read construction failures from that owned error.

### Proof

- Through an installed C archive outside the repository, an invalid settings document makes `thinkthen_engine_new_with` return NULL with an owned error whose code and message match the native settings refusal, and a valid one returns an engine with `*error` NULL. Freeing the error twice is not required; freeing it once is clean under the existing ownership checks.
- The existing C ownership and error checks pass, then the configured gate through pm job.

## Evidence

0690 deletes every `thinkthen_error_*` and `thinkthen_session_error_*` accessor. Those per-thread accessors are the only way today to learn why `thinkthen_engine_new*` returned NULL (`libraries/c/include/thinkthen.h`, near `thinkthen_engine_new_with`). Callers that read them: `libraries/cpp/include/thinkthen/native_implementation.hpp`, `libraries/php/fixtures/constructor_error.c`, and the constructor-stage check 0640 kept. The C interface freezes at 0688, so this changes before it. Rule 4 of the 0.2.1 ruling already replaced per-thread errors with an error out-parameter for calls; this applies the same rule to construction. Found by the phase 3 readiness review, 2026-10-20.

## Progress

- 2026-10-20 Standing ruling7 covers the operation generators, installed-consumer runner and C example required by the constructor Outcome; Gator job1792509913060-750763 passed the applicable C ownership, diagnostics and installed archive checks, with combined generation and gate remaining.
