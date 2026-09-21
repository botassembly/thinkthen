---
flow: build
priority: 46
opens: transforms/compare transforms/README.md demos/14-grade-a-batch sdlc/planning
---

# 0043: Compare named `annotate` answers

Status: ready

## Outcome

`compare.jq` compares two detailed `annotate` runs one named question at a time. It keeps record pairing, evidence checks, and label checks at the record level, then reports each shared question's value changes and yes-or-no probability movement. A mistaken `jq -s` invocation fails and names `jq -n` instead of printing a believable wrong report.

## Decisions

Ian can overturn every item.

1. The transform accepts one shape per invocation: scalar detailed rows or `annotate` detailed rows. It infers the mode from every nonempty row on both sides. One empty side adopts the other's mode. Two empty sides return ticket 0042's scalar report. Mixing shapes or giving an invalid `annotate` row fails with a fixed safe message that echoes no row content.
2. Scalar reports keep every field and meaning from ticket 0042. An `annotate` report says `mode: "annotate"`. Its top-level `before` and `after` objects have `rows`, `question_sets`, and `models`. Top-level `changed` has `question_set` and `model`. The report then keeps `repeated_ids`, `only_in_before`, `only_in_after`, `paired`, `compared`, `mismatched_input`, and `mismatched_label` at record level, and adds `questions_only_in_before` and `questions_only_in_after`.
3. `questions` is an object in lexical question-name order. Each shared name has `before` and `after`, containing `questions` and `thresholds`; `changed`, containing `question` and `threshold`; and `compared`, `same`, `changed_values`, `flips`, `changes`, and the existing `yes_no_probability` shape. A change entry needs only the record `id`, because its enclosing key names the question.
4. A valid `annotate` row is an object with an object-valued `input` whose `id` is a string; string `meta.model`; a 64-character lowercase hexadecimal `meta.questions_sha256`; and object-valued `value` and `answers` with identical keys. Every nonempty side uses one consistent key set. Each nested answer has the same `value` as the top-level member, an object-valued `question`, a null, numeric, or string `threshold`, and an object-valued `answer`. `yes_no` takes a boolean or null, `choice` takes a string or null, `tag` takes an array of strings, and `score` takes a number. Tag arrays compare in order. Other answer kinds and value pairings are invalid. The transform does not validate fields it does not read, including request, URL, usage, and replay status.
5. Per-question `changed.question` compares the unique full question objects in the two runs. `changed.threshold` compares the unique thresholds. The top-level `changed.question_set` compares valid `meta.questions_sha256` values. When either run is empty it is null. If a nonempty row lacks a valid digest, the fixed invalid-annotate message wins instead of a text fallback.
6. The existing probability tolerance applies independently to every named yes-or-no question. Other question kinds report exact values and no invented probability movement. Adding or removing a question does not turn every record into a value change; the report names that question under `questions_only_in_before` or `questions_only_in_after`.
7. `compare.jq` requires null-input mode. It checks `input_filename` before reading rows and fails with `compare: run with jq -n` when invoked with `jq -s`, ordinary input, or a file containing literal null. Today it is the repository's only transform that reads a run through `--slurpfile`.

## Scope

Write the focused test red first. Refactor only enough shared comparison code to keep the scalar and named-question rules in one place. Update `compare.jq`, its header and example, the executable transform page, and how-to 14 with one short saved-run comparison. Use replay or synthetic rows only. Make no paid call and do not change page 16 in this ticket.

Excluded: treating a whole annotation object as one scalar value, probability rules for choice, tag, or score, repeated-trial aggregation, page 16 polish, `cost.jq`, sweep, monitors, grouped sweep, and human-label checks.

## Acceptance

- A red-green focused test covers a mixed `annotate` set with decide, choose, tag, and score answers. It proves exact scalar and tag-array changes, every boolean/null flip direction, lexical question and id order, and `compared == same + changed_values` for every question.
- Record-level duplicate ids, missing ids, changed evidence, and changed trusted labels remain visible and never reach a question's counts. A question present in one run alone is listed once and compared nowhere.
- Per-question question and threshold changes, top-level set-digest and model changes, an empty run, and a legacy scalar report have pinned results. Scalar output stays unchanged apart from the new invocation guard.
- Yes-or-no probability movement below, at, and above the tolerance keeps ticket 0042's rules inside a named question. Choice, tag, and score answers have `{"tolerance":0.08,"compared":0,"changed":0,"summarized_same_value":0,"largest_summarized_delta":null}` at the default tolerance.
- Missing or mismatched `value` and `answers` names, a mismatched nested value, an invalid question, threshold, answer, set digest, named value, or mixed row shapes fail with fixed diagnostics. Hostile ids, evidence, question names, question text, and answer fields do not appear in diagnostics.
- `jq -s`, ordinary piped input, and a literal-null input file all fail with `compare: run with jq -n`. The documented `jq -n --slurpfile before OLD -f compare.jq NEW` form succeeds.
- How-to 14 compares two replayed or committed safe runs and shows one named question's result. The page stays within its limits. The focused test, executable transform page, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0042.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 3
- Cost of error: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: this adds one public report shape and must prove four answer kinds, malformed rows, secrecy, scalar compatibility, and the invocation trap. It adds no network, process, or durable state.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation changes saved result rows, accepts more than the documented question value shapes, or touches code outside transforms and their pages.

## Review

The independent design review rejected the first draft because it left the nested report fields and empty-run mode implicit, allowed inconsistent question sets inside one run, described malformed rows too loosely, and misnamed the zeroed probability summary. A second pass found that the exact per-question shape omitted `compared`. The accepted design fixes the top-level and per-question shapes, makes mode inference exact, pins every field the transform reads and every supported answer-kind/value pairing, and gives the exact non-yes-or-no probability object. The reviewer accepted the invocation guard, scope, tag ordering, record mismatch rules, secrecy requirements, and the decision to leave page 16 for its next ticket.
