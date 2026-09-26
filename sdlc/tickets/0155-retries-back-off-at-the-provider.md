---
flow: build
priority: 155
opens: sdlc/planning/adr/0052-retries-back-off-at-the-provider.md sdlc/planning/adr/0034-status-and-count-only-usage.md sdlc/planning/adr/0048-records-batch-into-full-requests.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/backoff.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/src/engine/usage.rs crates/thinkthen/src/engine/usage crates/thinkthen/src/engine/width_tests.rs crates/thinkthen/src/engine/deadline_tests.rs crates/thinkthen/src/cli/schedule/width_tests.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/status.rs crates/thinkthen/src/public/settings.rs crates/thinkthen/tests/backend/backoff.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/public_backoff.rs crates/thinkthen/tests/status.rs crates/thinkthen/tests databases/sqlite/tests/test_deadline.py databases/sqlite/NOTES.md databases/sqlite/README.md databases/duckdb/README.md databases/postgresql/README.md libraries conformance specification/backends.md specification/settings.md specification/check.md specification/recording.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0155: Retries back off at the provider and count apart

Status: ready for review. Owner: Claude. It carries ADR 0052. It builds only after tickets 0146 and 0148 land on main, and before ticket 0154 and the batching design's B5, by the coordinator's ruling of 2026-09-26.

Review route: a fresh read-only Claude session reviews this design, ADR 0052, and later the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `thinkthen filter --jobs 8` against a backend that starts answering 503. Today each of the 8 requests in flight meets the overload, waits on its own timer, and sends again, while new requests keep going out. After this ticket the first 503 closes one gate for that address. Every request to it waits, then the run resumes at full width. Each request still has its own retries, 3 by default, and the totals report retries apart from first sends.

Ian ruled on 2026-09-26, and the coordinator relayed the rulings and Ian's two corrections. Ian can overturn each.

1. The retry default changes from 2 to 3. Retries stay configurable.
2. Backoff happens at the provider, not per message. One shared gate for each process and each resolved address.
3. The throttle is fixed. "Don't be too fancy with throttle." `--jobs N` never shrinks or grows.
4. Each request has its own retry count. A retry never spends another request's retries.
5. Every send spends one unit of a request budget, first send or retry.
6. `--jobs N` bounds sends in flight, retries included. A request waiting out a backoff holds no slot and takes one before it resends.
7. Retries are reported apart from `requests_sent` in the usage totals and the run facts.

## What happens today

Read from `origin/main` `e0a6150a`.

- `engine/http.rs::Client::post_observed` loops over attempts. Each attempt takes a permit from the process's width gate, sends, and drops the permit. After a retried status it waits `bounded_wait(asked, wait, timeout)`, the header's wait or its own doubling wait, capped by 60 seconds and `--timeout`, then doubles its own wait. Nothing tells another request about the wait.
- The width gate, `engine/mod.rs::Widths`, is one per process. A forked child gets a fresh one through `process::Guarded`. `width_tests.rs::a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one` and `a_replayed_answer_takes_no_permit` pin that a retry wait and a replay hold no permit. So rule 6 already holds.
- `--max-retries` defaults to 2 in `cli/args.rs:171`. `EngineBuilder::build` fixes 2 in `public/settings.rs:237`. Ticket 0148 exposes `max_retries` on every library with the command's default.
- Each attempt calls `Counters::request_sent`, so `requests_sent` counts every attempt. `HttpAnswer::requests_sent` is retries plus one. The usage file `thinkthen.usage/1` holds `requests_sent`, `input_tokens`, `output_tokens` and `cache_answers`, with `deny_unknown_fields`. `thinkthen status` prints `month_` and `total_` for each.
- The command has no request budget. The library's `max_requests` refuses a call over its number of records before anything is sent (`public/bulk.rs::within_limit`). The SQL process request total reads the engines' `requests_sent` before each call (`databases/duckdb/src/engines.rs::within_total`, `databases/sqlite/src/scalars.rs::flush`).
- The conformance backend's `/arm/503/v1` answers 503 with `retry-after-ms: 10`. Tests that count its sends at the default expect 3.

## Design

