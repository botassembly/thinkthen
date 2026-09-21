---
flow: build
priority: 40
opens: transforms/sweep transforms/README.md demos/14-grade-a-batch demos/25-check-the-judge sdlc/planning
---

# 0049: Sweep tag and annotate against human labels

Status: proposed

## Outcome

One sweep checks every supported threshold-bearing answer shape against a human field. A `tag` run gets one independent decision sweep per label. An `annotate` run gets the established decision, choice, or tag report for each mapped named answer. Users keep human labels under meaningful field names instead of rewriting rows to `input.label`.

## Current facts and decisions

Page 14 checks one named `annotate` decision with an ad hoc equality count. Ticket 0045 deferred `tag` and named `annotate` because one row can hold several probabilities. Ticket 0048 separated record groups from answer groups and assigned this ticket the human-truth mapping. The existing sweep owns the arithmetic; this ticket routes rows into it and copies no metric formula.

This ticket makes these decisions. Ian can overturn them:

1. A standalone detailed `tag` run takes `--arg truth /input/human_tags`. The RFC 6901 pointer is resolved against each whole result row. Missing or null truth marks that case unlabeled for every tag. Present truth is a unique array drawn from the question’s label list. Another shape, duplicate, or unknown label fails safely.
2. A tag run must have unique string case ids and one exact saved-result shape: `question.verb` is `tag`; every row shares the same question object, ordered unique string labels, and numeric threshold from zero through one; `answer.kind` is `tag`; probabilities have exactly those label keys with numbers from zero through one; and `value` is exactly the labels at or above the stored threshold in question order. The same rules apply to a tag nested in `annotate`.
3. A tag report has keys `{mode,truth,rows,labels}`. `mode` is `"tag"`. `labels` follows question order. Each entry has `label` followed by the unchanged decision fields `{rows,labeled,unlabeled,pick,sweep}`. Membership in the human array is boolean truth. Each label selects its own highest-F1 cut. The report has no combined score.
4. A detailed `annotate` run takes `--argjson truth '{"correct":"/input/human_correct"}'`. The nonempty object maps existing question names to RFC 6901 pointer strings. Partial mappings are deliberate. Every mapped answer must be `yes_no`, `choice`, or `tag`. Missing or null pointed values are unlabeled. Present truth must be boolean for yes/no, one listed option for choice, or a valid tag array. Invalid present truth fails instead of becoming unlabeled.
5. An annotate run has object `input`, a unique string case id, object `value` and `answers` with identical keys, and one shared `meta.questions_sha256` of exactly 64 lowercase hexadecimal characters. Every row has the same answer names. Each mapped question object, threshold, answer kind, option list, or label list is shared across the run. Unknown mapped names, score or find answers, changed definitions, malformed rows, and duplicate ids fail without echoing row, question, label, or pointer data.
6. An annotate report has keys `{mode,truth,rows,questions}`. `mode` is `"annotate"`; `truth` repeats the mapping; and `questions` follows mapping order. Its exact entries are:
   - decision: `{verb:"decide",rows,labeled,unlabeled,pick,sweep}`;
   - choice: `{verb:"choose",rows,labeled,unlabeled,options,sweep}`;
   - tag: `{verb:"tag",rows,labels:[{label,rows,labeled,unlabeled,pick,sweep}]}`.
   The nested entries omit only the surrounding `mode` and `truth`. Tag label entries keep their own row count because each is a complete decision report.
7. Named arguments fail explicitly. `group` and `truth` are mutually exclusive. A nonempty tag or annotate run without `truth` names the required argument. `truth` on scalar decide, choose, or score rows is refused. Tag requires a string truth pointer; annotate requires an object map. Empty input with `truth` is refused because no row identifies the result shape. Existing calls without `truth` or `group` and grouped decision calls stay byte-identical.
8. Page 14 replaces its equality block with the named sweep for `correct`. Page 25 adds one compact explanation within its limits. The page-14 replay is the outside check. Synthetic tag rows prove the multi-label behavior. No paid call or new fixture text is needed.

## Scope

Extend `sweep.jq` red-green by adapting tag labels and mapped annotate answers into its existing report functions. Add focused tests and executable examples, replace page 14’s equality block, update page 25 and the transform index, and update both active plans. Make no product-code change and no live call.

Excluded: score and find answers, exact field equality, exact-set tag accuracy, averages across questions or labels, calibration by named answer, repeated annotate trials, record-group combinations, custom grids, product options, and a report command.

## Acceptance

- Every existing valid sweep fixture and grouped-record report stays byte-identical without `truth`.
- Synthetic standalone tag rows prove ordered labels, independent best cuts, positive and negative membership, one wholly unlabeled row, zero denominators, exact key order, and no combined score. Missing/null truth is unlabeled. Invalid truth, question, threshold, probability keys or values, stored value, ids, and cross-row changes fail with fixed data-free diagnostics.
- Synthetic annotate rows prove decision, choice, and tag entries in the exact shapes above; partial and reordered maps; shared definitions; missing/null truth; invalid present truth; empty maps; unknown names; score refusal; malformed digest; changed definitions; duplicate and malformed ids; pointer escapes; and data-free errors.
- Calls pin fixed refusals for tag and annotate without truth, truth on scalar rows, group with truth, the wrong truth top-level type, and empty input with truth. Non-null ordinary input still fails once naming `jq -n`.
- Replaying page 14 and mapping `correct` to `/input/human_correct` reports six labeled rows and picks cut 0.6 with accuracy and F1 of 1. The saved 0.2:0.8 band left two labeled cases unresolved; the sweep resolves all six at the fitted cut. The page uses the transform rather than an equality count.
- Page 14 and page 25 remain within 120 lines and 900 words. Focused checks, executable examples, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0048.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: one pure transform routes three answer shapes through existing arithmetic, but it must validate nested result rows and human truth without hiding mislabeled cases. It adds no state, network, dependency, or product surface.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation duplicates metric arithmetic, adds aggregate scores, supports score or find, or changes an existing report.

## Review

The independent design review rejected the first draft because it left nested report keys ambiguous, validated human tag truth without validating the saved tag result, left argument combinations undefined, allowed a merely nonempty question-set digest, and used scores outside the routing rubric. This rewrite fixes every output key, validates the complete stored tag shape, defines the dispatch matrix, requires a 64-character lowercase hexadecimal digest, and records the corrected level-3 score.
