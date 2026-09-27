---
flow: build
priority: 169
opens: crates/thinkthen/src/cli/interrupt.rs crates/thinkthen/src/cli/interrupt crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/cli/mod.rs crates/thinkthen/tests/backend/interrupt.rs specification/channels.md specification/records.md specification/check.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md CHANGELOG.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0169: A signal stops the command plainly

Status: built, awaiting fresh read-only code review. The coordinator accepted it on 2026-09-27 after a fresh read-only design review, with the fixes that review named. Owner: Codex in the retained command lane.

Review route: a fresh read-only Codex reviewer checks the final diff. Ian's handover routes the accepted Claude ticket to Codex. The accepted behavior stays fixed.

## Outcome and authority

Three things start to hold for the command.

1. A run stopped by Ctrl-C never blames the backend. A request that times out after the signal is reported as the signal's stop.
2. The stop line after a signal names no record. Today it can name a record that never arrived.
3. SIGTERM stops a run as SIGINT does. The run finishes its sent requests, writes its finished output, prints the stop line and ends by SIGTERM. `channels.md` states both signals, the second-signal escape and the exit statuses.

This is the command half of Batch B in `sdlc/planning/work-plan-2026-09-27.md`. Ticket 0168 carries the binding half. The two share no file, no helper and no test. This half edits the command's stop code beside ticket 0162, so it builds after 0162. The coordinator assigned Batch B on 2026-09-27. Ian can overturn each decision below.

## Prior experiment evidence

- Local experiment 284, file 46: one Ctrl-C waits out a hung request and then blames the backend, and the second-press escape is undocumented. Its sources are local experiment 273, report 01 issue 10 and report 11 issue 8.
- Local experiment 284, file 56: after Ctrl-C the stop line names a record that never arrived. Its source is report 01 issue 9.
- Local experiment 284, file 116: SIGTERM is unspecified and ends a run abruptly. The carrier hard-codes SIGINT. Its source is report 11 issue 3.
- This ticket's author ran the debug binary of `origin/main` `18f0381e` in a scratch folder against loopback listeners only, with a placeholder key variable and no network:
  - A listener that accepts and never answers. `decide --lines --timeout 4 --max-retries 0` over two lines, SIGINT at 1 s. The run ended by SIGINT after 4.0 s. Standard error read `thinkthen: the backend timed out; increase --timeout or try again`, then `thinkthen: stopped at record 1; 0 records finished`. The same held at `--jobs 1`.
  - The same run with a second SIGINT 0.5 s after the first. It ended by SIGINT at 1.5 s with empty standard error.
  - The same run with SIGTERM at 1 s. It ended by SIGTERM at 1.0 s with empty standard error and no stop line.
  - The conformance backend's generic arm. `decide --lines --jobs 1` with standard input a pipe left open after two lines, SIGINT after both rows printed. Two rows, then `thinkthen: stopped at record 3; 2 records finished`. Record 3 never arrived.

## What happens today

Read from `origin/main` `18f0381e`.

### The signal carrier

- `cli/interrupt.rs:275-279`, `sigint_set`, builds a set of SIGINT alone. `UnixRouting::start` blocks it on the main thread (lines 182-185), and the carrier thread unblocks it (line 291).
- `cli/interrupt.rs:320-331`, `register`, registers four actions for SIGINT alone, in the order `ACTIONS` gives at lines 17-22: a conditional default, the cancel flag, a second conditional default, and the arm. The first signal fires the cancel and arms the default. A second signal meets an armed conditional default and dies by it at once.
- `cli/interrupt.rs:91-111`, `Guard::finalize` and `finish`, call `emulate` when the cancel fired. `emulate` at lines 333-336 re-raises SIGINT alone. `finish` returns 130 only when the emulation returns.
- Engine workers mask every host signal (`engine/workers.rs:177-195`). So SIGTERM reaches only the main thread or the carrier, where no handler exists, and its default action kills the process.
- ADR 0017's amendment of 2026-09-22 is Ian's ruling for SIGINT: stop starting requests, let sent requests finish within their attempt timeout, print completed output and the stop line, then re-raise SIGINT. Nothing rules SIGTERM.

