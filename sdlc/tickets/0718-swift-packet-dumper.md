# 0718: Replace the Swift fixture consumer with one packet dumper

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0686, 0688, 0740

## Outcome

- Changes: `libraries/swift/Tests/TypeCase/main.swift`, `libraries/swift/Tests/Dump/main.swift`, `libraries/swift/Tests/fixtures/complete_parity.py`, `libraries/swift/Tests/fixtures/installed.py`, `libraries/swift/Package.swift`, `libraries/swift/check.sh`, `sdlc/scripts/release-pack.d/swift`, `sdlc/scripts/children`, `sdlc/scripts/coverage_installed_managed.py`.
- One installed Swift consumer over the public API replaces `Tests/TypeCase/main.swift` and the `session_projection` comparison in `Tests/fixtures/complete_parity.py`. The installed consumer runs each shared case through the language's run call and compares the answer, probability, presence and owned error fields it converted against `conformance/fixtures/packets.json`, as 0640 does for C. It does not print Rust's Call JSON back. The `ThinkThenTypeCase` target leaves `Package.swift`, and `release-pack` packs the consumer source in its place.
- The routine profile compares the routine shared cases instead of the single `settings-max-requests-refuses-past-the-limit` case.
- `sdlc/scripts/children` and the Swift binary names in `sdlc/scripts/coverage_installed_managed.py` follow the change.

### Proof

- On the M5 through `pm job run --machine m5 0718 SHA --memory 8 -- sh libraries/swift/check.sh`, with `THINKTHEN_ARTIFACT` naming the Apple package built at the same revision by the release route, the installed consumer's comparison passes for every routine shared case. Before the change the routine profile runs one parity case.
- The M5 builds both macOS C packages, cross-compiling `x86_64-apple-darwin` with the pinned Rust toolchain; `release-pack` takes the target and `THINKTHEN_RELEASE_EXPECTED_SHA` as the Swift and Objective-C wrappers require.
- Done once by hand and not committed: changing one expected value in the oracle fails the Swift comparison with a diff.
- Linux arm64: `pm job run --machine m5 0718 SHA --memory 16 -- sdlc/scripts/linux-check surface libraries/swift`; Linux x86_64: the same command with `--machine gator --memory 7`.

## Evidence

- `libraries/swift/Tests/fixtures/installed.py` builds `Owned`, `Parity` (from `Tests/TypeCase/main.swift`) and `Example` in a scratch SwiftPM consumer. `Tests/fixtures/complete_parity.py` drives `Parity` through `native_parity` and `session_projection.project_session`.
- `libraries/swift/check.sh` passes `--routine --case settings-max-requests-refuses-past-the-limit` outside the full profile.
- `libraries/swift/Package.swift` declares the `ThinkThenTypeCase` executable target, and `part_source_wrapper` in `sdlc/scripts/release-pack` copies `Tests/TypeCase/main.swift` into the package. `sdlc/scripts/children` lists `libraries/swift/Tests/fixtures/complete_parity.py`, and `sdlc/scripts/coverage_installed_managed.py` names the Swift binaries `Owned` and `Parity`.
- 0642 defines the dumper contract and the oracle. The installed consumer is written once against the run-call results from 0686.
- Split from 0685 on 2026-10-19 so packaging and the test move land separately.
