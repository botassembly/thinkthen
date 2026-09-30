# Rank by graded relevance: custom weights and rank fusion

Status: open for asks 2 and 3. Shortened 2026-09-30. Ask 1 shipped: ticket 0223 (`0e242567`) lets `rank @score-file` order records by the weighted score `score` already computes, with `--top N`, and Quick Fix `782b0aea` documented it (`specification/rank.md`). Ian can overturn both remaining asks.

Priority: ranked in `../planning/issue-priorities-2026-09-30.md`. Owner: none until a user asks.

Filed by the marketing session on 2026-09-24 from turbopuffer's post "Jev is a state-of-the-art search reranker". The post weights ten relevance levels into one number, sorts by it, and fuses rankings with reciprocal rank fusion (RRF).

## Remaining asks

2. **Optional weights per level.** `rank` and `score` weight levels by position 0 to K-1. Add per-level weights only if a user needs another scale.
3. **RRF as a transform.** RRF is arithmetic over ranks and needs no model. It fits as a `transform`, such as `fuse`, which merges two ranked lists by id. It is not an eleventh function.
