# Checkpoints publish no packages for 16 surfaces

Status: open until checkpoint 2's folder holds the packages; ticket 0362 makes `surfaces --publish` pack them. Asked by the release QA team on 2026-09-30 after round 1 on `checkpoint/surfaces/2026-09-30-1`. Owner: the queue owner, as batch C4 in `../planning/issue-priorities-2026-09-30.md`.
Kind: real test gap.
Milestone: 0.1

`surfaces --publish TAG` copies only the files `release-pack --reuse` packs: the command, the C library, Python, TypeScript, Ruby, SQLite, DuckDB, PostgreSQL and the first-run sample. QA tests only the published folder, so 160 language cells were skipped in round 1. `release-pack` already has parts for Go, C++, C#, the JVM, Swift, Zig, PHP, Dart, Ada, Objective-C and COBOL, but `--reuse` refuses them because they need a fresh C build. The Rust crate, Rust Polars and R have no part.

QA's order: Go, the JVM languages, C# and the Rust crate first.

Fix: let a checkpoint publish pack the source-wrapper parts after the sweep, with one fresh C build, and add `.crate` and R source parts. Touches `sdlc/scripts/surfaces`, `sdlc/scripts/release-pack` and `sdlc/scripts/publish-builds`. Done when the next checkpoint folder holds a package for each of the 16 surfaces and its `SHA256SUMS` covers them.
