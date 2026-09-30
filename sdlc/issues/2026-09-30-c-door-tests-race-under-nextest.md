# The C door tests race when each runs in its own process

Status: open. Found in the 0322 slice 2 code review. Owner: ticket 0346, batch B3 of `../planning/issue-priorities-2026-09-30.md`.

Kind: debt

Pay when: `libraries/c/check.sh` or `sdlc/scripts/test` runs the door tests under nextest.

Debt: 025

Severity: low

`archive()` in `libraries/c/tests/door/main.rs:48` lays out one fixed `scratch("archive")` folder, and `compile()` writes one fixed driver binary. `cargo test` runs every door test in one process behind a `OnceLock`, so `check.sh` passes. Under nextest each test is its own process, and two door tests failed 4 of 4 runs with "the soname link: File exists" at `main.rs:87`. Keeping it blocks a move of the door tests to nextest. Builder lesson 5 names the fix: build to a new name and rename.
