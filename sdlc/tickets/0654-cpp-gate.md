# 0654: Replace the C++ fixture consumer and tool shims in its gate

Status: WITHDRAWN.

Opened as: 2026-10-19, 0615 and 0618 already did its gate work, and its packet dumper lands with 0655..

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0653, 0688

Depends on: 0655

## Outcome

- Changes: `libraries/cpp/check.sh`.
- The gate takes cmake and python3 from the 0615 manifest; the absolute tool variables go there.
- A short C++ dumper over the public API, following the 0642 contract, replaces fixtures/owned/main.cpp, owned_calls.py, result_construction.py and schema_projection.cpp.
- The concurrency, lifetime and ASan checks remain as small tests of C++ ownership.

### Proof

On the M5 through `pm job run --machine m5 0654 SHA --memory 8 -- sh libraries/cpp/check.sh`, the gate installs the CMake package, builds the dumper outside the repository, and matches the 0642 oracle for every shared case.

## Evidence

- `libraries/cpp/check.sh` resolves `cmake` and `python3` through `sdlc/scripts/resolved-tools` since 0615, and 0618 made a missing tool fail. It still refuses a non-absolute tool path.
- `fixtures/owned/main.cpp` calls `poll`, `next` and `collect` on the C++ session loop that 0655 deletes.

## Amendment, 2026-10-19

This ticket's work now lives elsewhere. 0615 and 0618 moved the gate's tools to the manifest; 0653 drops the remaining absolute-path checks while it rewrites the gate's CMake step. The fixture consumers use the session loop that 0655 deletes, so 0655 builds the dumper and deletes them. The PM should withdraw this ticket as superseded by 0653 and 0655. It depends on 0655 so it never starts alone.
