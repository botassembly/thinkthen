# 0736: Preserve charged usage in the pricing examples

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision 772d146, accept

## Outcome

- Changes: `transforms/rows/runs/`, `transforms/README.md`, `transforms/rows/record.sh`, `demos/28-what-a-run-cost/README.md`.
- Current native result packets in the saved rows runs retain the charged usage that the existing pricing lesson requires. The unchanged executable pricing examples pass without zero-cost replay rows or division by zero.
- Reproduce the failure on main before repair. Rebuild affected fixture packets through existing retained recordings or labeled synthetic loopback fixtures, preserving truthful provenance and the current native result shape.
- Preserve every distinct pricing assertion, probability, decision and lesson. Make no paid calls, invent no historical provenance, and add no retained generator, helper or runner. Prose changes serve only truthful fixture provenance. Expand an exact claim only when diagnosis proves the same defect class needs it.

### Proof

- Through pm job on the M5, the existing executable pricing page fails on unchanged main, then passes on the repair. Run existing dependent rows/transform checks for the affected data. Preserve the original failure receipt and all pricing expectations.

## Evidence

M5 job 0724/1792505486688-96923, based on main 969e17b, passed the focused Rust checks but failed four demos/28 pricing cases. It reports zero USD and division by zero after main a06b0ebe5 rebuilt the saved rows as cached synthetic replay packets. This ticket verifies the cause and repairs the fixture data without weakening the lesson.

## Progress

- 2026-10-20 started
