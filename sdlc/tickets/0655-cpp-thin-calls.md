# 0655: Thin the C++ calls onto run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

Depends on: 0653, 0727

## Outcome

- Changes: `build-tools/results/templates/cpp.py`, `build-tools/results/templates/cpp_results.py`, `build-tools/results/legacy/cpp.txt`, `libraries/cpp/include/thinkthen/`, `libraries/cpp/src/client.cpp`, `libraries/cpp/examples/`, `libraries/cpp/fixtures/`, `libraries/cpp/check.sh`, `libraries/cpp/README.md`, `libraries/cpp/README-PUBLIC.md`, `conformance/library-surface/cpp.json`, `conformance/library-surface-test.py`, `conformance/test_parity.py`, `site/examples/functions/*/cpp.cpp`, `site/src/data/catalog.mjs`.
- Each named call runs `thinkthen_run_call` once with one request on the input path 0635 chose, and passes no language identity.
- `Call`, `Poll`, `ResultCall` and the rest of the C++ session loop, with its private closed, reader and producer error text, are deleted. Async use runs the call on one worker and cancels through the handle.
- `value` and `probability` read the packet's answer fields. The per-function projections that `cpp.py` and `cpp_results.py` generate are deleted, and both leave this language's exception file `build-tools/results/legacy/cpp.txt`.
- `tt::Result` and its parts have public `to_json()` members. No example or site sample names a `detail` namespace.
- `plan_impl` passes the request through without re-splitting it.
- A short C++ dumper over the public API, following the 0642 contract, replaces `fixtures/owned/main.cpp`, `fixtures/owned_calls.py`, `fixtures/result_construction.py`, `fixtures/schema_projection.cpp` and `fixtures/session_consumer.cpp`, which are deleted. The concurrency, lifetime and AddressSanitizer tests in `fixtures/tests/` stay as small tests of C++ ownership.
- The C++ entry in `conformance/library-surface/cpp.json`, the README and the site's C++ text describe the remaining API. The site's C++ samples replay with it.

### Proof

- On the M5 through `pm job run --machine m5 0655 SHA --memory 8 -- sh libraries/cpp/check.sh`, the installed CMake package builds every example from outside the repository with `-Werror`, and the C++ dumper matches the 0642 oracle for every shared case.
- `python3 build-tools/results/answer_rules.py` and `python3 conformance/library-surface.py --library cpp` pass. Before the change, removing `cpp.py` and `cpp_results.py` from `LEGACY_FILES` fails the first check.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/cpp.txt` and its surface entry in `conformance/library-surface/cpp.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- `Client::start` in `libraries/cpp/include/thinkthen/native_implementation.hpp` serializes question, input and options separately and starts each call through `thinkthen_session_new_named_with_surface` with the identity `"cpp"`. `Call::read` drives `thinkthen_session_next_packet`, `thinkthen_session_try_read` and `thinkthen_session_wait`.
- Each call's value and probability come from a generated `Project` function per function (`ResultCall<Project>` in `client.hpp`, and `calls_generated.hpp`). `answer_rules.py` lists `cpp.py` and `cpp_results.py` as legacy exceptions.
- The examples and `site/examples/functions/*/cpp.cpp` print results through `tt::detail::serialize` and `tt::results::detail::encode`.
- `Client::plan_impl` re-reads the request to split it again before `thinkthen_request_plan_named_value`.
- The fixture consumers use the deleted session loop: `fixtures/owned/main.cpp` calls `poll`, `next` and `collect`, and `check.sh` runs `fixtures/session_consumer.cpp` through `conformance/children/session_cases.py`. So the dumper lands with this ticket. `test_cpp_full_runner_emits_each_required_case_once` in `conformance/test_parity.py` reads that `check.sh` line.
- `conformance/library-surface.json` lists `Call`, `ResultCall`, `Poll` and `PollState` for C++, and `conformance/library-surface-test.py` plants changes in `client.hpp` and `calls_generated.hpp`. `site/src/data/catalog.mjs` tells C++ callers to use `collect` or `poll`.
- Since 2026-09-15, 25 of 129 C++ commits are repairs, mostly fixture repairs: b37410df5, da2938041, 89c57312f, ca3ee370a.
