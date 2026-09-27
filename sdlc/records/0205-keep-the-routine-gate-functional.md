# 0205 build record: keep the routine gate functional

Status: design evidence only. No implementation, new test run, or acceptance is claimed. The design ticket is `sdlc/tickets/0205-keep-the-routine-gate-functional.md` at `b079ad5a` plus its pending review correction.

## Why the gate needs separation

The proximate 0202 Python failure is a statistical exposure test: at least 100 qualifying child-exit windows in 1,000 children, driven by eight concurrent children and an adaptive busy wait. Recent candidate runs exposed 44 to 61 windows per 1,000; an older run exposed 106 in 640. Every observed worker finished. The exact fixture instruction that reduced exposure is unresolved. Those numbers neither prove a freeze nor turn the plant green. The proposed 1,000-run controller was never implemented; its trial design was abandoned under Ian's newer ruling. This ticket does not resume that experiment or lower its threshold.

The systemic gate cause is unconditional mixing: `pytest` over all tests, Cargo `--all-targets`, and port `check.sh` loops run functional contracts beside shutdown statistics, 400-call timing ratios, 200-row load, and engine churn. Compilation and artifact build costs are not separated from test execution in that combined wall time. The remedy under Ian's 2026-09-27 ruling is an explicit opt-in home for intact campaigns, small public functional witnesses in ordinary checks, counted cache/replay reuse, and deletion only with a named stronger proof. The full 54-case functional corpus and load campaigns require separate opt-in commands, so requesting full functional coverage never starts stress.

## Current baseline and next evidence

The existing 0200 surfaces log reports Python 60 passed in 82.00 seconds and pandas 2 13 passed/1 skipped in 12.14 seconds. Those are historical test-process durations, not a current compile or lock measurement. A read-only preflight of per-port counts and compile-only timings is running in the retained 0148 lane. The 0205 implementation record will add exact commands, commit, cache state, tool versions, lock wait, collected/selected test counts, backend sends, and Linux before/after execution and rebuild timings. macOS and Windows are not measured on this host. The old full suite and heavy campaigns will not be rerun merely to create a baseline.
