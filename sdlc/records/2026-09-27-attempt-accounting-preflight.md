# Attempt accounting before run facts

This read-only investigation uses executable main `91766a17` and accepted SQL-settings design `2999814e`, item 4. It prepares the smallest correction needed before ticket 0170 prints exact `requests_sent` and `retries`. It does not implement, test or amend that design. The SQL host settings and future `SendBudget` reservation remain with 0149.

## Current boundary

`engine/request.rs:97-103` passes `Counters::attempt_sent(retry)` into `Client::post_observed_with_retry`. In `engine/http.rs:153-168`, a gate permit is acquired, `cancel.stop_or_remaining()` checks the stop, the counter hook runs, `cancel.remaining()` checks only the deadline, then `send` starts. `engine/mod.rs:131-139,177-185` confirms the distinction. If the deadline expires inside the hook, the second check prevents transport after a count was charged. A fired token is not the specific post-hook check at issue. The counted attempt and retry are stored together in `engine/usage.rs:62-69`; its `add` acquires a mutex, computes a month, may grow the pending vector and may spawn the writer. Calling it “nonblocking” would be inaccurate.

The existing `engine/deadline_tests.rs::accounting_that_outlasts_the_budget_sends_nothing` deliberately sleeps inside the pre-attempt hook and asserts deadline plus zero connections. It does **not** assert `Counters::snapshot`, so it passes while an unsent attempt could be charged. The adjacent spent-deadline and `engine/http/tests.rs` retry-wait/refused-attempt cases separately prove no initial send, no second send after a stopped wait, and counting a refused transport attempt. Preserve those distinct behaviors.

## Options considered

| Approach | Result and limit |
| --- | --- |
| Move the current hook after `cancel.remaining()` | Insufficient. Its mutex and writer startup can outlast the deadline. The existing slow-hook test would then send after expiry if merely moved. The test cannot be weakened to bless that behavior. |
| Count after `send` returns | Reject as a stand-alone correction. `Counters::snapshot` would undercount a held in-flight response; SQL's current pre-call process-total checks could admit another call before the first count appears. It also reverses the explicit pre-transport count order in accepted 0149. |
| Prepare bookkeeping before the final check, then mark immediately before transport | Candidate bounded slice. Acquire and prepare the counter state before admission; mark `requests_sent` and `retries` together after a callback-free final check, release the guard, then call `send` without another check. This keeps an in-flight attempt visible. Future `SendBudget` reservation stays a separate 0149 step. |

## Required internal design

Independent read-only review accepted the boundary analysis above but did not accept the first prepared-guard proposal. The correction must satisfy these rules before implementation:

1. Call today's `cancel.stop_or_remaining()` outside the usage queue lock. It preserves the host callback's current opportunity to run and report cancellation or panic. `engine/workers.rs::on_worker` normally runs production HTTP on a worker, where the caller-thread host callback does not run, but the private HTTP door can run on the caller thread. A callback may read usage. Never rely on the worker convention to make callback-under-lock safe.
2. Prepare the usage mutex, pending-vector capacity and any writer startup before the final check. This preparation must publish no count or zero-count month. If the stop/deadline rejects the attempt, dropping preparation leaves `snapshot()` and disk unchanged and creates no persistence-failure warning, even if writer startup was attempted. A spawned but idle writer may be retired normally. Preserve the current checked-overflow behavior at the actual mark: keep the previous in-memory total if `checked_add` fails, set the same failure state for an admitted attempt, and keep disk failure separate from the in-memory snapshot.
3. Under the prepared guard, run a **callback-free** final check of the fired flag, caller token and deadline; then sample the month for this admitted attempt, mark the in-memory total and pending month together, release the guard and start transport with no second stop check. Sampling at the mark avoids assigning a prepared attempt to the prior month across rollover. Reserve capacity before this point so the mark does not wait for the writer or allocate a pending slot. The existing `Cancel::remaining()` checks only the deadline; `stop_or_remaining()` can call user code. Neither is the required callback-free token/deadline check. A small private helper in `engine/mod.rs` is therefore an exact prerequisite and needs a separate file claim from 0201. It must not change the public host-callback rule.

This is a concrete proposed design, **not** an accepted runtime change. The reviewer must check the guard lifetime, month sample and abandoned-writer behavior against the real implementation. A clock read and brief in-memory mark remain; an operating-system scheduling pause between final check and transport is unavoidable. The code can promise no deliberate blocking wait there, not that the deadline cannot pass during descheduling.

The smallest expected product scope is `crates/thinkthen/src/engine/usage.rs`, `engine/http.rs`, `engine/request.rs` and the private callback-free check in `engine/mod.rs`. The latter is held by 0201 and needs an exact shared claim before implementation. Keep `Counts`'s `thinkthen.usage/1` shape and defaulted `retries`, and leave public APIs, SQL hosts and `SendBudget` out of this slice. The existing `engine/deadline_tests.rs`, `engine/http/tests.rs` and narrow `engine/usage/tests.rs` are the proof surfaces. Avoid a new general accounting layer unless review rejects this bounded guard.

## Bounded red and green proof

Extend the slow-accounting deadline case to use real `Counters` and assert zero `requests_sent`, zero `retries` and zero connections after its delay. Keep that delay before the final check. Exercise an abandoned preparation with a usage folder and prove no month row or persistence warning appears. A separate held-response case should observe one in-flight attempt in `snapshot()` before releasing the reply. Existing refused-attempt/retry-wait cases retain their exact counts; the old usage-row case retains zero defaulted retries. Run only these selected unit cases, format and focused lint during the slice; the related-ticket checkpoint handles wider validation. No test or experiment was run for this investigation.

The remaining decision is independent acceptance of the prepared-guard design and coordination of `engine/mod.rs` with 0201. If that exact claim or guard safety cannot be obtained, implement 0149's send-boundary mechanism first and defer exact 0170 facts. Do not substitute a post-response count or an extra stop check after the mark without a reviewed change to the accepted contract.
