---
flow: build
priority: 118
opens: databases/duckdb sdlc/planning/databases/duckdb.md sdlc/planning/adr/0038-duckdb-relate-runs-on-the-callers-database.md
---

# 0118: Relate on the caller's DuckDB database

Status: design accepted 2026-09-24. Owner: Claude.

## Outcome and authority

Add `thinkthen_relate(query, rules)` to the crate `thinkthen-duckdb` that ticket 0110 builds at `databases/duckdb`. Relate runs its query on a connection that belongs to the caller's database (ADR 0038) and returns one row per edge. Port it from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API. Every call reaches the real engine through `thinkthen`.

Ian ruled on 2026-09-24 that the DuckDB port splits at the relate seam, as finding 2 of the 0110 design review (`sdlc/records/2026-09-24-design-review-0110.md`) proposed. 0110 keeps the extension core. This ticket takes the kept connections, the registry, the identity probe, the reaper, the relate guard, relate from rows, the queue-wait message, the SIGINT bridge thread, and conformance cases 51 and 52. ADR 0038 on main records relate on the caller's database. Accepted ticket 0105 is the template for this ticket's shape. Ian can overturn every decision here except the split.

## Design and decisions

1. **The kept connection and the registry.** Relate resolves the caller's database at bind through `duckdb_table_function_get_client_context` and `duckdb_client_context_get_catalog`, against a registry of one kept connection per loaded database (ADR 0038). The tag's rules carry over: the registry keys each database by a random identity, never by its name (R2-13). A read-only database gets its probe, and a forged probe is refused (R4-4). No setting names the identity (R3-6). A freed slot is never reused (R4-3). A counted handle keeps a prepared relate's connection alive across a reaper pass (R3-1). The identity is 32 hex characters from 128 bits read from `/dev/urandom`, as at the tag, and adds no crate (R5-26).
2. **The reaper.** A closed database's kept connection leaves the registry, so the file's lock is released. The reaper sleeps until a guard on a retired database drops and backs off when idle (R3-23). A settings read that fails on a retired or closing database refuses `@file` and never allows it (R4-5).
3. **Relate from rows.** Main's relate takes entities with a name and a kind (`specification/relate.md`). The tag's relate was the superseded all-yes-or-no design over bare text. `thinkthen_relate(query, rules)` reads a query whose columns are id, name, and kind. A two-column query of id and name reads kind `*` and accepts only bare rules, as the command's `--lines` does. `rules` is `'@file.json'`, the version-one relate file as JSON text through `Relate::from_json`, or a `LIST` of the command's inline rule strings. Per ADR 0047 item 9, the binding dedupes rows by name and kind in first-seen order, calls `relate_with` once, and returns one row `(relation, source, target, probability)` per matching id pair. Ids come back as text. The tag's `{"either": [...]}` rule list leaves, and `either` rides in the relate file. The 2026-09-23 issue on relate in a database asks for this shape in its item 1. Its items 2 to 4 are out of scope.
4. **The guard.** The query runs read-only as one `SELECT` (R2-2). `COPY … TO`, `EXPORT DATABASE`, `ATTACH`, `SET GLOBAL`, `LOAD`, and `SET VARIABLE` refuse (R3-7). A relate nested in a relate query refuses (R2-6). More than 255 rows is `usage`, read under `LIMIT 256` (R3-12). This row cap is stricter than the engine's unique-pair cap, and the README says so. `SET thinkthen_relate_seconds` bounds the query, and `memory_limit` stays the hard bound for blocking operators (R5-22). The tag's gate3 lane message for a relate that waits in the queue behind another relate carries over. The plan-size guard carries over unchanged.
5. **Settings and file access.** Relate reads the caller's four engine settings of 0110 decision 5 and its file settings of 0110 decision 10 from the bind context. It builds its engine through the same process-wide map, so relate honors `SET thinkthen_width` and the others. `rules` given as `'@file.json'` passes the same access check as 0110's scalars. If 0110 took the fallback of its decision 10 and refuses every `@file`, this ticket restores `@file` in every door through the tag's rule over the kept connections, and R4-5 proves that rule. Builder note, 2026-09-24: if 0110 took its engine-settings fallback, relate reads no engine settings from the bind context and uses the engine `thinkthen_configure` last set.
6. **Prompt Ctrl-C.** Relate's engine call runs on 0110's detachable worker (shared rule 4). The worker owns the deduped entities, a clone of the parsed rules, and an internal `CancelToken`. The DuckDB thread waits in 50 ms ticks on 0110's interrupt predicate. On a stop it cancels the token, detaches the worker, and raises `thinkthen cancelled: ` at once. Relate passes the remaining `thinkthen_relate_seconds` budget to the engine through `CallOptions::deadline_millis`. The relate query runs once, and a cancelled relate never re-runs it (R4-22).
7. **The signal pipe and the bridge thread.** 0110's handler gains one step: it writes one byte to a pipe owned by the current process, with `write`. POSIX lists `write` as async-signal-safe. An atomic beside the pipe's descriptor holds the owner's process id. The handler compares it with `getpid()`, also async-signal-safe, and skips the write on a mismatch. A forked child that has not built its own pipe therefore never writes to the parent's. Builder note, checked at code review: a child building its pipe stores the new descriptor first and then its process id, both with release ordering. The handler loads the process id first with acquire ordering, then the descriptor. A handler that sees the child's id then always sees the child's descriptor. The bridge thread reads the pipe and calls `duckdb_interrupt` on each busy kept connection. A forked child builds its own pipe on first use, keyed by process id, so the child's SIGINT never reaches the parent's relate (R6-5). 0110's R5-21 source check gains `write` on its allowed list and still refuses a lock, an allocation, and a call into `thinkthen`.
8. **Test hooks.** The retire hook and the forced reaper pass sit behind 0110's `test-hooks` feature, each named with the `thinkthen_test_hook` prefix. 0110's shipped-build check covers them unchanged.
9. **The site page.** `site/src/data/examples/relate__duckdb.json` draws the tag's shape. 0110 files `sdlc/issues/2026-09-24-duckdb-site-examples-draw-the-tag-shapes.md` with this ticket's relate shape and hands the deck's owner the matching issue. The check runs the ported relate replacement and prints the divergence line that cites that issue.
10. **The record of rulings.** This ticket completes the DuckDB amendment to ADR 0038 that 0110 starts. It adds relate from rows (decision 3), the row cap and the relate time limit (decision 4), the identity source (decision 1), and the bridge's per-process pipe (decision 7). With it, R2-29's DuckDB half closes.

