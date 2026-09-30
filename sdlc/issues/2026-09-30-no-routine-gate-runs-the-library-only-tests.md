Status: open. Found by the review of the release rehearsal build quick fix on 2026-09-30. Owner: none yet.

Kind: debt

Pay when: a second library-only failure reaches a rehearsal, or before 0.1.

Debt: 028

Severity: low

Keeping it lets a new library test that starts the command without the `cli` gate pass every routine gate and fail only in the release `crate` job.

# No routine gate runs the library-only tests

## The problem

`sdlc/scripts/package` runs `cargo test --package thinkthen --no-default-features --features bundled-sqlite --all-targets` in its own target folder, so a test that starts the command without `#[cfg(feature = "cli")]` fails there. Only the release `crate` job runs `sdlc/scripts/package`. `sdlc/scripts/lint` refuses to run it, and `sdlc/scripts/test` runs the default features, which build the command. The five tests in `sdlc/issues/closed/2026-09-30-crate-job-library-tests-start-a-command-never-built.md` escaped this way.

## A fix

Add the library-only `--test library` run, in its own target folder, to `sdlc/scripts/test`. It costs one more build of the library without the command's dependencies.
