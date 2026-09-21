---
flow: build
priority: 41
opens: transforms/sweep transforms/README.md demos/13-pick-a-threshold sdlc/planning
---

# 0048: Sweep separate record groups

Status: proposed

## Outcome

The existing sweep can fit and report one decision cut for each named group of records, such as one cut per document type. Each group stands on its own, so a large group cannot choose the cut for a small one.

## Current facts and decisions

The triage reference asks for “one cut per group of records, such as one per document type.” The current decision sweep already owns the cut grid, metrics, tie rule, and automatic pick. Repeating that arithmetic in a second transform would create two meanings for the same report.

Ticket 0045 also used “grouped sweep” for the independent labels in `tag` and the named answers in `annotate`. Those rows need a mapping from each answer name to its human label. The next human-label ticket owns that mapping. This ticket uses the narrower meaning from the triage reference: partition otherwise ordinary detailed decision rows by a field, then run the existing sweep separately over each partition.

This ticket makes these decisions. Ian can overturn them:

1. `sweep.jq` keeps its current behavior with no arguments. `--arg group /input/document_type` turns on grouped decision mode. The argument is an RFC 6901 JSON Pointer into each detailed result row. The empty pointer, a malformed escape, a missing value, or a non-string value fails with a fixed data-free message.
2. Grouped mode accepts `answer.kind: "yes_no"` rows only. It retains every existing decision validation rule, including unique string case ids and `input.label` as the trusted boolean. Choice and score reports already have no automatic pick. Splitting them would repeat tables without fitting a cut, so grouped mode refuses them.
3. The report has `mode`, `group`, `rows`, and `groups`, in that order. `mode` is `"grouped_decide"`; `group` is the pointer. `groups` is an array ordered lexically by group name. Each entry has `value` followed by the unchanged decision report fields: `rows`, `labeled`, `unlabeled`, `pick`, and `sweep`.
4. Every record belongs to exactly one group. A missing group is an error rather than a silent extra bucket. An empty run succeeds with an empty `groups` array because there is no record on which to resolve the pointer.
5. Every group selects its own highest-F1 cut with the existing middle-of-ties rule. The transform prints no pooled pick and no weighted or macro average. A reader sees each group’s row and label counts beside its cut and decides whether the evidence is sufficient.
6. The transform makes no request and adds no product option. How-to 13 gains one short example and states that a separate cut is justified only when the groups have different operating needs and enough labeled cases. One global cut remains the simpler default.

## Scope

Extend `sweep.jq` red-green without changing its ungrouped JSON values, add focused grouped cases to its test and example, and add the compact grouped example to how-to 13 within its present limits. Update both active plans. Make no live call.

Excluded: question-level human-label mappings, `tag`, `annotate`, grouped choice or score tables, nested groups, pooled metrics, minimum sample rules, significance claims, custom grids, product code, and a report command.

## Acceptance

- Every existing sweep fixture stays byte-identical with no `group` argument.
- Synthetic decision rows in two document types prove independent best cuts, lexical group order, exact per-group rows and labels, a tie in one group, an unlabeled row in the other, and no pooled pick.
- A group with many rows cannot change another group’s report. Duplicating every row in one group leaves the other group byte-identical.
- Root and escaped pointer members work. Empty, relative, malformed, missing, null, and non-string group values fail once with exact data-free diagnostics. Choice, score, mixed, malformed, and repeated-id rows remain refused with fixed messages. Non-null ordinary input still names `jq -n`.
- Empty input yields `{"mode":"grouped_decide","group":"/input/document_type","rows":0,"groups":[]}`.
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
- Reasons: one pure transform adds an optional partition before unchanged decision arithmetic. The proof must pin JSON Pointer errors, independent groups, and byte-identical ungrouped output. It adds no state, network, dependency, or product surface.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation adds another transform, duplicates sweep arithmetic, accepts per-question label maps, or changes an ungrouped report.

## Review

Pending independent design and code review.

