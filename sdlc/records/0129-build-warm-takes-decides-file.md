# 0129: Build warm takes the question file decide uses

Status: built; ladder run once after the merge of `origin/main`; ready to land. Owner: Claude.

Branch `ticket/0129-warm-takes-decides-file`. The ticket is `sdlc/tickets/0129-warm-takes-decides-file.md`. Two design reviews accepted it on 2026-09-25, the second after three fixes. The change raises two surface ceilings, so the repository rule asks a second agent to review the code and name what it checked. The coordinator routes that review. Ian can overturn every choice this record marks as decided.

## Result

- **SQLite.** `warm_question` takes any decide question, cut or banded. Choose, tag, and score keep today's sentence. `flush` sends each arm of `LoadedQuestion` through `decide_many_with`. The README says warm takes a band and ignores it.
- **DuckDB.** LOAD takes a process-wide serial, passes it to `register_all`, and registers the kept connection under it where that call sat before. `register_warm` stores the serial as the aggregate's extra info. `Warm::finish` begins its invoke, then reads an `'@file'` question through `kept_file` in `src/tables.rs`:
  - an `'@~'` path refuses at once (decision 7);
  - `connections::by_serial` returns a counted guard, or the release refusal (decision 6);
  - `Kept::gate` waits with a stop closure: a running relate query refuses at once, and Ctrl-C reads the cancel sentence (decision 4);
  - `Conn::files` builds `Files` from the kept connection's client context inside the gate, and `read_named` reads the file with decide's sentences. `Files` drops first, then the gate, then the guard.
- `Caller::named_file` now calls the same `read_named`, which folds its four refusal sentences into one table.
- Relate's release sentence now reads "reopen the database writable and LOAD the extension". `databases_suite.py` pins it.
- **PostgreSQL.** No source change. `check.sh` gains `warm_takes_the_banded_file_decide_uses`.
- ADR 0038 gains a 0129 amendment, whose bold sentence `source_checks.py` now pins in place of part one's warm sentence. The DuckDB README and NOTES follow it. `sdlc/issues/2026-09-25-public-library-api-gaps.md` gains item 9, the two-arm helper.

## Stop rule 1: the DuckDB spike

The spike ran after design steps 1 to 3, before any test or page. Script and log sit in the session scratchpad under `t0129/`, outside the repository.

| Case | read_text | decide | warm | Outcome |
| --- | --- | --- | --- | --- |
| 35 access cases: five file settings over seven folders | as DuckDB rules | agrees in 35 | agrees in 35 | No disagreement |
| Two databases, B with `enable_external_access = false` | | | A reads, B refuses | Agrees |
| `SET home_directory`, then `'@~/q.json'` | reads | reads | missing | Disagrees. Decision 7: warm refuses `'@~'` |
| `SET file_search_path`, then a relative path | reads | missing | missing | Decide and warm agree. No change |

No case outside decision 7 disagreed, so decision 8's fallback did not run. The `file_search_path` row shows that decide already ignores the search path. That predates this ticket and is not filed here.

## Plants

Each plant was applied to a copy, built, run against its test, and restored. Each restored file was touched, and the ladder then rebuilt it.

| Plant | Test | Red result |
| --- | --- | --- |
| SQLite: refuse a band in `warm_question` | `test_warm_takes_the_banded_file_decide_uses` | warm and the inline warm read the refusal |
| SQLite: build warm's question from the file's `decide` text alone | same | 4 sends, wanted 2 |
| SQLite: let warm take any kind | `test_a_banded_question_is_refused_where_it_has_no_answer` | the refusals dict differs |
| PostgreSQL: build warm's question from the file's `decide` text alone | `warm_takes_the_banded_file_decide_uses` | `bcount` 40, wanted 20 |
| DuckDB D1: restore the `@file` refusal | `r2_22_warm_takes_the_banded_file_decide_uses` | warm read the refusal |
| DuckDB D2: send `'@file'` down the inline path | same | 4 sends, wanted 2 |
| DuckDB D3: build a decide question from the file's text | `r2_22_warm_judges_one_question_per_group` | warm answered 1 |
| DuckDB D4: read the file with `std::fs` | `access_cases_match_duckdb` | the `external_off` cases read and sent |
| DuckDB D5: take the first registry entry | `two_databases_each_judge_their_own_access` | B's warm read the file |
| DuckDB D6: drop the busy check | `warm_inside_a_relate_query_refuses_and_never_hangs` | the child hung until the 60 s timeout |
| DuckDB D7: read a missing entry's path as inline text | `warm_after_a_release_names_a_fix_that_works` | warm answered 1, not the sentence |
| DuckDB D8: drop the `'@~'` refusal | `access_cases_match_duckdb` | warm read the missing-file sentence |

The first SQLite run had P1 and P3 green. Their edits missed the formatted source, so no plant applied. The runner then asserted each edit, and both turned red. The first nested-relate test passed for the wrong reason, because relate's `COLUMNS(*)` pruned the unused warm column. A `WHERE w > 0` keeps it, and D6 then hangs.

## Lines

| Part | Budget | Nonblank lines, net |
| --- | --- | --- |
| `databases/sqlite/src` | 10 | 10 |
| `databases/duckdb/src` | 90 | 77 |
| `databases/postgresql/src` | 0 | 0 |
| SQLite tests | 30 | 20 |
| DuckDB tools | 110 | 73 |
| PostgreSQL `check.sh` | 15 | 10 |

Ratchets rose to the measured totals: SQLite `ratchet.json` 1616 and `ratchet.py.json` 1199, DuckDB `ratchet.json` 3828 and `ratchet.py.json` 1894.

## Ladder

The branch merged `origin/main` at `0c38769e` before the run, and the merge touched nothing under `databases/`. The ratchets were measured again after it. Each rung ran once, with the real key unset.

| Rung | Exit | Note |
| --- | --- | --- |
| `install` | 0 | 17:10 to 17:17 |
| `lint` | 0 | 17:19 to 17:22 |
| `test` | 0 | 17:22 to 17:41 |
| `spec` | 0 | 17:41 to 18:04 |
| `surfaces` | 0 | 18:04 to 18:17. All ten surfaces pass, DuckDB, SQLite, and PostgreSQL included |

The first ladder attempt wrapped `install` in `flock` on the heavy lock. The rung then waited for the lock its own wrapper held, and other builds queued behind it for about 30 minutes. The build stopped that chain by process id. `install` then ran once with the held-lock variable set, and the other rungs ran unwrapped.

## Stop rules

None crossed. Stop rule 3: no other branch on `origin/main` changed `connections.rs`, `src/ffi.rs`, or `src/relate.rs` before the merge.

## For the lander

- Close `sdlc/issues/2026-09-25-warm-refuses-the-question-file-decide-uses.md` into `closed/` with a status line naming this ticket and its landing commit.
- Tickets 0126, 0127, and 0128 share `databases/duckdb/README.md`, `databases/sqlite/README.md`, `databases_suite.py`, `databases/postgresql/check.sh`, and the SQLite tests. Whichever lands second merges.
