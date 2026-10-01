FINDINGS

# Design review: 0117, the loopback backend arms

Reviewer: a fresh, read-only Claude session that did not write the ticket. Date: 2026-09-24.

Read: the ticket at `8cd17f9d` on `ticket/0117-backend-arms-for-surfaces`. Also read, on main: `AGENTS.md` (the `CLAUDE.md` target), `sdlc/README.md`, `sdlc/ratchet.json`, the shared rules in `sdlc/planning/surfaces-port-guide.md`, `conformance/backend/src/{arms,listener,lib,main}.rs`, `conformance/backend/tests/binary.rs`, the 0092 ticket and build record, the listener issue, `crates/thinkthen/src/cli/edge/deadline_tests.rs`, and the four motivating reviews in `/tmp/claude-1000/`.

Observed by command: 20 starts of 0092's debug binary, all kept alive until every port was read, took 112 ms with 20 distinct ports at a load average near 13 (`target/debug/conformance-backend` in the `thinkthen-0092` worktree). `binary.rs` on main has 131 nonblank lines. Its `line` helper calls `read_line` with no timeout. `sdlc/ratchet.json` counts only `.rs` files.

## What holds

- **Coverage.** Every need in the four reviews has an arm. Ruby R4-2's four rounds use `round`. The SQLite and PostgreSQL held tests get one backend each, and the measured start cost supports that. The 0106 wall-time and R1-24 deadline tests use the delay arm at width 8. Each kept connection has its own thread, so eight delayed replies run in parallel. The 0105 R4-23 single-call test uses `wait 1`, SIGINT, and then `release`. The batch Ctrl-C tests use `wait W`, SIGINT, `release`, and a later `count`. The fallback timers in 0109 finding 3 and 0112 work either as a delay arm or as a test-side `release`. I found no uncovered need.
- **Round is race-free under the order rule.** The reply snapshots the round number before the count moves. So a request that `wait K` counted always holds an older number than the next `round`, and its release is certain. A request that arrives during a `round` either gets the older number and goes, or gets the newer number and holds for the next round. The round number only grows, so no held reply can miss a later round. The move does not change behavior for callers in `crates`: no `answering` closure there blocks, and events already follow the reply.
- **`wait N` is honest.** It prints the real count on both paths, and the test compares that count with N.
- **Loopback and secrecy.** The backend still binds `127.0.0.1:0`. No arm reads or keeps a key or a header. The delay arm adds none.
- **Budgets.** 60 source lines fits a parse, a two-field gate, `Backend::round`, and two `run` lines. 220 test lines is tight but fits, including the reader in finding 1. The ratchet counts only `.rs`, so 60 + 220 = 280 matches.
- **Duplicated listeners.** Deferring them is sensible. Keeping out of `crates` avoids a collision with 0113, 0114, and the engine tickets. No listener needs a new arm. The move needs its own red-green proof.

## Findings

1. **The file does not read standard output with a timeout today, so three plants hang the rung.** Acceptance says each test reads standard output "with a receive timeout, as the file does today". On main, `line` calls a blocking `read_line`. Only the HTTP answers go through `recv_timeout`. Under the "drop the bound" plant, `wait 1` never prints, and the test blocks until the outer timeout. The same happens to each `wait K` in the round tests under their plants. Fix: add a reader thread that sends each output line into a channel. Make `line` take a `recv_timeout`, such as 7 s. Correct the sentence, and count the reader in the 220-line budget.

2. **The `wait x` plant cannot turn its test red as written.** Under the plant, `wait x` prints the count. The `count` that follows prints the same number. The first line read then matches, and the test stays green. Fix: after `wait x` and `count`, close standard input and assert exactly two lines follow: the count and the final count. With the channel from finding 1, you could also assert that no line arrives within 300 ms after the count line.

3. **"Round, back to back" has no plant.** The repo requires red-green proof, and the review cannot stand in for it. The order bug has a deterministic plant. Restore count-before-choose and sleep 20 ms between the count and `reply`. `wait K` then returns before the snapshot. The `round` lands first, and the reply takes the new number and holds. Fix: state that each round waits for its reply with a 1 s timeout, and name this plant. The test then fails on the first round.

