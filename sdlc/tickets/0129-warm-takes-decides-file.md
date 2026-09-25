---
flow: build
priority: 129
opens: databases/sqlite/src/scalars.rs databases/sqlite/tests/test_files.py databases/sqlite/tests/test_values.py databases/sqlite/README.md databases/sqlite/ratchet.json databases/duckdb/src/tables.rs databases/duckdb/src/tables/ffi.rs databases/duckdb/src/questions.rs databases/duckdb/src/questions/ffi.rs databases/duckdb/src/connections.rs databases/duckdb/src/connections/ffi.rs databases/duckdb/src/ffi.rs databases/duckdb/tools/verbs_suite.py databases/duckdb/tools/settings_suite.py databases/duckdb/tools/databases_suite.py databases/duckdb/tools/source_checks.py databases/duckdb/README.md databases/duckdb/NOTES.md databases/duckdb/ratchet.json databases/postgresql/check.sh sdlc/records sdlc/tickets sdlc/issues
---

# 0129: Warm takes the question file decide uses

Status: in progress, design under review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user writes a question once in a file and uses the same `'@file'` in every SQL call. `thinkthen_warm('@abbey.json', title)` fills the cache. A later `thinkthen_decide('@abbey.json', title)` over the same rows sends nothing. This holds on SQLite, DuckDB, and PostgreSQL, the only surfaces with a warm call. It holds when the file carries a band, such as `"threshold": "0.3:0.7"`.

The ask is `sdlc/issues/2026-09-25-warm-refuses-the-question-file-decide-uses.md`. The backlog of 2026-09-25 does not list it, because it was filed after the backlog was written. The coordinator assigned it on 2026-09-25.

## What is wrong today

The public API is not the fault. `Engine::decide_many_with` already takes a `BandedQuestion`. The engine builds a request plan from the question alone and applies the threshold only when it reads the answer (`crates/thinkthen/src/engine/facade.rs`, `judge`: `Plan::new` takes no threshold, then `answer.read(threshold)`). `specification/channels.md` line 108 says the same for the request body. The request digest and the cache entry are therefore the same with or without the band. Each database has its own gate in front of that call.

| Surface | Today | Where |
| --- | --- | --- |
| SQLite | A banded question, from a file or inline JSON, reads `thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide` | `warm_question` calls `only`, which refuses every `LoadedQuestion::Banded`. `flush` calls `plain`, which treats a band as a defect |
| DuckDB | Any `'@file'` reads `thinkthen usage: thinkthen_warm reads no '@file' question; pass the file's text from DuckDB's read_text, which applies this database's file settings`. Inline banded JSON already works | `Warm::finish` in `src/tables.rs`. The aggregate has no client-context accessor in the C API (ticket 0110 decision 10, "Warm") |
| PostgreSQL | Works. Warm parses `'@file'` through `crate::question` and sends through `decide_distinct`, which matches both arms. No test covers a banded file in warm | `src/warm.rs` finalize, `src/lib.rs` `decide_distinct` |

The libraries have no warm call. `sdlc/issues/2026-09-25-public-library-api-gaps.md` item 6 owns whether they gain one.

## Design

### The public API gains nothing

The fix belongs in each database, not in `crates/thinkthen`. The engine already does the right thing with a banded question, and each fault is a surface's own gate. A public helper such as `impl DecisionQuestion for LoadedQuestion` would fold the two-arm `match` that each surface repeats. It widens the public inventory, and it settles nothing this issue names. It goes to `sdlc/issues/2026-09-25-public-library-api-gaps.md` as a new item, which is in 0.1, so the owner of that issue decides it with the other public API gaps.

### SQLite

`warm_question` takes any decide question, cut or banded. Every other kind keeps today's refusal sentence and sends nothing. `flush` matches both arms of `LoadedQuestion` and calls `decide_many_with` on each, as `decide` already does. `plain` keeps its other callers. The README's line "A banded decide question goes to `thinkthen_decide` and `thinkthen_details` only" gains warm, and says warm ignores the band.

