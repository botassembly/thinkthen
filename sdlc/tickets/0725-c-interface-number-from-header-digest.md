# 0725: The C interface number is a digest of the header

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0709

## Outcome

- Changes: `sdlc/scripts/generate-c-header.py`, `libraries/c/src/ffi/values.rs`, `libraries/c/src/ffi/runtime_version/ffi.rs`, `libraries/c/include/thinkthen.h`, `libraries/c/README.md`, `libraries/c/tests/c/runtime_version.c`, `libraries/c/tests/door/runtime_version.rs`, `libraries/go/thinkthen.go`, `libraries/zig/build.zig`, `libraries/csharp/src/NativeLoader.cs`, `libraries/csharp/tests/package_check.py`.
- The hand-bumped `THINKTHEN_INTERFACE_NUMBER` becomes a digest of the generated header, written by the header generator, so any change to the C interface changes it without anyone remembering to bump it. `thinkthen_version` returns it.
- Go, Zig and C# stop comparing `THINKTHEN_VERSION_MAJOR`, `MINOR` and `PATCH` as a compatibility check and call `thinkthen_version` at load instead. The release number macros stay as the package release number.

### Proof

- On the M5 through `pm job run --machine m5 0725 SHA --memory 8 -- cargo nextest run --manifest-path libraries/c/Cargo.toml runtime_version`, a test loads a library whose header digest differs and gets the mismatch error, and editing one header declaration without regenerating fails `sdlc/scripts/generate-c-header.py --check`. Before the change, the number stays 1 after a header change.
- On the M5 through pm job, `sdlc/scripts/surfaces` passes the Go, Zig and C# checks.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md), which keeps a check that a wrapper loaded the native library from the same build. Wrappers that load a separately built library (C#, Dart, JVM, PHP, Swift, Zig, Go, Ada, COBOL, C++) can meet a mismatched library, so the identity check stays; a header digest serves it without a hand-kept number. Split from 0709 because it changes the generator and three wrappers' load checks, which land apart from the rename.

Starts from `THINKTHEN_INTERFACE_NUMBER` in `libraries/c/src/ffi/values.rs` and `thinkthen_version` in `libraries/c/src/ffi/runtime_version/ffi.rs` (0692), the cgo `#if THINKTHEN_VERSION_MAJOR` guard in `libraries/go/thinkthen.go`, `headerVersionMatches` in `libraries/zig/build.zig`, and the header version check in `libraries/csharp/tests/package_check.py`.
