# 0077: Build one process width cap

Status: built on `ticket/0077-process-width-cap`, rebased onto main at `65f43693` (0076 landed). Code review pending: a fresh read-only Claude session reviews the final diff. Owner: Claude.

## Result

`engine::Width` exists only for 1 through 32. `Width::new` refuses 0, 33, and anything larger as `Error::Usage("a width is a whole number from 1 through 32")`, whose kind is `Usage`.

`engine::Widths` is the one process-state component. It holds the selected explicit width, the count of attempts holding a permit, one mutex, and one condition variable. `Widths::select(Option<Width>)` is the private seam that tickets 0084 through 0086 consume. `None` never selects and returns the effective width: the selected one, else the fallback 4. The first explicit width selects itself and wakes every waiter. The same width again is accepted. A different width returns `WidthActive`, whose text is `width 4 is already active for this process; use width 4 or drop the width argument` with the active number.

`Widths::acquire` waits for room under the effective width. It reads the width again on every wake, so a waiter queued before a lower width activates cannot pass it. It observes cancellation and the deadline before every check and wakes at least every 50 ms, or sooner when the budget ends. A stopped waiter returns `Cancelled` or `Deadline` and holds nothing. The `Permit` gives its room back on drop.

`Client::post_observed` acquires a permit at the top of each attempt. That point comes after cache and replay decided a live send is needed. It comes before ticket 0073's attempt-start check, request accounting, and the transport call. The permit drops as soon as `send` returns, before the timeout rule, the retry wait, decoding, recording, and output. A retry takes a new permit. `send` stays private to `engine/http.rs`.

`process_width()` is the one accessor for the static `PROCESS_WIDTH`. `Client::new` reaches it through `client_width()`. Ticket 0096 can guard that accessor with its process-ID check and replace the whole `Widths` value.

The command keeps `--jobs` explicit. `schedule::width(Option<u8>)` turns `--jobs N` into `Some(Width)` and an omitted `--jobs` into `None`, selects, and returns the width the schedulers run at. `jobs_of` (ordinary records, `recognize`) and `annotate` call it before the recorder, key lookup, client, or input dispatch. `relate` refuses `--jobs` and `find` takes none, so both stay implicit. A conflict maps to `Failure::WidthActive`, exit 2, with the sentence above after `thinkthen: `. The `DEFAULT_JOBS` constant and the two `map_or(4, …)` copies in `annotate.rs` are gone. Help and output are unchanged.

`policy.py` gains `check_doors`. It refuses `ureq` in any production file but `engine/http.rs`. It refuses `PROCESS_WIDTH` anywhere but its declaration and the accessor in `engine/mod.rs`, test files included. Five planted doors must fail and three controls must pass, or the check fails.

ADR 0017 gains an amendment for the width row of section 5. `sdlc/planning/libraries/README.md` and `rust.md` now say the process shares one attempt gate and a later different width fails.

## Request-path inventory after 0083

Every live send reaches `Client::post_observed`, and nothing else in production names `ureq` (now a policy check).

- Direct judgments: `judge.rs` through `asking.rs` `Judging::row` for decide, choose, tag, score, filter, and rank. Tickets 0082 and 0083 added no other request path.
- Ordinary records and CSV or TSV tables: `cli/schedule.rs::over_records` over `engine::schedule`.
- Grouped `annotate`: `annotate.rs` over `engine::annotate_schedule`, each group through `ask_prepared`.
- Aggregate `find`: `find.rs` through `ask`.
- Split chunks: `PreparedRequests` chunks through `Asking::chunks` (`annotate`, `recognize`).
- `recognize` and `relate`: `Asking::chunks`, and records through `over_records`.
- Retries: the loop inside `post_observed`.
- The conformance runner: `engine::request::ask_profile`.

## Red then green

