# 0525: Make C++ thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision 7e68e2e90a0bc885944c11e0164cc8c6844727ce, accept

## Outcome

C++ callers find one CMake package that carries the native library, call the ten functions with standard library values, and read generated C++ result types with explicit presence. Failures are typed exceptions with retained facts. RAII releases every native resource, and cancellation does not block the caller. Hand-copied layouts, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). C++ blocks the caller and keeps hand-copied native views in `libraries/cpp/include/thinkthen/native.hpp` and `native_views.hpp`. Callers install the native library themselves.
- Keeps: All ten functions, files and images, failure facts, cache and replay behavior and installed package checks.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the C++ section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the C++ target template and generated headers;
  - native input conversion, typed exceptions, the declared scheduling idiom, cancellation and RAII cleanup;
  - the CMake package slice under 0517's design, starting from `libraries/cpp/cmake/ThinkThenCppConfig.cmake.in`;
  - `libraries/cpp/README.md`, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/cpp/**` narrowed and named per slice.
- Proof: Shared conformance runs through an installed CMake consumer, covering absent, null, failures, unknown output fields, files, images and cancellation. If the API offers async calls, one installed held-provider case shows other work progressing, and cancel and cleanup returning before the provider is released. Pending final facts stay pending. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Dead `complete.hpp` removal goes in 0515. Final distribution assembly belongs to 0530. Native qualification beyond this machine follows Ian's release hold.

## Progress

- 2026-10-10 landed a7ff49569; next: C++ number parsing and serialization preserve JSON semantics under comma locales; PHP polling advances bounded generators and preserves terminal facts. Focused old/new and public consumer checks pass with accepted source review. Installed distributions, platform qualification and compatibility retirement remain open.
