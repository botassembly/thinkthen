# Attempt accounting before run facts

This read-only investigation uses executable main `91766a17` and accepted SQL-settings design `2999814e`, item 4. It prepares the smallest correction needed before ticket 0170 prints exact `requests_sent` and `retries`. It does not implement, test or amend that design. The SQL host settings and future `SendBudget` reservation remain with 0149.

## Current boundary

`engine/request.rs:97-103` passes `Counters::attempt_sent(retry)` into `Client::post_observed_with_retry`. In `engine/http.rs:153-168`, a gate permit is acquired, `cancel.stop_or_remaining()` checks the stop, the counter hook runs, `cancel.remaining()` checks only the deadline, then `send` starts. `engine/mod.rs:131-139,177-185` confirms the distinction. If the deadline expires inside the hook, the second check prevents transport after a count was charged. A fired token is not the specific post-hook check at issue. The counted attempt and retry are stored together in `engine/usage.rs:62-69`; its `add` acquires a mutex, computes a month, may grow the pending vector and may spawn the writer. Calling it “nonblocking” would be inaccurate.

The existing `engine/deadline_tests.rs::accounting_that_outlasts_the_budget_sends_nothing` deliberately sleeps inside the pre-attempt hook and asserts deadline plus zero connections. It does **not** assert `Counters::snapshot`, so it passes while an unsent attempt could be charged. The adjacent spent-deadline and `engine/http/tests.rs` retry-wait/refused-attempt cases separately prove no initial send, no second send after a stopped wait, and counting a refused transport attempt. Preserve those distinct behaviors.

## Smallest reviewed choice

| Approach | Result and limit |
| --- | --- |
| Move the current hook after `cancel.remaining()` | Insufficient. Its mutex and writer startup can outlast the deadline. The existing slow-hook test would then send after expiry if merely moved. The test cannot be weakened to bless that behavior. |
| Count after `send` returns | Reject as a stand-alone correction. `Counters::snapshot` would undercount a held in-flight response; SQL's current pre-call process-total checks could admit another call before the first count appears. It also reverses the explicit pre-transport count order in accepted 0149. |
| Prepare bookkeeping before the final check, then mark immediately before transport | Candidate bounded slice. Acquire/prepare the counter state, pending month slot and writer before the final stop/deadline check; then mark `requests_sent` and `retries` together, release the guard and call `send` without another stop check. This keeps an in-flight attempt visible and moves predictable blocking work before the check. The future `SendBudget` reservation can be inserted between the final check and mark under 0149 without adding a command budget. |

The candidate needs a focused design/code review before implementation. Holding a usage mutex across `stop_or_remaining()` may interact with a host callback that reads usage; `engine/workers.rs::on_worker` normally runs HTTP on a worker while the host check is restricted to the caller thread, but the private HTTP test door can call directly. Preparing the month before the check may cross a calendar boundary before the mark. The implementation must resolve both without moving blocking work back after the check. No implementation can prevent an operating-system scheduling pause between the last check and transport; the contract can require that the code introduces no wait there, not that a deadline cannot pass while the process is descheduled.

The smallest expected product scope is `crates/thinkthen/src/engine/usage.rs`, `engine/http.rs` and `engine/request.rs`. Keep `Counts`'s `thinkthen.usage/1` shape and defaulted `retries`, and leave public APIs, SQL hosts and `SendBudget` out of this slice. A builder may need `engine/mod.rs` only if it proves a private stop check must change; claim that file before editing. The existing `engine/deadline_tests.rs` and selected `engine/http/tests.rs` are the proof surfaces. Avoid a new general accounting layer unless the guard risks above make it necessary.

## Bounded red and green proof

Extend the slow-accounting deadline case to use the real `Counters` mark and assert `snapshot().requests_sent == 0` and `retries == 0` alongside zero accepted connections. Keep its delay before the final deadline check. A separate held-response case should observe one in-flight attempt in `snapshot()` before releasing the reply, and the existing refused-attempt/retry-wait cases should retain their exact attempt counts. Read an old usage row without `retries` using the existing `engine/usage/tests.rs` case; do not alter its schema. Run only these selected unit cases, format and focused lint during the slice; the related-ticket checkpoint handles wider validation. No test or experiment was run for this investigation.

The unresolved decision is the prepared-guard form, including callback reentrancy and month attribution. If it cannot preserve the accepted final-check/count/transport order in a small private change, implement 0149's send-boundary mechanism first and defer exact 0170 facts. Do not substitute a post-response count or an extra stop check after the mark without a reviewed change to the accepted contract.
