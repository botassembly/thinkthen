# Issue priorities, 2026-09-30

Status: current, refreshed 2026-09-30 evening by the debt and issue sweep. It orders the 31 open issues in `sdlc/issues/`, 14 of them `Kind: debt`. The sweep checked every issue against main `88649ec0d`. None was already fixed. Nine merged into same-family issues and moved to `closed/`, so the count went from 40 to 31 and debt from 15 to 14. Git history holds the earlier page and its investigations. Later the same evening a records Quick Fix closed rank 16 and filed rank 19a, so 31 issues stay open, 13 of them debt. Ian can overturn any rank, owner or trigger.

## How the ranks were set

Ruling 10: no public release before 0.1, and 0.1 waits for every surface and binding. "Blocks 0.1" means the release cannot ship on every channel without it. 0.1 blockers come first, in the order they can start. Then work that can start now, then work waiting on a slice, then later features. Size is small, medium or large.

## Ranked table

| Rank | Issue | Blocks 0.1 | Owner | Waits for | Size |
| ---: | --- | --- | --- | --- | --- |
| 1 | `2026-09-30-postgresql-check-keeps-wall-clock-limits-under-load.md` (debt) | yes | ticket 0340, lane 1 | running | small |
| 2 | `2026-09-30-ordered-output-test-races-the-next-request-under-load.md` (debt) | yes | ticket 0340, lane 1 | running | small |
| 3 | `2026-09-30-graded-rank-tests-rewrite-one-question-file-in-place.md` (debt) | yes | ticket 0340, lane 1 | running | small |
| 4 | `2026-09-30-duckdb-split-denials-case-failed-once-under-load.md` (debt) | yes | ticket 0340 if it takes it; otherwise none until it fails again | running | small |
| 5 | `2026-09-30-static-library-exports-sqlite-symbols.md` (debt) | yes | a new ticket with one M5 proof | nothing | medium |
| 6 | `2026-09-25-public-library-api-gaps.md` (debt) | yes, items 1, 2, 3, 9, 10 | item 10 a Quick Fix now; one SQL host ticket for 1, 2, 3, 9; 6 and 7 after 0.1 | 0304 slice 4 for items 1, 2, 3, 9 | medium |
| 7 | `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | yes | marketing | nothing | small |
| 8 | `2026-09-30-question-file-reader-copies-and-uncapped-loaders.md` (debt) | yes | a Quick Fix | 0304 slice 4 (`public/relate.rs`) | small |
| 9 | `2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md` | yes | one relate ticket: the default and the both-ways edge shape | 0304 slice 4 | medium |
| 10 | `2026-09-30-audit-and-diff-lost-the-batch-setting.md` (debt) | yes | 0304 slice 5, or a Quick Fix | 0304 slice 4 | small |
| 11 | `2026-09-30-old-batching-files-still-have-live-callers.md` (debt) | yes | 0304 slices 4 and 5 | 0304 slice 4 | large |
| 12 | `2026-09-30-site-replay-folders-have-no-fixture.md` (debt) | yes, before the site goes public | 0304 slice 5; marketing for items 2 to 4 | 0304 slice 5 | medium |
| 13 | `2026-09-20-new-user-stumble-register.md` | yes, rows 9, 18, 19 | page 11 of rank 25 for row 9; 0128 Phase 4 for row 18; marketing for row 19 | row 9 on 0304 slice 4 | small |
| 14 | `2026-09-29-readme-key-backend-and-overhead-lines.md` | yes | marketing's overhead benchmark, then the queue owner | the benchmark run | small |
| 15 | `2026-09-25-release-and-install-for-0-1.md` | yes, it is 0.1 | ticket 0128 phases 3b and 4; Ian dispatches | every rank above; Ian's registry accounts | large |
| 16 | closed: `closed/2026-09-30-how-to-list-has-two-hand-kept-copies.md` (debt, paid 2026-09-30) | no | done | nothing | small |
| 17 | `2026-09-30-debt-issues-lack-the-three-ruled-fields.md` | no | the coordinator, a records Quick Fix | ticket 0340 landing | small |
| 18 | `2026-09-26-count-secure-connections-at-sixteen-jobs.md` | no | the queue owner, a loopback experiment | nothing | small |
| 19 | `2026-09-30-live-batching-flake-and-unexplained-usage-calls.md` | no | the queue owner; the calls traced to rank 19a, the exit 4 waits for a live run with `--details` | the next live bench run | small |
| 19a | `closed/2026-09-30-test-stress-writes-the-real-usage-totals.md` (severity 2, closed) | no | the queue owner, a Quick Fix in `sdlc/scripts/test-stress` | nothing | small |
| 20 | `2026-09-30-spec-no-calls-edges-need-a-real-send.md` | no | the queue owner | the release QA suite's edge list | medium |
| 21 | `2026-09-29-docs-page-naming-supported-providers.md` | no, except the Liquid timeout line for stumble row 19 | marketing | nothing | small |
| 22 | `2026-09-26-every-surface-should-give-back-run-facts.md` | no | 0314 slice 4 for item 1; tickets 0300 and 0302 for items 2 and 3; marketing for item 6 | 0314 slice 4 | large |
| 23 | `2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md` (debt) | no | upstream (ollama/ollama#18718); ticket 0339 builds the workaround | Ollama; 0339 after 0304 slice 4 | small |
| 24 | `2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md` (debt) | no | upstream (Zig) | a Zig release | small |
| 25 | `2026-09-25-docs-how-tos-and-spec-claims-owed.md` | page 11 only | a docs ticket; marketing for page 23 | page 11 on 0304 slice 4 (`find` fixtures) | large |
| 26 | `2026-09-25-recognize-and-relate-scale-and-shape.md` | no | a ticket after 0304 slice 4 | 0304 slice 4 | large |
| 27 | `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md` | no | a ticket after 0.1 | 0304 slice 4 | medium |
| 28 | `2026-09-23-annotate-options-from-a-file-or-a-record.md` | no | a ticket after 0.1 | 0304 slice 4 | medium |
| 29 | `2026-09-30-polars-door-cannot-test-lazy-streaming.md` (debt) | no | none until a user asks | a user, or clean advisories | small |
| 30 | `2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md` | no | tickets after 0.1 | nothing | large |
| 31 | `2026-09-24-rank-by-graded-relevance-for-search-reranking.md` | no | none until a user asks | a user | small |

## Blocks 0.1

Every debt with `Pay when: before 0.1`, then the other blockers:

1. Load flakes, ranks 1 to 4. Ticket 0340 pays ranks 1 to 3 now.
2. The macOS static library and R package, rank 5.
3. The SQL host copies of engine code, rank 6, items 1, 2, 3, 9 and 10.
4. The capped reader and the uncapped loaders, rank 8.
5. The audit and diff batch warning, rank 10.
6. What 0304 slices 4 and 5 must move, and the SQL host store proofs, rank 11.
7. The site smoke and its fixtures, rank 12.
8. Marketing: the reference page (rank 7), the overhead line (rank 14), the Liquid timeout line (rank 21).
9. The relate default and the both-ways edge shape, rank 9.
10. Stumble rows 9, 18 and 19, rank 13.
11. The release itself, rank 15.

## Debt by trigger

| Trigger | Debt |
| --- | --- |
| Now, in ticket 0340 | ranks 1 to 4 |
| Before 0.1, can start now | rank 5; rank 6 item 10 |
| Before 0.1, after 0304 slice 4 | rank 6 items 1, 2, 3, 9; rank 8; rank 10 |
| 0304 slice 5 lands | ranks 11 and 12 |
| The next docs ticket | none; rank 16 paid 2026-09-30 |
| Upstream fixes | rank 23 (Ollama), rank 24 (Zig) |
| A user asks | rank 29; rank 6 items 6 and 7 |

## Ready as tickets now

None of these touches the files lanes 1 to 3 edit: lane 1's flake tests and `databases/postgresql/check.sh`, lane 2's `libraries/` ports, and lane 3's crate, conformance and binding test files.

1. **Rank 6 item 10, Quick Fix.** Outcome: SQLite's and PostgreSQL's `thinkthen_plan` serialize the crate's `PlanEstimate`, and their plan checks pass with no byte changed. Touches `databases/sqlite/src/scalars/plan.rs` and `databases/postgresql/src/keyed.rs`.
2. **Rank 5, ticket.** Outcome: the macOS `libthinkthen.a` defines only the header's `thinkthen_` functions, the macOS R package exports no `sqlite3_` name, and a check proves both on the M5. Touches `libraries/c/localize.sh` and the R package build. Its M5 proof should come before Ian's rehearsal dispatch, so the rehearsal's macOS jobs confirm it.
3. **Rank 16, Quick Fix. Done 2026-09-30.** Outcome: the how-tos are listed once, and the other file points to that list. Touches `demos/README.md` and `sdlc/planning/documentation-plan.md`.
4. **Rank 18, experiment.** Outcome: a loopback TLS count of the connections a `--jobs 16` run opens. It becomes a ticket only if the count exceeds 16.
5. **Rank 19, investigation. Done 2026-09-30; it found rank 19a.** Outcome: the extra usage calls are traced to their runs by comparing `status --json` totals with the bench's logs, with no paid call.
6. **Rank 7, marketing.** Outcome: the reference page matches `channels.md` and `backends.md`, and the three site links name `closed/` paths.

Rank 17 follows ticket 0340's landing, so the Severity and number fields go onto the final debt list.

## Waiting on a slice

- **0304 slice 4** (lane 3): rank 6 items 1, 2, 3, 9 (outcome: PostgreSQL uses a public relate rule parser and `thinkthen::Error`, the SQL hosts share one usage total and one rule for a loaded question); rank 8 (outcome: one crate reader returns the text or a typed reason, and every surface maps it, with the Rust and Python loaders refusing over 1 MiB); rank 9 (outcome: relate's single-answer relations ask one choice with none of these, if the paid bench run beats F1 0.523, and both-ways edges print an unordered pair); rank 10 (outcome: `audit` and `diff` read the batch setting again and restore ADR 0085's warning); rank 11 items 1 and 2; rank 23's workaround (ticket 0339); ranks 25 (page 11), 26, 27 and 28.
- **0304 slice 5**: ranks 11 and 12, then marketing's site items in rank 12.
- **0314 slice 4** (lane 2): rank 22 item 1.
- **0335 slice 2**: none on this list. Ticket 0340 is moving rank 1's timing limits out of the routine check.

## Coordinator defaults

Taken 2026-09-30 under the workspace rule to record reviewed choices and proceed. Ian can overturn each one.

1. Both-ways relate edges get an unordered `pair` shape before 0.1, in the relate ticket after 0304 slice 4, because a breaking change after 0.1 costs every consumer (rank 9).
2. Ian's `rehearse` dispatch waits for 0304 slice 3 and the PostgreSQL macOS ticket 0336; both have landed.
3. Relation pairs keep the whole text for 0.1, with the distance limit as an opt-in, because natural text with a relation three sentences apart was never measured (rank 27).
4. Homebrew stays a Mac option; the curl script covers Linux (rank 15, item 3).
