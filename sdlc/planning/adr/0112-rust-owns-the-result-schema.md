# ADR 0112: Rust owns the result schema

- Status: **Accepted** by the coordinator, 2026-09-30, after a fresh design review. Ian can overturn each item.
- Date: 2026-09-30

This ADR builds Ian's ruling 7 of 2026-09-29 in `sdlc/planning/cleanup-2026-09-30.md`: Rust owns every type, surfaces read results as JSON described by one schema generated from Rust, and ports add only named outcome and error codes and a null that cannot be mistaken for a failure. Ruling 8 limits the rest. It amends ADR 0082 and carries deferred ticket 0291, so the fourteen C-door bindings change once. Ticket 0314 builds it.

## Context

ADR 0082 made `specification/result.schema.json` a hand-written file of 385 lines. `specification/fixtures/types/` checks it against a corpus and the C door, and eleven ports replay that corpus through their `public_types.py` or `type_cases.py`. `spec/result.md` holds a second hand copy of each command's members. Every addition, such as `estimated_cost_usd`, is written three times: in Rust, in the schema and in the ports.

The Rust side already serializes every result with `serde`: `DecisionResult`, `AnnotateResult`, `Meta`, `Answer`, `Value`, `AnnotatedValue` and the recognize and relate details in `core/result*`, `core/answer.rs`, `core/recognize.rs`, `core/relation.rs` and `core/relate_file.rs`. Some types write themselves with a hand `Serialize` impl, such as `NamedValues`, `NamedAnswers`, `Distribution` and `Threshold`. Two outputs have no Rust type at all. The C door builds `{"value":…,"facts":…}` with `format!` in `libraries/c/src/call.rs`, and builds `facts` with `json!` in `libraries/c/src/failures.rs`.

The ports then parse that JSON again, in shapes of their own:

| Port | Hand-kept result layer today |
| --- | --- |
| Ada | `src/thinkthen.adb` (832 lines) has its own JSON validator and a closed facts member set; `thinkthen.ads` declares `Run_Facts`, `JSON_Result`, `Entity`, `Relation_Edge`, `Label_Set` |
| C++ | `door.hpp` declares `CallFacts`, `Entity`, `Recognized`, `RelatedEntity`, `Edge`; `json.hpp` (234 lines) is a strict decoder |
| COBOL | `tt_shape.c`, `TTJSON.c`, `tt_validate.cob`; `tt_cobol_facts` allows seven keys only |
| Dart | `lib/src/typed.dart` (245 lines): `CallFacts`, `CallResult`, `Entity`, `Edge`, `Recognition`, `Relations`; `_map` rejects extra keys |
| JVM | `ResultEnvelope.java` (205 lines), a hand reader with allowed and required key sets |
| Objective-C | `facts_decode` in `ThinkThen.m` and `TTJSON.c` (134 lines) compare against a seven-key list |
| C#, Go, Swift, Zig | a typed `Facts` or `CallFacts` struct each |
| R | `src/rust/src/calls.rs` and `calls/render.rs` build every list field by field with extendr |
| Ruby | `src/ffi/result.rs` and `result.rs` build every hash key by key with magnus |
| PHP, Python, TypeScript | already JSON into host values; Python adds a `Facts` class |

The strict readers break the compatibility rule of `specification/result.md`: a reader ignores a member it does not know. That is why `estimated_cost_usd` needed matching Ada, Objective-C and COBOL releases.

## Decision

### 1. One set of Rust types is the source of truth

Every output a surface reads has one Rust type that serializes it. The schema is derived from those types and from nothing else.

| Output | Rust type |
| --- | --- |
| Bare answer per function | `Value` for `decide`, `choose`, `tag`, `score`; the existing recognize, relate and find value types |
| Detailed row | `DecisionResult`, `AnnotateResult`, recognize and relate details, with `Answer` for the five answer kinds |
| Record row and record error | the existing record row and `thinkthen.record-error/1` types |
| `meta` and usage | `Meta`, `Usage` |
| Annotate member | `AnnotatedValue`: `Answered(Value)` or `Failed(FailedValue)` |
| Call facts | `public::Facts`, which gains `Serialize` |
| Call error | a `CallError` form of the public error: `kind`, `retryable`, `message`. It exists only in the schema, as a test type, until a surface prints one. (Amended in 0314 slice 2.) |
| C door reply | a public `DoorReply` envelope: `value`, `facts`, optional `attempts` |

