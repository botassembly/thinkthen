# 0076: Build whole-call deadlines

Status: built on `ticket/0076-whole-call-deadlines`, rebased onto main at `18c0dc10`. Code review pending: a fresh Claude session reviews. Owner: Claude.

## Result

`engine::Deadline` holds a caller's budget and the one `Instant` made from it by `Deadline::after`. `Deadline::after` returns `None` when `checked_add` cannot hold the budget, so no instant means no deadline and no panic. The deadline rides on the existing `Cancel` token: `Cancel::with_deadline` shares the stop flag and adds the one deadline. Every existing stop checkpoint now calls `Cancel::stop_or_remaining` or `Cancel::stop`. Each of them observes cancellation first and the deadline second. `Cancel::wait` wakes at the deadline and returns the stop it observed. Each caller returns that stop unchanged.

`Error::Deadline` now carries `Budget(Duration)`. `Kind::Deadline` stays separate from `Kind::Backend`. `Budget`'s private `Display` prints `the deadline of 5 s passed before the call answered`. It uses whole seconds, else whole milliseconds, else nanoseconds, all as integers. The command still has no deadline option. Its conversion keeps the existing `Failure::Defect("an unavailable deadline reached the command")` for an engine deadline, so the command gains no diagnostic and no exit code.

`Client::post_observed` sets each attempt's blocking limit to the lesser of `--timeout` and the remaining budget. The limit is set through ureq's per-request `timeout_global`. An attempt that times out under a limit set by the deadline returns the deadline, even when cancellation was set during the send. A timeout under the plain `--timeout` stays `Transport(Timeout)`. Retry waits still honour 0064's cap, and they also end at the deadline. 0089's rule stands: a transport failure is never retried.

## Request-path inventory after 0083

Every live request path reaches `engine::request::ask_prepared` through `cli/asking/request.rs`. Every send reaches `http::Client::post_observed`. The paths are:

- direct judgments: `judge.rs` through `asking.rs` `Judging::row` (decide, choose, tag, score, filter, rank);
- ordinary records and CSV/TSV tables: `cli/schedule.rs::over_records` over `engine::schedule`;
- aggregate `find`: `find.rs` through `ask`;
- grouped `annotate`: `annotate.rs` over `engine::annotate_schedule`, with each group's chunks through `ask_prepared`;
- split requests: `PreparedRequests` chunks through `Asking::chunks`;
- `recognize` and `relate`: `Asking::chunks`, and records through `over_records`.

No path opens another transport door. The re-score condition did not trigger.

## Red then green

- A stub `stop_or_remaining` that ignored the deadline turned 11 engine tests red. It turned the recorder-gate test and both command-path tests red too; the command tests failed on `decide key lookups` and `records key lookups`. The held-lock and held-gate tests hung until the owner released, and the annotate-group test hung the same way. The display, overflow, unbounded, and no-deadline control tests passed under the stub, as they should. The real `stop_or_remaining` turned all of them green.
- Planted R1-23 (a retry wait that ignores the deadline: `Cancel::wait` observed only cancellation). `a_retry_wait_past_the_budget_ends_as_the_deadline_without_a_second_send` failed on `started.elapsed() < SECOND * 10` after 30.00 s.
- Planted R6-4 (an empty-input path that skips the spent-deadline check: the record scheduler checked only after one dispatch). `a_spent_deadline_over_empty_bulk_input_stops_before_reading` failed on its `Outcome::Stopped` match and reached a complete, empty run.

## Acceptance

