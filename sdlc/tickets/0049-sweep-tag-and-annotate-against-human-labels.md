---
flow: build
priority: 40
opens: transforms/sweep transforms/README.md demos/14-grade-a-batch demos/25-check-the-judge sdlc/planning
---

# 0049: Sweep tag and annotate against human labels

Status: proposed

## Outcome

One sweep checks every supported threshold-bearing answer shape against a human field. A `tag` run gets one independent decision sweep per label. An `annotate` run gets the established decision, choice, or tag report for each mapped named answer. Users keep their human labels under meaningful field names instead of rewriting rows to `input.label`.

## Current facts and decisions

Page 14 currently checks one named `annotate` decision with an ad hoc equality count. Ticket 0045 deferred `tag` and named `annotate` answers because one row can hold several independent probabilities. Ticket 0048 separated record groups from answer groups and assigned this ticket the remaining human-truth mapping. The existing sweep already owns all decision and choice arithmetic; this ticket must route rows into it rather than copy those formulas.

This ticket makes these decisions. Ian can overturn them:

1. `sweep.jq` keeps every current call and JSON value unchanged. A standalone detailed `tag` run takes `--arg truth /input/human_tags`. The pointer is resolved against each whole result row. A human truth that is missing or null marks that case unlabeled for every tag. A present truth is a unique array of labels from the question’s shared ordered label list; another shape, duplicate, or unknown label fails with a fixed data-free message.
2. The tag report has `mode: "tag"`, `truth`, `rows`, and `labels`. `labels` follows question order. Each entry has `label` followed by the unchanged decision report fields. Membership in the human array is the boolean truth. Every label selects its own highest-F1 cut; no micro, macro, or exact-set score is invented.
3. A detailed `annotate` run takes `--argjson truth '{"correct":"/input/human_correct"}'`. The object maps one or more existing question names to RFC 6901 pointers. Every name and pointer must be a string, and every mapped answer must be `yes_no`, `choice`, or `tag`. Missing or null pointed values are unlabeled. A yes-or-no truth must otherwise be boolean, a choice truth must be one listed option, and a tag truth follows the standalone array rule. Invalid present truth fails instead of silently becoming unlabeled.
4. The annotate report has `mode: "annotate"`, `truth`, `rows`, and `questions`. `questions` is an object in the mapping’s order. A mapped decision has `verb: "decide"` plus the unchanged decision report. A mapped choice has `verb: "choose"` plus the unchanged choice report. A mapped tag has `verb: "tag"` plus its tag report. The nested reports omit their redundant top-level `mode`, `truth`, and `rows` only where the surrounding question already states them.
5. One annotate run has object `input`, one shared nonempty question-set digest, identical named answer keys on every row, and a valid unique string case id on every row. Each mapped question definition, threshold, answer kind, option list, and label list is shared across the run. Malformed rows, an unmapped name, score or find answers, changed definitions, and duplicate ids fail with fixed messages that echo no row, question name, label, or pointer value.
6. Partial mappings are deliberate. A team may have human truth for `correct` and none for `grounded` or `severity`. Every mapped supported answer is swept, and unmapped answers remain visible in the saved run without entering the report. An empty truth map is refused because it would report nothing.
7. Page 14 replaces its ad hoc equality block with the named sweep for `correct`. Page 25 adds one compact cross-link and explanation within its limits. The committed page-14 replay is the outside check. Synthetic tag rows supply the multi-label proof; no paid call or new fixture text is needed.

## Scope

Extend `sweep.jq` red-green by adapting tag labels and mapped annotate answers into its existing report functions. Add focused tests and executable examples, replace page 14’s equality block, update page 25 and the transform index, and update both active plans. Make no product-code change and no live call.

Excluded: score answers, find answers, exact field equality, exact-set tag accuracy, averages across questions or labels, calibration by named answer, repeated annotate trials, record-group combinations, custom grids, product options, and a report command.

## Acceptance

- Every existing valid sweep fixture and grouped-record report stays byte-identical without `truth`.
- Synthetic standalone tag rows prove ordered labels, independent best cuts, positive and negative membership, a wholly unlabeled row, zero denominators, and no combined score. Missing/null truth is unlabeled; wrong containers, duplicates, and unknown labels fail safely.
- Synthetic annotate rows prove a decision, a choice, and a tag in one question set; partial and reordered truth maps; shared questions and thresholds; exact nested report shapes; missing/null truth; invalid present truth; empty maps; unknown names; score refusal; changed definitions; duplicate and malformed ids; pointer escapes; and data-free errors.
- Replaying page 14 and mapping `correct` to `/input/human_correct` reports six labeled rows and picks cut 0.6 with accuracy and F1 of 1. The saved 0.2:0.8 band had left two of those labeled cases unresolved; the sweep shows that the stored probabilities can resolve all six at the fitted cut. The page uses the transform rather than an ad hoc equality count.
- Non-null ordinary input still fails once naming `jq -n`. Page 14 and page 25 remain within 120 lines and 900 words. Focused checks, executable transform examples, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0048.

## Complexity

- Contract: 3
- State and timing: 0
- Reach: 1
- Proof: 3
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: one pure transform routes three answer shapes through existing arithmetic, but it must validate nested result rows and human truth without hiding mislabeled cases. It adds no state, network, dependency, or product surface.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation duplicates metric arithmetic, adds aggregate scores, supports score or find, or changes an existing report.

## Review

Pending independent design and code review.
