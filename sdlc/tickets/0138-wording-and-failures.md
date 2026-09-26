---
flow: build
priority: 138
opens: crates/thinkthen/src/cli/failure/status.rs crates/thinkthen/src/cli/failure/recording.rs crates/thinkthen/src/cli/failure/recognize.rs crates/thinkthen/src/cli/mod.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/recognize/config.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/src/core/pointer.rs crates/thinkthen/src/core/question_file.rs crates/thinkthen/src/core/question_file/tests.rs crates/thinkthen/src/core/mod.rs crates/thinkthen/tests/backend/exchange.rs crates/thinkthen/tests/backend/status_reason.rs crates/thinkthen/tests/backend/support.rs crates/thinkthen/tests/backend/loopback_arms.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/backend/default_cache_storage.rs crates/thinkthen/tests/backend/pointer_echo.rs crates/thinkthen/tests/backend/recognize.rs crates/thinkthen/tests/backend/annotate.rs crates/thinkthen/tests/question_file/grammar.rs crates/thinkthen/tests/diff.rs databases/duckdb/src/tables.rs databases/duckdb/tools/verbs_suite.py databases/duckdb/README.md databases/duckdb/ratchet.json databases/postgresql/src/warm.rs databases/postgresql/check.sh databases/postgresql/README.md databases/postgresql/ratchet.json specification/backends.md specification/question-file.md specification/annotate.md specification/recording.md specification/recognize.md specification/choose.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0138: Command wording fixes, the retried statuses' next step, and pointers that never echo a control character

Status: landed 2026-09-26. Design review accepted on the second pass (fresh read-only Claude session). Code review accepted. Record: `sdlc/records/0138-build-wording-and-failures.md`. Owner: Claude.

Lane: thinkthen-lane-4

Review route: a fresh read-only Claude session reviews this design and the final diff. The change raises the size ceiling, so the code reviewer names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A user who meets any of six refusals reads a sentence that matches what happened and names a next step. Today each one points the wrong way: a bare status line after the retries, a missing file that is there, a key the file holds, a function the SQL user never called, a folder the user never named, and a kind count the user never broke. A pointer typed with a terminal escape is refused, and the escape never reaches the terminal. The `diff` help names its two pairing rules. The status sentences sit in one test table.

The authority is the coordinator's brief for 0138 and the backlog `sdlc/planning/issue-backlog-2026-09-25.md`, section A row 7. The ticket takes three issues:

1. `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`: every open item the ownership rules below allow.
2. `sdlc/issues/closed/2026-09-25-exchange-400-rows-belong-in-the-status-reason-table.md`. Ticket 0127 has landed, so it is unblocked.
3. Item 5 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. Ticket 0133 owns that file. This ticket does not edit it. The coordinator marks item 5 at landing.

Other in-flight tickets own some files. 0133 owns the rung scripts, `heavy-lock`, the children check, `cli/interrupt.rs`, and the harness issue file. 0134 owns the public library API and every `crates/thinkthen/src/public/*.rs`. 0135 owns audit, `core/measure`, and `cli/audit.rs`. 0137 owns the `filter` and `rank` framing and `specification/filter.md`, `rank.md`, and `records.md`. This ticket edits none of those. 0137's stop rule 4 stops it if another branch changes `cli/asking.rs`, `core/records.rs`, or `cli/failure.rs`, so this ticket leaves all three alone. It edits two kinds of file that 0137 or 0135 also list: `cli/args/command.rs`, in the `diff` help alone, and the ratchet files. Each edit sits in a hunk those tickets do not change. The coordinator orders the landings.

## Items checked against main

Checked at `origin/main` `3b6954d6` on 2026-09-26, by reading the source and running the built command.

