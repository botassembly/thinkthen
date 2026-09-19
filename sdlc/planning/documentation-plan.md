# Documentation plan

Written 2026-09-19, rewritten to ADR 0016 by ticket 0018. ADR 0011 rules that one file is the demo, the how-to, and the test. ADR 0016 cut the list from 40 pages to 27 and put limits on each. This page holds the list. Each line names the task, the demo that teaches it, the slice of `plan.md` that turns it green, and its state. A demo number is a folder under `demos/`. New numbers are reserved here and get a folder when their slice starts.

The published documentation has three kinds of page. How-tos are the green demos. Reference is `specification/`, with its executable examples in `spec/`. The README is the one tutorial and the one explanation. Nothing is written twice.

## The form and the standard

The title starts with "How to" and names a task. One paragraph says when to use it, and a block that asserts something follows inside the first twenty lines. Then: the input files, the steps with commands and real outputs, "What can go wrong" with the exit codes and the traps, and related how-tos.

ADR 0016 adds the limits: at most 120 lines and 900 words, at most six asserting blocks and every `bash` block asserts, one command unless the title names the contrast, at most four steps, no design argument on a green page, and at most four closing links. `sdlc/scripts/demos` measures every one of them and names the page, the rule, and the number when a page breaks one. `sdlc/scripts/demos-self-test` proves each check against a page that breaks it.

## Start here

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Gate a script step on a yes/no answer | 01 | 4 | Green |
| Gate a risky command and fail closed: yes, no, unsure, and could not ask | 19 | Ticket 0010, rewritten by 0018 | Green |
| Test a script with no network, with `--record` and `--replay` | 27 | Ticket 0010 | Green |
| Point the tool at another server with `THINKTHEN_BASE_URL` | 18 | 13 | Red |

## Gates and branches

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Act only when the answer is sure, and send the rest to a person | 04 | 7 | Green |
| Branch on a label with `choose` and `case`, and file a whole folder by it | 02 | 5 | Green. It absorbed 05 under ADR 0016 |
| Tell "not stated" from "false" with `decide` and `choose` | 20 | 5 | Green |
| Route a request by how hard it is, with `score` and `jq -e` | 17 | 5, rewritten by 0018 | Green |
| Choose the next action from a list that changes at every step | 21 | 7, rewritten by 0018 | Green |
| Say what yes and no mean | 40 | 7b | Red. Coming with ticket 0017 |
| Tune a question once and use the same file in the test and in the gate | 41 | 7b | Red. Coming with ticket 0017 |

## Many records

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Keep only the records that match a meaning | 03 | 8 | Red. It absorbs 09, and one sentence names transcript compaction as the same command |
| Put the best matches first | 06 | 8 | Red |
| Resume a long run that stopped, with `--cache` and `--jobs` | 12 | 7 | Green. It owns `--cache` |
| Find the one line that answers a question | 15 | 11 | Red. Slice 11 measures the longer documents first |
| Serve a loop from one long-lived process | 42 | 8 | Red. New under ADR 0016: record mode through a Bash `coproc`, and no `serve` command |

## Many questions at once

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Screen one message for several hazards and a severity in one request | 39 | 9 | Red. New under ADR 0016, and it absorbs 08 |
| Add several judged columns in one pass, including a `score` column | 07 | 9 | Red. It absorbs 22 and 34 |
| Build a triage pipeline that drafts, blocks, or asks a person | 16 | 9 | Red. The flagship: all three question types, a policy in `jq`, and audit rows |

## Evals

Ian ruled on 2026-09-19 that evals are a first-class section of the how-tos. An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence. Ordinary code does the policies, the metrics, the comparisons, and the presentation. ADR 0016 cut this section from fifteen pages to eight, because seven of them taught half an idea each. Each how-to below is also a transform folder under `transforms/` where it has `jq` in it, as ADR 0012 proposes.

| How to | Demo | Slice | State |
| --- | --- | --- | --- |
| Grade a batch with a reusable definition of named checks, over structured cases | 14 | 9 | Red. Still written against the removed `report`. It absorbs 29 and 31 |
| Mix exact checks in `jq` with judged checks | 30 | 10b | Red |
| Keep a run that can be traced and replayed: cases, definition, rows, model version, recording | 23 | 9 | Red |
| Pick a threshold from labeled cases, and change it later without asking again | 13 | 10a, ticket 0008 | Green. `sweep.jq`, `score.jq`, and `band.jq` over committed rows. It absorbs 36 |
| Compare two runs: improvements, regressions, missing cases, and why a value changed | 24 | 10a, ticket 0008 | Green. `compare.jq` over two committed runs |
| Check the judge against human labels, and read whether a probability means what it says | 25 | 10a, ticket 0008 | Green. `counts.jq`, `score.jq`, and `calibration.jq`. It absorbed 38 and absorbs 35 and 26 |
| Test the judge with hard and hostile cases | 37 | 10b | Red. The live probe measured the hostile case first |
| Know what a run cost | 28 | 10a, ticket 0008 | Green. `cost.jq` over committed rows and a replayed row |

### Ian's six capabilities, and the how-tos that teach each

| Capability | How-tos |
| --- | --- |
| Structured cases with stable ids, one case or a batch | 14 |
| Reusable definitions that mix exact and judged checks and control what each judgment sees | 14, 30 |
| Detailed results that keep probabilities, keep checks apart, and tell no from unsure from missing from failed | 19, 25 |
| Traceable artifacts, an unchanged source, and a spreadsheet as another view | 23, 07 |
| Local reporting and comparison, with thresholds changed without asking again | 25, 13, 24, 28 |
| Validation of the judge itself | 25, 37 |

## Numbers that left

| Number | Where its idea went |
| --- | --- |
| 05 | 02, as the closing section that files a folder |
| 08 | 39 |
| 09 | 03 |
| 10, 11 | Left with the configuration file and `segment` under ADR 0010 |
| 22, 34 | 07 |
| 26, 35, 38 | 25 |
| 29, 31 | 14 |
| 32 | 19 |
| 36 | 13 |

No number is reused. The pages once planned for transcript compaction and for routing a request to a cheap or a strong model became a sentence in 03 and the whole of 17.

## Rules for the list

- A how-to enters when a user task needs it. A feature with no how-to here has no place in version one.
- A ticket that turns a demo green writes it in the form, holds it to the standard, and updates its line here.
- Every state on this page, in `demos/README.md`, and in the folders under `demos/` says the same thing.
