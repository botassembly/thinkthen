---
flow: build
priority: 166
opens: crates/thinkthen/src/public/options.rs crates/thinkthen/src/public/batch.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/tests/public_controls.rs libraries/c/include/thinkthen.h libraries/c/DESIGN.md libraries/c/tests/door/main.rs libraries/c/tests/c/cancel.c libraries/c/ratchet.json libraries/c/ratchet.c.json sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0166: A cancelled call never succeeds

Status: accepted. The coordinator accepted it on 2026-09-27 after a fresh read-only review with four fixes. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A C host fires a call's token while the backend holds the reply. Today the call returns `THINKTHEN_OK` and writes the answer. After this ticket it returns `THINKTHEN_ECANCELLED`, and its out parameters keep what they held. The call starts no request and no retry after the fire. A request already sent finishes. Every reply reaches the counters, and a complete answer reaches the cache when the engine caches, so the same call with a fresh token pays nothing more. The header and `DESIGN.md` state that contract exactly.

The fix sits in the public door that every binding calls, so the Rust library, the Polars extension and the C door share it. The Zig and Go packages call the C door, and both issues name this fault as their blocker. The coordinator assigned this ticket on 2026-09-27. Ian can overturn each decision below.

## Prior experiment evidence

- Local experiment 273 reproduced the fault three times through Python ctypes calling the C door directly, and in all four Zig consumers (`sdlc/issues/2026-09-26-cancelled-c-scalar-call-can-return-success.md`, `sdlc/issues/2026-09-26-zig-consumer-proof-needs-a-supported-package.md`). Held bulk cancellation, held deadlines and fresh-token recovery passed there.
- Local experiment 274 reproduced it in all four Go consumers (`sdlc/issues/2026-09-27-go-consumer-proof-needs-a-supported-package.md`).
- This ticket's author ran scratch tests against `origin/main` `a057c594` on loopback listeners, with no key and no network, and deleted them. With the token fired while the reply was held and the reply released 250 ms later:
  - `decide_with`, `details_with`, `find_with` and a one-rule `relate_with` returned their answers.
  - `decide_many_with` over one and three texts, `filter_with`, `rank_with`, `annotate_with` and `recognize_with` returned `Cancelled`.
  - With the release right after the fire, `decide_many_with` over one text and a one-rule `relate_with` returned their answers in five of five runs.
  - With a first reply of 503 and `retry-after-ms: 10`, held 300 ms, and the token fired when the request arrived, `decide_with` sent the retry and returned `Yes` in five of five runs. The listener counted 2 requests. A 10 ms wait leaves the calling thread's 50 ms tick a chance to see the token first, so the proof rows below ask for `retry-after-ms: 0`. The worker's first `stop_or_remaining` inside `wait(0)` then decides alone.
- The author then ran the design below as a scratch patch. Every row above returned `Cancelled`, the retry row counted 1 request, and a cached engine answered a fresh-token call from the reply the cancelled call had paid for. `cargo test -p thinkthen --lib --test public_controls` and `cargo test` in `libraries/c` passed with the patch. The patch was reverted.

## What happens today

Read from `origin/main` `a057c594`.

