# 0346: The C door writes relate and annotate rows from the crate's types, and each door test builds in its own folder

Status: in progress. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B3, after ticket 0345 in the same lane. Pays Debt 012, `sdlc/issues/2026-09-30-c-door-relate-rows-have-no-owner.md`, and Debt 025, `sdlc/issues/2026-09-30-c-door-tests-race-under-nextest.md`.

## Outcome

The C door's `relate` and `annotate` results serialize the crate's result types with `serde_json`, the way ticket 0314 slice 2 made the other rows. No door function joins JSON text by hand. The door's bytes do not change. Every door test can run in its own process under nextest, because each test process lays out its archive and drivers in its own folder.

## Evidence

- Starts from: `libraries/c/src/door.rs` `relate_with_facts`, which formats `{"edges":[...]}` from `Edge::to_json` strings; `libraries/c/src/call.rs` `annotate`, which joins `value_json()` rows into `[...]`; both checked on main `d8018dd96`. Ticket 0314 slice 2 (`cf314e045`) and its deferred gap for `annotate`, record arrays and `relate`. `libraries/c/tests/door/main.rs` `archive()` and `compile()`, which lay out one fixed `scratch("archive")` folder and one fixed driver path behind a `OnceLock`. Two door tests failed 4 of 4 runs under nextest with `the soname link: File exists` (0322 slice 2 review). Builder lesson 5.
- Keeps: every door byte, pinned by `libraries/c/tests/door/golden.rs`, and ticket 0344's `"either":true` edge member. `libraries/c/check.sh` and its `cargo test` run.
- Changes: `door.rs` and `call.rs` serialize a typed value for `relate` and `annotate`. Each edge, annotate row and record-array row crosses as a raw JSON value holding the line the crate already wrote, inside a serialized array or `Edges` struct, so the bytes cannot change and the public API gains nothing. Main's record arrays in `call/records.rs` still joined text, so the build took them too. `tests/door/main.rs` puts every scratch folder, the archive and the compiled drivers under `door/<pid>/`, so no process deletes or overwrites another's files, and the programs it compiles point at their own process's folder. Each process holds a lock on `door/<pid>.lock` for its life, and the first scratch call deletes the folders whose locks it can take, so one run's copies last until the next run. `cargo test` still builds once per process behind the `OnceLock`. The two debt issues move to `closed/` with the landing commit.
- Proof: `golden.rs` passes with no byte changed, and gains one relate golden with a both-ways edge and one annotate golden if none exists. `cargo nextest run` over the door tests passes 20 runs in a row at the machine's load, where main fails. `libraries/c/check.sh`, the Go, C++ and Zig checks that read the door's archive, `policy.py`, workspace clippy with `-D warnings`, `tickets`, and `lint` in a clean checkout.
- Defers: moving `libraries/c/check.sh` itself to nextest, which is the check owner's choice.

## What the build taught us
