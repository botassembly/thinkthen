ACCEPT

# Review of Quick Fix qf-model-mismatch-test

Reviewer: a fresh read-only Opus session that did not write the work. Its first reply, on `8cfd6c5f`, listed findings. Its second reply, on `37c06950`, accepted the fix. This page restates both.

## First pass, `8cfd6c5f`

1. A correct run could still fail. Group 2 had to answer after the engine handled group 1's reply, and the 3-second window only made that likely.
2. At `--jobs 1` the engine handles the mismatch before it sends anything else. The test can expect exactly 2 requests with no helper and no wait. The reviewer recommended this.
3. The harness `spawn` has no time limit. A planted bug that never ends the run hangs the test. File an issue.
4. The test passes the four-question gate.

Plants: recording the failure without `stop()` hung 3 of 3 runs until a 90-second limit. Setting `halted` without clearing pending work, with `dispatch` ignoring `halted`, failed 4 of 4 runs with `left: 4, right: 3` in about 0.2 s. The unmodified copy passed 6 of 6 at about 3.3 s each.

## Second pass, `37c06950`

ACCEPT. One job still proves that a mismatch cancels groups that have not started. No other test covers it, because the other two mismatch tests put the mismatch on the last group. Notes: write this record before landing, and optionally pin the mismatch message.
