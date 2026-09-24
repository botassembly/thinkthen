# Other test binaries wait on children with no limit

Status: open. Found 2026-09-24 during Quick Fix `qf-test-deadlines`. Owner: Claude.

Quick Fix `qf-test-deadlines` gave a deadline to every plain child wait in the `backend` test binary and in the SIGINT and identity tests. Other test binaries still call `output()`, `wait()`, or `wait_with_output()` with no limit. They are `tag_edge.rs`, `version.rs`, `transform.rs`, `relate_edge.rs`, `choose_and_score_edge.rs`, `status.rs`, `demo_runner.rs`, `find_edge.rs`, `decide_edge.rs`, and `question_file/harness.rs` under `crates/thinkthen/tests`. In the lib tests, `engine/host_signal_tests.rs`, `cli/schedule/width_tests.rs`, and `cli/conformance_tests/command.rs` do the same. A defect that keeps the binary running turns any of those tests into a hang.

Each integration test file is its own binary, so each needs its own `#[path]` include of `crates/thinkthen/src/test_deadline/wait.rs`. The lib tests can call `crate::test_deadline::finish` directly. A fix swaps each plain wait for `finish`. A test that collects output with `output()` first switches to `spawn()` with piped output.
