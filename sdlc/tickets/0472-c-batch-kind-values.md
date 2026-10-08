# 0472: Match C batch metadata to the public constants

Status: in progress. The native mapping and complete-result warning path are corrected; final gates remain.

Milestone: 0.2

Owner: builder.
Severity: medium result correctness.

Reviews: revision caddaecbc, accept

## Outcome

C batch_setting and batch_warning identify records as THINKTHEN_BATCH_RECORDS_V1 and max as THINKTHEN_BATCH_MAX_V1. Existing bindings read the correct batch meaning without an ABI layout change.

## Evidence

- Starts from: 0462 fresh High ABI/ownership review; libraries/c/src/complete/metadata.rs::batch emits Max=1, Records=2, opposite to include/thinkthen.h. Go nativeBatchKind follows the header, so callers receive reversed meanings. The existing layout check passes because it does not check semantic enum values.
- Keeps: Public header values and struct layout, owned result lifetime, batch count, metadata warnings and all binding APIs.
- Changes: Correct the native discriminants once. Preserve an authored-threshold marker so complete results emit batch warnings only for tuned questions. Add focused public-getter assertions for both kinds and warning sides, with an affected foreign consumer checking the mapped meaning. No per-language conversion rewrite.
- Proof: Fresh ticket and whole-change review. Existing C metadata/ABI tests and one foreign consumer, affected package check, policy and applicable full tests/lint. Keep exact kind/count assertions; no new parity tool or hosted workflow.
- Defers: No ABI revision, new header field, per-language write-ups, paid calls or release management.
