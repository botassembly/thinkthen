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

## Outcome

Rust owns the result types. A generator emits each language's typed results from them, with explicit presence wherever missing and explicit null differ. Hand-copied result schemas and readers go.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). 0502 found off-the-shelf schema generators erase missing versus null. Hand-copied readers in Python `_complete.py`, Ruby `complete.rb`, R `complete.R` and TypeScript `_complete.js` and `_complete.d.ts` lacked facts fields added by 0461 and 0468.
- Keeps: The result schema as the published contract, distinct null and failure outcomes, and failure facts.
- Changes: Mark presence explicitly in the Rust result types where it matters. Build one in-repo generator with a template per target: Python stubs and Rust-owned classes, TypeScript declarations, Ruby, R, C#, Java, Kotlin, Scala, Dart, Swift, Go, C++, PHP, Objective-C, plus C, Zig, Ada and COBOL layouts. Readers accept unknown fields. Claim `crates/thinkthen/src/public/results/**`, `specification/result.schema.json` and a new generator folder.
- Proof: One shared conformance fixture with missing, explicit null, failure, unknown field and every function's result passes in every language through its installed package. A check fails when generated output is stale.
- Defers: Per-language async and cleanup idiom (0516, 0504).

## Review amendment

Extend the existing Rust-derived schema pipeline; do not create another schema framework or runtime validator. Keep both complete schemas generated together and reuse `sdlc/scripts/generate-c-header.py`; constrained layouts derive from that header. Preserve missing, null, failures, every known observation and unknown JSON members through ordinary conversion or round trip. Unknown members never activate proxy behavior.

The initial shared slice claims `crates/thinkthen/src/schema_tests.rs`, `crates/thinkthen/src/schema_tests/complete.rs`, `crates/thinkthen/src/public/results/complete_facts.rs`, `specification/result.schema.json`, `crates/thinkthen/src/public/results/complete.schema.json`, and new `sdlc/generators/results/generate.py` with target templates under `sdlc/generators/results/templates/`. Name each template and existing gate entry before implementing that target. Host outputs belong to their migration slices. Amend ADR 0112 sections 4 and 6 under Ian's ruling to replace the typed-host prohibition while retaining schema ownership, versioning and tolerant reading; claim `sdlc/planning/adr/0112-rust-owns-the-result-schema.md`.

## Progress

- 2026-10-09 started
- 2026-10-09 landed aa49e3b32; next: The reviewed C# facts generation slice is landed; run combined qualification with shared admission. Finish authoritative function variants and observations before generating all results and adopting installed bindings.
- 2026-10-09 landed 0d3a61857cbea36661bf4a48b57858fb4ae1cf6d; next: Facts and selected typed answer, error, observation and map generation are landed after fixing schema-derived identity dispatch. Focused checks and branch lint pass. Complete function carriers, Rust presentation gaps, session packet generation and installed language adoption remain open.
- 2026-10-09 landed b7d90f93d425e3e8d89d12aedac56db77a7c813a; next: Concrete atomic serializers and function schemas are landed; focused native, schema, generation and Clippy checks pass. Preserve complete native details next, then generate full function and session views before installed host adoption.