The annotate union is bool, label, labels, number, null or failure. A not sure answer is JSON `null`. A failure is always an object with one member, `failed`. No answered value is ever an object, so the two cannot be confused in JSON. The schema states this as an `anyOf` with the failure branch closed. The derive emits `anyOf`. It stays unambiguous because no `value` branch is an object and `failed` is a closed object. (Amended in 0314 slice 1; the first draft said `oneOf`.)

Questions enter results as the `question` member. That member's type is part of the result schema. The question-file input grammar is not; see section 3.

### 2. `schemars` derives the schema in a unit test

`schemars`, pinned `=1.2.2`, becomes a **dev-dependency** of `crates/thinkthen`, with its `raw_value` feature when `DoorReply.value` is a `RawValue`. Its `preserve_order` feature is never enabled. Each type in section 1 carries `#[cfg_attr(test, derive(schemars::JsonSchema))]`, which works because the drift test is the crate's own unit test.

About fifteen types on the result path write themselves with a hand `Serialize` impl: `Labels`, `QuestionText`, `Description`, `Meaning`, `Threshold`, `Distribution`, `TagProbabilities`, `Odds`, `NamedValues`, `NamedAnswers`, `FindAnswer`, `FindQuestionOwned`, `ProfileName`, `AnnotatedRecord`, `Json` and `Pointer`. Each field of those types uses `#[schemars(with = …)]` naming the plain form it prints, such as `String`, `Vec<String>` or a map of `String` to `f64`. A full hand `JsonSchema` impl is the last resort, under `#[cfg(test)]` beside its `Serialize` impl. The corpus is the backstop for these forms. Probability ranges and closed objects get `schemars` attributes only where a corpus case tests them.

The per-verb bare definitions, `decide`, `choose`, `tag`, `score`, `filter`, `rank` and `find`, have no single Rust type: `Value` is untagged, and `filter` and `rank` print records through `json!`. The drift test adds each verb's definition by name from the Rust type that serializes it, such as `subschema_for::<Option<bool>>()` as `decide`, `Option<String>` as `choose` and `Vec<String>` as `tag`. No schema-only newtype is added. Attributes keep today's other definition names, such as `detailed`, `failed`, `facts` and `callSuccess`, so the corpus and the port checks keep working unchanged.

One unit test, `crates/thinkthen/src/schema_tests.rs`, builds the Draft 2020-12 schema and compares it byte for byte with the committed file. On a difference it fails and prints the command that rewrites the file: the same test with `THINKTHEN_WRITE_SCHEMA=1`. With that variable set, the test rewrites the file and still fails with `schema rewritten; rerun`, so a green run always compares. It lives outside `core`, which reads no environment.

Why this and not the alternatives:

- **Weight.** A dev-dependency enters no build, binary, wheel, gem, archive or binding. It adds six crates to test builds: `schemars`, `schemars_derive`, `serde_derive_internals`, `dyn-clone`, `ref-cast`, `ref-cast-impl`. `syn`, `quote` and `serde` are already in the tree. All six sit in the local registry at 1.2.2, so gates stay offline.
- **A hand generator in a test** would be a third hand copy of the shape in Rust. It could drift from the `Serialize` impls exactly as the JSON file drifts today.
- **A build script or a feature** runs on every build or needs its own gate. A unit test runs under the plain `cargo test --workspace` every landing already runs.

The residual risk is the `with` forms and any hand impls. They are few, and the corpus in `specification/fixtures/types/` still validates real door output against the generated file, so a wrong hand impl fails there.

### 3. Where the schema lives and how it is versioned

