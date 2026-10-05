# MLX score criteria need text rendering

Status: open.

Kind: debt
Debt: 039
Severity: medium
Pay when: The Strands System One server accepts authored rich score criteria

## Problem

MLX score criteria need text rendering

Found at experiment 0030 and ticket 0421 while working a landed ticket.
Milestone: later

Experiment 0030's 2026-10-04 diagnosis found that the pinned Strands schema accepts score criteria as a list of strings and rejects rich objects with HTTP 422. Ticket 0421 uses the existing text-description rendering; null or empty descriptions become their level names. This loses authored description structure while retaining text. Keep this workaround separate from hosted and llama.cpp request encoding.

Primary upstream source: https://github.com/strands-labs/strands-decider/blob/75c9fd32e664954cdc18481434018aa507eee8fb/src/strands_decider/schema.py . Its score levels are bounded 2–10. Generic mlx_lm chat serving is not this System One route. No upstream issue number is claimed. Remove the workaround only after a reviewed supported server accepts the authored request and compatibility checks pass. The final real local-server check remains pending; this issue records the observed schema limit, not model quality.
