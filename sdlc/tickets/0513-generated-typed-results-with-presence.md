# 0513: Generate each language's typed results from the Rust result types

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision ff51c55756dcac15ab199beb8210b9034587c79e, reject

Reviews: revision 6f89cdf1b3949ce57bff66bb147a998ecd3cc7f9, accept

Reviews: revision 41c61e7e979ec306ad216bfa63ad7733e576cd1c, reject

Reviews: revision 7a1f13950217a608f2d54830a7b53ddad7f846c1, accept

Reviews: revision af05f73ad0f67267cf9d58174449e53689ddb823, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Rust owns the result types, and its serializers emit every fact the native values hold. One in-repo generator turns that complete semantic graph into typed results, with explicit presence wherever missing and explicit null differ. This ticket delivers the shared graph, the generator mechanics and the generated C# reference that 0516 needs. Each host migration adds its own target template and adopts the output.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 1 of [the surface assessment](../records/0521-surface-contract-assessment.md). 0502 found that off-the-shelf schema generators erase missing versus null. Hand-copied readers in Python `_complete.py`, Ruby `complete.rb`, R `complete.R` and TypeScript `_complete.js` and `_complete.d.ts` lacked facts added by 0461 and 0468. [The 0513 record](../records/0513-generated-results.md) names native fields that the annotation and relation serializers omit.
- Keeps: The result schema as the published contract, its versioning and tolerant reading. Distinct null and failure outcomes, failure facts, every known observation, and unknown JSON members through ordinary conversion or round trip. Unknown members never activate proxy behavior.
- Changes: Extend the existing Rust-derived schema pipeline. Create no other schema framework or runtime validator. Keep both complete schemas generated together and reuse `sdlc/scripts/generate-c-header.py`. Slices, in order:
  - Shared generation and C# facts. Claim `crates/thinkthen/src/schema_tests.rs`, `crates/thinkthen/src/schema_tests/complete.rs`, `crates/thinkthen/src/public/results/complete_facts.rs`, `specification/result.schema.json`, `crates/thinkthen/src/public/results/complete.schema.json`, `sdlc/generators/results/generate.py` and templates under `sdlc/generators/results/templates/`. Name each template and gate entry before implementing it.
  - Native presentation repairs. Claim the annotation and relation serializers under `crates/thinkthen/src/core/result/complete/` and the question presentation paths before the slice. Preserve native sources, observations, partial usage, failed-member thresholds, described labels, authored readings and located ordinals.
  - Complete function and session views. The request or session function selects the atomic result type, because an unrestricted object shape cannot identify the function. Supply the complete graph to 0503's session and 0505's C views. Keep this work off the 0516 prerequisites where it is not needed.
  - Amend sections 4 and 6 of `sdlc/planning/adr/0112-rust-owns-the-result-schema.md` to replace the typed-host prohibition.
- Proof: Extend the existing native-to-generated cases with each repaired loss. A schema round trip alone cannot prove a fact its serializer omitted. The generated C# reference reads the shared fixture with missing, explicit null, failure, unknown field and every function's result. The generator fails on stale output and refuses unsupported known forms.
- Defers: Other target templates, host outputs, async and installed adoption go to 0494–0498, 0504, 0505, 0516, 0518 and 0522–0529. Host scheduling belongs to each migration, and Python async to 0496.

## Progress

- 2026-10-09 started
- 2026-10-09 landed aa49e3b32; next: The reviewed C# facts generation slice is landed; run combined qualification with shared admission. Finish authoritative function variants and observations before generating all results and adopting installed bindings.
- 2026-10-09 landed 0d3a61857cbea36661bf4a48b57858fb4ae1cf6d; next: Facts and selected typed answer, error, observation and map generation are landed after fixing schema-derived identity dispatch. Focused checks and branch lint pass. Complete function carriers, Rust presentation gaps, session packet generation and installed language adoption remain open.
- 2026-10-09 landed b7d90f93d425e3e8d89d12aedac56db77a7c813a; next: Concrete atomic serializers and function schemas are landed; focused native, schema, generation and Clippy checks pass. Preserve complete native details next, then generate full function and session views before installed host adoption.
