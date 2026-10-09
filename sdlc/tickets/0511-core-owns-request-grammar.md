# 0511: Let core own the whole request grammar

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

One public core module admits every request: inputs, question files, label descriptions, limits, defaults and validation. Every surface calls it. No binding, extension or host package restates a rule.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The audits found the complete-call input grammar in `databases/sqlite/src/complete_native` and `libraries/r/thinkthen/src/rust/src/complete`, pulled into other crates by `#[path]` includes; the C door's verb-key grammar in `libraries/c/src/call.rs`; find and relate limits restated in DuckDB, SQLite, PostgreSQL and C; the label grammar restated in Ada `thinkthen.adb` and COBOL `tt_shape.c`; settings, batch, deadline and relate-rule checks restated in Ruby, R and TypeScript.
- Keeps: The 0491 Request contract, its generated schema, error kinds and every current accepted input.
- Changes: Move the shared grammar into a public core module and delete each restated copy so core refusals reach the caller. Remove every `#[path]` include across crates. Claim `crates/thinkthen/src/public/request/**` plus the named duplicate files; narrow per slice.
- Proof: A lint fails when a binding source names a limit constant or a `#[path]` reaches outside its crate. Shared conformance cases drive one over-limit and one malformed input through each surface and get the same core error kind.
- Defers: Result reading (0513) and the CLI and MCP pipelines (0512).

## Review amendment

This amendment governs the earlier wording. One public Request edge admits requests through shared Rust grammar, limits, defaults and validation. Pure core retains typed semantic rules and imports no Request, decoder, reader or runtime handle, preserving ADR 0125. Surfaces retain host conversion, pointer representability and file-authority checks without restating engine semantic limits.

The first slice removes duplicate SQL record-descriptor validation through existing Request conversion and admission. Claim `crates/thinkthen/src/public/request/input.rs`, `crates/thinkthen/src/public/request/composition.rs`, `crates/thinkthen/src/public/request/transport.rs`, `databases/sqlite/src/complete_native/inputs.rs`, `databases/sqlite/src/complete/request.rs` and `crates/thinkthen/tests/request_contract/projections.rs`. Name later consumers before their slices; remove cross-crate includes after those consumers migrate. Retain legacy C translation, accepted duplicate-key behavior and diagnostics. The requested lint detects restated engine semantic limits, not legitimate host checks or any mention of a constant.
