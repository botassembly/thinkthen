# 0085 code review: the real engine facade

Reviewer: a fresh Claude session, read-only, new to this work. Branch `origin/ticket/0085-real-engine-facade` at `8d9a6dfd`. Last code commit `9a9ddfe8`. Diff base `46f34c23` (the merge base; it has no Rust change against `86f4012e`, the record's base).

Verdict: FINDINGS. Two must-fix items (F1, F2), both small tests. The rest are cleanups under the new test rule and record fixes. The file budget can be lifted.

## What I ran

All runs used a scratch export of `8d9a6dfd` under `/tmp/claude-1000`, a separate target folder, and `THINKTHEN_API_KEY` unset. No test reached a non-loopback address.

- `cargo test --workspace --all-targets --all-features`: 807 passed, 0 failed.
- `ratchet.mjs`: 49701/49701. `policy.py`: passed.
- Load: 20 busy loops on 16 cores. Load average 15 to 22. The facade, width, scheduler, grouped-scheduler, and interrupt tests ran 30 times (38 tests each). The whole library suite ran 8 times. Zero failures.

## 1. Correctness and unchanged command behavior

The command's behavior holds. I read every command diff for output, exit codes, and requests sent.

- Each function keeps its request bytes, digests, and send order. The facade calls the same `ask_profile` and `ask_prepared` owners. `recognize` and `relate` execution moved into `engine/facade/{recognize,relate}.rs` line for line, with `Failure` swapped for new engine error variants.
- The seven new `Error` variants map back to the same `Failure` values in `failure/convert.rs`. Messages and exit codes do not change. `NoKey`, `WidthActive`, and `RecognizeKinds` fall under the `usage` kind. `ModelsDiffer`, `UsageOverflow`, `RecognizeLogical`, and `RelateLogical` fall under `backend`. The six kinds stay six.
- Width order holds. `jobs_of` and `width` still register `--jobs` before input is read. `Engine::new` selects the same width again, and the width state accepts an equal width.
- The annotate `streams` flag and the recording flag compute the same values as before.
- `Engine::new` opens no folder and connects nowhere. `Recorder::of_private` only stats the path.
- The single-send thread change passes the existing command SIGINT tests for decide, find, relate, and recognize.
- Nothing reaches a paid backend. The shared-case runner uses the default vendor address with replay only and a key function that fails. A replay miss is a local error. The fault runner builds an engine whose closure never sends.

Gaps against the ticket:

- **F1 (must fix). The facade's own width registration is untested.** Plant P2 (`Engine::new` calls `select(None)` and ignores `settings.width`) stays green across the whole workspace. The command registers `--jobs` first, so this line is dead for the command. It is the only width door ticket 0086 will have. Add one test to the existing width child process (`cli/schedule/width_tests.rs`, `in_child`). In that process, an explicit width registers, and a conflicting explicit width fails with the `usage` kind. The failure must come before any count, file, or send. The child process is the real process boundary, so the "it changes every later test" concern in the record does not apply.
- **F2 (must fix). The relate rule "no usable answer fails the call" is untested.** Plant P5 removes the `RelateLogical` check. It stays green across the whole workspace, spec pages included. The gap predates this ticket. This ticket moved the rule into the facade, and its partial-result bullet names `relate`. Add one loopback test where every logical relation answer fails and the call returns `RelateLogical`. In the same test, a partly failed run keeps its good edges and its failed count.
- F3 (record). The shared-case runner keeps only `judged.answered` and recomputes values itself. Plant P4 (`judge` drops the threshold) stays green in the library suite. Eight command tests and one demo catch it. Coverage exists, but the record's claim that the runner compares bare values at the typed seam overstates it. Change the claim, or have the runner read `judged.value`.
- F4 (record, for 0086). The facade has typed entries for `judge`, `find`, `recognize`, and `relate`. `annotate`, `filter`, and `rank` go through generic `split`/`ask_chunks`/`groups`/`records` with command closures. Annotate's per-record assembly (model check, usage sum, digests, failed members) still lives in `cli/annotate/aggregation.rs`. That matches "orchestration only", but 0086's public `annotate` needs that assembly. State in the record that 0086 must move it below the facade and must not copy it.
- Minor: `cli/annotate.rs` `answer_group` uses `places.next().unwrap_or_default()`. A chunk-count mismatch would then be silently empty. Return a `Defect` instead.

## 2. The 29 production files

My count matches: 29 production files, 1,166 added and 929 deleted nonblank lines.

| Class | Files |
| --- | --- |
| Real logic (8) | `engine/facade.rs` (new), `engine/workers.rs` (`on_worker`), `engine/request.rs` (`ask_sent`), `engine/error.rs` (variants, `retryable`), `engine/usage.rs` (`snapshot`), `cli/asking.rs` (engine builder, reroute), `cli/annotate.rs` (`Judging::new`, split places), `cli/edge.rs` (key error type, shared counters) |
| Moved, not new (2) | `engine/facade/recognize.rs`, `engine/facade/relate.rs` |
| Whole-file deletions (3) | `cli/asking/request.rs`, `cli/recognize/relation.rs`, `cli/relate/plan.rs` |
| Deletion plus call site (4) | `cli/recognize.rs` (-272/+21), `cli/relate.rs` (-105/+13), `cli/relate/result.rs`, `engine/schedule.rs` |
| Call site or import only (12) | `cli/find.rs`, `cli/annotate/aggregation.rs`, `cli/annotate/plan.rs`, `cli/annotate_schedule.rs`, `cli/recognize/dry_run.rs`, `cli/relate/dry_run.rs`, `cli/schedule.rs`, `cli/failure/convert.rs`, `engine/http.rs`, `engine/mod.rs`, `core/mod.rs`, `lib.rs` |

New logic sits in 8 files, or 10 counting the two moved files. That is under 16. The other 19 files are deletions or call sites that routing all ten functions requires. A split would only move call-site edits into a second ticket and leave a half-routed command in between. **Verdict: lift the file budget to 29 and record it in the 0085 record. No split.** Ian can overturn this.

## 3. The R5-19 substitution

The substitution is sound. The ticket's claim was wrong about which test turns red.

- The facade test runs at width 1. The scheduler dispatches an item only while `dispatched - next < jobs`. No second item waits in the queue when the cancel fires. The ticket's plant has nothing to skip there, so the facade test stays green.
- In production each item asks under the batch token. The gate wait and the pre-attempt check would still stop a queued item, so the plant cannot cause a send.
- Plant P7 (the ticket's plant: the worker skips `cancel.stop()`) does turn red, in the scheduler's own test `engine::schedule::tests::cancellation_with_work_in_flight_ignores_later_input_and_joins`. Update the record: the ticket's plant is caught by that owner test. The fresh-token plant proves the facade passes the call's token to the scheduler.
- The facade test's `rows.len() <= 1` is loose. It can be `rows.len() == 1`, because the held first item completes before the stop.

## 4. The two test-only hooks

- **`workers::SENDS` can move to the real boundary.** I removed the thread-ID assertion and planted "send on the calling thread". The signal half of the test alone turned red in 5 of 5 runs with `the answer: Transport(Other)`. A socket read with a timeout is not restarted after a signal. The current test releases every reply before `find`, so only `decide` is under the signal. To cover `find` without the hook, hold its reply too: `wait(1)`, signal, `round()`, `wait(2)`, signal, `release()`. Then delete `SENDS`. The ticket's bullet asks for this seam, but it predates the new rule. The owner should record that the rule replaces the seam. Ian can overturn that.
- **`Engine::gated` can go.** Two tests use it:
  - `mixed_concurrent_calls_never_pass_the_one_width` tests one contract at a second layer. The existing child-process test `every_live_path_and_every_engine_share_one_cap` already runs all nine command paths at once under the real process cap. Plant P6 (the gate lets one extra through) turns both red, plus four `engine::width_tests`. Delete the facade copy.
  - The R5-19 test needs width 1. Move it into the same width child process with `Settings { width: Some(1) }`.

  With F1's test, that child process becomes the one home for facade width.
- The same rule applies to `a_refused_connection_fails_at_once_and_is_not_retryable`. Plant P9 (`Refused` counts as retryable) turns it red, and also `http::tests::no_transport_failure_is_sent_again`, `http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry`, and command test `exchange::a_refused_port_fails_before_the_first_default_retry_wait`. Name those as the R2-21 proof and drop the facade copy. Keep the G2 test. It proves each of six facade paths separately, which no owner test does.
- `only_the_one_accessor_reads_the_retained_state` scans source text. It misses destructuring (`let Self { state, .. } = self`), which `gated` itself uses. Once `gated` goes, this matters less. A `policy.py` rule would be the natural home. Not blocking.

## 5. Worker joining and conflicting width

- Worker joining needs no new test. `on_worker` uses `thread::scope`, so the language joins the thread and resumes a panic. `records` and `groups` call the schedulers, whose lifetime tests (`scoped_observed` with a lifetime guard) already prove the join. A test here would pin the language.
- Conflicting width needs a test before landing. See F1: the plant survives everything today.

## 6. My planted bugs

Each plant ran alone on a clean copy and was reverted.

| Plant | Result |
| --- | --- |
| P1 send on the calling thread, `SENDS` assertion removed | red, 5 of 5 (signal alone) |
| P2 `Engine::new` ignores the explicit width | **green, whole workspace** (F1) |
| P3 `ask_chunks` uses a fresh cancel token | red: `local_refusals_send_nothing_and_store_nothing`, `deadline_tests::a_spent_deadline_on_every_direct_path_sends_nothing` |
| P4 `judge` drops the threshold | red: 8 command tests and 1 demo; library green (F3) |
| P5 `relate` drops the no-usable-answer check | **green, whole workspace** (F2) |
| P6 the gate lets one extra attempt through | red: facade width, child width, 4 width tests |
| P7 the ticket's R5-19 plant | red: `schedule::tests::cancellation_with_work_in_flight_ignores_later_input_and_joins` |
| P9 `Refused` is retryable | red: 3 existing tests plus the facade test |

Timing: every phase in the new tests waits on a channel, a barrier, a held reply, or `wait(n)` with a 5-second bound. No test sleeps to set an order. Under load 15 to 22, 38 tests ran 30 times and the whole library suite ran 8 times with no failure.

## 7. The raised ceiling and duplication

- The deletions are real. `asking/request.rs`, `recognize/relation.rs` and its test, and `relate/plan.rs` are gone. `Aggregate`, `TokenInput`, `Method`, `PreparedRelation`, `Logical`, and `Execution` each have one definition now. The test-only `request::ask_with` and `schedule::run` are gone. Production grew by 237 net lines. Most of the 900-line ratchet rise is tests: +663 net.
- Duplication remains. Three copies accumulate request metadata (model check, usage sum, replayed flag, send count, digests): `facade/recognize.rs` `Aggregate::add_answered`/`add`, `facade/relate.rs` `add_meta`, and `cli/annotate/aggregation.rs`. Their rules differ. Annotate's usage becomes absent if any reply lacks usage. The other two keep the known part. Relate reports a model mismatch without names, and recognize names both models. All three copies predate this ticket. The record says where duplication was removed but does not name these copies as kept. Name them as deliberately kept and file one issue to fold them behind one owner with the three rules made explicit. Do not change behavior in this ticket.
- Carrying out section 4 (delete the width duplicate, the R2-21 duplicate, and both hooks) would lower the ratchet by roughly 80 to 120 lines.

## Required before landing

1. F1: add a conflicting-width test in the width child process.
2. F2: add a relate test for all answers failed and for a partial failure.
3. Record fixes: lift the file budget; correct the R5-19 plant note (P7 is red in the scheduler test); correct the runner's bare-value claim (F3); name the three kept metadata copies; add the 0086 note for annotate assembly (F4).

Recommended in the same pass: section 4 (move both hooks and delete the two duplicate tests), `rows.len() == 1`, and a `Defect` in place of `unwrap_or_default`.

## Re-review at 8b8035ce

Verdict: ACCEPT.

I re-read the code diff from `8d9a6dfd` to `8b8035ce` and exported `8b8035ce` to a scratch copy. `cargo test --workspace --all-targets --all-features` ran with the key unset: 806 passed, 0 failed. This covers `8b8035ce`, which the ladder did not run in full. The count is one lower than before. The review removed three tests and added two.

### Each finding is fixed

- F1 is fixed. `cli/schedule/width_tests/facade_tests.rs` runs in the width child process. In that process, `Engine::new` registers width 1 and refuses width 2 with `WidthActive`, under the `usage` kind. The process width stays 1, and nothing is sent.
- F2 is fixed. `relate_fails_when_no_answer_is_usable_and_keeps_a_partial_result` covers all answers failed and a partial failure. The failure check it was written for is now deleted (see below).
- F3 and F4 and the kept copies are fixed. The record now says the runner does not compare bare values. It names the three metadata copies as kept on purpose. It tells 0086 to move annotate, filter, and rank assembly below the facade.
- The R5-19 note now cites the scheduler test for the ticket's plant. `rows == [Some(0.9)]` replaces `rows.len() <= 1`.
- Both hooks are gone. `workers::SENDS` and `Engine::gated` are removed. The signal test holds both the judgment and the find, and sends signals during each. The width test copy and the refused-connection test copy are deleted. The R5-19 test now runs in the child process.
- `unwrap_or_default` is now a `Defect`.
- The file budget is lifted to 29 in the ticket and in the record.

### Plants, each on a clean copy across the whole workspace

| Plant | Result |
| --- | --- |
| P1 send on the calling thread | red: `a_host_signal_on_the_calling_thread_never_fails_a_single_send` |
| P2 `Engine::new` ignores the explicit width | red: the facade width child test (was green) |
| P3 `ask_chunks` uses a fresh cancel token | red: `local_refusals_...`, `a_spent_deadline_on_every_direct_path_...`, `sigint_between_recognition_chunks_...` |
| P4 `judge` drops the threshold | red: 8 command tests |
| P6 the gate lets one extra through | red: 4 width tests and the shared-cap child test |
| P7 the ticket's R5-19 plant | red: `schedule::tests::cancellation_with_work_in_flight_...` |
| P8 `records` uses a fresh token | red: the facade width child test, `a_spent_deadline_over_records_...` |
| P9 `Refused` is retryable | red: 2 transport tests and 1 command test |
| P10 relate stops counting a failed answer | red: the new relate test and 4 command relate tests |

P5 no longer applies, because its check is gone. Load: 20 busy loops, load average 22 to 23. The facade, width, scheduler, and interrupt tests ran 30 times (37 tests each) with 0 failures.

### The deleted check was safe to delete

- The check cannot be reached. `decode_response` in `core/adapters/systemone/response.rs` returns an error when every answer in a reply failed. That decoder builds every production `Reply`: `Reply::new` has no other production caller. Replay and live replies go through `decode` or `decode_observed`, which both call it. So every reply `relate` sees holds at least one answered question. `failed > 0` implies one reply exists, which implies `answered >= 1`. `answered == 0 && failed > 0` cannot happen.
- Nothing else reaches the error. After the commit, `RelateLogical` and `relate::Error::Logical` appear nowhere in code. Only the record and the earlier review text name them.
- The output and exit codes do not change. An all-failed relate reply was already a decode `Reply` error on `main`, so it exits 4 with the decode message, before and after. The deleted message, "the backend returned no usable relation answer", could never print. No spec page, demo, or test names it. The spec's rules for a partial failure (exit 6) and a reply with no usable answer (exit 4) still hold. Plant P10 shows the command's relate partial-failure tests stay live.

### Not blocking

- The ticket's "Sending thread" bullet still asks for a `cfg(test)` thread-ID seam. The record explains the removal under the new test rule. Add one clause to the ticket bullet saying the rule replaced the seam, so the two texts agree.
