# demos/

A demo is a small, real shell job written as an executable page. It lives at `demos/NN-name/README.md` with its input files beside it. It shows work a stranger would want done, written the way a careful shell user writes: `set -euo pipefail`, pipes, `jq`, exit codes read with `case`, files published with `mv`, and no command run because a model said so.

ADR 0005 puts the demos in front of the code. Writing a demo is the first test of the design. Where a command line reads badly, needs a flag that is absent, or fights a Unix habit, the specification changes and the demo says so.

## How to read one

Each page opens with a status line, the verbs it uses, and the story of the job. Then come the input files, then the walk-through in `bash` blocks. `mustmatch test demos/` runs a block when the block pipes into `mustmatch`, so the shown commands are the tests.

The blocks assert on exit codes, `assessment.status`, `assessment.value`, picks, row counts, column names, and field names. No block asserts on a probability. A probability is a measurement of one model on one day, and a page that pinned one would be testing the vendor instead of the tool.

Every page closes with "What this demo decides". That section names the open choices the demo bears on, what the demo argues for, and anything the specification lacks or gets wrong. `FINDINGS.md` gathers those across all twelve pages.

## Red and green

A demo starts **red**. Nothing under `demos/` runs today, because only `decide if` is specified as settled and no verb is built. A red demo is a plan.

A demo turns **green** when the `spec` rung runs it against a recording and it passes. A green demo is a regression test and a page of documentation at once. The `spec` rung runs every green demo and skips every red one.

Each page carries `--replay recording/` on every `thinkthen` command, so a gate touches no network. No recording exists yet, and `--replay` is not in the specification yet either. That is the first finding in `FINDINGS.md`.

## What a demo never asks

Measurement of the first decider model fixes the shape of every question on these pages. A narrow yes/no question about a fact visible in the evidence works. A pick from a short list of options that exclude one another is steady, and adding an irrelevant option or changing the order moves the odds. Rating on a rubric is weak. Judging quality, completeness, or correctness fails badly. High confidence can be wrong when the needed evidence was never shown, and answers in the unsure band flip between runs.

So no demo asks whether something is good. Every demo asks about a visible fact, and every demo has a branch for unsure.

## Index

| # | Name | Verbs | Status |
| --- | --- | --- | --- |
| 01 | [Refund gate](01-refund-gate/) | `decide if` | red |
| 02 | [Route a ticket](02-route-a-ticket/) | `decide which` | red |
| 03 | [Grep for meaning](03-grep-for-meaning/) | `decide where` | red |
| 04 | [Review queue](04-review-queue/) | `decide where` | red |
| 05 | [Sort a folder](05-sort-a-folder/) | `decide which` | red |
| 06 | [Top search hits](06-top-search-hits/) | `decide where`, `decide rank` | red |
| 07 | [Judged columns](07-judged-columns/) | `decide where` | red |
| 08 | [Release checklist](08-release-checklist/) | `decide run` | red |
| 09 | [What leaves the machine](09-what-leaves-the-machine/) | `decide where`, `--plan` | red |
| 10 | [Another backend](10-another-backend/) | `decide if` | red |
| 11 | [Split a thread](11-split-a-thread/) | `decide segment` | red |
| 12 | [Keep going](12-keep-going/) | `decide if`, `decide where` | red |