| Item | State on main | This ticket |
| --- | --- | --- |
| 1. A `1e300` deadline prints 301 digits | open. The sentence is built in `crates/thinkthen/src/public/options.rs`, which 0134 owns | left (see Deferred gaps) |
| 2. Help lists admin commands first | fixed: `thinkthen --help` lists the ten functions first | marked settled by ticket 0126, commit `36929f63`, landed at `5f9eea8f` |
| 3. `relate --either` with a question file | fixed: `cli/relate/config.rs` names the file rule | settled by 0123, landed at `6d33e99f` |
| 4. relate help lists `--jobs` | fixed: `relate --help` has no `--jobs` | settled by 0123, landed at `6d33e99f` |
| 5. WidthActive says width | fixed: `engine/mod.rs` says throttle | settled by 0126, `36929f63` |
| 6. check prints the model it sent | fixed: `cli/check.rs` prints `model asked` and `unspecified` | settled by 0126, `36929f63` |
| 7. recognize `--dry-run` help | fixed: the help says what the plan prints | settled by 0126, `36929f63` |
| 8. recognize help has no cost sentence | fixed: the help opens with the paid requests | settled by 0126, `36929f63` |
| 9. `--jobs N` connections | fixed: the help says up to N connections | settled by 0126, `36929f63` |
| 10. diff help omits its pairing rules | open | fixed |
| 11. 502, 503, 504, 529 print no next step | open | fixed |
| 12. `annotate @set.json` reads as a missing file | open | fixed |
| 13. "holds no key `version`" | open | fixed |
| 14. `thinkthen_warm` names `decide_many` | open on DuckDB and PostgreSQL. SQLite already says `thinkthen_warm takes a decide question; ask others with thinkthen_decide` | fixed |
| 15. The default cache is called the recording folder | open | fixed |
| 16. "unresolved" names the not-sure answer | open. The printed line is in `cli/audit.rs` (0135), the public docs in `public/*.rs` (0134), and page prose in `specification/audit.md` (0135) and `filter.md` and `records.md` (0137) | left (see Deferred gaps) |
| 17. `recognize --kind PER` blames the kind count | open | fixed |

## Design

### Status 502, 503, 504, and 529 (item 11)

`cli/failure/status.rs` gives 502, 503, 504, and 529 the 500 phrase, held once as a constant: `the backend failed after the allowed attempts; try again later or change --max-retries`. These are the retried statuses of `engine/error.rs` `RETRIED`, less 429, which keeps its rate-limit phrase. `check` prints the same line, because it reads the same table. The PHRASES table grows from 8 to 12 rows. The table in `specification/backends.md` names the four statuses on its 500 row. The library and SQL surfaces keep their own bare status line, and the page gains a sentence that says so. `databases/sqlite/tests/test_deadline.py` pins that line for 503, and it does not change.

### One status table (issue 2)

The `decide` helper in `tests/backend/exchange.rs` moves to `tests/backend/support.rs`, unchanged. `status_reason.rs` calls it and drops its own argument list and `spawn`. Its table takes the 500 row from `exchange.rs` and new rows for 502, 503, 504, and 529. Each row runs with `--max-retries 0`, pins the exact standard error, exit 4, empty standard output, and one request. The 400 rows it already holds stay. The `exchange.rs` test `common_request_statuses_name_fixed_actions_and_hide_the_body` is deleted. The table's marked-body row already proves the body stays hidden. The `503` row of `loopback_arms.rs` `each_wire_fault_arm_yields_its_kind_and_sentence` pins the new sentence too, because it pins the whole line today.

### diff help (item 10)

The `diff` help gains two sentences after its first paragraph: `diff pairs answers by record id and answer name only.` and `It compares question digests only when both runs saved --details.` `specification/diff.md` already states both rules, so it does not change.

### `annotate @FILE` (item 12)

`annotate` accepts `@FILE` as the same file as `FILE`, as `decide`, `choose`, `score`, `tag`, `relate`, and the SQL surfaces already do. A leading `@` is stripped before the open. `./@name` still reaches a file whose name starts with `@`. `specification/annotate.md` says so.

### A question file takes no key (item 13)

