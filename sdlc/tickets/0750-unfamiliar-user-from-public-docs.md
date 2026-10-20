# 0750: An unfamiliar user succeeds from the public documents alone

Status: OPEN.

Milestone: 0.2.2

Phase: 1

## Outcome

A reader who has never seen ThinkThen, with only the published README and site, installs the command and one language library and gets an answer to a yes/no question. The question runs against a recorded or local backend, so the trial spends no model money.

- The trial is written down: what the reader read, each command they ran, and every place they got stuck or guessed.
- Each stuck point is fixed in the public documents, or becomes its own ticket when the fix is not a docs change.
- The reader is a fresh agent or person with no access to this repository's `sdlc/`, source or history.

### Proof

- The written trial, in this ticket's record.
- The documentation commits that fix each stuck point.
- A second fresh reader follows the fixed documents and does not get stuck at the same places.

## Evidence

The workspace ideal state names "nobody outside has used it" as its first gap, and no plan addressed it before 0.2.2. The [new-user stumble register](../issues/2026-09-20-new-user-stumble-register.md) holds earlier stumbles from the 0.1 docs.

## Progress
