# The Objective-C package's two headers collide on macOS

Status: open. Found by the wave 4 macOS dry run on 2026-09-29 at `2c5ac772b`. Still present at main `d2262df9a`.

## The problem

`libraries/objective-c/Sources/` holds `ThinkThen.h` and `thinkthen.h`. `thinkthen.h` is a link to `../../c/include/thinkthen.h`. macOS's default filesystem ignores case, so the two names are one file there.

- A plain checkout of this repository on a default Mac volume is dirty at once. `git status` shows `ThinkThen.h` modified, because the C header won on disk.
- An unpacked Objective-C package on the same volume loses one of the two headers.

The package targets macOS, so its main users meet this first.

## What should happen

Every file in the repository and in each package keeps a name that differs from its siblings in more than case. The release pack refuses a case-only name collision.

## Evidence

Local experiment 218, the wave 4 macOS run on macOS 26.4 (arm64).
