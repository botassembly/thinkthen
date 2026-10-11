---
flow: build
priority: 143
opens: crates/thinkthen/src/engine/facade.rs crates/thinkthen/src/engine/facade/relate.rs crates/thinkthen/src/engine/workers.rs crates/thinkthen/src/cli/relate.rs crates/thinkthen/src/cli/relate/config.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/tests/backend/relate.rs crates/thinkthen/tests/backend/relate crates/thinkthen/tests/backend/interrupt.rs crates/thinkthen/tests/backend/refusals.rs crates/thinkthen/tests/backend/refusals/relate.rs crates/thinkthen/tests/relate_edge.rs crates/thinkthen/tests/public_controls.rs crates/thinkthen/tests/backend/annotate/splitting.rs databases/duckdb/tools/relate_suite.py databases/duckdb/ratchet.py.json databases/sqlite/tests/test_interrupt.py databases/sqlite/ratchet.py.json specification/relate.md specification/records.md specification/settings.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0143: relate and split requests run at once

Status: COMPLETE.

Opened as: 2026-10-11. 2026-09-26 (`sdlc/records/0143-build-relate-runs-at-once.md`). Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `thinkthen relate` over a set with several rules, or with a profile that splits a relation, and its requests go out together under the throttle. Today they go one after another, so the run takes the sum of every request's round trip.

This is ticket J1 of `sdlc/issues/closed/2026-09-26-batching-design.md`. Its row reads: "One text's split requests and one relate's relations run under the engine's throttle. `relate` takes `--jobs`. Closes `2026-09-25-relate-sends-one-chunk-at-a-time.md`." Its proof is "Ticket 0118's loopback count reaches the throttle". Its ADR column says `relate.md` needs an amendment, because that page says the command sends its requests in order and refuses `--jobs`. Ian's ruling 9 of 2026-09-26 orders J1 after B0, C1, B1, and B2, and before B3. The row lists no dependency. Ian can overturn the ruling and the row.

- One relate's relations, and each relation's split requests, run at once under the engine's throttle.
- Every other caller of `Engine::ask_chunks` gains the same: one `recognize` text's split requests, and one `annotate` group's split requests.
- `relate` takes `--jobs N`, 1 to 32, default 4, as the other commands do.
- Output, standard error, exit codes, digests, and recordings of a finished run stay byte for byte what the in-order run gives.

## What happens today

- `Engine::relate` loops over the prepared relations one at a time (`crates/thinkthen/src/engine/facade/relate.rs:116`). For each relation it calls `ask_chunks`.
- `Engine::ask_chunks` sends each chunk and waits for its reply before it sends the next (`crates/thinkthen/src/engine/facade.rs:257-276`). It hands each reply to the caller's closure in chunk order.
- `ask_chunks` has three product callers: relate, `recognize`'s `execute` (`facade/recognize.rs:187`), and `annotate`'s `answer_group` (`facade/annotate.rs:74`). `check` calls it with one chunk. Two unit tests call it.
- The throttle lives in the HTTP client. Every live attempt takes a permit from the process's `Widths` before it posts (`engine/http.rs:124`), so no caller can pass the width. The engine also holds its width in `State::width`, which the record and group schedulers read.
- `workers::scoped` (`engine/workers.rs`) already runs a bounded group of engine workers around one body on the calling thread. The record scheduler and the annotate group scheduler use it.
- `relate` refuses `--jobs` at exit 2 with "`relate` sends its requests in order, so it takes no --jobs" (`cli/relate/config.rs:27`). Ticket 0123 hid `--jobs` from `relate --help` with `mut_arg` on the `Relate` variant (`cli/args/command.rs:215`). `cli/relate.rs` passes `None` as the width to `asking::engine`.
- A relation with no profile is one request. The hosted address splits a relation above 96,000 bytes (ticket 0123). `--profile` limits such as `max_questions` split it too.

## Design

`ask_chunks` keeps its signature and its promise to the caller: the closure sees each reply in chunk order, and the first failure in chunk order is the one returned. Only the sending changes.

