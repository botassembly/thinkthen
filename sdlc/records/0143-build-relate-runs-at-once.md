# 0143: Build relate and split requests run at once

Status: built 2026-09-26 in lane `worktrees/thinkthen-lane-4`. Owner: Claude.

Branch `ticket/0143-relate-runs-at-once`. The ticket is `sdlc/tickets/0143-relate-runs-at-once.md`. A fresh read-only design review accepted it on 2026-09-26. The change raises the ceiling and widens `relate`'s command surface, so a fresh read-only Claude session reviews the code and names what it checked. Ian can overturn every decision the ticket lists.

## Coordinator rulings at build

The build crossed stop rules 1 and 2 and stopped. The coordinator ruled on both on 2026-09-26. Ian can overturn either ruling.

1. **Plant (i) became plant (i3).** The first plant (i) kept feeding chunks after `cancel.stop()` saw a stop, and the SIGINT test stayed green. For SIGINT the feed loop's stop check is a second guard: the send path checks the shared flag itself before it posts. So no plant on the feed loop's check alone turns a SIGINT test red. Plant (i3) sends the chunks one at a time, and the test turns red because the four held requests never arrive. The feed loop's check exists for host interrupts, and plant (j) proves that path.
2. **The source budget was re-scored** from 70 added and 50 net to 120 added and 90 net. `ordered` is new engine logic that rustfmt lays out vertically. The coordinator asked for trims to `ordered` wherever it stays clear. The build then trimmed `ordered` by one line, and later replaced the inner feed `while` and its `let`-`else` with a bounded `take`, which also keeps its nesting under the lint.

The ticket records both rulings in place.

## Result

- `workers::ordered` in `engine/workers.rs` runs items on up to `jobs` scoped workers and hands each result on in item order. The calling thread feeds one item to each free worker, never more. It holds replies by item number in a `BTreeMap` and hands them on once every earlier item has answered. A failed item, a failure of the caller's closure, or a stop feeds nothing further. The items in flight finish, and the first failure in item order returns. The stop runs through `cancel.stop()` between feeds, so the host check runs on the calling thread.
- `Engine::ask_chunks` in `engine/facade.rs` keeps its signature. With fewer than 2 jobs, where jobs is the engine width capped at the chunk count, it sends inline as before. Otherwise it calls `ordered`.
- `Engine::relate` in `engine/facade/relate.rs` gathers every relation's chunks into one list, beside each chunk's relation index, and makes one `ask_chunks` call. The closure takes the next mappings from that relation's iterator. The check that every iterator ends empty now runs after the call and raises the same Defect.
- `relate` takes `--jobs`. `cli/relate/config.rs` lost its refusal, `cli/relate.rs` passes the width to `asking::engine`, and `cli/args/command.rs` lost the `mut_arg` that hid the flag. The `--jobs` help in `cli/args.rs` names `relate`.
- `specification/relate.md`, `records.md`, and `settings.md` say so. Ticket 0140's settings table landed first, so the Throttle row names `relate`, and the `relate --jobs` line left "Settings on the way".
- No dependency, public library type, method, or message changed.

## Tests

New, all against the compiled command or the public API over the loopback backend:

- `crates/thinkthen/tests/backend/relate/at_once.rs`: `relate_requests_reach_the_throttle_and_no_further`, `split_relations_print_what_one_job_prints`, and `a_failed_chunk_stops_the_run_as_one_job_does`. Its third row has `bravo` answer first from model `other-1`, so the relate closure fails once `alpha` answers. The `--jobs 2` run matches the `--jobs 1` run's exit code 4, standard error, and standard output. It sent 3 requests to the one-job run's 2: `bravo`'s early reply freed a worker, and `charlie` went out before `alpha` answered. The review asked for exactly 2. No timing gives 2 at two jobs over six rules, because a worker is fed whenever a reply frees it and the closure runs only in chunk order. The row pins 3, which slow replies after `bravo` make exact.
- `crates/thinkthen/tests/public_controls.rs`: `a_host_interrupt_during_relate_chunks_sends_nothing_new`, with two rows. Six rules leave two relations unfed when the check fires. Four rules are all fed and held, so only the in-flight check can see the stop.
- `crates/thinkthen/tests/relate_edge.rs`: the help test now pins the `--jobs` sentence.
- `databases/duckdb/tools/relate_suite.py`: `the_throttle_reaches_a_relate`, which restores ticket 0118's count of 8 in flight.

