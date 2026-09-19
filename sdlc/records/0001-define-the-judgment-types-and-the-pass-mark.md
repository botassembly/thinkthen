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

`serde` 1.0.229 with derive, `serde_json` 1.0.151, `thiserror` 2.0.20, and `proptest` 1.11.0 as a development dependency. The closure was 66 packages, and the review trimmed it to 52. Every one resolves from crates.io with a checksum, and every license expression offers MIT, Apache-2.0, Unicode-3.0, or Unlicense. The expressions in the tree use every operator the reader now handles: a bare `MIT`, an `OR`, the deprecated slash in `Apache-2.0 / MIT`, a `WITH` exception in `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`, and a parenthesized `AND` in `(MIT OR Apache-2.0) AND Unicode-3.0`.

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

## Review

A second reviewing agent read the branch after it was rebased onto `75cad05`, the commit that settled `specification/result.md` on a compact one-line document. The figures above describe the branch as the build agent left it. The figures in this section describe the branch as it now stands.

### What the review checked

It read `AGENTS.md`, `sdlc/planning/rust-standards.md`, Ian's Rust ideal state, the settled part of `specification/result.md`, the ticket, the record, the whole diff against main, and every file under `crates/thinkthen-core/`.

Against the ticket it walked each Scope item and each Acceptance bullet to the code and the test that satisfies it. It broke three behaviors and watched the matching test fail, then restored the code. Turning `1 - p >= P` into `1 - p > P` failed `the_pass_mark_rules_follow_the_specification` at `probability 0.1 under Symmetric(PassMark(0.9))`. Swapping the `question` and `answer` fields failed `a_result_serializes_in_the_order_the_specification_prints` on the rendered string. Widening `value <= 0.5` to `value < 0.5` failed `new_refuses_what_is_not_a_pass_mark` at `mark 0.5`.

Against the rebased specification it checked the compact rendering character by character, the field names and their order, the always-present `value` and `min_prob` printed as null when nothing was assessed, `usage` as the only field carrying `skip_serializing_if`, token counts as `u64`, and `meta.adapter` serializing as `systemone`, which `specification/backends.md` settles. It checked the three pass-mark rules, including a probability exactly on the mark accepted at both ends.

On size it counted every file, hunted duplication across the whole crate rather than the diff, and listed every public item against what this ticket and tickets 0002 and 0003 need.

On the public surface it confirmed every struct field is private, every fallible constructor returns a result, every fixed set is an enum, no value travels as a bare string, and every public item carries a doc comment that `cargo doc` with warnings denied proves. Every module is private and reachable only through `lib.rs`, so `unreachable_pub` leaves no `pub` that could be `pub(crate)`.

On errors it confirmed three `thiserror` enums, no string error, no boxed error, no ignored result binding, and variants a caller can act on kept apart. `EmptyTextError` names which of the three text values arrived empty.

On ownership it found no clone outside one test comparison, no owned parameter that is not kept, and no reference counting or interior mutability.

On dependencies it checked the four crates against the starting set in the standards, the exact versions, the crates.io source and checksum of every resolved package, the license expression of every package against the four allowed identifiers, the accepted sets in `policy.py` against both manifests, and the SPDX reader against its own fourteen-case table.

On gates it confirmed the workspace lint table, both `clippy.toml` files, and all four ladder scripts are byte for byte what main carries. No `#[allow]` appears anywhere in the crate. The largest file is `assessment.rs` at 179 non-blank lines, well under the ceiling of 500.

### What the review changed

- Trimmed `proptest` to `default-features = false, features = ["std"]`. The fork, timeout, and bit-set strategies were never asked for, and the closure falls from 66 packages to 52.
- Dropped every `Deserialize` derive, the `TryFrom` conversions behind them, the `Fields` shadow struct, `AssessmentShapeError`, and the private `Schema` enum. Nothing in `thinkthen` parses a `thinkthen` result. `Assessment` has one constructor, `assess`, which cannot build an inconsistent value, so the shape check guarded a path no caller reaches. The two tests that parsed the specification example back were deleted, and the `schema` field now holds the `SCHEMA` constant directly.
- Deleted the read-back accessors on `Meta`, `Usage`, and `DecisionResult`, except `DecisionResult::assessment`, which ticket 0003 needs to pick an exit code under `--status`.
- Reworded the first documentation line of `text.rs` and `question.rs` so neither states two jobs joined by "and".

### What the review left, and why

- The ticket asks for a test that compares a serialized result **by value** with the specification example. The rebased specification prints one compact line and calls the indented example a reading aid, so the conformance test is now a string comparison against that one line. A by-value comparison would need a parser the program never runs, and `serde_json::Value` is banned in the core, so there is no smaller way to keep it. The steering agent can overturn this and restore `Deserialize` on `DecisionResult` alone.
- `Probability` and `PassMark` share about thirty lines of newtype shape. A `macro_rules` could fold them the way `text.rs` folds its three, but the two differ in their error variants, their range rules, and their documentation, so the macro would take a rule and three doc fragments as arguments and read worse than the two plain types. The duplication stays and is named here.
- `Question::verb` and `Answer::kind` read back fields that nothing calls today. They stay because ticket 0002 builds a request from a question and may dispatch on the verb.
- `Condition::new` accepts text that holds only whitespace. The specification says nothing about trimming, and adding a rule is a specification decision rather than a review fix.
- `serde_json` stays a normal dependency of the core while only test code calls it. Clippy resolves a ban-list path against the graph of the target it is linting, so a development dependency would leave the dynamic JSON bans silent on library code. Ticket 0003 gives the dependency a second reason when the binary prints.

### The ratchet

The ceiling was 951 when the review started and is 799 now. Every step down landed in the commit that removed the lines.

### The ladder after the review

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Nineteen unit tests, one integration test, one documentation test, and four spec examples pass.

### The verdict

The work is good enough to land.
