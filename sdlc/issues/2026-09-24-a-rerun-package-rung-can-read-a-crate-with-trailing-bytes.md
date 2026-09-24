# A rerun package rung can read a crate with trailing bytes

Status: open. Found 2026-09-24 by the 0102 builder.

`sdlc/scripts/package` runs `cargo package` and then unpacks `target/package/thinkthen-0.0.1.crate` with `tar -xzf`. On a second `lint` run in the same worktree, the crate held 10 bytes after the end of its gzip stream. `gzip` warned "decompression OK, trailing garbage ignored", `tar` exited 2, and `lint` failed. The earlier crate was 454830 bytes and the fresh one 454820 bytes. The file seems to be rewritten in place without truncation when the new crate is shorter. Deleting the crate and rerunning `lint` passed.

Fix: remove `target/package/thinkthen-*.crate` in `sdlc/scripts/package` before `cargo package`, and add a check that runs the rung twice. Owner: unassigned.