- **One chunk.** It is sent as today, on the calling path, with no new worker. `check`, and every relation that fits one request, stay exactly as they are.
- **Many chunks.** It runs `workers::scoped` with `min(width, chunks)` workers, where `width` is the engine's `State::width`. The body on the calling thread feeds one chunk to each free worker, in chunk order. It never queues more chunks than there are free workers. It holds each reply by its chunk number and hands the replies to the closure in chunk order, as soon as every earlier chunk has answered.
- **Failure.** When a send fails, or the closure returns an error, the body feeds no further chunk. It waits for the chunks in flight, hands on in order every reply before the first failure, and returns the first failure in chunk order. Every chunk before a failed chunk was fed earlier, so this is the error the in-order run returns. Chunks after it that were already in flight finish, and their replies are recorded and counted as any sent request is.
- **Stop.** Between feeds, and while it waits for replies, the body calls `cancel.stop()`, as the record scheduler (`engine/schedule.rs:263`) and the group scheduler (`engine/annotate_schedule.rs:170`) already do. It does not use `poll_between_sends` (`engine/mod.rs:156`), which `workers::on_worker` uses. That call skips the host check while any send is counted, so with chunks in flight a host interrupt, such as DuckDB's, would go unseen and new chunks would be fed until the call ends. A stop feeds no further chunk. The chunks in flight finish, and the call returns the stop.
- **Nesting.** `annotate`'s `answer_group` and `recognize`'s `execute` in record mode already run on engine workers. There `ask_chunks` starts its own inner workers, as decision 8 rules.

`Engine::relate` gathers every relation's chunks into one list, in relation order and then chunk order, beside the index of the relation each chunk belongs to. It makes one `ask_chunks` call over the list. The closure finds the chunk's relation by that index and takes the next mappings from that relation's iterator, as it does now. Each relation's iterator must be empty at the end, as now. That check now runs after every relation has answered, not before the next relation starts. It raises the same Defect. So `edges`, `logical`, and `requests` fill in the same order as today.

The command side:

- `cli/relate/config.rs` drops the `--jobs` refusal.
- `cli/relate.rs` passes `arguments.common.jobs` to `asking::engine`, as a set command, where it registers the width.
- `cli/args/command.rs` drops the `mut_arg` that hides `--jobs` from `relate --help`. This is one line, three lines above the audit help that ticket 0135 changes.
- The `--jobs` help in `cli/args.rs` names `relate`: "It acts in record mode, on `annotate`, where a single text can make several grouped requests, and on `relate`, where each relation makes its own requests."

## Decisions

Each is the agent's decision under the J1 row. Ian can overturn any of them.

1. **The change lives in `ask_chunks`, not in relate alone.** The J1 row asks for "one text's split requests" as well as relate's relations. `ask_chunks` is where every split request is sent, so one change reaches relate's split relations, `recognize`'s split text, and `annotate`'s split groups. The issue's options 1 and 2 both follow from it.
2. **Relate makes one `ask_chunks` call over every relation's chunks.** It needs no second scheduler. A relate with eight one-request rules then holds eight requests in flight, which is the 0118 count.
3. **Replies are handed on in chunk order, not arrival order.** Every caller builds its result in that order. So the output bytes, the `--details` request list, and the model check stay what the in-order run gives.
4. **The first failure in chunk order wins, and no chunk starts after a failure is seen.** The message and exit code match the `--jobs 1` run. The cost is up to `width - 1` requests in flight that finish after the failure. They may be billed and recorded, as a retried request may be today. The design states it on `relate.md` and `records.md`.
5. **The width is the engine's.** `relate --jobs N` selects it, as on other commands. A library relate follows its engine's throttle. `recognize` over one text still refuses `--jobs` in this ticket, and its split requests follow the process width, 4 unless another call selected one. Ian ruled that `recognize` accepts `--jobs` for one document (`sdlc/issues/2026-09-26-recognize-design.md:231`). Ticket R4b of that design owns the flag and depends on this ticket.
6. **`--jobs` enters no plan and no digest.** A `--dry-run` plan and every request body are unchanged.
7. **The in-flight chunk budget is the worker count, not a queue.** Feeding one chunk per free worker keeps a failed run's extra requests to those already in flight. A queue filled up front would send chunks after a failure.
8. **Nested workers are allowed.** The coordinator ruled this on 2026-09-26, the design reviewer's option b, because speed wins. One long text inside a small record run would otherwise go one chunk at a time. No new pool is built.
    - **Thread bound.** The record and group schedulers run at most `width` outer workers. Each outer worker's `ask_chunks` starts at most `width` inner workers. So a run holds at most `width × width` inner workers beside its `width` outer ones: 16 at the default 4, and 1,024 at `--jobs 32`. The bound is reached only when every record in flight splits into at least `width` chunks. An inner worker waits on the permit's condition variable, and spends no processor time while it waits.
    - **Requests stay within `--jobs`.** Every live attempt takes its own permit from the process `Widths` in `engine/http.rs` before it posts, and gives it back when the attempt ends. An outer worker waiting on its inner workers holds no permit, because it posts nothing itself. So at most `--jobs` requests are on the wire, however deep the nesting.
    - **Cancellation.** `Cancel::checked` runs the host check only on its calling thread, so an outer or inner worker never runs it. Workers learn of a stop through the shared stop flag, the `fired` `Arc<AtomicBool>` that every clone of a call's `Cancel` shares. This is the mechanism the record workers already use. The outermost calling thread runs the host check in its scheduler loop through `cancel.stop()`, and a true check fires the flag. The command's SIGINT handler sets the same flag, registered through `signal_hook::flag::register` on `Cancel::flag` (`cli/interrupt.rs:327`), and a library cancel sets it through `public/options.rs:218`. An inner `ask_chunks` body reads the flag through `cancel.stop()` between feeds, feeds no further chunk, and waits for its chunks in flight. An inner worker blocked on a permit sees it through `Widths::acquire`, which checks `stop_or_remaining` every 50 ms and returns `Cancelled`. So no chunk starts after the flag fires, at any depth.

