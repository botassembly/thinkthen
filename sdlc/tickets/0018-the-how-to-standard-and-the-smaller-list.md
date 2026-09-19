---
flow: build
priority: 45
opens: demos README.md sdlc/scripts/demos sdlc/planning/documentation-plan.md sdlc/planning/plan.md sdlc/live-tokens transforms
---

# 0018: The how-to standard and the smaller list

Status: ready

## Outcome

Every green how-to meets the standard of ADR 0016, `sdlc/scripts/demos` enforces the parts a script can check, the green pages that overlap are merged, and the README shows the front window. No behavior of the binary changes.

## Current Facts

ADR 0016 holds the standard, the front window, and the list of 27. Thirteen or more pages are green and the rest are red. The three weakest green pages are 05 (162 lines, three near-identical loops), 04 (177 lines, four ideas), and 02 (149 lines, two blocks that test the fixture). Red pages hold design arguments by their nature, and the standard's rule 7 binds green pages alone.

## Scope

- `sdlc/scripts/demos` checks every green page for rules 1, 2, 3, 4, 5, 7, and 8 of ADR 0016: the line and word limits, where the first block starts, the count of runnable blocks, the count of distinct commands against the title, the count of step headings, the banned heading, and the count of closing links. A failed check names the page, the rule, and the measured number. A red page is checked for nothing new.
- Every green page is brought under the limits. Trim before anything else: a block that asserts nothing a reader learns goes, a step another page owns becomes a link, and set-up before the first result goes.
- The merges among pages that are green today: 05 into 02 as its closing section, 38 into 25, and the band and the audit block of 01 into 19. 19 takes the scenario of a proposed shell command and the title "How to gate a risky command and fail closed". 04 keeps three steps. 17 takes the scenario of routing a request by how hard it is. 21 takes the scenario of picking the next action from a list that changes at every step, because ticket 0013 turned it green with a returns desk before ADR 0016 landed, and the action list is the use case people ask about. A page that leaves is deleted with its folder, and a recording that no block uses any more is deleted with it.
- A page whose scenario changes is recorded again through `sdlc/scripts/live`, by a `record.sh` beside it. The live spend stays under 30,000 input tokens.
- The merges among red pages are made in the plan and the index alone: the red folders 08 and 09 stay until their absorbing pages turn green, and each gains one line at the top that names the page that will absorb it.
- `documentation-plan.md` and `demos/README.md` are rewritten to the list of ADR 0016, with every state current. `README.md` shows the front window: seven lines, each a title, a one-line scenario, and a link, with the pages that are still red marked as coming.
- `plan.md` follows: each slice names the how-tos it now turns green.

Excluded: any change to the binary, any new command or option, and the pages that tickets 0014, 0015, and 0017 turn green.

## Acceptance

- The spec rung prints every green page green under the new checks, with the key unset and no network.
- A test page under `sdlc/scripts/` fixtures, or an equivalent self-test, proves that each new check fails a page that breaks its rule.
- `demos/README.md`, `documentation-plan.md`, and the folders under `demos/` agree on every number, title, and state.
- The record lists each page with its lines and words before and after, each recording deleted, and each scenario changed.
- The recordings hold no key, and the three searches return zero.
- The whole ladder is green, and the ratchet is unchanged.