### DuckDB

The warm aggregate reads `'@file'` through the kept connection of the database it was registered in. Ticket 0118 keeps one connection per loaded database. Each LOAD registers the aggregate into that one database's catalog, so a warm call always comes from the database it was registered in.

1. **A serial per kept entry.** `connections` gains a process-wide counter. LOAD takes the next serial first, passes it to `register_all` and so to `register_warm`, and then calls `connections::register(database, serial)` where that call sits today. DuckDB connection ids repeat across databases, so the serial names the entry, not the connection id. A LOAD whose `register_all` fails spends a serial and registers no entry, as today.
2. **The serial rides on the aggregate.** `register_warm` stores the serial as the aggregate's extra info, a boxed `u64` with a destroy callback. Every `unsafe` line stays in `src/tables/ffi.rs`, `src/questions/ffi.rs`, and `src/connections/ffi.rs`.
3. **The read runs under the database gate.** `Warm::finish` takes the serial. A question that starts with `@` runs these steps in this order, in one scope:
   1. `connections::by_serial` returns a counted `Kept` guard, as `for_caller` does, or nothing.
   2. The warm waits for the database gate through `Kept::gate`, with a stop closure (decision 4).
   3. Inside the gate, it builds `Files` from `duckdb_connection_get_client_context` on the kept connection, through a new constructor in `src/questions/ffi.rs`. It reads the file through the code that `Caller::named_file` runs today. That code moves into a free function over `Files`, so both callers print the same sentences. The text parses with `Question::from_json`, and errors map through `from_file`.
   4. `Files` drops first, then the gate, then the guard. The worker starts after all three are gone.
4. **Nothing else moves.** The inline path, the engine choice, and the one-question-per-group rule do not change. Warm still runs on the engine `EngineBuilder::from_env()` builds.

The kept connection is a second client context on the same database. Decision 10 asks DuckDB's own settings to decide every file read. `enable_external_access`, `allowed_directories`, `allowed_paths`, and `disabled_filesystems` are database-wide settings in DuckDB, so the kept connection's file system should refuse what the caller's refuses. `home_directory` and `file_search_path` are per session, so a `~` path or a relative path could read differently. The spike (stop rule 1) measures every one of these after design steps 1 to 3, before the tests and pages.

### PostgreSQL

No source change. A test pins the behavior that already works.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **Warm takes the band and ignores it.** Warm returns a count, not answers, so the band changes nothing it reports. The cache entry is the same, so decide later applies the band when it reads. The issue's other choice, keeping the refusal and naming the fix, still makes the user type the question twice.
2. **The public API stays as it is.** See "The public API gains nothing".
3. **DuckDB reads `'@file'` through the kept connection.** Other options, weighed:
   - Keep the refusal and name the `read_text` form. This costs no code, but the user still writes the question two ways. It is the fallback under stop rule 1.
   - Open the file with `std::fs`. Experiment 253 (D2) found that copying DuckDB's file rules is unsound, and decision 10 removed that route.
   - Turn warm into a table function or a scalar, which have a client context. This changes warm's shape on one surface only and breaks every query that uses it.
   - The kept connection costs about 70 lines and reuses 0118's registry, guard, and gate.
