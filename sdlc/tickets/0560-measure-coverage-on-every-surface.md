# 0560: Measure outside-in coverage on every surface

Status: OPEN.

Milestone: 0.2

Depends on: 0549, 0550, 0551, 0552, 0553, 0554, 0555, 0556, 0558

Reviews: revision 41499bf, accept

Reviews: revision 8b96113, reporting repair accepted; coverage baseline accepted by PM amendment; ticket remains open until candidate.

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

PM amendment, 2026-10-12. Accept the 2026-10-12 installed pair at da1e3bb17 as the 0.2 baseline. Do not make coverage reproducible. The two runs differ by at most six lines in any collection, from real scheduling paths, and line numbers shift with every deletion anyway, so later comparisons use counts. The candidate passes the floor when each collection's reached count is at least the lower of the two baseline counts, minus the lines the waves deleted. Objective-C joins at the candidate on macOS. Proceed with the waves now, test deletion included. A developer deletes a test after naming the outside test that covers its claim, as the outside-in skill says. Commit only the Markdown summary of a coverage run. Keep the raw per-line JSON as build output outside Git, and remove the two installed-pair JSON files from the branch. Run no further full coverage measurement before the candidate.

## Progress

- 2026-10-12 started
- 2026-10-12 next: The native tooling slice retains feature-selection, profile-write and launcher failures as diagnostics; finish Cargo launch metadata and source-unit classification, validate the frozen slice, then add installed C, Python and JVM collections before the full main baseline.
- 2026-10-12 next: Finish the owned native validation at 307296e1d; fresh source review found incomplete signal-terminated profile reporting and timeout descendant cleanup, then installed collections and the baseline remain before deletion waves.
- 2026-10-12 next: Native collection, routine test and lint passed at 307296e1d and all owned jobs exited; fix signal-profile reporting and timeout descendant cleanup, remove transient record pointers, then add installed collections for the full baseline before deletion waves.
- 2026-10-12 next: Profile lifecycle regressions pass and durable-report fixes are integrated on ticket/0543-final; installed C/Python and JVM collectors are developing in separate files before remaining library collections and the complete baseline.
- 2026-10-12 next: C/Python and JVM artifact collections passed; remaining collectors and shared privacy-preserving source remaps are developing, and Ian is clarifying the Linux-before-cleanup baseline versus candidate-only Apple coverage ordering.
- 2026-10-12 next: C/Python and JVM collectors passed; Go now produces valid continuous profiles, C++/Zig need native compiler path remaps, extension and C source-unit checks are owned and running, and Apple baseline ordering remains with Ian.
- 2026-10-12 next: Bounded fresh review accepted native, C/Python and C-host tooling and four measured fixture-limit increases; R lifecycle checks passed, Rust/Polars and remaining managed collectors are running, then the combined baseline and repeat remain.
- 2026-10-12 next: All installed collectors have bounded source review and available-platform proofs; repaired tooling at bec7a013b is running routine test and lint, then the combined baseline, repeat and planted-code proof remain, with Apple baseline ordering awaiting Ian.
- 2026-10-12 next: The repaired routine test and real-code classification probe passed; reviewed fixture limits and Apple collectors are integrated, partial-report durability is being repaired, and the locally runnable baseline will guide cleanup with Objective-C explicitly unmeasured until the approved candidate Apple checks.
- 2026-10-12 landed 1a84f9b6c; next: Coverage tooling is on main with passed routine and lint evidence plus twelve integrated lifecycle checks; run the locally runnable baseline and repeat on one frozen revision, retaining the Objective-C omission and failing status before cleanup waves.
