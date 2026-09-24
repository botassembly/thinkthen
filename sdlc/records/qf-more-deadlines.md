# Quick Fix qf-more-deadlines: bound the child waits in every other test binary

Status: landed. It closes `sdlc/issues/2026-09-24-other-test-binaries-wait-on-children-with-no-limit.md`.

## Result

- `crates/thinkthen/src/test_deadline/run.rs` holds `output`. It runs a command the way `Command::output` does, with an empty input and both outputs piped, and ends it through `finish` from `wait.rs`. A child still running at 60 seconds is killed. The error names the program and its arguments, and never the environment. The lib tests reach it as `crate::test_deadline::output`.
- Each integration test binary includes `wait.rs`, and `run.rs` where it needs it, through `#[path]`. One copy of each file serves every binary. No file gained a suppression.
- `version`, `relate_edge`, `tag_edge`, `transform`, `demo_runner`, `status`, and one `choose_and_score_edge` case call `run::output` in place of `output()`.
- `choose_and_score_edge`, `decide_edge`, `find_edge`, and `question_file/harness.rs` feed their child its input, so they keep `spawn()` and call `wait::finish` in place of `wait()` or `wait_with_output()`.
- In the lib tests, `engine/host_signal_tests.rs` and `cli/conformance_tests/command.rs` call `crate::test_deadline::output`. `cli/schedule/width_tests.rs` already called `finish` on main.
- The 30-second signal park from `qf-test-deadlines` needed no new caller. No test child outside the SIGINT and identity tests parks for a signal.
- No production code changed.

## Left as they are

- The `transform.rs` child that holds its input open already runs under its own 20-second loop.
- Waits that follow a `kill` stay.
- The `mkfifo` and `kill` calls through `status()` run system tools and never the binary under test.
- `conformance/backend/tests/binary.rs` already reads each output line under its own deadline.

## Review

A fresh read-only Opus review of `15f6a33b` returned ACCEPT with two notes. Its reply is in `sdlc/records/qf-more-deadlines-review.md`.

## Planted bugs

Each plant ran once on `origin/main` at `9527d661` and once on the fix at `15f6a33b`, under `timeout 120`, at a load under 8. Plant A makes `main` park forever before `thinkthen::entry()`. Plant B makes `file_size_child` park forever before its recording.

| Plant and test | Code | Result |
| --- | --- | --- |
| A, `version::version_flag_prints_the_identity_line_and_exits_zero` | `origin/main` | hung. `timeout 120` killed it at 120 seconds, exit 124 |
| The same | fix | failed in 60.1 seconds with `thinkthen --version ran past 60 seconds and was killed` |
| A, `decide_edge::a_model_flag_replaces_the_default_model` | `origin/main` | hung. `timeout 120` killed it at 120 seconds, exit 124 |
| The same | fix | failed in 60.7 seconds with `decide asks for a refund --dry-run --model jev-1.13.0 ran past 60 seconds and was killed` |
| B, `engine::host_signal_tests::a_host_sigxfsz_action_stays_installed_through_a_recording` | `origin/main` | hung. `timeout 120` killed it at 120 seconds, exit 124 |
| The same | fix | failed in 60.2 seconds with `sh -c ulimit -f 1; exec "$0" --exact engine::host_signal_tests::file_size_child … ran past 60 seconds and was killed` |

Plant A covers `run::output` and `wait::finish` in integration binaries. Plant B covers `crate::test_deadline::output` in the lib tests. The plants were reverted before any commit.

## The test gate of `2026-09-24-tests-earn-their-place.md`

This fix adds no test. It changes how existing tests wait, and the planted bugs above prove the change. A regression test of the helper would hang for 60 seconds on every run.

## Ratchet

The ceiling rises from 50981 to 51041. `run.rs` adds 31 lines and the `#[path]` includes add 30. `rustfmt` splits several call sites that now wrap their builder. Each binary needs its own include, and `run.rs` holds the spawn and pipe setup once, so no call site repeats it.

## Checks

The whole ladder ran with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and each rung under `timeout`, at the commit that adds this section. That commit lands. The merge commit on main states its result. `sdlc/scripts/live` did not run.
