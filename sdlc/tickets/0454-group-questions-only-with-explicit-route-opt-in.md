# 0454: Store actual batch size and preserve per-question caching

Status: COMPLETE. Reviewed implementation and adoption pass the full 29-consumer installed campaign at 60f0dcb9a. See [0432 qualification](../records/0432-shared-parity-cases.md#final-installed-qualification-2026-10-08). Final platform, release QA and publication remain under 0425.

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

## Native work in progress

Complete question-source carriers expose optional batch_size: NonZeroU32 as JSON integer and Option<u32> through the typed accessor. Historical absence stays absent. No batch count, group membership or forced scalar policy enters an identity. The superseded group-key WIP was removed before committing. Annotation clauses now retain per-question reuse and distinguish evidence selection, record-batch bounds and actual wire-question count. Persistence, retry/split/coalescing propagation and the 13/5/8 overlap regression remain open.

Native WIP: public/CLI native_store overlap regression independently expects A–M, N–R and S–Z wire bodies (13/5/8 missing questions), followed by counted zero-send replay. Stored observations retain their original batch_size and IDs; cache coalescing does not change counts, and missing legacy counts remain absent. Batch size does not enter either question-key/2 or legacy observation identity. Whole native integration/review and final adoption remain open.

Native foundation update: successful observations persist actual wire-question counts outside identity, with cache/replay/coalesced reuse retaining original counts, retry counts unchanged and accepted split children carrying their actual counts. Legacy absence remains absent. The existing overlap regression independently sends 13/5/8 missing questions and checks zero-send replay/key rebuild. Annotation contracts retain per-question reuse and distinguish record bounds from wire grouping. Native/CLI execution is implemented; host adoption and root whole review/landing remain open.
