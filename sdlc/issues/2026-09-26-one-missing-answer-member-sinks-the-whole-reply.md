# One missing answer member sinks the whole reply

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 05, finding 2.3. Blocks 0.1: the code breaks a written adapter promise. Owner: ticket 0160 on `ticket/0160-the-answer-contract-holds`, ready for review.

## What happens

`specification/backends.md` line 100 promises per-question failure. Decode marks one logical question failed when its answer "lacks a probability", and `result.md` lists `missing_probability`. The code instead fails the whole reply at exit 4 when a `noul` answer has no `noul` member, or a choice or score answer has no `probabilities` member. An `annotate` row with four good answers and one malformed member returns nothing and stops the run. The spec says it returns exit 6 with one marker.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/core/adapters/systemone/response.rs:37-56` declares `noul` and `probabilities` as required fields of `ResponseAnswer`, so serde refuses the whole `Response` when one is missing. The exit 4 runs come from the report.

## What would fix it

Parse each answer member on its own. Map a missing field to `missing_probability` for that question, as the spec says. Add the missing-field cases to the partial-reply tests.

The duplicate-name issue (`2026-09-26-a-duplicate-answer-name-is-accepted-and-the-last-wins.md`) touches the same parse, so the two may share one fix.

## Done when

A reply with one answer missing its probability gives that question a failed marker, the other answers stand, and the run exits 6. A test pins it for `noul`, choice and score.
