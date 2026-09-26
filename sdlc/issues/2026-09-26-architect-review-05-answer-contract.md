Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review. Findings 1 and 2: ticket 0160 on `ticket/0160-the-answer-contract-holds`. Finding 2.1 for single question files: ticket 0159 on `ticket/0159-pin-the-default-model`. Both are ready for review.

# Architect review 05: the question and answer contract

A fresh reviewer tested the question file, the answer shapes, thresholds and the `--details` line as an architect whose code acts on ThinkThen answers. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 6 severity 2 and 7 severity 3 issues. The full detail sits in the architect review report 273, 05. Probe names such as probe5 refer to the report's local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. A missing answer member sinks every answer in its request (severity 2)

Evidence. `backends.md` "The adapter contract" promises per-question failure when an answer "lacks a probability", and `result.md` lists `missing_probability`. A `noul` answer without `noul`, or a choice without `probabilities`, fails the whole reply at exit 4 (probe5, probe6). `ResponseAnswer` makes those fields mandatory at parse time (`core/adapters/systemone/response.rs:37-56`).

What an integrator hits. An annotate row with four good answers and one malformed member returns nothing and stops the run, instead of exit 6 with one marker.

Direction. Parse each answer member leniently and map a missing field to `missing_probability`, as the spec says. Add the missing-field case to `response_partial_tests.rs`.

## 2. The schema identifier does not version the output (severity 2)

Reviews 05 (issue 2.4) and 09 (issue 9) both found this. This file carries it.

Evidence from review 05. ADR 0036 renamed `meta.replayed` to `meta.cached` without an alias and kept `thinkthen.result/1`. `annotate`, `recognize` and `relate` emit different shapes under the same identifier (probe5; `recognize.md`, `relate.md`). No result JSON Schema is published, and no page states which changes keep `/1`.

Evidence from review 09. Beatles Bench `./run.sh` on a scratch copy, using the current build: 11 folders matched byte for byte, then `examples/audit/replay/audit-0.5.json` differed because of the new `by_bin` array. `meta.tool` in every `--details` line carries the version.

What an integrator hits. A consumer that validates on `schema` gets no warning when a field is renamed, and must sniff for `answers` against `answer` to know the shape. Tests that `cmp` saved outputs fail on every upgrade for reasons unrelated to answers.

Direction. Before 0.1, publish a result JSON Schema per shape and give aggregate shapes their own identifiers or a `shape` field. Write a compatibility rule: additions keep `/1`, and renames and removals bump it. Advise tests to compare `value` and probabilities, not bytes.

## Carried in other files

- 2.1, a tuned threshold silently follows a floating model alias, and 2.2, one run and one cache can mix model versions (severity 2). Both share one root cause with review 08's issue 1. The architect review 08 file carries them with review 05's evidence.
- 2.5, details differ by surface on main (severity 2). Ticket 0151 landed on main after the review base. Recheck DuckDB `thinkthen_details` against the `--details` line before closing this point. If 0151 covers it, nothing is owed.
- 2.6, accepted default batching will change what a per-record probability means, and today's tuned files will not warn (severity 2). See `2026-09-26-batching-design-review-before-0146.md` item 7.

## Severity 3 titles

- Tie policy differs across functions, and one policy is undocumented (score picks the lowest tied level, `core/answer.rs:71-78`).
- Only `decide` can express a measured not-sure region, and `find` answers even when nothing fits. Live, `find` without `--none` returned an unrelated line at 0.70 and exit 0.
- `--threshold` cuts a different quantity in each function, and a profile limit can switch relate's method under the same cut.
- Parse traps in the details line: `threshold` is a number, a string or null, and probabilities carry meaning in object order.
- The question file cannot carry a version, while a question set requires `version: 1`.
- A record run stops at the first failed record. Carried in the architect review 01 file.
- The contract pages disagree with each other and with output in small ways. `2026-09-25-command-wording-and-help-fixes-before-0-1.md` item 16 covers part of this.
