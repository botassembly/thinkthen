# 0117: The loopback backend arms the surface tests need

Status: built on `ticket/0117-backend-arms-for-surfaces`. The code review (`sdlc/records/0117-code-review.md`) found two blocking items and two gaps. All four are fixed below. Not merged.

## Result

- `/arm/delay/MS/v1` answers by the generic rule after MS milliseconds. The sleep runs on each connection's own thread through `Canned::after`. A value that is not a whole number gets status 500 and `the delay arm needs a whole number of milliseconds`. A value above 10000 gets status 500 and `the delay arm allows at most 10000 milliseconds`. Both answer at once.
- A whole number is ASCII digits only. `whole` in `arms.rs` checks the digits before it parses, so `+200` is refused. The delay arm and `wait` share it.
- The held gate holds an open flag and a round number. A held reply takes the round number when its answer is chosen. `round` and `Backend::round` add one to the number. `release` and `Backend::release` set the flag and stay permanent.
- `serve_kept` in `listener.rs` adds the request to in-flight and peak, then chooses the answer, then adds it to the count. A comment says why.
- `Backend::wait(n)` returns the count once it reads at least N, or at 5 s. The `wait N` line runs it on a thread of its own and then prints `wait K`. Every output line goes through one lock.
- `run` now takes an output that is `Send + 'static`, and `main` passes `io::stdout()`. The `wait` thread is detached, so closing standard input prints the final count and exits without waiting for it.
- `Listener` keeps its record receiver behind a `Mutex`. `Backend` is then shareable with the `wait` thread. The public API of `Listener` is unchanged.
- `conformance/README.md` gains the delay row with both sentences, the `round` and `wait` lines, the whole-number rule, and the section "One backend per test".
- The listener issue gains the third listener at `src/cli/edge/deadline_tests.rs` and the plan for the move.

## The three notes from the design confirmation

- The `last` helper in `binary.rs` takes the final count and skips any line that starts with `wait `.
- `run` takes an output that a separate thread can own (`impl Write + Send + 'static`). It uses a detached thread, not `thread::scope`.
- `sdlc/records/0117-design-review.md` holds the review and its confirmation, and it lands with the ticket.

## Red on main

The new tests ran against the source from `origin/main`. The in-process test did not compile, because `Backend::wait` and `Backend::round` do not exist there. With that one test left out, 9 of 14 failed: both delay tests, the refusals, both round tests, both `wait` tests that print, the held-reply test that now reads `wait 1`, and the two-binary test. The 5 that passed test behavior main already has: the port and count line, bind, a permanent release, a bad `wait` as an unknown line, and twenty starts.

## Each plant turns its test red

Each plant was applied to the committed source and its test was run alone. Then the source was restored from Git. All fifteen failed as the ticket predicts. The table gives what the failing test printed.

| Plant | Test | Red |
| --- | --- | --- |
| Skip the delay's `after` call | `the_delay_arm_answers_after_its_delay` | "it answered early" |
| One shared lock across the sleep | `eight_delayed_replies_wait_in_parallel` | "the delays ran in turn", 1.61 s |
| Read a bad delay as zero | `the_delay_arm_refuses_…` | `abc` got `HTTP/1.1 200 X` and the generic answer |
| Parse with `u64::from_str` | `the_delay_arm_refuses_…` | `+200` got `HTTP/1.1 200 X` |
| Drop the ceiling | `the_delay_arm_refuses_…` | 10001 got no answer within 1 s |
| Refuse at 10000 | `the_delay_arm_refuses_…` | 10000 got the ceiling refusal within 300 ms |
| `round` sets the open flag | `four_rounds_on_one_backend_…` | round two answered before its `round` line |
| Count before choosing, 20 ms apart | `fifty_rounds_back_to_back_…` | the first round's reply timed out at 1 s |
| `release` acts as `round` | `a_release_stays_open_for_later_held_replies` | the later decide timed out at 1 s |
| Print the count at once | `a_wait_line_answers_once_the_count_reaches_it` | read `wait 0`, wanted `wait 1` |
| Drop the 5 s bound | `a_wait_line_gives_up_at_5_s_…` | no `wait` line within the 7 s timeout |
| Run the `wait` on the input thread | `a_wait_line_gives_up_at_5_s_…` | no `0` line within 300 ms |
| Read a bad `wait` number as zero | `a_wait_line_with_no_whole_number_prints_nothing` | `wait 0` arrived after the count |
| One process-wide gate | `two_backends_in_one_process_…` | `round` on the first let the second's reply go |
| Bind a fixed port | `twenty_backends_start_at_once_on_twenty_ports` | the second start's port line never came |

The bad-`wait` test first followed the ticket's wording: send three lines, close the input, and require exactly the lines `0` and `0`. Its plant stayed green. The process exited before the planted `wait 0` thread printed. The test now reads the `0` line and requires 300 ms of silence before it closes the input, the form the design review offered in finding 2. Under the plant it fails. The lines it requires are the same two.