## Shared rules

This ticket follows the seven shared rules of `sdlc/planning/surfaces-port-guide.md` on main (`446a4d6b`, with the builder wording of `6eb1303e` and the paid-backend rule of `d783ab6b`).

1. **No Docker.** Relate's suites run in 0110's offline check with the pinned toolchains and no container.
2. **Toolchains under `~/.cache/thinkthen-toolchains/`.** Relate adds no toolchain. It uses 0110's CLI and venv under `~/.cache/thinkthen-toolchains/duckdb/v1.5.5/`.
3. **One cache and one backend per test.** Each relate test sets its own cache folder through `SET thinkthen_cache` and starts its own loopback backend. The held reply that can hold again and one backend per test come from ticket 0117.
4. **Prompt Ctrl-C.** Decision 6 runs relate's engine call on the detachable worker. R4-22 meets 100 ms.
5. **A deny plant that reaches deny offline.** This ticket adds no dependency. 0110's deny plant stands.
6. **No test or plant reaches a paid backend.** Relate's suites run under 0110's `check.sh`. That script unsets `THINKTHEN_API_KEY`, sets a fake key only beside a loopback `THINKTHEN_BASE_URL`, and refuses any other address. No plant here changes how the engine is built. Relate's settings test proves seeding through the cache folder: a second relate after `SET thinkthen_width = 8` answers from the environment's cache folder with a loopback count of 0.
7. **Public engine settings.** Relate honors 0110's four settings on the engine built on `EngineBuilder::from_env()`. A test that needs requests in flight runs `SET thinkthen_width = 8` in its own child process.

## What moves from the tag

- `src/connections.rs` and `src/relate.rs`, split into files under 500 nonblank lines each. Every `unsafe` line goes to `src/ffi/`. The settings reader that 0110 replaced leaves.
- The relate section of `README.md`, the relate entry of `examples.json`, and the relate section of `sdlc/planning/databases/duckdb.md`.
- The suites: `relate_guard_suite.sh`, `relate_wait_timer.py`, `two_databases.py`, `review3_duckdb.py`, `review4_cancel.py`, `review4_forge.py`, `review4_retired_read.py`, and `review4_rowcap.py`. Each moves onto a loopback backend it starts itself. The builder regroups them into at most ten files.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 32 DuckDB rows and 17 rows on other surfaces that name DuckDB. This ticket owns 16 DuckDB rows and one of the 17. Ticket 0110 owns the other 16 DuckDB rows and the other 16 cross-surface rows. No row is owned twice or dropped.