`QuestionFileError::UnknownKey` reads `a question file takes no key `K``. It matches the existing `a `choose` question file takes no key `true``. When the key is `version`, the sentence adds where it belongs: `a question file takes no key `version`; `version` belongs in a question set, a recognize file, or a relate file`. The key keeps its JSON escapes. `specification/question-file.md` quotes both sentences.

### `thinkthen_warm` names itself (item 14)

DuckDB's `Warm::finish` and PostgreSQL's warm `finalize` refuse a question that is not `decide`, before any request, with SQLite's sentence: `thinkthen_warm takes a decide question; ask others with thinkthen_decide`. A banded decide question still passes. Each surface prints it with its own prefix, `thinkthen usage: `. The DuckDB and PostgreSQL READMEs say so, as SQLite's already does.

### The default cache's storage sentence (item 15)

The default cache folder gets its own sentence at exit 5: `the default cache folder could not be read or written; check its permissions and free space, use --no-cache, or set THINKTHEN_CACHE to another folder`.

`cli/mod.rs` `entry` passes each failure of `run` through one new function, `told`, before it reports it. `told` rewrites a `RecordingStorage` failure to `Failure::Configuration` with that sentence when the command's folders are the platform default cache. The test for the default is `Folders::of(..).private_default`. When `Folders::of` fails, as with `TwoFolders`, `told` keeps the original failure. Ticket 0124 already splits the backend mismatch sentence by that flag. `Configuration` is the existing variant that prints a fixed local sentence at exit 5, so `cli/failure.rs` does not change. A stopped run whose cause is `RecordingStorage` prints only the storage sentence today (`stopped` in `cli/failure.rs`). `told` rewrites that whole stopped run to the same `Configuration` failure, so the output stays one line at exit 5.

`told` needs the command's shared options. `cli/mod.rs` gains a private `common` function, one match over the ten record commands, beside `entry`. `Command` in `cli/args/command.rs` gains no accessor, because 0135 and 0137 both list that file.

The engine error does not change, because its public mapping lives in `public/error.rs`, which 0134 owns. The file-size claim at the start of `entry` reports its own storage failure before any folder is known, so it keeps the recording-folder sentence. `specification/recording.md` quotes both sentences.

### `--kind` without `=` (item 17)

`cli/failure/recognize.rs` gains `KindWithoutSign`, at exit 2: `--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind`. `command_kinds` returns it when a `--kind` entry holds no `=`. It follows the `--option` and `--label` sentences. `specification/recognize.md` quotes it.

### A pointer holding a control character (harness item 5)

`Pointer::new` refuses a pointer that holds a control character, as `char::is_control` defines it, with `PointerError::Control`: `a pointer is one line of printable text`. The check runs first. The labels use the same test and the same words.

Two sentences echo a refused pointer as it was typed. Both now write it with JSON escapes through `safe_key`, which the unknown-key sentence already uses:

- `QuestionFileError::Pointer` in `core/question_file.rs`, for `--field` on the four question verbs and for a file's `on`.
- `Failure::Pointer`, for `--field` on `find` and `annotate` and `--options` on `choose`. Its format sits in `cli/failure.rs`, which this ticket leaves alone. `told` in `cli/mod.rs` escapes its typed text instead, the same way. The pointer failure never runs inside a stopped run, so `told` sees every one.

A pointer that passes can no longer hold a control character. So `the record holds nothing at` in `core/records.rs` never echoes one. It still echoes a quotation mark or backslash as typed, and that file belongs to 0137. `specification/question-file.md` (the evidence row) and `specification/choose.md` (`--options`) state the refusal and the escapes. The rule in `specification/records.md` belongs to 0137's page and is left.

### What reaches the library and SQL surfaces

