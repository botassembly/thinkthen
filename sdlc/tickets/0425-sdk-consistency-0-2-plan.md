# 0425: Put the complete SDK outcome into the 0.2 plan

Status: landed. The complete reviewed 0.2 plan assigns every PM ask, dependencies and acceptance checks. Product implementation remains open in its owning tickets.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Every PM ask has a ticket owner, dependencies, retained behavior and an acceptance check. The current plan replaces superseded scope and lane orders. This planning ticket changes no product behavior.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 1–12.
- Keeps: Landed product behavior and existing qualified-run history. Specific publication approval remains required.
- Changes: Create 0426–0445; promote and amend 0296, 0300, 0406–0414, 0417 and 0418; retain 0393 as the npm owner. Update the milestone, team note and run-facts issue. Answer all twelve mail asks with ticket numbers after fresh ticket reviews.
- Proof: Review each new or materially amended ticket with a fresh read-only reviewer. Check the twelve-ask map, nonduplicated ownership, dependencies, milestone lines and links. Run the ticket/documentation checks; no runtime or release experiment is needed for this planning-only change.
- Defers: Implementation, migrations, paid calls, registry/settings changes and further release runs. A later final-commit qualification remains an acceptance criterion, not current work.
