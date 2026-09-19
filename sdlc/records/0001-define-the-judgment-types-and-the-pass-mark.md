# Record 0001: Define the judgment types and the pass mark

- Ticket: `sdlc/tickets/0001-define-the-judgment-types-and-the-pass-mark.md`
- Branch: `ticket/0001-judgment-types`
- Landed: 2026-09-19

The ticket changed hands. A first build agent added the dependencies, taught `policy.py` to read SPDX expressions, and wrote the module skeletons before it was stopped by accident. A second build agent took the work over, read every file the first one left, kept what matched the specification, and finished the ticket.

## What was built

`thinkthen-core` now holds the vocabulary of one yes/no judgment in eight modules, each with one job stated in its first line.

- `probability.rs`: `Probability`, a finite number from zero to one, with `ProbabilityError`.
- `pass_mark.rs`: `PassMark`, above one half and at most one, with `PassMarkError`.
- `text.rs`: `Condition`, `BackendName`, and `ModelName`, three non-empty text values declared by one macro, with `EmptyTextError`.
- `question.rs`: `Verb` and `Question`.
- `answer.rs`: `AnswerKind` and `Answer`.
- `policy.rs`: `Policy`, either a symmetric mark or none.
- `assessment.rs`: `AssessmentStatus`, `Assessment`, `AssessmentShapeError`, and the total function `assess`.
- `result.rs`: `SCHEMA`, `Adapter`, `Usage`, `Meta`, and `DecisionResult`.

Every field is private and every value that can be refused is built through a constructor that returns a result. Each enum carries only the variant `decide if` needs. The schema enum is private, so the public surface names the schema once, as the constant the ticket asked for.

## What was kept and what was changed from the draft

- Kept the eight modules, the type names, the table test of the pass mark rules, the three property tests, and the serialization tests.
- Kept the SPDX reader in `policy.py`. The allowed set is still MIT, Apache-2.0, Unicode-3.0, and Unlicense. No dependency needed a wider list.
- Wrote `result.rs`, which the draft left as tests with no code.
- Added a table check that holds the SPDX reader to fourteen expressions, because a reader that widens quietly decides which crates the repository may depend on.
- Made `Assessment` validate on deserialization, so no document can claim a status its value and its mark do not support.
- Deleted the duplicate shape rule. The deserializer and the property test now read one function.
- Dropped the unused `Default` on `Policy` and the public `Schema` enum and its accessor.
- Sorted the module list in `lib.rs`.

## Red then green

Every test below failed for its stated reason with the code broken and passed once the break was undone.

- `result.rs` as the first agent left it: the whole crate failed to compile with `unresolved imports crate::result::{Adapter, DecisionResult, Meta, SCHEMA, Usage}`.
- `the_pass_mark_rules_follow_the_specification`: with `>=` turned into `>` in `assess`, it failed at `probability 0.9 under Symmetric(PassMark(0.9))`.
- `a_result_serializes_in_the_order_the_specification_prints`: with `question` and `answer` swapped in `DecisionResult`, the rendered string stopped matching.
- `usage_is_absent_when_the_backend_reports_none`: with `skip_serializing_if` removed, the rendered meta carried a null usage.
- `a_document_whose_status_contradicts_its_fields_is_refused`: with the shape check short-circuited, six contradictory documents parsed.
- `every_probability_reaches_exactly_one_status`: with the status forced to accepted, it failed on `Assessment { status: Accepted, value: None, min_prob: Some(PassMark(0.714...)) }`.
- `an_accepted_yes_and_an_accepted_no_never_meet` and `mirroring_the_probability_flips_the_value`: with the no branch scaled by 1.5, both failed at `probability = 0.2695..., threshold = 0.6556...`.
- `new_refuses_what_is_not_a_probability`, `new_refuses_what_is_not_a_pass_mark`, and `new_refuses_empty_text`: with each range or emptiness check widened, each failed on its first refused case.
- The SPDX table check: with GPL-3.0 added to the allowed set, `policy.py` exited 1 and named the two expressions whose answers changed.

The proptest regression file those breaks produced was deleted, because the inputs came from broken code.

## The dynamic JSON bans fire

`serde_json` now resolves, so the bans that Clippy had been ignoring were proved. A function taking `&serde_json::Value`, calling `json!`, and calling `Value::get` was planted in the core's non-test code. `sdlc/scripts/lint` refused each one with the ban-list reason.