4. **Warm never waits on a running relate query.** Relate holds the gate for its whole query and marks the kept connection busy. A warm with `'@file'` inside a relate query would wait on a gate its own outer query holds, and neither would finish. Warm cannot tell a nested relate from a relate on another connection, because DuckDB may run the aggregate on any worker thread. The stop closure therefore refuses at once whenever the kept connection is busy, and on Ctrl-C it reads relate's cancel sentence. Other gate holders are the reaper's pass and other warm reads, which hold it for one file read or one count query, so waiting on them is bounded. The refusal reads `thinkthen usage: thinkthen_warm cannot read '@file' while a relate query runs on this database; run it before or after the relate, or pass the file's JSON text` and sends nothing. A relate query that calls warm with `'@file'` therefore fails with that sentence, never hangs. The better fix, a second kept connection used only for file reads, would change the reaper's count of 1 and 0118's release rule. It is not taken.
5. **A serial names the kept entry.** The aggregate's extra info carries it. A caller never routes by a database name or a connection id, as 0118 already rules.
6. **A released kept connection names a fix that works.** The reaper releases a kept connection once its database has no other connection. A C or Rust host can hold a database handle with no connection, then connect again. After that, warm finds no entry. A second `LOAD` does nothing, because DuckDB skips an extension that database already loaded. The warm refusal reads `thinkthen usage: thinkthen_warm cannot read '@file' here, because this database's kept connection was released when its last connection closed; reopen the database and LOAD the extension, or pass the file's JSON text`, and sends nothing. 0118's relate sentence makes the same wrong claim, so this ticket rewrites it to `thinkthen usage: this connection's database answers no loaded identity probe, so relate cannot find its own connection; reopen the database writable and LOAD the extension, since a read-only database cannot carry a probe and a released one lost it`. The test pinning it in `tools/databases_suite.py` changes with it.
7. **`~` paths and relative paths, decided before the spike.** If the spike finds that `SET home_directory` makes decide and warm read different files for `'@~/q.json'`, warm refuses every `'@~'` path with `thinkthen usage: thinkthen_warm cannot read an '@~' path, because home_directory is a session setting it cannot see; write the full path` and sends nothing. If `SET file_search_path` makes them read different files for a relative path, the README states that warm reads a relative path from the process's working folder, and the record names the case. Refusing every relative path would break the issue's own `'@abbey.json'`. If both readings agree, nothing changes.
8. **The fallback is decided now.** If stop rule 1 trips on any case decision 7 does not cover, the build takes decision 3's first option on DuckDB and records the failing cases. The DuckDB refusal then names the `read_text` form and says warm takes the band. SQLite and PostgreSQL land either way. Ian can overturn the fallback.
9. **SQLite's refusal for other kinds keeps its sentence.** Rewording it belongs to the wording batch, and this ticket changes no other help text.
10. **PostgreSQL gets a test and no code.** It already takes the file and the band.

## Edge cases

Evidence is the generic loopback arm, which answers yes at 0.9. A band of `0.85:0.95` therefore reads unsure, which makes the band visible when decide reads the warmed entry.

| Input to warm | SQLite | DuckDB | PostgreSQL |
| --- | --- | --- | --- |
| `'@file'` holding a banded decide question with `true`, `false`, and `model` | Takes it. Changed | Takes it. Changed | Takes it. Kept |
| The same question as inline JSON text | Takes it. Changed | Takes it. Kept | Takes it. Kept |
| `'@file'` holding a cut decide question | Takes it. Kept | Takes it. Changed | Takes it. Kept |
| A choose, tag, or score question | Today's refusal sentence, 0 sends. Kept | The engine's refusal `thinkthen usage: decide_many does not take a choose question`, 0 sends. Kept for inline text, and now the same for `'@file'` | Today's refusal. Kept |
| A missing `'@file'` | Today's sentence. Kept | `thinkthen local: the question file PATH was not read: it does not exist or could not be opened`, 0 sends, as decide reads. Changed | Today's sentence. Kept |
| `'@file'` that the database's file settings refuse | Not applicable | `thinkthen local: the question file PATH was not read: this database's file settings refuse it`, 0 sends, as decide reads. Changed | Today's file gate. Kept |
| Two databases in one process, B with `enable_external_access = false` | Not applicable | A's warm reads the file. B's warm reads the refusal. Changed | Not applicable |
| `SET home_directory` in the caller's session, then `'@~/q.json'` | Not applicable | As decide reads it, or decision 7's refusal. Decided by the spike | Not applicable |
| `'@file'` warm inside a relate query | Not applicable | Decision 4's sentence, 0 sends, the relate query fails with it. New | Not applicable |
| `'@file'` warm while a relate query runs on another connection of the same database | Not applicable | Decision 4's sentence if the relate query is running, else the file reads. Neither hangs. New | Not applicable |
| The kept connection was released | Not applicable | Decision 6's sentence, 0 sends. New | Not applicable |
| Decide with the same file after warm | 0 sends, rows read NULL under the band | 0 sends, rows read NULL under the band | 0 sends |

