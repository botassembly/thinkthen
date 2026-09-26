# Issue backlog after core and surfaces

Written 2026-09-25 by Claude from the workspace, on Ian's request. It is for the dev team to work through once the engine and every surface are on main. Ian can overturn the order.

## What changed today

- Five fresh reviewers checked all 105 issues that were not marked closed against main, the tickets, and the records.
- 40 were already done, replaced by later work, or reference pages. Each now carries a closing status line that names the ticket, commit, or later issue (c7cb79e2 and this commit).
- Every closed issue moved to `sdlc/issues/closed/`. That is 139 files. Links outside `sdlc/records/` were rewritten. Sealed records keep their old paths.
- The 66 open issues became 22. Seven merged issues replace 51 old files, and each item in them was checked again against main.
- The marketing copy leftovers ("buckets" and an unsourced 3.6 cents) moved to the mktg repo as `sdlc/issues/2026-09-25-thinkthen-copy-leftovers-from-the-issue-sweep.md`.
- `sdlc/issues/README.md` now sets the rule: landing a ticket closes the issues it settles, in the landing commit. Most of the 40 stale files came from landings that never touched their issues.

## The 22 open issues, in order

### A. Before 0.1 ships

These are fixes a user would hit, or work Ian ruled into 0.1.

| # | Issue | Items | Why first |
| --- | --- | --- | --- |
| 1 | `2026-09-25-public-library-api-gaps.md`, item 8 only | 1 | A live bug on main. Python Polars builds its own cells against ADR 0047 item 10. A widened score of 1.0 prints as `1`. |
| 2 | `closed/2026-09-25-the-answer-cache-three-fixes.md` (closed by 0124) | 3 | Pruning by the model alias deletes every entry, which loses user data. |
| 3 | `2026-09-25-recognize-and-relate-scale-and-shape.md`, items 1 and 2 (fixed by 0123) | 2 | relate sends requests Jev refuses, over about 65,536 input tokens. The splitter fix shares the same loop. |
| 4 | `2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md` | 1 | diff reports success on a comparison that compared nothing. |
| 5 | `2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md` | 1 | Spend totals miss library and SQL calls. It shares a root cause with API gaps item 1. Build them together. |
| 6 | `2026-09-25-audit-is-complete-for-0-1.md` | 4 parts | Ian's ruling: audit grades every function type, scores by the user's measure, shows how steady its bar is, and hands the bar back. |
| 7 | `2026-09-25-command-wording-and-help-fixes-before-0-1.md` (0123 fixed items 3 and 4 and the relate half of item 8) | 9 | Small wording and help fixes. One Quick Fix batch could take most of them. |
| 8 | `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, sections "Spec or doc claims that are wrong" and "Pages owed for 0.1" | 4 + 6 | Four doc lines claim what the code does not do. Fix those first. |
| 9 | `2026-09-25-test-harness-and-review-leftovers.md`, items 2 to 4 | 3 | Land before ticket 0119, because the mutation audit keeps the secrecy tests and uses the shared backend. |
| 10 | `2026-09-25-release-and-install-for-0-1.md` | 15 | No release ticket exists. This is the largest 0.1 gap. It needs Ian's rulings below. |
| 11 | `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md` | 7 | Items 1 to 4 and 7 can go now. Item 5 waits for DuckDB (0110) and pandas (0122). Item 6 waits for Beatles Bench ticket 0006. |
| 12 | `2026-09-20-new-user-stumble-register.md` | living | Stays open until launch by design. Add a row per stumble. |

`2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md` is the plan for packing as a setting. Its timing follows the plan and experiment 261, which is running. It is not a defect.

### B. In flight, owned by a ticket

| Issue | Owner |
| --- | --- |
| `2026-09-21-the-polars-shape-as-the-deck-shows-it.md` | 0122 pandas. Close when it lands. |
| `2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md` | 0110 DuckDB |
| `2026-09-24-the-churn-probe-left-one-panic-unexplained.md` | 0094's C churn probe |
| `2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md` | 0119, after every surface lands. It also takes test harness item 1. |

### C. After 0.1

- `2026-09-25-public-library-api-gaps.md`, items 1 to 7. Some need rulings (below) before 0.1 if the public API is frozen at 0.1.
- `2026-09-25-recognize-and-relate-scale-and-shape.md`, items 3 to 8.
- `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, "Pages that can follow 0.1", 9 pages.
- `2026-09-25-test-harness-and-review-leftovers.md`, items 5 to 10.
- Feature ideas, each its own design: `2026-09-24-rank-by-graded-relevance-for-search-reranking.md`, `2026-09-23-annotate-options-from-a-file-or-a-record.md`, `2026-09-23-record-the-backends-own-time-for-each-call.md`, `2026-09-21-what-a-procedure-runtime-asks-of-a-judgment.md`, `2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md` (the default-model pin and the refusal on a model change).

## Ian's rulings, 2026-09-25

Ian answered every question the sweep raised. Each ruling sits in its issue too.

1. Release publishing moves to GitHub Actions as an intentional release process in 0.1, with trusted publishing on all four registries. This overturns the 2026-09-22 Actions pause for release jobs only. See the release issue.
2. The Homebrew tap lives under `botassembly`.
3. R ships through R-universe.
4. No separate Rust Polars crate. The Rust Polars door moves into `thinkthen` behind an optional `polars` feature. This amends ADR 0047. Ian can overturn this reading of his question.
5. No setting may do nothing. `cache_bytes` leaves the library, ticket 0084, and every binding. Sweep the settings for any other setting with no effect before 0.1.
6. `check` shows the model, the provider, the URL, and all its outputs. It prints "unspecified" when no model name is given.
7. Nobody asks rusqlite upstream for a fix. Keep the workaround and add a test.

These choices belong to the dev team. They need no ruling from Ian:

- Engine counters per engine or per process.
- Public error constructors, or one documented pattern.
- How to spell find's "none" option and annotate's per-question parts.
- The `tag` key rule and the `rank` order measure in the audit issue.

## How to use this report

Work section A top to bottom. When a ticket lands, close its issues in the landing commit and move them to `closed/`. Rewrite this report when section A is empty.