```
error: use of a disallowed type `serde_json::Value`
   = note: wire bodies must decode into typed structs and never into dynamic JSON
error: use of a disallowed macro `serde_json::json`
   = note: wire bodies must decode into typed structs and never into dynamic JSON
error: use of a disallowed type `serde_json::Map`
   = note: wire bodies must decode into typed structs and never into dynamic JSON
error: use of a disallowed method `serde_json::Value::get`
   = note: wire bodies must decode into typed structs and never into dynamic JSON
```

`json!` fired twice, once as the macro and once as the `serde_json::Map` it expands to. No ban path needed correcting, so `clippy.toml` and the accepted copy in `policy.py` are unchanged. The probe was removed and the sources match what they were before it was planted. The corresponding gap in `sdlc/planning/rust-standards.md` under "Not yet enforced" is closed.

## Float boundary

The rules are `p >= P` for yes and `1 - p >= P` for no, applied to the numbers as they are, with no epsilon. Both documented boundary cases hold exactly in IEEE arithmetic. `0.9 >= 0.9` is true, and `1.0 - 0.1` is the same double as `0.9`, so a mark of 0.9 takes 0.9 as a yes and 0.1 as a no. The mirroring property was also checked outside the test, over two million random pairs and over the four doubles either side of `P` and `1 - P` for two hundred thousand marks, and it never failed. No rule change is needed.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Twenty-two unit tests, one integration test, one documentation test, and four spec examples pass.

## The ratchet

The ceiling was 87 and is now 951, which is the measured total. The commit that raises it names what grew, why the lines are earned, and where duplication was sought first.

## Dependencies

`serde` 1.0.229 with derive, `serde_json` 1.0.151, `thiserror` 2.0.20, and `proptest` 1.11.0 as a development dependency. The closure is 66 packages. Every one resolves from crates.io with a checksum, and every license expression offers MIT, Apache-2.0, Unicode-3.0, or Unlicense. The expressions in the tree use every operator the reader now handles: a bare `MIT`, an `OR`, the deprecated slash in `Apache-2.0 / MIT`, a `WITH` exception in `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`, and a parenthesized `AND` in `(MIT OR Apache-2.0) AND Unicode-3.0`.

## What the specification and the standards left open

- `specification/result.md` shows the result pretty-printed and `specification/channels.md` says one JSON document and a newline. Neither says which one the tool prints. The core serializes compactly in the field order the document shows, and the binary will settle the layout when it prints.
- The document says `usage` is absent when the backend reports none, and it does not say whether `value` and `min_prob` are absent or present as null when nothing was assessed. The paragraph on `unassessed` says both are null, so both keys are always present and `usage` is the only field that disappears.
- The document names the adapter `systemone` in an example and nowhere states the set of adapter names. `Adapter` is an enum with the one variant, which serializes to `systemone`.
- The document does not give a type for the token counts. They are unsigned 64-bit integers.
- `sdlc/planning/rust-standards.md` says the core's ban list covers the `serde_json::Value` access methods and the `json!` macro. The list also covers `serde_json::Map`, `to_value`, and `from_value`. The document understates what is banned.
- `sdlc/planning/rust-standards.md` says `proptest` arrives with the first parser. There is no parser yet, and the property tests here cover the acceptance policy. The rule reads as a floor rather than a gate.

## The second review

The reviewing agent checked the dependency additions and the public surface. What it checked is recorded below.

It checked the argument for each of the four crates against the standard library and against the starting set in the standards. It checked that each version is an exact published release, that all 66 resolved packages come from crates.io with a checksum, and that `cargo metadata --locked` succeeds. It listed every license expression in the closure, named the six shapes that are not a plain OR of allowed identifiers, and confirmed each passes through an OR or a satisfied AND. It exercised the SPDX reader directly on refused expressions, including `MIT AND GPL-3.0`, a bare `GPL-3.0`, `BSD-2-Clause AND MIT`, a lone `WITH` exception, and the empty string. It confirmed the allowed set is still the four identifiers and that the accepted dependency sets match both manifests.

On the public surface it checked that every exported name is in the ticket's scope, that `Adapter` is there because the `meta.adapter` field requires it, and that each enum carries one variant for `decide if` and no more. It checked that every struct field is private with an accessor, that every fallible constructor returns a result, and that the infallible ones take already validated types. It confirmed no file, environment variable, socket, clock, or process appears in the crate, and that `serde_json::Value`, `Map`, and `json!` appear nowhere. It walked the serialized field order, the snake_case values, the lowercase adapter, and the skipped usage field against the specification.

It found no defect in either category. It noted one fact. `serde_json` is a normal dependency of the core while only test code uses it today. The ticket asks for exactly that, because the Clippy ban paths do nothing until `serde_json` resolves, and the binary will print through it.
