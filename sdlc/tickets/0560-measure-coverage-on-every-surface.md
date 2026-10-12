# 0560: Measure outside-in coverage on every surface

Status: OPEN.

Milestone: 0.2

Depends on: 0549, 0550, 0551, 0552, 0553, 0554, 0555, 0556, 0558

## Outcome

A committed release-suite script measures how much of the Rust core and the C library each public surface reaches. The surfaces are CLI commands, the public Rust API and each installed language library. The script also measures the inside unit tests separately. Each run stores a summary under `sdlc/records/coverage/`, so later runs can be compared. The first recorded run gives the baseline that the outside-in test cleanup works from.

## Evidence

- Starts from: the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md) and a read-only coverage review of the 0.2 combined branch. That review ran `cargo llvm-cov` by hand in a temporary worktree. Its commands were not kept in the repo.
- Keeps: the routine suite as it is. Coverage runs only in the release suite, under the [two-suite ruling](../decisions/2026-10-09-two-test-suites.md). Tests replay saved responses, and no paid call runs.
- Changes: these pieces.
  - Add `sdlc/scripts/coverage`. It builds the core and C library with coverage instrumentation in its own target directory, and it reports nonblank-line coverage per crate and module for these runs:
    - Rust integration tests and CLI tests only (outside-in);
    - unit tests in `src` only;
    - each installed language library's routine checks against the instrumented native library, one surface at a time. Start with C, Python and one JVM language, then add the rest.
  - Write a short Markdown summary for each run to `sdlc/records/coverage/YYYY-MM-DD-REV.md`. List the totals per surface, the 30 files with the most lines reached only by unit tests, and the 30 files with the most lines no test reaches.
  - Document the command, its run time and its tool needs in `sdlc/scripts/README.md`. A missing tool makes the script exit 77 in plain mode and fail in release mode, the same as `surfaces`.
- Proof: these checks.
  - One full run on main, after the 0549 to 0558 combined revision lands, stores its summary. 0543 reruns it on the candidate.
  - A rerun on the same revision gives the same totals.
  - A planted dead function shows as unreached, and a planted unit-only function shows as reached only by unit tests.
- Defers: deleting or moving tests. The outside-in test cleanup ticket owns that and uses this baseline.
