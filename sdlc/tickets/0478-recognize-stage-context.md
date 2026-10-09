# 0478: Let callers supply context for each recognition stage

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Design caller-supplied context separately for recognize stages so a caller can guide boundary, kind/edge, or relation questions without repeating one shared summary everywhere.

## Evidence

- Reviewed design: [ADR 0126](../planning/adr/0126-recognition-stage-context-and-boundary-proposals.md) defines stage names, call/saved/per-record precedence, empty clearing and stage-local identity. It uses Request-generated types rather than a second carrier grammar.

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 2; the current shared `--context` can carry a short summary. Existing `step_one_context`, `step_two`, and relation plans use one aggregate context.
- Keeps: The whole-call context default and existing request/cache identity when no stage setting is supplied. Context remains caller data, not SDK routing policy.
- Changes: Under Ian's 2026-10-08 ruling, settle stage names, precedence with shared and per-record context, saved/native/CLI typed spellings, admission before sends, request-size behavior, and per-stage cache identity in the specification. Implement in 0.2 through the shared request contract and generated bindings.
  Claim `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/core/recognize/questions.rs`, `crates/thinkthen/src/public/recognize.rs`, `crates/thinkthen/src/public/recognize_question.rs`, `crates/thinkthen/src/public/request/options.rs`, `crates/thinkthen/src/public/complete/recognize/**`, `crates/thinkthen/src/engine/facade/recognize.rs`, `crates/thinkthen/src/cli/recognize.rs`, `crates/thinkthen/src/cli/recognize/**`, `crates/thinkthen/src/result_json/complete/recognize.rs`, `specification/recognize.md` and `specification/request.schema.json`. Add individually named paths when implementation requires them; do not claim the whole public API or conformance tree.
  Additional named seams: `crates/thinkthen/src/core/mod.rs`, `crates/thinkthen/src/core/recognize/stage_context.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/definition.rs`, `crates/thinkthen/src/public/request/tests.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/request.rs`, `crates/thinkthen/tests/backend/recognize/context.rs` and `crates/thinkthen/tests/native_complete/recognize_context.rs`.
  Stage execution and saved schema also own `crates/thinkthen/src/core/recognize.rs` and `crates/thinkthen/src/public/request/definition/schema.rs`.
- Proof: Offline exact-request and replay tests show context reaches only selected stages, omitted controls preserve existing bodies, changed context changes affected identities, and invalid combinations send nothing.
- Defers: Proxy configuration policy and model selection by stage.

## Progress

- 2026-10-08 started
