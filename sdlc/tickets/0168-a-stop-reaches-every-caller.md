---
flow: build
priority: 168
opens: libraries/python/src/worker.rs libraries/python/tests/test_stopping.py libraries/python/ratchet.json libraries/python/ratchet.py.json libraries/ruby/src/ffi.rs libraries/ruby/tests/test_interrupt_single.rb libraries/ruby/ratchet.json libraries/ruby/ratchet.rb.json specification/settings.md CHANGELOG.md sdlc/issues sdlc/records sdlc/tickets
---

# 0168: A stop reaches every caller

Status: accepted. The coordinator accepted it on 2026-09-27 after a fresh read-only review, with the fixes that review named. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A Python or Ruby caller fires its token while the backend holds the reply. The reply then lands in the same 50 ms tick. Today the call can return the answer while the token reads as cancelled. After this ticket the call raises cancellation every time. The settings page also states what a stopped call costs later calls: a request already sent keeps its throttle place until it ends.

This is the binding half of Batch B in `sdlc/planning/work-plan-2026-09-27.md`. Ticket 0169 carries the command half. The two halves share no file, no helper and no test, so they build apart. This half builds right after ticket 0166. The command half waits for tickets 0146 and 0162. The coordinator assigned Batch B on 2026-09-27. Ian can overturn each decision below.

## Prior experiment evidence

- Ticket 0166 found the wait-loop race by reading and filed `sdlc/issues/2026-09-27-a-binding-wait-loop-lets-an-answer-beat-a-stop-in-the-same-tick.md`. That issue lands with 0166. It was not run.
- Local experiment 284, file 76, says a SQLite cancel leaves detached work holding a throttle permit. It asks for a count of detached work in `thinkthen_usage()` and a sentence on every surface. Its source is local experiment 273, report 04, finding I12.
- Local experiments 273 and 274 proved held cancellation through the C door. Ticket 0166 fixes the shared door path and leaves the six binding loops alone.
- This ticket's author read each finding on `origin/main` `18f0381e`. No binding was built or run for this ticket.

## What happens today

Read from `origin/main` `18f0381e`.

### The wait loops

Six bindings run each call on a worker thread and wait on the calling thread in ticks. Each reads its stop at every tick. Only two of them take a stop the caller can read back afterwards.

- Python, `libraries/python/src/worker.rs:123-155`, `wait`. Lines 134-142 return the worker's answer as soon as `recv_timeout` yields it. Lines 143-146 read the caller's token only after a timeout. So a token fired just before the answer lands is never read, and the call returns the answer. `Token.cancelled` then reads `True`.
- Ruby, `libraries/ruby/src/ffi.rs:154-189`, `cross`. Line 175 returns `Taken::Ready` before lines 184-187 read the tokens. The same race returns the answer while `cancel.cancelled?` reads true.
- R (`libraries/r/thinkthen/src/rust/src/calls.rs:112-123`), DuckDB (`databases/duckdb/src/worker.rs:42-63`), SQLite (`databases/sqlite/src/worker.rs:67-85`) and PostgreSQL (`databases/postgresql/src/call.rs:250-266`) read a host interrupt, not a caller token. R reads `pending()`. DuckDB reads its signal count. SQLite reads `sqlite3_is_interrupted`. PostgreSQL runs `poll()`. None of them takes a token from its caller.

The worker passes the engine its own token, never the caller's. Python hands the caller's token to the engine only as an interrupt check (`worker.rs:112-113`), and ticket 0166's last token read covers `CallOptions::cancel` alone. So 0166 does not close this race.

### A stopped call's sent request

