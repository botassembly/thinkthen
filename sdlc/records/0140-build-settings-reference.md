# 0140: Build every setting explained in one place

Status: built; ladder run once after the merge of `origin/main`; ready for code review. Owner: Claude.

Branch `ticket/0140-settings-reference`, built in `worktrees/thinkthen-lane-3`. The ticket is `sdlc/tickets/0140-settings-reference.md`, design ticket C1 of `sdlc/issues/2026-09-26-batching-design.md`. It lands after ticket 0139, ADR 0048, which is on main. Ian can overturn every decision the ticket lists.

## Result

- `specification/settings.md` is new. Prose above the table defines a setting, states precedence with the typed and environment split, and says how to read a cell. The table under `## Settings` has the ticket's 17 columns and 39 rows, one for each setting on main. Below it sit "Defaults with no recorded reason", "Settings on the way" with one line for each designed setting, and "Keeping this page true".
- `sdlc/scripts/settings` is new, 198 nonblank lines. It checks the header order, the cells, every long flag in every command's help in both directions, every quoted `THINKTHEN_` name in product source in both directions, and every question-file key in the schema in both directions. `--self-test` plants five faults into a copy of the page and pins every sentence, and a sixth case passes the real page.
- `sdlc/scripts/spec` runs `settings --self-test` and `settings` right after its build puts the binary on `PATH`.
- `sdlc/tickets/README.md` states the rule: a ticket that adds or changes a setting updates its row in the same commit. `specification/README.md` and `sdlc/scripts/README.md` list the page and the check.
- `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md` files the surface gaps: timeout, retries and profile on no library or SQL surface, no engine-level model in C and SQL, and the DuckDB and SQLite README lines that say `cache_bytes` caps the cache. It cites tickets 0109 and 0110 for the deliberate SQL address and key gap.
- `sdlc/issues/2026-09-26-site-builds-its-settings-page-from-the-settings-table.md` asks the website owner to build the site page from the table.

## What the check reads on main

At the final run: 39 rows, 49 help flags, 3 product variables (`THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`), and 9 question-file keys (`true`, `false`, `options`, `labels`, `levels`, `threshold`, `on`, `model`, `profile`). The help includes `audit --match`, which ticket 0135 added after the design was written.

## Plants

Each plant rewrote the real page with the self-test's own case, ran `sdlc/scripts/spec`, and restored the page from a saved copy. `cmp` confirmed the restored page equals the saved one, and it was touched. Scripts and logs sit in the session scratchpad under `t0140/`. The diff holds no plant text outside the self-test's own cases.

| Plant | Rung | The real check's sentences |
| --- | --- | --- |
| (a) the `--jobs` row removed | `spec` exit 1 | `settings.md: --jobs is in the help and has no row` |
| (b) a row naming `--planted-setting` and `THINKTHEN_PLANTED` | `spec` exit 1 | the two stale-row sentences |
| (c) the Default and Allowed values headers swapped | `spec` exit 1 | the column 3 and column 4 sentences |
| (d) `THINKTHEN_CACHE` removed from its cell | `spec` exit 1 | `settings.md: THINKTHEN_CACHE is read by product code and has no row` |
| (e) `on` removed from its cell | `spec` exit 1 | `settings.md: on is a question-file key and has no row` |

Every plant went red. The rung stopped at the self-test, whose real-page case failed, and the check alone printed each pinned sentence.

## Ladder

Run once each after merging `origin/main` at `4741a31e`:

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint` | exit 0, 2 min 21 s |
| `test` | exit 0, 939 Rust tests passed, live-test all cases passed, 1 min 58 s |
| `spec` | exit 0; `settings self-test: 6/6 cases hold`; `settings: 39 rows, 49 flags, 3 environment names, 9 question-file keys, 0 failures`; demos 21 green |

`surfaces` did not run. No Rust source, public API or surface changed, as the ticket says.

## Budgets

| Item | Budget | Measured, nonblank lines |
| --- | --- | --- |
| `specification/settings.md` | 110 | 80 |
| `sdlc/scripts/settings` | 200 | 198 |
| `sdlc/scripts/spec` | 4 added | 4 |
| The three README files | 6 changed | 4 |
| The site issue | 15 | 5 |
| The surface-gap issue | 25 | 7 |

No Rust source changed, so `sdlc/ratchet.json` did not move. No dependency.

## Code review fixes

The code review of `d100a394` returned one medium finding, five low and one nit. All are fixed:

1. The recognize question file's `relation_threshold`, `recognize.kinds` and `recognize.relations`, and the relate file's `relate.fields` and `relate.relations`, now fill the Question-file key cells. The check now reads the schema one level deep and the two key lists `core/recognize_file.rs` checks a recognize file against. Before the cells were filled, the new check failed the real page with five sentences, one per missing key, so it catches the gap the reviewer found. A rule's own `either` is not checked, and the page and the ticket's deferred gaps say so.
2. Flags are read from every help section but `Commands:` and `Arguments:`.
3. Python's `on=` row names `annotate` as well as `recognize`.
4. Each "on the way" line names the library and SQL surfaces it reaches and their tickets, and the word-list line adds `defaults`.
5. The surface-gap issue adds `databases/postgresql/README.md` line 44.
6. The self-test adds a fenced decoy table under `## Settings`, pinned to no failure, and a repeated row, pinned to `settings.md: the setting "Threshold" has two rows`. It now holds 8 cases.
7. `check` returns the row count, and the key and variable patterns sit with the other constants.

The script reached 207 nonblank lines with the fixes. Rewrapping the docstring and the column list and folding three comments brought it to 200, the budget, with no behavior removed.

After merging `origin/main` at `596846f6`, which holds `7275b54a`, `lint` passed in 3 min 4 s. `spec` passed, with `settings self-test: 8/8 cases hold`, `settings: 39 rows, 49 flags, 3 environment names, 14 question-file keys, 0 failures`, and 21 demos green.

## Lane

`worktrees/thinkthen-lane-3` measured 9.4 GB before the build and 9.4 GB after.

## Stop rules

None crossed. `cache_bytes` has no effect on the libraries and SQL surfaces; the row says so and cites item 4 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. Tickets 0138 and 0143 are still in flight. Whichever lands after this one updates its rows under the same-commit rule.
