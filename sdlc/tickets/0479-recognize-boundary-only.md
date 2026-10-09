# 0479: Let callers request recognition boundaries without classification

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Design and implement a caller-requested step-1-only recognition mode with caller kinds and wording. Return boundary proposals without running kind, edge, or relation stages.

## Evidence

- Reviewed design: [ADR 0126](../planning/adr/0126-recognition-stage-context-and-boundary-proposals.md) defines the typed proposal value, saved/call mode precedence, threshold meaning, result/schema ownership and reusable logical boundary answers with mode-specific complete identity.

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, narrowed ask 3. Existing recognition always decodes step 1 then may classify and adjust names in step 2. Existing caller instructions, entity definitions, and described kinds already reach step-1 questions. A kind-only span filter was dropped and needs no ticket.
- Keeps: Whole recognition as the default, its existing result and identity, and the proxy boundary for business routing. The new mode is explicit caller control.
- Changes: Settle mode spelling, output shape for unclassified proposals, probability and threshold meanings, interaction with relations and saved declarations, admission/errors, and mode-specific cache identity in the 0.2 specification before implementation. Use the existing BILOU decode and caller wording; do not invent a kind from skipped step 2. Adopt the shared request contract and generated bindings under Ian's 2026-10-08 ruling.
  Claim `crates/thinkthen/src/core/recognize.rs`, `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/public/recognize.rs`, `crates/thinkthen/src/public/recognize_question.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/options.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/definition/**`, `crates/thinkthen/src/public/request/tests.rs`, `crates/thinkthen/src/public/results/aggregate_reading.rs`, `crates/thinkthen/src/public/results/complete.schema.json`, `crates/thinkthen/src/public/complete/recognize/**`, `crates/thinkthen/src/engine/facade/recognize.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/request.rs`, `crates/thinkthen/src/cli/recognize/**`, `crates/thinkthen/tests/backend/recognize/**`, `crates/thinkthen/tests/native_complete/recognize*.rs`, `specification/recognize.md`, `specification/request.schema.json` and `specification/result.schema.json`. Add further individually named seams only when implementation needs them; no claim covers the whole public API or conformance tree.
- Proof: Exact-request tests prove step 1 alone sends, step-2/relation requests do not, caller kinds and wording reach each token question, and default output remains identical. Identical boundary questions intentionally reuse saved question answers across modes; resolved mode separates complete answer identity and typed value, including when whole recognition sends no later questions. Boundary proposals cannot stand in for a complete whole result. Add no identity-only model wording or cache-key mechanism.
- Additional named seams from ADR 0126: `crates/thinkthen/src/core/recognition_result.rs`, `crates/thinkthen/src/core/result/complete/recognize.rs`, `crates/thinkthen/src/result_json/complete/recognize.rs`, `crates/thinkthen/src/schema_tests.rs`, `crates/thinkthen/src/schema_tests/complete.rs`, `crates/thinkthen/src/public/results/complete_recognize.rs`, `crates/thinkthen/src/public/results/source_recognition.rs`, `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/core/mod.rs` and `crates/thinkthen/src/core/result/complete/tests/aggregate.rs`. Request mode belongs to `public/request/options.rs`; the independent deadline repair owns `public/options.rs`.
- Defers: Stage-specific model policy, automatic routing, and the dropped kind-only filter.
- Further individually named implementation seams: `crates/thinkthen/src/public/recognize/proposals.rs`, `crates/thinkthen/tests/native_complete.rs` and `crates/thinkthen/tests/backend/recognize.rs`. Preserve implicit versus authored relation thresholds in RequestDefinition wire encoding under ADR 0126; retain default whole complete output and identity.
- Offline reading seams: `crates/thinkthen/src/core/measure/items.rs`, `crates/thinkthen/src/core/measure/key.rs`, `crates/thinkthen/src/cli/audit/cases.rs` and `specification/audit.md`. Read and compare unclassified proposal offsets and probabilities under the resolved mode; retain the existing run-cut floor and Whole decoding. Exercise audit and diff through the existing recognition boundary tests.
- Offline table rendering seam: `crates/thinkthen/src/cli/diff.rs`. Print an unclassified proposal with its offsets and four-place probability; preserve Whole table formatting and JSON comparison behavior.

### Added public declarations

```text
enum RecognitionMode
RecognitionMode::Whole
RecognitionMode::BoundaryOnly
RequestOptions::mode: Option<RecognitionMode>
RequestOptions::relation_threshold: Option<RequestThreshold>
fn Recognize::with_mode(self, RecognitionMode) -> Recognize
const fn RecognizeBuilder::mode(self, RecognitionMode) -> RecognizeBuilder
struct BoundaryProposal
fn BoundaryProposal::text(&self) -> &str
const fn BoundaryProposal::start(&self) -> usize
const fn BoundaryProposal::end(&self) -> usize
const fn BoundaryProposal::length(&self) -> usize
const fn BoundaryProposal::probability(&self) -> f64
enum RecognitionValue
RecognitionValue::Whole::entities: &'a [RecognizedEntity]
RecognitionValue::Whole::relations: Option<&'a [Relation]>
RecognitionValue::BoundaryOnly(&'a [BoundaryProposal])
const fn Recognized::mode(&self) -> RecognitionMode
fn Recognized::proposals(&self) -> Option<&[BoundaryProposal]>
fn Recognized::value(&self) -> RecognitionValue<'_>
const fn RecognitionReading::mode(&self) -> RecognitionMode
fn RecognitionProbabilities::proposals(&self) -> Option<Vec<BoundaryProposal>>
struct SourceBoundaryProposal
const fn SourceBoundaryProposal::proposal(&self) -> &BoundaryProposal
const fn SourceBoundaryProposal::location(&self) -> &SourceLocation
const fn SourceRecognition::mode(&self) -> RecognitionMode
fn SourceRecognition::proposals(&self) -> Option<&[SourceBoundaryProposal]>
impl Default for RecognitionMode
impl Deserialize<'de> for RecognitionMode
impl Serialize for RecognitionMode
```

## Progress

- 2026-10-08 started