- `crates/thinkthen/src/engine/http.rs:125` takes one throttle permit for each attempt, and line 134 drops it when the send returns. A sent request is never interrupted, by Ian's ruling in ADR 0017's amendment of 2026-09-22.
- `engine/mod.rs:339-359`, `Width::acquire`, reads `stop_or_remaining` at every poll. A cancelled call waiting at the gate leaves it and takes no permit. So stopped calls never hold more places than the throttle, and repeated cancels cannot grow the work in flight past it.
- `engine/request.rs:100` counts each attempt in the process usage before it is sent. So `usage()` already shows the cost of a detached request.
- `databases/sqlite/README.md:69` states the hold: a detached worker "holds its throttle permits" until its sent requests end, "at most the 30-second request timeout". No other surface says a later call may wait for it. The Python, Ruby, R and TypeScript pages say only that a sent request finishes. The throttle row of `specification/settings.md:58` says nothing about it.

## Which findings hold

| Finding | Holds on main? | This ticket |
| --- | --- | --- |
| Wait-loop race, Python | Yes, by reading | Fixed |
| Wait-loop race, Ruby | Yes, by reading | Fixed |
| Wait-loop race, R, DuckDB, SQLite, PostgreSQL | The loops read the stop late, but no caller can see it | Dropped. See below |
| Experiment 284 file 76, the held place | Yes, and SQLite alone documents it | One sentence on the settings page |
| File 76, a count of detached work | Not needed | Dropped. See below |

**Why the four host-interrupt loops need no change.** A host interrupt leaves no state the caller reads later. An answer that lands in the same tick as the interrupt cannot be told from an answer that landed just before it. Both return the answer, and the host acts on its interrupt at its next check. A token is different. The caller reads it back, and a fired token beside a returned answer breaks the promise that a fired token cancels the call. So the fix goes only where a token exists. Moving the four reads anyway would change no outward behavior, and no test could tell.

**Why no detached-work counter.** File 76 asks the counter to make the in-flight cost visible and to bound it. Both already hold. `usage()` counts each attempt before it is sent, and the throttle gate bounds the work in flight. A new counter would widen `Counters` on every surface to report a number no caller has asked for. The page sentence covers what the user cannot see today: the wait.

## Retained behavior

- A token fired before the call starts still refuses it, with nothing sent.
- A token fired during a held send still raises cancellation within one tick, and the worker is left to finish.
- An interrupt never fires the caller's token.
- Ctrl-C, `Thread#raise`, `Thread#kill`, a raising `with_tick` block and a Python signal handler's own error behave as today.
- A call with no fired token returns its answer or its error as today.
- R, DuckDB, SQLite, PostgreSQL, TypeScript, the C door and the command do not change.
- A sent request still finishes, and its reply still reaches the cache and the counters.

## The change

### Each token loop reads the token after each wait

The rule for both loops: after every wait, read the caller's stop before using what the wait returned.

- Python `wait`: move the token read at lines 143-146 above the `match waited` at line 134. The read fires the worker's own token and returns `Cancelled` with "the call was cancelled", as today. `py.check_signals()` stays after the match.
- Ruby `cross`: move the token read at lines 184-187 above `match handoff.take()` at line 174. It reads `own` and `caller` and returns `Fault::cancelled()`, as today. `rb_thread_check_ints` stays where it is.

Each move keeps its lines. The loops gain no helper and no branch. A shared helper would have to live in the public crate, the one crate both bindings reach. It would widen the public surface to save two moved blocks, and each loop's wait differs anyway: Python releases the interpreter and Ruby releases the VM lock.

A token that fires after that read races the call's return. The call returns what it had. Ticket 0166 states the same limit for the C door.

### The settings page states the held place

The throttle row of `specification/settings.md` gains this sentence after "The most requests in flight at once in one process.":

> A request already sent when its call stops keeps its place until it ends, within its attempt timeout (the Timeout row), so a later call can wait that long for a place.

That row serves every surface, so one sentence reaches all of them. `databases/sqlite/README.md:69` keeps its own sentence. `CHANGELOG.md` gains one line for the Python and Ruby fix.

## Decisions

Each is the ticket author's call. Ian can overturn any of them.