- The state tests first ran against a stub whose `select` returned 4 and recorded nothing. Six went red on selection. After `select` was real, a stub `acquire` that never waited left four gate tests red: both transition tests and the cancel test failed on `the waiter blocked on a full gate`, and the deadline test counted one attempt where it wanted none.
- Planted R4-12, a `Width` that accepts 0: `a_width_exists_only_for_1_through_32` failed.
- Planted R2-9, one gate per client (per settings): the child test failed with `5 requests in flight over a cap of 4` in the two-engine phase.
- Planted a missing gate (room for 68): the same child failed in the two-engine phase. With that phase skipped, the command-path phase failed the same way.
- Planted a permit held through the retry wait: `a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one` failed on `the wait holds no permit`.
- Planted a waiter that read the width once: both transition tests failed.
- Planted a permit taken before replay: `a_replayed_answer_takes_no_permit` failed.
- Planted an omitted `--jobs` that selects 4: `the_width_setup_maps_each_jobs_value_to_one_selection` and the command child failed on `an omitted --jobs selects nothing`.
- Planted a width selected after the input was read: the command child failed on `no input was read` (1 read, wanted 0).
- Planted `policy.py` plants: with the `ureq` rule off, the check printed both `ureq` plants as not refused. A real `PROCESS_WIDTH` reference added to `cli/find.rs` failed the check.

## Acceptance

| Criterion | Proof |
| --- | --- |
| Implicit engines never select; calls use 4 until an explicit width activates; later calls use it | `engine::width_tests::implicit_engines_never_select_and_follow_the_first_explicit_width` |
| Lower-width transition: four fallback permits held, implicit waiters on both sides of activation, one release at a time, nothing above 1 after activation, every waiter wakes | `a_lower_first_width_holds_every_waiter_until_the_old_attempts_drain`. Each waiter reports the active count as it enters, and each must see 1. Channels mark each waiter as blocked before the next phase |
| Higher-width transition wakes waiters without passing 8 | `a_higher_first_width_wakes_waiters_without_passing_it` |
| Widths 1, 4, 8, 32: first activates, same accepted, different refused with the winner's number; a two-thread race picks exactly one winner | `the_first_explicit_width_wins_and_only_a_different_one_is_refused` pins the sentence for every pair and once as a literal. `two_racing_first_widths_select_exactly_one_and_the_loser_names_it` runs 50 barrier races |
| Omitted `--jobs` does not activate 4; `--jobs 4` does; a later conflict fails before input, key lookup, connection, accounting, and request, each counted | `cli::schedule::width_tests::command_setup_child`, run in a child copy of the test binary. It counts input reads, the listener's connections and requests, the token's key lookups, and the usage file's `requests_sent`. `the_width_setup_maps_each_jobs_value_to_one_selection` tests the setup function over a private state |
| One held-reply listener: every live path shares one cap, no next request until a release, peak equals the cap | `shared_cap_child`, phase two. It runs decide on one text, decide with a retried 503, decide over records, choose over records, grouped `annotate`, split `annotate` under `max_questions: 1`, aggregate `find`, `recognize` under `max_questions: 1`, and `relate`, all at once in one process. The listener holds each answer until the test sends a token. The test releases one answer only after 150 ms pass with no new request. It fails if a fifth answer is ever held. The most held is exactly 4, and every path's marked body reaches the listener |
| A retry releases its permit for the wait and takes a new one; cache and replay take none; counts stay exact | `a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one` takes the width-1 permit during a 600 ms retry wait, and `requests_sent` is 2. `a_replayed_answer_takes_no_permit` replays under a full gate. Existing count tests pass |
| Cancellation while waiting returns cancelled, sends nothing, consumes no permit; workers join | `a_waiter_cancelled_behind_a_full_gate_sends_nothing_and_holds_nothing`. Existing cancellation and join tests pass |
| One accessor; `policy.py` refuses another reference, with a planted failure | `process_width()`; `check_doors` and its plants |
| A deadline spent behind a full gate returns `Deadline`, sends nothing, uses no permit | `a_deadline_spent_behind_a_full_gate_sends_nothing_and_holds_nothing` (200 ms budget) |
| R4-12: `Width` only for 1 through 32; 0 and 33 fail as `Usage` before any request | `a_width_exists_only_for_1_through_32`; `width_in(…, Some(0))` returns the usage failure |
| R2-9: two engines with different settings share one cap | `shared_cap_child`, phase one: two clients with 5 s and 7 s timeouts, four sends each. The most held is exactly 4 |
| Planted bugs, one per row | See Red then green |
| `None` for the lazy convenience engine and an omitted builder width | No public builder exists yet. `select(None)` is the seam, and the first state test pins it |
| Existing tests, focused tests, policy, exact ratchet, format, Clippy, `git diff --check`, four rungs, loopback only | See Gates |

