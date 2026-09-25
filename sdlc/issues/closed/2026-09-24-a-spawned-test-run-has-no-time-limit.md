# A spawned test run has no time limit

Status: closed by Quick Fix `qf-test-deadlines`. Found 2026-09-24 by the review of Quick Fix `qf-model-mismatch-test`. Owner: Claude.

`spawn` in `crates/thinkthen/tests/backend/harness/mod.rs` starts the compiled binary and calls `wait_with_output` with no deadline. A defect that keeps the binary running turns a failing test into a hang. The review planted one: a model mismatch that records its failure without calling `stop()`. The annotate row never finished, `done()` never returned true, and 3 of 3 runs sat until an outside 90-second limit killed them.

A fix gives `spawn` a deadline well past any honest run, such as 60 seconds. On expiry it kills the child and returns an error that names the arguments, so the test fails with a message.
