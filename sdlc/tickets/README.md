# tickets/

One file per ticket; take the next free number. A ticket authorizes work and lands on `ticket/NNNN-slug` through a lane per `sdlc/planning/worktrees.md`. A ticket that changes a setting updates `specification/settings.md` in the same commit.

- `Status:` as pm sets it.
- `Milestone:` `0.1`, `0.2`, `0.3`, or `later`, on its own line near `Status:`, while the ticket is open. [milestones.md](../planning/milestones.md) lists each milestone's items.
- `## Outcome`: what is true when it lands.
- A Proof: what shows the outcome holds.
- Other notes only when the builder needs them.

A landing record is optional. Prefer pm commands for ticket edits; any edit tool is fine for ticket text.
