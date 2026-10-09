# 0503: Add a bounded session interface for JSON requests

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd, accept

Reviews: revision e8f546aeca5644f3703f61cf6c6cf51b3038e0d6, accept

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

## Surface assessment amendment

ADR 0129 owns shared session execution and lifetime rules. Its packet graph is supplied by 0513 before typed adoption; native session work need not wait for every target generator. 0470 consumes this session for DuckDB and must not build another queue or C handle family. Keep descriptor admission and sink dispatch under one writer at a time.

The native cancel/free guarantee concerns prompt caller return; terminal facts still require actual settlement. 0516 owns the first host waiting and cancellation design over this interface. Its installed task-boundary proof must distinguish prompt cancellation from eventual provider cleanup and final facts. Other hosts reuse that contract through their own scheduling idiom.

## Progress

- 2026-10-09 landed f6768b855; next: Slice A settles ADR0129 after a real producer-closure correction and fresh acceptance. No runtime/session implementation is claimed. Implement the reviewed owned bounded session after shared admission and generated result-view interfaces; preserve frozen C compatibility and prompt caller cancellation/free.
- 2026-10-09 landed cb1c47896; next: Accepted observation packets preserve detailed native events through the existing bounded queue. Implement shared session ownership and sink dispatch after shared descriptor admission; frozen C compatibility and prompt cancel/free remain required.
