# 0685: Make the Swift package resolvable by URL and checksum

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0686, 0688

## Outcome

- The release declares the native framework as a binary target with `url:` and `checksum:`, generated from the release inventory.
- A consumer adds the package by git URL alone.
- Linux support either uses a published artifact the same way or is listed as unsupported in the inventory.
- The swiftc smoke path is deleted. Checks build only through SwiftPM.
- The Swift conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer is deleted.

### Proof

On the M5 through pm job, a clean SwiftPM consumer depends on a local git mirror of the candidate tag and a local HTTPS mirror of the framework. It resolves, builds and runs all ten named calls with no library path.

## Evidence

- **Not resolvable remotely.** `libraries/swift/Package.swift:5` declares the macOS binary target with `path:`, so SwiftPM cannot resolve the package from a git URL or registry. The README tells users to unpack a release tarball and add it as a local package.
- **Two loading paths.** On Linux the package uses dlopen on a `.so` from `Bundle.module` (`NativePackage.swift`). On macOS it uses the binary target.
- **Smoke profile bypasses the package.** It compiles the sources directly with swiftc (`check.sh:46-56`).
- **One parity case.** The routine profile runs only one (`check.sh:30,83`).
- **Silent skips.** `check.sh:15,36` exit 77.
