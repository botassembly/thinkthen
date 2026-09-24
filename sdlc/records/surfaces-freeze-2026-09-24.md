# Surfaces freeze, 2026-09-24

Claude froze `surfaces-wave7` on 2026-09-24 so the one-surface-per-ticket ports in `sdlc/planning/surfaces-port-guide.md` on main start from a checked state. The branch is never merged into main.

## What was merged

The three open lanes were merged into `surfaces-wave7` at `f6a7faea` (tag `surfaces-wave7-final`, which stays where it is).

| Lane | Tip | Merge commit | Conflicts |
|---|---|---|---|
| `w7/gate3` | `1a3df493` | `c6bb5063` | none |
| `w7/python4` | `417433b4` | `ff57ef56` | `sdlc/surfaces-ratchet.json` |
| `w7/fast` | `d2747a54` | `40c8f705` | none |

The ratchet conflict was arithmetic. `w7/gate3` raised the ceiling from 36,510 to 36,531 and `w7/python4` raised it from 36,510 to 36,808. The merged tree measures 36,829, the sum of both raises. The merge sets the ceiling to 36,829.

Tag `surfaces-wave7-frozen-2026-09-24` points at `40c8f70563c7ed04f4972706c3427130e1d4721d`. Every surface result below was measured at that commit. A second-agent review then landed as `sdlc/records/surfaces-notes/REVIEW-wave-7-freeze.md`, with the whole review beside it in `surfaces-ceiling-review.md`. The size-ceiling check and its self-test were run again after that commit. Tag `surfaces-wave7-frozen-2026-09-24b` marks the tip that carries the review and this record. Neither commit changes code.

## How the checks ran

