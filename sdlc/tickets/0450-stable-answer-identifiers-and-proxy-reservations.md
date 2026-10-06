# 0450: Give each answer a stable identifier and reserve proxy policy fields

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Native admission WIP: reserved opaque IDs, normalized code readings,
question/recognition reservations and typed response overrides are concrete
types. Explicit null, empty and populated activation all refuse with the
same usage sentence at the public call/builder edge. Counted loopback public
tests pin zero sends, no started facts, refusal before replay loading and
Debug secrecy without imposing a Debug bound on actionable values. Surface
tokens are closed validated values; transport adoption and stable ID
computation/propagation remain open.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Every full logical result has a stable answer identifier usable for downstream outcome correlation. The typed proxy request/reply shape is reserved in 0.2; override execution waits for the admitted 0.3 proxy protocol.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 4; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add persistent observation identities and result answer IDs through one native implementation. Reserve optional proxy question_id, the resolved code threshold and a tagged override response. Publish schema/types and reject activation in 0.2 with zero sends; never send these fields to ordinary vendors.
- Proof: Cross-surface saved/live/cache/replay cases pin stable IDs for the same stored observations and normalized reading; changed cuts, member scope/rank position and refresh distinguish identities. Test partial member/failure versus null, aggregate ordering, no-store ephemeral answers, deterministic read-only legacy conversion and absence of key/header leaks. Inspect vendor request bytes to prove reserved fields cannot leak; active reserved inputs refuse before cache lookup or send. Vendor fields cannot attest proxy policy.
- Defers: Override execution, outcome-reporting service, routing/policy logic and proxy screens; these require the admitted 0.3 proxy protocol.

## Dependencies and ownership

0450 owns the identity/reservation implementation; 0442 owns the coordinated result/2/shared schema and ADR. 0443 supplies the existing collision-resistant ID facility and propagates typed metadata; 0444 stores/converts observation identity; 0426–0431/0410/0435 expose full result IDs; 0411 verifies changed-reading IDs and 0432 enforces every surface.

## Design notes

Call IDs identify invocations; request IDs identify sends; question keys identify stored inputs. Add observation_id per newly accepted wire answer at the engine edge, persisting it in cache/record metadata. Refresh creates a new observation even for equal probabilities; cache/replay retain it. No-store keeps identity only in memory. Derive answer_id in pure code using a versioned length-framed tuple of function/scope, ordered observation IDs and normalized resolved reading/aggregation/member identity. Exclude surface, file path, origin, call/request IDs, timing, cost, packing and display. Compute before host index conversion. Returning to an original rule over the same stored observations restores its ID. Aggregate roots include ordered children; successful null differs from failure. A partial result carries failed-member occurrence identity without storing failed answers; terminal errors fabricate no successful answer. Legacy identity is derived offline from the original validated saved key/answer and any existing stable metadata, before rekeying; no invented timestamp is required. Preserve it during migration and derive it in memory for read-only replay. Document that indistinguishable old snapshots cannot recover distinct historical occurrences. IDs provide correlation, not authentication or a new receipt system.

Expose answer_id on every new complete typed result regardless of whether probability details are requested. CLI details, SQL details/observations and frame carriers expose per-result/member identities, including filter rejections and rank omissions through observation routes. Existing bare CLI/scalar SQL/convenience outputs remain compatibility views without metadata. Typed C accessors expose IDs; old generic JSON envelopes retain compatibility, and an explicit additive identity-enabled complete route provides them. Update all strict new full-result readers coherently under 0442.

Reserve proxy.question_id as an optional opaque bounded string and proxy.code_threshold as the existing normalized number/band/null grammar. Derive the code threshold from current precedence/defaults; a caller cannot supply a contradictory second threshold. Reserve meta.proxy.decision_id and override tagged none/threshold/decision, carrying code_threshold, effective_threshold where required, and typed code_value/overridden value for a decision override. Exact bounds and function-specific reading fields are settled in the shared contract before code. These are logical reserved SDK shapes, not invented HTTP paths or headers. In 0.2 emit no proxy attestation and apply no override; activation refuses before lookup/send. Unknown vendor fields remain tolerated but cannot populate this namespace. In 0.3 an explicitly configured validated proxy protocol may override even an explicit code threshold; preserve raw probabilities, code reading and effective reading, and replay recorded policy without contacting today’s proxy. Deferral prevents unvalidated business decisions and vendor leakage while the reserved shape and stable IDs land now.

## Native work in progress

Lane0 has distinct validated CallId, SdkRequestId, ObservationId, FailureId and AnswerId leaves, with exact lowercase hexadecimal parsing and safe validation diagnostics. Typed constituent identity/source carriers reserve the closed provenance values. Transport generation, stable pure derivation, storage persistence, runtime propagation and proxy activation refusal remain open; no runtime completion is claimed.

Native WIP: accepted wire answers now receive one fresh ObservationId, stored outside question identity with their actual optional successful wire-question count. Cache/replay/coalesced occurrences retain it; equal refresh responses receive new observations. Legacy observation IDs use the verified original per-question key, typed answer and only existing model/count/time/origin metadata. Original recording/1 conversion derives IDs before generated time/origin, usage shares or quoting. Full logical AnswerId construction and all ten executable complete result routes remain in progress.
