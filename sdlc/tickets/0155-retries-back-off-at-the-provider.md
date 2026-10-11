---
flow: build
priority: 155
opens: sdlc/planning/adr/0052-retries-back-off-at-the-provider.md sdlc/planning/adr/0034-status-and-count-only-usage.md sdlc/planning/adr/0048-records-batch-into-full-requests.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/backoff.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/src/engine/usage.rs crates/thinkthen/src/engine/usage crates/thinkthen/src/engine/width_tests.rs crates/thinkthen/src/engine/deadline_tests.rs crates/thinkthen/src/cli/schedule/width_tests.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/status.rs crates/thinkthen/src/public/settings.rs crates/thinkthen/tests/backend/backoff.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/public_backoff.rs crates/thinkthen/tests/status.rs crates/thinkthen/tests databases/sqlite/tests/test_deadline.py databases/sqlite/NOTES.md databases/sqlite/README.md databases/duckdb/README.md databases/postgresql/README.md libraries conformance specification/backends.md specification/settings.md specification/check.md specification/recording.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0155: Retries back off at the provider and count apart

Status: COMPLETE.

Opened as: 2026-10-11. after independent code review and the related runtime batch validation on 2026-09-27. The coordinator accepted it on 2026-09-26 after a fresh read-only review. It carries ADR 0052. Tickets 0146 and 0148 have landed. The 2026-09-27 work plan adds review-register items 91, 92, 119 and the message half of 118 to this build.

## Current-main reproduction and added scope

At current main `0fb8d55f`, the exact HTTP unit test `a_retry_after_header_is_read_in_seconds_and_stops_at_the_ceiling` passes: it expects a zero header to cause no wait and a large header to become 60 seconds. The source passes `requests_sent` through one hook for every attempt, drops the width permit before the private retry sleep, and has no address gate. `engine/http.rs` is exactly 500 nonblank lines and `engine/usage.rs` is 498, so the existing HTTP test module and the small usage count type must move into claimed child files before adding behavior. The reproduction was one offline, locked library test; it took 10.48 seconds to compile and 0.00 seconds to run.

The work plan also assigns these existing review findings here. Item 91 requires a positive server `Retry-After` to remain a floor, including a zero-value minimum delay; cancellation or a call deadline may end without another send. Item 92 requires bounded consumption of an error response body before the connection returns to the pool. Item 119 requires a key containing a line break to fail locally without sending, and a fixed explanation for a refused 302 without following it. The message half of item 118 requires a certificate failure to name TLS trust rather than generic reachability. Private TLS root configuration remains separate. Tests use local listeners and safe fixed diagnostics; no response body or key enters an error message.

This added scope supersedes the older ticket's 60-second then-send rule only where it would violate a valid server retry floor. The default, fixed width, exact-address process gate, per-request retry count, permit release, and separately counted retries stay as ADR 0052 settled them. The previous five-run timing campaign and whole-ladder handoff are historical plans; current routine proof uses a few held calls, actual send counts, focused format and strict policy checks, and the related batch integration checkpoint.

Review route: the accepted design had fresh read-only review. The coordinator assigns a fresh independent code reviewer for the final diff, then owns verification and landing.

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
- `Permit::release_closing(self, url, wait)`, in `engine/mod.rs`: consumes the permit. Under the `Widths` mutex it calls `close(url, wait)`, lowers `active`, notifies the waiters, and unlocks. So no waiter can take the slot before the gate is closed, and the order is part of the code's structure, not of the order of two lines. The lock order is `Widths`, then the backoff table, never the reverse. `wait_open` and the recheck take the backoff lock alone and hold no `Widths` lock.
- `wait_open(url, until, cancel)`: blocks until the gate is open or the instant `until` passes, observing cancellation and the call's deadline every 50 ms, as `Widths::acquire` does.

`post_observed` changes in three places.

