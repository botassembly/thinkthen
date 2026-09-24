# 0097: Run the host interrupt check

Status: built on `ticket/0097-interrupt-check`; code review pending. Owner: Claude.

Base: main `836e3f23` with 0085. Code commit `334fc076`.

## Result

The call's stop token, `engine::Cancel`, carries the private call options. It already held the call's deadline. It now also holds an optional `&(dyn Fn() -> bool + Sync)` check and the ID of the thread that set it, through `Cancel::with_check`. The command sets no check, so command behavior does not change.

- The check enters only through `Cancel::stop_or_remaining`, the one cancel-and-deadline poll that `stop`, `wait`, the width gate, the recorder and lock waits, and both scheduler loops already call. It runs only when the polling thread is the thread that set it. A worker's poll skips it.
- A `true` return fires the call's token, and the call returns `Cancelled` through the existing stop paths. A bulk run returns `Stopped` with its cause and position.
- A panic in the check fires the token, then resumes the payload unchanged. The unwinding `thread::scope` joins every worker, and each worker has already stopped. No path turns the panic into `Defect`.
- 0085 sends a single call's live attempt on one scoped worker, so the width gate and the retry wait run there. `workers::on_worker` now takes the token, and the calling thread polls every 50 ms until the worker ends. `http::Client::post_observed` counts each blocking send on the token, and the calling thread skips the check while that count is above zero. A scheduler's calling thread keeps polling during sends, because its workers carry the sends.
- No call site was added in `annotate_schedule.rs` or `schedule.rs`.

## Tests

`crates/thinkthen/src/engine/facade_tests/interrupt_tests.rs` holds three tests against counted loopback listeners. Each held call runs on its own thread, and the check records that thread and the listener count on each run.

- A table of four held waits: a width gate held by four sends, a 503 retry wait of 20 s, a recording folder held by another lock owner, and a bulk run of six records with four replies held. The check answers `true` on its third run while the call is held, and a call that never reaches it fails at 3 s. Each row asserts that every run was on the calling thread, that the first run came before the call's first send, that the call returned `Cancelled`, and that the listener saw no new request. The width row also asserts that the four held sends finish with their answers.
- One held single send: no check runs across six polls, and one ran before the send.
- A check that panics during a held width gate. The gate is freed two polls after the panic. The caller receives the same `&str` payload, the four held sends finish, and the listener count stays at four.

The red step was a compile failure: `with_check` did not exist. The planted bugs below show each test failing for its stated reason. Twenty repeated runs passed, each in about 0.45 s.

## Planted bugs

Each plant ran alone on `334fc076` and was reverted.

| Plant | Result |
| --- | --- |
| The check runs on any thread, a worker included | red: "width gate: the check ran on a worker" |
| A panic in the check becomes `Defect` | red: "the panic was swallowed into" an `Err` |
| The panic path resumes without firing the token, and the unwinding scope joins | red: "nothing was sent after the panic", 5 sends where 4 were expected |
| The check runs only once per calling thread | red: "width gate: the check ran at each poll", and the panic test's check never panicked |
| Extra: the send count is dropped, so the check runs during a send | red: "no check ran during the held send", 6 runs where 0 were expected |

## Budgets

Measured with `git diff -U0` over `*.rs`, nonblank lines, against the merge of main `836e3f23`:

- Production: 6 files (`engine/mod.rs`, `engine/workers.rs`, `engine/http.rs`, `engine/request.rs`, `cli/edge.rs`, `cli/interrupt.rs`). 108 lines added and 24 deleted. The budget is 6 files and 180 lines.
- Tests: 245 lines added, under the 400-line budget.
- The ratchet rose from 49733 to 50059.

## Known limits

- The annotate scheduler (`groups`) has no row of its own. It polls the same function each tick as the record scheduler, which the bulk row covers.
- The check runs on a scheduler's calling thread while its workers send. The ticket bars a check only during one held single send.

## What the ticket did not foresee

- No private call-options type exists yet. The token already carried the deadline, so the check joined it. The borrowed check gives `Cancel` a lifetime. The command's two long-lived tokens now name `Cancel<'static>`, which put `cli/edge.rs` and `cli/interrupt.rs` in the file count. Two test helpers also name `'static`.
- 0085 moved a single call's gate and retry wait onto a worker, so polling in the waits alone would never run the check on the calling thread. The calling thread's join now polls, and the send count keeps the check out of a blocking send.
- `Batch` and its `next` do not exist until 0086. The facade's `records` and `groups` loops poll each tick. 0086's `Batch::next` must reach the same poll.
- The "joins without cancelling" plant can turn red only on a single call. A scheduler never queues work beyond its free workers, so a bulk run has no queued item left to send. The panic test therefore holds a single call at the width gate.