Each re-proof runs against the real engine through a loopback backend in the stock v1.5.5 CLI or the venv's `duckdb` module. The record plants each bug below and shows its test turning red, then green once the bug is removed. "Counted" means the backend's `count` line. Every held test ends with an explicit release and reads the count after it. Every subprocess runs under `timeout` with a stated bound.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-16 | closed | Two file databases in one process each relate over their own table, with 4 and 11 edges. A temporary table answers the ADR 0038 boundary sentence. | Route relate to the last-loaded database. The counts cross. |
| R2-13 | closed | Two `:memory:` databases each relate. `ATTACH … USE` counts the used database's table. A closed file database reopens, since no kept connection holds its lock. | Key the registry by database name. Both in-memory relates fail. |
| R2-2 | closed | A relate query holding a `DELETE` is refused before it runs, and the table keeps its rows after the caller's `ROLLBACK`. | Skip the statement-kind check. The row count drops. |
| R3-7 | closed | `COPY … TO`, `EXPORT DATABASE`, `ATTACH`, `SET GLOBAL`, `LOAD`, and `SET VARIABLE` as the relate query each refuse, and no file appears. | Check table writes only. `COPY` writes its file. |
| R2-6 | closed | A relate nested in a relate query refuses within 1 s under `timeout 30`. | Drop the nesting guard. The timeout fires. |
| R3-1 | closed | 200 rounds: a prepared relate is held while the `test-hooks` build forces a reaper pass, then executes and answers, with exit 0. | Drop the counted handle. The run exits 139. |
| R3-6 | closed | No `thinkthen_instance_token` setting exists. Database B's relate cannot read database A's tables. | Resolve identity from a setting. B reads A. |
| R4-3 | closed | A closes, the reaper drops A, B loads, and A reconnects. A's relate reads A's table and still refuses `/etc/hostname` with access off. | Reuse a freed registry slot. A reads B. |
| R4-4 | closed | A read-only database gets its probe. Detaching the probe and attaching `:memory:` under its name is refused. | Fall back to the name when the probe is missing. The forged read succeeds. |
| R4-5 | closed | The `test-hooks` retire hook marks eight live databases retired and makes their settings read fail. `@file` in decide and in relate's rules refuses with the pinned sentence, and `strace` counts zero opens. Under 0110's `@file` fallback, the tag's registry rule returns, and the plant is the tag's: skip the check for a retired database. | Allow the file when the settings read fails. The open count reads 1. |
| R5-26 | closed | Two processes produce distinct identity names of 32 hex characters. A source check finds `/dev/urandom` as the only seed. | Seed the hash with a constant. The names match. |
| R3-23 | closed | 500 idle databases cost under 5 percent of one core in CPU time over 10 s, read by `getrusage`. | Poll every database every 200 ms with no backoff. |
| R3-12 | closed | Relate over `generate_series(1, 8000000)` refuses with the pinned 255 sentence within 2 s and under 200 MB of peak growth. Another database's relate runs meanwhile. | Read every row before counting. The memory bound fails. |
| R5-22 | waive | With `SET thinkthen_relate_seconds = 2`, the `unnest` and misjudged-join shapes stop by 3 s with the `deadline` word. The README pins the sentence that `memory_limit` is the hard bound. | Drop the timer. The `timeout 30` bound fires. |
| R4-22 (stop) | closed | After `SET thinkthen_width = 8`, one SIGINT stops two and then four concurrent held queries, scalars and a relate among them. 20 of 20 runs stop each query within 100 ms of the signal. | Stop only the thread that saw the signal. Another query finishes. |
| R4-22 (once) | closed | In the same runs, the backend and a counting view each show the relate query ran once. | Re-run the relate query after a cancel. The run count reads 2. |
| R6-5 | closed | A forked child's SIGINT, sent before the child's first call, leaves the parent's held relate running. A second SIGINT after the child's first call does the same. The parent's relate answers after release. | Drop the process-id comparison in the handler. The parent's relate reads `cancelled`. |
| R2-29 DuckDB half | open | The ADR 0038 DuckDB amendment holds one pinned sentence per ruling from 0110 and decision 10. A `check.sh` step reads each. | Delete the relate-from-rows sentence. The step fails. |

## Other acceptance

- Red first: the relate suites fail against 0110's crate before this ticket's code, for the stated reason, then pass.
- Conformance cases 51 and 52 run through `thinkthen_relate` with recomputed digests. The runner's "relate lands in 0118" reason leaves its closed list, and the three counts still sum to the cases in `cases.json`.
- 0110's R1-10 test gains the relate scan boundary. In the `test-hooks` build, a panic there reads `defect`, and the CLI answers the next query.
- 0110's secrecy test gains every relate error message.
- A single held relate gets `cancelled` within 100 ms of a SIGINT sent once `count` reads 1. The count stays 1 after release.
- `SET thinkthen_width = 8` reaches relate. With `THINKTHEN_BASE_URL` pointed at the test's loopback backend, a relate over 16 rows holds 8 counted requests there. A second relate over the same rows answers from the `THINKTHEN_CACHE` folder, and the loopback count for that run reads 0.
- The queue-wait message keeps the tag's sentence, and `relate_wait_timer.py` pins it.

## The check

0110's `check.sh` runs the relate suites. This ticket adds no rung and no gate change under `sdlc/scripts`. It adds no dependency, so deny, the lock, and the license exceptions stay as 0110 left them.