## Proof

Each test counts requests at the loopback backend, never through `--dry-run` or `thinkthen_usage`. Each lives in the surface's existing suite and runs through its existing harness, so this ticket adds no harness code. A new test passes the environment only through the harness's `extra=` argument or `environment()`.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| SQLite `test_warm_takes_the_banded_file_decide_uses`, in `tests/test_files.py` | A file holds `{"decide":"Is it red?","true":"Red paint.","false":"Any other colour.","model":"judge-b","threshold":"0.85:0.95"}`. Warm over three rows, two distinct, reads `[[2]]`, and the backend counts 2. Decide with the same `'@file'` over the three rows reads `[[None]]` three times, and the backend still counts 2. The same warm with the file's JSON as inline text reads `[[2]]` and adds 0 | Restore `only` in `warm_question`: warm reads the refusal. Build warm's question from the file's `decide` text alone, as the user's workaround does: decide counts 2 more sends |
| SQLite kind refusal, in `tests/test_values.py` `test_a_banded_question_is_refused_where_it_has_no_answer` | The test sends its banded question to score, choose, and tag, and today to warm. Warm now takes a banded question, so its entry changes to a choose question file's JSON, and it pins today's warm refusal sentence with 0 sends | Let warm take any kind: the engine refuses the choose question with another sentence, and the pinned sentence fails |
| DuckDB `r2_22_warm_takes_the_banded_file_decide_uses`, in `tools/verbs_suite.py` | The same file and rows under `THINKTHEN_CACHE`. Warm reads 2, the backend counts 2. Decide with `'@file'` reads NULL for each row, and the count stays 2 | Restore the `@file` refusal: warm reads it. Send `'@file'` down the inline path, so warm asks the path text as a plain question: decide counts 2 more sends |
| DuckDB kind refusal, folded into `r2_22_warm_judges_one_question_per_group` | The old `'@q.json'` refusal assertion leaves. In its place, a choose question file in warm pins `thinkthen usage: decide_many does not take a choose question`, and the count stays 0 | Skip the engine's kind check by building a decide question from the file's text: the warm sends |
| DuckDB access cases, in `tools/settings_suite.py` `access_cases_match_duckdb` | Each of the 35 `@file` cases also runs `thinkthen_warm('@PATH/q.json', 'refund now')`. Each warm result matches `read_text` on the same file, and each refused one adds 0 sends. Stop rule 1's `home_directory` and `file_search_path` cases join the same test, pinned to decision 7's outcome | Read the file with `std::fs`: the `external_off` and `local_disabled` cases read the file where `read_text` refuses |
| DuckDB two databases, in `two_databases_each_judge_their_own_access` | After A's warm reads `'@q.json'`, B's warm reads the refusal sentence | Look up the first registry entry in place of the serial: B's warm reads the file through A's connection |
| DuckDB nested relate, in `tools/databases_suite.py` | A relate whose query calls `thinkthen_warm('@q.json', name)` reads decision 4's sentence within the suite's timeout, and the backend counts 0 | Wait on the gate with no busy check: the query hangs and the suite's timeout fires |
| DuckDB concurrent relate and warm, in `tools/databases_suite.py` | Two connections to one database. One runs 50 relate queries over a slow `SELECT` on `range`. The other runs 50 warms with `'@q.json'` at the same time. Each warm reads 1 or decision 4's sentence, never anything else. Each relate answers. Both loops end within the timeout, and no result reads `defect` | Build `Files` outside the gate: a read overlaps the relate query on the kept connection. If that shows no failure, the plant is instead to drop the gate wait's stop closure, and the loops hang. The record names which plant ran |
| DuckDB released connection, in `tools/databases_suite.py` beside `r3_1_a_bound_relate_outlives_a_forced_reaper_pass` | In the `test-hooks` build, `thinkthen_test_hook_reap()` releases the kept connection. Warm with `'@q.json'` then reads decision 6's sentence, and the backend counts 0. The forged-relate test pins decision 6's new relate sentence | Treat a missing entry as the inline path: warm reads the path as plain question text and sends 1 |
| PostgreSQL `warm_takes_the_banded_file_decide_uses`, in `check.sh` beside `warm_then_decide_sends_nothing` | The fixture `@refund.json`, which carries the band `0.2:0.8`, `true`, and `false`. Warm over 20 distinct rows reads 20, `bcount` reads 20. Decide with `'@refund.json'` over the same rows counts 20 true, and `bcount` stays 20 | Build warm's question from the file's `decide` text alone: `bcount` reads 40 |

