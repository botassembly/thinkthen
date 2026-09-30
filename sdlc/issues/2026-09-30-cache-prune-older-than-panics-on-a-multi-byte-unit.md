# `cache prune --older-than` panics on a multi-byte unit

Status: open. Filed 2026-09-30 from the system grading (`sdlc/planning/grading-2026-09-30/02-question-cache.md`, item 1), checked against main `d3fd19419`. Owner: queue owner.
Kind: bug
Pay when: before 0.1.

`duration` in `crates/thinkthen/src/cli/cache.rs` splits the value with `text.split_at(text.len().saturating_sub(1))`. `str::split_at` panics when the index falls inside a character. So `thinkthen cache prune DIR --older-than 5é`, a plain typo, panics where every other bad value prints the fixed sentence "--older-than takes a positive integer and one lowercase unit". Read from the code and Rust's documented behavior; not run.

## Fix

Split with `char_indices` or `strip_suffix`, and add a non-ASCII row beside the existing `--older-than` cases in `cli/cache.rs`.

## Done when

A multi-byte last character prints the usage sentence and exits 2, and a test pins it.
