---
flow: build
priority: 50
opens: recipes demos/13-pick-a-threshold demos/14-grade-a-batch demos/24-compare-two-runs demos/28-what-a-run-cost demos/README.md sdlc/scripts sdlc/live-tokens sdlc/planning/documentation-plan.md
---

# 0008: Metric recipes over live `decide` rows

Status: ready

## Outcome

`recipes/` holds the metric recipes as folders, written in `jq`, tried on real saved rows, and taught by four green how-tos. The ticket's record says how clumsy each recipe was to write and to use. That verdict feeds Ian's five outcomes for `report` in ADR 0010 and the proposal in ADR 0012.

## Current Facts

`report` left the plan, and recipes come first (ADR 0010). ADR 0012 proposes that a recipe is a folder: the question or question file, the `.jq` files, one short script with the pipeline line, and a page that is a how-to under ADR 0011. Only `decide` is built, and record mode is not. A shell loop over a case file with `decide --details` produces rows today. `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` lists the rules a recipe must follow. Demos 13 and 14 are red and still written against the removed `report` command.

## Scope

- A case file of about forty made-up support messages with stable ids and a trusted yes/no label. A few cases are hard on purpose, so some answers land in the middle.
- One live job, run through `sdlc/scripts/live`, loops over the cases with `decide --details --record`, and writes one row per case. Each row is shaped the way `specification/records.md` and `result.md` give for `decide --jsonl --details`, so the recipes do not change when record mode lands. A second run with a reworded question gives the comparison recipe two real runs. Both runs and their recordings are committed.
- The recipes, each a `.jq` file with a header that states what it reads, its arguments, and its policies: counts of yes, no, and unresolved; accuracy, precision, recall, and F1 at a cut given through `--argjson`; a sweep over cuts; accuracy at coverage for a band; a comparison of two runs by case id; and the input tokens and cost of a run.
- Every recipe follows the issue's rules. An explicit three-way test stands where `//` would turn unresolved into no. Unresolved rows are counted apart and never scored right or wrong. A zero denominator yields `null` and the header says so. A case with no label is reported and never dropped silently. The comparison checks that paired cases carry the same input and the same label, lists the flips in each direction and the cases in only one run, and says whether the question or the model changed by reading `meta`.
- Demos 13 and 14 are rewritten over the recipes in the how-to form of ADR 0011, and demos 24 and 28 are added. All four turn green. Every block runs from committed files with no network. `demos/README.md` and `documentation-plan.md` follow.
- The `install` rung fails with a clear message when `jq` is missing.

Excluded: any change under `crates/`, any new command, the policy recipe, the monitors, and the grouped sweep. Those wait for `annotate`.

## Acceptance

- Each recipe has at least one block in a green how-to that pins its real output on the committed rows.
- A block proves the three-way rule: a row with an unresolved answer is counted apart by every metric recipe.
- The spec rung prints five green demos with the key unset and touches no network.
- The recordings hold no key: a search for the key prefix, `authorization`, and `bearer` finds nothing.
- The live tokens spent are added to `sdlc/live-tokens` by the script and to the table in `sdlc/planning/plan.md`.
- The record gives, for each recipe, its length in lines, every trap met while writing it, and a plain verdict: fine as a file, or clumsy enough to earn a command.
