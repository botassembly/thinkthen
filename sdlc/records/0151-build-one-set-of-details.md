# 0151: Build one set of details

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0151-one-set-of-details`, in lane `worktrees/thinkthen-lane-1`. The ticket is `sdlc/tickets/0151-one-set-of-details.md`, accepted at `61f0541e`. Ian can overturn every decision the ticket lists. No live call ran. Every rung and plant ran with `THINKTHEN_API_KEY` unset, against the loopback backend.

## Result

- `thinkthen_details` in DuckDB returns the command's `--details` line as JSON text. The eight-member struct and its builders are gone.
- Rust's `Details` gains `confidence()` and `url()`. Ticket 0084's frozen block lists both, and `inventory` checks them.
- TypeScript's `index.d.ts` declares `answer.confidence?: number`. `meta.url` was already declared.
- `conformance/cases.json` expects `usage`, `requests_sent` 1 and `cached` false on every single case, and `confidence` on cases 06 and 11. Case 12's reply sends no `usage`, so its details expect none.
- The Rust consumer, C, Python, TypeScript, Ruby, R, SQLite, PostgreSQL and DuckDB runners check `confidence`, `usage`, `requests_sent`, `cached` and the served url, `BASE/systemone`.
- Each binding README gains a "Run facts" section. The Polars README points to it, and `conformance/README.md` states the url rule.

## Plants

Each plant edited one file, ran the named runners under the heavy lock, and restored and touched the file. Scripts and logs sit in the session scratchpad under `t0151/`. A grep of the diff for plant text found none.

| Plant | Result |
| --- | --- |
| None | Green on every surface |
| (a) The ticket's copy of `cases.json` | Red on all nine runners, each naming cases 01, 02, 04, 06, 07 and 12 |
| (b) `Meta::new` drops `/systemone` from the url | Red on url in every runner, and at "line url" in the Rust consumer |
| (c) DuckDB strips `meta.usage` from the line | Red: 13 single cases fail, 01 among them |
| (d) DuckDB returns the old struct | Red: `verbs_suite.py` fails its probability row, and every single case in `conformance.py` is refused |
| (e) `confidence()` returns `None` | Red: cases 06 and 11 fail |
| (f) `url()` returns the model name | Red: every single case fails |
| (g) `confidence?` leaves `index.d.ts` | Red: `tsc` reports TS2339 |

## Deviations

- **`databases/duckdb/tools/selftests.sh` changed.** The ticket does not open it. Its pinned failure sentence now names the first differing field, `bare: wanted False, got True`, because the old mismatch text held the backend's port.
- **Plant (d) ran twice.** `verbs_suite.py` stops at its first failure, so `conformance.py` then ran alone against the reverted build.
- **R keeps a counter across two caches.** The runner runs the counters on their own cache folder, so it adds the first cache's `requests_sent` to the SENT line.
- **Every binding ratchet moved.** The ticket names only the root and DuckDB ratchets. Each binding's own `ratchet*.json` also requires its ceiling to equal its total, so each moved by its runner's growth.

## Budgets

Nonblank lines, net against `origin/main`.

| File | Budget | Measured |
| --- | --- | --- |
| `results.rs` | at most 16 | 14 |
| `cases.json` | at most 100 added, 8 removed | 90 added, 4 removed |
| consumer `cases.rs` | at most 20 | 20 |
| C, R, PostgreSQL, SQLite, Python, Ruby runners | at most 12 each | 4, 9, 4, 1, 2, 4 |
| TypeScript `cases.mjs` | at most 12 | 2 |
| `databases/duckdb/src` | negative | -48 |
| `databases/duckdb/tools` | at most 30 added to `conformance.py`, net at most 10 | 19 added, net -9 |
| `index.d.ts`, `types.test.ts` | 1, at most 3 | 1, 2 |
| Each README | at most 8 | 3 |

Ratchets: `sdlc/ratchet.json` rises from 69,192 to 69,226 for the two accessors and the consumer's checks. DuckDB's Rust ceiling falls from 3,835 to 3,787, and its Python from 1,916 to 1,908. The binding ceilings rise as follows: C 2,147 to 2,154, Python 2,316 to 2,318, R 1,283 to 1,293, Ruby 1,531 to 1,535, SQLite 1,213 to 1,214, PostgreSQL 249 to 254, TypeScript `.mjs` 742 to 744 and `.ts` 272 to 275.

## Ladder

Run after merging `origin/main` at `7eb041cc`.

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint`, with `THINKTHEN_PRIVATE_NAMES` set | exit 0 at `865784fe`, after the ratchet commits |
| `test` | exit 0 |
| `spec` | exit 0, demos 21 green |
| `surfaces` | exit 0 at `865784fe`: all ten surfaces pass, plus the release smoke |

## Code review fixes

The first code review returned five findings, and its re-review added the R absent reader. Each fix is its own commit.

- DuckDB's runner checks case 40's details before its counters. A planted usage of 11 in case 40 turns it red.
- R guards its run-fact checks on the `single` kind, not on the expectation.
- C and R read a missing field as "absent", so an absent `confidence` or `usage` no longer matches null. A planted `"confidence": null` in case 07 turns C red. A planted `"usage": null` in case 12 turns R red.
- PostgreSQL builds its served address in one `url(case)` helper.
- `conformance/README.md` names the command runner's exception from decision 7.

After the fixes, `lint`, `test` and `spec` each exit 0, and the DuckDB, R, C and PostgreSQL `check.sh` runs pass.

## Left for landing and later

- The lander filed `sdlc/issues/2026-09-26-site-run-facts-after-0151.md` and marked asks 5 and 6 of the run-facts issue settled.
- The ticket's deferred gaps 1 to 5 stand.
