# 0454: Store actual batch size and preserve per-question caching

Status: open. Ian's corrected ruling supersedes the earlier grouping-key proposal; focused ticket review follows.

Milestone: 0.2
Owner: builder.
Risk: High, stored-observation compatibility and truthful answer metadata.

## Outcome

Keep batching configurable at any admitted size and keep the cache per question, regardless of batch. Each stored decision records the actual size of the request that produced it so callers can measure grouping effects. Batch size never enters the cache key.

## Evidence

- Starts from: PM message `2026-10-06-pm-0036-found-that-grouping-questions-in-one-request-changes-answers.md`, corrected at shared mailroom commit 128dff2 on 2026-10-06 with Ian's ruling. It replaces both original asks. Experiment 0036 at thinkthen-exp branch commit 7ed783c8 found changed receipt answers under grouping on two local routes; its small unchanged Imajev cohort does not establish general equivalence. Those findings inform documentation and observation metadata, not a new cache namespace or forced scalar default.
- Keeps: Current caller-controlled batching and precedence, the default of eight concurrent requests, existing vendor/profile limits, one endpoint/key/API type, independent per-question cache/in-flight identity, missing-piece requests, six errors, cancellation, secrecy, spending controls, atomic migration and zero-send replay. Explicit image order and original evidence stay intact.
- Changes: Persist optional actual batch size on each accepted stored observation and expose it through complete typed result metadata. Define batch size as the SDK's actual wire-question count in the successful request, not a requested maximum or guessed provider-internal batch. Retried requests retain the observed count; accepted split children carry their own counts. Cached/replayed/coalesced results retain the original observation's count. Historical absence remains unknown, never invented as one. Keep this metadata outside every key/observation-identity derivation during conversion.
- Proof: Request batches A–M, G–R, then O–Z plus A–D on the same route/model/evidence. Every question is sent once; cached portions are reused and only missing pieces are requested. Independent expected arrivals and bodies prove reuse, not an internal counter alone. Pin original observed sizes 13, 5 and 8 where the fixture's limits admit those requests. Add retry/split counts, read-only legacy conversion with absent size, stable key rebuild across batch changes, zero-send replay and complete typed metadata checks. Reuse saved/loopback exchanges; no live equality benchmark is required.
- Defers: Accuracy guarantees, automatic route grouping policy, capability discovery, forced scalar defaults, group-key namespaces, new paid cohorts and model rankings.

## Contract and ownership

ADR 0120's exclusion of packing from key identity remains in force. 0444 implements its original per-question key/store migration; do not add grouped/legacy-unspecified key namespaces or bypass per-question cache reuse because a caller changes batching. 0443/0445/0450 propagate truthful observation metadata and stable identities. This ticket's former group-membership key proposal is withdrawn by Ian's newer ruling.

Reconcile annotate.md's contradictory exact-chunk/whole-group reasking clause with the retained per-question reuse promise. Logical evidence selection, record batching and actual wire membership are separate concepts. A changed group alone does not invalidate already stored individual questions. Document that grouping may move model answers and that reuse returns the recorded observation rather than guaranteeing fresh scalar equivalence.

Implement in the same native lane, then expose the field through 0426–0431, SQL and frames. 0432 enforces the actual public consumers. Record the exact bounded field/grammar with code, one fresh High code review of the native change, and one short record at landing. This ticket remains open until required carriers and behavior are checked. Both corrected PM asks receive this ticket and ruling.