- `public/options.rs:209-220`, `Stop::begin`, refuses a token that fired before the call. It builds the call's `Cancel` from a fresh flag and the deadline. The caller's token never enters that `Cancel`.
- `public/options.rs:234-247`, `Stop::interrupted`, reads the token and then the host check. `Stop::run` at lines 252-261 hands it to the engine as a check through `Cancel::with_check`.
- `engine/mod.rs:131-135`, `Cancel::checked`, runs the check only on the calling thread. `engine/mod.rs:156-160`, `poll_between_sends`, skips the check while a send is in flight. So the token is read only on the calling thread, only between sends, and only at the 50 ms tick.
- `engine/workers.rs:22-41`, `on_worker`, runs a single call's attempt on a worker and polls from the calling thread. When the send ends, it joins the worker and returns its result.
- `engine/http.rs:126` checks `cancel.stop_or_remaining()` before each attempt, and line 153 waits for a retry through `cancel.wait`. Both run on the worker, and both read only the call's own flag (`engine/mod.rs:121-126`). A retry wait shorter than one tick sends the retry before the calling thread reads the token.
- `public/options.rs:258-260`: `Stop::run` returns the engine's result after `finish`. `finish` at lines 264-269 resumes a held check panic and reads no token. So a single call whose token fired during its last send returns success.
- `public/batch.rs:166-183`, `Stream::pull`, reads the token at each tick and fires the call's flag. At `Event::End` (lines 204-208) it returns the scheduler's result after `finish`. A token that fires after the last row is queued, or in the same tick as the last reply, ends the batch with no error.
- The C door calls these paths on the caller's thread. `thinkthen_decide_opts` reaches `door::decide`, which calls `details_with` (`libraries/c/src/door.rs:54-73`). `thinkthen_decide_many_opts` collects `decide_many_with` rows (`door.rs:76-98`). `thinkthen_call_opts` calls the same public functions (`libraries/c/src/call.rs:114-206`).
- `thinkthen.h:18-20` says "the wait checks the token and the budget on every tick, so a cancel or a spent budget ends the call within one tick." A send in flight is never interrupted, so a cancel waits for it. `thinkthen.h:154-157` says the calls carrying a fired token "return THINKTHEN_ECANCELLED with no results". `DESIGN.md` section 1 says the same.

### Which bindings share the path

Every binding in this repository depends on `crates/thinkthen` and calls the public API. None calls the C door.

- **The Rust library, the Polars extension, the C door, and the Zig and Go packages over it** pass the caller's token into `CallOptions` and wait inside the engine. They have the fault, and this ticket fixes it for all of them.
- **Python, Ruby, R, DuckDB, SQLite and PostgreSQL** run each call on a worker thread and wait on their own thread in 50 or 100 ms ticks. On a stop they fire their own token and return cancellation at once, leaving the worker to finish. They pass the engine their own token, never the caller's. So a held reply cannot make them succeed, and this ticket changes only what their abandoned worker does: it stops a pending retry at once. One narrower race remains in each. Their loops take an answer that lands in the same tick as the stop before they read the stop. That fix sits in six separate wait loops, not in the shared path, so this ticket files it as an issue and leaves it.
- **TypeScript** rejects the promise from the `AbortSignal` handler in the event loop and detaches the worker. A late answer is dropped. It does not have the fault.
- **The command** uses its own flag and the SIGINT rule of ADR 0017's amendment of 2026-09-22, which prints completed output. This ticket does not change it.

## Retained behavior

- A token that fired before the call begins still refuses it before anything is sent, with "the call was cancelled".
- A request already sent still runs to its end, bounded by the attempt timeout and the budget. No transport is interrupted and no worker is abandoned.
- The reply that arrives after the fire is decoded and counted as today. A complete answer, one with no failed question, is cached when the engine caches. A later call with a fresh token replays it and sends nothing.
- A host check that returns true stops its own call alone. The caller's token stays clear, as `public_controls.rs:232` pins.
- A check's panic still resumes on the caller after every worker joins.
- A batch still yields the rows it finished before its end. It now ends with the cancellation error.
- The command, its SIGINT rule and its exit codes stay as they are.
- The six bindings with their own wait loops keep their behavior.

## The change

### The engine reads the token on every thread

`Cancel` in `engine/mod.rs` gains one field, `token: Option<Arc<AtomicBool>>`, and one builder beside `with_deadline`:

```rust
/// Share this stop flag with one call that a caller's token also stops.
pub(crate) fn with_token(&self, token: Option<Arc<AtomicBool>>) -> Self
```

`stop_or_remaining` reads the token beside the call's own flag, before the host check. Every checkpoint goes through it: the width gate, the check before each attempt at `http.rs:126`, the retry wait, and the schedulers' feeds. So a worker sees a fired token before its next attempt or retry. `fired()` keeps its meaning, the call's own flag, because the command reads it. `with_check` already copies every field, so the token reaches the calling thread's copy.

`CancelToken` in `public/options.rs` gains a private `flag()` that clones its `Arc`. `Stop::begin` builds the call's `Cancel` with `.with_token(options.cancel.map(CancelToken::flag))`. A fire stays one atomic store, so `thinkthen_cancel` still allocates nothing and a signal handler may still call it.

