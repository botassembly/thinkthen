# 0314: Rust owns the result schema

Status: slice 1 landed; slice 2 ready

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

## Slice 1 build

Branch `ticket/0314-s1-generated-schema`. `crates/thinkthen/src/schema_tests.rs` derives `specification/result.schema.json` from the Rust types and compares it byte for byte. No output byte changes.

### Added public declarations

```text
fn CallError::new(ErrorKind, bool, impl Into<String>) -> CallError
fn DoorReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<DoorReply, Error>
impl Serialize for CallError
impl Serialize for Counters
impl Serialize for DoorReply
impl Serialize for ErrorKind
impl Serialize for Facts
struct CallError
struct DoorReply
```

`Facts` declares its fields in name order, so its serialized keys match today's door bytes. `Counters` declares its fields in the door's `usage` order. `DoorReply` takes the value as JSON text, so no `serde_json` type enters the public API; its `Debug` shows the facts alone. Slice 2 moves the C door onto these types.

## What the build taught us

- Core forbade the dynamic-JSON lints, and every derive in core trips them: the derived schema code calls `json!`, `to_value`, and `Map`. Core now lifts `disallowed_methods`, `disallowed_types`, and `disallowed_macros` under `cfg(test)` only. Production builds keep all three forbidden, and `policy.py` still scans every core source, tests included, for file, environment, socket, clock, and process paths. The alternative was hand `JsonSchema` impls for about fifty core types outside core, the hand copy ADR 0112 rejects. Ian can overturn.
- Doc comments become `description` and `title` and stop a container `with` from being transparent. The generator strips both keywords, and wrapper types such as `Labels`, `Record`, and `Probability` carry `schemars(inline)`.
- The annotate member union is `anyOf`, and ADR 0112 section 1 now says so. The derive emits `anyOf`. The union stays unambiguous because no `value` branch is an object and the `failed` branch is a closed object.
- Only corpus-tested constraints survive, as the ADR allows: batch position and batch setting of at least 1, closed `failed`, `annotateFailure`, and `recordError`, and each decision row's answer kind paired with its question's verb. Probability ranges, constant `schema` strings, the cost pattern, and the other closed objects are gone from the file.
- Three corpus examples were stubs no Rust type prints: two `choose` questions without `options` and a relate question without `fields`, `relations`, and `threshold`. They gained those members. Every corpus verdict is unchanged. These edits tighten the corpus under the compatibility rule: the `choose` stubs gained `options`, and the relate stub gained `fields`, `relations`, and `threshold`, so each example now matches a question Rust prints.
- Every definition the corpus and port checks name keeps its name. Unused hand names gave way to derived ones: `judgmentQuestion` is `question`, the five answer definitions are `answer`, `edge` and `recognizedRelation` are `relatedEntityEdge` and `entityEdge`, and `probability`, `annotateQuestion`, and `judgmentFields` are gone. `doorRequest` moved to `question-file.schema.json`; `types/check.py` and the Dart parity script read it there.
- `FindQuestionOwned` and `FindAnswer` dropped their hand `Serialize` for a derive with a serde struct tag; their bytes are unchanged. `QuestionText`, `Meaning`, `Record`, and `RecognizeSpec` print any JSON in the schema. A typed recognize question is a deferred gap. `Json` and `Threshold` are the only hand `JsonSchema` forms; they sit in `schema_forms.rs`, outside `core`, so every test build has them, including the `--no-default-features` package run.
- Proof: the drift test passes; it failed on a one-byte edit of the committed file and on a `probe` field added to `Meta`; `sh specification/fixtures/types/self-test` passes 55 cases against the real door; `sh specification/fixtures/question-file/self-test` passes with `doorRequest`; `cargo tree -e normal` for `crates/thinkthen`, `libraries/c`, and `libraries/python` shows no `schemars`.
