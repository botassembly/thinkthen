# ADR 0090: Choose batches preserve each record's options

- Status: Accepted by fresh independent design review of ticket [0213](../../tickets/0213-choose-record-batching.md) at `02e60bc6`. This ADR changes no runtime behavior until built. Ian can overturn it.
- Date: 2026-09-27

## Problem

[ADR 0048 item 1](0048-records-batch-into-full-requests.md) says equal evidence inside a batch is asked once, and each copy receives that answer. `choose --options POINTER` already takes its ordered candidate labels and descriptions from each whole JSON record. `--field /note` can select equal evidence from two records whose `/codes` differ. Main `1e771675` `cli/asking.rs::Asks::FromRecord` builds a separate `Question::Choose` for each row, while `core/batch.rs::Batcher` currently deduplicates by the selected record JSON alone and keeps one fixed question. Applying that key to `choose` would give one row an answer to the other row's candidate list. The fixed-options [batch-choose fixture](../../../specification/fixtures/systemone/batch-choose.request.json) does not exercise this collision.

## Decision

Amend ADR 0048 item 1's equal-evidence sentence for a verb whose complete question can vary by record: an answer is shared only when the selected record's compact JSON bytes **and the complete resolved question** are equal. For `choose`, question equality includes the text, the ordered label names and each optional description. Two equal selected records with different options occupy two positions in the no-context `{"records":[…]}` evidence and two separately named wire questions. Their answers may differ. Repeating both the same selected record and the same resolved question remains one wire question whose answer is copied to each logical member. A context batch likewise asks both differing questions over one shared context.

The SHA-256 content cut remains a function of the selected record's compact JSON alone, with the first eight bytes read big-endian modulo 4,096, as ADR 0048 item 2 requires. Candidate-list changes alone do not move the cut. Batch size counts logical members, including repeated pairs, while question/profile limits and request bytes count the body actually encoded from distinct pairs. No-context `--batch 1` keeps today's exact request body and digest, because its one row uses that row's original complete question and evidence. Context changes the request digest and raw context hash, not the question digest. A changed option list changes the question digest and the encoded request identity. The limit, 413 halving and replay paths must retain every row's original question when rebuilding a batch.

## Why this is narrow

ADR 0048's duplicate optimization remains unchanged for `decide`, `filter` and `rank`, whose run has one fixed question. The new equality only distinguishes a question that the existing `choose --options` feature already permits to vary. It does not change the default maximal setting, option validation, threshold/tie/null semantics, content cuts, record order, attempt facts, or other host APIs. B12a's proposed public `choose_many` with one typed fixed question can use the original fast path; its public return shape is under separate 0212 review and is not decided here.

## Proof before landing

One outside-in loopback case supplies three JSONL rows with equal selected `/note`: the first two have differing ordered `/codes`, while the third repeats the first complete pair. It expects two wire questions and two equal entries in the no-context evidence list, three ordered result rows, the corresponding per-row question digests, and one batch request digest. A no-context singleton compared to the existing choose fixture proves old recording bytes. A 413 half with differing candidate lists proves reconstruction does not replace either row's options. The ticket's context and refusal cases cover the same route after reviewed 0172. No product source or paid call is part of this ADR draft.

## What Ian can overturn

Ian may choose to stop deduplicating `choose` records altogether or change the batch evidence form. Either changes the cost/identity rule and needs a revised accepted decision and matching exact-wire proof. This decision retains sharing only for identical record/question pairs.