### The door reads the token after the last request

`Stop::finish` takes the call's result and returns the result the caller gets:

```rust
pub(crate) fn finish<T>(&self, result: Result<T, Error>) -> Result<T, Error>
```

It resumes a held check panic first, as today. Then, when the caller's token has fired, it returns `Error::cancelled()` in place of the result. `Stop::run` returns `self.finish(result)`. `Stream::take` at `Event::End` passes the joined result through `self.stop.finish(ended)`. Both reads come after every request of the call has ended, because `run` returns after the engine joins its workers and `End` comes after `join`.

A token that fires after that read races the call's return. The call returns what it had. The header says so.

### Why this opens `engine/mod.rs`

Ticket 0155 opens `engine/mod.rs` and `engine/http.rs`. This ticket stays out of `http.rs`. It cannot stay out of `mod.rs`. The retry leak needs a worker thread to see the token, and a worker reads only what `Cancel` holds. The token enters the engine today only through the check, which `Cancel::checked` confines to the calling thread so a host check never runs on a worker. Every route outside `mod.rs` fails a rule:

- Letting the call's flag be the token's own flag would let a call's own stop fire the caller's token. `public_controls.rs:232` pins that it does not.
- Registering each call with the token, so a fire sets every call's flag, puts a lock in `thinkthen_cancel`. `DESIGN.md` section 1 promises that a fire allocates nothing and that a signal handler may fire it.
- A watcher thread per call that polls the token adds a thread to every call and keeps the same one-tick window.

The edit adds about 10 lines to `Cancel` at lines 33-126. Ticket 0155 edits `Permit` and `Widths`, far below. The two diffs share no line, and 0155's planned `wait_open` observes cancellation through `stop_or_remaining`, so it inherits the token read.

### Pages

`libraries/c/include/thinkthen.h`, lines 17-20, from "A C host hears" to "within one tick.", becomes:

> A C host hears its own interrupts by firing a token from another thread. The call reads the token before every request and every retry, and its wait reads the token and the budget on every tick. A spent budget ends the call within one tick. A fired token starts no new request and no retry. A request already sent runs to its end, within its attempt timeout and the budget, and a complete answer it brings still reaches the cache and the counters. Then the call returns THINKTHEN_ECANCELLED with no results, whatever that reply held. The call reads the token a last time after its last request ends; a token fired after that read does not change the result.

`thinkthen.h:154-157`, the comment on `thinkthen_cancel`, becomes:

> Fire a token: the calls carrying it start no new request or retry, let the requests they sent finish, and return THINKTHEN_ECANCELLED with no results, even when a sent request's reply arrives after the fire. A token is one-shot: a fire leaves it fired, a second fire is ignored, and no call re-arms it. Thread-safe from any thread, and it allocates nothing; a null token is accepted and ignored.

`libraries/c/DESIGN.md` section 1, the sentence "No new request starts after the fire, requests already sent finish, and the calls that carried the token return `THINKTHEN_ECANCELLED` with no results." becomes:

> No new request or retry starts after the fire. A request already sent finishes within its attempt timeout and the budget. A complete answer reaches the cache, and every reply reaches the counters, so the same call with a fresh token replays it and pays nothing more. The call that carried the token then returns `THINKTHEN_ECANCELLED` with no results, whatever the reply held, a failure included. The call reads the token on every thread it uses, and once more after its last request ends. A fire after that last read does not change the result.

The `CancelToken` doc comment at `public/options.rs:27-30` becomes:

> A cancel flag a caller may set from any thread.
>
> Every clone shares one flag. A call that carries it starts no request or retry after the fire, lets sent attempts finish, and returns [`Error::Cancelled`] whatever those attempts answered. A batch ends with that error after the rows it already yielded.

