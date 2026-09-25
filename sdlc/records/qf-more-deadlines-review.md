# Review of Quick Fix qf-more-deadlines

Reviewer: a fresh read-only Opus session that did not write the work. It reviewed `15f6a33b` and returned ACCEPT with no must-fix finding and two notes.

Notes:

1. Three short `status()` calls still have no limit: the `mkfifo` in `crates/thinkthen/tests/transform.rs` and the `kill` calls in `crates/thinkthen/src/cli/interrupt/tests.rs` and `crates/thinkthen/tests/backend/interrupt.rs`. They run system tools and never the binary under test, and the issue did not name them. They stay as they are.
2. The worktree held the planted parks while the review ran. The reviewer ran clippy and the tests on a scratch copy of the commit and deleted the copy. The plants were reverted before any commit.

What it checked: every `output()`, `wait()`, `wait_with_output()`, `status()`, and `spawn()` under `crates/` and `conformance/`; the stdio each swapped command set; that the deadline error names the program and arguments and never the environment; that the helper has no copy and no file gained a suppression at its top; the ratchet explanation; and the test gate. Clippy over the workspace was clean. The nine changed test binaries passed, and so did the lib tests `host_signal_tests` and `conformance_tests::command`.
