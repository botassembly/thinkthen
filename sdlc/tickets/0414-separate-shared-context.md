# 0414: Define separate context on aggregate functions

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: 0.2

## Outcome

Define and review separate shared context for annotate, find, recognize and relate, then expose approved functions through core/CLI/SQL without changing their items. Keep existing refusals until the additive contract lands.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). annotate_with and annotate_each_with, find_with, recognize_with and relate_with call without_context. Rich record evidence is supported but does not provide a separate context channel. PM layer model expects independent context.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: define separate context on aggregate functions. Product work starts only after the contract and ticket receive fresh review.
- Proof: Pin aggregate item identity, offsets/candidates/entities, group pointers and stage states; count context-specific misses and strict replay; unchanged calls retain exact bytes and digests.
- Defers: This is a design gap, not a false claim that existing contracts permit context. Different relation evidence semantics require review; foreign adapter releases later.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