- The file stays `specification/result.schema.json` with `$id` `urn:thinkthen:result`. Its `description` says it is generated and names the test. Nobody edits it by hand.
- The C door request, `doorRequest`, is input, not output. It moves to `specification/question-file.schema.json`, which stays hand-kept. That file is already held to the production parser by its own parity corpus, which is a drift test for inputs. Generating it would need the question parser rebuilt on derived types, which is outside ruling 7's result scope; section 6 defers it.
- Versioning keeps `specification/result.md`'s rule. Rows say `thinkthen.result/1`. A release may add a member. A renamed, removed or retyped member moves rows to `thinkthen.result/2`. The drift test turns every change into a visible diff of the committed file, and the code reviewer applies the rule to that diff. No script judges compatibility.
- `specification/types.md` and ADR 0082 change one sentence each: the schema is generated from Rust, not hand-written.

### 4. What ports do

A port reads the JSON the engine gives it into the host's ordinary JSON value: a dict, map, hash, list, `JsonNode`, `serde_json::Value` or equivalent. It adds exactly three things:

1. **Named outcomes.** `YES`, `NO`, `UNSURE` with the C header's values 1, 0, 2, in the host's enum or constant idiom. A port maps JSON `true`, `false`, `null` to them.
2. **Named error kinds.** `usage`, `backend`, `deadline`, `local`, `cancelled`, `defect`, with the C codes 1 to 6, in the host's enum, exception class or constant idiom. The retryable flag and message ride along.
3. **Null versus failure.** One helper per port reads an annotate answer as unresolved, a value, or a failure with its `kind` and `cause`. A static language returns a three-case type. A dynamic language returns the JSON value and offers `failed(member)`, which returns the failure object or nothing. No port turns a failure into null. The helper applies only to the answer names of a row's `value` and `answers`. The record members of an annotated row stay open in the schema and are never read as failures.

Everything else stays host JSON. A reader ignores members it does not know.

All packages are 0.0.1 and unreleased, so removing typed classes needs no deprecation. Deleted in the port pass, subject to each family's exact inventory:

- Ada: the JSON validator and closed facts set in `thinkthen.adb`; `Run_Facts`, `JSON_Result`, `Entity`, `Relation_Edge`. `Outcome`, `Error_Kind` and the annotate field state stay.
- C++: `CallFacts`, `Entity`, `Recognized`, `RelatedEntity`, `Edge`; the strict decoder shrinks to what reading host values needs. `ErrorKind` and `FailedField` stay.
- COBOL: the seven-key facts check. The 88-level outcome and failure codes stay.
- Dart: `typed.dart` except `ErrorKind` and the sealed annotate field.
- JVM: `ResultEnvelope.java`'s allowed and required sets.
- Objective-C: `facts_decode`'s key list and `TTCallFacts`. `TTErrorKind` and `TTFieldState` stay.
- C#, Go, Swift, Zig: the typed facts structs. Their error-kind enums stay.
- R: the field-by-field `list!` builders in `calls.rs` and `calls/render.rs`. The Rust side returns the serialized JSON and R reads it with `jsonlite`, already imported. R's one-based offset conversion stays.
- Ruby: the key-by-key hash builders in `src/ffi/result.rs` and `result.rs`. Ruby reads the JSON with the standard `json` library.
- Python: the `Facts` class becomes the same read-only mapping every other result already is. Its error classes stay.
- TypeScript and PHP: nothing to delete.

The C door itself stops building JSON by hand. `call.rs` and `failures.rs` serialize `DoorReply` and `Facts`. The door has no JSON call error: a failed call returns a code, a message and facts. (Amended in 0314 slice 2.) Today's `facts` bytes come from an unordered `json!` map, so their keys print in sorted order; the struct fields take that same order, and door output stays byte-identical.

### 5. How this combines with 0291

0291 adds `thinkthen_plan_json` and public plan, deadline, active cap and score and tag refusal methods to the same fourteen doors. Both tickets edit every binding's public wrapper, so they share one pass per binding family:

