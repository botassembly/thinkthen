# 0675: Ship the Objective-C package without unsafe flags

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0676, 0688

## Outcome

- `Package.swift` declares no unsafe flags. ARC and blocks come from standard settings or are defaults, and `-Werror` applies only inside the repository check.
- The native library ships as an xcframework binary target with `url:` and `checksum:`.
- The supported platform list, macOS 15 and anything added, comes from the shared inventory.
- The Objective-C conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer is deleted.

### Proof

On the M5 through pm job, a clean SwiftPM consumer adds the package from a local git mirror of the candidate, builds with default flags, and runs a named call.

## Evidence

- **Unsafe flags block remote use.** `libraries/objective-c/Package.swift:11` sets `cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks", "-Wall", "-Wextra", "-Werror"])]`. SwiftPM refuses unsafe flags in a remote dependency, so no one can consume the package from a git URL or registry.
- **`-Werror` on consumers.** Consumers may also inherit `-Werror` and break when their compiler changes. That is uncertain.
- **Not run on Linux.** `check.sh:8` exits 77 there. 0618 requires the manifest to list Linux as unsupported or the gate to run it.
