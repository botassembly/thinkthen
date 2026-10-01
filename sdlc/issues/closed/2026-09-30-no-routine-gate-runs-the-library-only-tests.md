Status: Closed by the quick fix landed as `Land quick fix: pre-0.1 small fixes`. Found by the review of the release rehearsal build quick fix on 2026-09-30. Owner: queue owner. Resolution: `sdlc/scripts/test` builds the library-only tests (`--no-default-features --features bundled-sqlite --tests`) in `library-only` under the target folder, which `sdlc/scripts/package` shares, and under nextest runs only the integration tests, since only they can start the command. On a warm lane the step took 2.2 s for 131 tests. A plant that removed the `cli` gate from `key_address::command_refuses_before_plan_status_or_recording_output` failed the step. Its first run also found `engine::usage::tests::a_failed_retry_sidecar_keeps_the_durable_base_and_warns_once` failing on main after the UTC month turned to 2026-10, in every feature set; the same quick fix reads the current month there.

Kind: debt

Pay when: a second library-only failure reaches a rehearsal, or before 0.1.

Debt: 028

Paid: 2026-09-30

Severity: low

Keeping it lets a new library test that starts the command without the `cli` gate pass every routine gate and fail only in the release `crate` job.

# No routine gate runs the library-only tests

## The problem

`sdlc/scripts/package` runs `cargo test --package thinkthen --no-default-features --features bundled-sqlite --all-targets` in its own target folder, so a test that starts the command without `#[cfg(feature = "cli")]` fails there. Only the release `crate` job runs `sdlc/scripts/package`. `sdlc/scripts/lint` refuses to run it, and `sdlc/scripts/test` runs the default features, which build the command. The five tests in `sdlc/issues/closed/2026-09-30-crate-job-library-tests-start-a-command-never-built.md` escaped this way.

## A fix

Add the library-only `--test library` run, in its own target folder, to `sdlc/scripts/test`. It costs one more build of the library without the command's dependencies.
