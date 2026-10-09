# 0500: Move PostgreSQL onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision 9d6e7798c077c88fd48e69591c409c4c605167e8, accept

Reviews: revision 17d9c9b3b6d27b7ee512a607974acf0fd846d1ff, accept

## Outcome

Adopt shared Request and generated results in PostgreSQL through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current PostgreSQL adapter repeats admission/result construction.
- Keeps: privileged question-file loading and existing client evidence-reader workaround; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `databases/postgresql/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## Shared legacy result conversion

The reviewed design at `9d6e7798c077c88fd48e69591c409c4c605167e8` adds shared legacy result projections needed by PostgreSQL and the other compatibility adapters. Atomic and record-aware conversions to `Details` reuse the existing result/1 serializer; records retain their originals and an input-free scalar projection. Annotation keeps the existing primitive named-value semantics. Withhold authored values, named probabilities and nearest labels from existing and converted `Details` Debug output.

This ticket owns the narrow conversion files `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/legacy.rs`, `crates/thinkthen/src/public/results/complete_annotation.rs`, `crates/thinkthen/src/public/results/member.rs`, internal accessors in `crates/thinkthen/src/core/result.rs`, `crates/thinkthen/tests/request_legacy_projection.rs`, the existing Rust API specification and measured ratchet. It owns no Request executor or admission file. Keep exact public declarations with their implementation; publish no future export inventory ahead of code. Compare native batch scalar metadata with the corresponding batch oracle, rather than claiming it matches a non-batched call.

The reviewed metadata repair also owns `crates/thinkthen/src/public/complete.rs`. Its batch-run metadata and existing native member Details share one crate-private batch-warning helper in the result module. Preserve explicit saved batch tuning and effective threshold tuning, including default-max tag warnings. Keep opt-in attempt receipts accepted by the existing result/1 schema; the old batch Details omission is corrected rather than copied.

### Added public declarations

```text
fn CompleteAnnotated::value_json(&self) -> Result<String, Error>
fn CompleteChoice::legacy_details(&self) -> Result<Details, Error>
fn CompleteDecision::legacy_details(&self) -> Result<Details, Error>
fn CompleteRecord::legacy_details(&self) -> Result<Details, Error>
fn CompleteScore::legacy_details(&self) -> Result<Details, Error>
fn CompleteTags::legacy_details(&self) -> Result<Details, Error>
```

The generic record declaration is implemented for each of CompleteDecision, CompleteChoice, CompleteTags and CompleteScore with T: Serialize.

## Retained planning diagnostic

Keep `thinkthen_plan` on the existing shared native `Engine::plan_with` planner. This diagnostic sends no judgment request; preserve its admission, file authority and output, and prove zero sends. Do not replace planning with execution or duplicate estimation in PostgreSQL. The migration covers every executing public function; this explicit diagnostic ruling requires no additional Request planning API.

Migrated rank calls use the shared Request blank-input diagnostic, `the evidence is empty or blank`, in place of PostgreSQL's prior `evidence is text, not white space`. Preserve Usage classification, zero sends and secrecy. Record and test this intentional wording change; add no adapter-specific whitespace validator.

## Progress

- 2026-10-08 started
