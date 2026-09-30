# 0313: The four remaining package gates pass from a clean checkout

Status: building. Lane claude-3. Branch `ticket/0313-package-gates`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Issue: `2026-09-29-nine-package-gates-fail-from-clean-checkouts.md`.

## Outcome

The own gates of `libraries/python`, `libraries/polars`, `databases/duckdb` and `databases/sqlite` pass under a minimal `env -i` environment from a clean checkout of main. Each remaining failure is fixed or recorded in the issue with its exact cause.

## Evidence

- Starts from: main `ac295007c`. Experiment 302's final run at `4c0ef210` failed these four gates. Tickets 0284, 0286, 0287 and 0298 later fixed each original failing clause with focused proof. No whole-gate run of the four followed.
- Keeps: every gate's checks and planted failures, ratchets equal, `policy.py` clean. No edit under `crates/thinkthen/src/core/adapters`, `core/batch*` or `conformance/`, which ADR 0111 slice 1 is rewriting.
- Changes: stale tests and harness steps that each gate run shows failing.
- Proof: one whole-gate run per package under `env -i` with `CARGO_BUILD_JOBS=2`, before and after.
- Defers: failures caused by wire bytes go to ADR 0111 slice 1. Environment enforcement for the other families, shared-counter consolidation and the release checkpoint stay in the issue.
