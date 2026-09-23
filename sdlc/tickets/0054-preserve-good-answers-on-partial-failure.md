---
flow: build
priority: 36
opens: crates specification conformance probes sdlc/scripts sdlc/planning
---

# 0054: Preserve good answers when one question fails

Status: landed

## Outcome

An otherwise valid backend reply can fail one logical question without discarding its good answers. `annotate` prints the good named values, marks the failed name as `failed`, reports the number of failed logical questions in metadata, and exits 6 after completing the input. `null` continues to mean `not sure` and never means failed.

## Current facts and decisions

The System One adapter currently turns one missing, wrong-shaped, or invalid answer into one `DecodeError`, so every good answer in the same paid reply is lost. `Reply` can carry only `Answer`. `annotate` groups several logical questions into one request and is the current command that can expose partial success. `tag` expands one logical question into several wire questions.

Ian approved the partial-success shape and distinct exit code on 2026-09-21 in `sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`. The vocabulary section titled “The words for numbers” rules the words below. Ian can overturn any item.

1. One decoded logical question is either answered or failed. A failure serializes as `{"failed":{"kind":"backend","cause":CAUSE}}`. The closed cause list is `missing_answer`, `wrong_kind`, `missing_probability`, `invalid_probability`, `invalid_distribution`, and `unexpected_probability`.
2. In bare `annotate` output, a failed name holds that marker in place of its value. In detailed output, the same name under `answers` carries `question`, `failure`, and `request`; `failure` holds the same `{"kind":"backend","cause":CAUSE}` object. The detailed entry carries no `value`, `answer`, or `threshold`. Successful entries keep their current shape.
3. Detailed metadata always carries `failed_questions`, including zero. It counts failed logical questions in that result. A failed `tag` counts once even when one of its several wire answers caused the failure.
4. Exit 6 means that the command completed its input and printed every result, but at least one logical question failed. `annotate` continues through later groups and records after a partial reply. It prints no diagnostic merely because it exits 6; the marker identifies each failure. A later whole-run failure keeps its existing code and stopping behavior.
5. Partial decoding requires at least one valid logical answer in that reply. A reply with no valid logical answer remains a backend failure at exit 4. The one-question commands and a standalone `tag` therefore keep their existing behavior and never invent a bare failure value.
6. Missing answers, wrong answer kinds, missing or extra probabilities, out-of-range probabilities, and invalid distribution totals fail only their logical question when another logical question in the same reply is valid. One bad wire member fails its whole logical `tag` question.
7. Malformed top-level JSON, missing or invalid model identity, an unexpected answer name, transport or authentication failure, and recording failure remain whole-request failures. This ticket adds the missing exact-answer-name check; an extra `qN` is never silently ignored.
8. Recording stores the raw response as it does today. A replay produces the same good answers, failed markers, failure count, and exit 6 without a request.
9. Amend ADR 0017 for the shared result shape and ADR 0007 for exit 6 before changing the settled specifications. Use `not sure` and `failed` in new prose. The separate vocabulary correction remains outside this ticket.

## Scope

Add a typed answered-or-failed outcome to the pure core, salvage failures while decoding a valid multi-question response, and teach `annotate` and its scheduler to render and accumulate partial results. Add the failure marker and count to public result types. Extend the shared conformance document and offline validator with one mixed `annotate` reply. Update the result, annotate, adapter, channels, recording, conformance, ADR, and active-plan pages required by the behavior.

Excluded: library and SQL behavior, the C interface, `recognize`, `relate`, backend profiles, the crate move, record-return changes, cache defaults, retries, transport failures, paid probes, broad vocabulary repair, and a public engine function. The conformance document records that typed single calls, bulk library calls, and database calls still need their ruled host forms before C freezes.

## Acceptance

- A mixed `annotate` reply with valid `decide`, `choose`, `score`, and `tag` answers plus one failure of each cause preserves every valid value and gives each failed logical name the exact marker.
- Bare and detailed output pin the exact marker, omitted detailed fields, `meta.failed_questions`, question-set order, per-answer request identity, usage, model, and aggregate request list.
- One failed wire label makes one logical `tag` failed marker while valid neighboring logical questions survive. An all-failed group and every one-question command still fail at exit 4 and print no answer.
- Missing, wrong-kind, missing-probability, invalid-probability, invalid-distribution, and unexpected-probability cases are salvageable. Malformed JSON and invalid model identity fail the whole reply. A reply with a valid planned answer plus an unexpected answer name also fails as a whole, proving that the extra name overrides salvage and is never ignored.
- A document with a partial reply prints its result and exits 6. A record stream with partial replies completes every record in input order and exits 6. A later whole-run failure keeps its existing code and stop boundary.
- Live and replay paths produce identical output apart from `meta.replayed`; replay sends no request. Existing request and recording bytes do not change.
- `meta.failed_questions` is zero on ordinary detailed results from all eight commands. One text passed to `decide` without `--details` still prints one bare Boolean and keeps its answer exit code.
- The shared conformance file adds one partial-success case. Its validator uses the production decoder and checks the marker, count, logical request identity, and distinction from a valid `null` answer.
- Focused decoder, result, annotate, scheduling, recording, conformance, and secrecy tests pass. All four repository rungs and `git diff --check` pass with the key unset and no network request.

## Dependencies

Ticket 0053 and the approval in `sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 2
- Proof: 2
- Cost of error: 2
- Total: 10
- Minimum level floor: level 3 for partial failure through concurrent scheduling
- Final level: 4
- Reasons: the change crosses the adapter, public result types, concurrent record scheduling, recordings, conformance, and a settled exit-code contract. The failure boundary must preserve paid good answers without turning a broken answer into `not sure`. The outcome is irreducible because the decoder shape, rendered marker, metadata count, and exit status form one public behavior; splitting them would create an intermediate contract that either loses good answers or reports a clean run.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation needs a new request, retry rule, engine API, dependency, or host-language contract.

## Review

Independent design review rejected the first proposal because it changed the approved detailed key from `failure` to `failed`, recorded level 3 for a rubric total of 10, and did not make the unexpected-answer proof override an otherwise salvageable reply. This revision restores the approved key, records irreducible level 4 with Sol Medium, and makes the hostile extra-name case exact.

Independent code review rejected the first implementation because its old successful-answer accessor could shift question alignment, it replaced the ordinary successful annotate conformance case, and its compiled JSON proof checked fragments instead of the complete result shape. The repair keeps only ordered answered-or-failed outcomes, retains both conformance cases, and parses the compiled result to pin the exact failed entry and metadata. The same reviewer accepted the repair.
