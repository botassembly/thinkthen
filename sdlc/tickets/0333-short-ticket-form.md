# 0333: Short ticket form and a smaller ticket checker

Status: in progress

## Outcome

Tickets use the short form in `sdlc/tickets/README.md`. `sdlc/scripts/tickets` checks only ticket evidence: the five Evidence items from 0120, and from 0333 an Outcome section and, once landed, `## What the build taught us`. Ruling 8.

## Evidence

- Starts from: main `f57650111`. Ticket 0312 cut `ticket-preparation.md` to 706 words. The checker ran 174 lines with 13 self-test cases, most on bold, italic, tilde, and mixed-fence spellings.
- Keeps: the five Evidence items from 0120, bold labels, old tickets unchanged, and the Boundaries and Proof rules in `AGENTS.md`.
- Changes: the checker drops fence parsing and heading-level parsing and adds the Outcome and lessons checks from 0333. The ticket README, `ticket-preparation.md`, `sdlc/README.md`, and `AGENTS.md` stop restating ticket rules.
- Proof: `tickets --self-test` and `tickets` pass on every ticket; `policy.py` and `lint` pass.
- Defers: checks that evidence paths resolve and that a landed ticket names its landing commit. Old tickets cite moved files, and a ticket cannot name the merge that lands it.