### The gate

A new `engine/backoff.rs` holds one process-wide table from posting URL to the instant its gate opens, behind one mutex and one condition variable. It follows the width gate's pattern: a `process::Guarded` value, so a forked child starts with every gate open.

- `close(url, wait)`: the gate's opening becomes the later of its current opening and now plus `wait`.
- `wait_open(url, until, cancel)`: blocks until the gate is open or the instant `until` passes, observing cancellation and the call's deadline every 50 ms, as `Widths::acquire` does.

`post_observed` changes in three places.

1. Before each attempt it sets one wait deadline: now plus the smaller of 60 seconds and its own `--timeout`. The call's deadline ends the wait sooner, as it ends a retry wait today. The attempt calls `wait_open(url, until, cancel)`, then `self.width.acquire`. After it takes the permit, it checks the gate once more. If the gate closed meanwhile and the wait deadline has not passed, it drops the permit and calls `wait_open` again under the same deadline. Once the deadline passes, it sends with the permit it holds, even if the gate is closed. So a waiting request never holds a slot, and the recheck loop always ends.
2. `send` returns, and the attempt is classified before `drop((sending, permit))`. A retried status calls `close(url, bounded_wait(asked, wait, timeout))` while the permit is still held, and only then does the permit drop. So a request that takes the freed slot always finds the gate closed on its recheck. The request's own doubling and retry count go on as today, and its next attempt waits in step 1.
3. When its retries are spent on a retried status, it closes the gate the same way, before the permit drops, and then returns the failure.

A replay or cache answer never reaches `post_observed`, so it never waits.

### The default

`cli/args.rs` sets `default_value_t = 3` for `--max-retries`. `EngineBuilder::build`, or the default that ticket 0148 gives its `max_retries` setter, becomes 3. The libraries and SQL surfaces build through it, so all of them take 3. Every page that ticket 0148 writes with the default of 2 changes to 3: the library READMEs under `libraries/`, `conformance/README.md` and any shared case that states the default, and the retries lines of the DuckDB, PostgreSQL and SQLite READMEs.

### Retries counted apart

`Counts` gains `retries: u64` with `#[serde(default)]`, so a usage file written before this ticket reads as 0 retries. `Counters` gains `retry_sent()`. `post_observed`'s hook tells the counters whether the attempt is a retry, and the counters add one to `requests_sent` for every attempt and one to `retries` for a retry. `thinkthen status` prints `month_retries` and `total_retries` after `month_requests_sent` and `total_requests_sent`, in text and in `thinkthen.status/1`. A row's `meta.requests_sent` and `HttpAnswer::requests_sent` do not change.

The run facts do not exist yet. ADR 0052 item 9 adds `retries` to ADR 0048 item 10, and B5 builds it. The library's `Counters::retries()` joins ticket 0157, the library settings follow-up.

### Request budgets

The command has none. The library's `max_requests` counts records and is unchanged, by decision 12. The SQL process request total already counts retries, because it reads `requests_sent`. ADR 0052 item 8 fixes what happens when it runs out in the middle of a request: the retry is not sent, the request fails with its retried status, and the call raises the spent-total refusal. Ticket 0149 builds that, because it carries the SQL settings and the check must move from each SQL surface into the engine's send gate. This ticket changes no SQL surface code.

### Pages

