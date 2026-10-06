# ADR 0120: SDK result and cache contract

- Status: **Accepted** contract under Ian's authorized 0.2 outcome; fresh High whole-contract review accepted; landed with 0442.
- Date: 2026-10-06
- Risk: **High**: storage compatibility, secrecy and result schema.
- Owner: [ticket 0442](../../tickets/0442-proxy-ready-sdk-contract.md).

## Context

The reviewed 0442/0450 tickets and bounded designer draft define shared metadata and storage before engine/C/host implementation. ADR 0119 settles one endpoint, effective key and provider API type per engine. Proxy routing, model groups, fallback, A/B policy, curation and automatic threshold tuning stay outside the SDK. Explicit literal models and deterministic caller readings remain compatible. No unknown reply field or hostname grants proxy authority.

## Decision

1. [Result/2](../../../specification/result.md#result2-target-for-02) coordinates answer identity, truthful provenance and 0436's rank-position change. Preserve five atomic answer variants and existing scalar/bare/generic C compatibility projections. New complete routes expose IDs without probability-details opt-in. The Rust-generated schema remains landed result/1 until owning serializers, shared corpus and complete-result decoders adopt result/2 together under ADR 0112.
2. Use distinct validated call, SDK request, observation, failure and answer ID types. Transport headers report the compiled engine version and explicit closed surface token. Calls receive fresh IDs, status retries retain request IDs, and split children get new IDs. Transient IDs enter neither keys, recording bodies nor count-only usage. Persistent observation IDs survive cache/replay; pure answer IDs include normalized reading/scope and ordered children.
3. [Cache/2](../../../specification/cache.md) frames adapter, canonical final URL, literal requested and reported models, shared state and wire question. Online mismatch reuse is forbidden; offline historical ambiguity refuses without a send. Ordered image bytes/media/version belong to state; paths do not. Mutable aliases that echo themselves still require caller freshness controls or provider no-store. No SDK discovery or routing index is added.
4. Verify original v1 identity and typed contents before normalization and rekeying. Convert writable stores atomically before lookup; replay builds its index in memory. Unknown/damaged/ambiguous stores refuse before sends. Preserve legacy request validation and committed offline fixtures. Support no old/new concurrent writers or downgrade promise and add no migration command or backup mechanism.
5. Honor bounded response Cache-Control conservatively for storage. Refresh sends no-cache, bypasses held answers and evicts old working answers after a successful nonstorable reply. Explicit recording fails locally rather than silently promising replay; it never retries that paid send. Answer bodies remain header-free.
6. [Recording timing](../../../specification/recording.md#optional-timing-history-for-02) is optional, bounded and timing-only. All ten functions retain opt-in attempts and final started-failure facts. CLI command time excludes the union of HTTP/body-read intervals. Missing provider time remains absent.
7. Reserve bounded proxy question/code-threshold/override types in result/2. Activation in 0.2, including empty/null input, refuses before store access, lookup or send. Vendor bytes exclude these fields and vendor replies cannot attest them. Execution waits for an admitted explicit 0.3 protocol.

## Two draft corrections

Legacy IDs hash only validated existing metadata. Original recording/1 envelopes need no missing timestamp or origin; do not hash synthetic converter values. Existing validated question-row history remains usable. See [the derivation and code evidence](../../../specification/cache.md#legacy-observation-identity).

A zero-observation aggregate has `origin:null`, `cached:false`, empty source/observation arrays and omitted `answered_by`, even in replay mode. Retain historical `model` only as compatibility metadata. A lone-entity relation can ask nothing, so neither a cache hit nor actual answered model exists. An empty actionable array after answers still reports its actual provenance. See [the exact metadata rules and code evidence](../../../specification/result.md#metadata-and-empty-aggregates).

These are bounded truthfulness corrections to the adopted draft; other settled choices remain intact.

## Adoption and compatibility

0443/0444/0445/0450 implement transport/storage/attempts/identity. 0436 adopts rank; 0426–0431 adopt carriers; 0410/0296 frames; 0435 SQL; 0411 rereading; 0432 parity; 0447/0448 image serialization/admission. 0426 owns remaining additive C signatures and lifetimes. 0449 preceded the reviewed 0442 landing. This ticket changes contracts only and claims no runtime completion.

## What Ian can overturn

Ian can overturn the result version/compatibility views, ID framing/types, cache normalization/migration, conservative storage parser/bounds, empty-result provenance and reserved proxy types through a reviewed amendment. Implementation generator details remain the implementation owner's responsibility. No new proof framework, receipt review or proxy protocol is authorized here.
