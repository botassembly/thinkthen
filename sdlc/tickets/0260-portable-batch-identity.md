---
flow: build
priority: 260
opens: sdlc/records/2026-09-28-content-batch-identity-preparation.md
---

# 0260: Pin portable record spelling and content-batch identity

Status: core, CLI and C proof accepted at `3a194a88`; TypeScript, Ruby and R proof accepted at `73959b19`. SQL host proof accepted at `068fd3fd`; public Rust, Python/frame and Rust Polars proof accepted at `cc3a535c`. These slices are integrated; C-wrapper host proof is open. Register64 is not complete. The accepted design `f68d9496` retains the existing runtime contract. The build record lists the finite remaining public routes.

## Outcome

A reader can spell a supported selected record as the Rust core does, determine its content cut from literal UTF-8 bytes, and reproduce every packed request digest at one resolved backend address. The shared fixture pins ordered five-text membership `[1,2]`, `[3,4]`, `[5]`, each close reason, each actual body and digest, and three actual sends. Its first outside-in proof runs the compiled CLI and the C JSON door with the same accepted text values through distinct input encodings. Register 64 closes only after the applicable host runners consume the shared case or a documented shared C corpus plus per-wrapper forwarding proof covers them. A two-host pass alone is an incremental checkpoint.

## Evidence

- Starts from: Original `experiments/284-issue-register/64-cross-language-batch-identity.md`, accepted preparation `77a44b94`, ADR 0048 items 1, 2 and 5, and main `0954b1e8`. The prior note's five-text hashes were independent candidate calculations; the preflight confirms source and a dry-run observation without claiming a new built test.
- Keeps: Existing scalar and text entrypoints, each host's accepted input domain, compact Unicode spelling, JSON object order and finite numeric distinctions, byte-identical singleton requests, and digest/cache/replay identity over the exact encoded request. No host gets a second batch hasher.
- Changes: The record spelling rule becomes explicit in the settled records contract and a single checked-in fixture records independent literal record bytes, cut heads and decisions, ordered groups, full request bytes and fixed-address digests. Focused runners consume those expectations at real CLI and host entrypoints, then the applicable remaining surfaces.
- Proof: Five complete text records at Max, fixed question/model/address, no context/cache/retry, and generous limits. Assert five accepted rows, ordered membership and content/content/end closes, three captured sends, exact listener body bytes and independently fixed digest per request; compare returned request identities to fixed digests where the public result exposes them. PostgreSQL array rows and SQLite warm expose no digest: their exact captured bodies, ordered membership and counted sends prove grouping; digest identity follows by explicitly qualified source inference from the shared Exchange path. Do not add SQL metadata solely for this proof. One structured CLI-only arm pins member order, nested values, Unicode, escapes, `1`/`1.0`, `-0.0` and finite exponent spelling without assigning those object values to text-only hosts. Retain existing singleton, batching, replay and per-host fixtures.
- Defers: The remaining 11 C-wrapper host package routes; accepted native, Rust, Python/frame, Polars and SQL source-route proofs keep their recorded installed-target limits; register 64 stays open until their shared-corpus proof is invoked. Structured inputs remain nonapplicable on text-only hosts. Provider, stress, answer-accuracy, token-economy and all-platform release proof remain separate. The wire-duplication issue and register 19 answer parity are not closed by this ticket.

## Build boundaries

First add a compact record-spelling subsection to `specification/records.md`, linked from the batching fixture page, and literal fixtures under `specification/fixtures/batching/`. Reuse the core batch test and `crates/thinkthen/tests/backend/batching.rs`; add a focused CLI listener assertion there or in one small adjacent file. Reuse `libraries/c/tests/door` and its compiled driver for the C JSON-array route. Keep its large `cases.rs` to a small module declaration and put new proof in an adjacent file. The [preflight](../records/0260-portable-batch-identity-preflight.md) lists host runner follow-ups, measured source caps, current pins and the focused gate route. The coordinator must claim these source/spec files when other lanes release them.

If a fixture exposes a disagreement within an accepted domain, stop and name the changed wire/cache behavior as a separate risk for fresh High design review before altering `core/json.rs`, `core/render.rs`, `core/batch/questions.rs`, the System One encoder or host conversions. A documentation and outside-in proof build that retains their behavior needs ordinary fresh code review. Before landing, add `## What the build taught us` with confirmed surprises and remaining host obligations.

## What the build taught us

The independently written five-text heads and manually fixed request bodies matched the current core, compiled CLI and C JSON door. Escaped `\u00e9` on CLI JSONL normalized to literal UTF-8 `é`, so the first cut and all three request bodies stayed the same; hashing that escape spelling directly would move the cut. The separate structured JSONL arm kept nested member order, `1` versus `1.0`, negative zero and exponent spelling, with the integer record closing its batch.

The C conformance case loop forces batch 1, so the Max proof belongs in a focused adjacent door test. The detailed `meta.requests` field is an array of digest strings, which corrected an initial test assertion. The sandbox blocked local listener binding on the first CLI run; a focused offline rerun with local loopback permission passed. No production encoder, cache or adapter changed. [The build record](../records/0260-portable-batch-identity-build.md) lists exact checks, ratchet growth and every remaining host package. Register 64 stays open after this first slice.

Fresh code review found that the routine root test script did not invoke the new core and CLI cases, and that their CLI/C digest checks copied helpers the existing harnesses already supplied. The correction adds four exact routine selectors and reuses those helpers. It lowers the measured root and C ratchets by ten and eleven lines respectively without changing the literal fixture or product behavior.

The native host slice reuses the literal corpus and existing capture helpers. TypeScript, Ruby and R each return five ordered rows, three exact bodies and the expected request identities. Fresh review reran all three cases and the retained R facts case. Its [record](../records/0260-native-host-proof.md) distinguishes source-tree proof from installed-target qualification.

Fresh SQL review accepted the existing PostgreSQL array and SQLite warm result shapes. Their exact three captured body files and send counts prove membership; neither public result returns a request digest. DuckDB returns and checks the five row identities directly. The SQL [record](../records/0260-sql-host-proof.md) labels that difference. Review also caught an omitted installed DuckDB selector. Corrected `068fd3fd` adds it and retains the passing focused installed-copy case. This is a proof clarification with no SQL API change.

The public Rust and Python/Polars slice passed fresh review at `cc3a535c`. Its [record](../records/0260-python-rust-host-proof.md) pins the separate question-set frame bodies alongside the existing decide bodies. Both carry the same five text cells and three content groups. The reviewer checked row digests, actual selectors, file caps and precise source-tree limits. Earlier native artifact provenance was unconfirmed; fresh rebuilds establish current proof without implying that a timestamp alone proves a source mismatch.
