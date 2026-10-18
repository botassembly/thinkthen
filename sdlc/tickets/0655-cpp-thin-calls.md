# 0655: Thin the C++ calls onto run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Calls go through the single run-call entry point with one request on the input path 0635 chose.
- `Call::read` and the rest of the C++ session loop, with its private closed, reader and producer error text, are deleted. Async use runs the call on one worker and cancels through the handle.
- `value` and `probability` read the packet's answer fields. The generated per-function projection templates in sdlc/generators/results/templates/cpp.py are deleted.
- `tt::Result` and its parts have public `to_json()` (or `operator<<`) members. No example or test outside fixtures names a `detail` namespace.
- `plan_impl` passes the request through without re-splitting it.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed CMake package builds every example from outside the repository with `-Werror`. The C++ packet dumper matches the 0642 oracle.

## Evidence

- The C++ client starts each call through `thinkthen_session_new_named_with_surface` after serializing question, input and options separately (libraries/cpp/include/thinkthen/native_implementation.hpp:83-90).
- Each call's value and probability come from a generated `Project` function per function (`ResultCall<Project>`, client.hpp:43-48; calls_generated.hpp). That is one of about fifteen copies of the projection logic.
- The public examples reach into internal namespaces to print results: examples/decide.cpp:13 uses `tt::detail::serialize` and `tt::results::detail::encode`.
- `plan_impl` re-parses a document with `value.at("function")` to split it again (native_implementation.hpp:92-98).
