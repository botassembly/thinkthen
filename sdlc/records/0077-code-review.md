ACCEPT

# Code review: ticket 0077, one process width cap

Reviewer: fresh read-only Claude session (Opus 5.5). Branch `ticket/0077-process-width-cap` at `1f4db620`, code in `a2097e53`, base `65f43693`. I edited, committed, and pushed nothing in the repo. Every run used a scratch clone with every `THINKTHEN_` variable unset. The scratch clone is deleted. `sdlc/scripts/live` never ran.

## 1. One cap across every path

Read: `Client::post_observed` takes a permit at the top of each attempt. That point comes after cache and replay, and before the 0073 attempt-start check, accounting, and `send`. The permit drops right after `send` returns. A retry takes a new permit. Every production `Client::new` (asking, annotate, find, recognize, relate, conformance runner) gets `process_width()`. `check_doors` in `policy.py` keeps `ureq` inside `engine/http.rs` and `PROCESS_WIDTH` inside its accessor. Its plants and controls ran in lint.

`shared_cap_child` runs decide on one text, a retried 503, decide and choose over records, grouped annotate, split annotate, find, recognize, and relate at once on one held-reply listener. It also runs two clients with different timeouts. Its per-arrival assertion `holding <= cap` counts held replies through the listener hook. The test's count can only be at or under the true in-flight count, so it cannot report a false breach.

Red by command (scratch copy, `cargo test --lib width_tests`):
- Gate removed from `post_observed`: `shared_cap_child`, the cancel test, the deadline test, and the retry test failed.
- Per-client gate in production (`client_width` returns a fresh gate): `shared_cap_child` failed on `requests in flight over a cap of 4`.
- Implicit engine selects 4: the state test, the setup-mapping test, and `command_setup_child` failed.
- Conflicting width accepted: the table, the race, the setup-mapping test, and `command_setup_child` failed.

## 2. Selection rule

`Widths::select` matches the ruling. `None` never writes and returns the selected width or 4. The first `Some` selects it and wakes waiters. The same width returns Ok. A different width returns `WidthActive` with the exact ticket sentence. The command maps it to exit 2. `schedule::width` runs before `Folders`, the recorder, key lookup, the client, and input in annotate. It also runs before input dispatch and key lookup in `asking.rs` and `recognize.rs`. `command_setup_child` counts input reads, connections, requests, key lookups, and `requests_sent`, and it pins the full stderr line. Relate refuses `--jobs` and find takes none, so both stay implicit.

## 3. Departure 2: per-client gates in the unit binary, the 150 ms window

The per-client gate exists only in tests. `client_width`'s branch, `WIDTH_CHILD`, and `Client::gated` are all `#[cfg(test)]`. The binary and `tests/backend` never compile them.

I found no flake. The 150 ms window decides only when the test releases an answer. It never decides whether a breach is real. The one timed assertion is the lower bound `most == 4`. It fails only if four answers are never held at once. After the first fill, each release frees a permit to a queued waiter, so the next arrival comes within milliseconds. Stress: 25 of 25 passed on two cores shared with four busy loops. 15 of 15 passed on one core shared with eight busy loops. The 0116 flake came from `Listener::peak` over-counting by one. This test avoids that counter.

What the window does weaken: a missing gate is caught only when the fifth request arrives within 150 ms of the fourth. That case would be a false pass, not a false failure. The planted bugs all went red. This also departs from the ticket text "timing does not choose the result". The build record discloses the departure.

Deterministic replacement (optional follow-up, not blocking). Release on counts, the way 0116 did. Hold until `holding == min(cap, expected_total - released)`, with the expected total of held requests fixed per path. Keep the outer 120 s timeout only for deadlock. In the two-engine phase, give each `send_once` a `Cancel::observed` channel. Wait for four "blocked at the gate" signals before the first release. That proves the next four wait at the gate and did not just arrive late.

Minor note: in the unit binary, `schedule::width` still writes the real `PROCESS_WIDTH`, and clients get private gates. No in-crate test passes `--jobs` today. A future in-crate test that runs a command with an explicit `--jobs` would conflict with another test's width. It would need to run in a child copy too.

## 4. Departure 1: two test files outside `opens`

Acceptable. `engine/mod.rs` is 328 lines, and `cli/schedule.rs` is 228. Neither could take about 400 test lines under the 500-line file limit. The ticket requires in-crate proofs of the command setup, so `tests/backend` could not hold them. All five production files are inside `opens`.

## 5. The default-width issue

0077 leaves no new defect. It changes neither command behavior nor the fallback. The issue's "runs past the documented limit" is a measured rate: 1,267 to 1,319 requests a minute at width 4 against a documented 1,200, with no refusals. That is a pre-existing default-policy question plus a documentation gap. The `--jobs` reference pages carry no measured numbers. 0077 makes a later change one line (`Width::FALLBACK`). Owner: Claude, as this repo's queue owner, through a Quick Fix ticket. It decides and records keep-4-or-3 under the configuration rule and adds the numbers to `specification/records.md` and `site/src/pages/reference.astro`. Ian can overturn the fallback width, as the ticket already says.

## 6. Size

`ratchet.mjs` in lint printed `47955/47955`. `git diff -U0 65f43693 a2097e53` over production Rust counts 183 nonblank lines added and 7 removed, against a limit of 550. It touches 5 files against a limit of 12. The two test files hold 405 and 402 nonblank lines, 807 against a limit of 1,000. The record's numbers are correct.

## 7. Merging

0116 landed on origin/main (`7e8301c4`, ratchet 47007). A trial merge of `1f4db620` with origin/main conflicts only in `sdlc/ratchet.json`, and the measured total becomes 47990. Adding 0115 (`origin/ticket/0115-runner-guards`, ratchet 47121) also conflicts only in `sdlc/ratchet.json`, and the total becomes 48140. On the merged tree (main plus 0115), `policy.py` passed, `cargo test --lib` passed 314 with 0 failed, and `--test backend scheduling` passed 14. That includes 0116's deterministic global-queue test under the shared gate. Set the ceiling to the measured total at land time.

## 8. Full ladder at `1f4db620`

The one-minute load was 7.5 at the start. install exit 0. lint exit 0 (policy with `check_doors`, ratchet 47955/47955, format, Clippy, docs). test exit 0: 780 passed, 0 failed, including all 15 width tests and the global-queue test. spec exit 0: 21 demos green, 0 red.
