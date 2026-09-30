# 0314: Rust owns the result schema

Status: ready

Lane claude-1. Branch `ticket/0314-rust-result-schema`. Design: [ADR 0112](../planning/adr/0112-rust-owns-the-result-schema.md). Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 7 and 8. Builds with [0291](0291-remaining-language-doors.md), so the fourteen C-door bindings change once.

## Outcome

`specification/result.schema.json` is written by a test from the Rust types that serialize every result, and that test fails when the two drift. The C door serializes its reply, facts and call error through those same types. Every port reads results as JSON in its host idiom and adds only three things: named outcome codes, named error kinds, and a null that cannot be mistaken for a failure. Per-port result classes and strict facts validators are gone.

## Evidence

- Starts from: main `f4feeadb5`. ADR 0082 made `result.schema.json` a hand-written 385-line file, checked by `specification/fixtures/types/` against examples and the C door. `spec/result.md` holds a second hand copy of each command's members. The C door builds `{value,facts}` and `facts` by hand with `format!` and `json!` in `libraries/c/src/{call.rs,failures.rs}`. Ports hold their own result and facts parsers, listed in the ADR.
- Keeps: every result byte, `thinkthen.result/1`, the C ABI and its symbols, `THINKTHEN_E*` codes 1 to 6 and `THINKTHEN_YES/NO/UNSURE`, the annotate `{"failed":…}` marker, offsets per surface, the question-file schema and its parser parity corpus, the `jsonschema` test tool, and conformance cases.
- Changes: `schemars` as a dev-dependency only; `JsonSchema` derived under `cfg(test)` on the result, meta, facts, call-error and door-reply types; one unit test writes and compares the schema; `doorRequest` moves to `question-file.schema.json`; the C door serializes typed replies; ports delete their typed result layers and strict validators and gain the three additions; 0291's plan, deadline, cap and refusal methods land in the same port pass.
- Proof: per slice in the ADR. In short: the drift test fails on an edited schema and on an added field; `types/self-test` passes on the generated schema; every conformance runner passes unchanged; each port's one shared null-versus-failure case and one error-kind case pass; `cargo tree -e normal` shows no `schemars`.
- Defers: a generated question-file schema, per-language label-set types, typed C structs, SQL enums and domains, Pydantic models, frame dtype changes, and the other type-review recommendations.
