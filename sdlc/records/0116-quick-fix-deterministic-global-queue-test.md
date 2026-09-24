# 0116: Deterministic global-queue test

Status: built on `ticket/0116-deterministic-global-queue-test` from `65f43693`. A fresh review has not run yet. The ceiling rise needs that review before landing.

## Result

- The document half of `one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` no longer delays each answer 25 ms. A `Gathering` helper holds each of the first `min(jobs, 6)` requests until all of them have arrived. The test asserts the listener's peak equals `min(jobs, 6)`.
- The old checks were `peak <= jobs` and, above one job, `peak > 1`. The new check keeps both and also proves the queue fills to its bound.
- The last arrival waits 50 ms before it releases the others. That gives a request past the bound time to arrive while all are held. A correct run never depends on it.
- A run that never reaches the count waits out a 10-second failsafe and fails on its peak. The failsafe stays under the tool's 30-second request timeout.
- The stream half is unchanged. Its only check is `peak <= jobs`, and load cannot fail it.
- Ticket 0077 lists `crates/thinkthen/tests/backend` and `sdlc/ratchet.json`. The edit stays inside this one test and its helper, and the ratchet change is one number.

## Ratchet

The ceiling rises from 46972 to 47007. The 35 lines are the helper, its imports, and its construction. The harness `after_release` barrier cannot serve here: it waits forever when the second request never comes, and the listener's own timed gate is private to the conformance crate. The helper replaces two asserts with one.

## Red and green

Load: `stress-ng --cpu 48`, with 8 copies of the test binary in parallel for each round. The one-minute load rose to between 30 and 48.

| Run | Test | Tool | Result |
| --- | --- | --- | --- |
| 10 rounds of 8 | `origin/main` | correct | 74 pass, 6 fail at `scheduling.rs:102`, the old `peak > 1` check |
| 10 rounds of 8 | fixed | correct | 80 pass, 0 fail |
| 3 rounds of 8 | fixed | planted: `jobs + 1` workers and `in_flight <= jobs` | 0 pass, 24 fail: 10 on `document at 1` with peak 2, 14 on the `jobs == 1` order check |
| 1 run, no load | fixed | planted: 1 worker and `in_flight < 1` | fail, `document at 4`, peak 1 against 4, in 31 s |

The plants sat in `crates/thinkthen/src/engine/annotate_schedule.rs` and were reverted. No production code changed.

One earlier load run of the fixed test failed once with exit 5 at `document at 1`. The 8 parallel copies share one question file and one usage folder under `CARGO_TARGET_TMPDIR`, and one copy rewrote the file while another read it. That comes from the stress harness, and the gate never runs two copies of one test.

## Checks

With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset:

- `install`: exit 0.
- `lint`: exit 0, `ratchet: crates + conformance 47007/47007`.
- `test`: exit 0, 767 passed, 0 failed, 2 ignored across 18 result lines, `live-test: all cases passed`. The one-minute load was 8.99 at the start.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.
