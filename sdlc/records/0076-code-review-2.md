ACCEPT

Second code review of ticket 0076, whole-call deadlines. Branch `ticket/0076-whole-call-deadlines` at `726ddb8f`, rebased onto main `daac6bbc`. A fresh, read-only Claude session did this review. It read the first review, the fix commits `16e1b824` and `a386e0c7`, the updated build record and ticket, and ticket 0073. Every result below was observed by command in a scratch clone with the `THINKTHEN_` variables unset. The scratch clone was deleted afterward.

## The blocking finding is fixed

`post_observed` (`engine/http.rs:114-119`) now checks for a stop, runs `before_attempt()`, and only then reads the budget through `Cancel::remaining()`. The send limit comes from that second reading.

- Red. I moved the budget read back in front of `before_attempt()`. `accounting_that_outlasts_the_budget_sends_nothing` then failed at `deadline_tests.rs:234`, the nonblocking `accept` assertion. The listener had accepted a connection. The run took 0.71 s. The `Deadline` assertion passed first, which matches the first review's probe.
- Green. With the fix restored, the test passed in 0.50 s with zero accepted connections.

## The cancellation choice is correct

After accounting, the second reading checks only the deadline. A call cancelled while accounting runs still sends its one request. This is the behavior Ian ruled, not a defect.

- Ticket 0073 line 13 sets one attempt-start check "immediately before request accounting and transport". Line 21 says an attempt past that check ignores later cancellation until transport returns. Line 35 says request accounting counts the attempt that was sent. ADR 0017 rules the cancellation behavior, and 0073 records it. Ticket 0076 keeps "interrupting cancellation's already-started attempts" out of scope.
- The older test `cancellation_during_a_retry_wait_starts_no_second_attempt` (`http.rs:419`) fires cancellation inside the observer. It does this to put the cancellation inside the retry wait. It also requires `received == 1`, so the first request must go out. I swapped `remaining()` for `stop_or_remaining()` in the scratch clone. That test then hung: its server waits in `incoming().take(2)` for a connection that never arrives. This confirms the record's account.
- Money. A cancelled call can spend one request. The same thing happens when cancellation arrives a moment after the check, so the risk existed before this change and is the ruled trade-off. The attempt was already counted in `requests_sent`, so the count stays true. A deadline differs because ADR 0017 requires a spent deadline to send nothing. The fix therefore treats the two stops differently, and correctly so.
- One thing Ian can overturn: a slow usage lock makes the gap between the check and the send longer. If he wants cancellation during a usage-lock wait to send nothing, that belongs to 0073's rule, not to this ticket. Changing it would also mean changing the older test and accepting one counted attempt that was never sent.

## No other read-then-wait gap on the send path

- `ask_prepared` observes cancellation and the deadline before the key lookup. The key lookup reads the environment and does not wait. `post_observed` checks again at the top of every attempt.
- Between `remaining()` and `send` nothing waits. `send` sets ureq's `timeout_global` from that reading, and ureq applies it to the whole request, including name lookup and connection.
- A retry goes through `cancel.wait`. That call caps each sleep at the remaining budget and returns the stop it observed. The loop then checks again before accounting.
- Both call sites (`engine/request.rs:69`, `cli/asking/request.rs:63`) pass only `usage.request_sent()` as the observer.
- Not blocking: when only a few microseconds remain, `send` may still start a loopback TCP handshake before ureq's limit fires. No request bytes can be sent in that case, so no money is spent. No fix is needed.

## Ratchet

`node sdlc/scripts/ratchet.mjs` printed `crates + conformance 46972/46972`. The ceiling in `sdlc/ratchet.json` equals the measured total.

## Ladder at the tip

The one-minute load was 1.3 at the start and 5.3 during lint. The `THINKTHEN_` variables were unset, and none remained. The rungs ran one after another:

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/test`: exit 0. It passed 767 tests with 0 failures, and `live-test: all cases passed`, which uses dummy keys in a temporary repository.
- `sdlc/scripts/spec`: exit 0, with 21 demos green and 0 red.

`sdlc/scripts/live` never ran.

## Record

The build record's review-fix section matches what I observed: the red run took 0.70 s, the green run showed zero connections, the one-extra-count consequence is stated, and the ratchet went up 35. The ticket's Review section states the consequence and the note on the `convert.rs` mapping.
