# 0133 build: the rung allow list and three harness leftovers

Built 2026-09-26 by Claude in `worktrees/thinkthen-lane-1` on `ticket/0133-harness-leftovers`. No key was used, and no request left the machine. `THINKTHEN_API_KEY` was unset for every command.

## Outcome

Every rung but `install`'s tool checks, and every process a rung starts, now sees only the allow list in `sdlc/scripts/allow-list`. `heavy-lock` and `lint` both source it. A planted `FAKE_SERVICE_API_KEY` never reached a test binary that `lint`, `test`, `spec`, or `surfaces` started, after the code review fix. With an absolute `CARGO_TARGET_DIR` in the caller's shell, every surface passed, and nothing was built in the named folder. The interrupt handler has no failure switch, and a spawn failure is proved at the operating system's own limit. The children check's `PENDING` table is empty.

## Where the code lives

| File | Change |
| --- | --- |
| `sdlc/scripts/allow-list` | New. The 0127 loop, now unsetting every name but the allow list |
| `sdlc/scripts/heavy-lock` | Sources `allow-list` |
| `sdlc/scripts/lint` | The 0127 row became the allow-list row, which plants every allowed name. `lint` then sources `allow-list` |
| `sdlc/scripts/README.md` | A new `allow-list` row names the list |
| `sdlc/issues/2026-09-25-two-gate-failures-in-a-root-container.md` | Item 3: the new interrupt test fails as root |
| `crates/thinkthen/src/cli/interrupt.rs` | `start_with`, `readiness_fails`, and `injected_failure` are gone |
| `crates/thinkthen/src/cli/interrupt/tests.rs` | The switch rows are gone. The success path stays |
| `crates/thinkthen/tests/backend/interrupt.rs` | `a_carrier_that_cannot_spawn_is_a_defect_and_sends_nothing` |
| `crates/thinkthen/tests/relate_edge.rs`, `databases/postgresql/src/files.rs`, `databases/duckdb/tools/source_checks.py` | Each spawn builds its child's environment |
| `conformance/children/children.py` | `CARGO` names the toolchain variables beside `child_env` |
| `sdlc/scripts/children` | `PENDING` is empty. The DuckDB suite's inner spawn is `EXEMPT` |
| `crates/thinkthen/src/core/recording.rs` | Two doc lines say why `Unwritable` stays |

## Deviations from the ticket

Decision 4 kept the `lint` rung's environment whole. The first ladder run started with an absolute `CARGO_TARGET_DIR`, and `lint` failed. `sdlc/scripts/package` builds the crate package into the named folder, and `catalog.py` then looked in `target/package` and found no file. The first fix unset `CARGO_TARGET_DIR` in `lint` alone.

The code review then found that `lint` still ran `cargo` builds, tests, `clippy`, and `doc` under the caller's whole environment. The run 2 and run 3 probes showed a planted key in `lint`'s test binaries. The coordinator ruled the gap fixed now. The loop moved from `heavy-lock` into `sdlc/scripts/allow-list`. `heavy-lock` and `lint` both source it, and `lint` sources it right after the allow-list row. That replaced the two-line unset. `lint`'s other tools need only `PATH` and `HOME` from the list. Decision 4 is overturned.

The review also asked the row to plant every allowed name with a `/nonexistent` value and pin the whole sorted line. It now does, and it sorts under `LC_ALL=C` so the order holds in any locale.

The second code review found three more things, all fixed:

- `lint` sourced `allow-list` after `REPO=`, so an exported `REPO` lost its value and broke `lint` under `set -u`. Its first Python steps also saw the whole environment. `lint` now sources `allow-list` right after `set -eu`, before any step.
- A name added to `allow-list` and left unplanted passed the row. The row now also reads the names out of `allow-list` and pins them against the same line. Deriving the expected line alone would have let a planted key pass, so both lines are pinned.
- The ticket now matches the build. Decisions 2 and 4 are overturned, and the budgets carry the coordinator's re-score.

## Proof

Each plant ran once, went red, and was restored from a copy. Each restored file was touched. A grep of the diff found no plant text.

| Plant | Test | Red result |
| --- | --- | --- |
| P1: `FAKE_SERVICE_API_KEY` on the allow list | The `lint` allow-list row | `lint: a heavy rung's child saw names beyond the allow list or lost one: FAKE_SERVICE_API_KEY HOME PATH PWD THINKTHEN_DUCKDB_CLI THINKTHEN_HEAVY_LOCK THINKTHEN_HEAVY_LOCK_HELD THINKTHEN_TOOLCHAINS` |
| P2: `CARGO_TARGET_DIR` on the allow list | The same row | The same sentence with `CARGO_TARGET_DIR` first |
| P1 and P2 again after the review, in `allow-list` | The rewritten row | `lint: a rung's child did not see exactly the allow list: CARGO_HOME FAKE_SERVICE_API_KEY HOME ...`, then the same with `CARGO_TARGET_DIR` |
| P7: `R_LIBS_USER` off the allow list | The rewritten row | The whole line with `R_LIBS_USER` missing |
| P8: an unplanted `FAKE_NEW_NAME` added to the allow list | The row after the second review | `lint: allow-list names CARGO_HOME FAKE_NEW_NAME HOME ...` over the child's line without it. P1, P2, and P7 were rerun against this row and each went red too |
| P3: a spawn failure maps to `StartError::Restoration` | The spawn-failure test | `left: (Some(70), "thinkthen: defect: SIGINT routing could not be restored\n")` |
| P4: a spawn failure panics | The spawn-failure test | `left: (Some(101), "...panicked at crates/thinkthen/src/cli/interrupt.rs:194:27:\nplanted Resource temporarily unavailable (os error 11)...")`. The error is `EAGAIN` from the real `RLIMIT_NPROC` |
| P5: no `.env_clear()` in `relate_edge.rs` | `children` | `children: crates/thinkthen/tests/relate_edge.rs:14: this child inherits the whole environment; build it with the Rust child helper` |
| P6: no `env=` on `cargo tree` in `source_checks.py` | `children` | `children: databases/duckdb/tools/source_checks.py:169: this child inherits the whole environment; build it with the Python child helper` |

