---
flow: build
priority: 117
opens: conformance/backend conformance/README.md sdlc/ratchet.json sdlc/issues/2026-09-25-test-harness-and-review-leftovers.md
---

# 0117: Add the loopback backend arms the surface tests need

Status: landed 2026-09-24. The code review (`sdlc/records/0117-code-review.md`) accepted `816f253f` after four fixes. Owner: Claude.

## Design and decisions

The whole set is three additions and one proof. Each one answers a finding that more than one surface ticket hit. Ian can overturn each arm's form, the 10,000 ms ceiling, and the 5 s bound.

A whole number means ASCII digits only, in the delay arm and in `wait` alike. `+200`, `-1`, an empty value, and a value too large for `u64` are not whole numbers. Rust's `u64::from_str` accepts `+200`, so the parse checks the digits first.

1. **The delay arm, `/arm/delay/MS/v1`.** It answers by the generic rule after sleeping MS milliseconds. The sleep runs on the connection's own thread through the existing `Canned::after`. Concurrent requests then wait in parallel. A refusal answers at once with the drift status, a pinned body, and the same sentence on standard error:
   - a missing value or one that is not a whole number: `the delay arm needs a whole number of milliseconds`;
   - a whole number above 10,000: `the delay arm allows at most 10000 milliseconds`.

   The ceiling stays under the engine's 30 s request timeout. A delayed reply then never reads as a hang. Users: 0106 (width equality and the R1-24 deadline), 0107, and any test that needs a release at a set time (0105 R4-23, 0109 finding 3, 0112's 2 s timer). 0106 decision 10 carried this design first. It moves here unchanged apart from the strict parse.

2. **Rounds on the held arm.** A new `round` line on standard input, and `Backend::round()` in the library, let go every reply held at that moment. A reply that arrives later holds again. A test can hold and let go any number of times on one backend. The existing `release` keeps its meaning: every held reply goes, now and from here on. Callers on main and in accepted tickets rely on that.
   - Mechanism: the gate holds a round number and an open flag. A held reply takes the round number when its answer is chosen. It waits until the flag is set or the round number has moved past its own. `release` sets the flag. `round` adds one to the number.
   - Order rule: the listener chooses the answer before it adds the request to the count. Today it counts first (`listener.rs`, `serve_kept`). With the old order, a test could read the count, send `round`, and still leave the request holding for the next round. The builder moves the `reply(&request)` call above the request count, and a comment says why. The in-flight and peak updates stay above the reply. The 0116 scheduling test has an `answering` closure that blocks, and its peak must count a blocked request.
   - Users: 0112 R4-2 (four rounds in one test), and any test that holds after an earlier release.

3. **A bounded wait: `Backend::wait(n)` and the `wait N` line.** `Backend::wait(n)` returns the count once it reads at least N, or at 5 s, whichever comes first. The `wait N` line runs it on a thread of its own and then prints `wait K`, where K is the count it returned. The input lines behind a `wait` run at once, so a `round` or `release` from a test's timer never waits behind it. All output lines go through one lock. The prefix tells a `wait` line from a `count` line when they interleave. Closing standard input prints the final count and exits without waiting for a pending `wait`. A `wait` with no whole number is an unknown line: a line on standard error and nothing on standard output.
   - The test compares K with N, so a plant fails on the assertion and never waits out an outer timeout.
   - Every surface ticket polls "once the count reads N", and 0109 and 0112 each ask for a 5 s bound. One line in the backend replaces seven hand-written loops. Rust tests call `Backend::wait` directly.

4. **One backend per test, proved and documented.** No new code. The binary binds port 0, and its count, gate, and rounds live in the process. The review measured 20 starts of 0092's debug binary, all alive at once, at 112 ms at a load near 13 (`sdlc/records/0117-design-review.md`). The build record measures it again. A process per test is cheap enough. It isolates the count, the gate, and the port with no shared state to reset. A namespace inside one process was considered and dropped. It adds routing and state for an isolation the operating system already gives.
   - The README gains a section "One backend per test" with the recipe: start the binary, read the port line, drive `count`, `wait`, `round`, and `release` from one writer, and close standard input at the end. It says that a `wait` answers on its own line later and never holds up the lines behind it.

