# Area 3: Process-wide send limits

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

One process shares one throttle, one set of per-address 429 gates and pacers, and one request and estimated-input total. A fork rebuilds all of it, and every live attempt of every surface passes through it.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/`, `databases/` or `conformance/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, owner | `engine/limits.rs` (210), `engine/backoff.rs` (252, about 90 of them inline tests), `engine/send_budget.rs` (169), `engine/budget.rs` (159), `engine/process.rs` (105) |
| Code, send path | `engine/http.rs` (496 of 500), `engine/http/retry.rs` (46), `engine/workers.rs` (171), `engine/pipeline/send.rs` (209, shared with area 1), `config/backends.rs` (287, the rate field only) |
| Code total | about 1,610 lines across the first two rows, `send.rs` and `config/backends.rs` excluded |
| Tests | About 80 tests. Unit: `engine/width_tests.rs` (11), `engine/http/tests.rs` (13), `engine/facade/fork_tests.rs` (8), `engine/process/tests.rs` (4), inline in `backoff.rs` (5), `limits.rs` (1), `budget.rs` (1), `send_budget.rs` (1). Integration: `crates/thinkthen/tests/backend/backoff.rs` (6), `tests/backend/parallel.rs` (12), `tests/backend/named_backends/rate.rs` (2), `tests/library/public_backoff.rs` (3), `tests/backend/limits.rs` (4, these pin request-size and estimated-input limits, not send limits). Consumer: `conformance/consumer/fork-probe/tests/fork.rs` (9) |
| Contract | `specification/backends.md` "The request"; `specification/settings.md` rows Retries, Throttle, Requests a minute, Process request total, Estimated input admission total; `specification/records.md` "The default backend's published limits"; ADR 0052, ADR 0111 section 8, ruling 14 of `sdlc/planning/cleanup-2026-09-30.md`; tickets 0308, 0343, 0089, 0143 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 1,610 nonblank lines in seven owner files plus `http.rs`, with about 200 of them inline unit tests |
| States and concurrency | 5 | A process-ID-keyed rebuild cell with an owner and a rebuild marker (`engine/process.rs:21-78`), a width gate on a mutex and condvar (`engine/limits.rs:97-183`), a gate map and a pacer map on two mutexes and a condvar (`engine/backoff.rs:14-19`), lock-free CAS counters with refund on drop (`engine/budget.rs:86-137`), and workers that mask host signals (`engine/workers.rs:168`). It is used three times: `engine/limits.rs:26`, `engine/facade.rs:98`, `engine/budget.rs:13` |
| Rules and refusals | 4 | About 30. Eleven retried statuses, 60 s cap, doubling from 1 s, jitter of half to all, server header as floor, `retry-after-ms` first, zero raised to 1 s, HTTP-date ignored, per-request retry counts, width 1 to 32, first explicit width wins, a different width refused by one fixed sentence, rate 1 to 60,000 with three config sentences and one variable sentence, no default rate, variable outranks file, slot past the deadline fails at once, three send-budget denials, three estimated-input denials, and a zero limit refuses before the key is read |
| Surfaces touched | 5 | 22 of 22. Every live attempt of the command, the Rust API, Polars, the C door, every language over it and the three SQL hosts goes through `engine/http.rs:251-332` |
| Settings | 4 | Seven rows: Retries, Timeout, Throttle, Requests a minute, Process request total, Estimated input admission total, Named backends (for the per-backend rate) |
| Contract weight | 3 | Four spec pages (`backends.md`, `settings.md`, `records.md`, `result.md`) and three ADRs (0052, 0111 section 8, 0114), with ruling 14 |
| Churn and debt | 5 | `engine/limits.rs` was written at 08:49 on 2026-09-30 and changed again at 09:00 (`2038f4c11`, `adf809a60`). 36 commits on the seven owner files since 2026-09-23 and 70 with `http.rs` and `config/backends.rs`. One open issue on the send path (`sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`) |