The `ECANCELLED` comment at `thinkthen.h:74` stays true and does not change.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A fired token wins over every result.** A call whose token fired returns cancellation even when a sent request failed after the fire, for example with a 422. The header already promises `THINKTHEN_ECANCELLED` to every call carrying a fired token, and the six bindings with their own wait loops already return cancellation whatever their worker met. The alternative keeps a failure's own kind and replaces only success. It would tell a host more, and it would weaken the header's promise.
2. **The door reads the token after the join.** The answer is decoded, cached and counted first, so no paid reply is lost and a fresh-token call never pays twice.
3. **The engine reads the token on every thread.** This closes the retry leak with one field. It opens `engine/mod.rs` beside ticket 0155 on disjoint lines.
4. **A batch keeps yielding rows it finished before its end.** Only its end changes. Withholding rows after a fire would change the lazy iterator's contract for Rust consumers, and the C door already returns no rows on any error.
5. **The binding wait-loop race goes to an issue.** It is not in the shared path.

## Edge cases

The C rows run in order in one program on one engine, against the conformance backend's held arm, with a fresh cache and the process throttle of 4. "Held" means the backend holds the reply until the harness lets it go.

| # | Input | Expected |
| --- | --- | --- |
| 1 | `thinkthen_decide_opts` with token A and text `one`. The harness waits for 1 request and writes a line. The program's reader thread fires A and prints `fired A`. The harness reads that line, waits 250 ms as slack, then releases the held replies | `THINKTHEN_ECANCELLED`. `thinkthen_error_message` is `the call was cancelled`. `out` still holds `{7, 7.0}`. The backend has counted 1 request |
| 2 | `thinkthen_call_opts` of `{"decide":"Is this a complaint?","evidence":"two"}` with token B, fired the same way when the count reaches 2 | NULL. The error code is 5 and the message `the call was cancelled`. Count 2 |
| 3 | `thinkthen_decide_many_opts` over five texts with token C, fired the same way when the count reaches 6 | `THINKTHEN_ECANCELLED`. All five `out` slots still hold `{7, 7.0}`. Four requests were in flight. The fifth text is never sent: count 6 |
| 4 | `thinkthen_decide_opts` with a fresh token and the question and text of row 1, after every reply is released | `THINKTHEN_OK`, `THINKTHEN_YES` and 0.9, from the reply row 1 paid for. Count 6 |
| 5 | `thinkthen_decide_opts` with token A again | `THINKTHEN_ECANCELLED` before any request. Count 6 |
| 6 | `thinkthen_call_opts` of `{"usage":true}` | `requests_sent` 6 and `cache_answers` 1 |
| 7 | Rust `decide_with` with a token. The listener holds the first reply at a barrier and then answers 503 with `retry-after-ms: 0`. The test fires the token when the listener counts 1 request, then passes the barrier. A later request would get the decide answer | `Cancelled`. The listener counts 1 request. Today it sends the retry, counts 2, and returns `Yes` |
| 8 | The same with a first reply of 422 | `Cancelled`, count 1. Today it returns the backend kind |
| 9 | Rust `decide_many_with` over one text on an answering listener. The test pulls one row, fires the token, and pulls twice more | The first pull is the row. The second is `Cancelled`, and the third is `None`. Count 1. Today the second pull is `None` |
| 10 | A token fired before the call | Refused before any request, as `a_spent_batch_sends_nothing` and `tests/c/opts.c` pin today |
| 11 | A host check returns true while a shared token is clear | The call is cancelled, and the shared token stays clear, as `public_controls.rs:232` pins today |

## Tests and proof

| Test | What it proves | Deliberate breaks that turn it red |
| --- | --- | --- |
| `tests/c/cancel.c`, new, driven by `a_token_fired_during_a_held_reply_cancels_the_call`, new in `libraries/c/tests/door/main.rs` | Edge rows 1 to 6 through the public C door. The program reads one line from standard input for each of tokens A, B and C, on a second thread, fires that token, and prints `fired A`, `fired B` or `fired C` once `thinkthen_cancel` returns. The harness drives the backend: `wait(1)`, a line, read `fired A`, 250 ms of slack, `round`; then `wait(2)`, a line, read `fired B`, 250 ms, `round`; then `wait(6)`, a line, read `fired C`, 250 ms, `round`; then `release` and end of input. So each fire lands before its release by the program's own word. The program checks each code, message and out value itself and prints nothing else, as the other C rows do. The harness pins exit 0, standard output equal to the three `fired` lines, empty standard error, and a final count of 6 | (a) `finish` stops reading the token: rows 1 and 2 return `THINKTHEN_OK`. (e) A cancelled send skips the cache write, planted in `engine/request.rs` as an early return on `cancel.stop()` after `on_worker`: row 4 sends again, and the count reaches 7 |
| `a_token_fired_during_a_send_ends_the_call_cancelled`, new in `crates/thinkthen/tests/public_controls.rs`, a two-row table | Edge rows 7 and 8. Each row pins the kind and the listener's count | (b) `stop_or_remaining` stops reading the token: row 7 sends the retry and counts 2. (c) `finish` replaces only success: row 8 returns the backend kind |
| `a_token_fired_before_a_batch_ends_ends_it_cancelled`, new in `public_controls.rs` | Edge row 9. It pins all three pulls and the count | (d) `Stream::take` passes `End` around `finish`: the second pull is `None` |