## Budget

| Bound | Limit | Measured |
| --- | ---: | ---: |
| Production Rust files touched | 12 | 5: `engine/{mod,http}.rs`, `cli/{schedule,annotate,failure}.rs` |
| Nonblank production lines added | 550 | 183 added, 7 removed (`git diff -U0`) |
| Nonblank focused test lines added | 1,000 | 807: `engine/width_tests.rs` 405, `cli/schedule/width_tests.rs` 402 |

The ratchet rose from 46,972 to 47,955 (+983), the exact measured total. The commit names what grew and where I looked for duplication.

## Departures

- Two new test files sit outside `opens`: `engine/width_tests.rs` and `cli/schedule/width_tests.rs`. No opened file had room under the 500-line file limit. The ticket requires in-crate proofs of the command setup, so `tests/backend` could not hold them. Production stays inside `opens`.
- In the unit-test binary, `Client::new` gives each client its own gate. That binary runs hundreds of unrelated tests in one process. A shared cap of 4 there would let a held reply in one test push another test's deadline into the gate wait. The width tests that need the process gate run in a child copy of the binary, where a test-only flag gives every client the process gate. The compiled command and every `tests/backend` run use the process gate with no flag.
- The combined test counts held answers through the listener's own reply hook, not `Listener::peak`. `peak` can over-count by one, because the listener decrements after writing and the next request can arrive first. The global-queue test's known flake is the same race.
- The ticket asked for a synchronized test that "receives no next request until one held reply is released". The test proves it with a 150 ms quiet window before each release. A missing gate fails it at once, but the window is a time bound.
- 0076 named this ticket as the place to revisit the usage-lock window. In that window `requests_sent` can count an attempt that never went out after its deadline. The permit comes before accounting, so nothing changed there. Counter meanings are excluded here.
- No review-route line named Codex. The ticket already reads "A fresh Claude session reviews design and code", so nothing needed fixing.

## The default-width issue

`sdlc/issues/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` stays open. Ticket 0077 keeps the fallback at 4 on purpose and adds no numbers to the `--jobs` reference pages, so it fixes neither of the issue's two items. It does make the fallback one number for the process, so changing it later is a one-line change to `Width::FALLBACK`.

## Gates

This run covered build commit `a2097e53`, rebased on `65f43693`. `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` were unset. The one-minute load was 6.7 when the run started.

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint`: exit 0. This covered policy with `check_doors`, the ratchet at exactly 47,955, format, Clippy, and docs.
- `sdlc/scripts/test`: exit 0. 780 tests passed and none failed. The global-queue test passed.
- `sdlc/scripts/spec`: exit 0. Pages: 21 green, 1 coming.
- An earlier focused `tests/backend` run failed the known global-queue test once on `listener.peak() <= jobs` at `annotate/scheduling.rs:102`. It passed on each of three reruns. Workers equal the width in that test, so they never wait at the gate, and its timing is unchanged. Another ticket is fixing that test.
- This record and the ticket status line changed after the run, and neither is code. `policy.py`, `pages`, and `git diff --check` pass on the record commit.
- `sdlc/scripts/live` never ran. Every socket was `127.0.0.1`.

## What Ian can overturn

The same items as the ticket: the fallback width, the 1 through 32 range, the exact conflict sentence, and the one-cap-per-loaded-copy boundary.