Mean 4.1, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The code follows the contract on every primary path I checked. A backoff wait holds no send slot (`crates/thinkthen/src/engine/http.rs:121-125`). A paced wait holds none either (`engine/http.rs:122-125`). The failing attempt closes the gate and frees its slot in one step under the width lock (`engine/limits.rs:191-203`). A slot past the deadline fails at once and reserves nothing (`engine/backoff.rs:95-99`, pinned at `:243-255`). An unheaded wait is capped at 60 s and the timeout, and a server header is a floor (`engine/http/retry.rs:28-46`, `engine/http.rs:266`). No default rate survives: `rg` finds no `1,000 a minute` default in code, spec or fixtures except the historical note at `specification/records.md:147`. The SQL hosts' former copies are gone: `rg` finds no gate, pacer or counter in `databases/postgresql/src` or `databases/duckdb/src`, only `SendBudgetDenial` matches at `databases/postgresql/src/call.rs:77`. Minor drift: ADR 0052 still says a retried status is "429, 500, 502, 503, 504 or 529" (`sdlc/planning/adr/0052-retries-back-off-at-the-provider.md:10`) where the code and `backends.md` hold eleven (`engine/error.rs:105`). `engine/limits.rs:2-3` says one rebuild replaces all three, while `SendBudget` carries its own rebuild cell (`engine/budget.rs:13`, `:36-46`) |
| Reliability | C | Failure paths are well tested: fifty racing first widths select exactly one (`engine/width_tests.rs:118`), racing forked children publish one state (`engine/process/tests.rs:154`), a child beside a busy parent uses fresh state (`engine/facade/fork_tests.rs:100`), a cancel ends a paced wait (`engine/backoff.rs:227-239`), and a deadline during backoff reserves only the first send (`engine/http/tests.rs:352`). Three things hold it at C. Routine tests assert wall-clock spacing: the rate tests need 400 ms of a 500 ms spread (`crates/thinkthen/tests/backend/backoff.rs:137-140`, `tests/backend/named_backends/rate.rs:58-90`), and one test sleeps 1.2 s (`engine/http/tests.rs:479`). Ticket 0352 (no wall clock in routine tests) is still in progress, and ticket 0340 closed load flakes in this area. The owner was written and rebuilt on 2026-09-30, with review fixes in `c8db21ba5`, `99f04479e`, `44a5abb97` and `adf809a60`. A server `Retry-After` has no ceiling, so one reply of `99999` waits 99,999 s (`engine/http/tests.rs:22` pins it, `engine/backoff.rs:48-52` turns an overflow into `Never`), and only a cancel or deadline ends it |
| Maintainability | B | One owner for the three limits (`engine/limits.rs:15-24`) and one primitive for fork safety (`engine/process.rs`), with no lint suppression in any owner file. Three weaknesses. `engine/http.rs` holds 496 of its 500 lines, and the next rule added to `post_with_reservation_check` fails the cap. `client_width` branches on `cfg(test)` inside a production path (`engine/limits.rs:65-77`). `Cancel` carries three overlapping budget fields (`send_budget`, `process_budget`, `call_total`, `engine/mod.rs:47-50`) with their logic in a second file (`engine/send_budget.rs:48-138`), and a reader must hold both to follow one reservation |

## Strengths

- One owner and one rebuild rule. `Limits` holds the width, the gates and the total, and a fork replaces the set once (`engine/limits.rs:15-55`, `engine/process.rs:43-78`).
- The permit and its refund are drop-safe. A reservation refunds unless usage marked the attempt, and both the request and estimate counters refund together (`engine/budget.rs:119-137`, `engine/send_budget.rs:154-182`).
- The wait order is explicit and checked after each acquire. Gate wait, pace, width, then a recheck of the gate, with the permit dropped when the gate closed meanwhile (`engine/http.rs:113-131`).
- The pacer is per address and takes the next slot under one lock, so many threads keep one interval apart (`engine/backoff.rs:80-107`, test `:196-223`).
- The contract names each limit's scope. The settings rows say separate processes each get the full rate and each PostgreSQL backend paces alone (`specification/settings.md:80`).

