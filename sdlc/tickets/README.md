# tickets/

One file per ticket, numbered from 0001 in this repository. Take the next free number. A ticket authorizes work. It lands through a lane per `sdlc/planning/worktrees.md` on branch `ticket/NNNN-slug`. A ticket that adds or changes a setting updates its row in `specification/settings.md` in the same commit.

A ticket is short:

- `Status:` one word or phrase: `ready`, `in progress`, or `landed`. A candidate between build and landing may say `built, awaiting code review`. The status line holds no running history; put results in the build section or the record.
- `## Outcome`: what is true when it lands.
- `## Evidence`, five bullets labelled exactly `- Starts from:`, `- Keeps:`, `- Changes:`, `- Proof:`, `- Defers:`, each with text. They name the prior evidence, retained behavior, changes, proof, and deferred gaps. `sdlc/scripts/tickets` checks them from ticket 0120 on.
- Design notes only when the builder needs them.
- `## What the build taught us`, added before landing.
