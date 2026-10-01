# How-tos

Each page here is one real shell job. It gives the input files, the commands, their real output, and what can go wrong. Every green page replays a committed recording, so you can run it with no key and no network. A number is a folder under `demos/`. Start with the first table.

## Start with these

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 16 | [Build a triage pipeline that drafts, blocks, or asks a person](16-triage-pipeline/) | `annotate` | green |
| 01 | [Gate a script step on a yes/no answer](01-refund-gate/) | `decide` | green |
| 02 | [Branch on a label with `choose` and `case`](02-route-a-ticket/) | `choose` | green |
| 15 | [Find the line that answers a question](15-find-the-line/) | `find` | green |
| 43 | [Lint a change by meaning and fail the build](43-lint-a-change/) | `filter` | green |
| 06 | [Put the best matches first](06-top-search-hits/) | `rank` | green |
| 14 | [Grade an assistant's answers with a rubric](14-grade-a-batch/) | `annotate` | green |

## Gates and branches

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 19 | [Gate a risky command and fail closed](19-no-or-could-not-ask/) | `decide` | green |
| 40 | [Say what yes and no mean](40-what-yes-and-no-mean/) | `decide` | green |
| 17 | [Route a request by how hard it is](17-rate-and-sort/) | `score` | green |
| 21 | [Choose the next action from a list that changes at every step](21-options-from-the-record/) | `choose` | green |
| 41 | [Tune a question file and use the same file in the gate](41-tune-a-question-file/) | `decide` | green |
| 47 | [Split a chained question into two calls](47-split-a-chained-question/) | `choose` | green |

## Many records

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 03 | [Keep only the records that match a meaning](03-grep-for-meaning/) | `filter` | green |
| 12 | [Resume a long run that stopped](12-keep-going/) | `decide` | green |
| 48 | [Make a first cut before a long list](48-first-cut-a-long-list/) | `find` | green |

## Many questions at once

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 39 | [Screen one message for several hazards](39-screen-a-message/) | `tag` | green |

## Names and relations

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 44 | [Find names in a text without a network](44-recognize-names/) | `recognize` | green |
| 45 | [Map relationships in a complete entity set](45-map-relationships/) | `relate` | green |

## Tests, recordings, and servers

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 27 | [Test a script with no network](27-test-with-no-network/) | `decide` | green |
| 46 | [Find unused recording entries](46-find-unused-recording-entries/) | `cache unused` | green |
| 18 | Point the tool at another server and compare two deciders | `decide` | coming, slice 13 |

## Evals

An eval is a reproducible workflow over the same commands as everything else. The tool obtains the answers and keeps the evidence. Ordinary code does the policies, the metrics, and the comparisons. How-to 14, listed above, grades answers with a rubric. The three below complete the eval set.

| # | How to | Verbs | Status |
| --- | --- | --- | --- |
| 13 | [Pick a threshold from labeled cases](13-pick-a-threshold/) | `decide` | green |
| 25 | [Check the judge against human labels](25-check-the-judge/) | `decide` | green |
| 28 | [Know what a run cost](28-what-a-run-cost/) | `decide` | green |

Demos 13, 25, 28, and 41 read the `jq` transforms in `transforms/` over committed rows, which is what `report` would have done inside the tool. `documentation-plan.md` maps Ian's six capabilities to the how-tos that teach each one.

## What a demo never asks

Measurement of the first System One model fixes the shape of every question on these pages. A narrow yes/no question about a fact visible in the evidence works. A pick from a short list of options that exclude one another is steady, and adding an irrelevant option or changing the order moves the odds. Scoring on a rubric is weak. Judging quality, completeness, or correctness fails badly. High confidence can be wrong when the needed evidence was never shown, and answers inside the not sure band flip between runs.

So no demo asks whether something is good. Every demo asks about a visible fact, and every demo has a branch for not sure.

## Maintaining these pages

A how-to is one of the four names in the table in [`CONTRIBUTING.md`](../CONTRIBUTING.md#the-four-names). A green page is the how-to, the demo, and the test at once. `sdlc/scripts/spec` runs every block that asserts something against a committed recording. ADR 0011 rules that nobody writes a second copy.

ADR 0018 fixed the base list at 20 pages, and later tickets added focused pages. This page holds the one list of how-tos. `sdlc/planning/documentation-plan.md` keeps the form, the capability map, the numbers that left and the rules, and points here. `sdlc/scripts/pages` checks that every relative link resolves.

### How to read one

Each page opens with a status line and the verbs it uses. A green page then takes the how-to form: a title that starts with "How to", one paragraph on when to use it, a first result inside the first twenty lines, the input files, the steps with their commands and real outputs, a section named "What can go wrong" with the exit codes and the traps, and a closing list of related how-tos.

`mustmatch test demos/` runs a block when the block pipes into `mustmatch` and skips a block that asserts nothing, so a shown command is a test only when it ends in an assertion. The blocks assert on exit codes, bare values, field names, row counts, and the records that came back. No block asserts on a probability that is illustrative, and a page that pins a number says which recording measured it.

Every `thinkthen` command that would otherwise reach a backend carries `--replay` and a recording folder, so a gate touches no network and reads no key. `specification/recording.md` defines the flag.

### Red and green

A demo starts **red**, and this list marks it **coming** with the ticket or the slice that writes it. All ten functions are built. Pages that still need a command or recordings remain plans. A red page argues for a design choice, and `FINDINGS.md` gathers those arguments across every page.

A demo turns **green** when the `spec` rung runs it against a recording and it passes. When it turns green it takes the how-to form and the argument leaves the page. `sdlc/scripts/demos` runs every page whose status line reads exactly `Status: green` and skips every red one. It refuses a green page that asserts nothing, a `bash` block that asserts nothing, `like ""`, `set +e`, and a `--replay` folder the page does not hold. `sdlc/scripts/demos-self-test` proves each refusal against a page that breaks it.

Green pages follow the standard of ADR 0016 as writing guidance: at most 120 lines and 900 words, the first asserting block by line 20, at most six of them, one command unless the title names the contrast, at most four steps, no design argument, and at most four closing links. Ticket 0312 removed the script that measured these limits.

### Retired numbers

The folder numbers never change. Demo 11 left with `segment` under ADR 0010, and demo 10 left with the configuration file under the same ADR. Demos 05 and 38 left under ADR 0016, into 02 and 25. Demos 20, 24, 30, 23, 37, and 42 left under ADR 0018. Demos 04 and 07 left when page 16 turned green. The table at the end of `documentation-plan.md` says where each idea went. `specification/roadmap.md` says what each retired number held.