### The stop message

- `cli/mod.rs:85-88` runs the command and reports its failure through `told` at line 146. Nothing there reads the run's cancel.
- `engine/schedule.rs:263-264` refuses the next place when the cancel fires (`refuse` at lines 132-136), and a record already sent keeps running. When that send times out, `drain` at lines 151-165 stops at the timed-out record with the transport failure as the cause. So the cause says the backend timed out.
- `cli/failure.rs:450-464`, `transport_message`, says "the backend timed out; increase --timeout or try again". `cli/failure.rs:273-307`, `stopped`, prints the cause's line and then `stopped at record {at}; {finished} ... finished`.
- When the cancel alone stopped the run, `engine/schedule.rs:175-186` sets `at` to the place after the last finished record. That place may hold a record that never arrived.
- `specification/channels.md:5` names SIGINT alone. The exit table at lines 48-61 has no row for a signal. `specification/records.md:109` says the stop line names "the record where it stopped".

## Retained behavior

- The first signal stops new requests. Each sent request runs to its end, within its attempt timeout. Ian ruled this for SIGINT, and this ticket keeps it.
- Finished output is written in order before the command ends. A cache or recording entry for a sent request is still completed.
- The command ends by the signal, so a shell sees 128 plus its number.
- A second signal still ends the command at once with no stop line.
- A stop that no signal caused keeps its cause line and `stopped at record N; ...` line byte for byte. Demos 12 and 21 pin two of them.
- A local failure or a defect after a signal keeps its own line.
- The libraries and the extensions keep their own signal rules. The command's test acknowledgment variable keeps its name.

## The change

### The carrier routes SIGINT and SIGTERM

- `sigint_set` becomes a set of SIGINT and SIGTERM. The main thread blocks both while a command runs, and the carrier unblocks both, as it does SIGINT today.
- `register` runs the four `ACTIONS` for each of the two signals. Both signals share the one `default_armed` flag. So a second signal of either kind meets an armed default and dies by its own default at once.
- `State` gains `signal: Arc<AtomicUsize>`, which starts at SIGINT. The `Cancel` action registers `signal_hook::flag::register_usize` for its signal before it registers the cancel flag, so the handler stores the signal number before it fires the cancel. Only the first signal reaches that action, because a second one dies at the conditional default before it.
- `emulate` re-raises the signal `State` stored. `Guard::finish` returns 128 plus that number when the emulation returns.
- The defect messages that say "SIGINT routing" keep their words. Tests pin them, and they report a failed install, not a signal.

### A backend failure after a signal is the signal's stop

`Failure` gains one method in `cli/failure.rs`:

```rust
/// The failure a run reports once a signal stopped it: a backend cause becomes the stop.
pub(crate) fn after_signal(self) -> Self
```

It replaces a backend cause with `Failure::Cancelled`. The backend causes are the failures that say exit 4 about a sent request: `Transport`, `Status`, `TokenLimit`, `ReplyTooLarge`, `Reply`, and `Recognize(recognize::Error::LogicalQuestion)`. The last one comes from `engine/facade/recognize.rs:239`, and the engine gives it `Kind::Backend` at `engine/error.rs:119`. The build lists every `Failure` variant that the engine's `Kind::Backend` reaches and gives each one an arm. A `Stopped` run keeps its counts and gets `Cancelled` as its cause. After ticket 0146, a batched stop also carries the batch's range or the partial-reply form. `after_signal` clears both, so a signal stop never prints a range. Every other failure returns unchanged. `told` in `cli/mod.rs` calls it first when `environment.cancel.fired()`.

### The stop line after a signal names no record

In `stopped`, a `Cancelled` cause prints:

```text
thinkthen: stopped by a signal; {finished} {noun} finished{recording_clause}{withheld}
```

`stopped` checks for a `Cancelled` cause first, before it chooses between today's line and 0146's batch forms. The recording and withheld clauses keep today's words. Every other cause keeps `stopped at record {at}; ...`. The finished count is what a reader can count. The record after it may never have arrived, so the line does not name it. After ticket 0162, `at` becomes a line number and `finished` stays a count of sent records. This line reads only `finished`, so 0162's numbering does not change it.

