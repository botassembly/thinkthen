# Documentation plan

Written 2026-09-19. ADR 0011 rules that one file is the demo, the how-to, and the test. This page holds the full list of how-tos. Each line names the task, the demo that teaches it, the slice of `plan.md` that turns it green, and its state. A demo number is a folder under `demos/`. New numbers are reserved here and get a folder when their slice starts.

The published documentation has three kinds of page. How-tos are the green demos. Reference is `specification/`, with its executable examples in `spec/`. The README is the one tutorial and the one explanation. Nothing is written twice.

## The form of a how-to

The title starts with "How to" and names a task. One paragraph says when to use it. Then: the input files, the steps with commands and real outputs, "What can go wrong" with the exit codes and the traps, and related how-tos. The gate runs every block against a committed recording. Ticket 0010 adds the check that refuses a green demo out of form.

## Start here

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Gate a script step on a yes/no answer | 01 | 4 | Green. Takes the form in ticket 0010 |
| Tell "no" from "could not ask" in a script, with `case $?` and under `set -e` | 19 | Ticket 0010 | Needs only `decide` |
| Test a script with no network, with `--record` and `--replay` | 27 | Ticket 0010 | Needs only `decide` |
| Point the tool at another server with `THINKTHEN_BASE_URL` | 18 | 13 | |

## Gates and branches

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Act only when the answer is sure, and send the rest to a person | 04 | 7 | Red |
| Branch on a label with `choose` and `case` | 02 | 5 | Red |
| Tell "not stated" from "false" | 20 | 5 | |
| Rate on a scale, sort by it, and test it with `jq -e` | 17 | 5 | `score` has no demo today |
| Sort files into folders by label | 05 | 5 | Red |

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
| Keep an audit trail of every judgment | 23 | 9 | |

## Thresholds and evals

Each of these is also a recipe folder under `recipes/`, as ADR 0012 proposes.

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Pick a threshold from labeled cases | 13 | 10a, ticket 0008 | Red. Starts now on `decide --details` rows |
| Grade a batch against trusted answers | 14 | 10a, ticket 0008 | Red. Its several-check form waits for slice 9 |
| Compare two runs case by case, and say why a value changed | 24 | 10a, ticket 0008 | |
| Check the judge against human labels | 25 | 10b | |
| Watch a running pipeline: actions, overturns, and one cut per group | 26 | 10b | |
| Know what a run cost | 28 | 10a, ticket 0008 | |

## Rules for the list

- A how-to enters when a user task needs it. A feature with no how-to here has no place in version one.
- A ticket that turns a demo green writes it in the form and updates its line here.
- Demos 10 and 11 left with the configuration file and `segment`. Their numbers are not reused.
