# 0166: Build a cancelled call that never succeeds

Status: built 2026-09-27, awaiting code review. Owner: Claude.

Branch `ticket/0166-a-cancelled-call-never-succeeds`, in lane 3. The ticket is `sdlc/tickets/0166-a-cancelled-call-never-succeeds.md`, accepted at `3d8b55d2`. The build merged `origin/main` before it began. No live call ran. Every rung and deliberate break ran with `THINKTHEN_API_KEY` unset.

## Result

- `Cancel` in `engine/mod.rs` holds the caller's token flag beside its own, through the new `with_token`. `stop_or_remaining` reads it on every thread, before the host check. `fired()` keeps meaning the call's own flag.
- `CancelToken` gained a private `flag()`. `Stop::begin` passes it to `with_token`.
- `Stop::finish` takes the call's result. After it resumes a held check panic, a fired token turns any result into `Error::cancelled()`. `Stop::run` returns through it, and `Stream::take` passes the batch's end through it.
- `thinkthen.h`, `DESIGN.md` section 1 and the `CancelToken` doc comment carry the ticket's text.

## Changed from the ticket

- The two Rust tests live in a new module, `crates/thinkthen/tests/public_controls/fired.rs`, which `public_controls.rs` includes with `#[path]`. With them inline, `public_controls.rs` reached 548 nonblank lines, over lint's 500-line file ceiling. The module adds 10 lines of header and imports.
- Clippy's nesting limit moved the fire-on-arrival loop into a helper, `fire_on_arrival`.
- `libraries/c/tests/door/main.rs` grew 49 net lines against a budget of 45, inside the tenth that stop rule 1 allows.

## Edge rows

| Row | Result |
| --- | --- |
| 1. Held typed decide, token A fired | `THINKTHEN_ECANCELLED`, `the call was cancelled`, `out` unchanged. Count 1 |
| 2. Held JSON decide, token B fired | NULL, code 5, the same message. Count 2 |
| 3. Held bulk of five, token C fired with four in flight | `THINKTHEN_ECANCELLED`, five slots unchanged. Count 6 |
| 4. Fresh token, row 1's question and text | `THINKTHEN_OK`, YES, 0.9. Count 6 |
| 5. Token A again | `THINKTHEN_ECANCELLED`. Count 6 |
| 6. `{"usage":true}` | `{"requests_sent":6,"input_tokens":0,"output_tokens":0,"cache_answers":1}` |
| 7. 503 with `retry-after-ms: 0`, fired on arrival | `Cancelled`, count 1 |
| 8. 422, fired on arrival | `Cancelled`, count 1 |
| 9. One-text batch, fired after its row | Row, then `Cancelled`, then `None`. Count 1 |

## Tests

- `a_token_fired_during_a_held_reply_cancels_the_call` in `libraries/c/tests/door/main.rs` drives `tests/c/cancel.c` over rows 1 to 6. It pins exit 0, standard output of exactly `fired A`, `fired B` and `fired C`, empty standard error, and a count of 6.
- `fired::a_token_fired_during_a_send_ends_the_call_cancelled` covers rows 7 and 8.
- `fired::a_token_fired_before_a_batch_ends_ends_it_cancelled` covers row 9.

## Deliberate breaks

Each edited one source file, ran the named test under the heavy lock, and was restored with `git checkout`.

| Break | Result |
| --- | --- |
| (a) `finish` stops reading the token | Red. Rows 1 and 2 fail in the C program, which exits 1 |
| (b) `stop_or_remaining` stops reading the token | Red in five of five runs. Row 7 counts 2 |
| (c) `finish` replaces only success | Red. Row 8 returns `Backend` |
| (d) `Stream::take` passes `End` around `finish` | Red. The second pull is `None` |
| (e) An early return on `cancel.stop()` after `on_worker` in `engine/request.rs` | Red. Usage reads `requests_sent` 7 and `cache_answers` 0 |

## Ratchets

- `sdlc/ratchet.json`: 72168 to 72262, up 94. `engine/mod.rs` up 12, `public/options.rs` up 12, `public/batch.rs` unchanged, and the tests up 70 across `public_controls.rs` and `public_controls/fired.rs`.
- `libraries/c/ratchet.json`: 2209 to 2258, up 49.
- `libraries/c/ratchet.c.json`: 688 to 796, up 108 for `tests/c/cancel.c`.

## Rungs

Each rung ran once, alone, with `THINKTHEN_API_KEY` unset, at load under 10.

- `lint` with `THINKTHEN_PRIVATE_NAMES`: exit 0.
- `test`: exit 0.
- `spec`: exit 0. Demos: 21 green, 0 red.
- `surfaces`: exit 0. Every binding and every installed archive passed, `libraries/c` included, and the release smoke passed.
