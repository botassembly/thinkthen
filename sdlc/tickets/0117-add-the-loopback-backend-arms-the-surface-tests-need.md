---
flow: build
priority: 117
opens: conformance/backend conformance/README.md sdlc/ratchet.json sdlc/issues/2026-09-24-hand-rolled-loopback-listeners-duplicate-the-test-backend.md
---

# 0117: Add the loopback backend arms the surface tests need

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Give every surface's tests the timing tools that 0092's backend lacks, in one place, before the surface tickets are built. The design reviews of 0106, 0109, 0111, and 0112 each hit the same limits. The held arm's release is permanent, so a test cannot hold twice. No arm answers after a fixed delay, so no wall-time proof can run. Every surface polls the `count` line with its own bounded loop. The shared rules in `sdlc/planning/surfaces-port-guide.md` (2026-09-24) give each test its own backend and name this ticket for the rest.

The work stays inside `conformance/`. Production code in `thinkthen` is unchanged. It depends on nothing beyond main and can be built now. Ian can overturn each arm's form, the 10,000 ms ceiling, and the 5 s bound.

## Design and decisions

The whole set is three additions and one proof. Each one answers a finding that more than one surface ticket hit.

1. **The delay arm, `/arm/delay/MS/v1`.** It answers by the generic rule after sleeping MS milliseconds. The sleep runs on the connection's own thread through the existing `Canned::after`, so concurrent requests wait in parallel. A missing value, a value that is not a whole number, and a value above 10,000 each get the drift status and a line on standard error. The ceiling stays under the engine's 30 s request timeout. A delayed reply then never reads as a hang. Users: 0106 (width equality and the R1-24 deadline), 0107, and any test that needs a release at a set time (0105 R4-23, 0109 finding 3, 0112's 2 s timer). This design is the one 0106 decision 10 carried, moved here unchanged.

2. **Rounds on the held arm.** A new `round` line on standard input, and `Backend::round()` in the library, let go every reply held at that moment. A reply that arrives later holds again. A test can hold and let go any number of times on one backend. The existing `release` keeps its meaning: every held reply goes, now and from here on. Callers on main and in accepted tickets rely on that.
   - Mechanism: the gate holds a round number and an open flag. A held reply takes the round number when its answer is chosen. It waits until the flag is set or the round number has moved past its own. `release` sets the flag. `round` adds one to the number.
   - Order rule: the listener chooses the answer before it adds the request to the count. Today it counts first (`listener.rs`, `serve_kept`). With the old order, a test could read the count, send `round`, and still leave the request holding for the next round. The builder moves the `reply(&request)` call above the count, and a comment says why.
   - Users: 0112 R4-2 (four rounds in one test), and any test that holds after an earlier release.

3. **A bounded wait, the `wait N` line.** It prints the count once the count reads at least N, or at 5 s, whichever comes first. The test then compares the printed number with N. A plant then fails on the assertion and never waits out an outer timeout. Every surface ticket polls "once the count reads N", and 0109 and 0112 each ask for a 5 s bound. One line in the backend replaces seven hand-written loops. A `wait` with no whole number is an unknown line: a line on standard error and nothing on standard output. The library needs no counterpart, because a Rust test already reads `Backend::count` directly.

4. **One backend per test, proved and documented.** No new code. The binary already binds port 0, and its count, gate, and rounds live in the process. The drafter measured 0092's debug binary on 2026-09-24: 20 starts in a row took 226 ms in total under a load of 10.8. The build record measures it again. A process per test is cheap enough, and it isolates the count, the gate, and the port with no shared state to reset. 0117 adds a test that proves the isolation and a README section with the recipe: start the binary, read the port line, drive `count`, `wait`, `round`, and `release`, and close standard input at the end. A namespace inside one process was considered and dropped. It adds routing and state for an isolation the operating system already gives.

Not added:
- An HTTP control path for `count` and `release`. Every surface test starts its own backend, so the test owns its standard input.
- A time limit on held replies. A test that needs a timed release uses the delay arm.
- Any change to the case, generic, reset, 429, 503, refuse, or malformed arms.

## The duplicated listeners

0117 leaves them to a later ticket, and the issue stays open. Three reasons:
- They sit in `crates/thinkthen/src`. 0113, 0114, and the engine tickets build there now. Keeping 0117 inside `conformance/` lets it land beside them with no collision.
- None of them needs a 0117 arm. `serve_once` needs one scripted reply, and `Listener::serving` already gives it. The engine deadline server in `src/engine/deadline_tests.rs` needs a 503 with a wait, an answer, a hold, an arrival signal, and a check for a later connection. `Canned::status(..).asking(..)`, `Canned::after_release`, `Listener::answering_with_events`, and `Listener::connections` cover these. The two spent-deadline tests only need a listener that counts connections.
- The move is a test refactor with its own proof: the moved tests must still turn red on their old plants.

The issue lists `serve_once` and the listeners in `src/engine/deadline_tests.rs`. A third sits in `src/cli/edge/deadline_tests.rs` (`spent`, line 40). The record step appends that listener and the plan above to the issue. The transport tests in `src/engine/http.rs` keep their raw sockets, because they test the bytes on the wire.

## Acceptance

Every test below runs in `conformance/backend/tests/binary.rs` against the compiled binary, so it proves the standard-input lines and the library at once. Each reads standard output with a receive timeout, as the file does today, so a planted hang fails and does not stall the rung. The record plants each bug, shows the test red, removes the plant, and shows it green.

| Arm | Test | Planted bug |
|---|---|---|
| Delay | One decide on `/arm/delay/200/v1` gets the generic answer no sooner than 200 ms and within 1 s. | Parse the value and skip the `after` call. The answer comes at once. |
| Delay, parallel | Eight decides on eight connections to `/arm/delay/200/v1` all finish within 400 ms. | Hold one shared lock across the sleep. The eight finish in about 1.6 s. |
| Delay, refusals | `/arm/delay/abc/v1`, `/arm/delay/v1`, and `/arm/delay/10001/v1` each get status 500 with a body that names the delay. `/arm/delay/10000/v1` is not refused, and the test does not wait for its answer. | Read a bad value as zero. The first two answer with status 200. |
| Round | Four rounds on one backend. In each, a held decide goes out, `wait K` prints K, the reply is still unanswered 300 ms later, `round` goes in, and the answer arrives within 1 s. | Make `round` set the open flag. Round two answers before its `round` line. |
| Round, back to back | Fifty rounds in a row, each `wait K` and then `round`, all end within 5 s. | None certain. This test guards the order rule in decision 2. The review checks the order in the code. |
| Release stays permanent | After `release`, a new held decide answers at once. | Make `release` act as `round`. The new decide holds, and the test fails on its 1 s timeout. |
| Wait, reached | `wait 1` goes in first. A held decide is posted 200 ms later. The line prints 1 within 1 s. | Print the count at once. The line prints 0. |
| Wait, bounded | With no request, `wait 1` prints 0 between 5 s and 6 s. | Drop the bound. Nothing prints, and the 7 s receive timeout fails the test. |
| Wait, bad input | `wait x` prints nothing on standard output, and a `count` that follows prints the count. | Read a bad number as zero. The `wait` line prints the count. |
| One backend per test | The proof runs twice: on two in-process `Backend` values and on two binaries. In each run the two backends have different ports and each holds one decide. `round` on the first lets go only its own reply. After a second decide to the first, the first reads 2 and the second still reads 1. | Keep the gate in a process-wide static in the library. In the in-process run, `round` on one lets both replies go. |
| Cheap start | Twenty backends started one after another each print a distinct port, all within 5 s. | Bind a fixed port. The second start fails. |

Also:
- The existing binary and loopback tests stay green unchanged. They include `a_held_reply_answers_only_after_its_release`, `a_held_reply_waits_for_a_release_line`, and the bind and exit tests.
- `conformance/README.md` adds the delay row to the arm table, the `round` and `wait` lines, and a short section "One backend per test" with the recipe from decision 4.
- `git diff origin/main -- crates` is empty.

## The check it adds to the gate ladder

No new rung and no new script. The root `test` rung already runs `conformance/backend/tests/binary.rs`. `lint` holds the crate to its tables as 0092 set them.

## Budgets and the ratchet

- `conformance/backend/src`: at most 60 nonblank lines added. The delay arm is one match arm and a parse. The gate becomes a small struct. The `wait` line is a loop over `count`.
- `conformance/backend/tests/binary.rs`: at most 220 nonblank lines added for the eleven tests.
- `conformance/README.md`: at most 25 lines added.
- No dependency. The crate keeps `serde` and `serde_json` only.
- The ratchet. `sdlc/ratchet.json` counts `crates` and `conformance`. It rises to the measured total in the commit that adds the code, at most 280. That commit says what grew and why. It names where the builder looked for a duplicate to delete first: the held-reply loops in `binary.rs` and `loopback_arms.rs`, and a shared post helper for the new tests.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen`, or changing an existing arm's answer.

## Exclusions

Retiring the duplicated listeners (above). Any change to `thinkthen`. Any arm or line that no surface ticket names. Any live or paid call.

## Dependencies

After nothing beyond main. Before 0106. 0106 drops its own delay-arm design and depends on this ticket. The other surface tickets may use `round` and `wait` once this lands. Each owner decides whether to rewrite its polls onto `wait`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The raised ceiling and the widened public surface (`Backend::round`) call for the second-agent review the repo requires. The code review names it.

## Complexity

Contract 1; state and timing 3; reach 1; proof 2; cost of error 1; total 8. Final level: 2. The round order rule is the one timing risk.

## Review

- Design review: pending.
- Code review: pending.
