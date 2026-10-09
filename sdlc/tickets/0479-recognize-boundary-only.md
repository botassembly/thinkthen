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
- Defers: Stage-specific model policy, automatic routing, and the dropped kind-only filter.

## Progress

- 2026-10-08 started
