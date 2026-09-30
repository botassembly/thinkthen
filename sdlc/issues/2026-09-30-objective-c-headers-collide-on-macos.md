# The Objective-C package's two headers collide on macOS

Status: open. Found by the wave 4 macOS dry run on 2026-09-29 at `2c5ac772b`. Still present at main `e2ee1d9fa`. No ticket owns it. The release issue lists it as item 7.

## The problem

The Objective-C package puts `ThinkThen.h` and `thinkthen.h` side by side in `Sources/`. macOS's default filesystem ignores case, so the two names are one file there. An unpacked Objective-C package on a default Mac volume loses one of the two headers.

Ticket 0332 fixed the checkout half. The repository no longer tracks `libraries/objective-c/Sources/thinkthen.h`, so a plain checkout on a Mac is clean. The Objective-C check copies the C header in only on Linux x86-64. `release-pack` still copies the C header into the package's `Sources/` beside `ThinkThen.h`.

The package targets macOS, so its main users meet this first.

## What should happen

Every file in the repository and in each package keeps a name that differs from its siblings in more than case. The release pack refuses a case-only name collision.

## Evidence

Local experiment 218, the wave 4 macOS run on macOS 26.4 (arm64).
