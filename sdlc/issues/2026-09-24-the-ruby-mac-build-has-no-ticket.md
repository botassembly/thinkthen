# The Ruby Mac build has no ticket

Status: Open

Error-index row R5-37 says the Mac build recipe is not runnable as written. Its Ruby half is the Mac build of the Ruby surface. Ticket 0112 ports Ruby for Linux only at 0.1, and its `check.sh` prints "not run" on any other host. The release build is queue item 4 of `sdlc/planning/one-line-plan-2026-09-24.md`, and nothing tickets it today.

The release ticket carries these Ruby pieces:

- A Mac toolchain setup under `~/.cache/thinkthen-toolchains/`, pinned by sha256 as 0112's `toolchain.env` pins the Linux one.
- The extension file name from `RbConfig::CONFIG["DLEXT"]` (`.bundle` on a Mac). 0112's `build.sh` already derives it.
- The `.cargo/config.toml` `dynamic_lookup` flags, proved by a Mac build.
- A per-file bound in `check.sh` that does not rely on GNU `timeout`.
- One recorded Mac run of the Ruby check.

Found by the design review of 0112 (`sdlc/records/0112-design-review.md`, finding 1).

Ticket 0112 ran nothing on a Mac. Its `check.sh` exits 77 on any host other than Linux.
