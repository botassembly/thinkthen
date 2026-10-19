# 0685: Make the Swift package resolvable by URL and checksum

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0686, 0688, 0728

## Outcome

- Changes: `libraries/swift/Package.swift`, `libraries/swift/Sources/ThinkThen/NativePackage.swift`, `libraries/swift/Sources/CThinkThen/`, `libraries/swift/check.sh`, `libraries/swift/Tests/fixtures/smoke.swift`, `libraries/swift/Tests/fixtures/package_local.py`, `libraries/swift/Tests/fixtures/installed.py`, `libraries/swift/README.md`, `libraries/swift/README-PUBLIC.md`, `sdlc/scripts/release-pack.d/swift`, `sdlc/tools/swift.json`, `site/examples/install/swift/files/Package.swift`, `site/src/data/build-lines.mjs`, `site/src/data/catalog/swift.mjs`.
- The release declares the native framework as a binary target with `url:` and `checksum:`, written by `release-pack` from the packed framework.
- A consumer adds the package by git URL alone. The README, the site install sample and the site's Swift build and download lines show that.
- Linux support either uses a published artifact the same way or is listed as unsupported for Swift in `sdlc/tools.json`.
- The swiftc smoke path in check.sh and `Tests/fixtures/smoke.swift` are deleted. Checks build only through SwiftPM.

### Proof

- On the M5 through `pm job run --machine m5 0685 SHA --memory 8 -- sh libraries/swift/check.sh`, with `THINKTHEN_ARTIFACT` naming the Apple package built at the same revision by the release route, a clean SwiftPM consumer depends on a local git mirror of the candidate and a local HTTPS mirror of the framework. It resolves, builds and runs all ten named calls with no library path. Before the change SwiftPM cannot resolve the `path:` binary target from a git URL.
- Linux Swift, if kept, runs at the 0619 container checkpoint. Intel macOS is proven on the candidate in 0687.

## Evidence

- `libraries/swift/Package.swift` declares the macOS binary target with `path: "CThinkThen.xcframework"`, so SwiftPM cannot resolve the package from a git URL or registry. The README tells users to unpack a release tarball and add it as a local package, and `site/examples/install/swift/files/Package.swift` uses `.package(path: "thinkthen-swift")`.
- Two loading paths: on Linux the package builds `Sources/CThinkThen` and dlopens a `.so` from `Bundle.module` through `NativePackage.swift`. On macOS it uses the binary target.
- The smoke profile in `libraries/swift/check.sh` compiles the sources directly with swiftc against `Tests/fixtures/smoke.swift`.
- `part_source_wrapper` in `sdlc/scripts/release-pack` copies `CThinkThen.xcframework` into the Apple package and the `.so` into the Linux package. `sdlc/tools.json` lists Swift for darwin-arm64, darwin-x86_64, linux-arm64 and linux-x86_64.
- `.github/workflows/swift-lane.yml` still calls `libraries/swift/Tests/fixtures/guard.py`, which no longer exists. 0620 owns the release workflows.
- The Swift packet dumper is 0718's work.
