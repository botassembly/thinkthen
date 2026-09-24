# 0098: Build the binding members

Status: built, code review pending. Owner: Claude.

Branch `ticket/0098-binding-members`. The build sits on `origin/ticket/0086-public-rust-api` at `0b4b4e58`, after 0086's code-review fixes, merged while 0086 waits to land. It merges main again when 0086 lands. Ian can overturn every choice this record marks as decided.

## Result

The crate now exports every 0095 member that 0086 left out:

- `QuestionKind`, `Question::kind`, and `QuestionSet::members`.
- `Question::choose_labels`, `Question::tag_labels`, and `LabelBuilder`.
- `Recognize::from_json`, `Recognize::load`, `Relate::from_json`, and `Relate::load`.
- `Row::probability` and `ErrorKind::name`.
- `Details::to_json`, `AnnotatedRecord::value_json`, `Recognized::to_json`, and `Edge::to_json`.

Each member delegates to an existing owner. No parser, request path, or dependency is new.

- `src/result_json.rs` is the shared private module. It holds the assembly of the `thinkthen.result/1` line for one judgment, which used to sit inside `cli/asking.rs`. The command's `--details` row and `Details` both call it.
- The four JSON methods return a line written once when the result is made. A result keeps it in a private `Written` holder whose `Debug` withholds it, so no `Debug` line prints a question or a name.
- `from_json` calls `RecognizeSpec::parse` and `RelateSpec::parse`, the 0080 and 0081 parsers. `load` reads the file and maps both an unreadable file and a broken rule to `Local`, as `Question::load` does.
- `LabelBuilder` shares the typed builders' `Listing`. The typed step checks the choice's order, and the runtime step refuses a label already given. Both close through the same `Labels` checks.
- `Row::probability` carries the yes probability that `decide_many` already computed for each row.

### Choices made here

- Bare values keep the core's own serde types. The command writes `annotate`, `recognize`, and `relate` bare values with the core's `json_line`, and the library writes the same core values with the same call. Only the `--details` assembly lived in `cli`, so only it moved. 0095 says "the command calls these methods". The command calls the shared module, not the public methods, because the public results drop the core values the command still needs for record rows. The byte test below proves the equality. Ian can overturn this.
- `Recognize::from_json` refuses an `on` naming a part of a record, as `Question::from_json` and `QuestionSet::from_json` do. A library call's evidence is one whole text.
- `Relate::from_json` refuses any `fields` pointer other than `/name` and `/kind`, as 0095 rules. A file that spells the defaults out is accepted.
- `LabelBuilder::build` keeps each verb's default rule: no cut for `choose`, the 0.5 cut for `tag`. This matches `ChooseBuilder::build` and `TagBuilder::cut`.
- The runtime label step refuses a repeat with the core's own sentence: "a list holds each option once" for `choose` and "a list holds each label once" for `tag`.
- `public/question.rs` and `public/results.rs` would pass the 500-line file ceiling. `Choice` and `choices!` moved to `public/choice.rs` unchanged. The annotate results moved to `public/annotated.rs`, and there they gained the `value_json` member and its field. `results.rs` shares `withheld_debug` with the new file.
- Every JSON line is written once, when its result is made. The four methods then return `String` with no error, as 0095 declares them. An annotate row pays one serialization even when the caller never asks for JSON. `Details::of` digests the question twice, once for its field and once inside the shared line. Both costs are small beside the request, and removing the second digest would widen the shared function for the command too. Ian can overturn this.
- The builders' `Debug` lines withhold the question, the labels, and the descriptions. 0086's builders derived `Debug`, so a builder's line printed the question text. `Listing`, `DecideBuilder`, and `DescriptionBuilder` now write their own lines, which count what they hold. Every builder reads through one of them. The fix sits here because `LabelBuilder` would have leaked the same way, and 0086 is past its fix round. 0086 carries the leak until this ticket lands.

## Inventory check

`sdlc/scripts/inventory` now reads the whole added block of ticket 0095. It no longer reads 0086's "takes only" sentence. 353 declared items match, up from 326. The four built-in plants are still refused. A real source plant of `pub fn planted() {}` in `public/mod.rs` failed the check with "inventory: not in the contract: fn planted()".

## Tests

Each test answers the four questions in `CLAUDE.md`.

