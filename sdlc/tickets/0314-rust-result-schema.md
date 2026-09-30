# 0314: Rust owns the result schema

Status: design accepted; slice 1 ready

Lane claude-1. Branch `ticket/0314-rust-result-schema`. Design: [ADR 0112](../planning/adr/0112-rust-owns-the-result-schema.md). Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 7 and 8. Builds with [0291](0291-remaining-language-doors.md), so the fourteen C-door bindings change once.

## Outcome

`specification/result.schema.json` is written by a test from the Rust types that serialize every result, and that test fails when the two drift. The C door serializes its reply, facts and call error through those same types. Every port reads results as JSON in its host idiom and adds only three things: named outcome codes, named error kinds, and a null that cannot be mistaken for a failure. Per-port result classes and strict facts validators are gone.

## Evidence

- Starts from: main `f4feeadb5`. ADR 0082 made `result.schema.json` a hand-written 385-line file, checked by `specification/fixtures/types/` against examples and the C door. `spec/result.md` holds a second hand copy of each command's members. The C door builds `{value,facts}` and `facts` by hand with `format!` and `json!` in `libraries/c/src/{call.rs,failures.rs}`. Ports hold their own result and facts parsers, listed in the ADR.
- Keeps: every result byte, `thinkthen.result/1`, the C ABI and its symbols, `THINKTHEN_E*` codes 1 to 6 and `THINKTHEN_YES/NO/UNSURE`, the annotate `{"failed":…}` marker, offsets per surface, the question-file schema and its parser parity corpus, the `jsonschema` test tool, and conformance cases.
- Changes: `schemars =1.2.2` as a dev-dependency only, never with `preserve_order`; `JsonSchema` derived under `cfg(test)` on the result, meta, facts, call-error and door-reply types; `#[schemars(with = …)]` plain forms for the hand-`Serialize` types below; per-verb definitions named in the test from the Rust types that serialize them; one unit test compares the schema and, under `THINKTHEN_WRITE_SCHEMA=1`, rewrites it and still fails; `doorRequest` moves to `question-file.schema.json`; the C door serializes typed replies; ports delete their typed result layers and strict validators and gain the three additions; 0291's plan, deadline, cap and refusal methods land in the same port pass.
- Proof: per slice in the ADR. In short: the drift test fails on an edited schema and on an added field; a golden assertion pins today's door reply and error-facts bytes before the door switches to serde; `types/self-test` passes on the generated schema; every conformance runner passes unchanged; each port's one shared null-versus-failure case and one error-kind case pass; `cargo tree -e normal` shows no `schemars`.
- Defers: a generated question-file schema, per-language label-set types, typed C structs, SQL enums and domains, Pydantic models, frame dtype changes, and the other type-review recommendations.

## Design notes

Hand-`Serialize` types on the result path, each needing a `with` form or, last, a hand impl: `Labels`, `QuestionText`, `Description`, `Meaning`, `Threshold`, `Distribution`, `TagProbabilities`, `Odds`, `NamedValues`, `NamedAnswers`, `FindAnswer`, `FindQuestionOwned`, `ProfileName`, `AnnotatedRecord`, `Json`, `Pointer`. Per-verb definitions come from `Option<bool>` (`decide`), `Option<String>` (`choose`, `find`), `Vec<String>` (`tag`), `f64` (`score`) and the record list types `filter` and `rank` print. Slice order: 1 schema, 2 C door, 3 R, Ruby and Python, 4 port families with 0291 after ADR 0111 slice 3.
