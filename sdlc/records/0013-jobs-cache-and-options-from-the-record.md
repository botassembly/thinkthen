# 0013: Jobs, cache, and options from the record

Branch `ticket/0013-jobs-cache-options`. Built 2026-09-19.

## What landed

Three options entered the surface. `--jobs N` takes a whole number from 1 to 32 and defaults to 4, and it bounds how many requests are in flight over a record stream. `--cache DIR` points `--record` and `--replay` at one folder, so a rerun pays for the records that never finished. `choose --options POINTER` takes each record's candidate list out of the record, as a list of names or as a map from name to description.

Four items arrived from the steering session and landed with them. One connection pool is built once and shared by every worker. A rate limit's `Retry-After-Ms` and `Retry-After` headers set the wait. A reader that closes the pipe stops the reading and the scheduling. An option or a level holding a control character is refused before any request.

How-to 12 and how-to 21 are green over seven live exchanges.

## The module the binary gained

`crates/thinkthen/src/schedule.rs` holds the scheduler and knows nothing of judging. It takes a function over one record's bytes and the writer that owns standard output. The workers share one queue and one channel back, built on `std::thread::scope` and `std::sync::mpsc`. No asynchronous runtime entered and no dependency entered.

Records go out in input order and no more than `jobs` are outstanding. A finished row waits in a `BTreeMap` until the rows before it are written, and the invariant `in_flight <= jobs` bounds that buffer at `jobs` rows. Dispatch halts the moment any `Err` arrives, which is safe because every earlier record was already dispatched. The run reports the earliest failed record whatever order the answers came back in.

`judge.rs` reached 658 non-blank lines with the scheduler inside it, and the ceiling is 500. The split is the fix, and it is also the right seam: the scheduler names no verb and the judging opens no thread.

## Red then green

- **`--options`.** `error[E0599]: no method named choices found for struct Record`, then the two missing `RecordError` variants.
- **The control character.** The check and its test were written together, so the check was replaced with `return Ok(Self(values));` to watch the test fail, and then restored.
- **`Retry-After`.** `attempt.asked` was dropped from the sleep to watch the test fail. It read 6.02ms and a failure, and it read 1.01s and a pass with the line back.
- **`Retry-After-Ms`.** `error[E0061]: this function takes 1 argument but 2 arguments were supplied`. The integration case was then run with the two headers read in the other order, and it failed after 30.01s, which is the seconds header the backend also sent.
- **The parallel path.** `error[E0597]: queue does not live long enough` inside `thread::scope`, until the channel and the `Mutex` were built before the scope.

Going green found three real things. Four existing cases pinned the order the listener saw requests in, and they pass with `--jobs 1` now, because `parallel.rs` pins the parallel form instead. A cache-resume test asked for every record again, because a recording digest names the URL and the test had opened a second listener on a new port. The `--options` integration cases exited 4, because the listener answered with probabilities for two of three options and the decoder is right to refuse that.

## Acceptance, bullet by bullet

| Bullet | The test that proves it |
| --- | --- |
| `--jobs` bounds the requests in flight | `parallel::no_more_requests_are_in_flight_than_the_jobs_asked_for` |
| Output never depends on `--jobs` | `parallel::every_number_of_jobs_prints_the_bytes_that_one_job_prints` over six numbers, three runs, and both channels |
| Answers out of order print in input order | `parallel::a_backend_that_answers_out_of_order_still_prints_in_input_order` |
| `--jobs` acts over records alone and inside its range | `parallel::jobs_acts_in_record_mode_alone_and_inside_its_range` |
| `--cache` resumes and pays for the rest alone | `parallel::a_stop_keeps_what_finished_after_it_and_a_rerun_pays_for_the_rest_alone` |
| `--cache` stands beside neither `--record` nor `--replay` | `recordings::cache_is_the_two_options_on_one_folder_and_stands_beside_neither` |
| `--options` as a list and as a map | `from_record.rs`, six cases |
| A record's options refused before any request | `from_record.rs` asserts zero requests on the listener |
| One process reuses its connections | `parallel::one_process_reuses_the_connections_it_opens` |
| A closed pipe stops the reading and the scheduling | `parallel::a_reader_that_closes_the_pipe_stops_the_reading_and_the_scheduling`, which bounds 24 records at 12 requests |
| A control character in a label | `question::tests` on both label sources, `from_record.rs` on a record-supplied one, and the "What can go wrong" block of how-to 21 |
| The retry wait from either header | `http::tests::a_retry_after_header_is_read_in_seconds_and_stops_at_the_ceiling`, `http::tests::the_milliseconds_header_is_read_first_and_the_seconds_header_follows_it`, `exchange::a_rate_limit_waits_the_seconds_the_backend_asked_for`, `exchange::a_rate_limit_in_milliseconds_is_read_before_the_one_in_seconds` |
| No key and no evidence in an error or a `Debug` line | `http::tests::an_exchange_shows_neither_the_key_nor_the_evidence_it_carries`, and `Client` prints `Client(<pool>)` |
| How-tos 12 and 21 green | The `spec` rung: `demos: 15 green, 7 red` |
| The ratchet equals the measured total | `sdlc/scripts/lint` |

## The choices made where the pages were silent

Each of these is written into the page or the help that states it, and Ian can overturn any of them.

