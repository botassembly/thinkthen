# Record: ticket 0118, relate on the caller's DuckDB database

Branch `ticket/0118-duckdb-relate`, built on ticket 0110's branch. Built on Beelink on 2026-09-25 with the same toolchains as 0110's record. Nothing was downloaded.

## What landed

- `src/connections.rs` and `src/connections/ffi.rs`: one kept connection per loaded database, the random identity probe and marker table, the registry, the counted guard, the gate, and the reaper with its doubling sleep.
- `src/relate.rs` and `src/relate/ffi.rs`: `thinkthen_relate(query, rules)`, its two settings `thinkthen_relate_seconds` and `thinkthen_relate_holding_rows`, the statement guard, the plan-size guard, the 255-row cap, the time limit, relate from rows, and the `test-hooks` reaper pass.
- `src/signal/ffi.rs`: the handler writes one byte to a per-process pipe, and a bridge thread interrupts each busy kept connection.
- `src/ffi.rs` gained the `ANY` parameter type, the bind's client context, and the LOAD registration.
- Tests: `tools/relate_suite.py` (15 cases), `tools/databases_suite.py` (9 cases), four new cases in `tools/signal_suite.py`, relate boundaries in R1-10, relate errors in the secrecy case, and a fork unit test for the bridge. Conformance cases 51 and 52 run through `thinkthen_relate`.
- The README's relate section, the planning page's relate section, and part two of the ADR 0038 amendment. `tools/source_checks.py` reads one pinned sentence per ruling in both parts (R2-29).

## Deviations from the ticket

- `src/ffi/` became one `ffi.rs` beside each module, as in 0110, because `policy.py` allows `unsafe` only in files named `ffi.rs`. The merge of 0110's split puts `src/ffi.rs` under 500 nonblank lines, and each file stays under 500.
- The bridge packs the owner's process id and the write end into one atomic. The handler therefore never pairs one process's id with another's descriptor. Decision 7 asked for two atomics with ordered stores.
- The time limit and the queue limit read `thinkthen deadline: `, as R5-22 asks. The tag read `usage`.
- Decision 6 asked for `CallOptions::deadline_millis`. Relate passes `deadline_at` with the same instant as the query's limit, as the coordinator directed for calls that loop.
- The engine sends one relation's chunks one at a time, and one chunk holds every question when no backend profile splits it. A relate over 16 rows therefore holds 1 request, never 8. The ticket's acceptance line now proves the throttle through the conflict sentence and the cache hit instead, with the coordinator's approval on 2026-09-25, and `sdlc/issues/2026-09-25-relate-sends-one-chunk-at-a-time.md` records the engine behavior and its options.
- R5-22 uses a streaming filter over `range(100000000000)` that never fills its `LIMIT`. The `unnest` and misjudged-join shapes from the tag are not run.
- R3-12's 200 MB peak-growth bound is not measured, and the ticket now defers it. The relate statement alone refuses under 2 s.
- R4-4's read-only half is not tested, and the ticket now defers it.
- R4-22's counting view is not built, and the ticket now defers it. The backend's count after release equals the number of queries, so the relate query asked once.
- `examples.json` does not exist on this branch. The site-example check names the relate block as a divergence, since it draws the tag's rules shape.

## Review fixes

The first code review, at `ee6e7dc0`, returned 2 medium and 6 low findings. Each is fixed here.

- The reaper's shutdown at exit is back. `atexit` sets a flag, the reaper's tick ends its thread once the flag is set, and the bridge interrupts nothing after it. A unit test sets the flag and reads the tick's `false`. The test covers only the flag check. No test runs the `atexit` hook itself.
- The bridge interrupts each busy connection while it holds the registry, so the reaper cannot close one in between.
- A caller that matches no probe reads one usage sentence: `this connection's database answers no loaded identity probe, so relate cannot find its own connection; LOAD the extension again on a writable database, since a read-only database cannot carry a probe and a released one lost it`. A writable caller no longer reads a read-only sentence because another database lacks a probe, and a forged probe reads usage, not `defect`.
- Relate's refused rules file reads `the rules file PATH was not read`.
- `thinkthen_usage` and relate register through one `register_table`, and relate reads text parameters through the shared `owned_text`.
- R2-6 now refuses within 1 s, and R3-12's relate statement within 2 s, each timed by DuckDB's clock around that statement alone.
- Decision 5's two-database step on relate's path is covered by 0110's map-hit plant, since relate's bind reaches the cache probe through the same `engines::engine_for`.
- The other relate refusals are fixed sentences. The ones that insert text insert only DuckDB's own error for the caller's query, a path the caller named, or a number. The secrecy case covers a relate refusal from the backend, a missing rules file, and a missing table.