## Edge cases

| Case | Today | After |
| --- | --- | --- |
| One relation that fits one request | One request | One request, sent on the calling path with no new worker. Kept |
| Eight one-request rules, `--jobs 3` | One request at a time | Up to 3 in flight, 8 in all. Output bytes equal the `--jobs 1` run's. Changed |
| One relation split into 6 chunks by `--profile` `max_questions`, `--jobs 4` | One at a time | Up to 4 in flight. Output and `--details` equal the `--jobs 1` run's. Changed |
| Replies that arrive in reverse order | n/a, one at a time | Edges, `logical`, and the request list keep relation and chunk order. Changed timing, same bytes |
| A chunk that fails at the backend, `--max-retries 0` | The run stops at it and sends no later chunk | Same standard error and exit code as `--jobs 1`. No chunk starts after the failure is seen. Chunks already in flight finish. Changed request count |
| Two failed chunks, the later one failing first | The earlier one's message | The earlier one's message. Kept |
| A reply that fails some questions and answers others | Partial edges, exit 6 | Partial edges, exit 6, same bytes. Kept |
| A later chunk reporting another model | `ModelsDiffer` | `ModelsDiffer`, found in chunk order. Kept |
| `--jobs 1` | Refused, exit 2 | One request at a time, in chunk order, same requests in the same order as today. Changed |
| `--jobs 0` or `--jobs 33` | Refused, exit 2, relate's own sentence | Refused, exit 2, by the shared range parser as on every command. Changed |
| `--jobs 8` over empty input | Refused, exit 2 | Exit 0, no output, no request. Changed |
| `--jobs 8 --dry-run` | Refused, exit 2 | The plan, byte for byte the plan without `--jobs`. Changed |
| `--replay` over a full recording | Every chunk from disk | Every chunk from disk, same bytes. Kept |
| `--replay` missing one chunk | Exit 5 at that chunk | Exit 5 with the same message. Chunks in flight answer from disk. Kept |
| SIGINT while chunks are in flight | The one in flight finishes, none starts after | Those in flight finish, none starts after. The command dies by SIGINT with no output. Kept |
| `recognize` over one text split by a profile | One at a time | Up to the process width, 4. `--jobs` stays refused. Changed |
| `annotate` over one text, a group split by a profile | One at a time within the group | Each group's chunks start inner workers under the group scheduler's outer worker, per decision 8. Up to `--jobs` requests in flight across all groups, since every attempt takes a permit. Changed |
| `recognize` or `annotate` over records, where a record splits | Each record's chunks one at a time on its worker | Nested inner workers, per decision 8: at most `width × width` inner workers, 1,024 at `--jobs 32`, and at most `--jobs` requests in flight. SIGINT fires the shared stop flag, and no inner chunk starts after it. Changed |
| `check` | Four one-chunk calls | Kept |

## Proof

