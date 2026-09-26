# The result schema identifier never versions

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 05, finding 2.4. Blocks 0.1: the compatibility rule must exist before the first release, because consumers pin to it from that day. Owner: ticket 0160 on `ticket/0160-the-answer-contract-holds`, ready for review.

## What happens

Every detailed row names `thinkthen.result/1`. Nothing says which changes keep `/1`.

- ADR 0036 renamed `meta.replayed` to `meta.cached` with no alias and kept `/1`. Its line 10 says "The schema identifier remains unchanged in this unreleased interface."
- `annotate`, `recognize` and `relate` emit different shapes under the same identifier.
- No JSON Schema for the result is published.

A consumer that validates on `schema` gets no warning when a field is renamed. It must sniff for `answers` against `answer` to learn the shape. Golden-file tests of output also break on additive fields (report 09, issue 9), which a stated rule would settle.

## Checked on main

Verified: ADR 0036 line 10 reads as quoted. The shape differences come from the report and the verb pages.

## What would fix it

Write a compatibility rule before 0.1. For example: additions keep `/1`, and a rename or removal bumps it. Give each aggregate shape its own identifier or a `shape` field. Consider publishing a JSON Schema for each shape. Batching changes what a row means (report 05, finding 2.6), so the batching tickets should apply the rule when they land.

## Done when

`specification/result.md` states the rule, each shape is named, and a check holds the identifier to the rule.
