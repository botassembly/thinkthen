# 0138: Build command wording fixes, the retried statuses' next step, and pointers that never echo a control character

Status: built; design review accepted on the second pass; code review accepted with three nits, all handled; ladder green after the merge of `origin/main`. Owner: Claude.

Branch `ticket/0138-wording-and-failures`, built in lane `thinkthen-lane-4`. The ticket is `sdlc/tickets/0138-wording-and-failures.md`. Ian can overturn every decision the ticket lists.

## Result

- Status 502, 503, 504, and 529 now end with the 500 next step: `the backend failed after the allowed attempts; try again later or change --max-retries`. One table in `tests/backend/status_reason.rs` pins every status sentence. The `exchange.rs` duplicate is gone, and its `decide` helper moved to `tests/backend/support.rs`.
- `diff --help` names both pairing rules.
- `annotate @set.json` reads `set.json`.
- A question file "takes no key", and the key `version` names the three files that take it.
- `thinkthen_warm` on DuckDB and PostgreSQL refuses a question that is not `decide` with SQLite's sentence and sends nothing.
- A storage failure in the platform default cache prints its own sentence with `--no-cache` and `THINKTHEN_CACHE` as the ways around it. The mapping lives in `told` in `cli/mod.rs` and uses the existing `Failure::Configuration`, so `cli/failure.rs` did not change.
- `recognize --kind PER` names the missing `=`.
- A pointer holding a control character is refused with `a pointer is one line of printable text`. Both pointer refusal sentences echo the typed pointer with JSON escapes.

## Items

- Fixed: wording items 10, 11, 12, 13, 14, 15, and 17. The 400-rows issue. Harness item 5.
- Settled with their commits, checked on main `3b6954d6`: items 2, 5, 6, 7, 8, and 9 by ticket 0126 (`36929f63`, landed at `5f9eea8f`). Items 3 and 4 by ticket 0123 (landed at `6d33e99f`).
- Left: item 1, the deadline digits, in `public/options.rs`. Ticket 0134 owned that file when this ticket was written. 0134 has since landed (`0f3ac7dd`), so item 1 is now unblocked as a Quick Fix. Item 16, "unresolved", touched files that 0134, 0135, and 0137 owned when this ticket was written. It also needs a recorded choice on the JSON key. All three tickets have since landed.

## Reviews

- Design review 1 (fresh read-only Claude session): no blocking findings, four should-fix and five nits. The default-cache design touched `cli/failure.rs`, which 0137's stop rule 4 guards. Two test files would cross the lint's 500-line limit. The `backends.md` row and the SQLite bare 503 line went unnamed. The core changes reach the library and SQL surfaces. The ticket was revised. The mapping moved to `told` in `cli/mod.rs`, and the new tests went to new files.
- Design review 2: accepted with three nits, fixed.
- Code review (fresh read-only Claude session): accepted with three nits. The recording page's "which" clause was split. The duplicated command match and the untested quote escape went into the ticket's deferred gaps. A fourth test row would cross the test budget. As the second agent for the ceiling raise, the reviewer checked `status.rs`, `exchange.rs` and `status_reason.rs`, `cli/mod.rs` against `Command::input`, and the three warm sentences. It found nothing more to delete. It also noted that `default_cache_storage.rs` depends on `XDG_CACHE_HOME`, so the module is now gated to Linux.

## Plants

Each plant was applied with a script, the named test ran, and the file was restored from a saved copy and touched. The script and logs sit in the session scratchpad under `t0138/`, outside the repository. A grep of the staged diff found no plant text.

| Plant | Test | Result |
| --- | --- | --- |
| P1: 503 leaves the phrase table | `status_reason`, `loopback_arms::each_wire_fault_arm_yields_its_kind_and_sentence` | Red: both read the bare `status 503` line. The first attempt removed the row, so the array size failed to compile; the rerun renamed the row to 599 |
| P2: the digest sentence leaves the diff help | `diff::help_names_diff_and_says_what_it_never_does` | Red |
| P3: annotate keeps the `@` | `annotate::a_question_set_named_with_at_is_the_same_file` | Red: exit 5, wanted 0 |
| P4: `holds no key` again | `grammar` table and the escape test | Red: 2 tests |
| P5: `version` loses its clause | `grammar::every_refusal_the_grammar_makes_names_its_key_and_its_exit_code` | Red |
| P6: DuckDB checks only `tag` | `verbs_suite.py` `r2_22_warm_judges_one_question_per_group` | Red: got `decide_many does not take a choose question` |
| P7: PostgreSQL checks only `tag` | `check.sh` `bad_files_name_themselves` | Red: 51 passed, 1 failed |
| P8: `told` inverts the default test | `default_cache_storage` | Red: the recording-folder sentence |
| P9: `--kind` returns the kinds error | `recognize::a_kind_without_a_sign_names_the_sign` | Red |
| P10: the control check never fires | `pointer_echo` | Red: `the record holds nothing at` with the raw escape byte, then the stopped line |
| P11: `told` stops escaping | `pointer_echo` | Red: the raw byte on the `find` row |
| P12: `QuestionFileError::Pointer` stops escaping | `pointer_echo` | Red: the raw byte on the `decide` row |
| P13: `told` drops its stopped-run arm | `default_cache_storage` | Red: the `--jsonl` row reads the recording-folder sentence |