1. **A fired token wins over an answer and over an error.** This matches ticket 0166 decision 1 and each binding's README.
2. **Only the token loops change.** The four host-interrupt loops break no promise a caller can check.
3. **No detached-work counter.** The cost is counted and bounded today. The page states the wait.
4. **No shared helper.** Two moved blocks do not earn a public function.

## Edge cases

Each row runs on the conformance backend's held arm, one call, cache off, as `test_stopping.py` and `test_interrupt_single.rb` do today.

| # | Input | Expected |
| --- | --- | --- |
| 1 | Python `engine.decide` with a token. The parent waits for 1 request and tells the child. The child fires the token and prints `stopped`. The parent reads it, then releases the held reply | `Cancelled`, with "the call was cancelled". `token.cancelled` is `True`. The backend counts 1 request |
| 2 | Ruby `T.decide` with `cancel: token`, driven the same way | `ThinkThen::CancelledError`. `token.cancelled?` is true. Count 1 |
| 3 | A token fired while the reply stays held | `Cancelled` within 100 ms, as `test_a_token_stops_a_held_single_send` and `test_the_callers_token_cancels_a_held_decide_at_once` pin today |
| 4 | A token fired before the call | Refused with nothing sent, as today's tests pin |
| 5 | Ctrl-C during a held send | As today's tests pin |

## Tests and proof

| Test | What it proves | Deliberate break that turns it red |
| --- | --- | --- |
| `a_fired_token_beats_an_answer_already_waiting`, new unit test beside `a_closed_channel_with_no_result_is_a_defect` in `libraries/python/src/worker.rs`, about 12 lines | The loop's order, with no timing. It sends `Some(Ok(3))` on a `sync_channel` before the wait, fires the caller's `CancelToken`, calls `wait(py, receiver, &internal, Some(&token))`, and asserts the raised message is exactly "the call was cancelled" | (a) Move the Python token read back below the match: `wait` returns 3 on every run |
| `test_a_token_fired_as_the_reply_lands_cancels_the_call`, new in `libraries/python/tests/test_stopping.py` | Row 1, end to end | (a) The child prints the answer in most runs |
| `test_a_token_fired_as_the_reply_lands_cancels_the_call`, new in `libraries/ruby/tests/test_interrupt_single.rb` | Row 2 | (b) Move the Ruby token read back below `take`. The child prints the answer |

The Python unit test turns break (a) red on every run, because the answer already waits in the channel when the loop starts. The two held-arm tests pass on the fixed code whatever the timing. The token fires before the release, so any wake of the loop reads a fired token. A break turns a held-arm test red only when the answer wakes the loop before its next tick. The release comes a few milliseconds after the fire, and a tick is 50 ms, so that is most runs. Ruby has no unit seam, so its held-arm test carries its proof. The build runs break (b) 10 times and records how many runs turned red. It runs each new held-arm test 20 times on the fixed code.

Overlap was checked. Row 3's tests fire the token while the reply stays held, so the loop always reads the token on a timeout. No test releases the reply right after the fire.

The four questions:

- **What behavior does it protect?** A call whose caller's token fired before the call returned raises cancellation, never an answer.
- **What credible regression fails it?** A loop edit that takes the answer before it reads the token.
- **Why does no existing test catch it?** Every token test holds the reply past the next tick.
- **Does it need a test-only hook?** No. The held arm, the child's standard input and `release` drive it.

