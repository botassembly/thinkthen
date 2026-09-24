# Quick Fix qf-answer-labels: withhold answer labels under Debug and test the real request holder

Status: landed. It closes `sdlc/issues/2026-09-24-answers-still-print-record-labels-under-debug.md`. A fresh read-only Opus review is in `sdlc/records/qf-answer-labels-review.md`.

## Result

- `Distribution` and `TagProbabilities` in `core/answer.rs` print the labels' total byte length and every probability, such as `Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }`. One helper, `labelled_odds`, writes both.
- `Shape` keeps its derived `Debug`. Its `pick` and `level` fields are now a private `Label` newtype that serializes as the bare string and prints `<N bytes withheld>`.
- `Value` prints each label's length: `Choice(Some(<22 bytes withheld>))` and `Tag([<22 bytes withheld>])`. `YesNo` and `Score` print as before.
- `Answer`, `DecisionResult`, `AnnotatedAnswer`, `Reply`, and the other holders keep their derived `Debug`. They print no label, because every leaf that holds one withholds it.
- `PlanDocument` in `core/plan_document.rs` derived `Debug` and printed its `request` as `RawValue({"state":"...","questions":{...}})`. That line held the evidence and every label. It now prints `request: <N bytes withheld>` and every other field as before.
- `Request` in `core/adapters/systemone/request.rs` lost its hand-written `Debug` and derives `Debug` only in tests, as its wire question types already did. It lives only inside `encode_raw`, which writes it at once, so no production line can print it. The compiler now enforces that.
- `Response` and `ResponseAnswer` in `core/adapters/systemone/response.rs` key their odds by label. They now derive `Debug` only in tests too. They live only inside `decode_observed`.
- The test-only export `#[cfg(test)] pub(crate) use ...::request::request` in `core/adapters/systemone.rs` is gone, and `encode_raw` builds its `Request` inline again.

## Tests

The four questions for the reworked test follow.

- It protects the secrecy rule for `Debug` lines. No `Debug` line of a plan, a `--plan` document, an engine reply, an answer, a value, or a result row shows the evidence or a label a record gave.
- A credible regression is a new `derive(Debug)` on an answer type or a new field holding a raw label. Another is a new holder of the encoded request, as `PlanDocument` was.
- No other test prints an answer or a plan document under `Debug`.
- It needs no test-only export, flag, or hook. It drives the engine facade from ticket 0085, the call the command makes. `Engine::judge` sends each question to a counted loopback listener and reads the answer back. `PlanDocument::of` builds what `--plan` prints. `DecisionResult::new` builds every detailed row.

`no_record_label_or_request_debug_line_shows_the_evidence` in `crates/thinkthen/src/cli/failure/tests.rs` is now `no_record_label_request_or_answer_debug_line_shows_the_evidence`. It reads two labels from a JSON record, the first being the evidence marker. It builds the `--plan` document for `choose`, `tag`, and `score` over them. It then asks each question through `Engine::judge` against a loopback listener whose canned reply puts the marker first. It builds one result row per judgment and prints the plan, the document, each reply, and each row in plain and pretty `Debug`. It refuses the marker and the key marker, checks that three requests went out, and pins the exact `Answer` and `Value` line for all three verbs. It no longer pins the `Question` line, because `a_question_shows_its_own_text_in_debug_and_withholds_its_labels` in `core/question/tests.rs` pins it. It no longer prints a `Request`, because nothing outside a test can hold one.

Red first: with `core/answer.rs` and `core/plan_document.rs` as main has them, the test failed on the marker 32 times. It printed `request: RawValue({"state":"marker-evidence-7b3ac5"` from the plan document and `Answer(Choice { pick: "marker-evidence-7b3ac5"` from the answer.

Planted bugs, one at a time on the fixed tree, each failed the test:

- `Label`'s `Debug` printing its string showed `Score { level: "marker-evidence-7b3ac5"`.
- `PlanDocument`'s `Debug` printing `&self.request` showed `request: RawValue({"state":"marker-evidence-7b3ac5"`.
- `Value::Tag` printing its labels showed `Tag(["marker-evidence-7b3ac5"`.

## Ratchet

After merging main with ticket 0085, the ceiling rises from 49733 to 49872, by 139 lines. The test grows by 82 lines. It now builds an engine over a loopback listener and pins three verbs instead of one. The source grows by 63 lines. The `Debug` impls for `Distribution`, `TagProbabilities`, `Label`, `Value`, and `PlanDocument` add 80 lines. Dropping the `Request` impl, the `request` function, and the export gave back about 30. I looked for duplication in the answer tests and the question tests. `Withheld` already serves every site, and the only overlap was the `Question` pin, which the test dropped. The facade tests build their engine through a helper private to `engine`, so the secrecy test writes its own settings.

## Checks

Pending.
