# The PostgreSQL extension does not build on macOS

Status: open. Found by the wave 4 macOS dry run on 2026-09-29 at `2c5ac772b`. The same code stands at main `e2ee1d9fa`. No ticket owns it. The release issue lists it as item 7.

Priority: rank 3 of 25 in `../planning/issue-priorities-2026-09-30.md`. Owner: a new ticket, ready now; proof needs macOS.

## The problem

`cargo pgrx package` fails on macOS while compiling `databases/postgresql/src/ffi.rs`. It stops with seven E0425 errors: `open_how`, `SYS_openat2`, `O_PATH`, `RESOLVE_BENEATH`, `RESOLVE_NO_MAGICLINKS` and `AT_FDCWD` do not exist in macOS's `libc`. The file uses `openat2` and `O_PATH` at lines 268 and 293 with no platform gate.

The release plan lists PostgreSQL among the macOS packages, so the macOS release runner would fail at this part.

## What should happen

Either the extension builds and passes its package check on macOS, with a macOS path for the guarded file opens, or the release plan and the PostgreSQL README say that PostgreSQL ships for Linux only in 0.1.

## Evidence

Local experiment 218, the wave 4 macOS run on macOS 26.4 (arm64), with its release-pack log.
