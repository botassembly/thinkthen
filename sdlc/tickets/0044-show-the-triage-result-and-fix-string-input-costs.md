---
flow: build
priority: 45
opens: demos/16-triage-pipeline transforms/cost demos/28-what-a-run-cost sdlc/planning
---

# 0044: Show the triage result and fix string-input costs

Status: in progress

## Outcome

The flagship triage page shows the six decisions a person would inspect and states its two-part idea before the safe staging script. The cost transform accepts a detailed result whose `input` is a string and lists a missing usage count without crashing.

## Current facts and decisions

The accepted page 16 feedback asks for `id`, `subject`, `action`, and `reason` on all six fictional tickets, plus the short idea that `thinkthen annotate` judges and `jq -f triage.jq` applies the rules. The current first block prints four totals alone. The same feedback reproduces `cost.jq` failing at `$row.input.id` when `input` is a string.

This ticket makes two small decisions. Ian can overturn either:

1. `demos/16-triage-pipeline/run` reads the three JSONL audit files and prints a page-local human display first: six headerless TSV lines ordered by `input.id`, with id, subject, action, and reason in that order. It then keeps the four existing totals. The page's first block pins all ten lines and explains the four columns in prose. The product and saved audit files still emit JSONL; this adds no TSV output mode. A sentence before the complete staging script says: `thinkthen annotate ... | jq -f triage.jq`. The tool judges; the rules decide.
2. `cost.jq` uses `input.id` only when `input` is an object. A string, number, boolean, null, array, or object without an id uses the existing public label `with no id`. Usage and price calculations do not change.

## Scope

Change the page-local `run` output and its self-test, page 16, `cost.jq`, and how-to 28. Add the compact synthetic string-input case to how-to 28 as the focused executable cost proof. Keep both pages within 120 lines and 900 words. Use replay and synthetic rows only.

Excluded: policy changes, new actions or reasons, new output formats, command changes, transform redesign, pricing changes, and paid calls.

## Acceptance

- The first page 16 command shows all six ids and subjects with the exact reviewed action and policy reason, followed by `draft=2`, `block=1`, `review=3`, and `agreement=6/6`. The order is lexical by id and does not depend on the three output filenames.
- The short `annotate | jq` idea appears before the complete safe script. The page's existing staging, disclosure, failure, and question-set cautions remain accurate, and the page stays within its limits.
- A focused cost check feeds one detailed row with string `input`, missing `meta.usage`, and a hostile string. It succeeds, reports `no_usage:["with no id"]`, charges no tokens, and echoes no input text. Existing object-input totals stay pinned.
- Page 16, how-to 28, the triage self-test, the executable transform page, all four repository rungs, and `git diff --check` pass. No live call runs.

## Dependencies

Tickets 0041 and 0043.

## Complexity

- Contract: 1
- State and timing: 0
- Reach: 1
- Proof: 1
- Cost of error: 1
- Total: 4
- Minimum level floor: none
- Final level: 2
- Reasons: this changes one page-local display and closes one small transform type error with focused replay and synthetic proof. It changes no command, wire shape, state, or network path.
- Selected model: `gpt-5.6-luna` with high reasoning.

Re-score if implementation changes triage policy, cost arithmetic, or a product command.

## Review

The independent design review accepted the scope, id ordering, hostile-string proof, and level-2 Luna High route. It required this final draft to say that TSV is a page-local view over JSONL audit files rather than a product format, and to name how-to 28's synthetic block as the focused proof while holding that page to the same 120-line and 900-word limits.