## Cleanup

1. **Replace the wall-clock rate assertions with recorded slots.** Where: `crates/thinkthen/tests/backend/backoff.rs:113-140`, `tests/backend/named_backends/rate.rs:58-90`, `engine/http/tests.rs:479`. Why: the pacer cannot be tested without time. A clock seam or the slot values in `Gates::slots` would prove spacing with no sleeps, and ticket 0352 is open on the same theme. Size: M. Blocks 0.1: no.
2. **Decide a ceiling for a server's `Retry-After`.** Where: `engine/http/retry.rs:28-46`, `engine/backoff.rs:48-64`, `engine/http/tests.rs:22`. Why: ADR 0052 item 5 makes the header a floor that no cap shortens, so a bad header can stall a command with no deadline for a day with no output. Either cap it, or print the wait once. It needs an ADR amendment and Ian's choice. Size: M. Blocks 0.1: no.
3. **Split `engine/http.rs` before it reaches its cap.** Where: `engine/http.rs` (496 of 500). Why: move `Key`, `Roots` and the transport-error mapping (`:25-84`, `:500-530`) into their own modules. `Key` also belongs with area 6. Size: S. Blocks 0.1: no.
4. **Collapse the three budget carriers on `Cancel`.** Where: `engine/mod.rs:47-50`, `engine/send_budget.rs:48-138`. Why: one `SendLimits` value built once per call would remove the tuple, the `process_limit` merge and three `with_*` setters. Check it with area 4, which owns `Cancel`. Size: M. Blocks 0.1: no.
5. **Take the test branch out of the production client.** Where: `engine/limits.rs:65-77`. Why: a unit-test binary gets a private gate through a global flag and a `cfg(test)` return in shipping code. Pass the gate in `Client::new` and let tests call `gated`. Size: S. Blocks 0.1: no.
6. **Fix two sentences of contract text.** Where: `sdlc/planning/adr/0052-retries-back-off-at-the-provider.md:10`, `engine/limits.rs:2-3`. Why: the ADR lists six retried statuses where the code holds eleven, and the module comment says one rebuild where three cells rebuild (`engine/limits.rs:26`, `engine/facade.rs:98`, `engine/budget.rs:13`). Size: S. Blocks 0.1: no.
7. **Check that a re-closed gate does not spend pacer slots (unconfirmed).** Where: `engine/http.rs:120-130`. Why: when the gate closes again after a permit, the loop drops the permit and calls `pace` again, which spends another slot with no send. The code comment covers a stop, not this loop. I inferred the effect from the code and did not run it. Size: S. Blocks 0.1: no.
8. **Move the stop check out of the width lock (unconfirmed).** Where: `engine/limits.rs:158-162`. Why: `stop_or_remaining` can run a host callback (`engine/mod.rs:198-207`) while the width mutex is held. The host check only runs on the thread that made the call, and the width wait normally runs on a worker, so I found no path that does both. Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: every owner file in full, `engine/http.rs` in full, `engine/pipeline/send.rs`, the rate parsing in `config/backends.rs`, ADR 0052, ADR 0111 section 8, the backend and settings text for all seven rows, and the names and structure of the rate, backoff, width, process, fork and http tests, with the timing tests read closely.

Not checked: tickets 0308, 0343 and 0304 slices 3d and 3e were not read, only their commit subjects. The bodies of `tests/backend/parallel.rs`, `conformance/consumer/fork-probe` and `engine/process/tests.rs` were sampled. No test was run, so items 7 and 8 are inferred. The SQL hosts' own settings code was searched for leftover copies but not read. The 21 surfaces' throttle settings rows were not checked against each binding.
