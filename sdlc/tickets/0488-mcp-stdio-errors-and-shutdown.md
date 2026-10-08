# 0488: Preserve MCP errors and shutdown behavior

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c5b73b5ca04203ca5b5d6a6c91c14d31d8722ed3, accept

## Outcome

Resolve MCP error-ID and stdin-close behavior against the admitted protocol before changing replies or shutdown semantics.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, ask 6; runtime/protocol paths can emit null IDs and cancel queued work or discard a partial final line at EOF. Source behavior is confirmed; a violation of the admitted protocol is not yet established.
- Keeps: Framed stdio, cancellation/shutdown guarantees, ten tools, current installed conformance and no administration endpoint.
- Changes: Compare the admitted specification and actual public tests with readable-ID errors, queued calls and complete/partial EOF lines. Specify the chosen behavior and implement confirmed contract corrections in 0.2 under Ian's 2026-10-08 ruling. Claim `crates/thinkthen/src/mcp/**` and affected framed-client tests.
- Proof: Owned framed client cases cover valid/invalid IDs and EOF while work is queued, without paid calls. No arbitrary protocol expectation or silent entity/data loss.
- Defers: HTTP services, new tools and proxy business logic.

## Progress

- 2026-10-08 started