`Pointer::new`, `QuestionFileError`, and the unknown-key sentence are core, so every surface that reads a question file shares them. A question file whose `on` holds a control character is now refused on Python, R, Ruby, TypeScript, C, DuckDB, PostgreSQL, and SQLite. So is a relate file's `fields` pointer and a recognize file's pointer. The escaped pointer echo and the `takes no key` sentence print there too. A search of `libraries/`, `databases/`, `conformance/`, `spec/`, `demos/`, and `site/` on 2026-09-26 found no test that pins `holds no key`, a pointer with a control character, or a raw pointer echo. The `surfaces` rung runs to show it.

## Decisions

1. **Leave item 1 and item 16.** Each needs a file another in-flight ticket owns. The coordinator's brief says to stop and report such a fix, so the ticket names them and does the rest.
2. **One next step for every retried server status.** 502, 503, 504, and 529 take the 500 phrase. A user reads the same advice for the same situation.
3. **Accept `@FILE` in annotate.** The other choice refuses a leading `@` with its own sentence. Accepting matches every other file-taking verb and the SQL surfaces, and costs two lines.
4. **"takes no key".** The sentence names the key as not accepted, in the words the verb-key sentence already uses. The `version` sentence names the three files that take it.
5. **SQLite's warm sentence everywhere.** SQLite already pins it (`databases/sqlite/tests/test_values.py`). The three database surfaces then read alike.
6. **Map the storage failure at the command edge, onto the existing `Configuration` variant.** The engine cannot say default-cache without a new engine error variant, and that variant must reach `public/error.rs`. The command already knows whether its folder is the default. A new `Failure` variant would change `cli/failure.rs`, and 0137 stops on any change there.
7. **Escape the whole typed pointer in both refusal sentences.** A pointer with a quotation mark or backslash now reads with a JSON escape there too, as unknown keys already do. `the record holds nothing at` keeps its echo, since it can no longer hold a control character.
8. **Refuse the control character rather than only escape it.** The issue asks for the refusal, and labels, options, and table headers already refuse control characters.

## Edge cases

