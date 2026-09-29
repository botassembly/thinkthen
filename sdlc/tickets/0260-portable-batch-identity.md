---
flow: build
priority: 260
opens: sdlc/records/2026-09-28-content-batch-identity-preparation.md
---

# 0260: Pin portable record spelling and content-batch identity

Status: design proposed for fresh review. Register 64 remains open. The [design](../records/0260-portable-batch-identity-design.md) and [preflight](../records/0260-portable-batch-identity-preflight.md) specify the bounded first slice and the finite remaining proof. This ticket authorizes implementation only after the coordinator accepts the design and claims the implementation files.

## Outcome

A reader can spell a supported selected record as the Rust core does, determine its content cut from literal UTF-8 bytes, and reproduce every packed request digest at one resolved backend address. The shared fixture pins ordered five-text membership `[1,2]`, `[3,4]`, `[5]`, each close reason, each actual body and digest, and three actual sends. Its first outside-in proof runs the compiled CLI and the C JSON door with the same accepted text values through distinct input encodings. Register 64 closes only after the applicable host runners consume the shared case or a documented shared C corpus plus per-wrapper forwarding proof covers them. A two-host pass alone is an incremental checkpoint.

## Evidence

- Starts from: Original `experiments/284-issue-register/64-cross-language-batch-identity.md`, accepted preparation `77a44b94`, ADR 0048 items 1, 2 and 5, and main `0954b1e8`. The prior note's five-text hashes were independent candidate calculations; the preflight confirms source and a dry-run observation without claiming a new built test.
- Keeps: Existing scalar and text entrypoints, each host's accepted input domain, compact Unicode spelling, JSON object order and finite numeric distinctions, byte-identical singleton requests, and digest/cache/replay identity over the exact encoded request. No host gets a second batch hasher.
- Changes: The record spelling rule becomes explicit in the settled records contract and a single checked-in fixture records independent literal record bytes, cut heads and decisions, ordered groups, full request bytes and fixed-address digests. Focused runners consume those expectations at real CLI and host entrypoints, then the applicable remaining surfaces.
- Proof: Five complete text records at Max, fixed question/model/address, no context/cache/retry, and generous limits. Assert five accepted rows, ordered membership and content/content/end closes, three captured sends, exact listener body bytes and independently fixed digest per request; compare returned request identities to fixed digests. One structured CLI-only arm pins member order, nested values, Unicode, escapes, `1`/`1.0`, `-0.0` and finite exponent spelling without assigning those object values to text-only hosts. Retain existing singleton, batching, replay and per-host fixtures.
- Defers: Nonapplicable structured inputs on text-only hosts; a second hasher in another language; provider, stress, answer-accuracy and token-economy claims; all-platform release proof, which remains with each package/release ticket. The separate wire-duplication issue and register 19 answer parity are not closed by this ticket.

## Build boundaries

First add a compact record-spelling subsection to `specification/records.md`, linked from the batching fixture page, and literal fixtures under `specification/fixtures/batching/`. Reuse the core batch test and `crates/thinkthen/tests/backend/batching.rs`; add a focused CLI listener assertion there or in one small adjacent file. Reuse `libraries/c/tests/door` and its compiled driver for the C JSON-array route. Keep its large `cases.rs` to a small module declaration and put new proof in an adjacent file. The [preflight](../records/0260-portable-batch-identity-preflight.md) lists host runner follow-ups, measured source caps, current pins and the focused gate route. The coordinator must claim these source/spec files when other lanes release them.

If a fixture exposes a disagreement within an accepted domain, stop and name the changed wire/cache behavior as a separate risk for fresh High design review before altering `core/json.rs`, `core/render.rs`, `core/batch/questions.rs`, the System One encoder or host conversions. A documentation and outside-in proof build that retains their behavior needs ordinary fresh code review. Before landing, add `## What the build taught us` with confirmed surprises and remaining host obligations.