`libraries/c/tests/door/main.rs` splits `run` into `start`, which spawns the program with the door's environment, and `finished`, which waits for it under the minute's limit and checks the key never reached its output. `run` keeps its signature and calls the two, so its six callers do not change. The new test drives the program's standard input between them. The build runs each deliberate break by hand, confirms the named test turns red, and records the failing line.

Overlap was checked.

- `a_stop_at_the_throttle_gate_sends_nothing_new_and_sent_work_finishes` fires a token for a call waiting at the gate, before any send. It never fires during a send.
- `a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` and `a_host_interrupt_during_relate_chunks_sends_nothing_new` stop through the host check while requests are held. Both return `Cancelled` today, because the calling thread polls between feeds.
- `a_check_runs_before_a_held_send_and_never_during_it` pins that the check never runs in a send. It still holds, because the token read in `stop_or_remaining` is not the check.
- `a_stop_during_a_retry_wait_sends_nothing_new` stops through the check during a ten-second retry wait. Row 7 fires the token during the send, before a wait shorter than one tick.
- `tests/c/opts.c` fires a token before the call. No C row fires during a send.
- `find_with`, `relate_with` and `recognize_with` go through `Stop::run` as `decide` does. Break (a) reaches them through the same line, so no row repeats them. The C rows cover the typed scalar, the JSON scalar and the typed bulk door, which is what the issue asks.

The four questions:

- **What behavior does it protect?** A call whose token fired never returns an answer, starts no request or retry after the fire, keeps the paid reply in the cache, and leaves its out values alone.
- **What credible regression fails it?** A token read dropped from the door or the engine, a fix that discards the paid reply, a batch end that skips the read, and a rule that lets a failure through.
- **Why does no existing test catch it?** No test fires a token while a send is in flight. The held tests stop through the check, which the calling thread already polls between feeds, or fire before the call.
- **Does it need a test-only hook?** No. The conformance backend holds and releases replies, the listener counts requests at a barrier, and the C program reads standard input.

The gate ladder `sdlc/scripts/{install,lint,test,spec,surfaces}` runs before handing back. The `surfaces` rung runs, because every binding's engine path changes.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`.

- `engine/mod.rs`: at most 12 net.
- `public/options.rs`: at most 12 net, the doc comment included.
- `public/batch.rs`: at most 1 net.
- `tests/public_controls.rs`: at most 70 net.
- `sdlc/ratchet.json`: at most 95 above main. The scratch patch measured 19 net lines of source.
- `libraries/c/tests/door/main.rs`: at most 45 net. `libraries/c/ratchet.json` moves by the same amount.
- `libraries/c/tests/c/cancel.c`: at most 120, new. `libraries/c/ratchet.c.json` moves by the same amount.
- Pages: the header, `DESIGN.md` and the doc comment only, at most 15 net.
- No dependency. No paid call. The build needs none.

Each ratchet moves to the measured total in the commit that needs it, and that commit says what grew. The source grows by one field, one builder, one read and one result rule. The build looks first for duplication to delete in `Stop` and `Stream`.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs `engine/http.rs`, or any part of `engine/mod.rs` outside `Cancel`.
3. Stop if an existing test in `public_controls.rs`, `tests/c` or `tests/door` changes its expected result.
4. Stop if any deliberate break stays green.
5. Stop if a C or Rust row passes or fails by timing alone. Each fire waits for the backend's or the listener's count, each release waits for the program's `fired` line or the test's own fire, and the 250 ms is slack only.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

