# Documentation plan

Written 2026-09-19, rewritten to ADR 0018 by ticket 0021. ADR 0011 rules that one file is the demo, the how-to, and the test. ADR 0016 cut the list from 40 pages to 27 and put limits on each. ADR 0018 cut it again to 20, keeping the limits. This page holds the list. Each line names the number, the task, the slice of `plan.md` that turns it green, and its state. A demo number is a folder under `demos/`. New numbers are reserved here and get a folder when their slice starts.

The published documentation has three kinds of page. How-tos are the green demos. Reference is `specification/`, with its executable examples in `spec/`. The README is the one tutorial and the one explanation. Nothing is written twice.

`sdlc/scripts/pages` checks that this list, `demos/README.md`, and the folders under `demos/` agree on every number, title, and state, and that every link between them resolves. `sdlc/scripts/pages-self-test` proves each check against a copy that breaks it.

## The form and the standard

The title starts with "How to" and names a task. One paragraph says when to use it, and a block that asserts something follows inside the first twenty lines. Then: the input files, the steps with commands and real outputs, "What can go wrong" with the exit codes and the traps, and related how-tos.

ADR 0016 adds the limits: at most 120 lines and 900 words, at most six asserting blocks and every `bash` block asserts, one command unless the title names the contrast, at most four steps, no design argument on a green page, and at most four closing links. `sdlc/scripts/demos` measures every one of them and names the page, the rule, and the number when a page breaks one. `sdlc/scripts/demos-self-test` proves each check against a page that breaks it.

## Start here

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 01 | Gate a script step on a yes/no answer | 4 | green |
| 19 | Gate a risky command and fail closed | Ticket 0010, rewritten by 0018 | green |
| 27 | Test a script with no network | Ticket 0010 | green |
| 18 | Point the tool at another server and compare two deciders | 13 | coming, slice 13 |

## Gates and branches

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 02 | Branch on a label with `choose` and `case` | 5 | green |
| 40 | Say what yes and no mean | 7b | green |
| 17 | Route a request by how hard it is | 5, rewritten by 0018 | green |
| 21 | Choose the next action from a list that changes at every step | 7, rewritten by 0018 | green |
| 41 | Tune a question file and use the same file in the gate | 7b | green |

## Many records

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 03 | Keep only the records that match a meaning | 8 | green |
| 43 | Lint a change by meaning and fail the build | 8 | green |
| 06 | Put the best matches first | 8 | green |
| 12 | Resume a long run that stopped | 7 | green |
| 15 | Find the line that answers a question | 11 | green |

## Many questions at once

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 39 | Screen one message for several hazards | Ticket 0036: `tag` | green |
| 16 | Build a triage pipeline that drafts, blocks, or asks a person | 9 | coming, slice 9 |

## Evals

Ian ruled on 2026-09-19 that evals are a first-class section of the how-tos. An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence. Ordinary code does the policies, the metrics, the comparisons, and the presentation. ADR 0016 cut this section from fifteen pages to eight, and ADR 0018 cut it to four, because the rest taught half an idea each or repeated a neighbour. Each how-to below is also a transform folder under `transforms/` where it has `jq` in it, as ADR 0012 proposes.

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 14 | Grade an assistant's answers with a rubric | 9 | green |
| 13 | Pick a threshold from labeled cases | 10a, ticket 0008 | green |
| 25 | Check the judge against human labels | 10a, ticket 0008 | green |
| 28 | Know what a run cost | 10a, ticket 0008 | green |

### Ian's six capabilities, and the how-tos that teach each

| Capability | How-tos |
| --- | --- |
| Structured cases with stable ids, one case or a batch | 14 |
| Reusable definitions that mix exact and judged checks and control what each judgment sees | 14 |
| Detailed results that keep probabilities, keep checks apart, and tell no from unsure from missing from failed | 19, 25 |
| Traceable artifacts, an unchanged source, and a spreadsheet as another view | 27, 14, 16 |
| Local reporting and comparison, with thresholds changed without asking again | 25, 13, 41, 28 |
| Validation of the judge itself | 25 |

## Folders that stay and then leave

A green page leaves only when the page that absorbs it is green, so no lesson is ever missing. These two folders are not in the list of 20 and are still on disk.

| # | How to | Slice | State |
| --- | --- | --- | --- |
| 04 | Act only when the answer is sure, and send the rest to a person | Leaves when 16 is green | leaving, into 19, 16, and 13 |
| 07 | Judged columns | Leaves when 14 and 16 are green | leaving, into 14 and 16 |

## Numbers that left

| Number | Where its idea went |
| --- | --- |
| 04 | 19 for the band, 16 for the review pile, and 13 for the trade between coverage and accuracy. Its folder is still on disk |
| 05 | 02, as the closing section that files a folder |
| 07 | 14 for several judged columns on one record, and 16 for the spreadsheet view. Its folder is still on disk |
| 08 | 39. Its folder left with ticket 0015 |
| 09 | 03, whose `--dry-run` block proves what leaves the machine. Its folder left with ticket 0014 |
| 10, 11 | Left with the configuration file and `segment` under ADR 0010 |
| 20 | 40, which fixes "not stated" by saying what yes and no mean. Its pick with a `not_stated` label and its reading of a pick's odds are two traps on 40 |
| 22, 34 | 14 and 16, by way of 07 |
| 23 | 27 for the recording, and 14 for the row that carries the model version and the digests, once 14 is green |
| 24 | 41, where the comparison is step 3 of tuning, and `transforms/README.md`, which doctors a run and proves every pairing rule `compare.jq` claims |
| 26, 35, 38 | 25 |
| 29, 31 | 14 |
| 30 | 14, as one sentence: an exact check is a `jq` field on the record |
| 32 | 19 |
| 36 | 13 |
| 37 | 25, because hostile cases and human labels answer the same question about the judge. 25 is green and holds no hostile case yet, and a later ticket adds them |
| 42 | The README's section on what the tool is not for, which already names record mode through a `coproc` as the ceiling for a steady loop. ADR 0018 puts one sentence on 12, and no ticket has written it yet |

No number is reused. The pages once planned for transcript compaction and for routing a request to a cheap or a strong model became a sentence in 03 and the whole of 17.

## Rules for the list

- A how-to enters when a user task needs it. A feature with no how-to here has no place in version one.
- A ticket that turns a demo green writes it in the form, holds it to the standard, and updates its line here.
- Every state on this page, in `demos/README.md`, and in the folders under `demos/` says the same thing, and `sdlc/scripts/pages` fails the `lint` rung when one of them drifts.
