ACCEPT

# Review of Quick Fix qf-parallel-lock-test

Reviewer: a fresh read-only Opus session that did not write the work. It reviewed `36ea9a8c`. It found the code sound and returned two findings on the written record. This page restates the reply, and the next commit fixes both findings.

Findings:

1. This review record did not exist yet.
2. The SIGINT issue named only prefix mode. The `THINKTHEN_SIGINT_CHILD` mode also parks in `loop { park() }` after it prints `armed`. The issue now names both.

Checks:

- Every wait is bounded. `hold` uses `wait_timeout_while` with a 10-second limit. A lone request waits 10 seconds and replies, and the late second request does not wait.
- The peak is deterministic. The listener counts a request in flight before it calls the reply closure. The annotate test's helper moved with no change in logic, and that test passed.
- The test passes the four-question gate. It drives the compiled binary with no test-only hook and catches one lock shared across digests. The equal-digest tests do not cover that case.
- The ratchet reads 48812/48812. The old copy of the helper was deleted.
- `sigint_child` lives in the lib unit tests, and `Gathering` lives in the separate `backend` binary. The Gathering change could not have caused the 68-minute park.

Plant, at a one-minute load of 5.2 to 7.1:

| Code | Test | Runs | Result |
| --- | --- | --- | --- |
| Correct | lock test | 2 | pass, about 0.15 s each |
| `locks.join("planted-shared-lock")` | lock test | 4 | fail, `left: 1, right: 2`, about 10.2 s each |
| Correct | annotate test | 1 | pass |
