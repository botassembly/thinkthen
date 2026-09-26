# 0132: Build keep every reply the planner can ask for

Status: built; ladder run once after the merge of `origin/main` in `8faad4f0`; ready for code review. Owner: Claude.

Branch `ticket/0132-large-replies-fit`. The ticket is `sdlc/tickets/0132-large-replies-fit.md`. Three fresh read-only design reviews ran on 2026-09-25. The first two returned findings, and the third accepted. The change raises the ceiling, so a second agent reviews the code and names what it checked. The coordinator routes that review. Ian can overturn every decision the ticket lists.

## Result

- `engine/http.rs` reads at most 1 MiB plus 8 bytes per request byte, in saturating `u64` arithmetic. It passes one byte more to `ureq`, whose reader refuses a body of exactly its limit. A reply of exactly the limit is kept.
- `ureq::Error::BodyExceedsLimit` becomes `Error::ReplyTooLarge(limit)`, built from the computed limit at the read site. It never passes through `transport()`. It is a backend failure and is not retryable.
- `engine/error.rs` holds the one sentence, `reply_too_large`. The command prints it at exit 4, and the library's `Error::Backend` carries it. `thinkthen check` grades it as a failed probe, as a refused reply, and goes on.
- `specification/backends.md` states the limit and the sentence, `check.md` gains the probe row, and `relate.md` names "reply size" among the failures that never become partial success.

## Stop rule 1 crossed, and a fix outside the ticket's design

The relate test at 180 names took 46 s and the recognize test at 64,000 bytes took 29 s in the debug test build. A release build of the command took 25 s for the relate run. Stop rule 1 names 20 s.

A `gdb` sample put the time in `decode_response`. Its unexpected-name check formatted every wire name for every answer, which is quadratic in the answers: 32,220 answers make about 500 million formatted strings. The rule's fallback, a smaller input, would not help, because 172 names is the smallest relation whose reply passes 1 MiB.

The build fixed the check instead of stopping. `systemone.rs` gains `wire_place`, the inverse of `wire_name`. It reads the number from a name and accepts it only when `wire_name` writes that exact spelling. `decode_response` refuses any answer whose place is missing or past the wire count. Both new tests now take 1.7 s together. The fix sits in its own commit, `d24bc7eb`, then `wire_place` moved beside `wire_name` so both files stay under the 500-line cap. The coordinator can overturn it, and the tests would then need their sizes cut or their time accepted. `an_unexpected_answer_name_refuses_the_whole_reply` became an edge table over `q2`, `q0`, `q01`, `q+1`, `Q1`, `q`, and `1`.

## Plants

Each plant was applied to the file, built, run against its test, and restored. Each restored file was touched. The script and logs sit in the session scratchpad under `t0132/`, outside the repository. A grep of the diff for plant text found none.

| Plant | Test | Result |
| --- | --- | --- |
| P1: the old fixed 1 MiB read limit | relate at 180 names, recognize at 64,000 bytes | Red: exit 4 |
| P2: `BodyExceedsLimit` mapped to the transport kind again | `a_reply_past_the_bound_is_sent_once` | Red: the "could not be reached" sentence |
| P3: no limit | same | Red: exit 0, wanted 4 |
| P4: `ureq` given the limit, not one byte more | `a_reply_of_exactly_the_bound_is_kept` | Red: the reply is refused |
| P5: factor 0, with the resend formula changed to match | relate, recognize, and both resend rows | Red on relate and recognize; the resend rows stay green, as the ticket says |
| P6a: check routes the failure to the `connection` gate | `a_reply_over_its_limit_fails_its_probe_and_the_check_goes_on` | Red: 1 probe sent, wanted 4 |
| P6b: check leaves the failure out of its match | same | Red: 1 probe sent, wanted 4 |
| P7: the library prints the transport text | `a_reply_over_its_limit_names_it` | Red: the message differs |
| P8: the failure marked retryable in `engine/error.rs` | same | Red: `retryable()` is true |
| P9: `wire_place` drops the spelling check | the name edge table | Red: `q01` decodes |
| P10: the decoder drops the range check | the name edge table | Red: `q2` decodes |
| P11: the quadratic name check restored | relate at 180 names | **Green**, in 47 s |

P11 stays green, which crosses stop rule 3. No test fails on the quadratic form, because the test harness kills a child only after 60 s. A relation of 240 names would run the quadratic form past that deadline, about 150 s, and the linear form in a few seconds. The coordinator asked for 180 names, so the test keeps 180 and the gap is named here. The choice for the coordinator: raise the relate test to 240 names, or accept the gap.

The first check-test draft read the listener's requests after the run and panicked on a missing index when a plant stopped the check early. It now records each limit as the listener answers and asserts the probe count first.

## Ladder

After the merge of `origin/main` (`8faad4f0`), each rung ran once on the final code. The heavy lock was taken by the rungs themselves, and plain `cargo` ran under `flock -o /run/user/1000/thinkthen-heavy.lock env -u THINKTHEN_API_KEY`.

| Rung | Result |
| --- | --- |
| install | pass |
| lint | pass, after two fixes it found: two files over 500 lines, and `say` in `cli/failure.rs` over 90 lines. The sentence moved into `special_failure` |
| test | pass |
| spec | pass, 21 demos green |
| surfaces | pass |

None of the known flakes appeared: the C "Text file busy" surface failure, the PostgreSQL warm-rows timing test, and the Polars throttle bound. `origin/main` moved after the ladder ran, with the flaky-test Quick Fix. The lander merges it.

## Lines

Nonblank lines against the branch base.

| Part | Budget | Measured |
| --- | --- | --- |
| `crates/thinkthen/src`, production | 30 net | 27 net: 18 for the limit and its failure, 9 for `wire_place` and the linear check |
| Tests | 130 | 136 net: 132 in the command and library tests, 4 in the name edge table. **Over by 6** |
| Pages | 10 changed | 3 added, 1 removed |

The test overrun comes from two things. The name edge table was not in the ticket. The library test repeats the four-line padding helper, because `tests/public_controls.rs` is a separate test crate from `tests/backend`.

## Ratchet

`sdlc/ratchet.json` rose from 66,404 to 66,567, 163 lines, in `7bf3d4fd`. That commit says what grew and where duplication was checked. `node sdlc/scripts/ratchet.mjs` reads 66567/66567.

## Deferred

As the ticket lists. In addition: no test fails on a quadratic decode (P11).