This ticket builds on main now. It blocks the Zig and Go packages.

It shares files with tickets that are ready but not yet built. None of the shared lines overlap:

- Ticket 0155 opens `engine/mod.rs`, all of `crates/thinkthen/tests`, `libraries` and `conformance`. This ticket edits `Cancel` in `mod.rs`, one test file, and the C door's tests and pages. 0155 edits `Permit` and `Widths`.
- Ticket 0146 opens `public/batch.rs` and all of `crates/thinkthen/tests`. This ticket changes one line at `Event::End`.
- Ticket 0148 opens `libraries/c`. This ticket adds one C file and one test and edits two pages there.

The second ticket of each pair to land merges these lines.

## Scope and exclusions

Excluded: the same-tick race in the six bindings' own wait loops, withholding batch rows after a fire, the command's SIGINT path, deadlines, the Zig and Go packages, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 2; State/timing 3; Reach 3; Proof 2; Cost of error 2; Total 12. Minimum floor: level 3 for cancellation. Final level: 3. The risks are a paid reply lost to the cache, which row 4 guards, and a retry sent after the fire, which row 7 guards.

## Deferred gaps

- Python, Ruby, R, DuckDB, SQLite and PostgreSQL each take an answer that lands in the same tick as their stop. `sdlc/issues/2026-09-27-a-binding-wait-loop-lets-an-answer-beat-a-stop-in-the-same-tick.md` records it. It was found by reading and not run.
- A Rust consumer of a lazy batch still sees rows the batch finished after the fire, then the error. Decision 4 keeps it.
- A fire after the call's last token read does not change the result. The header states it, and no test can pin a race of that width.
- The Zig and Go packages rerun their gates with the strict cancellation contract after this lands. Their issues own that.

## What Ian can overturn

- Decision 1: a fired token wins over a failure too. The alternative keeps a failure's kind.
- Decision 3: the engine reads the token on every thread, beside ticket 0155's file.
- Decision 4: a batch yields its finished rows before the cancellation error.

## Closes

`sdlc/issues/2026-09-26-cancelled-c-scalar-call-can-return-success.md`. It lifts the native blocker named in `sdlc/issues/2026-09-26-zig-consumer-proof-needs-a-supported-package.md` and `sdlc/issues/2026-09-27-go-consumer-proof-needs-a-supported-package.md`. Those issues stay open for their packages.

## Evidence

- Starts from: Local experiments 273 and 274, as the cancellation, Zig and Go issues record them, and the author's scratch runs at `origin/main` `a057c594`, before and after a scratch patch of this design. The code: `public/options.rs:27-30`, `:209-220`, `:234-247`, `:252-269`; `public/batch.rs:166-183`, `:204-208`; `engine/mod.rs:33-42`, `:121-135`, `:156-160`; `engine/workers.rs:22-41`; `engine/http.rs:126`, `:153`; `libraries/c/src/door.rs:54-98`; `libraries/c/src/call.rs:114-206`; `thinkthen.h:18-20`, `:74`, `:154-157`; `DESIGN.md` section 1.
- Keeps: A pre-fired token sends nothing. Sent requests finish, bounded by the attempt timeout and the budget. The paid reply is decoded and counted, and a complete answer is cached when the engine caches. A host check stops its call alone. A check's panic resumes after the join. A batch yields its finished rows. The command and the six bindings with their own wait loops behave as today.
- Changes: A call whose token fired returns cancellation whatever its requests met. The engine reads the token on every thread, so no retry starts after the fire. The header, `DESIGN.md` and the `CancelToken` doc state the exact contract.
- Proof: A C program over the held arm for the typed scalar, JSON scalar and bulk doors with fresh-token recovery and counters, two Rust rows for the retry and the failure, one Rust row for the batch end, five deliberate breaks each run by hand, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The same-tick race in six bindings' wait loops, rows a lazy batch yields after the fire, a fire after the last read, and the Zig and Go gate reruns.