- `backends.md`, "The request": the default becomes 3, and the retry paragraph describes the gate, what closes it, who waits, the caps, per-request counts and the fixed throttle.
- `settings.md`: the Retries row's default is 3, with the reason from ADR 0052 item 1. Each library cell that ticket 0148 writes states a default of 3. The SQL cell says "Fixed at 3 on SQL until ticket 0149". The list of defaults with no reason loses "the 2 retries". The Throttle row gains one sentence: it counts retries, and a request waiting out a backoff holds no slot.
- `check.md`: `--max-retries` keeps its default of 3, and four probes send at most sixteen attempts.
- `recording.md`: the usage paragraph names the `retries` count, and `status` reports it.
- ADR 0034 gains `(Amended by ADR 0052.)` on its counts sentence. ADR 0048 item 10 gains the same marker. ADR 0017 section 5 gains it after ticket 0148 lands.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **Three retries by default.** Ian ruled it. The reason recorded: three doubling waits ride out about 7 seconds of overload where two rode out about 3. The cost: a request that fails every attempt may be billed four times.
2. **One gate for each process and exact posting URL.** Ian ruled a gate per process and address. The author keys it by the exact URL string, because `Backend::url` is already canonical: the address rules lower-case the scheme and host and keep the rest as typed.
3. **The gate closes for the wait the failing request would take today.** No new timing rule enters. The gate keeps the later opening, so a short wait never cuts a long one.
4. **A request with spent retries closes the gate too.** Its status still says the backend is overloaded, and the next request would meet it.
5. **One wait deadline for each attempt, capped by the waiter's own `--timeout` and 60 seconds, then the request sends.** The deadline covers the first wait and every recheck after a permit, so a gate that keeps closing cannot hold a request longer. Engines in one process may carry different timeouts. Each keeps its own promise that no wait exceeds its timeout.
6. **The throttle is fixed, counts retries, and a wait holds no slot.** Ian ruled it. The code already does this. The ticket adds a gate recheck after the permit, so a slot taken as the gate closes goes back. A retried status closes the gate before its permit drops, so the recheck cannot miss it.
7. **`requests_sent` keeps counting every send, and `retries` names the subset.** Ian ruled that retries count apart. This reading keeps every existing count's meaning and keeps retries inside the SQL request total with no code change, as Ian's correction requires. The other reading, `requests_sent` as first sends only, would change every row and let retries escape the total. ADR 0052 gives both. Ian can choose the other.
8. **Old usage files read as 0 retries under the same schema name.** No release has shipped. Ticket 0124 changed `thinkthen.status/1` in place for the same reason. An older binary refuses a file this build writes, and prints its one persistence warning.
9. **The mid-request budget check goes to ticket 0149.** It needs the SQL total inside the engine, which is SQL settings work. The rule is fixed here.
10. **Rows gain no `retries` field.** The ruling names the usage totals and run facts. A row's share of retries can join `meta` later if a user asks.
11. **Build after 0146 and 0148, before 0154 and B5.** The coordinator ruled this order on 2026-09-26. The section "Order and shared files" gives the reasons.
12. **"A retry counts against the request limit" reads as the SQL process total, not the library's `max_requests`.** Ian's correction has two readings. In the first, the request limit is the library's `max_requests`, and a retry should spend one unit of it. In the second, the request limit is the spend cap that counts sends, which is the SQL process request total. The ticket takes the second. `max_requests` caps the records of one call and is checked before anything is sent, so at that point there is no send to count, and making it count retries would turn it into a different setting. The SQL process total already counts every send, retries included, and ADR 0052 item 8 closes its gap inside a request. Ian can choose the first reading, which would need a new send-counting check on every library.

## Order and shared files

Order, ruled by the coordinator on 2026-09-26: 0146, then 0148, then 0155, then 0154, then B5.

| Ticket | Shared files | Why the order |
| --- | --- | --- |
| 0146 | `cli/args.rs`, `crates/thinkthen/tests`, `cli/schedule/width_tests.rs`, `engine/deadline_tests.rs`, `backends.md`, `settings.md` | 0146 opens `cli/args.rs`, the whole test folder, and the two unit test files. 0155 changes one default line there and the tests that count the default's sends. 0155 waits for it |
| 0148 | `public/settings.rs`, `settings.md`, `libraries`, `conformance`, the DuckDB, PostgreSQL and SQLite docs, `databases/sqlite`, ADR 0017 | 0148 exposes `max_retries` on every library with the command's default, writes that default on its pages, and opens `databases/sqlite`. 0155 changes that default to 3 on each page and the tests that count the default's sends. Its shared case "max-retries-zero-sends-once" sets 0 and does not change. 0155 waits for it |
| 0154 | `cli/args.rs`, `backends.md`, `settings.md`, ADR 0048 | Different lines: 0154 adds `--max-request-bytes` and the request-size row, and edits ADR 0048 items 2, 5, 6 and 9. 0155 changes the retries default and row, and marks item 10. 0155 lands first, and 0154 merges it |
| B5 | ADR 0048 item 10, `result_json.rs` | B5 builds the run facts. B5 builds after 0155 and 0154, so it builds `retries` with the other facts, and 0155 touches no run-facts code |
| 0149 | the SQL surfaces | 0149 builds ADR 0052 item 8's check inside a request. 0155 changes no SQL product code |

