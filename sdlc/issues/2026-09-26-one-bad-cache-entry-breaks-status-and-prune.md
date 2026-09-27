# One bad cache entry breaks `status` and `cache prune`

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 08, finding 3. Blocks 0.1: prune is the only thing that bounds the cache. Owner: ticket 0163 on `ticket/0163-the-cache-folder-and-its-pages`, ready for review.

## What happens

One malformed or foreign-schema entry in a cache folder stops `cache prune` and `status` for the whole folder. The message names no file. A restored backup, a sync tool, a disk error, or a later ThinkThen writing `thinkthen.recording/2` into a shared default cache can leave such an entry. A nightly prune then fails every night while the cache grows.

## Checked on main

Verified by reading the code: `scan` in `crates/thinkthen/src/engine/cache_prune.rs:200` returns `Error::CacheEntry` at the first entry that is not a regular file, changes identity, fails to parse or has the wrong name. Both callers at lines 66 and 92 propagate it. `cli/status.rs:126` reaches the same module. The folder-wide failure comes from the report's run.

## What would fix it

Let prune skip bad entries, report them by digest, and still trim the valid ones. Or move them aside. Let `status` report a count of bad entries and carry on. Add a test with one foreign entry among good ones.

## Done when

A folder with one bad entry still prunes, `status` still prints, and both name the bad entry by digest.
