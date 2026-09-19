# How-tos

Every page here is one shell job written so that it runs. A green page is the how-to, the demo, and the test at once, and `sdlc/scripts/spec` runs every block that asserts something against a committed recording. ADR 0011 rules that nobody writes a second copy.

A how-to is one of the four names in the table in [`../README.md`](../README.md).

ADR 0018 fixed this list at 20 pages, and `sdlc/planning/documentation-plan.md` holds the same list with the slice that turns each one green. `sdlc/scripts/pages` checks that the two lists and the folders here agree on every number, title, and state. A number here is a folder under `demos/`.

## How to read one

Each page opens with a status line and the verbs it uses. A green page then takes the how-to form: a title that starts with "How to", one paragraph on when to use it, a first result inside the first twenty lines, the input files, the steps with their commands and real outputs, a section named "What can go wrong" with the exit codes and the traps, and a closing list of related how-tos.

`mustmatch test demos/` runs a block when the block pipes into `mustmatch` and skips a block that asserts nothing, so a shown command is a test only when it ends in an assertion. The blocks assert on exit codes, bare values, field names, row counts, and the records that came back. No block asserts on a probability that is illustrative, and a page that pins a number says which recording measured it.

Every `thinkthen` command that would otherwise reach a backend carries `--replay` and a recording folder, so a gate touches no network and reads no key. `specification/recording.md` defines the flag.

## Red and green

A demo starts **red**, and this list marks it **coming** with the ticket or the slice that writes it. Only `decide`, `choose`, and `score` are built, so every page that needs another verb is still a plan. A red page argues for a design choice, and `FINDINGS.md` gathers those arguments across every page.

A demo turns **green** when the `spec` rung runs it against a recording and it passes. When it turns green it takes the how-to form and the argument leaves the page. `sdlc/scripts/demos` runs every page whose status line reads exactly `Status: green` and skips every red one. It refuses a green page out of form, a green page this index does not list, and a green page that names a `--replay` folder it does not hold.

It also holds every green page to the standard of ADR 0016, which ADR 0018 leaves unchanged: at most 120 lines and 900 words, the first asserting block by line 20 with nothing set up before it, at most six of them and every `bash` block asserting, one command unless the title names the contrast, at most four steps, no design argument, and at most four closing links. A failure names the page, the rule, and the measured number. `sdlc/scripts/demos-self-test` proves each check against a page that breaks it.

The folder numbers never change. Demo 11 left with `segment` under ADR 0010, and demo 10 left with the configuration file under the same ADR. Demos 05 and 38 left under ADR 0016, into 02 and 25. Demos 20, 24, 30, 23, 37, and 42 left under ADR 0018, and the table at the end of `documentation-plan.md` says where each idea went. `specification/roadmap.md` says what each retired number held.

## What a demo never asks

Measurement of the first decider model fixes the shape of every question on these pages. A narrow yes/no question about a fact visible in the evidence works. A pick from a short list of options that exclude one another is steady, and adding an irrelevant option or changing the order moves the odds. Rating on a rubric is weak. Judging quality, completeness, or correctness fails badly. High confidence can be wrong when the needed evidence was never shown, and answers inside the unresolved band flip between runs.

So no demo asks whether something is good. Every demo asks about a visible fact, and every demo has a branch for unresolved.

## Start here

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 01 | [Gate a script step on a yes/no answer](01-refund-gate/) | `decide` | green |
| 19 | [Gate a risky command and fail closed](19-no-or-could-not-ask/) | `decide` | green |
| 27 | [Test a script with no network](27-test-with-no-network/) | `decide` | green |
| 18 | Point the tool at another server and compare two deciders | `decide` | coming, slice 13 |

## Gates and branches

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 02 | [Branch on a label with `choose` and `case`](02-route-a-ticket/) | `choose` | green |
| 40 | [Say what yes and no mean](40-what-yes-and-no-mean/) | `decide` | green |
| 17 | [Route a request by how hard it is](17-rate-and-sort/) | `score` | green |
| 21 | [Choose the next action from a list that changes at every step](21-options-from-the-record/) | `choose` | green |
| 41 | [Tune a question file and use the same file in the gate](41-tune-a-question-file/) | `decide` | green |

## Many records

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 03 | [Keep only the records that match a meaning](03-grep-for-meaning/) | `filter` | coming, ticket 0014 |
| 43 | Lint a change by meaning and fail the build | `filter` | coming, ticket 0014 |
| 06 | [Put the best matches first](06-top-search-hits/) | `rank` | coming, ticket 0014 |
| 12 | [Resume a long run that stopped](12-keep-going/) | `decide` | green |
| 15 | [Find the line that answers a question](15-find-the-line/) | `find` | coming, slice 11 |

## Many questions at once

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 39 | Screen one message for several hazards | `annotate` | coming, ticket 0015 |
| 16 | Build a triage pipeline that drafts, blocks, or asks a person | `annotate` | coming, slice 9 |

## Evals

An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence, and ordinary code does the policies, the metrics, and the comparisons. `documentation-plan.md` maps Ian's six capabilities to the how-tos that teach each one.

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 14 | [Grade an assistant's answers with a rubric](14-grade-a-batch/) | `annotate` | coming, ticket 0015 |
| 13 | [Pick a threshold from labeled cases](13-pick-a-threshold/) | `decide` | green |
| 25 | [Check the judge against human labels](25-check-the-judge/) | `decide` | green |
| 28 | [Know what a run cost](28-what-a-run-cost/) | `decide` | green |

Demos 13, 25, 28, and 41 read the `jq` transforms in `transforms/` over committed rows, which is what `report` would have done inside the tool.

## Folders that stay and then leave

Each folder below holds a page that is not in the list of 20. It stays until the page that absorbs it is green, so no lesson is ever missing, and its first lines say which page that is. `sdlc/scripts/pages` refuses a folder here whose page does not say so.

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 04 | [Act only when the answer is sure, and send the rest to a person](04-review-queue/) | `decide` | leaving, into 19, 16, and 13 |
| 07 | [Judged columns](07-judged-columns/) | `annotate` | leaving, into 14 and 16 |
| 08 | [Release checklist](08-release-checklist/) | `annotate` | leaving, into 39 |
| 09 | [What leaves the machine](09-what-leaves-the-machine/) | `filter` | leaving, into 03 |
