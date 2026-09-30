# 0315: Usage lock wait and retry jitter

Status: built, awaiting code review. Lane claude-4. Branch `ticket/0315-usage-lock-and-jitter`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order items 4 and 8. Issue: `sdlc/issues/2026-09-26-architect-review-07-throughput-limits-cost.md`, items 2 and 3.

## Outcome

The usage writer waits for another process's usage lock with no fixed short sleep loop. Retries without a server header no longer move in lockstep across workers and processes.

## Evidence

- Starts from: main `6f62e74fa`. Issue item 2 (an unbounded exit wait on the usage lock) was already fixed by ticket 0225: `specification/recording.md` gives the writer one second from the finish point, then drops unwritten counts with the fixed usage warning, and the answer and exit code stand. `tests/backend/facts/usage_lock.rs` proves it. The fix left a fixed 10 ms `thread::sleep` poll in `engine/usage/lock.rs`. Issue item 3 still holds: the unheaded retry wait in `engine/http/retry.rs` is exactly 1, 2, 4 seconds for every worker.
- Keeps: the one-second finish bound, the fixed usage warning, the answer and exit code under a held lock, the server `Retry-After` floor, the 60-second and attempt-timeout caps, the per-address process gate and pacer.
- Changes: the usage lock retry pauses on the queue's condition variable, doubling from 1 ms to 100 ms, so publishing the finish deadline wakes it at once. The unheaded retry wait is drawn between half and all of its capped doubling. The draw comes from the standard library's per-thread random hasher keys and is a parameter of `bounded_wait`, so tests pass fixed draws. `specification/backends.md` says so. No dependency added.
- Proof: an edge table pins `bounded_wait` for header floors, caps, and draws 0, half, and max. A test draws waits on four threads and requires them inside half-to-full and not all equal; the old deterministic wait fails it. The held-lock command regression and the usage, HTTP, exchange, and backoff suites pass.
- Defers: jitter on server-headed waits (the server sets that floor); issue item 4 (one slow record stalls the ordered window); the other severity 3 titles.

## What the build taught us

The issue's severity 2 item was stale: ticket 0225 had already bounded the wait and chosen skip-with-warning. Check the spec before trusting a review's line numbers. The standard library has no lock with a timeout, so a short poll stays; waiting on the existing condition variable lets finish cut the pause short without a new signal.
