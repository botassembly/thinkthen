# A duplicate answer name is accepted, and the last one wins

Status: Closed on 2026-09-26. Done by ticket 0160 on 2026-09-26: a reply that names an answer or a label twice is refused, and a test pins each case. Filed 2026-09-26 by the queue owner from local experiment 273, report 06, finding I-4.

## What happens

A reply that names `q1` twice, first with `noul` 0.01 and then with 0.9, is accepted. `decide` prints `true` at exit 0, and `check` passes. Another JSON reader, such as `jq` over the saved entry, may pick the other value. Question files refuse duplicate names. Replies do not.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/core/adapters/systemone/response.rs:25` reads `answers` into a `BTreeMap` with default serde, which keeps the last duplicate. Choice and score `probabilities` use the same map type. The report did not reproduce the `probabilities` case, so that half is not verified.

## What would fix it

Refuse a duplicate member name in `answers` and in each `probabilities` map. Add a `check` fixture for it. The missing-member issue (`2026-09-26-one-missing-answer-member-sinks-the-whole-reply.md`) changes the same parse, so the two may share one fix.

## Done when

A reply with a duplicate answer name or a duplicate label is refused, and a test pins each case.
