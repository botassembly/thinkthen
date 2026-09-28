# ADR 0087: A shared context stops at the record that overflows

Status: accepted by ticket 0172. Date: 2026-09-27. Amends ADR 0048 item 11.

## Context

`--context FILE` makes one text the evidence of every batch request. ADR 0048 item 11 promised refusal before any request when that context and one record exceed a limit. A stream can have already sent earlier batches before a later record proves too large. Reading the stream ahead would break live pipes and the bounded input behavior.

## Decision

1. Before any request, refuse at exit 2 when the context, question and no record exceed the resolved request size or a profile limit. The message names the binding limit and echoes no context or record text.
2. Check each record when the reader reaches it. Close and send any earlier open batch before refusing a record that cannot fit beside the context. Stop at that record with exit 2; send nothing for it. Earlier batches may have been sent. `decide` and `filter` print completed rows; `rank` withholds them on a stop to preserve ordering.
3. Keep ADR 0051's one halving when the backend refuses an already sent batch as too large. A refused half fails at exit 4 under that rule.
4. Do not read ahead to predict a later record's size. A stream can be endless; the reader's 50 ms pause and memory bound still apply.

The context is sent once per request and enters the request digest. It stays out of the question digest. Ian can overturn this decision.
