# 0286 — DuckDB settings keyed forms and macros (T4)

Status: Draft preparation. Accepted ADR 0105 settles the outcome; implementation and issue closure remain open.

## Outcome

JSON-object `VARCHAR` overload beside the members `VARCHAR[]`; keyed `_many`
registration with the probed constructor
`to_json(map_from_entries(list({'key': CAST(id AS VARCHAR), 'value': title}
ORDER BY id)))`; deadline/context slot removals; macro named arguments
registered through `CreateMacroInfo` as the daily form (source-probed); the
scalar positional `:=` limit documented.

The macro CreateMacroInfo must set info.internal=true. Keep the seven old DuckDB package failures as distinct exact-case criteria and the child-env guard as a separate prerequisite.

Register `thinkthen_plan(question, keyed_json[, settings])` returning a native **STRUCT** with `requests`, `records`, `estimated_bytes`, the two-member `estimated_input_tokens` band, `upper_bound` and first request body. Its `VARCHAR` keyed JSON and optional settings take the same converter/core parser as `_many`; it calls 0283's summary through 0289's Engine wrapper. Preserve the vector daily form and the `VARCHAR[]`-members overload. A wrong settings type, invalid JSON or duplicate question/settings key refuses before send. P1 pins the exact no-send body/count/byte/token-band result through the STRUCT, alongside a small valid/invalid DuckDB conversion table.

## Prerequisites and proposed files

Prerequisite: T1; T7 before final cap/plan acceptance. Proposed file families: `databases/duckdb/cpp/src/{thinkthen.cpp,scalar_settings.cpp,scalar_settings.hpp,listed_result.cpp,listed_result.hpp,warm.cpp,warm.hpp}; databases/duckdb/src/{lib.rs,scalars.rs,tables.rs,questions.rs}; databases/duckdb/bridge/src/ffi/{settings.rs,listed.rs}; DuckDB tools/README/ratchets`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Installed DuckDB P1 STRUCT carries `records=1`, `requests=1`, 120 bytes, 61–109 token band and exact first body with zero listener accepts; invalid/duplicate input also sends zero. Retain macro name/unknown refusal, overload resolution, E1 vector/keyed bodies and seven old-case reconciliation. Record source and installed receipts separately; run affected functional cases and focused format/policy/ratchet/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name later qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: DuckDB settings overload, keyed many, native STRUCT plan over the shared core summary and true named macros with internal registration.
- Proof: P1 exact no-send STRUCT summary and invalid/duplicate refusal, installed macro/overload proof, E1 bodies and old-case reconciliation.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
