# ADR 0005: Demos drive the tests, and the tests drive the design

- Status: Accepted
- Date: 2026-09-19

Ian set the direction in dictation on 2026-09-19. The dictation was cut off, and an agent filled in the method. Ian can overturn the method cheaply.

## Context

The draft specification for the `decide` family came from chat captures. It raises eighteen choices, and nobody has used the tool to make any of them. Ian said the demo examples should drive the testing, and the testing should drive the decisions on the command-line design. He also said the design follows Unix principles.

The repository already runs executable Markdown pages under `spec/`. A page mixes prose with shell blocks, and a block fails when its output does not match.

## Decision

**A demo is a small, real shell job written as an executable page.** It lives at `demos/NN-name/README.md` beside its input files. It shows a job a stranger would want done, written the way a careful shell user would write it: pipes, `jq`, `set -euo pipefail`, exit codes, and files.

**Demos are written before the verbs they use.** A demo starts red. Writing it is the first test of the design. Where a demo reads badly, needs a flag that is absent, or fights a Unix habit, the specification changes. Each demo ends with a section named "What this demo decides". It lists the open choices it bears on and what the demo argues for.

**A choice in the draft specification settles when a demo needs it.** A choice no demo touches stays Draft, and a feature no demo uses is a candidate to cut.

**Demos replay recorded answers.** A demo is recorded once against a live backend under the testing budget. Gates replay the recording and touch no network. Recording and replay therefore move up the plan to the slice after `decide if`.

**The `spec` rung runs every demo marked green.** A red demo is a plan. A green demo is a regression test and a page of user documentation.

## Consequences

The order of work follows the demos. Replay is built second. The specification changes often while it is Draft, and each change names the demo that forced it.