0155 does not touch `engine/facade.rs`, `engine/schedule.rs`, `public/results.rs` or any `cli/asking` file.

## Edge cases

| Input | Expected |
| --- | --- |
| One record, backend answers 503 three times then 200, nothing set | 4 sends. Exit 0. Usage: `requests_sent` 4, `retries` 3 |
| The same with `--max-retries 2` | 3 sends. Exit 4, today's 503 line |
| The same with `--max-retries 0` | 1 send. Exit 4 |
| Three records, `--jobs 2 --timeout 10`: record 1 meets 503 with `retry-after-ms: 3000`, record 2 answers once record 1's 503 is written, record 3 answers | Record 3 arrives no sooner than 3,000 ms after the listener's instant before it wrote the 503. 4 sends in all. Rows 1 to 3 in order |
| The same, record 1 meets 503 with no header, `THINKTHEN_TEST_RETRY_WAIT_MS=1` | Record 3 arrives less than 2 seconds after that instant. The gate's length follows the failing request's wait |
| Record 1 meets 429 with `retry-after: 1` | The gate closes for 1 second, as for 503 |
| Record 1 meets 400, 401, 422 or a transport failure | The gate stays open. Today's failure |
| `retry-after-ms: 120000`, `--timeout 2` | The gate closes for 2 seconds. No request waits longer than 2 seconds before an attempt |
| Record 1's retries run out on 503 | The gate closes for its last wait. The run stops at record 1 as today |
| A request waiting on the only slot takes it just after a retried status frees it | The gate is already closed. The request gives the slot back and waits |
| A gate that closes again at every recheck | The request sends once its one wait deadline passes |
| An interrupt while requests wait on the gate | No new send. Today's cancellation lines |
| `--replay DIR` of any run | No wait, no send |
| `--cache DIR` with record 2 cached, record 1 meeting 503 | Record 2's answer comes from the cache. Only live sends wait |
| Two engines in one process at the same address, one with `max_retries(0)` meets 503 with a 5-second wait | Its call fails at once and the gate stays closed. The other engine's next call waits for the gate |
| Two engines at two addresses, one meets 503 | The other's call answers at once |
| A forked child after its parent closed a gate | The child's gate is open |
| A usage file written before this ticket | Reads as 0 retries. `status` prints `month_retries 0` |
| `thinkthen status` after the first edge row | `month_requests_sent 4` and `month_retries 3` |
| `check` against a backend answering 503 | At most 16 attempts across its four probes |
| A batch of 10 records at `--batch 10` meets 503 | The whole batch is resent. Every batch to that address waits on the gate |

## Proof