| Input | Standard error | Exit |
| --- | --- | --- |
| Status 500, 502, 503, 504, or 529 with `--max-retries 0` | `thinkthen: the backend answered with status N: the backend failed after the allowed attempts; try again later or change --max-retries` | 4 |
| Status 429 after the retries | unchanged: `...status 429: the backend's rate limit was reached` | 4 |
| Status 418 | unchanged: `thinkthen: the backend answered with status 418` | 4 |
| `annotate @set.json`, `set.json` present | answers as `annotate set.json` does | 0 |
| `annotate @absent.json` | `thinkthen: the question set could not be opened: No such file or directory (os error 2)` | 5 |
| A question file with `"version":1` | `thinkthen: a question file takes no key `version`; `version` belongs in a question set, a recognize file, or a relate file` | 5 |
| A question file with `"nope":1` | `thinkthen: a question file takes no key `nope`` | 5 |
| A question file with key `line\nbreak` | `thinkthen: a question file takes no key `line\nbreak`` on one line | 5 |
| DuckDB `thinkthen_warm('@c.json', t)`, a choose file | `thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide`, 0 sends | error |
| PostgreSQL `thinkthen_warm('{"choose":...}', body)` | the same sentence, then PostgreSQL's ` (retryable: no)`, 0 sends | error |
| Warm with a banded decide file | unchanged: answers | ok |
| No `--cache`, `--record`, `--replay`, or `THINKTHEN_CACHE`; the default cache folder fails | `thinkthen: the default cache folder could not be read or written; check its permissions and free space, use --no-cache, or set THINKTHEN_CACHE to another folder` | 5 |
| `--cache DIR` or `THINKTHEN_CACHE=DIR`, and `DIR` fails | unchanged: `thinkthen: the recording folder could not be read or written; check its permissions and free space` | 5 |
| `recognize --kind PER`, or `--kind PER --kind ORG` | `thinkthen: --kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind` | 2 |
| `recognize --kind PER=A person. person` (mixed) | unchanged: `recognize takes 1 to 20 distinct, nonblank kinds` (deferred gap) | 2 |
| `decide --jsonl --field $'/a\e[31m'` | `thinkthen: --field `/a\u001b[31m`: a pointer is one line of printable text` | 2 |
| `find --field $'a\e'` or `choose --options $'/a\e'` | `thinkthen: --field `a\u001b`: ...` or `--options `/a\u001b`: ...`, same reason | 2 |
| `decide --field '$.body'` | unchanged: `thinkthen: --field `$.body`: a pointer is RFC 6901, so it is empty or begins with `/`` | 2 |
| `decide --field 'a"b'` | `thinkthen: --field `a\"b`: a pointer is RFC 6901, ...` (was `a"b`) | 2 |

## Proof

Every test runs the built command or the built extension. None needs a test-only export, flag, or hook.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `status_reason.rs` `a_status_names_its_fixed_action_and_only_the_known_reason` (renamed from `a_400_names_only_the_known_reason_from_a_bounded_body`) | The table's rows: the six 400 rows, 422, 500, 502, 503, 504, and 529. Each pins the whole standard error, exit 4, and 1 request | P1: drop the 503 row from PHRASES. The 503 row reads the bare line |
| `loopback_arms.rs` `each_wire_fault_arm_yields_its_kind_and_sentence` | Its `503` row pins the new sentence after 3 requests | P1 also turns it red |
| `diff.rs` `help_names_diff_and_says_what_it_never_does` | Two more pinned sentences | P2: drop the digest sentence from the help |
| `backend/annotate.rs` `a_question_set_named_with_at_is_the_same_file` (new) | `annotate @set.json --dry-run` and `annotate set.json --dry-run` print the same plan, byte for byte, at exit 0. A dry run reads no key and opens no cache. `annotate @absent.json` exits 5 with the open sentence | P3: remove the strip. The `@set.json` run exits 5 |
| `question_file/grammar.rs` `every_refusal_the_grammar_makes_names_its_key_and_its_exit_code` | The `nope` row pins `takes no key`. A new `version` row pins the `version` sentence | P4: restore `holds no key`. P5: drop the `version` clause |
| `question_file/grammar.rs` `an_unknown_local_key_is_escaped_and_stays_on_one_line`, and the unit test in `core/question_file/tests.rs` | Updated to the new wording | P4 |
| DuckDB `verbs_suite.py` `r2_22_warm_judges_one_question_per_group` | The choose-file row pins the warm sentence. `backend.count()` stays 0 | P6: remove the DuckDB kind check. The row reads the `decide_many` sentence |
| PostgreSQL `check.sh` `bad_files_name_themselves` | A new inline choose row pins the warm sentence. `bcount` stays 0 | P7: remove the PostgreSQL kind check |
| `default_cache_storage.rs` `a_default_cache_that_fails_names_the_default_cache` (new file, because `default_cache.rs` holds 460 of the lint's 500 nonblank lines) | `XDG_CACHE_HOME` holds a private `thinkthen` folder with a malformed identity marker. `decide` exits 5 with the default-cache sentence and 0 requests. The same folder named by `THINKTHEN_CACHE` prints the recording-folder sentence. `cache_identity.rs` already pins `--cache`. A second case runs `--jsonl` over two records against the same folder, so the failure comes as a stopped run, and pins the same single line | P8: skip the mapping in `told`. Both default runs read the recording-folder sentence |
| `backend/recognize.rs` `a_kind_without_a_sign_names_the_sign` (new) | `--kind PER` and `--kind PER --kind ORG` each exit 2 with the sentence and 0 connections | P9: return the kinds error again |
| `pointer_echo.rs` `a_pointer_holding_a_control_character_is_refused_and_never_echoed` (new file, because `refusals.rs` holds 469 of the lint's 500 nonblank lines) | `decide --field`, `find --field`, and `choose --options`, each with an escape byte, exit 2 with the exact escaped sentence and 0 connections. A question file whose `on` holds an escape exits 5 with `the question file's `on` `/a\u001b`: a pointer is one line of printable text` | P10: drop the control check. `decide` reaches `the record holds nothing at` with the raw byte. P11: drop the escaping in `told`. The `find` and `choose` rows print the raw byte. P12: drop the escaping in `QuestionFileError::Pointer`. The `decide` and file rows print it |

The four questions for each new or changed test:

- **What behavior does it protect?** The status table protects the next step after every retried server status and the one home of the status sentences. The diff help test protects the two pairing rules a stranger reads. The annotate test protects `@FILE` as the same file. The grammar rows protect a refusal that names the key as not accepted. The warm rows protect a refusal that names the function the SQL user called, with no send. The default-cache test protects a sentence that names the folder the user did not name, and the way around it. The kind test protects the cause of a `--kind` refusal. The pointer test protects the terminal from a typed escape on every echo path.
- **What credible regression fails it?** P1 to P12. P1, P4, P6, P7, P9, and P10 are today's code. P3 and P8 are today's behavior. P2, P5, P11, and P12 are the easy slips.
- **Why does no existing test catch it?** Today's 503 row pins the bare line, and no test pins 502, 504, or 529. The diff help test pins no pairing rule. No test runs `annotate @FILE`, a question file with `version`, a failing default cache, `--kind` without `=`, or a pointer with a control character. The DuckDB row pins the old sentence, and PostgreSQL has no row.
- **Does it need a test-only hook?** No. Each reaches the command line, the loopback backend, or the SQL surface's own check. The default-cache test uses `XDG_CACHE_HOME`, which the default-cache tests already use.

## Budgets

Nonblank lines, measured with `grep -c .` against `origin/main`.

- `crates/thinkthen/src`: at most 90 added, net.
- `crates/thinkthen/tests`: at most 130 added, net of the removed `exchange.rs` test and the moved helper. No test file crosses the lint's 500 nonblank lines.
- `databases/*/src`: at most 20 added. `databases/*/tools` and `check.sh`: at most 8 added.
- Pages (`specification/`, the two database READMEs): at most 16 lines changed.
- `sdlc/ratchet.json` and `databases/duckdb/ratchet.json` and `databases/postgresql/ratchet.json` move to the measured totals in the commit that changes the code. The commit says what grew and where the builder looked for duplication to delete first.

## Stop rules

1. Stop if a fix needs a file listed under another in-flight ticket in "Outcome and authority", beyond the `diff` help hunk and the ratchet files named there.
2. Stop before crossing a budget.
3. Stop if any plant stays green.
4. Stop if the default-cache mapping needs an engine or public API change.
5. Stop if a surface other than the command, DuckDB, and PostgreSQL pins a changed sentence.

## Scope and exclusions

Excluded: items 1 and 16, the wording in `filter` and `rank` help, `specification/filter.md`, `rank.md`, and `records.md`, `cli/failure.rs`, `cli/asking.rs`, `core/records.rs`, the public library API, and every library surface's own code. The core changes reach the library surfaces as "What reaches the library and SQL surfaces" says. No live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Routing

Builder: Claude (Opus subagent) in lane 4. Reviewer: a fresh read-only Claude session for design and for code. The code review names what it checked for the ceiling raise.

## Complexity

Contract 2; state and timing 1; reach 3; proof 2; cost of error 1; total 9. Final level: 2. The reach is wide, over nine refusals on three surfaces. Each change is a sentence or a check before a send.

## Deferred gaps

1. Item 1, the `1e300` deadline digits. The sentence lives in `crates/thinkthen/src/public/options.rs`, which 0134 owns. The fix is `{value:e}` past the cap, and the Python test `libraries/python/tests/test_inputs.py` updates with it. It can follow 0134's landing as a Quick Fix.
2. Item 16, "unresolved". Its printed line and pages sit in files 0134, 0135, and 0137 own. It also needs a recorded choice on the JSON key `unresolved`. It should be its own ticket after those three land.
3. `the question set holds no key `K`` keeps its wording. A question set takes `version`, so it never misreads the way item 13 did.
4. Mixing bare kinds with `--kind` still prints the kind-count sentence.
5. Status 429 after the retries names the rate limit and not `--max-retries`.
6. The unit test `common_request_statuses_give_fixed_actions` in `cli/failure/tests.rs` repeats the 400 and 500 rows. That file is in 0137's list, so it stays.
7. The `annotate` help does not mention `@FILE`. The positional's help text lives in `cli/args.rs`, which 0137 lists.
8. `the record holds nothing at` echoes a quotation mark or backslash as typed. It lives in `core/records.rs`, which 0137 owns.
9. The library and SQL surfaces keep the bare status line after the retries. Their errors come from `public/error.rs`, which 0134 owns.
10. `told` escapes `Failure::Pointer` at the command edge. When 0137 lands, the escape can move into the variant's own format in `cli/failure.rs`.
11. `in_default_cache` in `cli/mod.rs` repeats the ten-arm command match of `Command::input`. Once 0135 and 0137 land, both can share one `Command` accessor in `cli/args/command.rs`.
12. No test pins the escape of a quotation mark or backslash in a pointer echo. P11 and P12 use control characters. A row would cost the test budget's last lines.

## What Ian can overturn

- Decision 1: leaving items 1 and 16 for after 0134, 0135, and 0137 land.
- Decision 2: one phrase for every retried server status.
- Decision 3: `@FILE` in annotate is accepted. The other choice refuses it with its own sentence.
- Decision 6: the default-cache sentence is chosen at the command edge. The other choice is an engine error variant through the public API.
- Decision 7: both pointer refusal sentences JSON-escape the typed pointer, quotes and backslashes included.

## Closes

- `sdlc/issues/closed/2026-09-25-exchange-400-rows-belong-in-the-status-reason-table.md`. The lander moves it to `closed/` with a status line naming this ticket.
- `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`: items 10 to 15 and 17 marked fixed by this ticket, items 2 to 9 marked with their commits. It stays open for items 1 and 16. This ticket edits it.
- Item 5 of `sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md`. 0133 owns that file, so the coordinator marks it at landing.

## Evidence

- Starts from: The wording issue's items, checked against `origin/main` `3b6954d6` by reading the source and running the built command. Experiment 218 wave 2, which found items 10 to 17 with a fake key against a loopback backend. The 400-rows issue from the 0123 code review. Harness item 5. SQLite's warm sentence and its test. Ticket 0124's split of the backend mismatch sentence by `private_default`. The `--option` and `--label` sentences. The label control-character check in `core/question.rs`.
- Keeps: Every exit code. The 400, 422, 429, and 418 sentences. The recording-folder sentence for a named folder and for the file-size claim. The stopped-run rule that prints only the storage sentence. The engine and public API error variants, and the library surfaces' bare status line. Warm on a banded decide file. The `$.body` pointer refusal and every other pointer reason. `cli/failure.rs`, `cli/asking.rs`, and `core/records.rs`.
- Changes: Six refusal sentences, two help sentences, `@FILE` in annotate, a pointer control-character refusal, JSON escapes in both pointer refusal sentences, and one status table in place of two. The pointer refusal and the question-file sentences reach every surface that reads a question file, and a search found no surface test that pins them.
- Proof: The twelve plants in "Proof", each turning its test red, over the status table, the diff help, annotate, the question-file grammar, DuckDB, PostgreSQL, the default cache, recognize, and the pointer refusals.
- Defers: Items 1 and 16, the question set's unknown-key wording, the mixed-kinds sentence, the 429 phrase, the duplicate unit test in `cli/failure/tests.rs`, the annotate help, the quote echo in `core/records.rs`, the surfaces' bare status line, and moving the pointer escape into `cli/failure.rs`.