The split test runs both jobs settings against one listener, because each digest carries the listener's address.

Existing tests changed because requests now arrive together. Three needed it, within stop rule 3's five.

- `sigint_between_recognition_chunks_starts_no_later_chunk` in `tests/backend/interrupt.rs`. The `held` helper takes the number of requests to hold. The recognize row holds 4 of the 14 chunks of "Ada met Bob at Acme in Paris" and asserts exactly 4 requests. Every other row holds 1.
- Four tests in `tests/backend/annotate/splitting.rs` serve one canned reply per chunk in arrival order. They pass `--jobs 1`, so each chunk meets its own reply. `different_models_across_chunks_keep_the_safe_failure` failed in the first ladder run without it. The other three could race the same way.
- `test_relate_is_cancelled_and_sends_no_more_after_release` in `databases/sqlite/tests/test_interrupt.py` sets throttle 1, so the second relation still waits behind the first.

The refusal rows changed as the ticket says: `jobs on one document` lists the verbs that still refuse, and `refusals/relate.rs` lost relate's own sentence.

## Plants

The code review at `de3fb9f6` found the design sound and asked for the fixes that plants (k) and (l) cover. Plants (j), (k), (l), and (i3) ran again on the fixed code. Each plant edited source, ran its test under the heavy lock, and restored the file with `git checkout` and `touch`. Plants (a) to (j) ran on the final source after the last merge of `origin/main`. Plant (i3) ran again after the last change to `interrupt.rs`. The script and logs sit in the session scratchpad under `t0143/`, outside the repository. A grep of the diff for plant text found none.

| Plant | Test | Result |
| --- | --- | --- |
| None, the code as built | all | Green |
| (a) one `ask_chunks` call per relation, the old loop | `relate_requests_reach_the_throttle_and_no_further` | Red: peak 1, not 3 |
| (a) on DuckDB | `the_throttle_reaches_a_relate` | Red: wanted 8, got 1 |
| (b) `cli/relate.rs` passes `None` as the width | same | Red: peak 4, not 3 |
| (c) hand replies on in arrival order | `split_relations_print_what_one_job_prints` | Red: the edges and request list reorder |
| (d) always send inline | same | Red: peak 1 |
| (e) feed every item up front | `a_failed_chunk_stops_the_run_as_one_job_does` | Red: 6 requests, not 2 |
| (f) return the first failure to arrive | same | Red: the 503 message, not the 500 |
| (g) hide `--jobs` from relate help again | `relate_edge` help test | Red |
| (h) restore relate's `--jobs` refusal | `relate_requests_reach_the_throttle_and_no_further` | Red: exit 2 |
| (i) keep feeding after `cancel.stop()` sees a stop | recognize SIGINT test | Green, as ruling 1 records |
| (i2) plant (i) and the send path's flag check removed | same | Green: evidence for ruling 1 |
| (i3) always send inline | same | Red: the fourth held request never arrives |
| (j) poll through `poll_between_sends` in place of `cancel.stop()` | `a_host_interrupt_during_relate_chunks_sends_nothing_new` | Red. With the count asserted first, the six-rule row read 6 requests and a finished call. The ticket predicted 6. The earlier runs asserted the result kind first, so they reported a finished call and never showed the count. In one run the six-rule row passed and the four-rule row failed, so the six-rule row alone is timing-dependent under this plant |
| (k) `failure = None` in place of `each(result).err()` | `a_failed_chunk_stops_the_run_as_one_job_does`, third row | Red: exit 70 with the Defect "a relation reply did not cover its question map", not exit 4 |
| (l) check the stop only while items are left to feed, the code before the review | `a_host_interrupt_during_relate_chunks_sends_nothing_new`, four-rule row | Red: the call finished, not `Cancelled`. The same row was red on the code before the fix |

