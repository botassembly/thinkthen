# ADR 0040: Split requests under backend limits

- Status: Accepted for ticket 0079
- Date: 2026-09-23

This amends ADR 0032 without changing its text. Ian can overturn the new limit or the splitting rule.

## Decision

The closed `thinkthen.backend-profile/1` object may carry `max_options`, an optional positive integer. It limits the options in one `choose` question. An exact limit passes. A choice one option over fails local preflight because splitting its options would change the question. The profile still selects no backend setting and estimates no tokens.

An ordered multi-question plan is split into the fewest contiguous requests that satisfy the profile's exact encoded request-byte and expanded-question limits. Each chunk repeats the evidence and restarts wire question names at `q1`. The longest fitting next prefix wins. Evidence, one-question request, and one-choice option failures remain unsplittable.

The engine encodes, digests, and preflights every chunk before replay, cache access, key access, or the first send. A plan that fits remains one request with its historical bytes and digest. Chunk execution uses the existing replay, cache, retry, accounting, recording, and cancellation path.

Results merge in logical plan order. Every reply must report one model. Usage is present only when every chunk reports it and is then the checked sum. Request sends include every retry. A result is cached only when every chunk replayed. Ordered request metadata and each answer's producing digest retain the chunk identities.

## Consequences

Splitting is a correctness boundary, not generic packing. Records, evidence text, options, and unrelated calls are never combined or divided. (Amended by ADR 0048, below.) The repository ships no Jev byte limit until a measurement supports one. Probes accepted choices with 101 and 255 options, so the stale 100-option claim is withdrawn while the tool's existing 255-option ceiling remains.

## Amendment, 2026-09-25, ticket 0123

A measurement now supports one hosted byte ceiling. The hosted backend refused a relate request over 65,536 input tokens with status 400 and `max_tokens_exceeded`. Eight live calls on 2026-09-24 (commit `10d425e5`) found 142 `appears_on` questions over the Beatles set passing at 65,423 input tokens and 143 failing. The 142-question request encodes offline to 126,820 bytes, so relate's JSON runs 0.516 input tokens a byte. Relation plans at the built-in address now split under 96,000 request bytes, about 49,500 tokens. The ceiling splits and never refuses. A profile's `max_request_bytes` replaces it. Other plans and other addresses keep no ceiling. The accuracy size of a request belongs to the packing ADR. (Amended by ADR 0048, below.) The splitter now finds the longest fitting prefix by doubling and then halving the gap. It picks the same chunks. Ian can overturn the number and its reach.

## Amendment, 2026-09-26: ADR 0048 batches records

ADR 0048 is the packing ADR line 24 names. Records combine into batches by its rules, and evidence text, options and unrelated calls still never combine. A batched record plan at the built-in address also closes at the 96,000-byte ceiling, and a profile's limits close batches everywhere. Ian can overturn this.