### Pages

`specification/channels.md` line 5, from "On SIGINT" to the end of the paragraph, becomes:

> On SIGINT or SIGTERM, it stops starting work. Each request already sent finishes, within its attempt timeout. The command then writes the output it finished, prints the stop line and ends by the same signal, so a shell reports 130 or 143. A backend failure that ends a sent request after the signal is reported as the signal's stop. A second SIGINT or SIGTERM ends the command at once by that signal. It prints no stop line, and output not yet written is lost. A supervisor that sends SIGTERM should wait longer than `--timeout`, 30 seconds by default, before it kills the command, so sent requests can finish.

The exit table gains a row after 70:

> | 130, 143 | SIGINT or SIGTERM stopped the command. It ends by that signal, and a shell reports 128 plus the signal's number |

`specification/check.md` line 100, "On SIGINT the check prints no report", becomes "On SIGINT or SIGTERM the check prints no report".

`specification/records.md` line 109 gains, after its first sentence:

> A run stopped by SIGINT or SIGTERM names no record, because the next record may never have arrived. Its line reads `thinkthen: stopped by a signal; 2 records finished`.

ADR 0017 gains an amendment after the SIGINT one:

> ## Amendment, 2026-09-27: SIGTERM stops the command as SIGINT does
>
> Ticket 0169 extends the amendment of 2026-09-22 to SIGTERM. The command stops starting requests, lets sent requests finish within their attempt timeout, prints completed output and the stop line, then re-raises the signal that stopped it. A shell sees 130 or 143. A second SIGINT or SIGTERM takes its default action at once. A backend failure after either signal is reported as the stop. The stopped-at line of the 2026-09-22 amendment now reads `stopped by a signal; N records finished` after either signal. The CLI alone still owns signal registration and default-signal emulation. No library or host signal policy changes. Ian can overturn it.

`CHANGELOG.md` gains one line for the three changes.

## Decisions

Each is the ticket author's call. Ian can overturn any of them.

1. **SIGTERM follows the SIGINT rule and re-raises SIGTERM.** Supervisors send SIGTERM first and read 143 as a clean stop. Re-raising SIGINT would tell them the wrong signal. SIGHUP and SIGQUIT keep their defaults.
2. **One armed flag serves both signals.** Any second signal ends the run at once, whatever the first was. A supervisor that sends SIGTERM and then SIGKILL is unaffected.
3. **The first signal still waits for sent requests.** Ian ruled this for SIGINT. The second signal is the escape, and the page now says so. A notice on the first signal is deferred.
4. **A backend failure after a signal becomes the stop.** Ticket 0166 decision 1 rules the same for a library call whose token fired. Local failures and defects keep their lines, because each names something the user must repair.
5. **The signal stop line gives a count and no record.** A count is what the reader can check against standard output.

## Edge cases

Rows 3 and 4 run in `tests/backend/interrupt.rs` through its `held` helper, which waits for the requests, signals the child, waits for the acknowledgment file, then releases the replies. The helper gains the signal to send as a parameter. Rows 1, 2 and 5 need the child to end before the release, so they share one small flow in the same file: wait for the requests, send the signals, wait for the child to end under the test deadline, then release the replies.