The four questions for each new test:

- **What behavior does it protect?** Warm fills the entries that decide with the same file reads, band and all. The DuckDB access rows protect decision 10: DuckDB's own settings decide warm's reads. The relate rows protect against a hang.
- **What credible regression fails it?** The plants above. The "question from the `decide` text" plant is the user's own workaround today, and it misses the cache whenever the file carries a model or criteria.
- **Why does no existing test catch it?** SQLite and DuckDB pin warm's refusals. PostgreSQL's `warm_then_decide_sends_nothing` uses inline plain JSON, and its file tests pin only a missing file and a broken file. No test runs warm and relate against one kept connection.
- **Does it need a test-only hook?** Only the released-connection row, which uses the existing `thinkthen_test_hook_reap()` from ticket 0118. The real release is reachable from a C or Rust host. The Python harness has no handle to a database with no connection, and the reaper's timing is not steady enough for a gate test.

## Shared files

Ticket 0127 owns the test harness and spawn sites, and 0128 is in flight too. This ticket adds tests to files they may touch: `databases/duckdb/tools/databases_suite.py`, `databases/postgresql/check.sh` (0127 and 0128), and the SQLite tests `tests/test_files.py` and `tests/test_values.py` (0127). It changes no harness helper. Whichever ticket lands second merges.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `databases/sqlite/src`: at most 10 added, net of lines removed.
- `databases/duckdb/src`: at most 90 added, net of lines removed.
- `databases/postgresql/src`: 0.
- Tests: SQLite at most 30, DuckDB at most 110 across its suites, PostgreSQL at most 15 in `check.sh`. No new fixture on PostgreSQL.
- Pages: the SQLite README, the DuckDB README and NOTES, and the pinned warm line in `tools/source_checks.py`, at most 14 lines changed in total.
- Each surface ratchet rises to the measured total in the commit that adds the code. The commit says what grew. Before adding, the builder looks for duplication to delete in `Caller::named_file`, `plain`, and the two-arm matches in SQLite's `scalars.rs`.
- No dependency. No public API change.

## Stop rules

1. **DuckDB spike after design steps 1 to 3.** Once the serial, the extra info, and the gated read exist, the build runs the 35 access cases, the two-database case, a `SET home_directory` case with `'@~/q.json'`, and a `SET file_search_path` case with a relative path. It runs them before any test or page lands. Decision 7 settles the last two. Any other disagreement with `read_text`, or a `defect` from the kept connection's context, takes decision 8's fallback. The record names the failing cases.
2. Stop before crossing a budget, adding a dependency, or touching `crates/thinkthen`.
3. Stop if another in-flight branch changes `databases/duckdb/src/connections.rs`, `src/ffi.rs`, or `src/relate.rs` before this one lands. The coordinator orders the two.
4. Stop if any plant stays green.

## Scope and exclusions

