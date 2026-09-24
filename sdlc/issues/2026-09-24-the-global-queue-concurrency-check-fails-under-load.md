# The global-queue concurrency check fails under load

Status: Open

`annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` asserts at `tests/backend/annotate/scheduling.rs:102` that the listener held more than one request at once. The listener holds each request for 25 ms. On a loaded machine the command can send its requests further apart than that, and the check fails.

Observed on 2026-09-24 while building ticket 0089:

- Main at `ab72203c`: 8 copies of the test binary run in parallel, 10 rounds, failed 2 of 80. The full `backend` suite passed 6 of 6.
- The 0089 branch at `3686414f`: the same stress failed 1 of 80. The full `backend` suite failed this test 2 of 7 runs. Alone, 40 runs passed 40.

The check proves a real property. A fix should hold the answers until two requests have arrived, for example with the `after_release` barrier the harness already has, instead of relying on a 25 ms window.