The command tests drive the compiled binary against the loopback (`tests/backend/harness`, `Listener::answering`). The library test drives the public API against two loopback listeners. The loopback's closure runs on its own thread as each request arrives, so it records the arrival instant and order with no hook in the tool.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_503_holds_every_request_to_that_address`, new in `tests/backend/backoff.rs` | The three-record gate row and its no-header partner, at `--jobs 2 --timeout 10`. Both rows set the existing `THINKTHEN_TEST_RETRY_WAIT_MS=1`, so a request's own doubling wait is 1 ms and only the header can make the gate long. The listener holds record 2's answer on a barrier until record 1's 503 is written, so record 3 can only go out after that 503. The listener takes its instant T just before it writes the 503. The test pins the send count and the stdout rows. With `retry-after-ms: 3000` it asserts record 3 arrived at or after T plus 3,000 ms, with no margin. That bound is exact: the tool reads the 503 after T, so the gate opens after T plus 3,000 ms, and record 3 cannot arrive before it opens. With no header it asserts record 3 arrived before T plus 2 seconds | (a) No gate, today's per-request wait: record 3 arrives within a few ms of T. (b) The gate ignores `retry-after-ms`: the gate lasts the 1 ms test wait and record 3 arrives early. (c) The gate never opens before its cap, say a fixed 60 s: the no-header row holds record 3 until the 10-second timeout, past 2 seconds |
| `retries_count_apart_from_first_sends`, new in the same file | The first three edge rows and the two `status` rows, with a private `XDG_CACHE_HOME`. A pre-written usage file without `retries` is read too | (a) The default stays 2: the nothing-set row exits 4 after 3 sends. (b) `requests_sent` stops counting retries: it reads 1. (c) `retries` is not written: `status` reads 0. (d) `retries` is required when reading: the old file fails to read |
| `the_gate_is_per_address_and_shared_by_engines`, new in `crates/thinkthen/tests/public_backoff.rs` | Two engines at listener A and one at listener B, in one process. Engine 2 first answers one text from A, so that text is in its cache. Engine 1 is built with `max_retries(0)` and meets 503 with `retry-after-ms: 5000`, so its call fails at once and leaves the gate closed. Listener A takes its instant T just before it writes the 503. Right after engine 1's call returns, the test starts three calls, each on its own thread and in this order: engine 2 at A, engine 2's cached text, and the engine at B. Called one after another, the call at A would outlast the gate and hide it from the other two. It asserts engine 2's call arrived at A at or after T plus 5,000 ms, with no margin, and that the cache answer and the call to B both returned before T plus 2.5 seconds | (a) One gate for the process: the call to B returns after 5 seconds. (b) One gate per engine: engine 2's call arrives at A before T plus 5,000 ms. (c) The cache answer waits on the gate: it returns after 5 seconds. (d) A request with spent retries leaves the gate open: engine 2's call arrives at A early |
| `a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one`, amended in `engine/width_tests.rs` | Three rows, each with a private gate table passed through a `backed_off` test builder, as `gated` passes a private `Widths` today. Row 1: while the retry waits on the gate, another caller takes the only permit, as today. Row 2, the recheck: the test holds the only permit, and the poster runs with `Cancel::observed` (`engine/mod.rs:204`). The poster finds the gate open and blocks on the permit, and `observed_block` tells the test so. The test then closes the gate for 300 ms, takes its instant, and releases the permit. The listener's arrival must come at or after that instant plus 300 ms, and the test must take the permit again within 200 ms of releasing it. Row 3, the close before release: the poster holds the only permit and meets 503 with `retry-after-ms: 300`, while a second caller waits on the permit. The test's `Widths` pauses the poster at its permit release through a new test-only handshake, checks the gate is already closed, then lets the poster go. The second caller's send must arrive at or after the gate's opening | (a) Wait on the gate while holding the permit: row 1's other caller times out. (b) Drop the recheck after the permit: row 2's send arrives before the gate opens. (c) Close after the permit drops, so record 3 goes early: row 3 finds the gate open at the release |
| Existing tests that count the default's sends | `tests/backend/exchange.rs`, `tests/backend/resend.rs`, `tests/backend/secrecy/routes.rs`, `cli/schedule/width_tests.rs`, `databases/sqlite/tests/test_deadline.py` and any other the build finds change 3 sends to 4 where they rely on the default. Each keeps its own subject | (a) The default stays 2: each is red |

The four questions:

