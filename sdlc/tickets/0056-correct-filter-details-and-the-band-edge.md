---
flow: build
priority: 37
opens: crates/thinkthen specification conformance demos sdlc/planning sdlc/ratchet.json
---

# 0056: Keep details inside the selection and include the band edge

Status: landed

## Outcome

`filter --details` prints detailed results for kept records alone. Asking for details never changes which records print. Under a band, a probability below `LOW` is `no`, one from `LOW` up to but not including `HIGH` is `not sure`, and one at or above `HIGH` is `yes`. `rank --details --top N` remains limited to the first `N` ranked records.

## Current facts and decisions

The command currently builds a detailed result before it applies `filter`'s selection, so `--details` widens the output to every input record. The specification contradicts itself: `filter.md` describes that widening while `result.md` says details preserve membership. Ian ruled that filter must still filter.

The threshold specification says boundaries are inclusive, but the implementation and worked table send a probability equal to the low edge to `no`. The approved behavior sends that value to `not sure`. Ian can overturn these decisions.

## Scope

Apply selection before rendering either bare or detailed filter rows. Correct the band rule and its worked examples. Pin `filter --details`, `rank --details --top`, and both band boundaries in focused tests and the shared contract where its existing shape can express them. Move any example that needs all records to `decide --details`.

Excluded: help defaults, exit-code prose outside the touched pages, diagnostics, backend profiles, result-shape changes, engine settings, cache work, request bytes, and paid calls.

## Acceptance

- With the same recorded answers, bare `filter` and `filter --details` print the same kept inputs in the same input order. Detailed output contains the full result only for those inputs.
- `rank --details --top 2` prints exactly the same first two inputs and order as bare `rank --top 2`.
- A probability below a band's low edge is `no`; a probability equal to either edge is `not sure` at the low edge and `yes` at the high edge; a probability between the edges is `not sure`.
- The filter, result, and threshold specifications agree with the binary. Examples that need every answer use `decide --details`.
- Focused tests first fail for the two faults and then pass. The four repository rungs and `git diff --check` pass with the key and base address unset and without a network request.

## Dependencies

Ticket 0055, Ian's ruling in `sdlc/issues/closed/2026-09-21-filter-details-must-still-filter.md`, and the threshold ruling in `sdlc/planning/build-queue-2026-09-21.md`. The open low-edge issue records the evidence that led to the queue ruling.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 1
- Cost of error: 1
- Total: 5
- Minimum level floor: none
- Final level: 2
- Reasons: both changes are local and already ruled, but they alter visible record membership and one exact threshold outcome. The tests must distinguish selection from rendering and pin both boundary values.
- Selected model: `gpt-5.6-luna` with high reasoning.

Re-score if the fix changes request bytes, a public result shape, scheduling, or any exit code.

## Review

Independent design review rejected the first draft because “includes both boundaries” contradicted the exact high-edge rule and because the dependency named an open issue as the ruling. This revision states all three intervals exactly and points to the build queue's durable threshold ruling. The same reviewer accepted the corrected design.

Independent code review rejected the first implementation because the shared conformance cases still used probabilities inside and above the band rather than its exact edges, and one filter option sentence still promised a result for every input record. The repair pins `LOW` and `HIGH` in the shared cases and says “per kept record.” The same reviewer accepted the repaired implementation and found no regression.
