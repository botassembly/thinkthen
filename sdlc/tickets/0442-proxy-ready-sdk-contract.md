# 0442: Settle SDK identity, provenance and cache compatibility

Status: in progress. Contract built; fresh High contract review and landing remain with the coordinator. Engine/C/host adoption follows separately.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

One reviewed contract defines the SDK metadata, proxy-compatible transport identity and versioned cache/replay migration used by all surfaces.

## Evidence

- Starts from: Branch base origin/main f22c092ca after 0433; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 9 and 10.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Record the chosen public schema/headers, origin and concrete model rules, cache-control policy, key formula and offline migration before implementation. Update result/settings/recording/cache specifications and ADRs together.
- Proof: Review invariants against existing cache/replay, six errors, retry/split accounting and secrecy. Use saved examples to show mixed origins/models, literal requested/reported-model lookup, replay ambiguity, no-store recording refusal and safe v1 conversion. No runtime migration or network request during design.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Precedes 0443/0444/0445 and final typed/schema adoption. 0300 supplies prices; 0435 owns SQL invocation facts. Contract additions must be adopted by 0426–0431 and 0432.

## Contract decisions

[ADR 0120](../planning/adr/0120-sdk-result-and-cache-contract.md) adopts the bounded designer draft and reviewed 0450 shape. [result.md](../../specification/result.md#result2-target-for-02) fixes all ten result/2 variants, exact metadata and distinct IDs, headers/surface tokens, attempt/failure facts and inactive proxy types. [cache.md](../../specification/cache.md) fixes framed v2 keys, URL/model freshness, parser bounds/no-store/refresh and transactional offline validation/migration. [recording.md](../../specification/recording.md#optional-timing-history-for-02) fixes timing-only sidecar bounds; settings and changelog distinguish the target from landed behavior. The generated schema stays with landed serializers under ADR 0112 until implementation adopts result/2.

Two bounded draft corrections: legacy identity includes only validated existing metadata, with no required or invented missing timestamp/origin; a zero-observation aggregate uses null origin, `cached:false` and omitted `answered_by`. The contracts name actual converter and lone-entity relation evidence and explain both choices. Empty actionable output after observations still reports its real sources.

Risk: **High**, storage compatibility, secrecy and result schema. No new proof framework. Preserve six errors, zero-send replay, scalar projections, existing answer variants and reserved activation's pre-store zero-send refusal. 0449's fresh reviewed single-route contract precedes landing; the coordinator merges final main and owns fresh High review, full landing gates and the single landing record. This branch does not land or qualify runtime adoption.

Adoption: 0443 transport/provenance/cache instructions; 0444 store/legacy identity; 0445 attempts/timing; 0450 stable IDs/reservations; 0436 rank; 0426–0431 typed carriers; 0410/0296 frames; 0435 SQL; 0411 rereading; 0432 parity; 0447/0448 image serialization/admission. Remaining cross-slice signature dependency: 0426's exact additive C exports and handle/string lifetimes.