The row's `FAKE.SERVICE_TOKEN` plant never reached the `awk` child under `dash`, as the ticket expected.

### The planted-key ladder

Each ladder run started every rung as `env -u THINKTHEN_API_KEY FAKE_SERVICE_API_KEY=fake-not-a-key CARGO_TARGET_DIR=<scratch>/abs-target sdlc/scripts/RUNG`. A probe beside the ladder read `/proc/PID/environ` of each running lane test binary every two seconds. It counted only whether each of the two names was there, and it printed no value.

Run 4 took 173 samples. The 75 taken during `lint` held the key. The 98 taken during `test`, `spec`, and `surfaces` held neither name. After the code review fix, run 5 took 50 samples: 15 while `lint` ran and 35 while `test` and `surfaces` ran. None held the key or `CARGO_TARGET_DIR`. Run 6 took 47 samples across `lint`, `test`, and `surfaces`, and none held either name. Run 7 started `lint` alone with `REPO=/nonexistent/repo` exported beside the key and the target folder. `lint` passed, and none of its 10 samples held the key or `CARGO_TARGET_DIR`. Run 3 took 176 samples. The first 59 came while `lint` ran, and each held the key, since `lint` keeps the caller's names. The other 117 came while `test`, `spec`, and `surfaces` ran. None held the key or `CARGO_TARGET_DIR`. Run 2 stopped after `lint`, and its 59 samples all held the key. The scratch target folder held only a `.rustc_info.json` from 06:54. Run 1's `lint` wrote it before the fix. Nothing wrote there after the fix.

The relative `CARGO_TARGET_DIR` case was not run as a ladder. The rule drops the name whatever its value, and the `lint` row pins the drop.

## Rungs

The lane started cold at 29 MB, with no build folders. Before the ladder, one `cargo test --test backend interrupt` and two plant runs warmed the root `target`. That first build took 3 minutes 55 seconds of wall time, most of it waiting on the heavy lock behind other builders. The lane measured 360 MB after it. The surface build folders were still cold when the ladder started.

| Run | install | lint | test | spec | surfaces |
| --- | --- | --- | --- | --- | --- |
| 1, at `813db497` | pass, 6 s | fail, 175 s: the package catalog, see Deviation | not run | not run | not run |
| 2, at `73fcc72d` | pass, 2 s | fail, 158 s: the ratchet had to fall 21 lines | not run | not run | not run |
| 3, at `79929045` | pass, 11 s | pass, 126 s | pass, 161 s | pass, 27 s | pass, 1,041 s: all ten surfaces |
| 4, at `99bd86a9`, after merging main with 0132 landed | pass, 794 s | pass, 185 s | pass, 157 s | pass, 515 s | pass, 665 s: all ten surfaces |
| 5, at `e3c69efd`, the code review fix | not run | pass, 112 s | pass, 355 s | not run | pass, 483 s: all ten surfaces |
| 6, at `c11f7089`, after merging main with 0136 landed | not run | pass, 101 s | pass, 118 s | not run | pass, 498 s: all ten surfaces |
| 7, at `1d9ce050`, the second review fix, with an exported `REPO` | not run | pass, 107 s | not run | not run | not run |

Run 3's wall time totals 1,366 seconds, about 23 minutes. Run 4's totals 2,316 seconds, about 39 minutes. In both runs every heavy rung printed that it waited for the heavy lock behind other builders. So these times measure a busy machine, not the lane alone. Run 4's `install` spent most of its 794 seconds waiting. Run 4's `surfaces` rebuilt only what the merge changed and took 665 seconds, against 1,041 in run 3. `du -sh` of the lane reads 9.2 GB after run 3 and after run 4, and 9.3 GB after runs 5 and 6.

## Ratchet

`node sdlc/scripts/ratchet.mjs` reads `crates + conformance 66373/66373` before the merge, down 21 from 66394. After merging main with 0132's 163 lines, it reads `66536/66536`, main's 66557 less 21. The PostgreSQL Rust ceiling rose 2 to 1810, and the DuckDB Python ceiling rose 2 to 1896. Commit `79929045` says why.

## Budgets

In nonblank lines.

| Budget | Limit | Measured |
| --- | --- | --- |
| `lint` over the replaced row | 14 net, after the coordinator's re-score | 14 net: 21 added, 7 removed |
| `allow-list` and `heavy-lock` together | 16 added, after the coordinator's re-score | 16 added, 10 removed |
| `interrupt.rs` and `interrupt/tests.rs` | fewer than today | 12 added, 65 removed |
| `tests/backend/interrupt.rs` | 30 added | 27 added |
| Spawn sites, `children`, `children.py` | 18 changed | 16 added, 15 removed. Three of the added lines in `relate_edge.rs` are `rustfmt` rewrapping one call |
| `recording.rs` | 3 added | 2 added |
| Dependencies | none | none |

## Defers

- A machine whose `/bin/sh` passes an unsettable name on. The GitHub gate. This build did not run it.
- A by-hand `check.sh` run with an absolute `CARGO_TARGET_DIR`.
- Tests for the three `pthread_sigmask` failure paths.
- The spawn-failure test fails as root. Item 3 of the root-container issue records it.
- Items 1, 4, 5, 7, 8, and 9 of the harness issue.
