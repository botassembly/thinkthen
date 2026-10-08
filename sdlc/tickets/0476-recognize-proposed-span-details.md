# 0476 — recognize-proposed-span-details

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

## Outcome

Add every step-1 decoded name stretch to CLI recognize `--details`, with its span probability, computed strength when available, and whether that proposal survives. Keep the bare value and default reading unchanged.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 4. `engine/facade/recognize.rs` has `rows`, decoded `stretches`, step-2 answers, and final `entities`; `core/recognize.rs::settle` currently discards declined, duplicate, and under-cut candidates. `SpanOdds::span` computes the probability across valid floored BILOU paths. Existing `answer.pieces` and `answer.names` do not tell which candidates survived.
- Keeps: Decode, edge/kind selection, deduplication, threshold, final entity schema, cache identity, request count, and no extra model call. Existing `strength` remains the four-decimal product of chosen-kind probability and original step-1 span probability, and is a ranking score, not a calibrated probability.
- Changes: Add `answer.proposals` to the CLI `--details` projection only, in decoded stretch order; do not require a native complete-result or host SDK schema change. Each entry identifies the original proposed `[start,end)` scalar span, `span_probability` as the unrounded `SpanOdds::span` of that exact step-1 stretch, optional selected `[start,end)` after edge choice, optional `strength` only when a kind was selected and the product was computed, and `kept` for the proposal that becomes a printed entity. Capture these typed fields while `settle` chooses, declines, deduplicates, and cuts; do not reconstruct retention by matching final text or compute every possible BILOU path. A declined kind has no invented strength. Preserve the selected kind when present so duplicate resolution is inspectable. Add the smallest typed internal capture and CLI serializer/schema projection.
- Proof: A saved or loopback case covers a kept span, declined kind, under-cut span, edge-adjusted span, and duplicate where one proposal wins. Pin proposal order, offsets, actual probabilities and omitted uncomputed fields; pin unchanged bare output, requests and replay/cache keys. A failed record emits no partial stage result; prior completed records retain their existing behavior.
- Defers: Calibration, alternative undecoded spans, a trace store/export API, and host SDK projection.