| # | Input | Expected |
| --- | --- | --- |
| 1 | `decide --lines --jobs 1 --timeout 1 --max-retries 0` over `first\n`. The reply stays held until the child ends. SIGINT after 1 request | Standard output is empty. Standard error is exactly `thinkthen: stopped by a signal; 0 records finished\n`. The child ends by SIGINT. 1 request. Today standard error holds the timeout line and `stopped at record 1; 0 records finished` |
| 2 | `decide` over one document, the same held reply and flags without `--lines`, SIGINT after 1 request | Standard output and standard error are empty. The child ends by SIGINT. 1 request. Today standard error holds the timeout line |
| 3 | The existing `record_finishes_the_started_row_stops_before_another_and_completes_cache` | Its standard error becomes exactly `thinkthen: stopped by a signal; 1 record finished, 0 records from a recording\n`. Everything else it pins holds |
| 4 | Row 3 with SIGTERM | The same standard output and standard error as row 3. The child ends by SIGTERM. 1 request. The cache holds one complete entry. Today the child dies at once with empty output |
| 5 | Row 3's run with SIGINT, then SIGTERM after the acknowledgment, while the reply stays held | The child ends by SIGTERM before the release. Standard output and standard error are empty. 1 request |
| 6 | `decide --lines --batch 2 --jobs 1 --timeout 1 --max-retries 0` over `first\nsecond\n`, with the one request held until the child ends. SIGINT after 1 request | Standard output is empty. Standard error is exactly `thinkthen: stopped by a signal; 0 records finished\n`, with no range. The child ends by SIGINT. 1 request |
| 7 | `stopped_counts_use_record_only_at_one` in `cli/failure/tests.rs:472-509`, existing | Its `Cancelled` case at line 491 becomes exactly `thinkthen: stopped by a signal; 1 record finished, 1 record from a recording\n`. Its other cases hold |
| 8 | 0146's `a_failed_batch_stops_at_its_first_record`, row 4, which interrupts a batched run | Its standard error becomes exactly the signal line. Its other rows hold |
| 9 | A stop no signal caused, such as a 503 on record 3 | Unchanged. The demos and the existing stop tests pin `stopped at record N; ...` |

Rows 1, 2, 5 and 6 release the reply only after the child ends. So the attempt timeout, not a reply, ends each held request.

## Tests and proof

Build amendment, 2026-09-27: Ian kept the routine gate functional and moved stress and mutation campaigns out of this handoff. The edge table below names credible regressions; the build runs focused compiled-binary signal behavior, the failure mapping table, the carrier mask tests and strict lint. It does not execute nine deliberate breaks or a full stress ladder. The change uses the existing SIGINT acknowledgment, held listener and cache test. The new held-until-exit flow differs because a timeout or a second signal must end the child before the listener releases its reply.

| Test | What it proves | Deliberate breaks that turn it red |
| --- | --- | --- |
| `a_signal_after_a_hung_request_is_the_stop`, new, a three-row table | Rows 1, 2 and 6 | (a) `told` skips `after_signal`: every row prints the timeout line. (f) `stopped` chooses 0146's batch form before it checks for `Cancelled`: row 6 prints a range |
| `a_signal_replaces_only_backend_causes_and_keeps_stop_counts`, new table in `cli/failure/after_signal.rs` | Each backend cause, bare and inside `Stopped`, becomes `Cancelled` and keeps its counts. `RecordingStorage`, `Input` and `Defect` come back unchanged | (g) Drop any one backend arm: its row stays a backend failure. (h) Match every failure: the three local rows change |
| `record_finishes_the_started_row_stops_before_another_and_completes_cache` and `stopped_counts_use_record_only_at_one`, existing, their lines updated | Rows 3 and 7 | (b) `stopped` keeps `stopped at record {at}` for a signal: rows 1, 3 and 4 turn red |
| `carrier_masks_workers_and_cleanup_restores_the_mask`, existing in `cli/interrupt/tests.rs:186-200` | It gains `contains(Signal::SIGTERM)` asserts for the main thread and for a spawned thread | (c) SIGTERM left out of `sigint_set`: this test turns red |
| `record_finishes_the_started_row_stops_before_another_and_completes_cache`, extended to both signals | Rows 3 and 4, including the completed cache entry | (c2) SIGTERM left out of `register`: the child dies at once with empty output. (d) `emulate` re-raises SIGINT: the child ends by SIGINT |
| `a_second_signal_of_either_kind_ends_the_run_at_once`, new | Row 5 | (e) SIGTERM's conditional default reads its own flag: the child waits for its held reply and misses the deadline |

The compiled-binary edge cases use one loopback listener and request counts per run. The timeout and second-signal cases bound the child wait to four seconds and release their held reply afterward. No paid or live backend is used.

Overlap was checked.

