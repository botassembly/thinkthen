# 0454: Make question grouping explicit and keep grouped answers distinct

Status: ready. Fresh High ticket review accepted; implementation joins the native lane.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only High review.
Risk: High, answer correctness and cache/replay identity.

## Outcome

Routes without established grouped/single equivalence ask one wire question per request by default. Explicit grouping is a caller choice for the engine's one resolved route. A grouped observation never silently supplies a single-question answer or another group.

## Evidence

- Starts from: PM message `2026-10-06-pm-0036-found-that-grouping-questions-in-one-request-changes-answers.md`, asks 1 and 2. Experiment 0036 report at thinkthen-exp branch commit 7ed783c8 records Clef losing 7 of 40 matched receipt decisions under grouping, Flash losing 2 of 40, and Imajev changing none in that small cohort. A scalar calibration claim does not establish grouped behavior. The new result/cache contract is ADR 0120 under 0442; its current exclusion of packing needs this deliberate amendment.
- Keeps: One endpoint/key/API type, eight concurrent requests, original evidence and image order, explicit caller settings, six errors, cancellation, spending controls, truthful observations, header-free stores, atomic migration and zero-send replay. Multiple images explicitly attached to one question remain one admitted input; do not confuse that with automatically grouping questions.
- Changes: Default to separate wire questions on all currently unverified routes. Keep explicit grouping available without silently treating an inherited default as opt-in. Distinguish record batching from wire-question grouping, including multiple annotation members and both-sides probes. Reuse existing packing and split/retry machinery. No capability discovery or business routing table is added.
- Proof: Count requests and inspect independently expected bodies for separate defaults and explicit groups through named public/CLI calls, including annotation and probes. Use saved responses that deliberately differ by grouping. Keep each complete group's ordered membership in identity; pin different groups, order, scalar/group separation, partial replies, retries and split children. Strict replay sends zero; incompatible group lookups refuse rather than falling back to a different observation. No new live comparison or benchmark is required.
- Defers: Model accuracy guarantees, automatic grouping enablement, route discovery, proxy policy, new paid cohorts and public rankings.

## Cache and replay amendment

0444 owns the versioned key/store adoption in the native lane. Add execution semantics and complete ordered question-group constituents to grouped identity. Capture the actual group that produced an accepted observation. Retries retain the same identity; changed split-child groups remain distinct. Transient request IDs, surface, filenames and timings still stay out. Store only the bounded metadata needed to reconstruct the key offline through the existing store, without a second cache or evidence system.

Grouped lookup cannot remove a cached member and then silently change the remaining group's context. Until equivalence is established, bypassing grouped local reuse is preferable to claiming per-question equivalence. Explicit record/replay must retain enough membership to reproduce the observed group identity and distinguish it from a scalar observation.

Do not infer missing historical grouping facts during v1 conversion. Preserve original rows and deterministic observation IDs; keep historical/unspecified observations in a separate compatibility namespace and identify them truthfully on explicit historical replay. They cannot become an online scalar or grouped cache hit by assumption. Refuse unknown/damaged/conflicting migration locally before a send. Update ADR 0120, the key formula, schema and examples together with implementation. Reconcile annotate.md's conflicting per-question and exact-chunk reuse clauses. Supersede ADR 0111's affected grouping/reuse promises explicitly; distinguish logical evidence selection, record batching and actual wire membership. Preserve explicit old replay compatibility without attesting equivalence it never measured.

## Ownership and completion

Integrate with 0443/0444/0445/0450 and native input semantics in one lane. 0426–0431, SQL and frames expose the shared setting and result semantics; 0432 enforces the required cases. Record the exact additive API details with implementation after review. One code review covers the coherent native change; one short record at landing. This ticket remains open until native and required surface behavior are checked. The PM's two asks receive this ticket number and the accepted ruling.
