# Language merge runbook

Status: open, prepared by the consumer-language program on 2026-09-28. Everything here is consolidation of verified facts from local experiments 273-301; nothing new is claimed. The merging agent copies from this table and never from sealed stage-two folders where a repin copy exists. Companion to `2026-09-28-language-port-integration-priority-brief.md` and `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`.

## 1. Copy table (binding -> merge input -> destination)

| Binding | Copy from (local experiment) | Destination | Notes |
| --- | --- | --- | --- |
| Zig | 273 `repin/package/` | `libraries/zig/` | binding source identical to stage2; README is the repin one |
| Go | 274 `repin/package/` | `libraries/go/` | module path is rehearsal `example.org/thinkthen-go` — owner picks the published path |
| JVM (Java/Kotlin/Scala) | 289 `repin/package/` | `libraries/jvm/` | Kotlin/Scala example mains were fixed in repin only; sealed copies are stale |
| C# | 290 `repin/package/` | `libraries/csharp/` | nupkg id per registration to-do (`Botassembly.` reserved prefix) |
| PHP | 291 `repin/package/` | `libraries/php/` | repin README explains `PHP_BINDING` until integration lands |
| COBOL | 292 `repin/package/` | `libraries/cobol/` | copybook unchanged; width discipline `BY VALUE SIZE IS 8` |
| Ada | 293 `repin/package/` | `libraries/ada/` | repin adds `Call_Value`/`Call_Facts` accessors |
| Swift | 294 `repin/package/` | `libraries/swift/` | packaged example asserts the envelope; sealed example fails on main |
| Objective-C | 295 `repin/package/` | `libraries/objective-c/` | GNU runtime, no Foundation; tokenizer carries envelope validation |
| Dart | 300 `stage2/package/` + `repin/package/README.md` | `libraries/dart/` | binding byte-identical; take the repin README; `stage3/thinkthen_flutter` is the Flutter-surface input |
| C++ | 301 `stage2/package/` | `libraries/cpp/` | post-fix (config + parser); CMake config name `thinkthen-cpp`, targets `thinkthen::thinkthen_cpp_shared/static` |

## 2. Toolchain pins for CI (ubuntu-24.04 runners)

Versions proven on this host; SHA-256 values live in each experiment's `inputs/toolchain.json` — cite the file, not memory. Zig 0.15.2; Go 1.22.2 (distro); JDK 21.0.12.1 + Kotlin 2.4.20 + Scala 3.9.0; dotnet-sdk-8.0 8.0.131; PHP 8.3.6 (`php -d ffi.enable=1`); gnucobol4 4.0-early-dev; gnat/gprbuild 13.3; Swift 6.4.0; Dart 3.13.4; Flutter 3.47.5; gobjc 4:13.2.0; Rust 1.95.0; gcc 13.3 / clang 18.1.3. Ubuntu packages installed by hand here: ninja-build, libgtk-3-dev (Flutter Linux build). Each experiment's gate is the reference command set for its CI job.

## 3. Registry metadata (decided vs open)

- NuGet: `Botassembly.` prefix reserved (Ian's to-do); package id `Botassembly.ThinkThen.C` carries the native door, wrapper package per the C# issue.
- Maven Central: namespace `io.github.botassembly` (Ian's ruling 2026-09-28); artifactId open — suggest `thinkthen-jvm`.
- Packagist: vendor `botassembly/thinkthen`, auto-publish from tags.
- pub.dev: `thinkthen_dart`, trusted publishing linked to the repo.
- Go: module path open (rehearsal used `example.org/thinkthen-go`).
- SwiftPM: package name `ThinkThen`; Swift Package Index listing optional later.
- Zig/CMake/COBOL/Ada/ObjC: no registry; GitHub Releases + per-language README; vcpkg/Conan/Alire optional add-ons.
- JVM trusted publishing / signing: GPG key creation is in Ian's registration to-do.

## 4. Per-language gate commands

Each experiment's `repin/check.sh` (or `stage2/check.sh` for Dart/C++) is the single entrypoint: offline native build under the shared lock, header-derived export check (21), package build, isolated consumers, strict contract, planted negatives. Port them to CI by replacing the lock and loopback-allow-list with runner defaults; do not remove planted negatives.

## 5. M5 Mac first commands (Apple lane)

1. Build the native library for macOS (the product lane already has the Intel macOS route; ARM64 selector proofs exist in the repo).
2. `libraries/swift` `swift build` + run its gate; then XCFramework packaging for iOS/macOS (new work, unproven).
3. `libraries/cpp` CMake build under AppleClang; run the shared consumer gate.
4. Dart/Flutter macOS embedder run mirrors the Linux `stage3` pattern (`flutter run -d macos`).

## 6. Documentation pages

Per-language page skeleton agreed 2026-09-28 (what it is / where the code lives / install from registry / install from source / native-archive rule / platforms / example / contract notes / registry). Source content: each repin README's clone-build-use triple. Site content belongs to Marketing's lane; this runbook only supplies the raw material.

## What the program did NOT do

No `libraries/` writes, no CI configuration, no publication, no product-lane changes, no live-model runs. All evidence is local experiments; rehearsal archives never ship.