4. **The delay refusals need a time bound, exact sentences, and a strict parse.**
   - "`/arm/delay/10000/v1` is not refused, and the test does not wait for its answer" gives no way to see the result. Fix: a refusal answers at once. Assert that no answer arrives within 300 ms. Assert each refusal arrives within 1 s. Then a dropped ceiling (10001 holds 10 s) and an off-by-one ceiling (10000 refused) each turn the test red. Name both plants.
   - "A body that names the delay" breaks the repo rule to pin the exact sentence. Fix: pin each refusal body.
   - Rust's `u64::from_str` accepts `+200`. Fix: define a whole number as ASCII digits only, for both the arm and `wait`. Add `+200` to the refusal cases.

5. **"Cheap start" can flake, and its plant depends on the order of events.** Linux can hand the same ephemeral port to two processes that ran one after the other. If each backend closes before the next starts, "distinct ports" then fails at random. The fixed-port plant can also bind again after an exit, because Rust sets `SO_REUSEADDR`. Fix: keep all twenty alive until every port is read, then close them. The ports are then distinct by construction, and the plant fails on the second bind. My run of that form took 112 ms.

6. **A `wait` blocks the lines after it, and the README must say so.** `run` reads standard input on one thread. While `wait N` runs, a `release` or `round` sent from a test's timer thread waits up to 5 s. Fix: the README recipe says lines are served in order, a `wait` holds the ones behind it, and one writer drives the input.

7. **The parallel delay bound is tight for this machine.** Eight 200 ms replies within 400 ms leaves 200 ms for eight connects and eight debug-build threads at a load of 12 to 14. The plant takes about 1.6 s. Fix: assert within 800 ms. The plant still fails by twice that margin.

8. **Small text fixes.**
   - `spent` sits at line 38 of `src/cli/edge/deadline_tests.rs` on main. The ticket says 40.
   - The in-process half of "one backend per test" must wait for each count to read 1 before `round`. It then needs its own polling loop. Either expose the `wait` loop as `Backend::wait(n)` and have `run` call it, or say that the one test loop is accepted. Decision 3 says the library needs no counterpart, and this test contradicts it.

The writing rules hold. I found no cleft sentence, contrastive appositive, dash gloss, or trailing "which" clause. Each overturnable choice names Ian's lever.

## Confirmation

ACCEPT

Read 0117 at `dbb55129`. All eight findings are fixed in the text.

1. A reader thread feeds a channel, and `line` reads with a 7 s timeout. The ticket corrects its claim about main.
2. The `wait x` test counts the lines that follow the input close, so the zero plant adds a third line and turns the test red.
3. "Round, back to back" waits 1 s for each reply and names the order plant.
4. Refusals have 1 s and 300 ms bounds, both bodies are pinned, a digits-only parse covers `+200`, and four plants cover bad values, `from_str`, a dropped ceiling, and an off-by-one ceiling.
5. All twenty backends stay alive until every port is read.
6. `wait` runs off the input thread, and a plant proves that a `count` sent after a `wait` still answers at once.
7. The parallel bound is 800 ms.
8. The `spent` line now reads 38, and `Backend::wait` removes the extra polling loop.

The new `wait` design is race-free. `Backend::wait` reads the same count that the order rule advances only after the reply's round number is taken, so a `round` sent after `wait K` always releases the reply it counted. The `wait` prefix tells a late line apart from a `count` line. One lock orders every write. The 80, 260, and 340 budgets add up and fit the listed parts. The `.rs`-only ratchet makes 80 + 260 = 340.

Notes for the builder. Neither note blocks this confirmation.
- A `wait` still pending when standard input closes can print its line before or after the final count. The `finish` helper takes the next line as the final count. It should skip lines that start with `wait `, or every test should let each `wait` answer before it closes the input.
- Sharing the output with a detached thread needs `run` to take a writer that is `Send + 'static`, and it rules out `thread::scope`. A scope would join the pending `wait` and break "exits without waiting". The routing line already names this as the new bound on `run`'s output.
- The ticket cites `sdlc/records/0117-design-review.md` for the 112 ms measurement. That record must land with the ticket.