1. Before each attempt it sets one local wait cap: now plus the smaller of 60 seconds and its attempt timeout. It waits outside the width permit, takes the permit, then checks the gate again. If another response closed the gate, it drops the permit and waits again. The local cap lets an unheaded exponential gate pass. A valid server retry header remains a floor even past that cap; cancellation or a whole-call deadline can end it without a send.
2. `send` returns, and the attempt is classified before the permit goes back. On a retried status, one call, `permit.release_closing(url, bounded_wait(asked, wait, timeout))`, closes the gate and frees the slot, in place of `drop((sending, permit))`. Every other outcome drops the permit as today. So a request that takes the freed slot always finds the gate closed on its recheck. The request's own doubling and retry count go on as today, and its next attempt waits in step 1.
3. When its retries are spent on a retried status, it closes the gate through the same `release_closing` call, then returns the failure.

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
3. **The gate keeps the later opening.** A valid server header gives a minimum delay, with zero raised to one second. An unheaded status uses the caller's bounded exponential wait. A later short wait never cuts an earlier long one.
4. **A request with spent retries closes the gate too.** Its status still says the backend is overloaded, and the next request would meet it.
5. **One local wait cap per attempt, and a server-header floor.** The cap covers an unheaded gate and every permit recheck. A valid server delay is never cut short by that cap. Engines may carry different attempt timeouts. A whole-call deadline or cancellation can stop either wait without another send.
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
| Three records, `--jobs 2`: record 1 meets 503 with a valid server delay, record 2 is already in flight, record 3 is queued | Record 3 waits without a send slot until the address gate opens. The throttle remains fixed and rows stay in order |
| The same, with no header | The gate follows the failing request's bounded exponential wait, not a server floor |
| Record 1 meets 429 with `retry-after: 1` | The gate closes for 1 second, as for 503 |
| Record 1 meets 400, 401, 422 or a transport failure | The gate stays open. Today's failure |
| `retry-after-ms: 120000`, `--timeout 2` | The gate remains closed for the server's two minutes; a whole-call deadline or cancellation may stop the caller without another send |
| Record 1's retries run out on 503 | The gate closes for its last wait. The run stops at record 1 as today |
| A request waiting on the only slot takes it just after a retried status frees it | The gate is already closed. The request gives the slot back and waits |
| An unheaded gate that closes again at every recheck | The request sends once its one local wait cap passes; a server-header floor still holds |
| An interrupt while requests wait on the gate | No new send. Today's cancellation lines |
| `--replay DIR` of any run | No wait, no send |
| `--cache DIR` with record 2 cached, record 1 meeting 503 | Record 2's answer comes from the cache. Only live sends wait |
| Two engines in one process at the same address, one with `max_retries(0)` meets 503 with a 500 ms header | Its call fails at once; the other engine's live call with a 100 ms whole-call deadline sends nothing. A cached answer proceeds |
| Two engines at two addresses, one meets 503 | The other's call answers at once |
| A forked child after its parent closed a gate | The child's gate is open |
| A usage file written before this ticket | Reads as 0 retries. `status` prints `month_retries 0` |
| `thinkthen status` after the first edge row | `month_requests_sent 4` and `month_retries 3` |
| `check` against a backend answering 503 | At most 16 attempts across its four probes |
| A batch of 10 records at `--batch 10` meets 503 | The whole batch is resent. Every batch to that address waits on the gate |

## Proof

The compact routine proof drives the compiled command and public API against local counted listeners. `backend/backoff.rs` covers four versus three versus one sends for default, explicit two and explicit zero retries; a 503 body drained before a second request reuses one connection; a linefeed key fails locally with zero sends; and a live 503 then 200 yields `requests_sent: 2`, `retries: 1` in `status`. The existing `status.rs` shape proof gains the new fields. The usage count test reads an old row without `retries` as zero.

`public_backoff.rs` warms a cache on listener A, then another engine receives 503 with `max_retries(0)` and a 500 ms server floor. A 100 ms whole-call deadline proves a new live call at A sends nothing, while the cached call at A and a call to listener B answer. Its second case gives a zero header and proves a 100 ms deadline stops after one send. This distinguishes an address gate from a process-wide gate, an engine-local gate, and a cache-path gate with only a few sends. The existing width test proves the retry releases its permit. A new deterministic width test closes the gate while the only permit is held, then proves the waiting poster rechecks it and sends nothing before its deadline. `release_closing` holds the width lock while closing the provider gate.

The HTTP table checks zero, positive subsecond and long server headers against the floor and an unheaded local cap. The loopback's accepted 429/503 arm counts change from three to four under the new default. The redirect test keeps its no-follow/no-secret assertion and asserts the safe fixed phrase. A local self-signed OpenSSL handshake produces the fixed TLS certificate diagnostic without a provider request or secret leak. The transport table keeps distinct refusal and timeout messages, and checks that `Io(InvalidData)` on a plain HTTP request or a completed response body remains generic. The previous 3-second and 5-second held-call timing campaign is not a routine gate under Ian's 2026-09-27 ruling. No live provider call or stress campaign runs.

