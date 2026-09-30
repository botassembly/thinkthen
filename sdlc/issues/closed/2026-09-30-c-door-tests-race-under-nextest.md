# The C door tests race when each runs in its own process

Status: Closed by ticket 0346, landed as `Land 0346: the C door joins no JSON text, and each door test process keeps its own folder`. Resolution: each door test process keeps its scratch folders, archive and drivers under `door/<pid>/` and holds `door/<pid>.lock` for its life; the first scratch call deletes the folders whose locks it can take. `cargo nextest run --test door` passed 20 runs in a row on the final code, where main failed.

Kind: debt

Pay when: `libraries/c/check.sh` or `sdlc/scripts/test` runs the door tests under nextest.

Debt: 025

Severity: low

Paid: 2026-09-30

`archive()` in `libraries/c/tests/door/main.rs:48` lays out one fixed `scratch("archive")` folder, and `compile()` writes one fixed driver binary. `cargo test` runs every door test in one process behind a `OnceLock`, so `check.sh` passes. Under nextest each test is its own process, and two door tests failed 4 of 4 runs with "the soname link: File exists" at `main.rs:87`. Keeping it blocks a move of the door tests to nextest. Builder lesson 5 names the fix: build to a new name and rename.
