# 0709 — C names drop the version suffix

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `build-tools/results/`, `sdlc/scripts/generate-c-header.py`, `libraries/c/`, `libraries/ada/`, `libraries/cobol/`, `libraries/php/`, `libraries/swift/`, `libraries/zig/`, `libraries/go/`, `libraries/csharp/`, `libraries/cpp/`, `libraries/dart/`, `libraries/jvm/`, `libraries/objective-c/`.
- No C name, type or export carries `_v1` or `_V1`, and no generated C#, Go or other wrapper name carries `V1`. The three names that would clash without the suffix get distinct names.
- The hand-bumped `THINKTHEN_INTERFACE_NUMBER` becomes a digest of the generated header, written by the header generator, so any change to the C interface changes it without anyone remembering to bump it. `thinkthen_version` returns it.
- Go, Zig and C# stop using the release number as a compatibility check and call `thinkthen_version` instead.

### Proof

- `git grep -nE '_[vV]1\b|V1\b'` over `libraries/` and `build-tools/` finds no generated or C name.
- The C library's installed tests and the routine checks of every C-based wrapper pass on the M5; a test loads a library whose header digest differs and gets the mismatch error.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The generated header has 582 distinct names with a `_v1` suffix and no `_v2` anywhere. Wrappers that load a separately built library (C#, Dart, JVM, PHP, Swift, Zig, Go, Ada, COBOL, C++) can meet a mismatched library, so a build-time identity check stays; a header digest serves that without a hand-kept number. This lands before the core freeze (0688) so every language moves once.
