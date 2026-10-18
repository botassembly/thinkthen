# 0653: Ship C++ as a CMake package that finds its bundled library

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0619, 0641, 0688

## Outcome

- The C++ archive contains the matching native library for its platform. `find_package(thinkthen-cpp)` locates it with no extra variables.
- Library names and system link flags come from CMake per platform (Linux, macOS, Windows), not literals.
- The implementation lives in a checked-in source file; configure writes no source.
- The C++ archive is assembled from the C archive by the release script, not by caller input.

### Proof

- On the M5 through pm job, natively and in the pinned arm64 container, an outside CMake project uses `find_package(thinkthen-cpp)` against the unpacked archive and runs an example, with no -D native paths.
- Windows, Intel macOS and Linux x86_64 are proven on the candidate in 0687.

## Evidence

- libraries/cpp/CMakeLists.txt:5-7 refuses to configure unless the caller passes THINKTHEN_NATIVE_SHARED, THINKTHEN_NATIVE_STATIC and THINKTHEN_C_HEADER by hand.
- It hardcodes the Linux file name `libthinkthen.so.0` (:19, :43) and Linux-only link flags `pthread dl m` (:33).
- It writes a one-line client.cpp at configure time (:8).
- The contract asks for "CMake package with prebuilt library" with no manual library path (BINDING-AUTHOR.md, C++ table).
