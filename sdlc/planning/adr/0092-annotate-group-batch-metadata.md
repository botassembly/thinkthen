# ADR 0092: Annotate names each group batch in a detailed row

- Status: **Proposed** for ticket 0216 design review and an explicit owner ruling. No public metadata change is accepted or implemented.
- Date: 2026-09-27

## Gap

ADRs 0048 item 9 and 0083 item 5 describe a singular `meta.batch`. An annotate record can have several normalized `on` groups, and an explicit profile can split one group's questions into contiguous chunks. Under B10 those groups or chunks may close record batches at different boundaries. One row can therefore receive answers from several batches with different `records`, `position`, `closed`, usage and attempts. `specification/result.md` and `core/result.rs::AnnotateMeta` currently name only `meta.requests` for annotate. Picking one contributing batch as `meta.batch` would misreport the rest; forcing all groups to close together would abandon B10's fill-to-limit behavior for a lighter group.

## Proposed decision

1. Keep the existing singular `meta.batch` object for verbs whose row has one batched request. Add optional `meta.batches` **only** to detailed annotate rows that include at least one multi-record or split chunk. It is an array with one entry for each contributing request, in exactly `meta.requests` order. At `--batch 1`, and when every contributing chunk is an ordinary singleton, omit it so current detailed bytes remain unchanged.
2. Each entry is an object with `group` (one-based ordinal of its normalized `on` group, in first-appearance question-set order), `request` (that entry's exact request digest) and the existing `BatchMeta` fields: `setting`, `records`, one-based `position`, `closed`, optional whole-request `usage`, `requests_sent`, and `split:true` only for a half of a refused batch. Entries for ordinary singleton chunks are included when another chunk makes the array present, so it aligns one for one with `meta.requests`. Equal digests in different groups keep distinct entries and positions. A profile's contiguous question chunks within one group keep their existing request order.
3. The per-row `meta.usage` and `meta.requests_sent` remain the checked sums of this row's even shares from all contributing requests, with the earliest members receiving remainders. Missing usage in any contributing request omits the row's usage rather than inventing zero. `cached` is true only if all contributing requests came from storage. Each named `answers` entry still carries its producing request digest; a failed entry keeps `question` and `failure`, with no false `value`.
4. This amends ADR 0048 item 9 and ADR 0083 item 5 **for annotate only**. It is additive under `thinkthen.result/1`; it does not change the type or meaning of `meta.batch`. `specification/result.md` and `spec/annotate.md` gain the form when B10 implements it. The resolved question-set digest and request digests do not include this presentation metadata.

## Why this proposal

A row-wide singular `records` or `position` cannot truthfully describe two groups with different batch boundaries. A parallel `meta.batches` array keeps request identity and field provenance inspectable without making one group block another from filling. It also keeps the historical batch-one row byte-identical. This is a new outward spelling beyond the accepted B10 behavior, so ticket 0216 records it as proposed until review and the owner's decision. Ian can overturn it.
