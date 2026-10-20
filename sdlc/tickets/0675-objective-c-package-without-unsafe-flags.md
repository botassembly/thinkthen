# 0675: Ship the Objective-C package without unsafe flags

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0676, 0688, 0728

## Outcome

- Changes: `libraries/objective-c/Package.swift`, `libraries/objective-c/check.sh`, `libraries/objective-c/checks/foundation_installed.py`, `libraries/objective-c/README.md`, `libraries/objective-c/README-PUBLIC.md`, `sdlc/scripts/release-pack.d/objective-c`.
- `Package.swift` declares no unsafe flags. ARC and blocks come from standard settings or are defaults, and `-Werror` applies only inside the repository check.
- The native library ships as an xcframework binary target with `url:` and `checksum:`.
- The supported platform list, macOS 15 and anything added, comes from Objective-C's entry in `sdlc/tools.json`.
- The consumer package that `checks/foundation_installed.py` writes uses no unsafe flags either, so it proves what a remote consumer sees.

### Proof

- On the M5 through `pm job run --machine m5 0675 SHA --memory 8 -- sh libraries/objective-c/check.sh`, a clean SwiftPM consumer adds the package from a local git mirror of the candidate, builds with default flags, and runs a named call. Before the change SwiftPM refuses the package as a remote dependency because of `unsafeFlags`.
- Intel macOS is proven on the candidate in 0687.

## Evidence

- `libraries/objective-c/Package.swift` sets `cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks", "-Wall", "-Wextra", "-Werror"])]` and declares `CThinkThen` with `path: "CThinkThen.xcframework"`. SwiftPM refuses unsafe flags in a remote dependency, so no one can consume the package from a git URL or registry. Consumers may also inherit `-Werror` and break when their compiler changes.
- The consumer `Package.swift` that `checks/foundation_installed.py` writes also passes `unsafeFlags(["-fobjc-arc", "-fblocks"])`.
- `sdlc/tools.json` lists Objective-C for darwin-arm64 and darwin-x86_64 only, and `check.sh` refuses non-Darwin hosts, so the 0618 rule already holds for Linux.
- `release-pack --apple-wrappers` packs the Objective-C package with a copied `CThinkThen.xcframework`. The release workflow assembles that framework in its Apple job; 0620 owns that workflow.
- The Objective-C packet dumper is 0717's work.
