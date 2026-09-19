# How-tos

Every page here is one shell job written so that it runs. A green page is the how-to, the demo, and the test at once, and `sdlc/scripts/spec` runs every block that asserts something against a committed recording. ADR 0011 rules that nobody writes a second copy.

A how-to is one of the four names in the table in [`../README.md`](../README.md).

`sdlc/planning/documentation-plan.md` holds the full list, the groups below, and the demo number reserved for each task. A number here is a folder under `demos/`.

## How to read one

Each page opens with a status line and the verbs it uses. A green page then takes the how-to form: a title that starts with "How to", one paragraph on when to use it, the input files, the steps with their commands and real outputs, a section named "What can go wrong" with the exit codes and the traps, and a closing list of related how-tos.

`mustmatch test demos/` runs a block when the block pipes into `mustmatch` and skips a block that asserts nothing, so a shown command is a test only when it ends in an assertion. The blocks assert on exit codes, bare values, field names, row counts, and the records that came back. No block asserts on a probability, and every page says once that its numbers are illustrative until a recording exists. A page that pinned a probability would be testing the vendor instead of the tool.

Every `thinkthen` command that would otherwise reach a backend carries `--replay` and a recording folder, so a gate touches no network and reads no key. `specification/recording.md` defines the flag.

## Red and green

A demo starts **red**. Only `decide`, `choose`, and `score` are built, so every page that needs another verb is still a plan. A red page argues for a design choice, and `FINDINGS.md` gathers those arguments across every page.

A demo turns **green** when the `spec` rung runs it against a recording and it passes. When it turns green it takes the how-to form and the argument leaves the page. `sdlc/scripts/demos` runs every page whose status line reads exactly `Status: green` and skips every red one. It refuses a green page whose title does not start with "How to", that has no "What can go wrong" section, or that this index does not list. A green page that names a `--replay` folder it does not hold stops the run.

The folder numbers never change. Demo 11 left with `segment` under ADR 0010, and demo 10 left with the configuration file under the same ADR. Both stay empty numbers, and `specification/roadmap.md` says what each one held.

## What a demo never asks

Measurement of the first decider model fixes the shape of every question on these pages. A narrow yes/no question about a fact visible in the evidence works. A pick from a short list of options that exclude one another is steady, and adding an irrelevant option or changing the order moves the odds. Rating on a rubric is weak. Judging quality, completeness, or correctness fails badly. High confidence can be wrong when the needed evidence was never shown, and answers inside the unresolved band flip between runs.

So no demo asks whether something is good. Every demo asks about a visible fact, and every demo has a branch for unresolved.

## Start here

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 01 | [Gate a script step on a yes/no answer](01-refund-gate/) | `decide` | green |
| 19 | [Tell "no" from "could not ask" in a script](19-no-or-could-not-ask/) | `decide` | green |
| 27 | [Test a script with no network](27-test-with-no-network/) | `decide` | green |

## Gates and branches

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 04 | [Act only when the answer is sure, and send the rest to a person](04-review-queue/) | `decide` | green |
| 02 | [Branch on a label with `choose` and `case`](02-route-a-ticket/) | `choose` | green |
| 20 | [Tell "not stated" from "false"](20-not-stated-or-false/) | `choose`, `decide` | green |
| 17 | [Rate on a scale, sort by it, and test it with `jq -e`](17-rate-and-sort/) | `score` | green |
| 05 | [Sort files into folders by label](05-sort-a-folder/) | `choose` | green |

## Many records

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 03 | [Keep only the records that match a meaning](03-grep-for-meaning/) | `filter` | red |
| 06 | [Put the best matches first](06-top-search-hits/) | `rank`, `filter` | red |
| 09 | [Control what leaves the machine](09-what-leaves-the-machine/) | `filter`, `--dry-run` | red |
| 12 | [Resume a long run that stopped](12-keep-going/) | `decide` | green |
| 21 | [Choose from a list that differs for every record](21-options-from-the-record/) | `choose` | green |
| 15 | [Find the one line that answers a question](15-find-the-line/) | `find` | red |

## Many questions at once

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 07 | [Add several judged columns in one pass](07-judged-columns/) | `annotate`, `filter` | red |
| 08 | [Check a document against a checklist](08-release-checklist/) | `annotate` | red |

## Evals

An eval is a reproducible workflow over the same commands as everything else. The tool obtains the judgments and keeps the evidence, and ordinary code does the policies, the metrics, and the comparisons. `documentation-plan.md` maps Ian's six capabilities to the how-tos that teach each one.

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 13 | [Pick a threshold from labeled cases](13-pick-a-threshold/) | `decide` | green |
| 24 | [Compare two runs of the same cases](24-compare-two-runs/) | `decide` | green |
| 25 | [Check the judge against human labels](25-check-the-judge/) | `decide` | green |
| 28 | [Know what a run cost](28-what-a-run-cost/) | `decide` | green |
| 38 | [See whether a probability means what it says](38-what-a-probability-means/) | `decide` | green |
| 14 | [Grade a batch with a reusable definition](14-grade-a-batch/) | `annotate`, `report` | red |

Demos 13, 24, 25, 28, and 38 read the `jq` transforms in `transforms/` over the committed rows of one live run, which is what `report` would have done inside the tool. Demo 14 still names `report`, which left the plan under ADR 0010, and it is rewritten when `annotate` lands.