- **`a_503_holds_every_request_to_that_address`.** It protects ADR 0052 items 2 to 4: one request's retried status holds every other send to that address, for the time the backend asked. A per-request wait, a gate that ignores the header, or a gate with a fixed length fails it. No test checks that one request's status delays another's send. It adds no hook. It uses the binary's existing `THINKTHEN_TEST_RETRY_WAIT_MS`, which other retry tests already set, because a real one-second doubling wait would hide whether the header set the gate. The loopback's barrier and arrival instants are an ordinary server's behavior. Order alone cannot prove the delay, because record 3 and record 1's retry leave the gate at the same instant and race for a slot. So the test asserts an exact lower bound, measured from the listener's own instant before the 503, and a generous upper bound for the no-header row. It never sleeps to wait for an absence.
- **`retries_count_apart_from_first_sends`.** It protects ADR 0052 items 1 and 9: the new default and the separate count. The default left at 2, a `requests_sent` that drops retries, or a count never written fails it. `status.rs` checks today's fields with no retries, and no test sends a retry and reads the totals. It needs no hook: `XDG_CACHE_HOME` is the real place the usage file lives.
- **`the_gate_is_per_address_and_shared_by_engines`.** It protects ADR 0052 items 2, 3, 4 and 11: the gate's scope on the libraries, that a request with spent retries closes the gate, and that answers from disk never wait. A gate per process, per engine, in front of the cache, or left open by a spent request fails it. The command runs one address and one engine, so no command test reaches the scope. It needs no hook: two public engines at two listeners are ordinary callers, and `max_retries` is a public setting after ticket 0148. Its own test binary keeps the process gate apart from other tests.
- **The amended permit test** extends an existing contract to the gate path. Rows 1 and 2 need no new hook: `Cancel::observed` and the private gate table are the existing test seams, and the gate table follows `gated`. Row 3 needs one test-only hook, a pause at the permit's release in a test's own `Widths`. The fault it catches lives between two lines of one thread, a few microseconds wide. No outside behavior can hold that thread there, so the command test would catch it only when a waiting request won that race, and stop rule 6 forbids a plant that stays green. The hook compiles only under `cfg(test)` and changes no product path.
- **The count changes** keep existing tests on their subjects under the new default.

## Budgets

Nonblank lines, measured with `grep -c .`. Net lines against main after tickets 0146 and 0148 land.

- `crates/thinkthen/src/engine/backoff.rs`: at most 90, new.
- `crates/thinkthen/src/engine/http.rs`: at most 30 net.
- `crates/thinkthen/src/engine/mod.rs`: at most 15 net, 10 of them the test-only pause at a permit's release.
- `crates/thinkthen/src/engine/usage.rs`: at most 15 net.
- `crates/thinkthen/src/cli/status.rs`: at most 12 net.
- `crates/thinkthen/src/cli/args.rs` and `public/settings.rs`: at most 4 net together.
- Product code total: at most 166 net.
- `crates/thinkthen/tests/backend/backoff.rs`: at most 220, new.
- `crates/thinkthen/tests/public_backoff.rs`: at most 150, new.
- `engine/width_tests.rs`: at most 110 net.
- `crates/thinkthen/tests/status.rs` and the existing tests that count the default's sends, including the library and conformance tests that ticket 0148 adds: at most 60 changed lines together, in at most 12 files.
- Pages under `specification/`: at most 30 net together. The library, conformance and SQL READMEs: at most 12 changed lines together, each changing 2 to 3. ADR markers: at most 3 lines.
- `sdlc/ratchet.json` moves to the measured total, at most 766 above main after 0146 and 0148 land: 166, 220, 150, 110 and 60. The commit says what grew.
- No dependency.
- The `surfaces` rung runs, because the libraries and SQL surfaces take the new default and share the gate.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if ticket 0146 or 0148 has not landed on main.
3. Stop if more than 12 existing test files change for the new default, or if any test must change for a reason other than the count of sends. Hand back the list.
4. Stop if a replay or cache answer waits on the gate, or if a recorded run's output changes.
5. Stop if the gate needs a change to what `--jobs` or the throttle counts, or an adaptive width.
6. Stop if any plant stays green, or if `a_503_holds_every_request_to_that_address` fails once in five runs on an idle machine. Report the timings. Do not widen the margin to pass.
7. Stop if the change needs a file in `databases/` beyond the SQLite test, the SQLite notes and the retries lines of the three SQL READMEs, or any SQL product code, or any library or conformance change beyond a stated default and a test that counts the default's sends. That work is ticket 0149's or ticket 0157's.
8. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`, and unset `THINKTHEN_API_KEY` for every rung.

## Scope and exclusions

Excluded: an adaptive throttle, by Ian's ruling. The request total's check inside a request, which ticket 0149 builds. `retries()` on the library `Counters` and each binding, which ticket 0157 carries. The run facts, which B5 builds. A `retries` field on rows. The HTTP-date form of `Retry-After`. `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code. The change raises the ceiling, adds a status field and changes a default on every surface, so the code review names what it checked.