Excluded: warm's engine choice on DuckDB. It still reads no session setting and sits outside `thinkthen_max_requests_total`, which `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md` item 2 owns. Any library warm call. Any change to relate beyond decision 6's sentence, the cache and recorder, audit, diff, help text outside warm and that sentence, the test harness, spawn sites, or `libraries/python`. Other tickets own them. No live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The change raises two surface ceilings and adds a lock path on DuckDB's kept connection, so the code review names what it checked.

## Complexity

Contract 1; state and timing 3; reach 2; proof 2; cost of error 2; total 10. Final level: 2. The risks are a DuckDB warm that reads a file the caller's settings refuse, and a warm that hangs on relate's gate. Stop rule 1 and decision 4 guard them.

## Deferred gaps

- One public helper for the two-arm `LoadedQuestion` match. Three surfaces repeat it. It goes to `sdlc/issues/2026-09-25-public-library-api-gaps.md`, which is in 0.1.
- DuckDB warm with `GROUP BY` opens the file once per group. Decide opens it once per init. No user has asked for more.
- DuckDB warm refuses `'@file'` while a relate query runs on its database, even one on another connection. A second kept connection for file reads would lift it.
- The engine's kind refusal names `decide_many`, a Rust name, and SQLite's sends a choose user to `thinkthen_decide`. The wording batch owns both.
- DuckDB warm's session settings, as named under exclusions.

## What Ian can overturn

- Decision 1: warm ignores the band. The other choice keeps the refusal and names the fix.
- Decision 3: DuckDB reads `'@file'` through the kept connection. The other choice keeps the refusal, as 0110 decision 10 did, and names the `read_text` form.
- Decision 4: warm refuses at once during any relate query on its database.
- Decision 7: `'@~'` refused and relative paths documented, if the spike finds them different.
- Decision 8: the fallback, if the spike trips it.
- Decision 2: no public helper in this ticket.

## Closes

- `sdlc/issues/2026-09-25-warm-refuses-the-question-file-decide-uses.md`. The lander moves it to `sdlc/issues/closed/` in the landing commit. If decision 8's fallback runs, the issue closes with a status line that names the DuckDB fallback and its record.

## Evidence

- Starts from: The issue above, from the code review of the talk's SQL slide. `crates/thinkthen/src/engine/facade.rs` `judge`: the plan carries no threshold, and the threshold applies on read. `specification/channels.md` line 108. `Engine::decide_many_with` takes a `BandedQuestion` (`crates/thinkthen/src/public/bulk.rs`). Ticket 0110 decision 10 and its "Warm" bullet: the aggregate has no client context in the C API, so warm refused `'@file'`. Experiment 253 (D2) in the workspace: DuckDB's own file system agreed with DuckDB in 35 of 35 access cases, and a copied rule disagreed in 10. It measured cache folders, and 0110 then added the `@file` half. Ticket 0118's kept connection per database, its counted guard, its gate with a stop closure, its busy flag, and `thinkthen_test_hook_reap()`. The PostgreSQL warm code already routes a banded `'@file'` through `decide_distinct`.
- Keeps: Every decide, details, and annotate path. Warm's count, its one-question-per-group rule on DuckDB, its chunking and request totals on SQLite, and its row cap on PostgreSQL. Warm's refusal of choose, tag, and score questions. Every file sentence decide prints. DuckDB's rule that its own settings decide every file read. Relate's gate and its nested-relate refusal. The public API.
- Changes: SQLite warm takes a banded question from a file or inline JSON. DuckDB warm reads `'@file'` through its database's kept connection under the gate, and refuses during a relate query and after a release. LOAD takes a serial before registering functions. Relate's released-connection sentence names a fix that works. PostgreSQL gains a test.
- Proof: The ten rows under "Proof", each counting sends at the loopback backend or pinning a refusal, each with a planted fault that turns it red. The DuckDB spike runs first under stop rule 1.
- Defers: A public helper for the two-arm match. DuckDB warm during another connection's relate. DuckDB warm's session settings and request total. A library warm call. The kind refusal wording.