Not added:
- An HTTP control path for `count` and `release`. Every surface test starts its own backend, so the test owns its standard input.
- A time limit on held replies. A test that needs a timed release uses the delay arm.
- Any change to the case, generic, reset, 429, 503, refuse, or malformed arms.

## Outcome and authority

Give every surface's tests the timing tools that 0092's backend lacks, in one place, before the surface tickets are built. The design reviews of 0106, 0109, 0111, and 0112 each hit the same limits. The held arm's release is permanent, so a test cannot hold twice. No arm answers after a fixed delay, so no wall-time proof can run. Every surface polls the `count` line with its own bounded loop. The shared rules in `sdlc/planning/surfaces-port-guide.md` (2026-09-24) give each test its own backend and name this ticket for the rest.

The work stays inside `conformance/`. Production code in `thinkthen` is unchanged. It depends on nothing beyond main and can be built now.

## The duplicated listeners

0117 leaves them to a later ticket, and the issue stays open. Three reasons:
- They sit in `crates/thinkthen/src`. 0113, 0114, and the engine tickets build there now. Keeping 0117 inside `conformance/` lets it land beside them with no collision.
- None of them needs a 0117 arm. `serve_once` needs one scripted reply, and `Listener::serving` already gives it. The engine deadline server in `src/engine/deadline_tests.rs` needs a 503 with a wait, an answer, a hold, an arrival signal, and a check for a later connection. `Canned::status(..).asking(..)`, `Canned::after_release`, `Listener::answering_with_events`, and `Listener::connections` cover these. The two spent-deadline tests only need a listener that counts connections.
- The move is a test refactor with its own proof. The moved tests must still turn red on their old plants.

The issue lists `serve_once` and the listeners in `src/engine/deadline_tests.rs`. A third sits in `src/cli/edge/deadline_tests.rs` (`spent`, line 38). The record step appends that listener and the plan above to the issue. The transport tests in `src/engine/http.rs` keep their raw sockets, because they test the bytes on the wire.

## Acceptance

Every test runs in `conformance/backend/tests/binary.rs`. A new reader thread sends each line of the backend's standard output into a channel. The `line` helper then reads with a 7 s receive timeout. On main it calls a blocking `read_line`, and a planted hang would stall the rung. HTTP answers keep their `recv_timeout`. The record plants each bug, shows the test red, removes the plant, and shows it green.

| Arm | Test | Planted bug |
|---|---|---|
| Delay | One decide on `/arm/delay/200/v1` gets the generic answer no sooner than 200 ms and within 1 s. | Parse the value and skip the `after` call. The answer comes at once. |
| Delay, parallel | Eight decides on eight connections to `/arm/delay/200/v1` all finish within 800 ms. | Hold one shared lock across the sleep. The eight take about 1.6 s. |
| Delay, refusals | `/arm/delay/abc/v1`, `/arm/delay/+200/v1`, and `/arm/delay/v1` each get status 500 and the whole-number body within 1 s. `/arm/delay/10001/v1` gets status 500 and the ceiling body within 1 s. `/arm/delay/10000/v1` gets no answer within 300 ms. | Three plants. Read a bad value as zero: `abc` answers with status 200. Parse with `u64::from_str`: `+200` answers with status 200. Drop the ceiling: 10001 gets no answer within 1 s. A fourth plant refuses at 10,000: that value answers within 300 ms. |
| Round | Four rounds on one backend. In each, a held decide goes out, `wait K` prints `wait K`, the reply is still unanswered 300 ms later, `round` goes in, and the answer arrives within 1 s. | Make `round` set the open flag. Round two answers before its `round` line. |
| Round, back to back | Fifty rounds in a row. Each sends a held decide, reads `wait K`, sends `round`, and waits for its reply with a 1 s timeout. All fifty end within 5 s. | Restore count-before-choose and sleep 20 ms between the count and the `reply` call. `wait K` returns before the reply takes its round number. The `round` lands first, the reply holds, and the first round fails. |
| Release stays permanent | After `release`, a new held decide answers within 1 s. | Make `release` act as `round`. The new decide holds, and the test fails on its timeout. |
| Wait, reached | `wait 1` goes in first. A held decide is posted 200 ms later. The line `wait 1` arrives within 1 s. | Print the count at once. The line reads `wait 0`. |
| Wait, bounded and apart | With no request, `wait 1` goes in and then `count`. The line `0` arrives within 300 ms. The line `wait 0` arrives between 5 s and 6 s. | Two plants. Drop the bound: no `wait` line arrives, and the 7 s timeout fails. Run the `wait` on the input thread: the `count` line waits 5 s. |
| Wait, bad input | `wait x`, then `wait +1`, then `count`, then standard input closes. Exactly two lines follow, `0` and the final `0`, and then the output ends. | Read a bad number as zero. A `wait 0` line appears, and the line count turns red. |
| One backend per test | The proof runs twice: on two in-process `Backend` values with `Backend::wait`, and on two binaries with `wait`. In each run the two backends have different ports and each holds one decide. `round` on the first lets go only its own reply. After a second decide to the first, the first reads 2 and the second still reads 1. | Keep the gate in a process-wide static in the library. In the in-process run, `round` on one lets both replies go. |
| Cheap start | Twenty backends start and stay alive until every port line is read. The twenty ports are distinct, and all twenty port lines arrive within 5 s. Then each closes its input and exits. | Bind a fixed port. The second start fails while the first still holds it. |

