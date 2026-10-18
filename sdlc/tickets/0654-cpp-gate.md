# 0654: Replace the C++ fixture consumer and tool shims in its gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0653, 0688

## Outcome

- The gate takes cmake and python3 from the 0615 manifest; the absolute tool variables go there.
- A short C++ dumper over the public API, following the 0642 contract, replaces fixtures/owned/main.cpp, owned_calls.py, result_construction.py and schema_projection.cpp.
- The concurrency, lifetime and ASan checks remain as small tests of C++ ownership.

### Proof

On the M5 through pm job, the gate installs the CMake package, builds the dumper outside the repository, and matches the 0642 oracle for every shared case.

## Evidence

- libraries/cpp/check.sh:13-17 requires absolute THINKTHEN_CMAKE_BIN and THINKTHEN_PYTHON_BIN paths and exits 77 when a tool is missing.
- C++ conformance runs through a 305-line consumer (fixtures/owned/main.cpp) plus Python drivers (owned_calls.py 256 lines, result_construction.py, schema_projection.cpp).
- Since 2026-09-15, 25 of 129 C++ commits are repairs, mostly fixture repairs: b37410df5, da2938041, 89c57312f, ca3ee370a.
