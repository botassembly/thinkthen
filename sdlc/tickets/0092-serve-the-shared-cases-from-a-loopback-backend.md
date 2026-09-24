---
flow: build
priority: 92
opens: conformance crates/thinkthen/tests/backend Cargo.toml sdlc/scripts sdlc/ratchet.json
---

# 0092: Serve the shared cases from a loopback backend

Status: draft, design not reviewed. Owner: Claude.

## Outcome and authority

Give every binding one offline backend that its tests can start from any language. It replaces the branch's `ENGINE_NULL` and `THINKTHEN_NULL` null backends and its `synthetic-partial` feature, which the real engine does not have (gap G10, question 10 of `sdlc/planning/surfaces-port-guide.md`). The binding points `base_url` at it, so the real transport path runs. No public fault hook or null backend enters `thinkthen` (ticket 0095). Ian can overturn the tool's form.

## Shape

- One unpublished workspace binary, `conformance/backend`, built from the loopback harness in `crates/thinkthen/tests/backend/harness`. The harness moves there, and the crate's tests use it as a dev-dependency, so one harness remains.
- It binds `127.0.0.1` on an ephemeral port, prints the port on its first line, and exits when its standard input closes, so it never outlives a test.
- It answers an exact request body with the case's response from `conformance/cases.json` and the recognize fixture. An unknown body gets a distinct 5xx and a line on standard error, so a drifted request fails loud.
- Fault arms, chosen per connection by case id: reset after the body is read (0089), 429 and 503 with a retry delay, a held reply released by a line on standard input (deadline and cancel), and a malformed reply for each of the six `FailureCause` values.
- It counts requests and prints the count on exit, so a test can prove "sends nothing" by count.

## Acceptance

- The command runner passes every shared case against this backend, the same as against in-process replay.
- Each fault arm yields its expected error kind and cause through the command.
- A test proves it opens no socket beyond loopback and leaves no process behind after standard input closes.
- Production code in `thinkthen` is unchanged. The binary and moved harness stay under 450 nonblank Rust lines, net of the harness lines removed from the tests. No new dependency beyond what the harness uses.

## Dependencies

After 0091. Before 0086, whose external consumer crate runs the shared cases through it, and before every surface ticket.

## Complexity

Contract 1; state and timing 2; reach 2; proof 2; cost of error 1; total 8. Final level: 2.

## Review

- Design review: pending.
- Code review: pending.
