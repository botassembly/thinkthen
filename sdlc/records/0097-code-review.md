# 0097 code review: run the host interrupt check

Reviewer: fresh Claude session, read-only. Target: `origin/ticket/0097-interrupt-check` at `4e81f4be`, code at `334fc076`, base main `836e3f23` with 0085.

Verdict: **findings**. The design is sound and the tests are good. One real bug in the send count needs a one-line fix and a test row. The 0086 hand-off duties live only in the 0097 record, and 0086's builder will not see them there.

## What I ran

All runs used scratch copies under `/tmp/claude-1000/`. Each copy had its own target folder, and the key and base-address variables were unset. Every listener binds `127.0.0.1:0` (checked in `conformance/backend/src/listener.rs` and the test file). Nothing reached a paid backend.

- The 3 interrupt tests passed, and the 82 `engine::` tests passed.
- `ratchet.mjs` reads 50059/50059. Production adds 108 nonblank lines in 6 files. Tests add 245. Both match the record.
- Load: 24 busy loops on 16 cores held the one-minute load between 19.6 and 25.8. The interrupt tests went green 20 times out of 20. `engine::` passed with 82 tests and `cli::` with 54. The builder's panic plant went red 10 times out of 10 under that load. My plant B went red 5 times out of 5.

## Findings

### F1 (medium): the send count is shared by every call on one token, so one call's sends silence another call's check

`Cancel::sends` is an `Arc` that `with_check` copies through `..self.clone()`. Every clone of one base token shares it. The command already shares one long-lived token across its calls (`Environment.cancel`). 0086 may map one host `CancelToken` onto one private token. Then a call waiting at the width gate reads `sends > 0` from the other calls' sends in `poll_between_sends`, and its check never runs.

Probe: four holders ask on `base`, and the held call asks on `base.with_check(&check)`. Result: "check ran 0 times during a held width gate on a shared token". Adding `sends: Arc::default(),` to `with_check` turns the probe green, and the three landed tests stay green.

Fix: give each call its own count in `with_check`. Add a width-gate row whose holders share the base token. The count belongs to the call, like the deadline and the check. The shared stop flag stays shared.

### F2 (medium): 0086's duties are recorded only in the 0097 record

`sdlc/tickets/0086-expose-the-public-rust-api.md` on `origin/ticket/0086-public-rust-api` (`bed6a4e6`) does not mention any of these duties:

1. `Batch::next` must reach `stop_or_remaining` on the calling thread each tick. `Batch` is neither `Send` nor `Sync` under 0084, so the thread ID holds as long as the batch is built on its calling thread.
2. The public door must call `with_check` at call entry on the calling thread. The thread ID is taken there, and a check attached anywhere else never runs.
3. 0086 line 47 requires that "no engine panic crosses a public engine method", and 0086 turns such a panic into `Defect`. 0097 resumes the host's panic with its payload unchanged, and it leaves no marker behind. 0086's `catch_unwind` cannot tell that panic from an engine defect. One answer: the public wrapper catches the host's panic around the host closure, stores the payload, returns `true`, and resumes the payload after the call joins. 0086 must pick one answer.
4. A `true` check fires the shared flag. When the host passes its own `CancelToken`, `is_cancelled()` then reads `true`, and every sibling call on that token stops too. The ticket's words ("fires the call's cancel token") allow this. 0086 should state it in the public docs or give each call its own flag.
5. When 0086's public tests land, the facade-level rows either move to `CallOptions::interrupt` or get deleted. Otherwise 0086's panic test and 0097's panic test test one contract at two layers.

Fix: add these five items to the 0086 ticket on its branch, or file them in `sdlc/issues/` where 0086's ticket links them. Line 25 of the 0086 ticket also says the check "delegates to 0085's private check". It should say 0097's.

### F3 (low to medium): the annotate scheduler needs a row now

My plant A kept deadline and fire handling in the annotate scheduler's calling-thread poll (`annotate_schedule.rs`, the `cancel.stop().filter(..)` in the loop) but dropped the host check. All 330 library tests stayed green. `groups` has its own poll site in its own module, so the record-scheduler row does not guard it. A table row copied from the bulk row, over `engine.groups`, costs about 25 test lines. That keeps the tests within the 400-line budget. Plant D, the same change in `schedule.rs`, went red on the bulk row, so the pattern works.

### F4 (low): `hold()` depends on the test-only `Cancel::fire`

`hold()` calls `base.fire()` (`#[cfg(test)]`) to free a call that the check failed to stop. The panic test already bounds its wait with `Deadline::after(BOUND)`. `hold()` can do the same and drop the test-only hook.

## Answers to the seven questions

