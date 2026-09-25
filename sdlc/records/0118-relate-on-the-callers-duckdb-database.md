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

- `src/ffi/` became one `ffi.rs` beside each module, as in 0110, because `policy.py` allows `unsafe` only in files named `ffi.rs`.
- The bridge packs the owner's process id and the write end into one atomic. The handler therefore never pairs one process's id with another's descriptor. Decision 7 asked for two atomics with ordered stores.
- The time limit and the queue limit read `thinkthen deadline: `, as R5-22 asks. The tag read `usage`.
- Decision 6 asked for `CallOptions::deadline_millis`. Relate passes `deadline_at` with the same instant as the query's limit, as the coordinator directed for calls that loop.
- The engine sends one relation's chunks one at a time, and one chunk holds every question when no backend profile splits it. A relate over 16 rows therefore holds 1 request, never 8. The acceptance line "holds 8 counted requests" cannot pass without an engine change. The proof that the throttle reaches relate is the conflict sentence instead: after a scalar built the engine with throttle 8, a relate under throttle 4 reads main's sentence with no send.
- R5-22 uses a streaming filter over `range(100000000000)` that never fills its `LIMIT`. The `unnest` and misjudged-join shapes from the tag are not run.
- R3-12's 200 MB peak-growth bound is not measured. The refusal comes in about 1 s, including the table's creation.
- R4-4's read-only half is not tested. A read-only database cannot attach its probe, and relate there reads the usage sentence that says so.
- R4-22's "counting view" half is not built. The backend's count after release equals the number of queries, so the relate query asked once.
- The tag's atexit shutdown of the reaper is dropped. The reaper thread ends with the process.
- `examples.json` does not exist on this branch. The site-example check names the relate block as a divergence, since it draws the tag's rules shape.

## Results

- `check.sh` results are in the section below, filled at the branch head.
- Ratchets: `src` rises from 2176 to 3652 non-blank Rust lines. The kept connections take 520, relate 736, the bridge and its fork test 196, and the rest of `ffi.rs` 34. `tools` rises from 1116 to 1583 non-blank Python lines, for the two relate suites and the relate cases added to the signal, secrecy, conformance, and source checks. The tag held 2,149 production lines for the same two modules. The duplicated SQL runners were folded into `harness.run` and one child script.
- Conformance: 51 pass, 0 fail, 3 not run, 54 cases. The relate reason left the closed list.
- R4-22: 20 runs of two and then four concurrent held queries, a relate among them, each stopped within 100 ms of one SIGINT.
- A held relate reads `cancelled` within 100 ms, and the bridge stops a running relate query within 100 ms with 0 sends.
- R3-23: 500 idle databases used under 5 percent of one core over 10 s.
- R3-1: 200 of 200 rounds answered after a forced reaper pass.

## Planted bugs

Each plant changed one or two lines, rebuilt the extension, and ran its one case. All eleven went red, and the source was restored and touched.

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
| R2-29: delete the relate-from-rows sentence | `source_checks.py` | `R2-29: ADR 0038's DuckDB amendment lost its sentence on relate from rows` |

## Not yet done

- The unmet acceptance line on 8 held relate requests, which needs an engine change (see deviations).
- R3-12's memory bound, R4-4's read-only half, and R4-22's counting view.

## For the landing agent

- Nothing new beyond 0110's list. This ticket adds no dependency and no surface.
