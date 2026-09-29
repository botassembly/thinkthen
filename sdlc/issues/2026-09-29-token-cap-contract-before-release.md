# Settle the token-cap contract before the public release

Status: Open. Ian requested a review of experiment2038's follow-up recommendations and asked to settle useful public API changes before release. The independent assessment found no existing issue owning this distinct contract. Experiment2038 `questions.md`, ruling6, accepts a no-send preview and process request cap for0.1 and names the token cap as the first follow-up. This issue records that follow-up; it does not silently replace the request cap with an unproved spend guarantee.

## Problem

Requests vary in size, so `max_requests_total` cannot bound input tokens or money. The current send budget atomically reserves actual attempts, including retries. Cache/replay answers send nothing. Reported token usage arrives after a response and may be absent, including on failed requests. Output tokens are unknown before the call. A process or PostgreSQL backend limit also differs from an account-wide limit.

Relevant source at main `9018c1e4c`: `crates/thinkthen/src/public/options/budget.rs`, `crates/thinkthen/src/engine/request.rs`, and `crates/thinkthen/src/engine/usage/facts.rs`. The accepted request-cap/tally draft is0289. The [follow-up assessment](../records/2026-09-29-sql-redesign-followup-assessment.md) maps adjacent work without duplicating it.

## Outcome

Write a reviewed contract before exposing a token-limit setting. Compare conservative input reservation, observed usage limits and any provider-supported output bound. Name what can actually be guaranteed and the quantity's units, process scope, estimator/version, reset/lifetime, refusal kind, and behavior when usage is missing. Distinguish estimated input from reported input/output and caller-priced estimated cost. Preserve the separate request cap.

Specify reservation and settlement for concurrent calls, retries, refusal splits, cache/replay, failed or unreported billable attempts and in-flight output. State any allowed overshoot explicitly; never call a post-response observation an exact pre-send token or dollar ceiling. Prefer the smallest honest user-facing API and the same core enforcement path across surfaces. No account-level guarantee follows from local counters.

## Proof and routing

Schedule the API/design decision beside the SQL/frame foundation; implementation follows0289's accepted reservation/tally work. The coordinator may assign0299 after the design scope is prepared. Current0.1 request-cap and preview work need not wait for speculative token enforcement. Any change to Ian's recorded first-follow-up release boundary needs an explicit reviewed disposition, not an assumed deadline.

Use one bounded loopback table with independently counted attempts: two concurrent reservations, a retry/split boundary, cache/replay with zero sends, absent usage, a failed response, and output arriving after admission. Derive expected values independently of the implementation. Do not repeat the full table in every wrapper or run a paid/saturation campaign. Implementation closure requires code, review and matching host conversion proof; an accepted design alone does not close this issue.