## Results

- `check.sh` passes at `e0142424`, after the merge of 0110's review fixes and this ticket's review fixes: fmt, clippy with `-D warnings`, the unit tests with the fork and shutdown tests, both builds, the source checks with R2-29, deny and its plant, the stock CLI call, every suite, conformance, the site examples, and the selftests. It prints 76 `ok` lines.
- Conformance: 51 pass, 0 fail, 3 not run, 54 cases. The relate reason left the closed list.
- R4-22: 20 runs of two and then four concurrent held queries, a relate among them, each stopped within 100 ms of one SIGINT.
- A held relate reads `cancelled` within 100 ms, and the bridge stops a running relate query within 100 ms with 0 sends.
- R3-23: 500 idle databases used under 5 percent of one core over 10 s.
- R3-1: 200 of 200 rounds answered after a forced reaper pass.

## Planted bugs

Each plant changed one or two lines, rebuilt the extension, and ran its one case. All twelve went red. After the merge and the review fixes, the ten suite plants ran again and all went red. R4-4's plant now answers a count where the fix reads the usage sentence, and the source was restored and touched.

| Plant | Case | Read |
| --- | --- | --- |
| R1-16: route relate to the last-loaded database | `r1_16_two_file_databases_each_relate_over_their_own_table` | `[11, 11, 11]` for `[4, 11, 4]` |
| R2-2 and R3-7: skip the statement-kind check | `r3_7_statements_that_write_or_attach_refuse` | `COPY` reached DuckDB and read its parser error |
| R2-6: drop the nesting guard | `r2_6_a_nested_relate_refuses_at_once` | the 30 s timeout fired |
| R3-1: drop the counted guard | `r3_1_a_bound_relate_outlives_a_forced_reaper_pass` | the child aborted with exit -6 |
| R4-4: accept a probe without its marker | `r4_4_a_forged_probe_is_refused` | the forged relate answered a count |
| R5-22: drop the timer | `r5_22_the_time_limit_stops_a_slow_query` | the 30 s timeout fired |
| Decision 5: skip the cache probe on relate's bind | `the_cache_probe_runs_on_relates_bind` | the refused folder answered 3 edges |
| Decision 7: the bridge never interrupts the kept connection | `the_bridge_stops_a_running_relate_query_within_100_ms` | `cancelled` after 59,503 ms |
| Decision 4: drop the plan guard | `the_plan_guard_refuses_a_large_grouping_before_it_runs` | the row-cap sentence came after the whole grouping ran |
| R6-5: drop the process-id check in the handler | `a_forked_childs_wake_never_reaches_the_parents_bridge` | the child's wake reached the parent |
| Review fix: the reaper ignores the exit flag | unit test `the_reaper_stops_once_the_process_exits` | the tick returned true |
| R2-29: delete the relate-from-rows sentence | `source_checks.py` | `R2-29: ADR 0038's DuckDB amendment lost its sentence on relate from rows` |

## Not yet done

- R3-7's zero-creates check under `strace` is not run. The case reads the folder listing, which shows no file.

- R3-12's memory bound, R4-4's read-only half, and R4-22's counting view.

## Re-scores

The coordinator approved these on 2026-09-25 as queue owner, and Ian can overturn each one.

- Files: the ticket's budget is eight production files. The approval covered nine. After the merge of 0110's split and the review's dedupe, this ticket touches eleven against 0110's tree: four new files (`connections.rs`, `connections/ffi.rs`, `relate.rs`, `relate/ffi.rs`) and seven small edits (`ffi.rs`, `lib.rs`, `questions.rs`, `questions/ffi.rs`, `signal.rs`, `signal/ffi.rs`, `tables/ffi.rs`). The two above the approval are `questions/ffi.rs`, where 0110's split moved the handles relate reads, and `tables/ffi.rs`, where the review asked `thinkthen_usage` to share relate's registration. The coordinator approved the eleven on 2026-09-25 as queue owner. Ian can overturn it.
- Ratchets: `src` rises to 3751 non-blank Rust lines and `tools` to 1760 over 0110's merged tree. The approval covered 3652 and 1583 before the merge. The kept connections take about 540 lines with the shutdown, relate about 710, and the bridge with its fork test about 200. The tools add the two relate suites and the relate cases in the signal, secrecy, conformance, and source checks. The tag held 2,149 production lines for the same two modules. The coordinator approved 3751 and 1760 on 2026-09-25 as queue owner, for the `ffi.rs` split and the shared helpers. Ian can overturn it.

## For the landing agent

- Nothing new beyond 0110's list. This ticket adds no dependency and no surface.
