# 0286 — DuckDB settings keyed forms and macros (T4)

Status: Done for the scoped implementation after fresh High code review ACCEPT of `698c84354e26ecf155f7a057de385abacd6bc896`. The source and copied Linux extension passed the selected installed proofs. Other-platform and release-archive qualification remain in their existing issues; this ticket does not close those outcomes.

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

T1's public `Settings` parser and T7's `Engine::plan_with` and active send reservation are merged. The shipped package route is `cpp/src/{portable,plan,find,nested,relate,thinkthen,warm}.cpp` through the Rust bridge's `ffi/portable*`, `ffi/plan/ffi.rs` and `relate/ffi.rs`; `cpp/CMakeLists.txt` explicitly includes both new members. Under accepted ADR 0081, `sdlc/scripts/release-pack`, `check.sh` and all four target builds select only the C++ route. The obsolete raw C API registration, entry and `libduckdb-sys` dependency are retired; `src/engines.rs` and `src/signal.rs` remain source imports of the C++ bridge. The unpublished inert root Cargo manifest remains for binding-policy metadata and exports no DuckDB entry.

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

The merged 0289 public planner returns P1's native STRUCT with the independent 120-byte body, 61–109 estimate, and zero loopback accepts even without a key. A positive one-request total admits exactly one actual call then gives the typed spent-total refusal. A wrong settings type, malformed/duplicate keyed object, question/settings conflict and unknown named argument refuse without sending. The retained seven prior package failure criteria, vector/keyed body, selected file-access and held cancellation checks pass against the matching rebuilt stock host. The current Linux build and byte-identical copied extension are not release archives or other-platform qualification; [the build record](../records/0286-duckdb-settings-keyed-forms-and-macros-build.md) names the exact selected receipts.

High review exposed one I9 macro gap: a typed NULL `VARCHAR` members placeholder followed by settings lost the fourth argument before the native guard. The shared macro now carries that NULL distinction, while two non-NULL settings objects still refuse. ADR 0105 item 8 also required ordinary retryability suffixes on direct C++ and typed bridge failures; exact plain removal sentences remain unchanged. The reviewed shared settings conflict fix `129ffe0a7` was merged, and the exact batch collision now refuses as Usage with zero sends through the installed host. The new core estimated-input constructor key still refuses at this host boundary. The focused [build record](../records/0286-duckdb-settings-keyed-forms-and-macros-build.md) gives the source/copy identity, selected checks and remaining package qualification.

The same High review caught an error provenance gap: caller SQL could forge a `thinkthen backend` label and `(retryable: yes)` through relate's host query error. The host-query boundary now fixes its own Usage/nonretryable context around those preserved words, while the separate Rust bridge reply path retains typed retryability. The existing unrelated-query case pins the forgery with zero sends; selected nested/catalog, backend 503 and plain-removal checks remain green. The build record names the red witness, corrected proof and matching copied artifact.