Also:
- The existing binary and loopback tests stay green. They move onto the new reader with no change to what they assert. They include `a_held_reply_answers_only_after_its_release`, `a_held_reply_waits_for_a_release_line`, and the bind and exit tests.
- `conformance/README.md` adds the delay row with both refusal sentences, the `round` and `wait` lines, the whole-number rule, and the section from decision 4.
- `git diff origin/main -- crates` is empty.

## The check it adds to the gate ladder

No new rung and no new script. The root `test` rung already runs `conformance/backend/tests/binary.rs`. `lint` holds the crate to its tables as 0092 set them.

## Budgets and the ratchet

- `conformance/backend/src`: at most 80 nonblank lines added. That covers the digit parse, the delay arm, the two-field gate, `Backend::round`, `Backend::wait`, the `wait` thread, and the output lock in `run`.
- `conformance/backend/tests/binary.rs`: at most 260 nonblank lines added, including the reader thread.
- `conformance/README.md`: at most 30 lines added.
- No dependency. The crate keeps `serde` and `serde_json` only.
- The ratchet. `sdlc/ratchet.json` counts `.rs` files under `crates` and `conformance`. It rises to the measured total in the commit that adds the code, at most 340. That commit says what grew and why. It names where the builder looked for a duplicate to delete first: the held-reply loop in `binary.rs`, the one in `loopback_arms.rs`, and a shared post helper for the new tests. The `wait` line replaces the loop in `binary.rs`.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen`, or changing an existing arm's answer.

## Exclusions

Retiring the duplicated listeners (above). Any change to `thinkthen`. Any arm or line that no surface ticket names. Any live or paid call.

## Dependencies

After nothing beyond main. Before 0106. 0106 drops its own delay-arm design and depends on this ticket. The other surface tickets may use `round` and `wait` once this lands. Each owner decides whether to rewrite its polls onto `wait`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The raised ceiling and the widened public surface (`Backend::round`, `Backend::wait`, and the new bound on `run`'s output) call for the second-agent review the repo requires. The code review names it.

## Complexity

Contract 1; state and timing 3; reach 1; proof 2; cost of error 1; total 8. Final level: 2. The round order rule and the `wait` thread are the timing risks.

## Review

- Design review: `sdlc/records/0117-design-review.md` found eight items. The arms held. This version adds the output reader with a timeout and a plant that can turn the `wait x` test red. It adds the order plant for back-to-back rounds, and bounds, pinned sentences, and a digits-only parse for the delay refusals. It keeps all twenty backends alive until every port is read and runs `wait` off the input thread. It sets the parallel bound at 800 ms, corrects the `spent` line, and adds `Backend::wait`. The re-review at `dbb55129` accepted the design and left three notes for the builder. The same record holds the confirmation.
- Build: `sdlc/records/0117-build-backend-arms-for-surfaces.md` gives every criterion, its proof, and each plant turning red.
- Code review: `sdlc/records/0117-code-review.md` found the in-flight order flake, a `wait +1` case that could not fail, and two untested promises. All four are fixed, and the build record shows each fix. The confirmation accepted `816f253f`.
