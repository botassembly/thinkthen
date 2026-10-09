# 0476: Show recognition proposals and their probabilities

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

## Outcome

Make proposal probabilities canonical facts in the Rust result, available through CLI recognize `--details` and the public Rust API. Include the union of decoded stretches and supplied seeds, including proposals below the final cut. Python uses its existing tolerant reader; this work does not depend on 0496. Keep the bare value and default reading unchanged. PM approved this scope in ask 2 of `repos/agents/inbox/thinkthen/2026-10-09-pm-thinkthen-binding-tickets-rewritten-on-main-new-numbers-and-order.md`.

### Added public declarations

PM approved these borrowed API details within the canonical-result outcome on 2026-10-09; they land with the reviewed implementation.

```rust
struct RecognitionProposal<'a>
fn RecognitionProbabilities::judged_proposals(&self) -> impl ExactSizeIterator<Item = RecognitionProposal<'a>>
fn RecognitionProposal::range(&self) -> std::ops::Range<usize>
fn RecognitionProposal::span_probability(&self) -> f64
fn RecognitionProposal::selected(&self) -> Option<std::ops::Range<usize>>
fn RecognitionProposal::kind(&self) -> Option<&str>
fn RecognitionProposal::strength(&self) -> Option<f64>
fn RecognitionProposal::kept(&self) -> bool
```

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 4. `engine/facade/recognize.rs` has `rows`, decoded `stretches`, step-2 answers, and final `entities`; `core/recognize.rs::settle` currently discards declined, duplicate, and under-cut candidates. `SpanOdds::span` computes the probability across valid floored BILOU paths. Existing `answer.pieces` and `answer.names` do not tell which candidates survived.
- Keeps: Decode, edge/kind selection, deduplication, threshold, final entity schema, cache identity, request count, and no extra model call. Existing `strength` remains the four-decimal product of chosen-kind probability and original step-1 span probability, and is a ranking score, not a calibrated probability.
- Changes: Add `answer.proposals` to the CLI `--details` projection only, in decoded stretch order; do not require a native complete-result or host SDK schema change. Each entry identifies the original proposed `[start,end)` scalar span, `span_probability` as the unrounded `SpanOdds::span` of that exact step-1 stretch, optional selected `[start,end)` after edge choice, optional `strength` only when a kind was selected and the product was computed, and `kept` for the proposal that becomes a printed entity. Capture these typed fields while `settle` chooses, declines, deduplicates, and cuts; do not reconstruct retention by matching final text or compute every possible BILOU path. A declined kind has no invented strength. Preserve the selected kind when present so duplicate resolution is inspectable. Add the smallest typed internal capture and CLI serializer/schema projection.
  Claim `crates/thinkthen/src/engine/facade/recognize/**`, `crates/thinkthen/src/cli/recognize/**`, `specification/recognize.md` and `conformance/**`.
- Proof: A saved or loopback case covers a kept span, declined kind, under-cut span, edge-adjusted span, and duplicate where one proposal wins. Pin proposal order, offsets, actual probabilities and omitted uncomputed fields; pin unchanged bare output, requests and replay/cache keys. A failed record emits no partial stage result; prior completed records retain their existing behavior.
- Defers: Calibration, alternative undecoded spans, a trace store/export API, and host SDK projection.

## Consumer scope ruling

The TCGA message `2026-10-08-tcga-demo-check-seed-judgment-separately-from-the-boundary-cut.md` asks for every supplied seed's bounds, kind distribution, span probability and final disposition, including seeds below the strength cut, through CLI, Rust Request and Python. Landed 0506 adds supplied seeds to the proposal union and preserves the existing strength formula; it does not expose the missing span probability or disposition. The CLI-only scope above cannot satisfy all three requested routes.

PM approved canonical proposal facts in the Rust result and public Rust API on 2026-10-09, overriding the earlier CLI-only Changes and host-projection deferral above. Capture the admitted proposal union once, including supplied seeds, and project it through CLI details. Python's actual complete reader rejects unknown fields, so the approved compatibility repair adds only typed proposal fields and preserves older documents; seed admission and broader Python migration remain with 0496. Keep the existing threshold and model calls unchanged. Callers inspect proposal facts separately from final kept entities.

## Progress

- 2026-10-09 started