- `sent_single_decide_and_aggregate_commands_flush_before_sigint_status` releases the reply at once, so no request fails after the signal. It still passes.
- `sigint_during_retry_wait_makes_exactly_one_request` ends in a retry wait, not a failed send. It prints nothing today and after.
- `cli/interrupt/tests.rs` pins `ACTIONS`, the install prefixes and the armed follow-up SIGINT through a child. It does not send SIGTERM. Its mask test gains two SIGTERM asserts, and no expected result changes.
- No test sends SIGTERM or lets a send fail after a signal.

The four questions:

- **What behavior does it protect?** A stopped run says a signal stopped it and names no record it cannot show. SIGTERM stops a run cleanly and ends by SIGTERM. A second signal escapes at once.
- **What credible regression fails it?** Each deliberate break above.
- **Why does no existing test catch it?** No test sends SIGTERM, and no test lets a sent request fail after a signal.
- **Does it need a test-only hook?** No. It uses the existing acknowledgment variable, the loopback listener and `kill`.

The coordinator runs the required landing gates after fresh code review. The builder runs formatting, policy, strict Clippy and the smallest affected CLI and compiled-binary tests before handing back.

## Budgets and measured ratchet

Nonblank lines against main `22332a38`, measured with the same rule as `sdlc/scripts/ratchet.mjs`. The accepted estimate of +167 understated the real-signal, end-before-release flow and the backend/local cause table. Ian's handover keeps the functional proof and removes the nine-mutation campaign. The coordinator authorized routine budget amendments within the settled outcome. This amendment changes line budgets, not signal behavior.

- `cli/interrupt.rs`: measured +25, at most +28. It registers both signals with the existing action order and remembers which arrived. I checked `register` and `emulate` for a shared path and use one loop over both signals.
- `cli/failure.rs`: measured +6, at most +8. Its current 492 nonblank lines stay under the 500-line cap. `after_signal` and the signal stop sentence live in `cli/failure/after_signal.rs` so the dispatcher stays short.
- `cli/failure/after_signal.rs`: measured 137 new lines, at most 145. It includes the backend/local cause table, the nested batch and stopped mappings, and the one signal stop sentence. The table uses one backend constructor list for bare and stopped cases. The result reuses `Failure::Cancelled` and keeps all existing local failure variants.
- `cli/mod.rs`: measured +3, at most +3.
- `tests/backend/interrupt.rs`: measured +170, at most +170. Existing held-request, listener, acknowledgment, cache and answer helpers serve the new SIGTERM and hung-request rows. A second flow keeps the backend reply held until after the child exits, which the existing helper cannot do. Its four-second deadline gives a failure bound; no timing threshold decides success. The second-signal row checks both signal orders.
- `cli/failure/tests.rs`: measured 0; one existing expected line changes.
- `cli/interrupt/tests.rs`: measured +12, at most +12, adding SIGTERM mask assertions on both the command and worker threads.
- `sdlc/ratchet.json`: main's 75,724 becomes the measured 76,077 nonblank Rust lines, +353: +105 source and +248 test lines. I checked the carrier's registration and emulation, the failure dispatcher, both test helpers, and the low-level carrier tests for duplication before raising it. The fresh reviewer checks this accounting.
- Pages: `channels.md`, `records.md`, `check.md`, the ADR 0017 amendment and one `CHANGELOG.md` line.
- No dependency. `signal_hook::flag::register_usize` is in the crate the command already uses. No paid call.

The commit names this growth and the duplication check. No low-level carrier test was added where a compiled-binary signal case already proves the same result.

## Stop rules

1. Stop before crossing the amended measured budgets by more than a tenth, or before adding a dependency.
2. Stop if the change needs the engine, the public API, a library or an extension.
3. Stop if an existing test changes its expected result, other than rows 3, 7 and 8 and the mask test's added asserts.
4. Stop if a focused signal boundary or the backend/local mapping table fails.
5. Stop if a row passes or fails by timing alone. Each signal waits for the listener's request count, and each release waits for the acknowledgment or the child's end.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

