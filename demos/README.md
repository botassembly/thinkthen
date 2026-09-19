# demos/

A demo is a small, real shell job written as an executable page. It lives at `demos/NN-name/README.md` with its input files beside it. It shows work a stranger would want done, written the way a careful shell user writes: `set -euo pipefail`, pipes, `jq`, exit codes read with `case`, files published with `mv`, and no command run because a model said so.

ADR 0005 puts the demos in front of the code. Writing a demo is the first test of the design. Where a command line reads badly, needs an option that is absent, or fights a Unix habit, the demo says so and the specification changes.

Every page on this branch is written to ADR 0007 and its clarifications: flat verbs, a bare JSON value on standard output, and one `--threshold` option. Demos 13, 14, and 15 also drive ADR 0008 and ADR 0009, which are Proposed. No page reaches for a command, an option, a field, or an exit code that one of the three does not name. Where a job could not be written cleanly inside that surface, the page writes it as well as the surface allows and records the gap.

## How to read one

Each page opens with a status line, the verbs it uses, and the story of the job. Then come the input files, then the walk-through in `bash` blocks. `mustmatch test demos/` runs a block when the block pipes into `mustmatch`, so the shown commands are the tests.

The blocks assert on exit codes, bare values, field names, row counts, and the records that came back. No block asserts on a probability, and every page says once that its numbers are illustrative until a recording exists. A page that pinned a probability would be testing the vendor instead of the tool.

Every page closes with "What this demo decides". That section says what the demo confirms, what it could not say cleanly, and what it argues should change. `FINDINGS.md` gathers those across all fifteen pages.

## Red and green

A demo starts **red**. Nothing under `demos/` runs today, because no verb in ADR 0007 is built and `find` is not yet designed. A red demo is a plan.

A demo turns **green** when the `spec` rung runs it against a recording and it passes. A green demo is a regression test and a page of documentation at once. `sdlc/scripts/demos` runs every page whose status line reads exactly `Status: green` and skips every red one. A green page that names a `--replay` folder it does not hold stops the run.

Each page carries `--replay recording/` on every `thinkthen` command that would otherwise reach a backend, so a gate touches no network. `specification/recording.md` defines the flag. A `--dry-run` block needs no recording, because it sends nothing. No recording exists yet, so every page is still red.

## What a demo never asks

Measurement of the first decider model fixes the shape of every question on these pages. A narrow yes/no question about a fact visible in the evidence works. A pick from a short list of options that exclude one another is steady, and adding an irrelevant option or changing the order moves the odds. Rating on a rubric is weak. Judging quality, completeness, or correctness fails badly. High confidence can be wrong when the needed evidence was never shown, and answers inside the unresolved band flip between runs.

So no demo asks whether something is good. Every demo asks about a visible fact, and every demo has a branch for unresolved.

## Index

| # | Name | Verbs | Status |
| --- | --- | --- | --- |
| 01 | [Refund gate](01-refund-gate/) | `decide` | red |
| 02 | [Route a ticket](02-route-a-ticket/) | `choose` | red |
| 03 | [Grep for meaning](03-grep-for-meaning/) | `filter` | red |
| 04 | [Review queue](04-review-queue/) | `decide` | red |
| 05 | [Sort a folder](05-sort-a-folder/) | `choose` | red |
| 06 | [Top search hits](06-top-search-hits/) | `rank`, `filter` | red |
| 07 | [Judged columns](07-judged-columns/) | `annotate`, `filter` | red |
| 08 | [Release checklist](08-release-checklist/) | `annotate` | red |
| 09 | [What leaves the machine](09-what-leaves-the-machine/) | `filter`, `--dry-run` | red |
| 10 | [Another backend](10-another-backend/) | `decide`, `config` | red |
| 11 | [Split a thread](11-split-a-thread/) | `segment` | red |
| 12 | [Keep going](12-keep-going/) | `decide` | red |
| 13 | [Pick a threshold](13-pick-a-threshold/) | `decide`, `report` | red |
| 14 | [Grade a batch](14-grade-a-batch/) | `annotate`, `report` | red |
| 15 | [Find the line](15-find-the-line/) | `find` | red |
