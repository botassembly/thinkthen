---
flow: build
priority: 50
opens: demos/01-refund-gate demos/19-no-or-could-not-ask demos/27-test-with-no-network demos/README.md sdlc/scripts crates/thinkthen/tests/demo_runner.rs sdlc/live-tokens sdlc/planning/documentation-plan.md AGENTS.md README.md
---

# 0010: The how-to form and its check

Status: done

## Outcome

A green demo is a how-to, and the gate enforces it, as ADR 0011 rules. Demo 01 takes the form first. Two new how-tos that need only `decide` are green. The README points a new reader at the list.

## Current Facts

Demo 01 is green and still reads as a design argument, with a closing section named "What this demo decides". ADR 0011 gives the form: a title that starts with "How to", one paragraph on when to use it, the input files, the steps with commands and real outputs, "What can go wrong", and related how-tos. `sdlc/planning/documentation-plan.md` reserves demo 19 (tell "no" from "could not ask") and demo 27 (test a script with no network).

## Scope

- Demo 01 is rewritten in the form. Its design argument moves to `demos/FINDINGS.md` or leaves. Its recording does not change, and every block still runs.
- Demo 19 teaches the `case $?` block, the behaviour under `set -e`, a backend failure that must never read as "no", and the advice to word a question so that yes permits the action. A failure is shown with a replay folder that lacks the entry, so no network is needed.
- Demo 27 teaches `--record`, then `--replay` with the key unset, and what a replay miss looks like. Its recording is made through `sdlc/scripts/live`.
- The check: the demos script refuses a green demo whose title does not start with "How to", that lacks a "What can go wrong" section, or that `demos/README.md` does not list. A red demo is never checked for form. The check has tests that need no network.
- `demos/README.md` becomes the how-to index in the order of `documentation-plan.md`. The root README gains the link and one sentence on the three kinds of page. `AGENTS.md` gains one line: a ticket that turns a demo green writes it in the form.

Excluded: any change to the binary's behaviour.

## Acceptance

- The spec rung prints three green demos with the key unset and touches no network.
- A test proves that the check refuses each of the three faults and passes a page in form.
- The recordings hold no key, and the live tokens spent are in `sdlc/live-tokens` and in the plan's table.
- The whole ladder is green.
