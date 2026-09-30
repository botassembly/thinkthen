# The Objective-C package's two headers collide on macOS

Status: closed 2026-09-30. Fixed by ticket 0337: the Objective-C package ships no copy of the C header, and `release-pack` refuses a source package with two names that differ only in case. The consumer takes `thinkthen.h` from the C archive, as the package README says.

## The problem

The Objective-C package put `ThinkThen.h` and `thinkthen.h` side by side in `Sources/`. macOS's default filesystem ignores case, so the two names are one file there. Ticket 0332 fixed the checkout half. `release-pack` still copied the C header into the package.

The package targets the GNU Objective-C runtime on Linux, and `release-workflow` builds no Objective-C archive on a macOS runner. Only a Mac user who unpacked the Linux archive on a default volume met the collision.

## Evidence

Local experiment 218, the wave 4 macOS run on macOS 26.4 (arm64). The rank 8 investigation in `../../planning/issue-priorities-2026-09-30.md`.
