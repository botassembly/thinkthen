# Area 2: Question cache

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

The question cache keeps one stored answer for each backend question under a SHA-256 key, in `thinkthen.sqlite` (live) or `thinkthen.jsonl` (committed fixture), answers lookups for the cache, record and replay modes, and offers `cache prune`, `cache unused`, `cache convert` and `status` over it.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/` or `sdlc/`. Line counts are nonblank lines.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, store | `engine/store.rs` (457), `engine/store/fixture.rs` (263), `engine/store/convert.rs` (157), `engine/store/prune.rs` (313) |
| Code, key and shares | `core/pack.rs` (244: `QuestionKey` at `:84-125`, `shares` at `:269`) |
| Code, command | `cli/cache.rs` (180), `cli/status.rs` (297) |
| Code, mode choice | `engine/pipeline.rs:278-296` (`Engine::store`, `:280`) |
| Tests | About 71 tests. Unit: `engine/store/tests.rs` (8), `engine/store/prune/tests.rs` (1). Integration under `crates/thinkthen/tests/backend/`: `question_cache.rs` (5), `question_cache_steps.rs` (4), `default_cache.rs` and `default_cache/` (22, 1,148 lines), `cache_configuration.rs` (5), `cache_partial.rs` (1), `cache_trust.rs` (3), `cache_identity.rs` and `cache_identity/` (13), `status.rs` (8), `default_cache_storage.rs` (1) |
| Contract | `specification/recording.md` "The question store", "Pruning a cache", "Converting a folder to the question fixture"; `records.md:89`; `result.md:226`; `settings.md` rows 91 to 97; ADR 0111 (sections 2, 3, 7, 10 and the 2026-09-30 amendment), 0033, 0036, 0100, 0113. ADRs 0035 and 0099 are withdrawn by 0111 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | 1,911 nonblank lines in 7 files, 1,667 without `core/pack.rs` (shared with area 1) |
| States and concurrency | 4 | No thread of its own, but the store is written by several processes at once under SQLite's file lock. Every statement retries a busy file with a doubling wait from 2 ms to 100 ms, for at most 30 s, and checks the call's stop between waits (`engine/store.rs:298-325`). Four modes (`Replay`, `Record`, `Cache`, `Refresh`) plus a lazy first open that imports a fixture inside the schema transaction (`engine/store.rs:244-282`) |
| Rules and refusals | 4 | About 28: four modes; two refresh triggers (`--refresh-cache`, alias `jev-latest`, `engine/pipeline.rs:282`); the folder refusals (path is a file, both files present, hot journal, non-private default folder, dangling link, wrong schema version); eight fixture-line refusals and checks (`engine/store/fixture.rs:106-128`); two fixed prune sentences (`engine/store/prune.rs:22-29`); five usage refusals in `cli/cache.rs`; the 500-key lookup chunk; two fixed storage sentences (recording and default cache); convert's folder and skip sentences |
| Surfaces touched | 5 | All 22. Every engine opens the store. Each of the 21 bindings has cache and replay checks (every `libraries/*/` and `databases/*/` holds files naming `THINKTHEN_CACHE` or `thinkthen.sqlite`). Only Polars has no replay file |
| Settings | 4 | Seven `settings.md` rows: Answer cache, Recording, Prune target, Prune preview, Unused entry report, Convert quoted form, Prune selectors. The Model row also drives the alias refresh |
| Contract weight | 4 | Four spec pages with sections (`recording.md`, `records.md`, `result.md`, `settings.md`), five ADRs (0111, 0033, 0036, 0100, 0113) and ADR 0111's amendment: 10 |
| Churn and debt | 5 | Rewritten on 2026-09-30 by ticket 0304 slice 2, then changed again after landing. 19 commits since 2026-09-23 on the store, cache command and status paths. Five fix or review-fix commits after `7efc6c7ab` (`18b537f3e`, `9ea8f9f3e`, `c572d60c2`, `469354f5e`, and the prune rewrite `7b7e74a18` then `5c6a08db9`). One open issue, `2026-09-30-sql-host-store-proofs-are-partial.md` |

Mean 4.1, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The store matches ADR 0111 sections 2 and 3 closely. The key is the five joined parts (`core/pack.rs:84-100`), states are stored once and deduplicated (`engine/store.rs:35-56`), and replay refuses a folder holding both files (`engine/store.rs:143`). A partial reply stores only its good answers (`engine/pipeline/run.rs:413-432`, rows built only for `Ok` answers). Three divergences. (1) ADR 0111 section 7 says a row's usage is absent when any question lacks a share (`sdlc/planning/adr/0111-question-cache-and-one-batching-path.md:193`). `engine/facade/each.rs:221` with `each.rs:296-304` keeps a partial sum, while `public/asking.rs:105-110` and `cli/asking/judged.rs:200-203` drop the usage. A stored answer with no token counts triggers it. (2) `cli/cache.rs:146` does `text.split_at(text.len().saturating_sub(1))` on the `--older-than` value, and `str::split_at` panics when the cut falls inside a multi-byte character, so `--older-than 5é` should crash with no fixed sentence. Read from the code and Rust's documented behaviour; not run. No test covers a non-ASCII unit (`cli/cache.rs:163-190`). (3) `specification/recording.md:32` says `status` "opens no connection" and two sentences later that it opens `thinkthen.sqlite` read-only (`engine/store/prune.rs:268`). "Connection" means network there, and a reader could take it the other way. `recording.md:74` does not state the absent-usage rule |
| Reliability | C | Failure paths have counting tests: a partial reply stores its good answers and a rerun asks only the failed one (`tests/backend/question_cache.rs:102`), two processes write one store (`:194`), a stored answer that no longer decodes is resent under a cache and fails a replay (`:236`), and the busy wait ends at the limit or on a stop (`engine/store/tests.rs:245`). Routine tests hold wall-clock limits: `engine/store/tests.rs:232` sleeps 300 ms to hold a lock, and `:262` and `:274` assert each wait ends in under 1 s. `tests/backend/default_cache/usage.rs:307-315` polls for up to 10 s. Ticket 0352 (in flight, see below) removes these. The area was rewritten in the last 7 days, which caps it at B, and the wall-clock limits drop it to C. The busy limit is a constant 30 s (`engine/store.rs:137`), settable only in unit tests (`:176-180`), so no command-level test reaches it. One open issue says the DuckDB and PostgreSQL runners do not count answer rows in the store |
| Maintainability | B | One module owns the store, and one `waiting` helper owns every busy wait (`engine/store.rs:298`). The only lint suppression is an `expect` for the non-Unix build (`engine/store.rs:454`). `Debug` for `Store` and `Found` withholds text. Small duplicates remain: the journal and sync pragmas are written twice (`engine/store.rs:255`, `engine/store/prune.rs:275`), the 30 s limit appears as a literal in three places (`engine/store.rs:137`, `engine/store/prune.rs:32`, `engine/store/convert.rs:76`), and `prune.rs` builds a fresh `Cancel::default()` in five functions. `engine/store.rs` holds 457 of its 500-line cap, and `prune.rs` holds 313 |

## Strengths

- One store and one key for every mode, so replay, cache and record cannot drift apart (`engine/pipeline.rs:282-289`).
- Writes take the lock first, in one transaction per reply, and roll back on failure (`engine/store.rs:226-239`). SQLite's own busy wait is off, so a stop always reaches the wait (`engine/store.rs:252`).
- A fixture line is checked against its own key and named by number, never by text (`engine/store/fixture.rs:106-137`). The fixture text is a function of the entries, so a review reads it (`engine/store/fixture.rs:80-103`).
- `cache convert` holds the live file's write lock from read to removal and writes through a synced temporary file (`engine/store/convert.rs:46-69`, `:142-158`).
- `cache prune` chooses its selection under the write lock, so no writer changes an answer between choice and delete (`engine/store/prune.rs:95-137`). `--dry-run` opens the file read-only (`:82-89`).

## Cleanup

1. **Give `--older-than` a total parser.** Where: `cli/cache.rs:146`. Why: `split_at` panics on a multi-byte last character, so a typo crashes where every other bad value gets a fixed sentence. Use `char_indices` or `strip_suffix` and add a non-ASCII row beside `cli/cache.rs:177-188`. Size: S. Blocks 0.1: yes (an untested crash on a documented command; unconfirmed until run).
2. **Make the usage rule one rule.** Where: `engine/facade/each.rs:221` and `:296-304`, `public/asking.rs:105-110`, `cli/asking/judged.rs:200-203`. Why: ADR 0111 section 7 says absent when any share is missing. One surface sums what it has. State the rule in `recording.md:74` and `result.md:226`. Overlaps area 1's item 3. Size: M. Blocks 0.1: no.
3. **Replace wall-clock asserts in the store tests.** Where: `engine/store/tests.rs:232`, `:262`, `:274`, `tests/backend/default_cache/usage.rs:307-315`. Why: `< 1 s` fails under load. Hold the lock with a channel and assert the error kind. Ticket 0352 is landing on main and may already cover them. Size: S. Blocks 0.1: no.
4. **Name the busy limit once.** Where: `engine/store.rs:137`, `engine/store/prune.rs:32`, `engine/store/convert.rs:76`. Why: three copies of 30 s for one documented rule. Share one constant, and share the connect pragmas (`engine/store.rs:255`, `engine/store/prune.rs:275`). Size: S. Blocks 0.1: no.
5. **Say what "opens no connection" means.** Where: `specification/recording.md:32`. Why: the same paragraph says status opens the store read-only. Write "opens no network connection". Size: S. Blocks 0.1: no.
6. **Count store rows in the DuckDB and PostgreSQL runners.** Where: `databases/duckdb` and `databases/postgresql` case runners (open issue `2026-09-30-sql-host-store-proofs-are-partial.md`). Why: a store regression there would pass every check. The issue says pay before 0.1. Size: M. Blocks 0.1: yes (the issue's own "Pay when: before 0.1").
7. **Add a command-level test for the busy limit.** Where: `tests/backend/question_cache.rs:194` neighbourhood. Why: the 30 s wait and its fixed sentence are reached only by unit tests, because the limit cannot be set from outside. A test with `THINKTHEN_`-free injection is not possible today, so a settable limit under `cfg(debug_assertions)` (as other test-only variables use) would serve. Size: M. Blocks 0.1: no.

## Confidence: medium-high

Read: all of `engine/store.rs` and `engine/store/{fixture,convert,prune}.rs`, `core/pack.rs` lines 1 to 140 and 255 to 275, `cli/cache.rs`, the status gatherer in `cli/status.rs` (lines 100 to 200), `engine/pipeline.rs` `Engine::store`, the recording page's store, prune and convert sections, ADR 0111 sections 2, 3, 7 and 9, the open issue, and the names of every cache test plus the wait tests in full.

Not checked: no test or build was run, so items 1 and 7 are inferred from code (item 1 from `str::split_at`'s documented panic). `cli/status.rs` formatting and JSON shape (lines 200 to 297) were skimmed. ADRs 0033, 0036 and 0100 were read by heading only. I did not read ticket 0304 slice 5 evidence or ticket 0318, so the SQL hosts' folder rules are taken from the spec. Area 12's ticket 0352 is landing on main after this commit; this report grades the pinned commit.
