Status: open. Found while building ticket 0352. Owner: none yet.

Kind: debt

Pay when: a piped batching test fails once under load, or the input pause gains a test setting.

Debt: 030

Severity: low

Keeping it lets a stalled reader thread close a batch early in a test that expects fuller batches.

# Piped batching tests race the 50 ms input pause

## The problem

`crates/thinkthen/src/engine/pipeline/run.rs` closes the open batch when input pauses for 50 ms (`PAUSE`). That is the product's behavior, and it is right for a slow producer. A test that pipes its records and then expects full batches assumes the engine's reader thread wakes within 50 ms. Under heavy load a thread can stall that long.

Ticket 0352 removed the large case. The batching ceiling test wrote 120 KB through a pipe, past the pipe's buffer, so the test's own blocked write could reach the pause. It now reads its input from a file. The smaller piped batching tests fit in the pipe buffer, so only a stall of the engine's own reader can close a batch early.

## A fix

Give the pause a debug-only test setting, as `THINKTHEN_TEST_RETRY_WAIT_MS` does for the retry wait, and set it long in batching tests that expect full batches. Or feed those tests from files.
