# 0223 design review

Status: pending fresh independent design review. Review the proposed [ticket](../tickets/0223-graded-relevance-reranking.md), [ADR 0095](../planning/adr/0095-rank-graded-score-questions.md) and [source preflight](0223-reranking-preflight.md) against main `81b145ed`. No runtime or settled page has changed, and this note records no acceptance.

The reviewer should decide whether the command-only `rank @score-file` form meets the issue's described-level use without making `Ranked::probability()` dishonest. Check that the score-shaped detailed row's numeric `value` is the same rounded number that sorts, while a yes/no rank row keeps null value. Confirm that score batch requests and digest identity are compared with `score` only under the same explicit framing and stable boundaries. Check the deliberate refusals of typed yes/no meanings with a score file, the unchanged `--threshold`/wrong-kind diagnostics, and 0220's top-N/failure bounds. Optional weights, positional levels, RRF and wider host APIs remain visible deferred asks, not silently closed work.

## What the build taught us

Preparation gave the reviewer a finite, outward-facing decision and an exact four-row proof table. It also named the existing 500-line headroom and the helper constructors that a field rename would reach. A fresh reviewer must correct any scope or parity overclaim before runtime implementation; no build result is asserted here.
