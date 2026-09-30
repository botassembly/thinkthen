# Rank by graded relevance for search reranking

Status: partially complete. Graded command ranking and its normative pages are shipped; optional custom weights and reciprocal-rank fusion remain open.

Filed by the marketing session on 2026-09-24. Source: turbopuffer's post "Jev is a state-of-the-art search reranker", clipped by Ian.

## What the post does

A search engine returns 50 to 1,000 candidates for a query. For each candidate, the post asks Jev's score endpoint one question with ten relevance levels, from "unrelated" to "perfect match". It weights each level's probability into one relevance number and sorts by it. It reports Jev agreeing with a frontier model's order more often than two leading rerankers, at $0.0138 a query. turbopuffer also fuses rankings with reciprocal rank fusion (RRF).

## Original gap, 2026-09-24

- `score` asks the same ten-level question per record, with a description per level from a question file. It weights levels by position 0 to K-1 and prints in input order.
- `rank` sorts records and cuts with `--top N`, but it sorts by one yes probability.
- Reranking the post's way takes three steps today: `score`, then a `jq` sort, then a cut. That is a function in a trench coat.

## Asks

1. `rank` accepts a graded question: levels with descriptions, sorted by the weighted value `score` already computes, with `--top N`. One command then covers the post's method.
2. Optional weights per level, if a user needs a scale other than 0 to K-1. Only if the first ask shows a need.
3. RRF is arithmetic over ranks and needs no model. It fits as a `transform`, such as `fuse`, which merges two ranked lists by id. It is not an eleventh function.

The vocabulary already names reranking as a `rank` use (`sdlc/planning/ten-use-cases.md` line 23). Ian can overturn all three.


## Graded command slice landed

Ticket 0223 passed fresh independent code review at `0e242567` and integration checks at `96e1058d`. The command now accepts `rank @score-file`, orders by the existing weighted score with stable top ties, retains score details and counts every judged record. Ordinary yes/no rank and library rank APIs retain their behavior. The build and review records identify exact request, digest, output and failure proof. Optional per-level weights and reciprocal rank fusion remain open, and public documentation is held for marketing. This source issue is not closed by the first slice.

## Command documentation completed

Quick Fix `782b0aea` passed fresh Medium review and completes the five-page 0223 handoff, command index and root table. The executable result consumer keeps exact ordinary and graded-rank shapes. The [copied-site reference issue](closed/2026-09-28-site-reference-misses-graded-rank-and-cache-byte-examples.md) belongs to marketing. This does not implement optional weights or RRF.
