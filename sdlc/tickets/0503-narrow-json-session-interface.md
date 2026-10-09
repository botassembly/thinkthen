# 0503: Add a bounded session interface for JSON requests

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd, accept

Reviews: revision e8f546aeca5644f3703f61cf6c6cf51b3038e0d6, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

C-interface languages call one owned, bounded engine session. A caller declares a call once, feeds descriptors without staging the whole input, reads results, and finishes, cancels or frees the session. Cancel and free return promptly without waiting for a blocked provider. Final facts stay truthful and arrive only when the work actually settles.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). [ADR 0129](../planning/adr/0129-owned-json-sessions.md) owns session execution and lifetime rules. The 0470 bounded-feed design supplies the ownership pattern.
- Keeps: Whole-set admission and completed prefixes where specified, the frozen 0.1 C ABI, cancellation, and failures with facts.
- Changes: Implement the owned session under ADR 0129: engine, session and result handles with push, read, finish, cancel and free. The session owns every input and result buffer and bounds its input and output queues. One writer at a time handles descriptor admission and sink dispatch. Admission follows 0511. 0513 supplies the result packet graph; native session work need not wait for every target generator. Claim `crates/thinkthen/src/public/**`, `libraries/c/src/**` and `specification/request.schema.json`, narrowed per slice.
  - 0470 consumes this session for DuckDB and builds no second queue or C handle family.
  - 0516 designs the first host wait and cancellation over this interface. Other hosts reuse that contract through their own scheduling idiom.
  - Direct-to-Rust languages stay on Rust.
- Proof: Typed outside-in tests cover backpressure, stopped readers, cancellation, failure facts and handle lifetimes. No reference outlives host storage. The tests distinguish prompt caller cancellation from eventual provider cleanup and final facts.
- Defers: Host adoption belongs to 0516 and the language migrations. Business policy and new networking need no ticket.

## Progress

- 2026-10-09 landed f6768b855; next: Slice A settles ADR0129 after a real producer-closure correction and fresh acceptance. No runtime/session implementation is claimed. Implement the reviewed owned bounded session after shared admission and generated result-view interfaces; preserve frozen C compatibility and prompt caller cancellation/free.
- 2026-10-09 landed cb1c47896; next: Accepted observation packets preserve detailed native events through the existing bounded queue. Implement shared session ownership and sink dispatch after shared descriptor admission; frozen C compatibility and prompt cancel/free remain required.
