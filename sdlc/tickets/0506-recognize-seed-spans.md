# 0506: Judge caller-supplied recognition spans

Status: OPEN.

Milestone: 0.2

Depends on: 0490
Depends on: 0491

Reviews: revision 7a5c4a25d, reject

Reviews: revision b6bb29af3, accept

### Added public declarations

```text
RecognitionSeedSpan::end: usize
RecognitionSeedSpan::kind: Option<String>
RecognitionSeedSpan::start: usize
RecordInput::seed_spans: Option<Vec<RecognitionSeedSpan>>
RequestItem::seed_spans: Option<Vec<RecognitionSeedSpan>>
RequestOptions::seed_spans: Option<Vec<RecognitionSeedSpan>>
RequestOptions::seed_spans_field: Option<String>
fn Recognize::with_seed_spans(self, Vec<RecognitionSeedSpan>) -> Recognize
fn RecordReading::with_seed_spans_field(self, &str) -> Result<RecordReading, Error>
impl Deserialize<'de> for RecognitionSeedSpan
impl Serialize for RecognitionSeedSpan
struct RecognitionSeedSpan
```

Reviews: revision 2010023e8, accept

## Outcome

Let a caller supply character spans for each record, with optional kinds. Merge these with step-1 proposals and judge them in step 2 through the existing recognition route. A supplied kind is a proposal, never a confirmed answer.

## Evidence

- Starts from: Ask 11 of the PM message `2026-10-08-pm-thinkthen-ticket-cleanup-and-0-2-priority-order.md` and the TCGA three-hooks ruling. The typed Request contract and tagged examples provide shared admission and per-record controls.
- Keeps: Existing behavior when seeds are absent; original record identity; Unicode character span semantics; final thresholds, model selection, cache/replay and request limits. Invalid input sends nothing.
- Changes: Add recognize-only `RecognitionSeedSpan { start, end, kind? }`, `RequestOptions.seed_spans`, `RequestOptions.seed_spans_field`, `RequestItem.seed_spans` and CLI `--seed-spans-field`. Omission or a missing selected member uses the fallback; `[]` clears it; explicit null refuses. Explicit item seeds conflict with a pointer. Omitted kind supplies no hint; null refuses; present kinds must match declared kinds exactly. These are record inputs, not answered examples or saved literal spans. Generate the schema from typed Request. Claim `crates/thinkthen/src/core/recognize.rs`, `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/engine/facade/recognize.rs`, `crates/thinkthen/src/engine/facade/recognize/**`, `crates/thinkthen/src/public/**`, `crates/thinkthen/src/cli/**`, `crates/thinkthen/tests/**`, `specification/recognize.md` and `specification/request.schema.json`.
- Span admission: Use zero-based Unicode scalar `[start,end)` offsets into the actual recognition evidence, without normalization. Require checked integer bounds, a nonempty interval within the text, and both edges at existing recognition piece edges. Refuse unrepresentable boundaries before calls; never round them. Document this restriction beside the public declaration.
- Proposal handling: Union decoded stretches with seeds, deduplicate identical bounds regardless of hint, retain distinct overlaps and sort by start then end. Group evidence extends to the maximum end across its proposals, including nested stretches. With declared kinds, ask the complete set and decline option; a hint neither forces nor restricts the answer. With no caller kinds, judge seeds through the existing choose route using internal ENTITY and decline options, including single-piece seeds. Preserve the existing final strength formula and threshold: a seed missed by step 1 can be classified yet still fail the final cut. Expose that distinction in details and claims. Seeds leave boundary questions and their identity unchanged; generated step-2 questions and evidence carry their identity.
- Planning: Include supplied seeds and their overlaps in name, classification-question and relation bounds. Do not assume one proposal per piece or one question per untyped seed. Preserve existing bounds without seeds. A no-send plan fixture checks overlapping seeds without declared kinds and bounds the later requests without claiming their answers are known.
- Proof: An outside-in generic fixture shows a seed missed by step 1 being classified in step 2, an incorrect supplied kind being corrected or refused, and native Request and CLI agreeing. Check details independently of the final strength cut. Cover single-piece seeds without kinds, nested overlaps and maximum evidence extent, duplicates, emoji, combining characters, leading excluded scalars, projected text, absent/empty/null, invalid pointer and kind, exact zero sends on invalid input, and cache/replay identity. Preserve unchanged default request bytes.
- Defers: A revise mode, a new function, business policy and independent per-language validation. Surfaces adopt these settled fields once through the existing migration tickets.

## Progress

- 2026-10-08 started
