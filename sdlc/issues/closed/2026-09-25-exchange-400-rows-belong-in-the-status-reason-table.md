# The exchange 400 rows belong in the status reason table

Status: Closed 2026-09-26 by ticket 0138.

Kind: cleanup. Found in the code review of ticket 0123.

## What happens today

Two tests spawn `decide` against a loopback backend that answers status 400.

- `common_request_statuses_name_fixed_actions_and_hide_the_body` in `crates/thinkthen/tests/backend/exchange.rs` checks the 400 and 500 sentences and that the body stays hidden.
- `a_400_names_only_the_known_reason_from_a_bounded_body` in `crates/thinkthen/tests/backend/status_reason.rs` checks the same 400 sentence for five bodies, the `max_tokens_exceeded` sentence, and status 422.

The 400 row in `exchange.rs` repeats a row of the `status_reason.rs` table. Ticket 0123 put its table in a new file because `exchange.rs` sat at the lint rung's 500-line file ceiling, and ticket 0127 owned `exchange.rs` then. So `status_reason.rs` builds its own `decide` spawn: the argument list and the `spawn` call with a fake key. The shared `decide` helper in `exchange.rs` does the same work.

## The fix

After ticket 0127 lands:

- Move the 400 and 500 rows of the `exchange.rs` test into the `status_reason.rs` table, with their stderr lines.
- Delete the `exchange.rs` test `common_request_statuses_name_fixed_actions_and_hide_the_body`.
- Replace the hand-built argument list and `spawn` call in `status_reason.rs` with the shared `decide` helper, moved where both files can use it.

That drops `exchange.rs` below its file ceiling and leaves one table for every status sentence.

Done when: one table holds the 400, 422, and 500 rows, no second test checks the 400 sentence, and `status_reason.rs` builds no spawn of its own.
