# 0709: C names drop the version suffix

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `build-tools/results/generate.py`, `build-tools/results/layouts.py`, `build-tools/results/native.py`, `build-tools/results/operations.py`, `build-tools/results/templates/`, `conformance/library-surface-test.py`, `conformance/library-surface.json`, `libraries/ada/checks/`, `libraries/ada/src/`, `libraries/c/README-PUBLIC.md`, `libraries/c/README.md`, `libraries/c/cbindgen.toml`, `libraries/c/examples/session.c`, `libraries/c/include/thinkthen.h`, `libraries/c/src/`, `libraries/c/tests/`, `libraries/cobol/checks/`, `libraries/cobol/src/`, `libraries/cpp/include/thinkthen/native_implementation.hpp`, `libraries/cpp/include/thinkthen/results_generated.hpp`, `libraries/csharp/src/NativeBridge.g.cs`, `libraries/csharp/src/OwnedCall.cs`, `libraries/csharp/src/OwnedUsage.cs`, `libraries/dart/lib/src/session/abi_generated.dart`, `libraries/dart/lib/src/session/client.dart`, `libraries/go/producer.go`, `libraries/go/session.go`, `libraries/jvm/session/thinkthen/Engine.java`, `libraries/jvm/session/thinkthen/NativeResults.java`, `libraries/objective-c/Sources/Foundation/TTResults.g.m`, `libraries/objective-c/Sources/Foundation/ThinkThen.m`, `libraries/php/fixtures/constructor_error.c`, `libraries/php/fixtures/session_consumer.php`, `libraries/php/src/`, `libraries/swift/Sources/`, `libraries/zig/src/`, `sdlc/scripts/check-c-exports.py`, `sdlc/scripts/install_check_consumers.py`, `sdlc/scripts/registry-readmes-self-test.py`, `sdlc/scripts/registry-readmes.py`, `site/examples/functions/*/c.c`, `specification/recognize.md`, `specification/settings.md`, `specification/types.md`.
- No C name, type or export carries `_v1` or `_V1`, and no generated C#, Go or other wrapper name carries `V1`. The three names that would clash without the suffix get distinct names.
- The change regenerates `libraries/c/include/thinkthen.h` with `sdlc/scripts/generate-c-header.py`, passes `sdlc/scripts/check-c-exports.py`, regenerates every wrapper from `build-tools/results/`, and updates `conformance/library-surface.json`, the C README examples and the site's C function examples in the same commit.
- The interface number and the wrappers' release-number checks are 0725.

### Proof

- `git grep -nE '_[vV]1\b|[a-z]V1\b' -- libraries build-tools site/examples sdlc/scripts specification ':!libraries/go/authored_json_skip_v2*.go'` finds nothing. Before the change it finds the 582 suffixed names.
- On the M5 through pm job, `sdlc/scripts/lint` (header currency and export check) and `sdlc/scripts/surfaces` pass: the C library's installed tests and the routine check of every C-based wrapper that runs on macOS. Linux-only wrapper builds run at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The generated header has 582 distinct names with a `_v1` suffix and no `_v2` anywhere.

Starts from `libraries/c/cbindgen.toml` and the Rust carriers under `libraries/c/src/` that cbindgen reads, the layout and operation generators `build-tools/results/layouts.py`, `build-tools/results/operations.py`, `build-tools/results/native.py` and `build-tools/results/templates/`, the hand-written callers in each wrapper (for example `libraries/csharp/src/OwnedCall.cs`, `libraries/go/session.go`, `libraries/jvm/session/thinkthen/Engine.java`), and the C snippets in `sdlc/scripts/check-c-exports.py`, `sdlc/scripts/install_check_consumers.py` and `sdlc/scripts/registry-readmes.py`. The `_v2` and `_v27` Go file names are Go toolchain build tags and stay.

This renames every C name, so it collides with the phase 2 C tickets 0640 (C example and dumper), 0641 (C README and packaging) and 0642 (dumpers), and with 0719 and 0707 on generated wrappers. Either order lands; whichever lands second rebases onto the new names. Every phase 3 language ticket waits on 0688 and so starts from the renamed names.
