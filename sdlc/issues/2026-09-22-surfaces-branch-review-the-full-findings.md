# Surfaces branch review: the full findings

Date: 2026-09-22. Source: an external code review of branch `surfaces` at `a42518a`, six reviewers, worst findings reproduced offline against the built libraries. This issue is the durable record; fixes land on `surfaces` and reference this file. The prior status claim "no open problems on our side" was inaccurate: the gates stayed green because suites used single rows or distinct texts, one driver cannot fail, and skipped suites still ended in "all landed checks green."

## Group 1 — Wrong answers returned with no error

1. DuckDB decide, probability, score put answers on wrong rows when a query has a repeated text or a NULL (`databases/duckdb/src/lib.rs:362-377`). One answer per distinct text; answer k written to row k. 10,000 rows alternating two texts returned 5 true and 9,995 false; correct is 5,000 each.
2. DuckDB conformance driver can never fail: prints FAILED, always exits 0, and compares `want in got` (`databases/duckdb/tools/conformance.py:41-46,186`).
3. Python annotate on a table arriving in several pieces returns the first piece again; copied rows point at freed memory (`libraries/python/src/arrow.rs:805,1095`). 2+3 rows came back [1,2,1,2].
4. Python annotate(on=) damages caller columns: categories become codes, structs empty, lists crash Polars.
5. R tt_annotate types each column from the first row (`libraries/r/thinkthen/R/thinkthen.R:276-300`); first answer "unsure" makes every later choice NA; tag columns keep only the first label.
6. A pandas Series runs the whole paid batch, then returns a one-row Series holding a wrapper object.
7. The stand-in fakes a failure for one fixed text (`standin/src/lib.rs:489`) — "order 4471: charged twice, please refund" always fails, on the live path, in every surface. A test fixture sits in product code.
8. The stand-in ignores address and width a host passes (`standin/src/lib.rs:130`); only environment variables take effect.

## Group 2 — Crashes and memory errors in the host

- C error message freed while another thread reads it (`libraries/c/src/lib.rs:125-138,211-225`); ASan-confirmed. Header promises "any number of threads" and "message lives until the next call" — both cannot hold.
- Panics cross into host processes: no C-facing function catches a panic; panic=unwind comment does not protect; DuckDB callbacks same.
- Large deadline: crashes Node, aborts Ruby, raises an uncatchable error in Python. All four surfaces copy one unchecked `Duration::from_secs_f64`; the contract should own a checked conversion.
- Python frames make pyarrow abort: release callbacks never clear their pointer (Arrow C interface requirement).
- R Ctrl-C jumps across Rust frames (`libraries/r/thinkthen/src/rust/src/lib.rs:47-49`); undefined behavior window.
- SQLite keeps one global database handle: two connections → interrupt can read a closed handle; possible second SQLite copy entry.

## Group 3 — Security

- DuckDB reads question files even with `enable_external_access=false` (`@/etc/passwd` was read).
- DuckDB relate runs on a different connection held in one global: cannot see temp tables or the caller's transaction; with two databases loaded it queried the wrong one.
- SQLite functions are marked safe to run from untrusted files: a view inside a downloaded database can make paid calls and read local files. Needs SQLITE_DIRECTONLY.
- PostgreSQL: any role can read server files through `@path` and make paid calls; nothing revoked from PUBLIC.
- PostgreSQL batches: a bad question reports as "defect: the batch thread stopped"; the real message is lost.
- Good as found: the PostgreSQL key setting is locked down; no surface prints or logs the key.

## Group 4 — Cancel and timeouts

- Ruby: Ctrl-C and Thread#raise cannot stop any public call; lock released with no wake-up.
- DuckDB: one Ctrl-C makes every later call in that process return "cancelled" for good; DuckDB's interrupt never reaches the engine.
- PostgreSQL single-row calls cannot be cancelled; pg_cancel_backend and statement_timeout wait for the network call (deadline is the actual tool; document and enforce).
- Stand-in retry sleeps up to 60 s ignore cancel and deadline.
- Python: cancel only via main-thread Ctrl-C; single calls not interruptible; Polars score column restarts the deadline per row.
- Node: each call leaks one abort listener; a shared shutdown signal leaks one per call.

## Group 5 — Merge readiness

- Main moved: 189 commits ahead, thinkthen-core folded into crates/thinkthen, policy script allows only that crate; nothing on main implements contract/. Trial merge conflicts in two issue files only.
- "Change one dependency line per surface" is wrong as coded: surfaces name `thinkthen_standin::BlockingEngine` directly; the contract trait has no constructor. Fix: a connector/factory owned by the contract.
- None of the new code is checked by main's gate (lint, size ratchet, cargo deny, CI) across 11 Rust workspaces (~18,800 lines); only 5 workspaces carry strict lints; three toolchain versions in use.
- Gate depends on things outside the repo: the wire stub in experiments/205; a private mktg repo deck (DuckDB check fails without it); unpinned npm/uv installs; floating Docker tags; a /root/.rustup path. Skipped wire suites still print "all landed checks green."

