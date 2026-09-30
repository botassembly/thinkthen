# 0314: Rust owns the result schema

Status: slice 3 built, awaiting code review; slice 4 waits on 0304 slice 3

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
fn DoorReply::new(String, Facts, Option<Vec<AttemptObservation>>) -> Result<DoorReply, Error>
fn RecordObservation::to_json(&self) -> Option<String>
impl Serialize for Counters
impl Serialize for DoorReply
impl Serialize for ErrorKind
impl Serialize for Facts
impl Serialize for Usage
struct DoorReply
```

`Facts` declares its fields in name order, so its serialized keys match today's door bytes. `Counters` declares its fields in the door's `usage` order. `DoorReply` takes the value as JSON text, so no `serde_json` type enters the public API; its `Debug` shows the facts alone. Slice 2 moves the C door onto these types. Slice 1 also added a public `CallError`; slice 2 took it out of the public API, since no surface prints a call error. It is now a `cfg(test)` type in `schema_tests.rs`, and the `callError` definition and its `call-error` corpus case stay.

## Slice 2 build

Branch `ticket/0314-s2-door-typed-replies`.

- Starts from: main `e434f6952`. `call.rs` builds the reply envelope and the usage counters with `format!` and the bare values through `json!`; `failures.rs` builds facts with `json!` and key-by-key inserts.
- Keeps: every door byte, the C ABI, the error code, message, and error-facts path, and the batching and record-array code, which ticket 0304 owns.
- Changes: the door serializes `DoorReply`, `Facts`, and `Counters` with serde, and writes bare values, `filter`, `rank`, and `find` straight from their typed values. The hand envelope, facts builder, usage `format!`, and the `serde_json::Value` writer are deleted.
- Proof: `libraries/c/tests/door/golden.rs` landed first, green on the hand-built code (`4ac8e07cd`), and passes unchanged after the switch. It pins one driver run: every verb, a detailed row, a null answer, a record array, attempts, usage, a per-record annotate failure, and a failed call with its message and error facts, with every optional facts key present. The C door's tests and `types/self-test` against the real door pass.
- Defers: the door has no JSON call error, so `CallError` stays a test type in `schema_tests.rs`, kept for the `callError` definition, until a surface prints one. `annotate`, record arrays, and `relate` still join the JSON text the public types give; turning those into typed rows waits for 0304's single batching path.

## Slice 3 build

Branch `ticket/0314-s3-native-bindings`.

- Starts from: main `cf314e045`. R builds every result, facts, detail, and receipt list field by field with extendr in `calls.rs`, `calls/render.rs`, and `relate.rs`, and sums facts in its own `Counts`. Ruby builds facts, details, completions, and usage hash key by key with magnus in `src/ffi/result.rs` and copies each detail into its own `Detail`. Python keeps a `Facts` class with seven getters and builds each detail with `json!`. Each binding spells the six failure causes itself.
- Keeps: every verb's value, the null-versus-failure marker in annotate cells, error kinds and their retry signal, R's one-based offsets and original positions, Ruby's symbol keys, Python's read-only mappings, and the batching code, which ADR 0111 slice 2 owns.
- Changes: the crate writes a question event as JSON through `RecordObservation::to_json`, from one serde type named `questionObservation` in the generated schema. R, Ruby, and Python read facts, details, and counters as JSON in the host's own reader: `jsonlite`, the `json` library, and `json.loads`. Python's `Facts` class goes; facts are a read-only mapping.
- Proof: `libraries/r/check.sh` passes; conformance pass 50, not run 4 (the fault-injection cases, as before), and case 41 passes, which pins positions `11..20`. `libraries/ruby/check.sh` passes with pinned Ruby 3.4.11; conformance pass 53, not run 1. The Python check's main lane passes 109 tests and conformance pass 53, not run 1; its pandas 2 lane is not run, because uv's offline cache lacks that pin set. The annotate null and failure cells and the error-kind cases pass through all three. Facts are host maps, so an unknown member such as `estimated_cost_usd` reads without error. `sdlc/scripts/test` passes 1,271 tests.
- Public changes, allowed because 0.0.1 is unreleased: `RecordObservation::to_json` and `Serialize` for `Usage` are new in the crate, listed with slice 1's added declarations. Python drops `thinkthen.Facts`; `facts` is a read-only mapping, so absent tokens and model are missing keys. Python details lose the `request_digests` alias and use the Ruby detail shape: the yes probability is a number, and named probabilities are `[name, probability]` pairs. R counts arrive as integers, R facts come from the crate's `Tally`, and a detail's `requests` is a list. Ruby values keep their shape; hash keys come in a new order.
- Line counts against main: R 331 added, 689 deleted; Ruby 120 added, 301 deleted; Python 138 added, 256 deleted. The crate gains 97 lines and its ceiling rises to 106190. Binding ceilings: R Rust 2236 to 1855, Ruby Rust 1434 to 1254, Python Rust 7383 to 7285, Python package 4580 to 4578. R's own code rises 2353 to 2379 and Ruby's 2324 to 2325, because the host now shapes frames and values from the JSON.
- Defers: TypeScript builds the same detail JSON by hand; it moves to `to_json` in slice 4. The Python pandas 2 lane waits for a networked machine.

## What the build taught us

- Core forbade the dynamic-JSON lints, and every derive in core trips them: the derived schema code calls `json!`, `to_value`, and `Map`. Core now lifts `disallowed_methods`, `disallowed_types`, and `disallowed_macros` under `cfg(test)` only. Production builds keep all three forbidden, and `policy.py` still scans every core source, tests included, for file, environment, socket, clock, and process paths. The alternative was hand `JsonSchema` impls for about fifty core types outside core, the hand copy ADR 0112 rejects. Ian can overturn.
- Doc comments become `description` and `title` and stop a container `with` from being transparent. The generator strips both keywords, and wrapper types such as `Labels`, `Record`, and `Probability` carry `schemars(inline)`.
- The annotate member union is `anyOf`, and ADR 0112 section 1 now says so. The derive emits `anyOf`. The union stays unambiguous because no `value` branch is an object and the `failed` branch is a closed object.
- Only corpus-tested constraints survive, as the ADR allows: batch position and batch setting of at least 1, closed `failed`, `annotateFailure`, and `recordError`, and each decision row's answer kind paired with its question's verb. Probability ranges, constant `schema` strings, the cost pattern, and the other closed objects are gone from the file.
- Three corpus examples were stubs no Rust type prints: two `choose` questions without `options` and a relate question without `fields`, `relations`, and `threshold`. They gained those members. Every corpus verdict is unchanged. These edits tighten the corpus under the compatibility rule: the `choose` stubs gained `options`, and the relate stub gained `fields`, `relations`, and `threshold`, so each example now matches a question Rust prints.
- Every definition the corpus and port checks name keeps its name. Unused hand names gave way to derived ones: `judgmentQuestion` is `question`, the five answer definitions are `answer`, `edge` and `recognizedRelation` are `relatedEntityEdge` and `entityEdge`, and `probability`, `annotateQuestion`, and `judgmentFields` are gone. `doorRequest` moved to `question-file.schema.json`; `types/check.py` and the Dart parity script read it there.
- `FindQuestionOwned` and `FindAnswer` dropped their hand `Serialize` for a derive with a serde struct tag; their bytes are unchanged. `QuestionText`, `Meaning`, `Record`, and `RecognizeSpec` print any JSON in the schema. A typed recognize question is a deferred gap. `Json` and `Threshold` are the only hand `JsonSchema` forms; they sit in `schema_forms.rs`, outside `core`, so every test build has them, including the `--no-default-features` package run.
- Proof: the drift test passes; it failed on a one-byte edit of the committed file and on a `probe` field added to `Meta`; `sh specification/fixtures/types/self-test` passes 55 cases against the real door; `sh specification/fixtures/question-file/self-test` passes with `doorRequest`; `cargo tree -e normal` for `crates/thinkthen`, `libraries/c`, and `libraries/python` shows no `schemars`.