## Complexity

Contract 2; state and timing 2; reach 2; proof 2; cost of error 1; total 9. Final level: 3. The risks are a gate that deadlocks or never opens, a wait that holds a send slot, a replay that waits, and a timing test that flakes. The cap in `wait_open`, the permit recheck, the replay row and stop rule 6 guard them.

## Deferred gaps

1. Ticket 0149 builds ADR 0052 item 8: the SQL request total checked at each send, so a retry that finds it spent is not sent. Ticket 0149 must carry ADR 0052 item 8 in its scope and proof. The coordinator's issue `sdlc/issues/2026-09-26-follow-on-tickets-0149-and-0157-carry.md` records this.
2. Ticket 0157, the library settings follow-up, gives `Counters::retries()` to Rust and every binding, beside ticket 0154's `max_request_bytes`. The same issue records this.
3. B5 builds `retries` in the `thinkthen.run/1` facts.
4. A row carries no share of retries.
5. The gate reads no HTTP-date `Retry-After`, as today.
6. ADR 0017's marker lands after ticket 0148.

## What Ian can overturn

- Ian's rulings: three retries by default, one gate for each process and address, a fixed throttle that counts retries with waits holding no slot, retry counts per request, every send spending a unit of a request budget, and retries counted apart.
- Decision 2: the gate keyed by the exact posting URL.
- Decision 4: a request with spent retries closes the gate too.
- Decision 5: one wait deadline per attempt, capped by the waiter's own timeout and 60 seconds, then a send.
- Decision 7: `requests_sent` keeps every send, and `retries` is the subset.
- Decision 8: old usage files read as 0 retries under the same schema name.
- Decision 9: the mid-request budget check left to ticket 0149.
- Decision 10: no `retries` field on rows.
- Decision 11, the coordinator's: the build order 0146, 0148, 0155, 0154, B5.
- Decision 12: "a retry counts against the request limit" read as the SQL process total, with the library's `max_requests` left a record count.

## Closes

No issue. No issue was filed for these rulings, and this ticket and ADR 0052 record them.

## Evidence

- Starts from: Ian's rulings of 2026-09-26, relayed by the coordinator with his two corrections. The code at `origin/main` `e0a6150a`: `engine/http.rs::post_observed`, `bounded_wait` and `honored`; `engine/mod.rs::Widths` and `process_width`; `engine/usage.rs::Counts` and `Counters`; `cli/status.rs`; `cli/args.rs:171`; `public/settings.rs:237`; `public/bulk.rs::within_limit`; `databases/duckdb/src/engines.rs::within_total` and `databases/sqlite/src/scalars.rs::flush`; `conformance/backend/src/arms.rs`'s 503 arm. The unit tests `a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one` and `a_replayed_answer_takes_no_permit`. Ticket 0148 at `d1afe98a` for the library default, its pages and its shared cases. `Cancel::observed` and `observed_block` at `engine/mod.rs:186-208`, and `Client::gated` at `engine/http.rs:98`. Ticket 0089's premise that a retried status may be billed. `specification/backends.md` "The request", `settings.md`, `check.md` and `recording.md`. No experiment ran. The rulings need no measurement, and the loopback proves the behavior.
- Keeps: The retried statuses, the headers read, the 60-second cap, the doubling from one second, and no resend after a transport failure. The throttle's number and what it counts. Replay and cache answers that never wait. Every row's `meta.requests_sent`. The library's `max_requests`. Every SQL product file.
- Changes: `--max-retries` and the engine default become 3. A process-wide backoff gate for each posting URL, closed by a retried status and waited on before every live attempt. A `retries` count in the engine counters, the usage file and `thinkthen status`. ADR 0052, and four specification pages.
- Proof: Three new loopback and public-API tests, one amended unit test, and the existing tests that count the default's sends, each with its plants, under "Proof".
- Defers: The SQL total checked at each send (0149). `retries()` on the libraries (ticket 0157). `retries` in the run facts (B5). A row share of retries. The HTTP-date header. ADR 0017's marker after 0148.
