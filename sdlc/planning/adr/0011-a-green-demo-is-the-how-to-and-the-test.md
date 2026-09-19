# ADR 0011: A green demo is the how-to and the test

- Status: Accepted by Ian on 2026-09-19
- Date: 2026-09-19

## Context

ADR 0005 made executable demos drive the design. Ian then asked for a full list of how-to documents, supported by the demos, with the test, the demo, and the how-to being one thing. Documentation that is written apart from the tests drifts from them. A page that the gate runs cannot drift.

## Decision

1. **One file is the demo, the how-to, and the test.** Each folder under `demos/` holds one page. The gate runs every block on it against a committed recording. The same page is the published how-to. Nobody writes a second copy.
2. **A red demo argues, and a green demo teaches.** While a demo is red it may argue for a design choice, and its open points go to `demos/FINDINGS.md`. When it turns green it takes the how-to form below, and the argument leaves the page.
3. **The how-to form.** The title starts with "How to" and names a task a user has. The page opens with one paragraph on when to use it. Then come the input files, the steps with their commands and real outputs, a section named "What can go wrong" with the exit codes and the traps, and a closing list of related how-tos. Plain words. No design history.
4. **Three kinds of page, and no others.** How-tos live in `demos/`. Reference lives in `specification/`, and `spec/` holds its executable examples. The README is the one tutorial and the one explanation: what the tool is, the first command, and a link to the how-to list.
5. **The list of how-tos is planned ahead.** `sdlc/planning/documentation-plan.md` holds the full list, each with its demo, its slice, and its state. A slice is done when its how-tos are green. A feature with no how-to on the list has no place in version one.
6. **A check enforces the form.** The gate refuses a green demo whose title does not start with "How to", that lacks the "What can go wrong" section, or that is missing from `demos/README.md`. Ticket 0010 writes the check.

## Consequences

Demo 01 is green and takes the form first. Every ticket that turns a demo green writes it in the form. `score` has no demo today, so the list gains one. The published documentation, when there is a site, is built from `demos/`, `specification/`, and the README with no rewriting.
