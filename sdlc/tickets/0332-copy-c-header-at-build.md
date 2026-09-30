# 0332: Copy the C header into the Swift and Objective-C packages at build time

Status: landed 2026-09-30. A fresh read-only code review asked for the lessons and the macOS issue update; both are fixed. Plan: `sdlc/planning/cleanup-2026-09-30.md`, "Next after the running work".

## Outcome

`libraries/c/include/thinkthen.h` is the only tracked C header. The Swift and Objective-C checks copy it into their package folders at build time. Git ignores the copies. The release archive copies it from the C source. No script checks tracked copies against the source, because no tracked copies remain.

## Evidence

- Starts from: ticket 0309 turned the two vendored headers into tracked symlinks (`8c1800f88`). Before that, `c50c06b9b` synced drifted copies by hand. Three places exist only for the links: `versions` reads the version from each link, `release-managed-pair.py` allows the two links in the source archive, and `release-pack` follows a link before it compares a packaged file with the archived commit. No other binding keeps a header in its tree. Go copies the header into build output already; C++, Zig, PHP, COBOL, Ada, Dart, C# and JVM read `libraries/c/include/thinkthen.h` in place.
- Keeps: both release archives still carry the header at the same path (`Sources/CThinkThen/include/thinkthen.h`, `Sources/thinkthen.h`). The installed-archive checks still compare the wrapper's header with the C archive's header. `release-pack` still compares every packaged file with the archived commit; the header is compared with `libraries/c/include/thinkthen.h`. The Swift and Objective-C checks build from a clean checkout.
- Changes: delete the two tracked symlinks and ignore their paths. `libraries/swift/check.sh` and `libraries/objective-c/check.sh` copy the header in before they build. `release-pack` copies the header from `libraries/c/include/`. `versions` drops the two copied-header places and its copied-header self-test case. `release-managed-pair.py` drops the two allowed links. The Swift README adds the copy step for a checkout build. A plain checkout on a default Mac volume is no longer dirty, since no tracked `thinkthen.h` sits beside `ThinkThen.h`; the macOS header issue says so.
- Proof: the Swift and Objective-C check scripts pass with no header present at start. `versions --self-test`, `versions`, `release-managed-pair-self-test.py` and `release-archive-self-test.py` pass. `python3 sdlc/scripts/tickets`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/lint` and `sdlc/scripts/surfaces --registry` pass.
- Defers: SwiftPM cannot run a copy step for a system library target, so a plain `swift build` or a `.package(path:)` consumer of a checkout needs the copy first. The README says so. A release archive needs nothing, since it ships the header. The Objective-C release archive still puts `thinkthen.h` beside `ThinkThen.h`, which collide on a case-blind Mac volume; `sdlc/issues/2026-09-30-objective-c-headers-collide-on-macos.md` keeps that half open.

## What the build taught us

- The two links had become the only reason for three release checks. Deleting the links removed a version place pair, a self-test case and an allowed-link table.
- SwiftPM's system library target cannot run a copy step, so the check script and the release archive own the copy. A checkout build needs one `cp`.
- `release-managed-pair-self-test.py` archives `HEAD`, so it reads committed links and ignores working-tree deletions. Commit before running it.