1. **Correctness.** The code matches the ticket, the 0084 contract (the check is `&'a (dyn Fn() -> bool + Sync)`, and `Batch` is not `Send`), and the 0073 ruling. A `true` check fires the flag. The worker stops at its next poll or at its attempt-start check. Sent attempts finish. The call returns `Cancelled`, and a bulk run returns `Stopped` with its cause. The panic path fires the flag before `resume_unwind`, and `thread::scope` joins every worker. I probed a panicking check during a bulk run: it resumed the same payload, and the listener stayed at four requests. One small window remains. `poll_between_sends` can read a count of zero just as a worker passes its attempt-start check, so the check may overlap the start of a send. 0073 puts the start of an attempt at that check, so this is within the ruling and needs no change. F1 is the only correctness bug.
2. **Riding on the stop token.** This is the simplest sound choice. The one poll function is a method on the token, and the token already carries the per-call deadline. The contract's borrowed check forces a lifetime somewhere. A `'static` boxed check would change the frozen signature. A separate options argument would touch every poll site. The cost is `Cancel<'static>` in two command files and two test helpers, which is small. F1 shows the one place where per-call state was left shared.
3. **Test-only hooks.** "width gate" appears only as an assertion label in the test. No production message carries it, and no plant depends on one. The real hooks are these: `with_check` has no production caller until 0086 (`#[allow(dead_code)]`); `Cancel::fire` is `#[cfg(test)]` (F4); and the older per-client width gate in `client_width` is `#[cfg(test)]`. The per-client gate only isolates tests from each other, and the behavior under test is the same with the process gate. `with_check` is the real private boundary until 0086 exists. That is acceptable as staging if F2 item 5 is recorded.
4. **Annotate row.** Yes, add it now (F3).
5. **The four questions.** The four-wait table is an edge-case table. It fails on plants B, C, and D and on the builder's worker-thread and run-once plants. No older test covers the check. It passes. The single-send test is a contract check. The dropped-count plant fails it, and its sleep can only fail toward green under load, never red. It passes. The panic test is a contract check. It fails on plant P and on the swallow plant. It passes now, and it becomes a duplicate once 0086's public panic test lands (F2 item 5). None of the three is scaffolding.
6. **My plants,** each alone on a fresh copy with its own target:
   - A: the annotate calling thread skips the check. **Green, all 330 tests** (F3).
   - B: `on_worker` joins without polling. Red: "width gate: the check ran at each poll", and the panic test never panicked.
   - C: a `true` check does not fire the flag. Red: the width row returned `None` where `Some(Cancelled)` was expected.
   - D: the record scheduler's calling thread skips the check. Red on the bulk row's "ran at each poll".
   - P (the builder's plant, re-run under load): the panic path does not fire the flag. Red, "5 sends where 4 were expected", 10 times out of 10.

   The probe for F1 is also red on `334fc076`.
7. **The `Batch::next` duty.** It is not recorded where 0086's builder will see it (F2).

Ian can overturn F2 item 4 (the shared-flag behavior) and F3. F1 is a bug fix.

## Re-review at d8666c50

Target: `origin/ticket/0097-interrupt-check` at `d8666c50`. The fix is `54ad43a2`, and `381a1ab4` merges main `badb9c57` (qf-answer-labels). The 0086 note is `fa6df53e` on `origin/ticket/0086-public-rust-api`. Verdict: **ACCEPT**.

- F1: `with_check` now sets `sends: Arc::default()`. The new row "width gate on a shared token" has holders that ask on clones of the call's base token. With the fix line removed, it fails with "width gate on a shared token: the check ran at each poll".
- F2: `fa6df53e` adds all five duties to `sdlc/tickets/0086-expose-the-public-rust-api.md` under "Builder note from 0097". It also corrects "0085's private check" to "0097's". Nothing else in that ticket changed.
- F3: a `groups` row runs over `engine.groups` beside the `records` row. With my annotate plant, the row fails with "groups: the check ran at each poll".
- F4: `hold()` takes a `base` token. Every caller passes `bounded()`, which is `Cancel::default().with_deadline(Deadline::after(BOUND))` with `BOUND` set to 3 s. The call to `fire()` is gone.
- Nothing else changed. `54ad43a2` touches only `mod.rs` (one line), `facade_tests.rs` (`feed` now takes a send closure, so both schedulers can use it), `interrupt_tests.rs`, the ratchet, and the 0097 record. The diff from main `badb9c57` to `381a1ab4` covers only the 0097 files. `d8666c50` changes only the record. `ratchet.mjs` reads 50276/50276.
- Load: 24 busy loops held the one-minute load between 16.3 and 21.0. The interrupt tests went green 12 times out of 12. The F1 plant went red 5 times out of 5, and so did the annotate plant. Every copy used its own target folder, with the key and base-address variables unset. All listeners are loopback.
