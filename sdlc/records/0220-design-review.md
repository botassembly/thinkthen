# 0220 design review

Fresh independent design review **ACCEPTED** source `52f0864b` after corrections to the ticket, preflight and ADR 0093. The reviewer verified the two retention layers, the private three-way flow, all direct scheduler adapters, and the held-output failure metadata. The bounded window may change speculative sends before failure or cancellation and pause-based cuts on timed input; exact request parity applies only to completed finite input with stable batch boundaries. The review required ADR 0093 before changing settled rank and records reference prose. The coordinator approved this routine design within the bounded-top outcome. This is design acceptance, not a runtime code review.

## What the build taught us

The design needed separate admission and failure-output semantics. A single `held` boolean cannot bound completed batches behind a stalled first request while preserving rank's empty-output stop message. The live source inventory also included three direct deadline-scheduler callers beyond the facade adapters. These findings set the implementation and proof boundary; the build record will report what the code and focused checks establish.
