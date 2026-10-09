# 0478: Let callers supply context for each recognition stage

Status: COMPLETE. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision 8656eb8ba, accept

Landed: ef46e41

## Outcome

Design caller-supplied context separately for recognize stages so a caller can guide boundary, kind/edge, or relation questions without repeating one shared summary everywhere.

## Evidence

- Reviewed design: [ADR 0126](../planning/adr/0126-recognition-stage-context-and-boundary-proposals.md) defines stage names, call/saved/per-record precedence, empty clearing and stage-local identity. It uses Request-generated types rather than a second carrier grammar.

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 2; the current shared `--context` can carry a short summary. Existing `step_one_context`, `step_two`, and relation plans use one aggregate context.
- Keeps: The whole-call context default and existing request/cache identity when no stage setting is supplied. Context remains caller data, not SDK routing policy.
- Changes: Under Ian's 2026-10-08 ruling, settle stage names, precedence with shared and per-record context, saved/native/CLI typed spellings, admission before sends, request-size behavior, and per-stage cache identity in the specification. Implement in 0.2 through the shared request contract and generated bindings.
  Claim `crates/thinkthen/src/core/mod.rs`, `crates/thinkthen/src/core/recognize.rs`, `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/core/recognize/stage_context.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/public/recognize.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/options.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/definition/schema.rs`, `crates/thinkthen/src/public/request/tests.rs`, `crates/thinkthen/src/public/results/aggregate_reading.rs`, `crates/thinkthen/src/public/results/complete.schema.json`, `crates/thinkthen/src/engine/facade/recognize.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/request.rs`, `crates/thinkthen/src/cli/recognize/config.rs`, `crates/thinkthen/tests/backend/recognize/context.rs`, `crates/thinkthen/tests/backend/recognize/sized.rs`, `crates/thinkthen/tests/native_complete/recognize_context.rs`, `specification/recognize.md`, `specification/request.schema.json` and `specification/result.schema.json`.
- Proof: Offline exact-request and replay tests show context reaches only selected stages, omitted controls preserve existing bodies, changed context changes affected identities, and invalid combinations send nothing.
- Defers: Proxy configuration policy and model selection by stage.

These declarations accompany the implemented stage-context API and the accepted ADR. Generated request and saved-question schemas use this same type.

### Added public declarations

```text
struct RecognitionStageContext
RecognitionStageContext::boundary: Option<String>
RecognitionStageContext::kind_edge: Option<String>
RecognitionStageContext::relation: Option<String>
RequestOptions::stage_context: Option<RecognitionStageContext>
const fn RecognitionStageContext::is_empty(&self) -> bool
fn RecognitionReading::stage_context(&self) -> &RecognitionStageContext
fn Recognize::boundary_context(self, &str) -> Recognize
fn Recognize::kind_edge_context(self, &str) -> Recognize
fn Recognize::relation_context(self, &str) -> Recognize
fn Recognize::with_stage_context(self, RecognitionStageContext) -> Recognize
impl Default for RecognitionStageContext
impl Deserialize<'de> for RecognitionStageContext
impl Serialize for RecognitionStageContext
```

## Progress

- 2026-10-08 started
