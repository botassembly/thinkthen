---
flow: build
priority: 41
opens: transforms/sweep transforms/README.md demos/13-pick-a-threshold sdlc/planning
---

# 0048: Sweep decision rows by record group

Status: proposed

## Outcome

The existing sweep can fit and report one decision cut for each named group of records, such as one cut per document type. Each group stands on its own, so a large group cannot choose the cut for a small one.

## Current facts and decisions

The triage reference asks for “one cut per group of records, such as one per document type.” The current decision sweep already owns the cut grid, metrics, tie rule, and automatic pick. Repeating that arithmetic in a second transform would create two meanings for the same report.

Ticket 0045 also used “grouped sweep” for independent `tag` labels and named `annotate` answers. This ticket names its narrower record-group job plainly. The next human-label ticket must map trusted labels and sweep every `tag` label and every threshold-bearing named `annotate` answer. Both active plans keep that obligation visible.

This ticket makes these decisions. Ian can overturn them:

1. `sweep.jq` keeps its current JSON value with no arguments. `--arg group /input/document_type` turns on grouped decision mode. The argument is an RFC 6901 JSON Pointer into each detailed result row. Relative and malformed pointers fail with fixed data-free messages. The valid empty pointer resolves the whole row, then fails the rule that a group value must be a string.
2. Grouped mode accepts `answer.kind: "yes_no"` rows only. It retains current decision validation and repeated-present-id handling. Existing sweep modes do not require a present string id on every row, so this ticket does not add that unrelated rule. Choice and score reports have no automatic pick; grouped mode refuses them rather than repeat tables that fit no cut.
3. The report has `mode`, `group`, `rows`, and `groups`, in that order. `mode` is `"grouped_decide"`; `group` is the pointer. `groups` is an array ordered lexically by group name. Each entry has `value` followed by the unchanged decision report fields: `rows`, `labeled`, `unlabeled`, `pick`, and `sweep`.
4. Every nonempty record belongs to exactly one string-valued group. A missing, null, or non-string group is an error rather than a silent extra bucket. Empty input succeeds with an empty `groups` array because no pointer must be resolved.
5. Every group selects its own highest-F1 cut with the existing middle-of-ties rule. The transform prints no pooled pick and no weighted or macro average. A reader sees each group’s row and label counts beside its cut and decides whether the evidence is sufficient.
6. `sweep.jq` currently accepts non-null ordinary input, so `jq -s` can print a tidy wrong empty report. This ticket adds the same first-line guard as the other repaired whole-run transforms: every mode accepts only null ordinary input and otherwise fails once with a fixed message naming `jq -n`.
7. The transform makes no request and adds no product option. How-to 13 replaces or trims existing text to make room for one compact example. It says that separate cuts need different operating needs and enough labeled cases. One global cut remains the simpler default.

## Scope

Extend `sweep.jq` red-green without changing its ungrouped output for valid `jq -n` calls, add focused grouped cases to its test and example, and replace or trim text in how-to 13 for the grouped example within its present limits. Update both active plans with this result and the still-open answer-group sweep. Make no live call.

Excluded: question-level human-label mappings, `tag`, `annotate`, grouped choice or score tables, nested groups, pooled metrics, minimum sample rules, significance claims, custom grids, product code, and a report command.

## Acceptance

- Every existing valid `jq -n` sweep fixture stays byte-identical without a `group` argument.
- Synthetic decision rows in two document types prove independent best cuts, lexical group order, exact per-group rows and labels, a tied best cut in one group, an unlabeled row in the other, and no pooled pick.
- Adding many rows with fresh unique ids to one group leaves the other group’s report byte-identical.
- `/cohort`, `/input/type~1name`, and `/input/type~0name` resolve exact top-level and escaped member names. Relative and malformed pointers fail with pointer diagnostics. Empty, missing, null, and non-string results fail with the fixed string-group diagnostic. No error echoes a pointer value from a row.
- Grouped choice, grouped score, mixed, malformed, and repeated-present-id rows remain refused with fixed messages. Missing and non-string ids retain their current behavior and focused cases pin it.
- Empty input yields `{"mode":"grouped_decide","group":"/input/document_type","rows":0,"groups":[]}`. Any non-null ordinary input, including `jq -s`, fails once with a fixed message naming `jq -n` in grouped and ungrouped modes.
- How-to 13 stays at no more than 120 lines and 900 words. The focused test, executable transform page, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0047.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: one pure transform adds an optional partition before unchanged decision arithmetic and closes its invocation trap. The proof must pin pointer errors, independent groups, and unchanged valid ungrouped output. It adds no state, network, dependency, or product surface.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation adds another transform, duplicates sweep arithmetic, accepts per-question label maps, or changes a valid ungrouped report.

## Review

The independent design review rejected the first draft because it left the earlier `tag` and `annotate` sweep promise unowned, described two absent validations as existing behavior, rejected the valid empty JSON Pointer, proposed an independence test that would trip the repeated-id guard, and tried to add words to a page already near its limit. This rewrite assigns answer-group sweeps to the next ticket and both plans, treats the `jq -n` guard as new work, matches RFC 6901, gives added rows fresh ids, and requires the page to replace or trim text.

