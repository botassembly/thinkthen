# 0478 — recognize-stage-context

Status: OPEN. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

## Outcome

Design caller-supplied context separately for recognize stages so a caller can guide boundary, kind/edge, or relation questions without repeating one shared summary everywhere.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 2; the current shared `--context` can carry a short summary. Existing `step_one_context`, `step_two`, and relation plans use one aggregate context.
- Keeps: The whole-call context default and existing request/cache identity when no stage setting is supplied. Context remains caller data, not SDK routing policy.
- Changes: Under Ian's 2026-10-08 ruling, settle stage names, precedence with shared and per-record context, saved/native/CLI typed spellings, admission before sends, request-size behavior, and per-stage cache identity in the specification. Implement in 0.2 through the shared request contract and generated bindings.
- Proof: Offline exact-request and replay tests show context reaches only selected stages, omitted controls preserve existing bodies, changed context changes affected identities, and invalid combinations send nothing.
- Defers: Proxy configuration policy and model selection by stage.
