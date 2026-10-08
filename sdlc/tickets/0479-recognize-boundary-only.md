# 0479 — recognize-boundary-only

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.3
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

## Outcome

Design and implement a caller-requested step-1-only recognition mode with caller kinds and wording. Return boundary proposals without running kind, edge, or relation stages.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, narrowed ask 3. Existing recognition always decodes step 1 then may classify and adjust names in step 2. Existing caller instructions, entity definitions, and described kinds already reach step-1 questions. A kind-only span filter was dropped and needs no ticket.
- Keeps: Whole recognition as the default, its existing result and identity, and the proxy boundary for business routing. The new mode is explicit caller control.
- Changes: Settle mode spelling, output shape for unclassified proposals, probability and threshold meanings, interaction with relations and saved declarations, admission/errors, and mode-specific cache identity in the 0.3 specification before implementation. Use the existing BILOU decode and caller wording; do not invent a kind from skipped step 2.
- Proof: Exact-request tests prove step 1 alone sends, step-2/relation requests do not, caller kinds and wording reach each token question, default output remains identical, and mode identities cannot replay as whole recognition.
- Defers: Stage-specific model policy, automatic routing, and the dropped kind-only filter.