Every new Rust test runs the compiled command against a counted loopback listener from `tests/backend/harness`. They live in a new file, `crates/thinkthen/tests/backend/relate/at_once.rs`, a child of `relate.rs`, which holds 457 nonblank lines and gains only the `mod` line. It reuses `relate::run`, `relate::answered`, `Gathering`, `Listener::peak`, and `Canned::after`.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `relate_requests_reach_the_throttle_and_no_further` | `relate --lines` over three names with nine bare rules `r1` to `r9`, `--jobs 3`. Each reply waits in a `Gathering` of 3. Exit 0. The listener read 9 requests, and `listener.peak()` is exactly 3. The width is 3, not the default 4, so the test also proves `--jobs` reaches the engine | (a) Loop over the relations with one `ask_chunks` call each: the peak is 1. (b) `cli/relate.rs` still passes `None` as the width: the engine runs at the default 4, and the peak is 4 |
| `split_relations_print_what_one_job_prints` | Two rules over the fixture of `relate/ceiling.rs` under a `max_questions` profile that splits each into 3 chunks. Replies wait longer for earlier requests, so they arrive in reverse order. The run at `--jobs 4` and the run at `--jobs 1` print the same standard output, bare and under `--details`, and the same empty standard error. The `--jobs 4` peak is above 1 | (c) Hand each reply on in arrival order: the edges and the request list reorder. (d) Keep the old loop: the peak is 1 |
| `a_failed_chunk_stops_the_run_as_one_job_does` | Six chunks at `--jobs 2`, `--max-retries 0`. Chunk 1 answers 500 at once. Chunk 2 answers after 300 ms. The run exits with the `--jobs 1` run's code and standard error, and the listener read exactly 2 requests. A second row fails chunk 2 at once with 503 and chunk 1 after 100 ms with 500. It prints the 500 message, as `--jobs 1` does | (e) Queue every chunk up front: 3 or more requests. (f) Return the first failure to arrive: the 503 message |
| `relate_takes_jobs` in `tests/relate_edge.rs` | The existing help test flips: `relate --help` holds the whole `--jobs` sentence of the design, in place of its `!help.contains("--jobs")` assertion | (g) Restore the `mut_arg` that hides `--jobs`: the sentence is absent |
| `the_throttle_reaches_a_relate` in `databases/duckdb/tools/relate_suite.py` | Ticket 0118's first acceptance line, restored. A child runs `SET thinkthen_throttle = 8` and a relate over 16 rows with nine bare rules `['r1', …, 'r9']` on the held arm. `backend.wait(8)` returns 8, and the count stays 8 while held. After release the query answers and the count reads 9. The child pattern is `signal_suite.py`'s | (a) again: `wait(8)` returns 1 |

At code review the failed chunk test gained a third row, where `bravo` answers first from another model. At `--jobs 2` it sends 3 requests to the one-job run's 2, with the same exit code, standard error, and standard output. The coordinator ruled on 2026-09-26 to keep the count at 3: it proves that feeding continues while `alpha` is in flight, and that nothing is fed after the model error is seen in chunk order. Ian can overturn this ruling.

One public Rust API test covers a host interrupt, in `crates/thinkthen/tests/public_controls.rs`, beside `a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new`, whose pattern it copies. The real boundary reaches it: `CallOptions::interrupt` is the public host check, and the loopback backend's held arm holds requests. No new hook is needed.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `a_host_interrupt_during_relate_chunks_sends_nothing_new` | An engine at the default throttle of 4 on the held arm. `relate_with` over two entities with six bare rules, so six one-request relations. The check returns true once the backend count reaches 4, while those 4 are held. A helper thread waits for 4, sleeps 400 ms, and releases. The call returns `ErrorKind::Cancelled`. Every check ran on the calling thread. The backend count stays 4 | (j) The body polls through `poll_between_sends` in place of `cancel.stop()`: the check is skipped while sends are counted, the freed workers are fed chunks 5 and 6, and the count reads 6 |

The first test also proves `--jobs` is accepted: plant (h), restoring the `config.rs` refusal, exits 2 and turns it red.

Existing tests that change:

