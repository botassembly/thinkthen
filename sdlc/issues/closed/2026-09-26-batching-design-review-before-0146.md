# Batching design review before 0146

Status: closed 2026-09-30. Replaced by ADR 0111 (one question cache and one batching path).

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

## Verdicts

1. Changes the design. The pause fires in every mode (ADR 0053 item 1). Ticket 0146 builds it.
2. Changes the pages and accepts the cost. The default stays. `records.md`, `decide.md` and D1 state the trust model, and `--batch 1` is the defence (ADR 0053 item 5). Design test 9 adds a planted arm.
3. Accepted as a documented cost. The default stays `max` by Ian's ruling. The quoted cost becomes the tool's own form, 272 to 274 right and 32 to 34 false yeses (ADR 0053 item 4, section 14 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`).
4. Changes the design. A cache keeps no reply that failed a question (ADR 0053 item 6). Ticket 0158 builds it before 0146 lands.
5. Accepted as a documented cost. The 96,000-byte default stays. ADR 0051's halving is the remedy. The documents stop calling 0.516 tokens a byte the worst rate.
6. Changes the design. A batch closes at 4,096 members (ADR 0053 item 2). Ticket 0146 builds it.
7. Changes the design. A file with a threshold and no `batch` counts as tuned at batch 1 for the warning (ADR 0053 item 3). B16 builds it before any release.
8. Already covered by ADR 0051 item 1 and ticket 0154. Ticket 0154 lands before any release.
9. Already covered by ticket 0144 deferred gap 5. B7 picks the policy and rewrites ADR 0048 item 11.
10. Accepted as a documented cost, and covered by ticket 0146 deferred gap 7. `settings.md` states it.

Marketing states the costs of findings 2, 3, 5 and 10 on the speed-and-cost page.
