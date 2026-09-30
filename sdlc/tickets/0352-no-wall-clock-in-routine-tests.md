# 0352: No wall-clock timing in routine tests

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-1. The ticket branch holds the full ticket.

## Outcome

No routine rung (`test`, `spec`, the surface checks) depends on how fast the machine runs.

## Evidence

- Starts from: ticket 0340 and the closed load-flake issues.
- Keeps: every cancellation, deadline and pacing regression.
- Changes: written on the ticket branch after the inventory.
- Proof: `sdlc/scripts/test` under a deliberate load of 24 busy processes, before and after.
- Defers: the C door tests, which ticket 0346 owns.
