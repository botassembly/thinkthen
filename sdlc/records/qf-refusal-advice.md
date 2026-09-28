# Quick Fix: give refusal advice that fits the request

Status: functional review accepted `da606611`; landed with the measured-growth explanation in the main merge commit. Branch: `ticket/qf-refusal-advice`, based on `origin/main` at `0c0d77d7`. Scope: experiment 284 register 53. No engine, scheduler, API, or batching-planner behavior changed.

## Findings on current main

Register 53's two named gaps were present. `crates/thinkthen/src/cli/failure/status.rs` reported 429 only as a rate limit. `crates/thinkthen/src/engine/http.rs::post_observed_with_retry` returns a retryable status only after its allowed attempts, including the first attempt when `max_retries` is zero; cancellation and an expired deadline return their own failures before that final status. `crates/thinkthen/src/cli/failure.rs::transport_message` gave every timeout the singleton advice to increase `--timeout` or try again. `crates/thinkthen/src/cli/asking/batched.rs::failed` wraps a failed request holding two or more records in `BatchFailed` with its last record, and `cli/schedule.rs` supplies the stopped record. That existing context supports a smaller-request hint without adding engine state. A one-record timeout remains a transport failure and needs its current sentence.

The surrounding fixed phrases still match their causes: 400 and 413 advise request size, 401–404 name key, credit, permission or address, 422 does not guess a lever, server statuses name exhausted attempts, and non-timeout transport kinds identify host, connection, TLS or an uncertain send. The six structured engine error kinds, retry list, split rule, exits and result shape remain unchanged. The new 429 wording says “allowed attempts,” not that a retry occurred; it is accurate under `--max-retries 0` and after the configured resends.

## Change and retained behavior

The 429 phrase now says the rate limit was reached after the allowed attempts and suggests trying later or changing `--max-retries`. A timeout of a request holding two or more records now suggests lowering `--batch` or `--max-request-bytes`, or increasing `--timeout`. The message still names the failed record range, exit 4 and finished count. Singleton timeout wording, retry behavior and the other fixed phrases stay as before. `specification/backends.md` names both cases and scopes “last allowed attempt” to include zero retries.

The existing stopped-run renderer moved unchanged in behavior to private `cli/failure/stopped.rs` so the new contextual branch fits beneath the 500 nonblank-line file cap. No new option or context was threaded through the engine. The one existing unit expectation for 429 and two existing listener tables now pin the changed sentence. A new listener test pins the multi-record timeout range, advice, exit code, empty output and one send. The original singleton timeout listener remains unchanged.

## Proof and checks

The new multi-record listener failed red on the prior singleton sentence and passed after the change. Focused listener checks passed for the 429 status edge table under `--max-retries 0`, the conformance backend's 429/503/reset/refusal arms, the new two-record timeout, and the original one-record timeout. Existing unit checks passed for the 429 phrase and stopped-run counts. `cargo clippy --locked --offline -p thinkthen --all-targets -- -D warnings` passed. Pages reported 1 coming and 21 green, tickets reported 0 evidence failures, and diff checks were clean. No provider, full suite, stress or build rung ran.

The measured source total is 81,135 nonblank Rust lines, raised 46 from 81,089 in `sdlc/ratchet.json`. The increase earns one outside-in timeout boundary and the small private renderer needed by the existing per-file cap. I looked first for an existing range-specific formatter: `failure.rs::stopped` already held the required context, so the change reused and extracted it rather than duplicating error state or altering the batch planner. No test was deleted or consolidated.

Recommend closing register 53 after fresh independent review and landing. The coordinator owns the plan and register.

## What the build taught us

The final status alone cannot say how many retries occurred, but the retry loop guarantees the allowed attempts are spent before reporting that status. “After the allowed attempts” is the truthful wording for both zero and several retries. A timeout's useful lever depends on the request's record count, which was already preserved by `BatchFailed`; a new engine field would add no information. Moving the stop renderer into a private module kept that context at the diagnostic boundary and protected singleton wording. The red listener case caught the precise old advice while also proving the command sent one batch request.

## Accepted review and landing

Fresh independent review accepted the functional change at `da606611`, checked zero-retry and exhausted-attempt semantics, the existing failed-request range, singleton and structured failures, and the measured81,135-line source total. Its focused429 and two-record timeout checks passed. The reviewer found only that the source commit message omitted the growth explanation already present above. The coordinator puts that justification and duplication search in the main landing commit that raises its ceiling by46; pushed source history is preserved. Later main edits touched claims and records only, so no additional runtime run is required. Pages, tickets, ratchet and diff checks pass. Register53 closes as the message defect fixed here.