## Budgets and stop rules

The 2026-09-27 addition of four review-register findings supersedes the original 159-line product and 659-line total caps, which measured only the accepted provider gate. The applicable source-size policy remains 500 nonblank lines per Rust file. The implementation moves the existing HTTP test module to `engine/http/tests.rs` and the existing usage count type to `engine/usage/counts.rs` rather than weakening that check. Revised review bounds are 135 nonblank lines for new `backoff.rs`, 220 for the moved HTTP test file, 60 for the moved count type, 180 for new compiled-command `backoff.rs`, and 150 for `public_backoff.rs`. The coordinator approved five extra `backoff.rs` lines on 2026-09-27 for a deterministic expired-floor/mixed-gate witness and positive-wait assertion; these protect against Condvar zero-wait spinning without a load campaign. Review any further growth before it lands; the broad source file cap remains a hard check. There is no new dependency.

Stop and report if a replay or cache answer waits, if a fixed throttle changes width, if a retry escapes `requests_sent`, or if a live/provider call or paid service becomes necessary. A file outside the main lane's exact claim needs a new claim before edit. The routine validation uses selected small contract tests, format, policy and strict lint. Full integration belongs to the related batch checkpoint. The older five-run 3-second/5-second timing campaign is opt-in, not a handoff gate.

## Scope and exclusions

Excluded: an adaptive throttle, by Ian's ruling. The request total's check inside a request, which ticket 0149 builds. `retries()` on the library `Counters` and each binding, which ticket 0157 carries. The run facts, which B5 builds. A `retries` field on rows. The HTTP-date form of `Retry-After`. `site/`.

## Routing

Owner: Codex after Ian's queue handover. Builder: retained Sol Medium agent in the claimed lane. Reviewer: a fresh independent read-only Codex session. The change raises the ceiling, adds a status field and changes a default on every surface, so the code review names what it checked.

## Complexity

Contract 2; state and timing 2; reach 2; proof 2; cost of error 1; total 9. Final level: 3. The risks are a gate that deadlocks or never opens, a wait that holds a send slot, a replay that waits, and a timing test that flakes. The one wait deadline, `release_closing`, the fixed lock order, the permit recheck, the replay row and stop rule 6 guard them.

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
- Decision 5: one local cap per attempt for an unheaded gate, with a valid server header as a floor.
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
- Keeps: The retried statuses, the headers read, the 60-second cap on unheaded waits, the doubling from one second, and no resend after a transport failure. The throttle's number and what it counts. Replay and cache answers that never wait. Every row's `meta.requests_sent`. The library's `max_requests`. Every SQL product file.
- Changes: `--max-retries` and the engine default become 3. A process-wide backoff gate for each posting URL, closed by a retried status and waited on before every live attempt. A `retries` count in the engine counters, the usage file and `thinkthen status`. ADR 0052, and four specification pages.
- Proof: Three new loopback and public-API tests, one amended unit test, and the existing tests that count the default's sends, each with its plants, under "Proof".
- Defers: The SQL total checked at each send (0149). `retries()` on the libraries (ticket 0157). `retries` in the run facts (B5). A row share of retries. The HTTP-date header. ADR 0017's marker after 0148.

## What the build taught us

- The provider gate changes assumptions in existing public-control tests. The interrupt and deadline retry rows originally shared one address; the interrupted request correctly left that address closed, so the later call sent nothing. Separate counted listeners retain a real first send and retry wait for each row. The public backoff proof separately covers zero sends at an already-closed address.
- An expired server floor could make Condvar polling use zero duration while a newer unheaded gate remained closed. The build filters expired floors and keeps a bounded deterministic regression instead of a contention campaign.
- TLS handshake errors may reach ureq as HTTPS opening-phase `Io(InvalidData)`. A real local self-signed handshake supports the fixed safe diagnostic; the structured error cannot distinguish every synthetic non-TLS invalid-data cause. The review accepted and recorded that limitation.
- Current file-size checks required moving the existing HTTP tests and usage count type before adding behavior. Preparation should identify these size constraints and address-scoped fixture assumptions before the next engine change.
- SQL per-send budget enforcement remains in 0149, public retry accessors in 0157, and private TLS roots in the open register118 work. The build and code-review records hold focused evidence and integration results; no paid call or repeated load campaign ran.
