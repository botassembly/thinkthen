# 0503: Add a bounded session interface for JSON requests

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Implement the experiment-selected narrow engine/session interface with bounded streaming and generated typed consumers.

## Evidence

- Starts from: PM architecture asks3/5 and0470 bounded-feed design.
- Keeps: Whole-set admission where specified, completed prefixes where specified, frozen ABI, cancellation and failures with facts.
- Changes: After0491/0502 decision, settle an ADR for engine/session/result handles and push/read/finish/cancel/free. Declare call once and feed descriptors without whole-array staging. Reuse0470 ownership design. Claim `crates/thinkthen/src/public/**`, `libraries/c/src/**` and `specification/request.schema.json`; adapters migrate in their own tickets.
- Proof: Backpressure, stopped readers, cancellation, failure facts and handle lifetimes pass typed outside-in tests; no references outlive host storage.
- Defers: Business policy and new networking. Size: large ownership/execution change.

## 2026-10-09 amendment

The thin, first-class ruling replaces the fallback mechanics: C-interface languages use this owned session and generated typed results. Follow 0511 admission and coordinate 0513 result presence. Settle the handle ADR before coding, preserve frozen C compatibility, own every input and result buffer, bound input/output queues, and cancel or free without waiting for a blocked provider. Keep whole-set semantics and truthful final failure facts. This shared interface precedes 0516 and family adoption; direct languages can remain directly on Rust. Review this amendment and narrow shared execution and C handle paths per slice.
