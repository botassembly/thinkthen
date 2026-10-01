# Issue priorities, 2026-09-30

Status: current, rewritten 2026-09-30 late night by the issue priorities Quick Fix against main `91878c233`. It classifies every open issue, then lists the remaining batches in order. Git history holds the earlier batches B1 to B6 and the earlier full table. Ian can overturn any class, batch or default below.

## Landed since the last rewrite

Batches B1 to B6 are done: tickets 0345 to 0351 and both B6 Quick Fixes landed. Since then these landed and closed their issues:

- 0354: deep records name their 127-level limit, and `find` returns its probability at the C door (`11294af2d`). It closed the deep JSON issue and the find probability issue.
- 0357: `rank` returns each record's place and probability at the C door (`a275d64ed`).
- 0358: every row sums its usage by one rule (`46c79749d`).
- 0359: the C door's relate reads records through the shared parser (`33a8e0268`). It paid Debt 031.
- The file cap Quick Fix: the 500-line cap covers the bindings and extensions (`501b641e4`). It paid Debt 027.
- The pre-0.1 small fixes Quick Fix (`056a20b6c`) and 0355, the release workflow for every registry (`fedd2dd89`).

## Running

- Lane claude-4: ticket 0356, the binding tests wait on events. It pays Debt 029.
- Lane claude-2: ticket 0360, the usage totals move to the state folder.
- Ian's release rehearsal (ticket 0128 phase 3b) runs outside the lanes.

## Remaining batches

Each batch is one lane's work in one area of files. Landers take `CHANGELOG.md` and `sdlc/ratchet.json` one at a time and rebase.

| Order | Batch | Work | Starts |
| ---: | --- | --- | --- |
| 1 | C1 test servers and input pause | One ticket. Every Python loopback test server sends `Connection: close`: Debt 020's workaround list, found with `grep -rl BaseHTTPRequestHandler`. The 50 ms piped input pause becomes settable for tests (Debt 030). | when 0356 lands (Debt 029) |
| 2 | C2 spec lines | The lines in `2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md`: the "Portable call settings" row, "Dry runs" in `recording.md`, and the Rust cell that cites ticket 0291. | when 0360 lands |
| 3 | Checkpoint 2 | The full surface sweep on one main commit. The coordinator tags it `checkpoint/surfaces/2026-MM-DD-N` when every check passes. | after C1 and C2 land |

Work outside the lanes:

- **Marketing, owner of `site/`.** The reference page, the providers and Liquid d1 pages, parts 3 and 4 of the site replay issue, and the overhead benchmark for the README line.
- **Ian.** The release rehearsal, then phase 4 and the registry accounts.

## Every open issue

25 open. Class: **running**, **batch** (C1 or C2), **outside** (marketing, Ian, or another team), **waits** (a named trigger), **after 0.1**.

| Issue | Blocks 0.1 | Class | Owner or trigger |
| --- | --- | --- | --- |
| `2026-09-30-binding-tests-still-time-stops-and-wait-on-short-bounds.md` (Debt 029) | no | running | ticket 0356 |
| `2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md` (Debt 020) | no | batch | C1 for the test servers; the product fix waits on upstream |
| `2026-09-30-piped-batching-tests-race-the-50-ms-input-pause.md` (Debt 030) | no | batch | C1 |
| `2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md` | no | batch | C2 |
| `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | yes | outside | marketing |
| `2026-09-29-docs-page-naming-supported-providers.md` | the Liquid timeout line | outside | marketing |
| `2026-09-30-site-replay-folders-have-no-fixture.md` (Debt 007) | yes | outside | marketing, parts 3 and 4; parts 1 and 2 done |
| `2026-09-29-readme-key-backend-and-overhead-lines.md` | yes | outside | marketing's overhead benchmark, then one sentence from the queue owner |
| `2026-09-25-release-and-install-for-0-1.md` | it is 0.1 | outside | ticket 0128; Ian's rehearsal and registry accounts |
| `2026-09-20-new-user-stumble-register.md` | rows 18, 19 | outside | row 18 ticket 0128; row 19 marketing; none of ours |
| `2026-09-30-spec-no-calls-edges-need-a-real-send.md` | no | outside | the external release QA team's edge list |
| `2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md` (Debt 014) | no | waits | upstream ollama |
| `2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md` (Debt 002) | no | waits | upstream Zig |
| `2026-09-30-polars-door-cannot-test-lazy-streaming.md` (Debt 004) | no | waits | a user, or clean advisories |
| `2026-09-24-rank-by-graded-relevance-for-search-reranking.md` | no | waits | a user |
| `2026-09-25-public-library-api-gaps.md` (Debt 018) | no | after 0.1 | items 6 and 7 |
| `2026-09-25-docs-how-tos-and-spec-claims-owed.md` | no | after 0.1 | pages 12 to 24; page 23 marketing |
| `2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md` | no | after 0.1 | Ian thinks through the evaluation flow first |
| `2026-09-26-every-surface-should-give-back-run-facts.md` | no | after 0.1 | tickets 0300 and 0302 |
| `2026-09-25-recognize-and-relate-scale-and-shape.md` | no | after 0.1 | a relate ticket |
| `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md` | no | after 0.1 | coordinator default 2 |
| `2026-09-23-annotate-options-from-a-file-or-a-record.md` | no | after 0.1 | a ticket |
| `2026-09-30-batch-command-runs-many-questions-in-one-process.md` (idea) | no | after 0.1 | stays an idea |
| `2026-09-30-opentelemetry-traces-after-0-1.md` (idea) | no | after 0.1 | stays an idea |
| `2026-09-30-proxy-service-for-shared-limits-and-traces.md` (idea) | no | after 0.1 | stays an idea |

Closed by this Quick Fix: `closed/2026-09-30-site-fixtures-converted-and-plan-examples-moved.md`. Its leftover line, the Rust cell that cites ticket 0291, joined the C2 issue.

## Remaining tickets

- **0356** and **0360**: running in lanes claude-4 and claude-2.
- **0128**: phase 3b, the rehearsal, and phase 4, Ian's release run.
- **0334** slice 2, a `backend` setting in each binding and SQL extension, waits until after 0.1.
- **0295**, **0296**, **0300** and **0302**: after 0.1.

## Coordinator defaults

Taken under the workspace rule to record reviewed choices and proceed. Ian can overturn each one.

1. Both-ways relate edges carry a trailing `"either":true` member (0344).
2. Relation pairs keep the whole text for 0.1, with a distance limit as an opt-in later.
3. Homebrew stays a Mac option; the curl script covers Linux.
4. 0290 is withdrawn, and 0295 and 0296 wait until after 0.1.
5. `Engine::usage` stays per engine, and detailed rows carry `meta.batch_setting` (0347, 0349).
6. C1 starts after 0356 because both edit binding test fixtures. C2 starts after 0360 because both edit `specification/settings.md`.
