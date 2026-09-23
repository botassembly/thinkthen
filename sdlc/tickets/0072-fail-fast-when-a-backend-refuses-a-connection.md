---
flow: build
priority: 72
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/tests/backend/exchange.rs specification/backends.md sdlc/ratchet.json sdlc/planning
---

# 0072: Fail fast when a backend refuses a connection

Status: landed

## Outcome and authority

A typed refused connection fails after its first attempt even when `--max-retries` is nonzero. The existing safe diagnostic appears promptly. Premature closes and other currently retryable transport failures retain their behavior. Ian authorized bounded tickets in the amended engine queue; ADR 0017 places fast failure among the private controls before recognition and the public API. Main records 0069–0071 for library-team decisions, so this build ticket takes the next collision-free number.

## Current fact and design

`engine/http.rs::io_transport` already maps `ConnectionRefused` to `TransportKind::Refused` and maps `UnexpectedEof`, `ConnectionReset`, `ConnectionAborted`, and `BrokenPipe` to `PrematureClose`. `is_retried` currently retries every `Error::Transport`, so a closed port follows the one- and two-second default waits before printing an actionable refusal.

Make only `TransportKind::Refused` non-retryable. Keep status retries, premature-close retries, timeout/name-lookup/other transport behavior, wait bounds, attempt observation, usage accounting, exit 4, and the exact refusal diagnostic unchanged. The first refused attempt still passes through the existing `before_attempt` observation once. Do not infer retry policy from display text or operating-system strings.

## Scope and exclusions

Allowed: retry classification in `engine/http.rs`, focused unit/integration tests, the retry sentence and refusal paragraph in `specification/backends.md`, exact ratchet and ticket/queue/record updates. Excluded: cancellation/deadlines, width, fork/signal work, transport taxonomy changes, diagnostics, response bodies, counters, cache/recording behavior, dependencies, workflows, surfaces, site, paid calls, and publication. Preserve existing source comments; add documentation only if a changed or new item requires it.

## Acceptance

- Observe a focused red test that `Error::Transport(Refused)` is not retried while `PrematureClose` remains retryable and the six statuses remain unchanged.
- A compiled command aimed at loopback destination port zero uses default retries, exits 4 promptly before the first default retry wait, prints exactly `thinkthen: the backend refused the connection; check that it is running and that --url is correct`, prints no stdout, and reveals neither key nor evidence. Use a robust bound that distinguishes one immediate refusal from the existing three-second waits without claiming a product latency guarantee.
- Existing counted-listener coverage proves close-before-headers retries once and returns `requests_sent:2`; retain it unchanged. Do not use `--max-retries 0` as proof of the new rule.
- Focused engine and backend exchange tests, policy, exact ratchet, formatting, and diff checks pass before code review. Coordinator runs the four local gates sequentially after acceptance. Actions remains disabled.

## Complexity

Contract 0; State/timing 0; Reach 1; Proof 1; Cost of error 1; Total 3. Minimum floor: none. Final level: 2. Reasons: one explicit private retry classification with one public timing effect and deterministic loopback transport proof; no concurrency, persistent state, recovery, cache invalidation, or security boundary changes. Selected implementation: `swe2-implementer` (`swe-2-high`). Independent design/code review: separate `sol-reviewer` sessions. Stop and re-score if the change requires cancellation, accounting semantics, or transport taxonomy changes.

## Review

Independent design review accepted the exact classification, bounded scope, and level-2 SWE-2 route. Independent code review rejected a raced released-port test, then accepted deterministic destination-port-zero proof after the coordinator supplied the complete staged diff and personally ran focused tests and Clippy. The final local ladder passed 625 Rust tests with one intentional ignore, doctests, all replay checks, and nineteen how-tos at exact ratchet 35,974. Landing remains pending.
