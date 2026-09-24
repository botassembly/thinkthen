# 0097: Run the host interrupt check

Status: built on `ticket/0097-interrupt-check`; code review pending. Owner: Claude.

Base: main `836e3f23` with 0085. Code commits `334fc076` and, after the code review, the review-fix commit named in the ladder section.

## Result

The call's stop token, `engine::Cancel`, carries the private call options. It already held the call's deadline. It now also holds an optional `&(dyn Fn() -> bool + Sync)` check and the ID of the thread that set it, through `Cancel::with_check`. The command sets no check, so command behavior does not change.

- The check enters only through `Cancel::stop_or_remaining`, the one cancel-and-deadline poll that `stop`, `wait`, the width gate, the recorder and lock waits, and both scheduler loops already call. It runs only when the polling thread is the thread that set it. A worker's poll skips it.
- A `true` return fires the call's token, and the call returns `Cancelled` through the existing stop paths. A bulk run returns `Stopped` with its cause and position.
- A panic in the check fires the token, then resumes the payload unchanged. The unwinding `thread::scope` joins every worker, and each worker has already stopped. No path turns the panic into `Defect`.
- 0085 sends a single call's live attempt on one scoped worker, so the width gate and the retry wait run there. `workers::on_worker` now takes the token, and the calling thread polls every 50 ms until the worker ends. `http::Client::post_observed` counts each blocking send on the token, and the calling thread skips the check while that count is above zero. `with_check` gives each call its own count, so sends of other calls on a shared token never silence this call's check (review F1). The stop flag stays shared. A scheduler's calling thread keeps polling during sends, because its workers carry the sends.
- No call site was added in `annotate_schedule.rs` or `schedule.rs`.

## Tests

`crates/thinkthen/src/engine/facade_tests/interrupt_tests.rs` holds three tests against counted loopback listeners. Each held call runs on its own thread, and the check records that thread and the listener count on each run.

- A table of six held waits: a width gate held by four sends on their own tokens, the same gate held by four sends on the call's base token, a 503 retry wait of 20 s, a recording folder held by another lock owner, and six records with four replies held through the record scheduler (`records`) and through the annotate scheduler (`groups`). The check answers `true` on its third run while the call is held. Each call's token carries a 3 s deadline, so a call the check fails to stop ends at 3 s and fails the row. Each row asserts that every run was on the calling thread, that the first run came before the call's first send, that the call returned `Cancelled`, and that the listener saw no new request. The width row also asserts that the four held sends finish with their answers.
- One held single send: no check runs across six polls, and one ran before the send.
- A check that panics during a held width gate. The gate is freed two polls after the panic. The caller receives the same `&str` payload, the four held sends finish, and the listener count stays at four.

The red step was a compile failure: `with_check` did not exist. The planted bugs below show each test failing for its stated reason. Twenty repeated runs passed, each in about 0.45 s.

## Planted bugs

The first five plants ran alone on `334fc076`. After the review fixes, the first four ran again, together with the last two, and all six were red again. Each plant was reverted.

| Plant | Result |
| --- | --- |
| The check runs on any thread, a worker included | red: "width gate: the check ran on a worker" |
| A panic in the check becomes `Defect` | red: "the panic was swallowed into" an `Err` |
| The panic path resumes without firing the token, and the unwinding scope joins | red: "nothing was sent after the panic", 5 sends where 4 were expected |
| The check runs only once per calling thread | red: "width gate: the check ran at each poll", and the panic test's check never panicked |
| Extra: the send count is dropped, so the check runs during a send | red: "no check ran during the held send", 6 runs where 0 were expected |
| Review F1: `with_check` keeps the base token's send count | red: "width gate on a shared token: the check ran at each poll" |
| Review F3: the annotate scheduler's calling thread keeps the flag and the deadline but skips the check | red: "groups: the check ran at each poll" |

## Budgets

Measured with `git diff -U0` over `*.rs`, nonblank lines, against the merge of main `836e3f23`:

- Production: 6 files (`engine/mod.rs`, `engine/workers.rs`, `engine/http.rs`, `engine/request.rs`, `cli/edge.rs`, `cli/interrupt.rs`). 109 lines added and 24 deleted. The budget is 6 files and 180 lines.
- Tests: 313 lines added, under the 400-line budget. `facade_tests::feed` now takes a send closure, so both schedulers' readers share it.
- The ratchet rose from 49733 to 50059 with the build, then to 50125 with the review's two rows.

## Known limits

- The check runs on a scheduler's calling thread while its workers send. The ticket bars a check only during one held single send.

## What the ticket did not foresee

- No private call-options type exists yet. The token already carried the deadline, so the check joined it. The borrowed check gives `Cancel` a lifetime. The command's two long-lived tokens now name `Cancel<'static>`, which put `cli/edge.rs` and `cli/interrupt.rs` in the file count. Two test helpers also name `'static`.
- 0085 moved a single call's gate and retry wait onto a worker, so polling in the waits alone would never run the check on the calling thread. The calling thread's join now polls, and the send count keeps the check out of a blocking send.
- `Batch` and its `next` do not exist until 0086. The facade's `records` and `groups` loops poll each tick. The five duties this leaves to 0086 are in a dated builder note on the 0086 ticket (`fa6df53e` on `ticket/0086-public-rust-api`).
- The command shares one long-lived token across its calls, and 0086 may map one host token onto one private token. The first build shared the send count through that token. The code review found it (F1).
- The "joins without cancelling" plant can turn red only on a single call. A scheduler never queues work beyond its free workers, so a bulk run has no queued item left to send. The panic test therefore holds a single call at the width gate.

