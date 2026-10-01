# tickets/

One file per ticket; take the next free number. A ticket authorizes work and lands on `ticket/NNNN-slug` through a lane per `sdlc/planning/worktrees.md`. A ticket that changes a setting updates `specification/settings.md` in the same commit.

- `Status:` `ready`, `in progress`, or `landed`.
- `Milestone:` `0.1`, `0.2`, or `later`, on its own line near `Status:`, while the ticket is open. [milestones.md](../planning/milestones.md) lists each milestone's items.
- `## Outcome`: what is true when it lands.
- `## Evidence`: `- Starts from:`, `- Keeps:`, `- Changes:`, `- Proof:`, `- Defers:`. They name prior evidence, retained behavior, changes, proof, and deferred gaps.
- Design notes only when the builder needs them.
- `## What the build taught us`, added before landing.

`sdlc/scripts/tickets` checks these sections.
