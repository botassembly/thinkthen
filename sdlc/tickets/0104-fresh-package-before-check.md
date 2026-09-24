---
flow: quick-fix
priority: 104
opens: lint
---

# 0104: Fresh package before the check

Status: landed. Owner: Claude. The review accepted it (`sdlc/records/0104-review.md`).

## Outcome and authority

A rerun of `lint` fails when the new source package is shorter than the one left in `target/package`. `cargo package` writes over the old file without truncating it, and `tar` refuses the trailing bytes (`sdlc/issues/2026-09-24-a-rerun-package-rung-can-read-a-crate-with-trailing-bytes.md`). This quick fix follows the package check that tickets 0083 and 0092 shaped.

## Work

1. Plant a stale crate longer than the fresh one and show `sdlc/scripts/package` fail.
2. Remove `target/package/thinkthen-*.crate` in `sdlc/scripts/package` before `cargo package`. A fresh `CARGO_TARGET_DIR` would also work, but it rebuilds the whole dependency closure on every run. Deleting one file costs nothing.
3. Make `sdlc/scripts/lint` plant the stale crate before every package run, so the fix stays proven.

Touches only `sdlc/scripts/package`, `sdlc/scripts/lint`, the issue, this ticket, and its record.
