# 0409: Match TypeScript rank and find declarations to runtime

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

The TypeScript compiler accepts only question shapes supported by rank/find runtime, or runtime supports an independently reviewed additive shape. Retain literal text calls. Reconcile with 0406 before choosing richer rank shapes.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 audit](../records/0405-audit-report.md). index.d.ts accepts DecideSpec for rank/find while index.js textFrom refuses every key beyond decide; verbs.test.mjs pins threshold rejection. JavaScript runtime is separate from the TypeScript contract.
- Keeps: existing request bytes, digests, cache identity, result shapes, failures and host adapters whenever the new capability is unused.
- Changes: match typescript rank and find declarations to runtime. Product work starts only after the contract and ticket receive fresh review.
- Proof: Compile valid/invalid consumer examples and run matching loopback calls; reject unsupported settings before sends. Test threshold, criteria, model/profile and literal @/{ text.
- Defers: Small declaration fix qualifies for 0.2. Richer find contract is outside scope.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
