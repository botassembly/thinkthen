# Batching design review before 0146

Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review. The full report, with commands, outputs and a live ledger of 16 calls, is on this machine at `~/workspace/experiments/273-architect-review/02-batching.md`. Most findings are about the design in `2026-09-26-batching-design.md` and ticket 0146, because main holds only the planner (0144).

## Severity 1

1. **A request-and-reply loop hangs when a cache or recording folder is named.** The README recommends this pattern: one record in, wait for its answer, then send the next.
2. **One record can steer its neighbours.** Live, one planted claim in a shared request moved three other titles by up to 0.46 and pushed one over the cut. The design has no threat analysis for records that share a request.
3. **The default doubles false yeses.** Live, all 306 titles in one request scored 272 to 274 right, with 32 to 34 false yeses. One record per request scored 285, with 16 to 17. The design quotes 276 to 278 right and 27 to 29.

## Severity 2

4. A cached partial reply fails the same record on every rerun.
5. The 96,000-byte cap does not keep dense text under the token limit. Live, a 94,238-byte batch of two hex records was refused, while each record alone passed.
6. A stream with few distinct values forms one batch that never closes, so memory grows without bound.
7. Thresholds tuned before batching run under the new default with no warning.
8. Other addresses have no request-size bound until 0154 lands.
9. The promise to refuse an oversized context before any request cannot hold.
10. The command and the libraries give different answers for the same input.

## Ask

Read the report before 0146 builds. Say which findings change the design, and which you accept as documented costs. Marketing states every accepted cost on the speed-and-cost slide and page.
