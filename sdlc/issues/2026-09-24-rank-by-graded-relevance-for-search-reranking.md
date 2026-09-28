# Rank by graded relevance for search reranking

Status: Open

Filed by the marketing session on 2026-09-24. Source: turbopuffer's post "Jev is a state-of-the-art search reranker", clipped by Ian.

## What the post does

A search engine returns 50 to 1,000 candidates for a query. For each candidate, the post asks Jev's score endpoint one question with ten relevance levels, from "unrelated" to "perfect match". It weights each level's probability into one relevance number and sorts by it. It reports Jev agreeing with a frontier model's order more often than two leading rerankers, at $0.0138 a query. turbopuffer also fuses rankings with reciprocal rank fusion (RRF).

## What ThinkThen does today

- `score` asks the same ten-level question per record, with a description per level from a question file. It weights levels by position 0 to K-1 and prints in input order.
- `rank` sorts records and cuts with `--top N`, but it sorts by one yes probability.
- Reranking the post's way takes three steps today: `score`, then a `jq` sort, then a cut. That is a function in a trench coat.

## Asks

1. `rank` accepts a graded question: levels with descriptions, sorted by the weighted value `score` already computes, with `--top N`. One command then covers the post's method.
2. Optional weights per level, if a user needs a scale other than 0 to K-1. Only if the first ask shows a need.
3. RRF is arithmetic over ranks and needs no model. It fits as a `transform`, such as `fuse`, which merges two ranked lists by id. It is not an eleventh function.

The vocabulary already names reranking as a `rank` use (`sdlc/planning/ten-use-cases.md` line 23). Ian can overturn all three.
