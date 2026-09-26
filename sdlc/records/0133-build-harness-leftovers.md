# 0133 build: the rung allow list and three harness leftovers

Built 2026-09-26 by Claude in `worktrees/thinkthen-lane-1` on `ticket/0133-harness-leftovers`. No key was used, and no request left the machine. `THINKTHEN_API_KEY` was unset for every command.

## Outcome

A heavy rung and every process it starts now see only the allow list in `sdlc/scripts/heavy-lock`. A planted `FAKE_SERVICE_API_KEY` never reached a test binary that `test`, `spec`, or `surfaces` started. With an absolute `CARGO_TARGET_DIR` in the caller's shell, every surface passed, and nothing was built in the named folder. The interrupt handler has no failure switch, and a spawn failure is proved at the operating system's own limit. The children check's `PENDING` table is empty.

## Where the code lives

| File | Change |
| --- | --- |
| `sdlc/scripts/heavy-lock` | The 0127 loop now unsets every name but the allow list |
| `sdlc/scripts/lint` | The 0127 row became the allow-list row. `lint` also unsets `CARGO_TARGET_DIR` |
| `sdlc/scripts/README.md` | The `heavy-lock` row names the list |
| `crates/thinkthen/src/cli/interrupt.rs` | `start_with`, `readiness_fails`, and `injected_failure` are gone |
| `crates/thinkthen/src/cli/interrupt/tests.rs` | The switch rows are gone. The success path stays |
| `crates/thinkthen/tests/backend/interrupt.rs` | `a_carrier_that_cannot_spawn_is_a_defect_and_sends_nothing` |
| `crates/thinkthen/tests/relate_edge.rs`, `databases/postgresql/src/files.rs`, `databases/duckdb/tools/source_checks.py` | Each spawn builds its child's environment |
| `conformance/children/children.py` | `CARGO` names the toolchain variables beside `child_env` |
| `sdlc/scripts/children` | `PENDING` is empty. The DuckDB suite's inner spawn is `EXEMPT` |
| `crates/thinkthen/src/core/recording.rs` | Two doc lines say why `Unwritable` stays |

## Deviation from the ticket

Decision 4 kept the `lint` rung's environment whole. The first ladder run started with an absolute `CARGO_TARGET_DIR`, and `lint` failed. `sdlc/scripts/package` builds the crate package into the named folder, and `catalog.py` then looked in `target/package` and found no file. `lint` now unsets `CARGO_TARGET_DIR` in two lines, as the heavy rungs drop it. `lint` still keeps every other name. The probe below shows that `lint`'s test binaries still see a planted key. That gap stays deferred.

## Proof

Each plant ran once, went red, and was restored from a copy. Each restored file was touched. A grep of the diff found no plant text.

| Plant | Test | Red result |
| --- | --- | --- |
| P1: `FAKE_SERVICE_API_KEY` on the allow list | The `lint` allow-list row | `lint: a heavy rung's child saw names beyond the allow list or lost one: FAKE_SERVICE_API_KEY HOME PATH PWD THINKTHEN_DUCKDB_CLI THINKTHEN_HEAVY_LOCK THINKTHEN_HEAVY_LOCK_HELD THINKTHEN_TOOLCHAINS` |
| P2: `CARGO_TARGET_DIR` on the allow list | The same row | The same sentence with `CARGO_TARGET_DIR` first |
| P3: a spawn failure maps to `StartError::Restoration` | The spawn-failure test | `left: (Some(70), "thinkthen: defect: SIGINT routing could not be restored\n")` |
| P4: a spawn failure panics | The spawn-failure test | `left: (Some(101), "...panicked at crates/thinkthen/src/cli/interrupt.rs:194:27:\nplanted Resource temporarily unavailable (os error 11)...")`. The error is `EAGAIN` from the real `RLIMIT_NPROC` |
| P5: no `.env_clear()` in `relate_edge.rs` | `children` | `children: crates/thinkthen/tests/relate_edge.rs:14: this child inherits the whole environment; build it with the Rust child helper` |
| P6: no `env=` on `cargo tree` in `source_checks.py` | `children` | `children: databases/duckdb/tools/source_checks.py:169: this child inherits the whole environment; build it with the Python child helper` |