The first DuckDB and PostgreSQL plant runs failed at `cargo fmt --check`, because the plant and the source were not formatted. `cargo fmt` then ran on both crates, and the plant runner formats after each plant. Both reran red for the stated reason.

## Ladder

`origin/main` was merged twice: `1d385569` brought ticket 0133, and `a17f38cc` brought 0134, 0137, and 0139. Each rung ran directly, with no outer `flock`, and `THINKTHEN_API_KEY` unset. The runner polled the heavy lock with `flock -n` before each heavy rung and timed that wait apart from the rung. No rung printed `waiting for the heavy-build lock`, so no rung waited once it started.

The final ladder, after the second merge:

| Rung | Result | Wall time | Wait for the lock before it |
| --- | --- | --- | --- |
| `install` | pass | 1 s | 0 s |
| `lint` | pass | 163 s | none; lint takes no lock |
| `test` | pass | 123 s | 108 s |
| `spec` | pass, 21 demos green | 26 s | 127 s |
| `surfaces` | pass, all ten | 568 s | 1,060 s |

The rung time totals 881 s. The lock waits total 1,295 s, from other builders. The one-minute load stood between 3 and 8 at the rung starts.

Two earlier ladders failed and were fixed:

1. After the first merge, `lint` and `test` failed on the property test `a_pointer_over_written_names_finds_the_value_they_nest`. It drew member names with control characters, which the pointer now refuses. The fix filters them out of the drawn names. The builder's dev loop had run only the integration tests. That ladder's `install` waited 887 s for the lock. Its `spec` (24 s) and `surfaces` (589 s) passed.
2. A `lint` rerun failed on a `proptest-regressions` file the failed property left in the lane. The file was moved to the scratchpad. The next `lint` failed on clippy's `type_complexity` in `default_cache_storage.rs`. The case table now uses vectors.

## Lane trial

This is the first warm measure. Lane 4 held the build folders of ticket 0136.

| Measure | Cold, ticket 0136 | Warm, ticket 0138 |
| --- | --- | --- |
| `install` | 11 s | 1 s |
| `lint` | 219 s | 163 s |
| `test` | 151 s | 123 s |
| `spec` | 929 s, under load near 10 | 26 s |
| `surfaces` | 848 s | 568 s |
| Five rungs | 2,158 s | 881 s |
| `du -sh` of the lane | 29 MB before, 9.3 G after | 9.3 G before the first ladder, 9.6 G after the last |

The warm ladder took 41 percent of the cold one. The largest drop was `spec`. 0136's `spec` ran under load near 10, so part of that gap is load. The lane grew 0.3 G across three ladders and two merges. The time the heavy lock kept this builder waiting, 1,295 s in the final ladder and 887 s in the first, came to more than the rungs took.

## Ratchets

| Ceiling | Before | After | Why |
| --- | --- | --- | --- |
| `sdlc/ratchet.json` (crates and conformance) | 66679 on main at the last merge | 66896 | +217: 88 source lines and 129 test lines, net of the deleted `exchange.rs` test and the moved helper |
| `databases/duckdb/ratchet.json` | 3828 | 3835 | the warm kind check and its sentence constant |
| `databases/postgresql/ratchet.json` | 1810 on main | 1817 | the warm kind check |

## Budgets

Nonblank lines against the branch point `3b6954d6`:

- `crates/thinkthen/src`: +88 net (budget 90).
- `crates/thinkthen/tests`: +129 net (budget 130). No test file crosses 500 nonblank lines.
- `databases/*/src`: +14 (budget 20). `databases/*/tools` and `check.sh`: +2 (budget 8).
- Pages: 10 lines changed (budget 16).

## Stop rules

None crossed. `cli/failure.rs`, `cli/asking.rs`, `core/records.rs`, every `public/*.rs`, `cli/audit.rs`, `specification/filter.md`, `rank.md`, `records.md`, and `sdlc/scripts` are unchanged. `cli/args/command.rs` changed in the `diff` help alone. No surface other than the command, DuckDB, and PostgreSQL pinned a changed sentence.

## Deferred gaps

As the ticket lists them. Items 1 and 16 are now unblocked, since 0134, 0135, and 0137 have landed.

## Closes, at landing

- `sdlc/issues/closed/2026-09-25-exchange-400-rows-belong-in-the-status-reason-table.md`: the lander moves it to `closed/`.
- `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`: this branch marks items 10 to 15 and 17 fixed and items 2 to 9 with their commits. It stays open for items 1 and 16.
- Item 5 of `sdlc/issues/closed/2026-09-25-test-harness-and-review-leftovers.md`: the coordinator marks it settled.
