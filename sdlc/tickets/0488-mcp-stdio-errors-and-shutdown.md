# 0488 — mcp-stdio-errors-and-shutdown

Status: OPEN.

Milestone: 0.3

Reviews: revision b9027d08b, accept

## Outcome

Resolve MCP error-ID and stdin-close behavior against the admitted protocol before changing replies or shutdown semantics.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, ask 6; runtime/protocol paths can emit null IDs and cancel queued work or discard a partial final line at EOF. Source behavior is confirmed; a violation of the admitted protocol is not yet established.
- Keeps: Framed stdio, cancellation/shutdown guarantees, ten tools, current installed conformance and no administration endpoint.
- Changes: Compare the admitted specification and actual public tests with readable-ID errors, queued calls and complete/partial EOF lines. Specify the chosen behavior and only then implement any confirmed contract correction. Keep this in 0.3 unless a reproducible 0.2 conformance breach warrants promotion through PM.
- Proof: Owned framed client cases cover valid/invalid IDs and EOF while work is queued, without paid calls. No arbitrary protocol expectation or silent entity/data loss.
- Defers: 0.2 implementation absent a confirmed contract breach; HTTP services, new tools and business logic.
