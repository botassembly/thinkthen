# 0700 — Token-limit refusal advice fits the command

Status: OPEN.

Milestone: 0.2.1

Reviews: revision d253870, accept

## Outcome

- Changes: `crates/thinkthen/src/cli/failure/status.rs`, `crates/thinkthen/src/cli/failure.rs`, `crates/thinkthen/tests/backend/status_reason.rs`.
- When a backend refuses a request as too large (status 400, `max_tokens_exceeded`, or the generic too-large refusal), the message names only remedies the failing command accepts and that would change the outcome. A command that splits its input into several requests suggests a lower `--max-request-bytes`. A command that must send its whole input in one request, such as `find` over one candidate set, suggests shortening the text or using fewer candidates, and does not suggest a request-size flag that would only refuse locally.

### Proof

- A loopback test drives each command that can hit the refusal (`judge`, `find`, `annotate`, `relate` and any other that prints `TOKEN_LIMIT` today) against a backend that answers status 400 with `max_tokens_exceeded`, and pins the exact sentence. Each named flag appears in that command's `--help`. The test fails before the change for at least `find`.

## Evidence

Research ran released 0.1.0 `find` on 21 JSONL candidates, 86,552 bytes. The plan estimated 43,297 to 76,190 input tokens; the hosted TypeSafe backend refused with `max_tokens_exceeded`. The message said to "shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile", but 0.1.0 `find --help` has no `--max-request-bytes`. Main has since added the flag to `find`, but `find` sends one request for the whole set, so a lower request size only turns the refusal into a local one. The one message in `cli/failure/status.rs` serves every command.

A preflight that compares the estimate with the model's input limit before sending belongs with backend capabilities as configuration (0594), which holds each model's context limit as data.

## Progress

- 2026-10-19 started