The gate ladder `sdlc/scripts/{install,lint,test,spec,surfaces}` runs before handing back. `surfaces` runs the Python and Ruby suites.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`.

- `libraries/python/src/worker.rs`: at most 15 net, the unit test included. `libraries/python/ratchet.json` moves by the same amount.
- `libraries/ruby/src/ffi.rs`: at most 2 net. `libraries/ruby/ratchet.json` moves by the same amount.
- `libraries/python/tests/test_stopping.py`: at most 25 net. `libraries/python/ratchet.py.json` moves by the same amount.
- `libraries/ruby/tests/test_interrupt_single.rb`: at most 22 net. `libraries/ruby/ratchet.rb.json` moves by the same amount.
- `sdlc/ratchet.json`: no change. No file under `crates` or `conformance` changes.
- Pages: one sentence in `settings.md` and one `CHANGELOG.md` line.
- No dependency. No paid call.

Each ratchet moves to the measured total in the commit that needs it, and that commit says what grew.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the fix needs a file outside the two wait loops, their two test files and the pages named here.
3. Stop if an existing Python or Ruby test changes its expected result.
4. Stop if break (a) leaves the Python unit test green once, or if break (b) leaves the Ruby held-arm test green in 3 or more of 10 runs. Then the test does not reach the race, and the design needs another look.
5. Stop if either new test fails once in 20 runs on the fixed code.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

1. Ticket 0166 lands first. It carries the issue this ticket closes.
2. This ticket builds on main after that. It does not wait for 0146 or 0162.
3. Ticket 0148 opens `libraries/python`, `libraries/ruby` and `settings.md`. Ticket 0163 opens the two READMEs and `settings.md`. Tickets 0146, 0154 and 0155 open `settings.md`. None of them edits the two wait loops or the throttle row's first sentence. The second ticket to land merges these lines.
4. Ticket 0169 shares no file with this one.

## Scope and exclusions

Excluded: the four host-interrupt loops, a detached-work counter, a cancellable socket read, the C door (ticket 0166), the command (ticket 0169), TypeScript, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and the code.

## Complexity

Contract 2; State/timing 3; Reach 2; Proof 2; Cost of error 1; Total 10. Minimum floor: level 3 for cancellation. Final level: 3. The risk is a test that cannot reach the race, which stop rule 4 guards.

## Deferred gaps

- A token fired after the loop's last read races the return. The call returns what it had.
- The four host-interrupt loops keep their order. A later host that exposes a durable stop would need the same move.
- A sent request still holds its place for up to the request timeout. A cancellable socket read is the full fix, and nothing schedules it.

## What Ian can overturn

- Decision 2: only the token loops change.
- Decision 3: no detached-work counter.

## Closes

`sdlc/issues/2026-09-27-a-binding-wait-loop-lets-an-answer-beat-a-stop-in-the-same-tick.md`, once 0166 lands it. The build moves it to `closed/` and records why the four host-interrupt loops need no change. It also carries local experiment 284 file 76, which `sdlc/issues/2026-09-26-architect-review-04-libraries-and-databases.md` and item I12 of `sdlc/issues/2026-09-26-architect-review-severity-3-findings.md` hold.

## Evidence

- Starts from: The wait-loop issue that ticket 0166 files, local experiment 284 file 76, and a reading of `origin/main` `18f0381e`: `libraries/python/src/worker.rs:112-155`, `libraries/ruby/src/ffi.rs:154-189`, the R, DuckDB, SQLite and PostgreSQL wait loops, `engine/http.rs:125-134`, `engine/mod.rs:339-359`, `engine/request.rs:100`, `databases/sqlite/README.md:69` and `specification/settings.md:58`.
- Keeps: A pre-fired token sends nothing. A token fired during a held send cancels within one tick. An interrupt never fires the caller's token. Sent requests finish and reach the cache and the counters. The four host-interrupt loops, TypeScript, the C door and the command behave as today.
- Changes: The Python and Ruby loops read the caller's token after every wait and before the answer, so a fired token always wins. The settings page states that a stopped call's sent request keeps its throttle place until it ends.
- Proof: A Python unit test that finds an answer already waiting and a fired token, red on every run of its break. One held-arm test each in Python and Ruby that fires the token and then releases the reply, each run 20 times green. The Ruby break runs 10 times and turns red in at least 8. The `install`, `lint`, `test`, `spec` and `surfaces` rungs pass.
- Defers: A fire after the last read, the four host-interrupt loops, a detached-work counter, and a cancellable socket read.