- `tests/backend/public_json.rs` covers G8. Every shared case that asks one whole text runs through the compiled command and through the public API on the same case arm. The cases are single judgments, `annotate` sets with no `on`, `recognize`, and `relate`. `Details::to_json` must equal the `--details` line, byte for byte. `value_json`, `Recognized::to_json`, and each `Edge::to_json` must equal the bare output. The total of 62 comparisons is pinned: 30 single texts, 12 annotate texts, 10 recognize texts, and 10 relate edges. It protects the library's JSON from drifting away from the command. Existing tests check only the command against the cases, and nothing else compared the two. It uses only public doors.
- `tests/public_members.rs` covers the rest through the public API alone.
  - Runtime labels equal `Question::from_json` of the same file, for `choose` with a description, `tag` with a cut, and `tag` under its default rule. A repeated label fails at its own step. `details` over runtime labels reads the typed call's value with one send each on the counted generic arm, and the bound typed question carries the same request digest (G5).
  - `members` follows set order, a banded member reads `Decide`, and `kind` reads each builder's kind.
  - `Relate::from_json` with `"either":true` equals the `both_ways` builder. Equality covers the whole parsed spec, and the digest reads only that spec. A broken rule is `Usage` from `from_json` and `Local` from `load`, with the same sentence, for both readers. A non-default `fields` and a recognize `on` are `Usage` with exact sentences.
  - `Row::probability` equals the served numbers 0.3 and 0.8, and equals the yes probability `details` reads for the same record.
  - `ErrorKind::name` gives the six words of `conformance/cases.json` in its order.
- A builder's plain and pretty `Debug` lines hold no word of its question, labels, or descriptions, for all six builders. It protects the rule that formatting is safe to log. A derived `Debug` fails it, and it failed on 0086's builders. No other test formats a builder.
- No test needs a test-only hook. Every send goes to a loopback listener with a fake key.

## Planted bugs

Each plant ran alone under the heavy lock and was reverted.

| Plant | Result |
| --- | --- |
| The library's `Details` drops `meta.requests_sent` from its line | red: `each_json_method_prints_the_commands_bytes_on_the_shared_cases`, first at `01-decide-yes-captured` |
| The shared serializer drops `meta.requests_sent` (`#[serde(skip)]` on the core `Meta`) | red: `a_result_serializes_in_the_order_the_specification_prints` and `meta_names_the_tool_and_drops_the_usage_a_backend_never_reported`. The byte test stays green, as it should, because both sides share the drop |
| `LabelBuilder::label` skips its repeat check. The ticket calls this the order check, but runtime labels have no order to check | red: `runtime_labels_build_the_files_question_and_refuse_a_repeat_at_its_step` |
| 0086's derived `Debug` on the builders, before this ticket's fix | red: `a_builders_debug_line_withholds_its_question_labels_and_descriptions` printed `DecideBuilder { text: QuestionText(String("sentinel-asked")), .. }` |
| An unexpected public export | red: `inventory: not in the contract: fn planted()` |

## Budgets

Measured nonblank lines against 0086 at `0b4b4e58`:

- Production Rust: 16 files touched, net 365 lines against 500. The builders' `Debug` fix adds 23 of them. The serializer lines that left `cli/asking.rs` are counted in the net.
- The file budget is ten, and this crosses it. This is the re-score. Each member lives beside its type, in `question.rs`, `builders.rs`, `set.rs`, `recognize.rs`, `relate.rs`, `results.rs`, and `error.rs`. `bulk.rs` and `engine.rs` pass the probability and the backend. `mod.rs` and `lib.rs` declare modules and exports. `core/mod.rs` exports `LabelsError` again, which 0086 had dropped as unused, so the runtime step refuses a repeat with the core's own sentence. `result_json.rs` is the one new module the ticket asks for. `choice.rs` and `annotated.rs` exist only to keep two files under the 500-line ceiling. Folding members into fewer files would break locality and the ceiling.
- Tests: 3 files, 437 lines against 700.
- Scripts: `inventory` shrinks by 5 lines. 0086's library-only package run now takes every test target, so `public_members` runs there with no script change.

## Ratchet

The ceiling rises from 0086's 60,351 to 61,153, an increase of 802 lines: 365 production and 437 test lines. The code review's `Debug` fix added 44 of them: three hand-written `Debug` lines and one test. Before adding lines, I looked for code to delete. The four `load` functions each read a file and map it to `Local` in three lines. A shared helper would save no net lines. The command's details assembly moved and was not copied. The test engine builders sit in separate test binaries, which cannot share a helper without a new support module.

## Ladder

At `9c0fc2cb`, `install`, `lint`, `test`, and `spec` passed in order under the heavy lock on 2026-09-24. The code review's fix commit changed three `Debug` lines and added one test after that run. Its focused `public_members` run passed, and the ratchet reads 61,153 of 61,153. The whole ladder has not run on the fix commit.

## Review

A fresh read-only Opus reviewer read `9c0fc2cb` against 0086 at `0b4b4e58`. It checked the shared serializer, the error kinds, the members, the ratchet, the inventory, the file sizes, the four-question gate, and the moves. It returned one must-fix and five nits.

1. Must-fix: a builder's derived `Debug` printed the question text and the labels. Fixed as the choices above say, with a test that went red on the old builders.
2. The record called both splits pure moves. Corrected.
3. The planted-bug row named an order check. Renamed to the repeat check.
4. A garbled doc sentence on `LabelBuilder::label`. Fixed.
5. A second digest in `Details::of` and eager annotate JSON. Kept and recorded above.
6. The ticket's Scope said ten files. The ticket now records the re-score.
