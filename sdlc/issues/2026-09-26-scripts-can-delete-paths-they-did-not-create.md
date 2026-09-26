# Scripts can delete paths they did not create

Status: Open for the scripts outside the lint scan. Filed 2026-09-26 by the queue owner after `sdlc/records/2026-09-26-lane-1-deleted-by-a-cleanup-trap.md`. Quick Fix `qf-scripts-delete-only-what-they-made` fixed the scripts the lint scan reads. See `sdlc/records/qf-scripts-delete-only-what-they-made.md`.

## Problem

Nothing checks what a script's `rm -rf` may remove. Today 20 lines across `sdlc/scripts/`, `libraries/*/check.sh` and `databases/*/check.sh` run `rm -rf` on a variable. Most variables come from `mktemp`, but lint does not know that. A cleanup that receives a wrong path deletes it. On 2026-09-26 that deleted a whole lane.

## Proposed fix

Add one sourced helper beside `verdict.sh`:

- `scratch_dir` makes a folder with `mktemp -d` and records it.
- `scratch_clean` removes only recorded folders, and refuses anything else with a nonzero exit.

Add a lint check that fails on any `rm -rf` in those scripts outside the helper. Allow a named exception list for the build-output paths `lint` already removes. Plant a script that removes a non-scratch path, and show lint turns red.

## Owner

The next free lane after 0128 Phase 2. It is a Quick Fix if it stays under 60 lines.

## Remaining

`sdlc/scripts/scratch.sh` and its lint check cover `sdlc/scripts/`, `libraries/*/check.sh`, and `databases/*/check.sh`. These scripts outside that scan still run `rm -rf` on a variable or a path:

- `databases/postgresql/runtime.sh`: `$EXTRACTED.partial` and `.runtime/tree`. Its stale run folder now passes a name guard.
- `databases/sqlite/setup.sh`, `databases/sqlite/tests/host_sqlite.sh`, and `databases/duckdb/tools/selftests.sh`.
- `libraries/python/build-wheel.sh`, `libraries/r/tests/with-backend.sh`, `libraries/r/tools/make-tarball.sh`, and `libraries/ruby/setup-ruby.sh`.
- `install.sh`, `site/public/install.sh`, `sdlc/live-test`, `transforms/*/test.sh`, `probes/*`, and `demos/16-triage-pipeline/`.

The fix is to widen the scan's file list and route each line through `scratch.sh` or name it as an exception.
