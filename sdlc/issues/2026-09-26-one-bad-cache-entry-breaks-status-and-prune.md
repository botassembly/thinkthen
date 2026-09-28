# One bad cache entry breaks `status` and `cache prune`

Status: closed on 2026-09-27. Ticket 0163 skips and reports bad entries, preserves valid-only prune counts and lets status report a bad-entry count. Reviewed source: `0292cba2`; proof: `sdlc/records/0163-code-review.md`.

## What happens

One malformed or foreign-schema entry in a cache folder stops `cache prune` and `status` for the whole folder. The message names no file. A restored backup, a sync tool, a disk error, or a later ThinkThen writing `thinkthen.recording/2` into a shared default cache can leave such an entry. A nightly prune then fails every night while the cache grows.

## Checked on main

Verified by reading the code: `scan` in `crates/thinkthen/src/engine/cache_prune.rs:200` returns `Error::CacheEntry` at the first entry that is not a regular file, changes identity, fails to parse or has the wrong name. Both callers at lines 66 and 92 propagate it. `cli/status.rs:126` reaches the same module. The folder-wide failure comes from the report's run.

## What would fix it

Let prune skip bad entries, report them by digest, and still trim the valid ones. Or move them aside. Let `status` report a count of bad entries and carry on. Add a test with one foreign entry among good ones.

## Done when

A folder with one bad entry still prunes, `status` still prints, and both name the bad entry by digest.