The one-minute load was 3.8 before the run. `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `ENGINE_BASE_URL` were unset. A `docker` shim that exits 127 led `PATH`, so no container ran. `sdlc/scripts/live` did not run. The wire suites ran against the repository's loopback stub (`tools/wire-stub`) on 127.0.0.1.

The run used a copy of `scripts/check_surfaces.sh` with four changes. A red ratchet did not stop the run. Ruby and PostgreSQL were left out of the surface loops, and their stubs on ports 8214 and 8219 were not started. Both surfaces need Docker. The copy's summary line read `green=356 skipped=41 diverged=0 failed=6 (wire: 6 of 10 wire suites ran in a passing surface, 0 in a failed surface, 2 lost)`.

SQLite failed that run for want of a SQLite amalgamation in `databases/sqlite/.runtimes`. A second run copied `sqlite-amalgamation-3500000` and the stock `sqlite3` CLI from the local `thinkthen-surfaces` worktree (no download) and ran `databases/sqlite/check.sh` alone with a stub on 8218.

## Results

"Lines" counts the gate's `ok` and `skip` result lines. Every `skip` line is a conformance case the one skip table (`conformance/skiptable.py`, 24 entries) holds back.

| Surface | Check command | Result |
|---|---|---|
| Surfaces ratchet | `node sdlc/scripts/surfaces-ratchet.mjs` | pass after the review, 36,829/36,829. At `40c8f705` it failed because nine commits lacked a review (below). |
| Ratchet self-test | `sdlc/scripts/surfaces-ratchet-self-test` | pass, 25 ok lines, before and after the review |
| Workspace lint | `sdlc/scripts/lint-workspaces` | FAIL on `libraries/ruby` only (needs Ruby or its builder container). Clippy and cargo-deny pass on the other 12 workspaces. |
| Workspace lint self-test | `sdlc/scripts/lint-workspaces-self-test` | pass |
| Contract | `cd contract && cargo test --locked` | pass, 29 tests |
| Stand-in, offline | `cd standin && cargo test --locked --lib --test ...` (every test but wire) | pass, 53 tests |
| Stand-in, wire | `cd standin && cargo test --test wire` against the stub on 8231 | pass, 3 tests and 35 ok lines |
| Offline checks | conformance file, skip table, runner honesty, checker tests, generated names, public names, private references, gate library, portable shell, locked calls, offline calls | pass, every step |
| Python | `libraries/python/check.sh` | pass: 22 Rust shim tests, 150 pytest cases, 10 examples, conformance 78 passed and 5 skipped, wire suite ran |
| TypeScript | `libraries/typescript/check.sh` | pass: 62 of 68 node tests passed, 6 skipped (the wire cases, which pass in the wire step), 71 ok lines, wire suite ran |
| R | `libraries/r/check.sh` | pass: 94 ok lines, 7 skip lines, wire suite ran |
| Rust | `libraries/rust/check.sh` | pass: 39 tests, 3 ignored, 76 ok lines, 7 skip lines, wire suite ran |
| C | `libraries/c/check.sh` | pass: 39 tests, 73 ok lines, 11 skip lines, sanitizer repro passed, wire suite ran |
| SQLite | `databases/sqlite/check.sh` (second run) | pass: 10 tests, 192 ok lines, 11 skip lines, wire suite ran |
| SQLite package | `databases/sqlite/package.sh --dry-run` | pass, `dist/thinkthen.so` staged at GLIBC 2.28 |
| PostgreSQL package | `databases/postgresql/package.sh --dry-run` | pass, tarball staged |
| Artifact paths | `scripts/check_artifact_paths.sh --built ...` | pass, 11 artifacts carry no home path |
| DuckDB | `databases/duckdb/check.sh` | not run. It needs `make configure`, which fetches a toolchain over the network. |
| Ruby | `libraries/ruby/check.sh` | not run. Every step runs in Docker. |
| PostgreSQL | `databases/postgresql/check.sh` | not run. It runs PostgreSQL 16 in Docker. |

## Failing cases

- `sdlc/scripts/surfaces-ratchet.mjs:233` (fixed by the review): at `40c8f705` these commits move the ceiling with no `Verdict: ACCEPT <sha>` record: `ff57ef565186314293ffc2c12c242642e5feac25` (the python4 merge), `b590a8994e0181c0d10c747b17b1c53f534f0faa`, `d74d95fb248fcb65b0d30ccf31c0d45002f49b9c`, `d048261edac1145775f9cdb951c5eede392df5c4`, `385afd29a72973e55b3226a3ed5942c1715ecf05`, `417433b48d6fd99d36024645976c6b70adaef243` (python4), and `ce2aef10eb1f51ac20cc88e174d4ba29992da5b2` (gate3).
- `sdlc/scripts/surfaces-ratchet.mjs:243` (fixed by the review): the script-change check did not run, because the raise check exits first. Two gate3 commits change the guarded files, and no review record names them: `35229b813ea8845f31f8796a2847470924000508` and `1a3df4931e1265cfe3fa4c6d81bc18628bcbb7a8`.
- `sdlc/scripts/lint-workspaces:234`: `libraries/ruby` failed because Ruby is not on `PATH` and Docker was blocked.
- `databases/duckdb/check.sh:36`: not set up. `databases/sqlite/check.sh:34` failed the same way in the first run and passed in the second.

## Review follow-ups

Fix each one when its surface ports to main.

- `d74d95fb` (Python): the `snapshot()` test helper sits inside the doc comment of `refusal` in `libraries/python/src/arrow.rs`. Move the helper below `refusal`.
- `d048261e` (Python): the bounds check that refuses with `UNREADABLE` is written out five times. Fold it into one helper, and give the null batch child its own refusal sentence.
- `35229b81` (gate): the ratchet stops at the first kind of waiting commit, so a pending raise hides a pending script change. Report both lists in one run, then drop the self-test's retry loop.

## Open

- DuckDB, Ruby, and PostgreSQL need a machine where `make configure` and Docker are allowed. Each port ticket re-runs its surface against the real engine anyway.
- The `surfaces` branch still points at `398d7bb2`. Fast-forwarding it waits for Ian.