## Budgets

Nonblank lines against `origin/main` at `7443d69d`, after the review fixes.

| Area | Ticket budget | Measured |
| --- | --- | --- |
| `crates/thinkthen/src` | at most 120 added, 90 net, after ruling 2 | 117 added, 33 removed, 84 net. `workers.rs` +68 −1, `facade/relate.rs` +29 −17, `facade.rs` +16 −6, the command side +4 −9 |
| `relate/at_once.rs` | at most 200 added | 160 |
| `public_controls.rs` | at most 35 added | 42 added, 1 removed. **Crosses by 7 lines, more than a tenth.** It was 36 before the review's second row. rustfmt lays the row loop out vertically, and the trims kept it at 42 |
| `interrupt.rs`, `refusals.rs`, `refusals/relate.rs`, `relate_edge.rs`, `annotate/splitting.rs` | at most 40 changed | 35 added, 17 removed. Within budget counted as added lines. Counted as added plus removed, it is 52 and crosses |
| `relate.rs` | the `mod` line | 1 |
| `relate_suite.py` | at most 30 added | 20 |
| `sqlite/tests/test_interrupt.py` | not budgeted | 6 added, 2 removed |
| Pages | at most 15 changed | 3 added, 4 removed |

`facade.rs` holds 368 nonblank lines, under its 500 cap.

`sdlc/ratchet.json` moves from main's 67,758 to 68,062, up 304. The review fixes added 26 of those lines. The builder looked for duplication first. `ordered` reuses `workers::scoped` and `Cancel::stop`, which the record and group schedulers use, so no scheduler was copied. The DuckDB Python ratchet moves from 1,896 to 1,916 for the throttle case, and the SQLite one from 1,209 to 1,213 for the throttle 1 setup and its reason.

## Ladder

The first run at `d448aa30` found four failures. Lint: the DuckDB Python ratchet. Test: `different_models_across_chunks_keep_the_safe_failure`, which read canned replies in arrival order. Surfaces: the SQLite relate interrupt test counted 2 sends where it wanted 1, and the Polars rung's clippy found `ordered` nested too deep. The fixes are under Tests and Budgets above.

After the fixes and the merge of `origin/main` at `7850db3f`, each rung ran once at `ed250b46`, called directly.

| Rung | Result |
| --- | --- |
| `install` | exit 0, 5 s |
| `lint` | exit 0, 167 s; ratchet 68,036 of 68,036 |
| `test` | exit 0, 151 s; 947 passed, 0 failed across 36 test binaries |
| `spec` | exit 0, 162 s; demos 21 green, 0 red |
| `surfaces` | exit 0, 1,266 s; Rust, C, Python, TypeScript, Ruby, R, DuckDB, SQLite, PostgreSQL, and Polars pass |

After the review fixes and the merge of `origin/main` at `7443d69d`, `lint`, `test`, and `spec` ran once more. No surface file changed, so `surfaces` did not rerun.

REVIEWLADDER

## Lane size

The lane measured 9.7G before the build and 11G after it.

## Deferred gaps

The ticket's gaps stand. `recognize` over one text still refuses `--jobs`, and ticket R4b owns that. The library `Engine::annotate` answers one text's groups one after another. A failed run may send and bill up to `width - 1` requests already in flight. The `site/` relate page waits on the website agent. The first plant (i) shows the SIGINT path has two guards, and no test proves the feed loop's check alone for SIGINT.

## Closes

- `sdlc/issues/2026-09-25-relate-sends-one-chunk-at-a-time.md`. The lander moves it to `closed/` in the landing commit.