## Acceptance

| Criterion | Proof |
| --- | --- |
| Delay: generic answer no sooner than 200 ms and within 1 s | `the_delay_arm_answers_after_its_delay`, green |
| Delay, parallel: eight within 800 ms | `eight_delayed_replies_wait_in_parallel`, green |
| Delay, refusals: pinned bodies within 1 s, 10000 unanswered at 300 ms | `the_delay_arm_refuses_a_bad_value_and_one_above_its_ceiling_at_once`, green. It also refuses a value too large for 64 bits |
| Round: four rounds on one backend | `four_rounds_on_one_backend_each_let_go_only_the_reply_held_then`, green |
| Round, back to back: fifty within 5 s, 1 s per reply | `fifty_rounds_back_to_back_each_let_go_the_reply_they_counted`, green |
| Release stays permanent | `a_release_stays_open_for_later_held_replies`, green |
| Wait, reached | `a_wait_line_answers_once_the_count_reaches_it`, green |
| Wait, bounded and apart: `0` within 300 ms, `wait 0` between 5 s and 6 s | `a_wait_line_gives_up_at_5_s_and_holds_up_no_line_behind_it`, green |
| Wait, bad input: exactly `0` and the final `0` | `a_wait_line_with_no_whole_number_prints_nothing`, green |
| One backend per test, in process and as binaries | `two_backends_in_one_process_keep_their_own_count_gate_and_port` and `two_backend_binaries_keep_their_own_count_gate_and_port`, green |
| Cheap start: twenty alive, distinct ports, within 5 s | `twenty_backends_start_at_once_on_twenty_ports`, green |
| Every plant turns red | The table above |
| Output read with a 7 s receive timeout | A reader thread in `start` feeds a channel. Every read is a `recv_timeout` |
| Existing tests stay green on the new reader | The port and count test, `a_held_reply_waits_for_a_release_line` (now on `wait 1`), and `it_binds_127_0_0_1_only` pass. `a_held_reply_answers_only_after_its_release` in `crates` passes in the `test` rung |
| README changes | `conformance/README.md`, 15 lines added |
| `git diff origin/main -- crates` is empty | The branch changes no file in `crates` |
| No dependency | `Cargo.toml` is unchanged |

The start cost measured again: twenty debug binaries started at once, and each read its port line. Three runs took 20 ms, 21 ms, and 18 ms, with twenty distinct ports each, at a one-minute load near 5. The design review measured 112 ms at a load near 13.

## Code review fixes

- **In-flight order.** Choosing the reply before the in-flight count hid a blocked request from the peak. The 0116 test `one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` then flaked, because its `answering` closure blocks. Only the request count now follows the reply. On the merged tree `8d568bd1` that test passed 20 of 20 runs.
- **`wait +0`.** The bad-`wait` test sends `wait +0` in place of `wait +1`. With a `wait` parse through `str::parse::<usize>`, it read `wait 0` and failed.
- **Standard error.** `a_delay_refusal_says_why_on_standard_error` pins both refusal lines. With the whole-number refusal built without `drift`, it read only the ceiling line and failed.
- **Exit.** `closing_the_input_exits_without_waiting_for_a_pending_wait` sends `wait 1`, closes the input, and requires the exit within 1 s with only the final count. With `run` joining every pending `wait`, it read `wait 0` and `0` after 5 s and failed.
- The round-order plant, with count before choosing and a 20 ms sleep, still turns `fifty_rounds_back_to_back_…` red under the new order.

## Budgets

| Budget | Limit | Measured, nonblank lines |
| --- | --- | --- |
| `conformance/backend/src` | 80 | 77 |
| `conformance/backend/tests/binary.rs` | 260 | 232 |
| `conformance/README.md` | 30 | 15 |
| Ratchet rise | 340 | 335 |

The source measures 78 and the tests 257 after the review fixes. The ratchet rose in three commits: `4ce44871` added 299, `db84ef82` added 10 for the bad-`wait` silence check, and `fe40e545` added 26 for the review fixes. The merge with `origin/main` at `9f47bd18` set it to the measured 48475, main's 48140 plus 335.

## Checks

The full ladder ran at `f125072c`, the merged tree with every code fix, with `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `THINKTHEN_URL` unset. The one-minute load was 9.88 when it started.

- `install`: exit 0.
- `lint`: exit 0, with the ratchet at 48475/48475.
- `test`: exit 0. The cargo suites reported 795 passed and 0 failed, including all 17 tests in `binary.rs`. `live-test: all cases passed`. It uses dummy keys and local jobs.
- `spec`: exit 0, `demos: 21 green, 0 red`.

The commit after `f125072c` adds only this section. `sdlc/scripts/live` did not run.
