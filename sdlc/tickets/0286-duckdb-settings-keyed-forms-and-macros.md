# 0286 — DuckDB settings keyed forms and macros (T4)

Status: Accepted design; C++ host-local implementation is a review candidate. Native STRUCT plan, 0289 process-cap integration, package qualification and issue closure remain open.

## Outcome

JSON-object `VARCHAR` overload beside the members `VARCHAR[]`; keyed `_many`
registration with the probed constructor
`to_json(map_from_entries(list({'key': CAST(id AS VARCHAR), 'value': title}
ORDER BY id)))`; deadline/context slot removals; macro named arguments
registered through `CreateMacroInfo` as the daily form (source-probed); the
scalar positional `:=` limit documented.

The macro CreateMacroInfo must set info.internal=true. Keep the seven old DuckDB package failures as distinct exact-case criteria and the child-env guard as a separate prerequisite.

Register `thinkthen_plan(question, keyed_json[, settings])` returning a native **STRUCT** with `requests`, `records`, `estimated_bytes`, the two-member `estimated_input_tokens` band, `upper_bound` and first request body. Its `VARCHAR` keyed JSON and optional settings take the same converter/core parser as `_many`; it calls 0283's summary through 0289's Engine wrapper. Preserve the vector daily form and the `VARCHAR[]`-members overload. A wrong settings type, invalid JSON or duplicate question/settings key refuses before send. P1 pins the exact no-send body/count/byte/token-band result through the STRUCT, alongside a small valid/invalid DuckDB conversion table.

## Prerequisites and implementation files

T1's public `Settings` parser is merged. The package route is `cpp/src/{portable,find,nested,relate,thinkthen,warm}.cpp` through the Rust bridge's `ffi/portable*` and `relate/ffi.rs`; `cpp/CMakeLists.txt` explicitly includes the new portable member. Tools, `check.sh`, README and DuckDB ratchets track that route. The older `databases/duckdb/src` Rust loadable crate remains a separate, unshipped source path with old signatures; it was inspected, not used as proof of the C++ package. T7's public `Engine::plan` and active request-cap API from 0289 must merge before the native STRUCT and final cap criterion can be implemented. The eventual build must decide whether that separate Rust loadable source is supported under the same new public contract or retired from source qualification; it cannot be counted as migrated from this C++ proof.

## Smallest meaningful proof

Installed DuckDB P1 STRUCT carries `records=1`, `requests=1`, 120 bytes, 61–109 token band and exact first body with zero listener accepts; invalid/duplicate input also sends zero. Retain macro name/unknown refusal, overload resolution, E1 vector/keyed bodies and seven old-case reconciliation. Record source and installed receipts separately; run affected functional cases and focused format/policy/ratchet/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name later qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: DuckDB settings overload, keyed many, native STRUCT plan over the shared core summary and true named macros with internal registration.
- Proof: P1 exact no-send STRUCT summary and invalid/duplicate refusal, installed macro/overload proof, E1 bodies and old-case reconciliation.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The actual v1.5.5 catalog needs `CreateMacroInfo` with `internal=true`: scalar `:=` binds positionally. An isolated installed test proved out-of-order named binding and unknown-name refusal. JSON `VARCHAR` settings coexist with native `VARCHAR[]` members, while explicitly NULL settings mean `{}`; an old numeric slot produces the corpus's removal sentence. The six-song vector and keyed calls used one packed request plus a cache answer in one scratch session, and byte-identical bodies when run with separate fresh caches. One cache answer is independent of the six record count.

The retained scalar and find decoders, `try_details` safe failure carrier, recognition result decoder and relate query guard could be reused. The old `thinkthen.cpp` scalar registrations and obsolete private find bridge were deleted after the new calls passed; old warm functional tests were replaced by an outside-in exact removal and zero-send case, while distinct file, invalid-input, secrecy, try-details, and saved-body cases remain. New unsafe C entries live in `ffi.rs` files, as the binding policy requires. The parser and due/batch helpers were shared instead of adding another host settings grammar. The source counters are measured in the [build record](../records/0286-duckdb-settings-keyed-forms-and-macros-build.md).

The current Linux build and byte-identical copied extension are not a release archive or other-platform qualification. The seven prior package failure criteria have selected local receipts in that record. P1's exact native STRUCT, its zero-send plan, and final process-cap behavior have no receipt until 0289 lands; no private core estimator was copied.
