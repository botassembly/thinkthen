# 0511: Let core own the whole request grammar

Status: OPEN.

Milestone: 0.2

## Outcome

One public core module admits every request: inputs, question files, label descriptions, limits, defaults and validation. Every surface calls it. No binding, extension or host package restates a rule.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The audits found the complete-call input grammar in `databases/sqlite/src/complete_native` and `libraries/r/thinkthen/src/rust/src/complete`, pulled into other crates by `#[path]` includes; the C door's verb-key grammar in `libraries/c/src/call.rs`; find and relate limits restated in DuckDB, SQLite, PostgreSQL and C; the label grammar restated in Ada `thinkthen.adb` and COBOL `tt_shape.c`; settings, batch, deadline and relate-rule checks restated in Ruby, R and TypeScript.
- Keeps: The 0491 Request contract, its generated schema, error kinds and every current accepted input.
- Changes: Move the shared grammar into a public core module and delete each restated copy so core refusals reach the caller. Remove every `#[path]` include across crates. Claim `crates/thinkthen/src/public/request/**` plus the named duplicate files; narrow per slice.
- Proof: A lint fails when a binding source names a limit constant or a `#[path]` reaches outside its crate. Shared conformance cases drive one over-limit and one malformed input through each surface and get the same core error kind.
- Defers: Result reading (0513) and the CLI and MCP pipelines (0512).