## Group 6 — Hygiene, layout, packaging

- The branch names the private mktg repo 56 times including shipped READMEs and code comments; ~10 committed files contain /home/ian paths. This repo goes public.
- Duplication: ~13,100 lines of hand glue; error-kind names spelled five places; nine conformance runners with per-runner hardcoded skip lists; the contract carries a weaker question-set parser than core's strict one (libraries accept files the CLI rejects; annotate column order differs).
- The conformance file proves less than claimed: 45 of 74 cases are recognize/relate lookups; score and tag have one case each; the checker ignores its file argument and fails from the repo root.
- Tests live in four places; nine working-notes files at the repo root (rules say sdlc/); DuckDB vendors 30 build-tool files with no license or upstream version, 5 used.
- Packaging: Ruby gem has no platform and is "UNLICENSED"; R has no LICENSE and points five folders up; Python and Node lack license fields; the stand-in's recordings compile into every package.

## What the review found solid

The blocking engine's shape; DuckDB choose/tag/details/annotate handle repeated texts and NULLs; Python zero-copy reads genuinely borrow caller memory; Node runs calls off-thread with a real AbortSignal; PostgreSQL key handling and error mapping; no committed binaries; one version file.

## Fix order (underway on `surfaces`)

1. Wrong answers first (DuckDB row mapping; Python annotate multi-piece and on=; R typing; stand-in fake failure and ignored arguments; multi-row repeated-text/NULL tests everywhere; DuckDB driver able to fail).
2. Crashes (per-thread C error slot; panic catch at every boundary; one checked deadline conversion in the contract; Arrow release pointers; R interrupt guard).
3. Security (@file respecting the access switch; caller's connection for relate; SQLITE_DIRECTONLY; REVOKE from PUBLIC; real batch errors).
4. Cancel (Ruby wake-up via the engine token; DuckDB poison clearing; PG single-row story = deadline, documented; retry sleeps honoring cancel; Node listener removal; Polars single deadline).
5. Merge prep (connector through the contract; lint/ratchet/deny coverage plan; wire stub in-repo; private names and home paths removed plus a check; Mac-capable scripts; notes under sdlc/; license fields; vendored tools trimmed and recorded).

Recommendation recorded for the build team: extend main's crate policy at merge rather than restructuring eleven workspaces now; the two-file trial-merge conflict supports the smaller step.

## Progress: waves 1 and 2 landed (2026-09-22)

As of branch `surfaces` tip `6a829b8`. Wave 1 is the first-review fix program (phases 1-2); wave 2 is the conformance honesty phase (phase 3). Wave 3, the second-review fixes, has not landed. Remaining items are tracked in `2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md`.

**Fixed in wave 1**, each with the commit that carries the fix and its discriminating test:

- Group 1: DuckDB row mapping `2b43c13` (before: `FAILED decide alternating 10,000 rows split exactly: want '5000|5000', got '1|9'`); the DuckDB driver able to fail `7337248` (selftest corrupts one expectation, exit 1); Python multi-piece annotate and `on=` columns `d9f7a55` (before: 2+3 rows returned [1,2,1,2]); R column typing by kind `b11e143` (baseline: six FAILs including the unsure-first choose and the tag column); the pandas Series `d9f7a55` (before: one-row wrapper); the stand-in fixture off by default `7acb3da` (env-arm in shipped binaries still open); the stand-in honoring address and width `526ec11` (`EngineConfig` first, environment second).
- Group 2: C per-thread error slots and panic containment `be8374f` (ASan use-after-free before, clean after); DuckDB callback panics `0345a42`; the checked deadline conversion in the contract `02e6370`, adopted at `be8374f`, `b11e143`, `d9f7a55`, `cbb0224`, `e60dbbf`, `9eb3cae`; Arrow release pointers `d9f7a55`; the R interrupt guard `b11e143` (record 0069); SQLite per-call handle `87bb8ac`.
- Group 3: DuckDB `@file` access switch and relate on the caller's database `0345a42` (ADR 0038; the `thinkthen_relations` hole and further relate-routing defects remain); SQLite direct-only `87bb8ac` (the CHECK-constraint hole on 3.45.1 remains); PostgreSQL PUBLIC revoke `9eb3cae`, `f73a67d` (grant-documentation and update-path gaps remain); PostgreSQL batch errors `9eb3cae`.
- Group 4: Ruby interruptible calls `cbb0224` (spurious-wakeup and shared-token defects remain); DuckDB one-shot token re-arm `0345a42` (the 250 ms window remains); PostgreSQL deadline `9eb3cae` (record 0070); stand-in retry sleeps `526ec11`; Python Polars single deadline `d9f7a55` (no cancel token and single-call interrupt remain); Node abort listeners `e60dbbf` (2,000 calls leave zero listeners).
- Group 5: the connector through the contract `02e6370`, `526ec11`, adopted per surface in wave 1 (C, Rust, and DuckDB's `request_digest` remain); the private deck out of the DuckDB check `537e310`; the rest of group 5 remains, with the merge-build fact recorded in `2026-09-22-merge-trial-the-build-break-list.md`.
- Group 6: remains (phase 4).

**Fixed in wave 2 (conformance honesty):** `50c6327` the checker honors its file argument and runs from any directory; `c83add5` one skip table in the conformance file replaces the runners' own lists; `bcaa5f7` score and tag coverage (score 1→3, tag 1→4 cases); `2143d7e` the shared repeated-text, NULL-row, and annotate cases with the driver shapes; `6a829b8` the gate counts skips beside greens and a bare `cargo test` in `libraries/rust` passes. The conformance file is 84 cases.

Durable records for the rulings this wave made: ADR 0037 (the C door) and ADR 0038 (the relate boundary) at `79a7efd`; records 0069 (the R interrupt window), 0070 (the PostgreSQL deadline), and 0071 (the two granted exceptions) at `aaccc63`.

## Final verification (2026-09-22, branch tip `ed5a819`)

The full gate ran with the stub up on all ten ports (8211–8219, 8231) and no key in the environment: `bash scripts/check_surfaces.sh` exit 0, `all landed checks green: green=927 skipped=73 diverged=18 failed=0 (wire: stub up)`. The offline checks pass on their own (generated lists; public names; `OK: 84 cases validated`; the checker's and the name-check's own tests). Every finding below was re-run as a command.

| Finding | Status | Proof (command → observed) |
| --- | --- | --- |
| 1.1 DuckDB row mapping | fixed | gate `ok decide alternating 10,000 rows split exactly`; a 10,000-row mix of two texts and NULLs against the built extension → true=3334, false=3333, NULL=3333, each group its own count |
| 1.2 driver can fail | fixed | `ok the corrupted answer expectation fails the driver (exit 1)` and its null-row and annotate twins; the driver reads its argument |
| 1.3 Python multi-piece annotate | fixed | `tests/test_review2_findings.py::test_a_sliced_struct_stream_answers_the_callers_rows PASSED`; surface tests 27 passed |
| 1.4 Python `on=` columns | fixed | gate python block: category, struct, list, and tz round-trips green |
| 1.5 R first-row typing | fixed | `r surface: null suite` 56 checks passed, including the unsure-first choose and the multi-label tag |
| 1.6 pandas Series | fixed | `test_a_refused_pandas_frame_makes_no_request`, `test_a_refused_pyarrow_table_makes_no_request` PASSED; both release tests PASSED |
| 1.7 stand-in fixture in product code | fixed | the fixture is compile-time only (`synthetic-partial`); the shipped shape cannot be armed by an environment variable |
| 1.8 stand-in ignored address/width | fixed | `cargo test --test settings_state` 2 passed: `a_narrow_engine_keeps_its_width_beside_a_wide_one`, `the_file_descriptors_stay_flat` |
| 2 crashes and memory | fixed | C: ASan clean, per-thread error slots, panic guards on all 19 symbols; one checked deadline conversion in the contract, adopted by every shim; Arrow release pointers cleared (pyarrow and Polars no longer abort); R interrupt guard; SQLite per-call handle |
| 3 security | fixed | DuckDB `@file` and `thinkthen_relations` respect the access switch (plus `allowed_directories`, `disabled_filesystems`, and prepared statements re-checking at execution); relate is read-only and single-statement; SQLite refuses every schema object (CHECK, DEFAULT, view, trigger, generated column, index expression; trusted on and off); PostgreSQL requires `@`, revokes PUBLIC on every function, and re-revokes on the update path |
| 4 cancel | fixed | Ruby hears token and raise (gate proof); DuckDB re-arms after one interrupt and DuckDB's own interrupt reaches the engine; PostgreSQL's deadline physics documented (FINDINGS correction); the stand-in's waits stop on cancel and deadline |
| 5 merge readiness | prepared | the connector is the contract's, one import per surface (grep proof in HANDOFF.md); the trial merge run as a build with the break list and the retarget decision points in MERGE-NOTE §8 |
| 6 hygiene | partial | the documents are corrected; **open**: the private-name and home-path sweep did not run (31 files name the deck's repository, 7 carry absolute home paths, no check enforces either) — owner: the merge ticket |

Beyond the reviews, this verification found and fixed one more blocker: the Ruby wire interrupt proof segfaulted 3 of 3 runs against the pre-fix build (eight `ttb-worker` threads faulting in `Error::guard` over freed memory after the call had returned); the poll now drains the interrupt on every path and the same proof passes 4 of 4 (`3238ed0`; `libraries/ruby/NOTES.md`, "the leaked workers").

> Closure note: the phase-4 hygiene group this appendix listed as open was fixed on 2026-09-23 in `37240fd`; see the closure section appended to the second review issue.