1. The C bridge first: 0291's `thinkthen_plan_json`, plus this ADR's typed door reply. `libraries/BINDING-AUTHOR.md`, 0291's guide, states section 4's thin-port rule and the three additions.
2. Then 0291's families in its order: C, Go, C++; C#, JVM, Dart; Swift, Objective-C, COBOL; Ruby, PHP, TypeScript; Ada, Zig. Each family deletes its hand-kept result layer, adds or keeps the three additions, and adds 0291's methods in one commit series and one review.

The port pass starts after ADR 0111 slice 3 lands, because that slice moves the C door and the SQL hosts onto `ask_all`. So the door changes under the bindings once.

### 6. What this does not do

Per ruling 7, these wait:

- Per-language label-set types, enum-derived label sets and description metadata beyond what each port has today.
- Typed C result structs or description arrays in the ABI. The C door stays JSON.
- SQL enums, domains or typed return columns.
- Pydantic models or other schema-typed host classes, and generated TypeScript declarations or Python stubs. `index.d.ts` and `__init__.pyi` stay as they are.
- Span types that name their offset unit, and frame dtype changes.
- A generated question-file schema.
- Replacing `spec/result.md`'s member table check with schema validation.

### 7. Slices

Each slice lands green: `cargo test --workspace`, `policy.py`, fresh code review.

1. **Generated schema.** Add the dev-dependency, the derives, the `with` forms, the per-verb definitions, `Facts` serialization, `CallError`, `DoorReply` and the drift test. Regenerate `result.schema.json`; move `doorRequest` to the question-file schema and point `types/check.py` at it. No output byte changes. Proof: the drift test passes, fails on a one-byte edit of the committed file, and fails when a field is added to `Meta` without regenerating; `sh specification/fixtures/types/self-test` passes with every corpus verdict unchanged, or a changed verdict is an extra-member case the commit names under the compatibility rule; `sh specification/fixtures/question-file/self-test` passes with `doorRequest`; `cargo tree -e normal` for `crates/thinkthen`, `libraries/c` and `libraries/python` shows no `schemars`.
2. **The C door serializes typed replies.** First add one golden assertion of today's exact reply and error-facts bytes with every optional facts key present, and land it green on the hand-built code. Then `call.rs` and `failures.rs` use `DoorReply` and `Facts`. The door has no JSON call error; it returns a code, a message and facts. Proof: the golden assertion passes unchanged after the switch; the C door's existing tests pass; the types self-test passes against the real door; every port's `public_types.py` or `type_cases.py` passes unchanged.
3. **Native bindings, right after slice 2.** R and Ruby read serialized JSON; Python's `Facts` becomes a mapping. Proof: each binding's conformance run passes; R case 41 still reports positions `11..20`; slice 4's null, failure, error-kind and extra-member cases pass.
4. **Port families, with 0291.** One family at a time, in section 5's order, after ADR 0111 slice 3. Proof per port: its corpus check passes against the generated schema; one shared annotate case with a `null` member and a `failed` member returns unresolved and failure distinctly through the public binding; one usage failure surfaces as the named `usage` kind with code 1; one row carrying an unknown extra member reads without error; 0291's P1 plan, deadline, cap and refusal proof passes; the family's deleted files and line count are in the build record.

## What this amends

| ADR or ticket | Change |
| --- | --- |
| ADR 0082 | The result schema is generated from Rust, not hand-written. `doorRequest` moves to the question-file schema. Its J3 to J8 port typing waits, except named outcomes, named error kinds and null versus failure |
| Ticket 0291 | Built in the same port pass as this ADR, after the C bridge |

## What Ian can overturn

Ian's ruling built here: Rust owns every type, one generated schema, thin ports with three additions (ruling 7).

The design author's calls:

1. `schemars` as a dev-dependency, derived in a unit test, with `with` forms or hand impls for hand `Serialize` types and per-verb definitions named in the test.
2. The question-file schema, now including `doorRequest`, stays hand-kept under its parser parity corpus.
3. Strict port readers become tolerant, per the existing compatibility rule.
4. Typed result classes in Ada, C++, Dart and the facts structs elsewhere are removed from public APIs.
5. The port pass waits for ADR 0111 slice 3 and shares 0291's families.
