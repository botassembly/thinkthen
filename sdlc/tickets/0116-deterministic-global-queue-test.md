---
flow: quick-fix
priority: 116
opens: crates/thinkthen/tests/backend/annotate/scheduling.rs sdlc/ratchet.json
---

# 0116: Deterministic global-queue test

Status: in progress. Owner: Claude. Built and recorded (`sdlc/records/0116-quick-fix-deterministic-global-queue-test.md`). A fresh review is next.

## Outcome and authority

`annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` fails on a loaded machine (`sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`). The listener holds each document request for 25 ms, and the test asserts that two overlapped. A loaded machine sends them further apart. This quick fix changes only test code.

## Work

1. Reproduce the failure under `stress-ng` load.
2. Hold each of the first `min(jobs, 6)` document requests until all of them have arrived. Assert the peak equals `min(jobs, 6)`. That keeps the bound and the overlap and adds the full fill.
3. Show 20 of 20 passes under the same load, and show failures on a queue that admits one too many and on a queue that admits only one.

Touches only the test file, `sdlc/ratchet.json`, the issue, this ticket, and its record. Ticket 0077 lists `crates/thinkthen/tests/backend` and `sdlc/ratchet.json`, so the edit stays inside one test and its helper.
