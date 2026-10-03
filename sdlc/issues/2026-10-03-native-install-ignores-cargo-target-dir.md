# `native_install` loses the C library when `CARGO_TARGET_DIR` is set

Status: open. Filed 2026-10-03 from the docs message "native_install loses the C library when CARGO_TARGET_DIR is set" of 2026-10-01. Owner: the queue owner.
Milestone: 0.2

The site's `npm run smoke-bindings` calls `native_install` in `sdlc/scripts/installed.sh`. On main `81062bd31` that function copies `libraries/c/target/debug/libthinkthen_c.so` by a fixed path. With `CARGO_TARGET_DIR` set to a folder outside the worktree, Cargo writes the library there, the copy finds nothing, and 65 samples over the C interface fail. With the variable unset, all 138 samples pass. Line 34 of the same file already reads `${CARGO_TARGET_DIR:-target}` for the conformance backend.

The fix reads the library from `${CARGO_TARGET_DIR:-libraries/c/target}`, or refuses with one sentence when the variable is set. A planted run with the variable set proves it. Site builders unset the variable until then.
