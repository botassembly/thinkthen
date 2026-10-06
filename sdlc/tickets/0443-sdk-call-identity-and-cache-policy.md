# 0443: Carry SDK call identity and cache instructions

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Every request identifies its actual engine version and public surface; every call/request has a unique opaque ID. Results state actual retrieval origin/model and obey response cache instructions.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 9.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement reviewed 0442 headers/metadata in the existing engine edge. One call ID covers packed/multistep requests; retries retain request IDs, split children get new IDs. Propagate truthful surface labels through the C/native door. No-store forbids cache/record writes; refresh sends no-cache and bypasses old answers.
- Proof: Loopback packed, multistep, retry, split and concurrent calls pin IDs, labels and unchanged bodies. Unknown reply fields/headers remain tolerated; malformed known data fails. Check mixed cache/live sources, replay zero sends, actual answered model, no-store eviction and explicit-record refusal with started facts and no retry. IDs expose no caller text/key/address/model and do not enter cache keys or recording bodies.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 contract first; family tickets own public host metadata/settings adapters and typed decoding. 0444 owns key/store behavior; 0445 owns attempt observations. Preserve actual sends/retry accounting even if a future proxy deduplicates bills.

## Revised SDK boundary and identity

0449 resolves one endpoint/key/API-type before work. Direct model selectors stay opaque explicit parameters; no model group/fallback/automatic policy routing. Coordinate 0450 observation/answer identity via the existing ID facility. Transient call/request IDs remain absent from answer recordings; persistent observation identity is separate allowed record metadata under 0444. Reserved proxy fields never enter ordinary vendor bytes, and direct replies cannot attest them.

Native lane0 WIP: concrete typed result/2 carriers and serializers now cover
the six atomic readings, annotation successes/failures and whole-set find;
record carriers retain caller originals without extra trait bounds. Pure
serializer checks preserve explicit legacy result/1 projection, successful
null versus failure, numeric rank positions and zero-observation metadata.
These are constituent types, not completed public execution routes.
The actual landed image source is merged. Native started calls now carry
one opaque call ID through success/failure facts; prepared sends carry distinct
SDK request IDs and compiled version/surface headers. Status retries retain
those IDs, split children get new request IDs, and CLI final facts use the
same call ID. Seven public/CLI loopback cases independently pin bodies,
headers, concurrency, retries, splits, empty aggregates and zero-send refusal.
Legacy serialized facts remain unchanged. Canonical facts serialization,
persistent observations, cache policy and complete execution routes remain open.

Native storage-policy WIP: bounded Cache-Control parsing returns only storage
permission; repeated fields, exact no-store names, quoted values, malformed
syntax and each parser bound are covered. The existing send/transaction spine
returns nonstorable cached calls without writing, rejects explicit recording
locally after the successful send, and evicts working answers transactionally
only on valid nonstorable refresh. Native EngineBuilder.refresh_cache(bool)
uses the existing freshness mode; retries and split children send no-cache.
Eight outside-in public/CLI policy cases include prior-failing regressions.
A partial nonstorable refresh evicts only accepted questions; failed-question
history remains held for a later invocation.
V2 validated migration, provenance and complete runtime results remain open.

The affected settings check also exposed missing image/media inventory cells
in the merged image source. The existing admitted flags now have their row;
this does not add a route or change image admission.

The transport accepts the explicit closed mcp surface for approved0455 and
uses the same compiled version/call/request headers. MCP module/CLI ownership
remains lane1; no alternate transport or engine is introduced.

Local image integration WIP: ready 92084c66d is merged with shared profile propagation. The estimated-budget blanket refusal is omitted under Ian's correction: uncalibrated local images retain approximate encoded-body admission. The original Imajev decoder accepts independently optional reported counts, carries input887/output unknown through per-question storage and zero-send replay, and retains original request/response bodies in the explicit recording transaction. All 23 focused image regressions pass. Native result execution, migration, CLI schema adoption and the whole High review remain open.
