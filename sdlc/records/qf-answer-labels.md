# Quick Fix qf-answer-labels: withhold answer labels under Debug and test the real request holder

Status: landed. It closes `sdlc/issues/2026-09-24-answers-still-print-record-labels-under-debug.md`. A fresh read-only Opus review is in `sdlc/records/qf-answer-labels-review.md`.

## Result

- `Distribution` and `TagProbabilities` in `core/answer.rs` print the labels' total byte length and every probability, such as `Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }`. One helper, `labelled_odds`, writes both.
- `Shape` keeps its derived `Debug`. Its `pick` and `level` fields are now a private `Label` newtype that serializes as the bare string and prints `<N bytes withheld>`.
- `Value` prints each label's length: `Choice(Some(<22 bytes withheld>))` and `Tag([<22 bytes withheld>])`. `YesNo` and `Score` print as before.
- `Answer`, `DecisionResult`, `AnnotatedAnswer`, `Reply`, and the other holders keep their derived `Debug`. They print no label, because every leaf that holds one withholds it.
- `PlanDocument` in `core/plan_document.rs` derived `Debug` and printed its `request` as `RawValue({"state":"...","questions":{...}})`. That line held the evidence and every label. It now prints `request: <N bytes withheld>` and every other field as before.
- `Request` in `core/adapters/systemone/request.rs` lost its hand-written `Debug` and derives `Debug` only in tests, as its wire question types already did. It lives only inside `encode_raw`, which writes it at once, so no production line can print it. The compiler now enforces that.
- The test-only export `#[cfg(test)] pub(crate) use ...::request::request` in `core/adapters/systemone.rs` is gone, and `encode_raw` builds its `Request` inline again.

## Tests

The four questions for the reworked test follow.

- It protects the secrecy rule for `Debug` lines. No `Debug` line of a plan, a `--plan` document, a decoded reply, an answer, a value, or a result row shows the evidence or a label a record gave.
- A credible regression is a new `derive(Debug)` on an answer type or a new field holding a raw label. Another is a new holder of the encoded request, as `PlanDocument` was.
- No other test prints an answer or a plan document under `Debug`.
- It needs no test-only export, flag, or hook. It drives only production calls. `PlanDocument::of` builds what `--plan` prints. `built_in::decode` reads every backend response and replay. `DecisionResult::new` builds every detailed row.

`no_record_label_or_request_debug_line_shows_the_evidence` in `crates/thinkthen/src/cli/failure/tests.rs` is now `no_record_label_request_or_answer_debug_line_shows_the_evidence`. It reads two labels from a JSON record, the first being the evidence marker. It asks `choose`, `tag`, and `score` over them and builds the `--plan` document. It decodes a response in which the marker leads every answer. It then builds one result row per answer and prints all of these in plain and pretty `Debug`. It refuses the marker and pins the exact choice `Answer` and `Value` lines. It no longer pins the `Question` line, because `a_question_shows_its_own_text_in_debug_and_withholds_its_labels` in `core/question/tests.rs` pins it. It no longer prints a `Request`, because nothing outside a test can hold one.

Red first: before the fix, the test failed on the marker 32 times. It printed `request: RawValue({"state":"marker-evidence-7b3ac5"` from the plan document and `Answer(Choice { pick: "marker-evidence-7b3ac5"` from the answer.

Planted bugs, one at a time on the fixed tree:

- `Label`'s `Debug` printing its string failed the test with `Answer(Choice { pick: "marker-evidence-7b3ac5"`.
- `PlanDocument`'s `Debug` printing `&self.request` failed the test with `request: RawValue({"state":"marker-evidence-7b3ac5"`.

## Ratchet

The ceiling rises from 48806 to 48898, by 92 lines. The `Debug` impls for `Distribution`, `TagProbabilities`, `Label`, `Value`, and `PlanDocument` take about 85 lines. The test grows by 38 lines to cover three verbs, the plan document, the reply, and the rows. Dropping the `Request` impl, the `request` function, and the export gave back about 30 lines. I looked for duplication in the answer tests and the question tests. `Withheld` already serves every site, and the only overlap was the `Question` pin, which the test dropped.

## Checks

Pending.