| Criterion | Proof |
| --- | --- |
| A spent deadline on every direct path returns the deadline before key lookup, request accounting, connection, or send, with each boundary counted | `cli::edge::deadline_tests::a_spent_deadline_on_every_direct_path_sends_nothing` covers decide, find, annotate, recognize, recognize split under a `max_questions: 1` profile, relate, and relate under that profile. `a_spent_deadline_over_records_stops_before_the_first_request` covers decide records, empty records, annotate records, and empty annotate input. Each case counts key lookups on the call's token, the usage file's `requests_sent`, and nonblocking `accept` on the loopback listener, and checks empty stdout. Each pins the exact diagnostic and exit 70. At the engine, `a_spent_deadline_stops_a_prepared_request_before_its_key` counts the key and send closures, and `a_spent_deadline_opens_no_connection_and_observes_no_attempt` counts the attempt observer and the listener |
| One instant spans every record, chunk, and group; no undispatched request starts; ordered results, stop metadata, cleared groups, joined workers | `deadline_tests::schedule::one_deadline_spans_every_record_and_starts_no_undispatched_request`: record 0 finishes, record 1 is held past the budget, record 2 is never read or started, and the result is `[0, 1]` and `Stopped { at: 3, finished: 2 }`. `a_deadline_clears_undispatched_annotation_groups`: one group starts, the second never does, and the run stops at row 1. Both schedulers run inside `thread::scope`, so every worker has joined when they return |
| A held reply returns the deadline before the listener releases, after one send and no second connection; a control with no deadline reaches the backend timeout | `a_held_reply_ends_as_the_deadline_after_one_send` (300 ms budget, 30 s attempt timeout) and `without_a_deadline_the_held_reply_reaches_the_attempt_timeout`. The scripted server counts connections and checks for a later one after release. Channels mark the phases |
| A retry delay past the budget returns the deadline during the wait with no second connection; a shorter delay retries; a retry that starts in time ends its send as the deadline | `a_retry_wait_past_the_budget_ends_as_the_deadline_without_a_second_send` (`retry-after-ms: 60000`, returns within 10 s), `a_retry_wait_inside_the_budget_still_retries` (two attempts, `requests_sent` 2), and `a_retry_started_inside_the_budget_ends_its_send_as_the_deadline` (two attempts, two connections). The attempt observer counts only started attempts |
| Held folder, recorder mutex, and digest lock return the deadline without the owner and send nothing | `a_held_folder_ends_as_the_deadline_without_the_owner` and `a_held_digest_ends_as_the_deadline_without_the_owner` check that the waiter met the lock. They count zero key lookups and zero sends and release the owner before any file assertions. `recorder::tests::a_held_recorder_gate_ends_as_the_deadline_without_the_owner` shows the waiter never opened or made the folder. The existing 0073 cancellation and cleanup-failure tests pass unchanged |
| Checkpoint precedence | Cancellation before the deadline: `cancellation_is_observed_before_a_spent_deadline` (token and wait), the HTTP and prepared-request tests above, and the fired-and-spent case in `a_spent_deadline_over_empty_bulk_input_stops_before_reading`. Barrier race: `a_result_queued_before_the_deadline_check_keeps_its_place_and_the_order` holds the coordinator in `emit` while a later result is queued and the budget passes. A queued success keeps its place and the deadline stops the next place. A queued backend failure at an earlier place is the reported cause. Either arrival order gives the same outcome, because the stop is reported by input place. A deadline-derived timeout with cancellation set during the send: the `cancel_in_flight` arm of the held-reply test. Budget and kind: `the_deadline_error_keeps_its_budget_and_prints_it_in_integers` |
| Empty bulk input returns the deadline; integer budget text; an unrepresentable instant means no deadline | `a_spent_deadline_over_empty_bulk_input_stops_before_reading` (record scheduler; annotate streaming and aggregate) and the `empty-records` and `empty-annotate` command cases. `4294967295 s` and `Duration::MAX` print exactly. `a_budget_no_instant_can_hold_means_no_deadline` |
| An unbounded call keeps attempt timeout, retry waits, counts, order, cache/replay, and exact bytes | The command builds `Cancel::default()`, which has no deadline. `stop_or_remaining` then returns `Ok(None)` and the attempt limit equals `--timeout`. `a_budget_no_instant_can_hold_means_no_deadline` and the no-deadline control pin this. The existing unit, backend, conformance, and spec suites, all green below, cover retry waits, request counts, order, cache and replay, and request and recording bytes. No encode or recording code changed |
| Planted-bug proof for R1-23 and R6-4 | See Red then green |
| Focused tests, policy, exact ratchet, format, Clippy, `git diff --check`, four rungs, no live call, loopback only | See Gates. `sdlc/scripts/live` never ran. Every socket is `127.0.0.1` |

## Budget

| Bound | Limit | Measured |
| --- | ---: | ---: |
| Production Rust files touched | 12 | 12: `engine/{mod,error,http,request,recorder,cache_lock,schedule,annotate_schedule}.rs`, `cli/{schedule,annotate_schedule,edge}.rs`, `cli/failure/convert.rs` |
| Nonblank production lines added | 500 | under 200 gross; `git diff -U0` counts 199 added nonblank lines in those files, test-only lines in `mod.rs` and `recorder.rs` included |
| Nonblank focused test lines added | 1,000 | 847: `engine/deadline_tests.rs` 360, `engine/deadline_tests/schedule.rs` 301, `cli/edge/deadline_tests.rs` 164, and the `recorder.rs` test |

The ratchet rose from 44,778 to 45,721 (+943). Most of the growth is tests (about 850 lines). The production growth is the deadline type, the stop observation, and the integer budget text. Before adding code I looked for duplication to delete. `Cancel::wait` and the three lock loops already shared one poll; each now returns the stop it saw, so no second wait or timer was added. The two scheduler stop mappings became one `Fn(Error) -> E`, which replaced two cancel-only closures. One duplicate remains: the engine test fixture `request()` repeats the private one in `request.rs`'s test module. That module is private, and reaching it would mean widening a test module's visibility.

## Departures

- The command-path proofs pin the command's existing defect sentence for an engine deadline. The ticket forbids a command diagnostic, and the command has no deadline kind to print.
- Key lookups are counted at the engine's one key call in `ask_prepared`, through a test-only counter on the call's token. A process-wide counter in `edge::key` would have raced other in-crate tests that look up a key.
- `tests/backend` is untouched, because every command-path proof is an in-crate unit test, as the ticket requires.

## Gates

One sequential run on the final tree, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. The one-minute load stayed between 5 and 8 on the shared 16-core machine.

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint` (format, Clippy with `-D warnings`, policy, exact ratchet at 45,721, per-file size): exit 0. An earlier run failed on per-file size (`annotate_schedule.rs` 502, `deadline_tests.rs` 652, ceiling 500). The scheduler tests then moved to `deadline_tests/schedule.rs`, and two stop checks became one line each.
- `sdlc/scripts/test`: exit 0. The global-queue test did not flake.
- `sdlc/scripts/spec`: exit 0 (demos 21 green, 0 red).
- `git diff --check`: clean.

## What Ian can overturn

The same items as the ticket: cancellation-before-deadline precedence, the later public sentence and its unit choice (s, then ms, then ns), and the budgets.
