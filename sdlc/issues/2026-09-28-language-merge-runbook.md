# Language merge runbook

Status: open. The consumer-language program prepared the experiment handoff on 2026-09-28. Ticket 0249 authorizes source integration; its build records hold current package proof. The table below names historical source inputs, not finished product packages. Use the accepted preparation records to apply later contract corrections. Never replace integrated source with an older experiment copy. Companion to `2026-09-28-language-port-integration-priority-brief.md` and `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`.

## 1. Copy table (binding -> merge input -> destination)

| Binding | Copy from (local experiment) | Destination | Notes |
| --- | --- | --- | --- |
| Zig | 273 `repin/package/` | `libraries/zig/` | retain the re-pin cancellation and envelope fixes; apply current settings and error-facts changes from Swift/Zig preparation |
| Go | 274 `repin/package/` | `libraries/go/` | replace rehearsal `example.org/thinkthen-go` with `github.com/botassembly/thinkthen/libraries/go`; nested module tags remain release work |
| JVM (Java/Kotlin/Scala) | 289 `repin/package/` | `libraries/jvm/` | Kotlin/Scala example mains were fixed in repin only; sealed copies are stale |
| C# | 290 `repin/package/` | `libraries/csharp/` | product wrapper `Botassembly.ThinkThen`; `Botassembly.ThinkThen.C` remains reserved for the separate native package |
| PHP | 291 `repin/package/` | `libraries/php/` | integrated Composer package is `botassembly/thinkthen`; use its product README |
| COBOL | 292 `repin/package/` | `libraries/cobol/` | copybook unchanged; width discipline `BY VALUE SIZE IS 8` |
| Ada | 293 `repin/package/` | `libraries/ada/` | repin adds `Call_Value`/`Call_Facts` accessors |
| Swift | 294 `repin/package/` | `libraries/swift/` | retain the re-pin example repair and add the separate post-J1 settings-constructor correction; see Swift/Zig preparation |
| Objective-C | 295 `repin/package/` | `libraries/objective-c/` | GNU runtime, no Foundation; tokenizer carries envelope validation |
| Dart | 300 `stage2/package/` + `repin/package/README.md` | `libraries/dart/` | take the re-pin README; Flutter wrapper and example are at stage-three root, not `stage3/thinkthen_flutter` or the duplicate Dart `stage3/package/`; product destination is `libraries/dart/flutter/` |
| C++ | 301 `stage2/package/` | `libraries/cpp/` | post-fix (config + parser); CMake config name `thinkthen-cpp`, targets `thinkthen::thinkthen_cpp_shared/static` |

## 2. Toolchain evidence for future CI (ubuntu-24.04 runners)

Versions proven on this host; SHA-256 values live in each experiment's `inputs/toolchain.json` — cite the file, not memory. Zig 0.15.2; Go 1.22.2 (distro); JDK 21.0.12.1 + Kotlin 2.4.20 + Scala 3.9.0; dotnet-sdk-8.0 8.0.131; PHP 8.3.6 (`php -d ffi.enable=1`); gnucobol4 4.0-early-dev; gnat/gprbuild 13.3; Swift 6.4.0; Dart 3.13.4; Flutter 3.47.5; gobjc 4:13.2.0; Rust 1.95.0; gcc 13.3 / clang 18.1.3. Ubuntu packages installed by hand here: ninja-build, libgtk-3-dev (Flutter Linux build). The experiments establish local host evidence, not completed CI. Product checks must discover the tools they actually execute and return 77 for a missing prerequisite. Use each integration build record for the tools and host it qualified.

## 3. Registry metadata (decided vs open)

- NuGet: intended prefix `Botassembly.` (registration remains Ian's to-do); package ID `Botassembly.ThinkThen.C` is reserved for the native door; the integrated wrapper uses `Botassembly.ThinkThen`. Source integration does not establish account ownership or publish either package.
- Maven Central: namespace `io.github.botassembly` (Ian's ruling 2026-09-28); artifact ID `thinkthen-jvm` is the integrated product name.
- Packagist: package `botassembly/thinkthen`; configure tag publication during release work.
- pub.dev: `thinkthen_dart`; configure trusted publishing during release work.
- Go: product module path `github.com/botassembly/thinkthen/libraries/go`, accepted in the Go/C++ preparation. Module tags and distribution remain separate.
- SwiftPM: package name `ThinkThen`; Swift Package Index listing optional later.
- Zig/CMake/COBOL/Ada/ObjC: no registry; GitHub Releases + per-language README; vcpkg/Conan/Alire optional add-ons.
- JVM trusted publishing / signing: GPG key creation is in Ian's registration to-do.

## 4. Per-language gate commands

Each integrated package has a product-local `check.sh`. The experiment gate and its fixtures are inputs to that check, not a dependency on an experiment checkout. Build the current native dependency source offline, and compare the installed library with the declarations in the matching header. The current header declares 30 exports; the earlier 21-export count is historical. A header-only comparison does not prove an unchanged engine binary.

Prove the current shared type corpus through the public host API, exact complete request bodies, isolated consumers, and the package's distinct settings, failure ownership, cancellation and privacy boundaries. The corpus currently has 55 schema cases and 29 executable public-binding cases; derive these subsets at the checked pin. Preserve meaningful planted failures. Reuse unchanged receipts and correct only the failed component; source integration does not require another whole-core, all-surface or stress run.

Use separate worktree outputs and lane locks. Independent builds may overlap when capacity allows, as the work plan records. The shared corpus startup builds its own backend before executing it; a preexisting executable must not be an unrecorded prerequisite. Actual Ubuntu 24.04 CI jobs and final release archive proof remain in the original port issues.

## 5. M5 Mac first commands (Apple lane)

1. Build the native library for macOS (the product lane already has the Intel macOS route; ARM64 selector proofs exist in the repo).
2. `libraries/swift` `swift build` + run its gate; then XCFramework packaging for iOS/macOS (new work, unproven).
3. `libraries/cpp` CMake build under AppleClang; run the shared consumer gate.
4. Dart/Flutter macOS embedder run mirrors the Linux `stage3` pattern (`flutter run -d macos`).

## 6. Documentation pages

Per-language page skeleton agreed 2026-09-28 (what it is / where the code lives / install from source / native-archive rule / platforms / example / contract notes). Show registry installation only once the name is held and the package exists. Use the integrated README and its proven clone-build-use recipe; a re-pin README is only a source input. Site content belongs to Marketing's lane; this runbook only supplies the raw material.

## Integration boundary

The original experiment program made no `libraries/` edits. Ticket 0249 now owns those source integrations and records each independent review. Original port issues retain their final-release pin, CI, native archive and distribution criteria. Local package proof does not close them. Nothing in this runbook publishes a package or configures accounts. Rehearsal archives never ship.

Accepted preparation: `../records/0249-php-dart-preparation.md`, `../records/0249-swift-zig-preparation.md`, `../records/0249-go-cpp-preparation.md`, and `../records/0249-ada-objc-cobol-preparation.md`. Ticket 0249 records C#/JVM preparation and all integration progress.
