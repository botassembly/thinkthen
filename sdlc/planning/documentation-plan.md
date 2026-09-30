# Documentation plan

Written 2026-09-19, rewritten to ADR 0018 by ticket 0021. ADR 0011 rules that one file is the demo, the how-to, and the test. ADR 0016 cut the list from 40 pages to 27 and put limits on each. ADR 0018 cut it again to 20, keeping the limits. Later tickets added focused pages, including page 45 for `relate`, page 46 for cache inventory and page 47 for a two-call question. `demos/README.md` holds the one list of how-tos, with each page's number, task, link and state. This page keeps the form, the capability map, the numbers that left and the rules. The slice or ticket that turned each page green is in Git history and in that page's ticket. A demo number is a folder under `demos/`. A new number is reserved on `demos/README.md` as coming and gets a folder when its slice starts.

The published documentation has three kinds of page. How-tos are the green demos. Reference is `specification/`, with its executable examples in `spec/`. The README is the one tutorial and the one explanation. Nothing is written twice.

`sdlc/scripts/pages` checks that every relative link here and in `demos/` resolves.

## The form and the standard

The title starts with "How to" and names a task. One paragraph says when to use it, and a block that asserts something follows inside the first twenty lines. Then: the input files, the steps with commands and real outputs, "What can go wrong" with the exit codes and the traps, and related how-tos.

ADR 0016 adds the limits: at most 120 lines and 900 words, at most six asserting blocks and every `bash` block asserts, one command unless the title names the contrast, at most four steps, no design argument on a green page, and at most four closing links. Amended 2026-09-30 by ticket 0312 under Ian's ruling 8: these limits are writing guidance. `sdlc/scripts/demos` checks that a green page and each of its `bash` blocks assert, and that every page `demos/README.md` lists as green says `Status: green`.

## Evals

Ian ruled on 2026-09-19 that evals are a first-class section of the how-tos. An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence. Ordinary code does the policies, the metrics, the comparisons, and the presentation. ADR 0016 cut this section from fifteen pages to eight, and ADR 0018 cut it to four, because the rest taught half an idea each or repeated a neighbour. Each eval how-to is also a transform folder under `transforms/` where it has `jq` in it, as ADR 0012 proposes.

The four eval how-tos are 14, 13, 25 and 28, listed under "Evals" in `demos/README.md`.

### Ian's six capabilities, and the how-tos that teach each

| Capability | How-tos |
| --- | --- |
| Structured cases with stable ids, one case or a batch | 14 |
| Reusable definitions that mix exact and judged checks and control what each judgment sees | 14 |
| Detailed results that keep probabilities, keep checks apart, and tell no from unsure from missing from failed | 19, 25 |
| Traceable artifacts, an unchanged source, and a spreadsheet as another view | 27, 14, 16 |
| Local reporting and comparison, with thresholds changed without asking again | 25, 13, 41, 28 |
| Validation of the judge itself | 25 |

## Numbers that left

| Number | Where its idea went |
| --- | --- |
| 04 | 19 for the band, 16 for the review pile, and 13 for the trade between coverage and accuracy. Its folder left with ticket 0041 |
| 05 | 02, as the closing section that files a folder |
| 07 | 14 for several judged columns on one record, and 16 for the spreadsheet view. Its folder left with ticket 0041 |
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
- A ticket that turns a demo green writes it in the form, holds it to the standard, and updates its line in `demos/README.md`.
- Each state in `demos/README.md` matches its folder's status line. `sdlc/scripts/demos` refuses a page listed as green whose status line differs.