- `sigint_between_recognition_chunks_starts_no_later_chunk` in `tests/backend/interrupt.rs` pins one request in flight at the signal. After this ticket a split `recognize` text holds up to 4. The test keeps its claim: it uses a text whose `max_questions` 1 profile gives more than 4 chunks. `held` gains the number of requests to wait for, 1 for every other row. The test waits for 4, signals, releases, and asserts exactly 4 requests. Plant (i): send the chunks one at a time, and the 4 requests never arrive. Amended at build by the coordinator's ruling of 2026-09-26, which Ian can overturn. The first plant (i) fed chunks after a stop was seen, and it stayed green. For SIGINT, the feed loop's stop check is a second guard behind the send path's own check of the shared flag, so no plant on that check alone can turn this test red. The check exists for host interrupts, and plant (j) proves that path.
- The shared refusal row `jobs on one document` in `tests/backend/refusals.rs` stops applying to `relate`, and relate's own sentence for it in `refusals/relate.rs` goes. The shared row still covers every other verb.
- Any test that reads `listener.requests()` in arrival order for a split `relate`, `recognize`, or `annotate` run compares them sorted, or passes `--jobs 1` where the verb takes it. The builder lists each one in the record.

The four questions for each new test:

- **`relate_requests_reach_the_throttle_and_no_further`.** It protects the J1 outcome: a relate's requests go together, up to `--jobs` and never past it. A relate loop that sends one relation at a time fails it, as does a command that never hands `--jobs` to the engine. No test counts relate's requests in flight. It needs no hook.
- **`split_relations_print_what_one_job_prints`.** It protects decision 3, the order of output and details. Handing replies on as they arrive fails it. No test runs a split relate with replies out of order. It needs no hook.
- **`a_failed_chunk_stops_the_run_as_one_job_does`.** It protects decisions 4 and 7: the in-order message and no send after a seen failure. A queue filled up front, or the first failure to arrive, fails it. No test fails a chunk of a split relate. It needs no hook.
- **`a_host_interrupt_during_relate_chunks_sends_nothing_new`.** It protects the Stop rule of the design for a host interrupt, the path a database surface or a library caller uses. Polling that skips the check while sends are in flight fails it. The CLI tests fire the shared flag through SIGINT, which never runs a host check, and the existing control tests reach only the record scheduler. It needs no hook: `CallOptions::interrupt` and the held arm are real boundaries.
- **`the_throttle_reaches_a_relate`.** It protects the proof the J1 row names, on the surface whose count first showed the defect. The old relate loop fails it. The Rust tests reach the command, not the DuckDB binding's engine map. It needs no hook: the held arm is the loopback backend's.

No unit test is added. `ask_chunks` is reached only through real callers, and the command reaches every behavior above.

## Specification pages

