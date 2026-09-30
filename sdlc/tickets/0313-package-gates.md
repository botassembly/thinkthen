# 0313: The four remaining package gates pass from a clean checkout

Status: landed. Lane claude-3. Branch `ticket/0313-package-gates`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Issue: `2026-09-29-nine-package-gates-fail-from-clean-checkouts.md`.

## Outcome

The own gates of `libraries/python`, `libraries/polars`, `databases/duckdb` and `databases/sqlite` pass under a minimal `env -i` environment from a clean checkout of main. Each remaining failure is fixed or recorded in the issue with its exact cause.

## Evidence

- Starts from: main `ac295007c`. Experiment 302's final run at `4c0ef210` failed these four gates. Tickets 0284, 0286, 0287 and 0298 later fixed each original failing clause with focused proof. No whole-gate run of the four followed.
- Keeps: every gate's checks and planted failures, ratchets equal, `policy.py` clean. No edit under `crates/thinkthen/src/core/adapters`, `core/batch*` or `conformance/`, which ADR 0111 slice 1 is rewriting.
- Changes: stale harness and test expectations in SQLite, DuckDB and Python; DuckDB `thinkthen_details` takes a choose, score or tag question again; one Python refusal sentence; one Python format fix.
- Proof: one whole-gate run per package under `env -i` with `CARGO_BUILD_JOBS=2`, before and after.
- Defers: Python's pandas 2 lane needs pandas 2.3.3 in uv's cache, which takes network once. Environment enforcement for the other families, shared-counter consolidation and the release checkpoint stay in the issue. No failure came from wire bytes.

## Build result

Each gate ran from lane `claude-3` under `env -i` with `PATH`, `HOME`, `XDG_RUNTIME_DIR`, `TMPDIR`, `CARGO_BUILD_JOBS=2` and the routine conformance IDs. Python and DuckDB received the loopback port from `backend_start`, as the surfaces rung does.

| Package | Before, main `ac295007c` | After |
| --- | --- | --- |
| databases/sqlite | FAIL: the guard count wants one `catch_unwind`; 0306 moved it to `thinkthen::contained` | pass; conformance 30 pass, 1 not run |
| libraries/polars | pass | pass |
| databases/duckdb | FAIL: `deny.toml` lags the root copy after 0307 | pass |
| libraries/python | FAIL: `cargo fmt --check` in `src/frame.rs` | not run (77) at the pandas 2 lane; everything before it passes: 108 tests, shared cases, examples, release wheel |

Each fix exposed the next failure. The full list:

- SQLite `check.sh`: the guard step counts no `catch_unwind` and one `contained(` call.
- DuckDB `deny.toml`: deleted after review; `check.sh` already reads the root file. The R5-25 copy check and the policy table's DuckDB entry and Zlib plant went with it.
- DuckDB `cpp/verify_interrupt.py`: dropped the held `thinkthen_warm` case. 0286 made warm a no-send refusal.
- DuckDB `thinkthen_details`: 0286 routed it through the decide-only parser, so a choose, score or tag question refused with `the question has another kind`. ADR 0105 keeps the details contract. The details validator and group now take the question's own kind, as `try_details` already did. `verbs_complete.py::complete_question_files_keep_identity` is the regression.
- DuckDB `verbs_complete.py`: the refused-file fixture moves the marker from a key to a value. The shared question-file contract names a refused key; the value stays unsaid.
- DuckDB `databases_suite.py`: one expectation gains ADR 0105's `(retryable: no)` suffix.
- Python `src/frame.rs`: `_arrow_probe` reformatted within its ratchet.
- Python `thinkthen/judge.py`: the frame refusal says `, or annotate with on=` like its two siblings.
- Python deadline: review kept ADR 0041. An explicit `deadline_ms=None` is refused before a send on every verb and on an applied judge; omission maps to -1. The range refusal now uses the engine's sentence. `test_surface.py` expects the shared `settings repeats` refusal for levels beside a built question.

Review fixes also deleted DuckDB's unreachable `cpp/src/listed_complete.{cpp,hpp}` and the two bridge entries it called, keeping the listed codec `complete_listed::run`. `verbs_complete.py` gained an unknown-key file whose refusal names ``line\nbreak\u0007`` with JSON escapes on one line.

Ratchets after merging main: DuckDB Rust 4169, C++ 1737, Python tools 3902, all lowered; Python tests rise from 4568 to 4571 for the None refusal helper. Final head: SQLite, Polars and DuckDB gates pass; Python passes up to the pandas 2 lane (77). `sdlc/scripts/test` passed 1,266 tests; workspace Clippy with `-D warnings` on all targets, `policy.py` and `sdlc/scripts/tickets` pass. `policy.py` and `sdlc/scripts/tickets` pass.

## What the build taught us

- Focused selections in 0284, 0286, 0287 and 0298 left whole gates failing on neighbouring steps. Each ticket that changes a package should run that package's whole gate once.
- Shared-code tickets (0306 guard, 0307 deny file) broke package steps that copy or count shared facts.
- The Python gate depends on a pandas 2 wheel in uv's cache. It exits 77 without it, so a clean machine needs one networked `uv pip install`.