1. **`--jobs` takes 1 to 32.** The pages gave a default and no range. One is the sequential form and 32 is four times the concurrency the vendor's endpoint allows, so a number above it buys nothing and a typo costs a user a refused run rather than a rate limit. `records.md` says so.
2. **`--jobs` outside record mode is exit 2.** One document sends one request, so the option would act on nothing. Silently ignoring it would hide a mistaken command line.
3. **A request that finished after the failed record is still recorded.** It was billed. Dropping it would make a resume pay twice. `records.md` says so.
4. **The stop always names the earliest failed record.** Several requests are in flight when one fails, and the record a user repairs must not depend on the arrival order.
5. **A closed pipe leaves the exit code the run had earned.** `channels.md` gives no code for a reader that went away, and a reader that took what it wanted did not fail the run.
6. **A map under `--options` sends its descriptions and prints its names.** The description steers the model, and the value on standard output is the label a script branches on.
7. **The control-character message never quotes the label.** The label is the untrusted string. Quoting it in a diagnostic would put the bytes on a terminal anyway.
8. **The control-character rule covers `score` levels too.** One rule over one type is simpler than two, and a level prints under `--details` the same way.
9. **The retry wait caps at 60 seconds.** A backend asking for an hour would hang a script with no way out but a signal.
10. **The HTTP-date form of `Retry-After` is ignored.** Reading it needs a clock and a date reader, and the core takes neither.
11. **The demo runner names the pages it ran rather than counting them.** The count moves with every ticket that turns a page green, and a page that failed leaves a non-zero code already.

## The live calls

Two, both through `sdlc/scripts/live` by a `record.sh` beside each page. How-to 21 spent 1026 input tokens over three exchanges and how-to 12 spent 1148 over four. The ledger went from 249,292 to 251,466 of 476,000,000. Every committed recording was searched for `apikey_`, `authorization`, and `bearer` in any case, and all three counts are zero.

How-to 12 assumed in red that a run against a closed port names the address it could not reach. It does not. The message reads "the backend could not be reached" and the transport's own words follow, and the page asserts that phrase now.

How-to 12 shows its stop and its resume through one `--cache` folder over a committed recording, because the `spec` rung touches no network. Every row of the rerun reads `meta.replayed: true` there, and the page says in one sentence what a real rerun reads instead. The stop and the resume are proven against a failing listener in `parallel.rs`.

## The sentences added to the specification

`records.md` lost its Draft mark, and `specification/README.md` now reads Settled for it. Five sentences entered `records.md`: the range and the default of `--jobs`, the usage error outside record mode, that any number prints the bytes `--jobs 1` prints on both channels, that the buffer holds at most `jobs` rows, that one process opens one pool, and that a request finished after the failed record is still written.

`channels.md` names `--cache DIR` and `--jobs N` among the advanced options. `choose.md` and `score.md` each gained one sentence on control characters, and `choose.md` says that every rule on a label holds for a label a record supplies. `backends.md` gained the three sentences on the two retry headers.

## The review

A second agent read the branch against the ticket, the specification, and the standards. It named what it checked: the public surface against every page, the scheduler for deadlock, lost rows, double prints and bounded memory, every `Debug` and every message for a key or a record, `thinkthen-core` for a file, a variable, a socket, a clock or a process, the dependency sets and both lint tables, the 500-line ceiling, the commits for attribution, and both pages for the how-to form and for public hygiene.

Six findings were fixed.

1. **A recording race under `--jobs`.** The temporary file was named `.{pid}.{digest}`, one path per digest per process. Two records that are byte for byte alike make one digest, so every worker that missed it wrote that one path, and one worker's cleanup unlinked another's file. The run then ended at exit 5 with "No such file or directory" or "File exists". A counter gives each attempt its own name now, and the rename stays the atomic step. `parallel::records_that_are_byte_for_byte_alike_write_one_entry_and_race_with_nobody` failed on three runs of three before the fix and passes on three of three after it.
2. **A dead worker ended the run at exit 0.** A closed channel with records still in flight broke the loop and reported success, and a failed hand-off dropped a record the same way. Both are `Failure::Defect` now, at exit 70.
3. **A stray doc comment.** `DEFAULT_JOBS` moved to `schedule.rs` and its doc block stayed in `judge.rs`, documenting `struct Asked`.
4. **`spec/decide.md` pinned six advanced options and not the two new ones.** The loop names `--cache` and `--jobs` now, so the spec rung proves both are hidden from `-h` and present in `--help`.
5. **Both pages broke ADR 0016.** ADR 0016 landed on main while this branch was out, and it caps a page at 120 lines, 900 words, six blocks, and four steps, with nothing set up before the first result. How-to 12 is 110 lines, 764 words, six blocks, four steps. How-to 21 is 95 lines, 804 words, five blocks, three steps. Neither opens with `mktemp` or `trap` any more, because a cache folder that holds every entry writes nothing and the repair pipes through `jq`.
6. **The branch was three commits behind main.** `origin/main` is merged in, so ADR 0016, three tickets, and two planning pages stay.

Two more changes came from its notes. `Judged` now has a hand-written `Debug` that withholds the printed line, which holds the whole record under `--details`. `parallel::no_more_requests_are_in_flight_than_the_jobs_asked_for` asked for the bound exactly, which a loaded machine can miss, and it asks for the bound and for more than one request in flight now.

Three findings stand.

- **How-to 21 keeps its returns-desk scenario.** ADR 0016 suggests "picking the next action from a list that changes at every step" for demo 21. The ticket, `documentation-plan.md`, and `demos/README.md` all name "choose from a list that differs for every record", and the recorded exchanges are that scenario. Changing it means a new live recording for a page that already teaches the option.
- **The `--jobs` property is a table and not a `proptest` case.** `proptest` is a development dependency of the core alone. Adding it to the binary is a new dependency and needs a review of its own.
- **`exchange::a_rate_limit_waits_the_seconds_the_backend_asked_for` sleeps about a real second.** The header names whole seconds, so one second is the smallest honest case.

## The gates

`install`, `lint`, `test`, and `spec` all exit 0. 205 tests pass. `demos: 15 green, 7 red`. The ceiling went from 7450 to 9025.

The ticket stays at `in progress`.
