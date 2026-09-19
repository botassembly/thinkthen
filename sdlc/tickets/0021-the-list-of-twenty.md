---
flow: build
priority: 47
opens: demos README.md sdlc/planning/documentation-plan.md sdlc/planning/plan.md
---

# 0021: The list of twenty

Status: landed

## Outcome

`documentation-plan.md`, `demos/README.md`, and the README's front window show the 20 how-tos and the seven featured pages of ADR 0018. Each green page whose absorbing page is green by then leaves with its folder. No behavior of the binary changes.

## Current Facts

Ticket 0018 rewrote the three pages to ADR 0016's list of 27 and added the checks of the how-to standard to `sdlc/scripts/demos`. ADR 0018 then cut the list to 20 and changed the front window to 01, 02, 15, 43, 06, 14, 16. Ticket 0017 writes pages 40 and 41 so that the green pages 20 and 24 can leave. Pages 43, 39, 14, and 16 are red until tickets 0014 and 0015 and the flagship slice land.

## Scope

- The three pages are rewritten to ADR 0018: every number, title, group, and state, with each red page marked as coming and naming its ticket.
- Green pages 20 and 24 are deleted with their folders once 40 and 41 are green, and only after a reading confirms that nothing they teach is lost. A recording that no block uses any more is deleted with its page. Page 04 stays until page 16 is green, and its row says so.
- The red folders of pages that left the list are deleted: 07 and 08 stay for ticket 0015, 09 stays for ticket 0014, and any other red folder outside the 20 goes now.
- `plan.md` follows: each slice names the how-tos it turns green.

Excluded: any change to the binary, any new how-to, and any live call.

## Acceptance

- `demos/README.md`, `documentation-plan.md`, and the folders under `demos/` agree on every number, title, and state.
- The README's front window shows the seven pages of ADR 0018 in order, with each red page marked as coming.
- The spec rung prints every green page green under the checks of the how-to standard, with the key unset and no network.
- The record lists each page deleted and where its lesson now lives.
- The whole ladder is green, and the ratchet is unchanged.
