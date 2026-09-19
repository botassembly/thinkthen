# Documentation plan

Written 2026-09-19. ADR 0011 rules that one file is the demo, the how-to, and the test. This page holds the full list of how-tos. Each line names the task, the demo that teaches it, the slice of `plan.md` that turns it green, and its state. A demo number is a folder under `demos/`. New numbers are reserved here and get a folder when their slice starts.

The published documentation has three kinds of page. How-tos are the green demos. Reference is `specification/`, with its executable examples in `spec/`. The README is the one tutorial and the one explanation. Nothing is written twice.

## The form of a how-to

The title starts with "How to" and names a task. One paragraph says when to use it. Then: the input files, the steps with commands and real outputs, "What can go wrong" with the exit codes and the traps, and related how-tos. The gate runs every block against a committed recording. Ticket 0010 adds the check that refuses a green demo out of form.

## Start here

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Gate a script step on a yes/no answer | 01 | 4 | Green, in the how-to form |
| Tell "no" from "could not ask" in a script, with `case $?` and under `set -e` | 19 | Ticket 0010 | Green, in the how-to form |
| Test a script with no network, with `--record` and `--replay` | 27 | Ticket 0010 | Green, in the how-to form |
| Point the tool at another server with `THINKTHEN_BASE_URL` | 18 | 13 | |

## Gates and branches

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Act only when the answer is sure, and send the rest to a person | 04 | 7 | Green, in the how-to form |
| Branch on a label with `choose` and `case` | 02 | 5 | Green |
| Tell "not stated" from "false" | 20 | 5 | Green |
| Rate on a scale, sort by it, and test it with `jq -e` | 17 | 5 | Green |
| Sort files into folders by label | 05 | 5 | Green |

## Many records

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Keep only the records that match a meaning | 03 | 8 | Red |
| Put the best matches first | 06 | 8 | Red |
| Choose from a list that differs for every record | 21 | 7 | |
| Control what leaves the machine, and prove it with `--dry-run` | 09 | 8 | Red |
| Resume a long run that stopped | 12 | 7 | Red |
| Find the one line that answers a question | 15 | 11 | Red. Draft until the measurement |

## Many questions at once

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Add several judged columns in one pass | 07 | 9 | Red |
| Check a document against a checklist | 08 | 9 | Red |
| Build a triage pipeline that drafts, blocks, or asks a person | 16 | 9 | The flagship. It uses all three question types, a policy in `jq`, and audit rows |
| Ask one question over two scopes and route the disagreement | 22 | 9 | |

## Evals

Ian ruled on 2026-09-19 that evals are a first-class section of the how-tos. An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence. Ordinary code does the policies, the metrics, the comparisons, and the presentation. Each how-to below is also a recipe folder under `transforms/` where it has `jq` in it, as ADR 0012 proposes.

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Write an eval case file with stable ids, and grade one case | 29 | 9 | |
| Grade a batch with a reusable definition of named checks | 14 | 9 | Red. Still written against the removed `report` |
| Control what each check sees, so a grounding check never reads the trusted answer | 31 | 9 | |
| Mix exact checks in `jq` with judged checks | 30 | 10b | |
| Read a result: no, unsure, missing data, and a failed request | 32 | 9 | |
| Keep a run that can be traced and replayed: cases, definition, rows, model version, recording | 23 | 9 | |
| Turn results into a spreadsheet and leave the source unchanged | 34 | 10b | |
| Report accuracy and F1 for every check with no new request | 35 | 10b | |
| Pick a threshold from labeled cases, and change it later without asking again | 13 | 10a, ticket 0008 | Green. `sweep.jq`, `score.jq`, and `band.jq` over committed rows |
| Compare two runs: improvements, regressions, missing cases, and why a value changed | 24 | 10a, ticket 0008 | Green. `compare.jq` over two committed runs |
| Account for repeated trials and unsure answers | 36 | 10b | |
| Check the judge against human labels | 25 | 10a, ticket 0008 | Green. `counts.jq` and `score.jq` over committed rows |
| See whether a probability means what it says | 38 | 10a, ticket 0008 | Green. `calibration.jq` over committed rows |
| Test the judge with hard and hostile cases | 37 | 10b | The live probe measures the hostile case first |
| Know what a run cost | 28 | 10a, ticket 0008 | Green. `cost.jq` over committed rows and a replayed row |

### Ian's six capabilities, and the how-tos that teach each

| Capability | How-tos |
| --- | --- |
| Structured cases with stable ids, one case or a batch | 29, 14 |
| Reusable definitions that mix exact and judged checks and control what each judgment sees | 14, 30, 31 |
| Detailed results that keep probabilities, keep checks apart, and tell no from unsure from missing from failed | 32, 35 |
| Traceable artifacts, an unchanged source, and a spreadsheet as another view | 23, 34 |
| Local reporting and comparison, with thresholds changed without asking again | 35, 13, 24, 36, 28 |
| Validation of the judge itself | 25, 38, 37 |

## Watching a pipeline

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Watch a running pipeline: actions, overturns, and one cut per group | 26 | 10b | |

## Rules for the list

- A how-to enters when a user task needs it. A feature with no how-to here has no place in version one.
- A ticket that turns a demo green writes it in the form and updates its line here.
- Demos 10 and 11 left with the configuration file and `segment`. Their numbers are not reused.