## Budgets

Measured on the tag with `grep -c .` over each file up to its test module: `connections.rs` holds 1,197 nonblank production lines and `relate.rs` 952, 2,149 in all. Their tools hold 966 nonblank lines in 8 files. The port adds relate from rows, the typed FFI calls, the bridge thread from `lib.rs`, `Debug` on every type, and the rewrites for the denied lints. It removes the tag's `either` rule list, the text-only relate, and the settings reader.

- Production Rust: at most eight files and 2,500 nonblank lines, each file under 500.
- Tests: at most ten Python and shell test files and 1,400 nonblank lines, and at most 200 nonblank Rust unit-test lines.
- Scripts: at most 40 nonblank lines added to `check.sh`.
- Documentation: the README's relate section, the planning page's relate section, and the second part of the ADR 0038 amendment, at most 180 net nonblank lines.
- Ratchet: `databases/duckdb/ratchet.json` and `ratchet.py.json` each move `max` to the measured total. The record names what each block earns.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen` or `conformance/`, or changing a 0110 decision.

## Exclusions

Everything 0110 owns. New relate forms for new rows against old rows, or links to a known entity table. Items 2 to 4 of the 2026-09-23 relate issue. Temporary-table and open-transaction visibility. ADR 0038 names it as a boundary of the stable C API. Settings for `thinkthen_warm` through the kept connection. Any change to `thinkthen`. Any live or paid call. Hand edits to `site/src/data/examples/` or the deck.

## Dependencies

After 0110, since this ticket extends its crate, its worker, its handler, and its check. After 0117 (a held reply that can hold again and one backend per test). Through 0110, after 0086 with the 0084 amendment of 2026-09-24 (`f19cf437`, `EngineBuilder::from_env`).

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 4; reach 2; proof 3; cost of error 4; total 15. Final level: 3. A kept connection, a reaper, and routing live inside another program's process. A wrong rule reads another database's tables or writes through a read-only query.

## Review

- Design review: its rows came from finding 2 of the 0110 design review. The confirmation in the same record (`sdlc/records/2026-09-24-design-review-0110.md`) found the R4-5 plant had no code to act on and that a forked child could write to the parent's pipe. Both are applied, with the small fix. The final check (same file) accepted this design at `c4010827`, with one note for the builder in decision 7.
- Code review: pending.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The tag `surfaces-wave7-frozen-2026-09-24b` holds `databases/duckdb/src/connections.rs` and `src/relate.rs` and 8 relate suites in `databases/duckdb/tools/`, under ADR 0038. `experiments/218-thinkthen-release-qa/wave2/PLAN.md` found that relate's global connection saw the wrong database and no temp tables, and that `random()` let the count query and the run query disagree past the row cap. `repos/jev-experiments`: none found.
- Keeps: One kept connection per database, the reaper, the guard that allows one read-only `SELECT`, and the queue-wait sentence.
- Changes: Relate reads entity rows, honours 0110's settings, runs on a worker, and uses a signal pipe. Its hooks sit behind `test-hooks`.
- Proof: The two relate conformance cases, `cancelled` within 100 ms, width 8 reaching relate, and a cache hit on the second run, and the relate panic reading `defect`.
- Defers: Temp-table and open-transaction visibility, relate between new and old rows, links to a known entity table, and settings for warm. Experiment 218's volatile-function row-cap probe carries forward as a regression case.

Amended 2026-09-24: the ADR 0017 amendment of that date on main renames the width setting to the throttle. The DuckDB setting `thinkthen_width` becomes `thinkthen_throttle`, and every public `width` here reads as `throttle`.

## Spike finding, 2026-09-24

Experiment 253 on Beelink (`~/workspace/experiments/253-thinkthen-duckdb-sqlite-spike/`, local only) tested decision 7 and the settings path this ticket inherits. Raw logs sit in its `logs/` folder.

- **Decision 7: passed.** The handler stores the descriptor, then the process id, with release ordering, and loads them in the reverse order with acquire ordering. It then compares the id with `getpid()` and writes one byte. Over 20 forks from a Python parent that loaded the extension, a child's SIGINT before and after building its own pipe never reached the parent's pipe (0 of 20). Each child also answered a query on its own new DuckDB connection. The planted build without the comparison leaked all 20 (20 of 20).
- **Inherited contradiction in decision 5.** 0110's finding on the same date applies here. Main keeps the first explicit width for the process's life, so relate's engine through the shared map cannot take a second throttle. A different `SET` reads `usage` with main's sentence. The `@file` check for `rules` moves to the caller's DuckDB file system with 0110's change, and the copied-setting rule leaves.
- **Naming.** `SET thinkthen_width = 8` becomes `SET thinkthen_throttle = 8` under Ian's 2026-09-24 throttle ruling.
