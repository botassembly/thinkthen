# 0205 build record: keep the routine gate functional

Status: design evidence only. No implementation, new 0205 test run, or acceptance is claimed. The design ticket is `sdlc/tickets/0205-keep-the-routine-gate-functional.md` on this branch.

## Why the gate needs separation

The proximate 0202 Python failure is a statistical exposure test: at least 100 qualifying child-exit windows in 1,000 children, driven by eight concurrent children and an adaptive busy wait. Recent candidate runs exposed 44 to 61 windows per 1,000; an older run exposed 106 in 640. Every observed worker finished. The exact fixture instruction that reduced exposure is unresolved. Those numbers neither prove a freeze nor turn the plant green. In `libraries/python/tests/test_release.py`, `Slow.__del__` sleeps 0.6 seconds in each child. If all 1,000 children run, 1,000 × 0.6 ÷ 8 gives 75 seconds of ideally scheduled destructor waiting before process and backend overhead; the zero-duration focused run took 87.99 seconds. The test can stop earlier after 100 windows, so 75 seconds is conditional, not a universal minimum. The proposed 1,000-run controller was never implemented; its trial design was abandoned under Ian's newer ruling. This ticket does not resume that experiment or lower its threshold.

The systemic gate cause is unconditional mixing: `pytest` over all tests, Cargo `--all-targets`, and port `check.sh` loops run functional contracts beside shutdown statistics, 400-call timing ratios, 200-row load, and engine churn. Compilation and artifact build costs are not separated from test execution in that combined wall time. The remedy under Ian's 2026-09-27 ruling is an explicit opt-in home for intact campaigns, small public functional witnesses in ordinary checks, counted cache/replay reuse, and deletion only with a named stronger proof. The full 54-case functional corpus and load campaigns require separate opt-in commands, so requesting full functional coverage never starts stress.

## Current baseline and next evidence

The existing 0200 surfaces log at `fead31c5` reports Python 60 passed in 82.00 seconds and pandas 2 13 passed/1 skipped in 12.14 seconds. Those are historical test-process durations, not a current compile or lock measurement. A read-only preflight on clean main `ffe94254` produced `target/codex-audits/test-timing/REPORT.md` in the codex-2 worktree. It ran compile-only commands under `flock -o`, with no tests, source edits, cache clean, paid call, or stress campaign. Rust/Cargo were 1.95.0; Python 3.12.3; pinned Node 22.22.3; pinned Ruby 3.4.11; R 4.3.3; GCC 13.3.0; PostgreSQL 16.15. Existing targets were preserved. The first invocation prepared caches and is **not** a no-change baseline. Numbers below are command wall seconds inside the lock; waiting is separate.

| Linux port | Host test declarations or calls | Shared cases | First compile-only / warm no-change s | First / warm lock wait s |
| --- | ---: | --- | ---: | ---: |
| Root crates and conformance | 1,020 Rust `#[test]` | 54 main | 53.15 / 0.19 | 0.019 / 0.019 |
| Rust library | 1 Rust `#[test]` | 54 main + 8 settings | 8.88 / 0.07 | 0.103 / 0.019 |
| C | 12 Rust `#[test]` | 54 main + 8 settings | 4.63 / 0.07 | 0.020 / 0.018 |
| Python | 61 `def test_` | 54 main + 8 settings | 6.01 / 0.08 | 0.018 / 12.211 |
| TypeScript | 25 `test` or `it` | 54 main + 8 settings | 4.09 / 0.08 | 0.023 / 0.020 |
| Ruby | 38 `def test_` | 54 main + 8 settings | 4.58 / 0.09 | 0.019 / 0.019 |
| R | 91 `check` calls | 54 main + 8 settings | 10.54 / 0.07 | 0.019 / 0.020 |
| DuckDB | 78 `@case` | 54 main; host-local settings | 7.27 / 0.08 | 0.019 / 0.018 |
| SQLite | 46 `def test_` | 54 main; host-local settings | 4.88 / 0.08 | 0.021 / 0.018 |
| PostgreSQL | 10 Rust `#[test]`, 58 shell checks | 54 main; host-local settings | 15.71 / 0.12 | 0.019 / 0.018 |

These declaration counts are lexical, not executed-case totals. The compile commands test only each port's Cargo target; they exclude wheel, gem, addon, R package and SQL extension packaging. The Python warm compile waited 12.211 seconds for the lock but used only 0.08 seconds inside it. A small-source-edit recompile and current test execution time remain unmeasured. Measure them immediately before and after a real reviewed 0205 source edit, then repeat unchanged; record both source hashes and the rebuilt units. macOS and Windows are not measured on this host. The old full suite and heavy campaigns will not be rerun merely to create a baseline.
