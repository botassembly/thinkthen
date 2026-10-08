# 0506: Judge caller-supplied recognition spans

Status: OPEN.

Milestone: 0.2

Depends on: 0490
Depends on: 0491

## Outcome

Let a caller supply character spans for each record, with optional kinds. Merge these with step-1 proposals and judge them in step 2 through the existing recognition route. A supplied kind is a proposal, never a confirmed answer.

## Evidence

- Starts from: Ask 11 of the PM message `2026-10-08-pm-thinkthen-ticket-cleanup-and-0-2-priority-order.md` and the TCGA three-hooks ruling. The typed Request contract and tagged examples provide shared admission and per-record controls.
- Keeps: Existing behavior when seeds are absent; original record identity; Unicode character span semantics; final thresholds, model selection, cache/replay and request limits. Invalid input sends nothing.
- Changes: Define a typed span declaration and a per-record pointer in the shared Request, recognition options and generated schema. Validate finite integer character bounds, ordered nonempty intervals within the original text, and optional kinds against the caller's declared kinds before any call. Specify duplicate and overlapping proposals using the existing proposal merge rules. Feed admitted seeds into existing step-2 classification rather than a separate confirmation route. Document errors, missing versus empty versus null and preservation of original coordinates. Claim `crates/thinkthen/src/core/recognize/**`, `crates/thinkthen/src/engine/facade/recognize/**`, `crates/thinkthen/src/public/**`, `crates/thinkthen/src/cli/**`, `crates/thinkthen/tests/**`, `specification/recognize.md` and `specification/request.schema.json`.
- Proof: An outside-in generic fixture shows a seed missed by step 1 being classified in step 2, an incorrect supplied kind being corrected or refused, and native Request and CLI agreeing. Cover Unicode bounds, overlap/duplicates, absent/empty/null, invalid pointer and kind, exact zero sends on invalid input, and cache/replay identity. Preserve unchanged default request bytes.
- Defers: A revise mode, a new function, business policy and independent per-language validation. Surfaces adopt these settled fields once through the existing migration tickets.
