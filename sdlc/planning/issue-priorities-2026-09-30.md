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
| 1 | C1 test servers and input pause | One ticket. Every Python loopback test server sends `Connection: close`: Debt 020's workaround list, found with `grep -rl BaseHTTPRequestHandler`. The 50 ms piped input pause becomes settable for tests (Debt 030). | landed by ticket 0361 |
| 2 | C2 spec lines | The lines in `2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md`: the "Portable call settings" row, "Dry runs" in `recording.md`, the Rust cell that cites ticket 0291, the settings table's wording for a site reader (item 4), and the `cache_answers` sentences in `recording.md` and `result.md` (item 5). Files: `specification/settings.md`, `specification/recording.md`, `specification/result.md`. Medium. | when 0360 lands |
| 3 | C3 small command fixes | One ticket, four small issues from the mailroom triage of 2026-09-30. `2026-09-30-check-ignores-the-estimated-token-cap.md` (blocks 0.1): `crates/thinkthen/src/cli/check.rs`, `specification/check.md`, a loopback test. `2026-09-30-blank-max-request-bytes-exits-2.md`: `crates/thinkthen/src/cli/edge.rs`, `specification/settings.md`. `2026-09-30-transforms-score-the-band-low-edge-as-no.md`: `transforms/band/band.jq`, `transforms/score/score.jq`, `transforms/sweep/sweep.jq`, `transforms/trials/trials.jq`, `transforms/trials/test.sh`. `2026-09-30-cache-convert-quote-skips-questions-that-start-the-text-is.md`: `crates/thinkthen/src/core/recording/convert.rs`, `crates/thinkthen/src/engine/store/convert.rs`, `crates/thinkthen/src/engine/store.rs`, `crates/thinkthen/src/cli/cache.rs`, `specification/recording.md`. Each small. | after C2, which edits `settings.md` and `recording.md` |
| 4 | C4 checkpoint packages | `2026-09-30-checkpoints-publish-no-source-wrapper-packages.md`: a checkpoint publish packs the 16 source-wrapper, crate and R packages QA asked for. Files: `sdlc/scripts/surfaces`, `sdlc/scripts/release-pack`, `sdlc/scripts/publish-builds`. Medium. | any free lane; it touches no file C1 to C3 touch |
| 5 | Checkpoint 2 | The full surface sweep on one main commit. The coordinator tags it `checkpoint/surfaces/2026-MM-DD-N` when every check passes. | after C1 to C4 land |

Work outside the lanes:

- **Marketing, owner of `site/`.** The reference page, the providers page's live `check` line once C3 lands, parts 3 and 4 of the site replay issue, and the overhead benchmark for the README line.
- **Ian.** The release rehearsal, then phase 4 and the registry accounts.

## Every open issue

24 open on 2026-10-01. Class: **running**, **batch** (C1 to C4), **outside** (marketing, Ian, or another team), **waits** (a named trigger), **after 0.1**.

| Issue | Blocks 0.1 | Class | Owner or trigger |
| --- | --- | --- | --- |
| `closed/2026-09-30-binding-tests-still-time-stops-and-wait-on-short-bounds.md` (Debt 029) | no | closed | closed by ticket 0356 |
| `2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md` (Debt 020) | no | waits | the test servers are done (ticket 0361); the product fix waits on upstream and Ian's resend choice |
| `2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md` | no | batch | C2 |
| `closed/2026-09-30-duckdb-check-fails-four-tests-on-macos.md` | no | closed | closed by the DuckDB macOS fork quick fix |
| `closed/2026-10-01-macos-forked-children-crash-on-a-channel-wait.md` | no | closed | closed by ticket 0365 |
| `2026-10-01-macos-tls-roots-fork-probe-fails.md` | no | batch | a lane with the M5 |
| `2026-09-30-check-ignores-the-estimated-token-cap.md` | yes | batch | C3 |
| `2026-09-30-blank-max-request-bytes-exits-2.md` | no | batch | C3 |
| `2026-09-30-transforms-score-the-band-low-edge-as-no.md` | no | batch | C3 |
| `2026-09-30-cache-convert-quote-skips-questions-that-start-the-text-is.md` | no | batch | C3 |
| `2026-09-30-checkpoints-publish-no-source-wrapper-packages.md` | no | batch | C4 |
| `closed/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | no | closed | closed by site tickets 0042 and 0044 |
| `closed/2026-09-29-docs-page-naming-supported-providers.md` | no | closed | closed by site tickets 0042 and 0043 |
| `2026-09-30-site-replay-folders-have-no-fixture.md` (Debt 007) | yes | outside | marketing; part 3 waits on site 0047 slice F; part 4 waits on a word check for three status phrases |
| `2026-09-29-readme-key-backend-and-overhead-lines.md` | yes | outside | marketing's overhead benchmark, then one sentence from the queue owner |
| `2026-09-25-release-and-install-for-0-1.md` | it is 0.1 | outside | ticket 0128; Ian's rehearsal and registry accounts |
| `2026-09-20-new-user-stumble-register.md` | row 18 | outside | row 18 ticket 0128; none of ours |
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
| `2026-09-30-a-lone-oversized-record-is-sent-anyway.md` (idea) | no | after 0.1 | coordinator default 7 |

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
6. C1 starts after 0356 because both edit binding test fixtures. C2 starts after 0360 because both edit `specification/settings.md`. C3 starts after C2 for the same files.
7. A lone record over the request size setting is still sent, and one backend refusal still stops the file, as `specification/backends.md:24` and `records.md:124` say. A change waits until after 0.1.
8. `cache_answers` counts only answers from the answer cache; C2 makes the specification say so.
