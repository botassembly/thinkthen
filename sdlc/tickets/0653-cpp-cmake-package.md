# 0653: Ship C++ as a CMake package that finds its bundled library

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0619, 0641, 0688, 0728

## Outcome

- Changes: `libraries/cpp/CMakeLists.txt`, `libraries/cpp/cmake/ThinkThenCppConfig.cmake.in`, `libraries/cpp/src/client.cpp`, `libraries/cpp/check.sh`, `libraries/cpp/README.md`, `libraries/cpp/README-PUBLIC.md`, `sdlc/scripts/release-pack.d/cpp`, `sdlc/scripts/release-go-cpp-pair`, `site/src/data/build-lines.mjs`.
- The C++ archive contains the matching native library for its platform. `find_package(thinkthen-cpp)` locates it with no extra variables.
- Library names and system link flags come from CMake per platform (Linux, macOS, Windows), not literals.
- The implementation lives in a checked-in source file, `libraries/cpp/src/client.cpp`; configure writes no source.
- The release script assembles the C++ archive from the C archive. Callers pass no native paths.
- The gate takes `cmake` and `python3` from the resolved manifest tools and drops its own absolute-path checks.
- The README's build commands and the C++ build line in `site/src/data/build-lines.mjs` lose the three `THINKTHEN_` CMake inputs, and the site's C++ samples replay with the new line.

### Proof

- On the M5 through `pm job run --machine m5 0653 SHA --memory 8 -- sh libraries/cpp/check.sh`, an outside CMake project uses `find_package(thinkthen-cpp)` against the installed package and runs an example, with no `-D` native paths. Before the change, configure refuses without `THINKTHEN_NATIVE_SHARED`, `THINKTHEN_NATIVE_STATIC` and `THINKTHEN_C_HEADER`, and the package config names `libthinkthen.so.0`, which macOS does not have.
- At the 0619 container checkpoint, the release suite passes the C++ gate in the arm64 container on the M5. Windows, Intel macOS and Linux x86_64 are proven on the candidate in 0687.

## Evidence

- `libraries/cpp/CMakeLists.txt` refuses to configure unless the caller passes `THINKTHEN_NATIVE_SHARED`, `THINKTHEN_NATIVE_STATIC` and `THINKTHEN_C_HEADER`.
- It hardcodes the Linux file name `libthinkthen.so.0` and the Linux-only link flags `pthread dl m`. `cmake/ThinkThenCppConfig.cmake.in` names `libthinkthen.so.0` too.
- It writes a one-line `client.cpp` that includes `native_implementation.hpp` at configure time.
- The `cpp` case of `sdlc/scripts/release-pack` copies sources only; the README tells callers to pass the C archive's files by hand, and `sdlc/scripts/release-go-cpp-pair` checks the pair.
- `libraries/cpp/check.sh` resolves tools through `sdlc/scripts/resolved-tools` since 0615, but still refuses non-absolute tool paths, and passes `.so` paths for the native library.
- `site/src/data/build-lines.mjs` passes the three `THINKTHEN_` inputs and `-l:libthinkthen.so.0` for every C++ sample.
- The contract asks for a "CMake package with prebuilt library" with no manual library path (`libraries/BINDING-AUTHOR.md`, C++ table).
