# 0490: Render caller-supplied tagged examples for recognition

Status: OPEN.

Milestone: 0.2

Depends on: 0489
Depends on: 0491

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Callers supply tagged examples without knowing recognition's piece boundaries or question wording. Build only after experiment 469 demonstrates benefit and the PM receives the added work size; target 0.2 under that condition.

## Evidence

- Starts from: TCGA asks 3 and 4 and PM high-priority context ask 3 of 2026-10-08. Experiment 469 owns usefulness evidence; the builder implements software contracts and does not interpret clinical content.
- Keeps: Per-record context from 0489 as an independent input; default requests and keys when examples are absent; all languages and one endpoint. Retrieval remains caller-owned.
- Changes: Review an ADR for `--examples FILE` and `--examples-field POINTER`, shared and per-record precedence, inline bracket notation and JSON Lines containing text plus Unicode-scalar spans and declared kinds. Render only answered step-1 questions through recognition's own piecer. Show current-pass kinds and mark other entities OUT; add no step-2 examples. Refuse undeclared kinds and span edges inside a piece before calls. Show rendered examples and token accounting in `--plan`; include examples in stage cache identities and the shared Request schema. Claim `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/engine/facade/recognize/**`, `crates/thinkthen/src/public/**`, `crates/thinkthen/src/cli/recognize/**`, `specification/recognize.md`, `specification/request.schema.json` and `conformance/**`. Family tickets adopt the schema once. The PM cleared the experiment gate on 2026-10-08; build after 0489 and before the controls.
- Proof: Generic software examples cover both forms, non-ASCII spans, per-record isolation, invalid kind/edge refusal with zero sends, plan rendering, exact requests and offline cache/replay parity through typed interfaces. Do not promise model improvement from software fixtures.
- Defers: Implementation until experiment 469's evidence arrives; retrieval, model selection and proxy business logic. Review the ticket now. Give the PM an incremental size and dependency assessment before starting implementation.