- `specification/relate.md`: "The command sends its requests in order and refuses `--jobs` at exit 2" becomes: "`--jobs N` bounds the requests in flight, 1 to 32, default 4, as [records.md](records.md) gives it. Relations and split requests go out together. Output keeps relation, expansion, question, and candidate order whatever `--jobs` is. A failed request stops the run as `--jobs 1` would, and requests already in flight finish."
- `specification/records.md`, `jobs`: "`annotate` also accepts `--jobs` for one document because distinct evidence groups make distinct requests" gains `relate`, whose relations and split requests make distinct requests. One sentence says a single text's split requests run under the throttle.
- `specification/settings.md`: if ticket C1 has landed it, the throttle row names `relate` in the same commit. If it has not, nothing is written there.
- `site/` is out of scope. A `relate` page there that says `--jobs` is refused gets an issue for the website agent.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src`: at most 120 added, doc and help lines included, and at most 90 net of lines removed. The coordinator re-scored this at build on 2026-09-26 from 70 added and 50 net, because `ordered` is new engine logic that rustfmt lays out vertically. Ian can overturn it. The code review still looks for cuts. `facade.rs` stays under 500 nonblank lines. The ordered feed may live in `engine/workers.rs` if that keeps `facade.rs` smaller.
- Rust tests: at most 200 added in `relate/at_once.rs`, at most 35 added in `tests/public_controls.rs`, and at most 40 changed across `interrupt.rs`, `refusals.rs`, `refusals/relate.rs`, `relate_edge.rs`, and order fixes in other test files. The coordinator ruled at code review on 2026-09-26 to accept 42 added lines in `tests/public_controls.rs`, because they are the second interrupt row the review asked for. Ian can overturn this ruling.
- `databases/duckdb/tools/relate_suite.py`: at most 30 added.
- Pages: at most 15 lines changed across `relate.md`, `records.md`, and `settings.md`.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. The commit says what grew. The builder first looks for duplication to delete in `facade.rs`, `facade/relate.rs`, and `workers.rs`.
- No dependency. No public library type, method, or message changes. The engine change reaches every surface's relate, so the `surfaces` rung runs.

## Stop rules

1. Stop before crossing a budget by more than a tenth, or adding a dependency.
2. Stop if any plant stays green.
3. Stop if more than five existing tests fail on arrival order. That means the change is wider than this design says.
4. Stop if any `spec/` page, green demo, or surface suite turns red other than those named above.
5. Stop before editing a file that ticket 0135 owns, other than the one `mut_arg` line in `cli/args/command.rs`. If 0135 has not landed when the build starts, the coordinator orders the two. Ticket 0138 also opens `cli/args/command.rs`, `tests/backend/recognize.rs`, and `tests/backend/annotate.rs`. If an order fix needs one of those two test files while 0138 is in flight, stop and ask the coordinator.
6. Stop if a stop or failure leaves a worker running after `ask_chunks` returns.
7. Stop if an interrupt or a failure in a record run leaves an inner worker sending after the flag fires. Stop if a nested run holds more than `--jobs` requests in flight.

## Scope and exclusions

Excluded: batching records into one request, which B3 and B4 own. The pool and usage fixes of B1 and B2. `recognize` taking `--jobs` for one text, which R4b owns. `annotate` running one text's groups at once from the library, where `Engine::annotate` still loops over groups. The audit grading of `recognize` and `relate`, which ticket 0135 owns. `site/`, which the website agent owns.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling and widens `relate`'s command surface, so the code review names what it checked.

## Complexity

Contract 1; state and timing 2; reach 2; proof 1; cost of error 1; total 7. Final level: 2. The risk is order and failure under concurrency, which the second and third tests guard.

## Deferred gaps

- `recognize` over one text refuses `--jobs`, and the refusal says a single text sends one request. A split text sends several. Ian's ruling at `sdlc/issues/2026-09-26-recognize-design.md:231` gives it the flag, and ticket R4b owns that work.
- The library `Engine::annotate` answers one text's groups one after another. The command's group scheduler already runs them at once.
- A failed run may send and bill up to `width - 1` requests after the one that failed. Cancelling requests in flight would need a stop that reaches a socket read.
- Ticket 0118's amended acceptance line stays as landed. This ticket's record names the restored count.

## What Ian can overturn

- The J1 row and its place in ruling 9's order.
- Decision 8: nested workers, the coordinator's ruling, with up to `width × width` inner workers.
- Decision 1: the change reaches `recognize` and `annotate` split requests, not relate alone.
- Decision 4: a failed run may bill requests already in flight.
- The amendment of `relate.md` in place, which the J1 row asks for.

## Closes

- `sdlc/issues/closed/2026-09-25-relate-sends-one-chunk-at-a-time.md`. The lander moves it to `closed/` in the landing commit.

## Evidence

- Starts from: The issue above, which ticket 0118's held arm measured: a 16-row DuckDB relate under throttle 8 reached 1 counted request. The J1 row and ruling 9 of `sdlc/issues/closed/2026-09-26-batching-design.md`. Experiment 268's round trip of 135 to 151 ms a request, which a relate of N requests in order pays N times. Ticket 0123, which hid `--jobs` from relate help. The code paths in "What happens today" at `origin/main` `d410ef4a`.
- Keeps: One-chunk calls, `check`, and every relation that fits one request send exactly as today. Output, `--details`, standard error, exit codes, digests, plans, and recordings of a finished run. The first failure in chunk order. The model check. SIGINT's rule that no request starts after the signal. `recognize`'s `--jobs` refusal for one text.
- Changes: `ask_chunks` sends a call's chunks at once under the engine's width and hands replies on in chunk order. Relate sends every relation's chunks in one call. `relate` takes `--jobs` and shows it in help. `relate.md`, `records.md`, and the `--jobs` help say so. A failed run may finish requests already in flight.
- Proof: The five new tests under "Proof", with plants (a) to (h) and (j), and the rewritten recognize interrupt test with plant (i). The DuckDB case restores 0118's count of 8 in flight.
- Defers: `recognize --jobs` for one text, which R4b owns. The library `annotate` group loop. Cancelling requests in flight after a failure. The `site/` relate page.