The row's `FAKE.SERVICE_TOKEN` plant never reached the `awk` child under `dash`, as the ticket expected.

### The planted-key ladder

Each ladder run started every rung as `env -u THINKTHEN_API_KEY FAKE_SERVICE_API_KEY=fake-not-a-key CARGO_TARGET_DIR=<scratch>/abs-target sdlc/scripts/RUNG`. A probe beside the ladder read `/proc/PID/environ` of each running lane test binary every two seconds. It counted only whether each of the two names was there, and it printed no value.

Run 3 took 176 samples. The first 59 came while `lint` ran, and each held the key, since `lint` keeps the caller's names. The other 117 came while `test`, `spec`, and `surfaces` ran. None held the key or `CARGO_TARGET_DIR`. Run 2 stopped after `lint`, and its 59 samples all held the key. The scratch target folder held only a `.rustc_info.json` from 06:54. Run 1's `lint` wrote it before the fix. Nothing wrote there after the fix.

The relative `CARGO_TARGET_DIR` case was not run as a ladder. The rule drops the name whatever its value, and the `lint` row pins the drop.

## Rungs

The lane started cold at 29 MB, with no build folders. Before the ladder, one `cargo test --test backend interrupt` and two plant runs warmed the root `target`. That first build took 3 minutes 55 seconds of wall time, most of it waiting on the heavy lock behind other builders. The lane measured 360 MB after it. The surface build folders were still cold when the ladder started.

| Run | install | lint | test | spec | surfaces |
| --- | --- | --- | --- | --- | --- |
| 1, at `813db497` | pass, 6 s | fail, 175 s: the package catalog, see Deviation | not run | not run | not run |
| 2, at `73fcc72d` | pass, 2 s | fail, 158 s: the ratchet had to fall 21 lines | not run | not run | not run |
| 3, at `79929045` | pass, 11 s | pass, 126 s | pass, 161 s | pass, 27 s | pass, 1,041 s: all ten surfaces |

Run 3's wall time totals 1,366 seconds, about 23 minutes. Its times include waits for the heavy lock behind other builders. The `surfaces` rung printed that it waited. `du -sh` of the lane after run 3 reads 9.2 GB.

## Ratchet

`node sdlc/scripts/ratchet.mjs` reads `crates + conformance 66373/66373`, down 21 from 66394. The PostgreSQL Rust ceiling rose 2 to 1810, and the DuckDB Python ceiling rose 2 to 1896. Commit `79929045` says why.

## Budgets

In nonblank lines.

| Budget | Limit | Measured |
| --- | --- | --- |
| `heavy-lock` | 10 added | 9 added, 6 removed |
| `lint` over the replaced row | 6 added | 4 net: 11 added, 7 removed |
| `interrupt.rs` and `interrupt/tests.rs` | fewer than today | 12 added, 65 removed |
| `tests/backend/interrupt.rs` | 30 added | 27 added |
| Spawn sites, `children`, `children.py` | 18 changed | 16 added, 15 removed. Three of the added lines in `relate_edge.rs` are `rustfmt` rewrapping one call |
| `recording.rs` | 3 added | 2 added |
| Dependencies | none | none |

## Defers

- `lint`'s test binaries still inherit the caller's names, the planted key included. `lint` runs `cargo test` from `sdlc/scripts/package`.
- A machine whose `/bin/sh` passes an unsettable name on. The GitHub gate. This build did not run it.
- A by-hand `check.sh` run with an absolute `CARGO_TARGET_DIR`.
- Tests for the three `pthread_sigmask` failure paths.
- The spawn-failure test fails as root. The landing adds it to the root-container issue.
- Items 1, 4, 5, 7, 8, and 9 of the harness issue.
