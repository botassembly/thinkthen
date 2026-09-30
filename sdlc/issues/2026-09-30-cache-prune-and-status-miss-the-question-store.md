# `cache prune` and `status` miss the question store

Status: open. Found by reading main at `ec130d3ee`. Owner: ticket 0304 slice 5. Source: `sdlc/planning/after-slice-3-prep.md`, slice 5, item 1.

## The problem

Since 0304 slice 2 (`1fbbe08e0`), the seven record functions keep their answers in `thinkthen.sqlite` (`engine/store.rs:28`). `cache prune`, `cache unused` and `status` still read only the old digest-named files:

- `cli/cache.rs` calls `engine/cache_prune` for `unused`, `prune` and `prune --dry-run`.
- `engine/cache_prune/scan.rs:84-103` (`scan_final`) skips every name that is not a digest file, so it never sees `thinkthen.sqlite`.
- `cli/status.rs:158-205` (`cache_status`) counts entries and bytes through `cache_prune::inspect`, and `status --json` stays `thinkthen.status/1` (`cli/status.rs:134`).

So `cache prune` cannot shrink the question store, `--answered-by-other-than` cannot remove an old model's answers from it, and `status` does not count it. The model-mismatch message tells users to run that prune (`specification/result.md:248`). `specification/recording.md:46,115` describe the prune target and selection over old entries only. `find`, `recognize` and `relate` still write old entries until 0304 slice 4, so prune still trims those.

## What should happen

0304 slice 5 does what ADR 0111 says (`adr/0111…:232`): `cache prune` and `cache unused` keep their selectors as SQL over the store's `taken_at`, `answered_by` and keys, delete states no answer uses, then run `PRAGMA incremental_vacuum`. `status` counts the store and moves to `thinkthen.status/2`. ADR 0114 says whichever of slice 5 and ticket 0334 lands second adds its fields to `status/2`. The recording page and the model-mismatch sentence follow.

## Evidence

The files and lines above, read on main. No run was needed: `scan_final` filters names before it opens a file.