1. Tickets 0146 and then 0162 land first. 0162 edits `cli/failure.rs`, `cli/schedule.rs` and the engine's stop outcome. This ticket reads only `finished` from a signal stop, so it does not depend on 0162's `at`. It edits `stopped` beside 0162's edits, so it rebases on 0162 to keep one merge.
2. Ticket 0154 opens `cli/failure.rs` and `records.md`. Ticket 0153 opens `cli/mod.rs`. Tickets 0146, 0152, 0153, 0156 and 0167 open `channels.md`. Tickets 0148, 0152 and 0155 open ADR 0017. This ticket touches `stopped`, `told`, `channels.md` line 5 and the exit table, `records.md` line 109, `check.md` line 100, and a new ADR section. It also changes 0146's `a_failed_batch_stops_at_its_first_record` row 4. The second ticket to land merges these lines.
3. Ticket 0168 shares no file with this one and builds on its own.

## Scope and exclusions

Excluded: a cancellable socket read, a notice on the first signal, SIGHUP and SIGQUIT, the libraries' and extensions' signal rules, killed runs' temporary cache files (local experiment 284 file 47, ticket 0163), and `site/`.

## Routing

Builder: Codex in the retained command lane under Ian's handover. Reviewer: a fresh read-only Codex session for the final code.

## Complexity

Contract 2; State/timing 3; Reach 2; Proof 2; Cost of error 2; Total 11. Minimum floor: level 3 for signals. Final level: 3. The risks are a second signal that no longer escapes, which row 5 guards, and a lost cache entry after SIGTERM, which row 4 guards.

## Deferred gaps

- The first signal still waits up to the attempt timeout for a hung request. A cancellable socket read is the full fix, and nothing schedules it.
- The first signal prints no notice that it is waiting. Report 11 suggested "finishing N requests in flight; press Ctrl-C again to stop now".
- SIGHUP and SIGQUIT keep their default actions.

## What Ian can overturn

- Decision 1: SIGTERM re-raises SIGTERM.
- Decision 3: the first signal waits for sent requests. This is his SIGINT ruling, extended.
- Decision 4: a backend failure after a signal becomes the stop.
- The ADR 0017 amendment.

## Closes

Local experiment 284 files 46, 56 and 116. The repository holds them in `sdlc/issues/2026-09-26-architect-review-01-filter-and-streams.md`, `sdlc/issues/2026-09-26-architect-review-11-failure-and-scripting.md` item 2 and its SIGTERM line, and items I9, I10, 3 and 8 of `sdlc/issues/2026-09-26-architect-review-severity-3-findings.md`.

## Evidence

- Starts from: Local experiment 284 files 46, 56 and 116, their sources in local experiment 273, and the author's scratch runs of the `origin/main` `18f0381e` debug binary against loopback listeners. The code: `cli/interrupt.rs:17-22`, `:91-111`, `:182-185`, `:275-336`; `cli/mod.rs:85-88`, `:146`; `cli/failure.rs:182-184`, `:273-307`, `:450-464`; `engine/schedule.rs:132-136`, `:151-186`, `:263-264`; `engine/workers.rs:177-195`; `channels.md:5`, `:48-61`; `records.md:109`; ADR 0017's amendment of 2026-09-22.
- Keeps: Sent requests finish within their attempt timeout. Finished output and cache entries are written. The command ends by the signal. A second signal escapes at once. Stops no signal caused keep their lines byte for byte. Local failures and defects keep their lines.
- Changes: SIGTERM stops a run as SIGINT does and ends by SIGTERM. A backend failure after a signal is reported as the stop. The signal stop line gives the finished count and names no record. `channels.md`, `records.md`, `check.md` and ADR 0017 state both signals, the second-signal escape and the exit statuses.
- Proof: Focused rows 1 to 8 across `tests/backend/interrupt.rs`, `cli/failure/after_signal.rs` and the existing low-level carrier tests; formatting, policy, strict Clippy and affected tests in the build lane. The coordinator runs landing gates after the fresh code review. The nine-mutation campaign is deferred under Ian's functional-gate ruling.
- One broader library `signal` filter observed the unrelated worker SIGUSR1 test fail once, and that exact test passed once in isolation. Its cause is unproven; `sdlc/records/0169-a-signal-stops-the-command-plainly.md` records the bounded observation and the coordinator tracks test reliability. No worker or engine source changed.
- Defers: A cancellable socket read, a first-signal notice, and SIGHUP and SIGQUIT.
