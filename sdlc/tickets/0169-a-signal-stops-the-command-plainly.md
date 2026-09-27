---
flow: build
priority: 169
opens: crates/thinkthen/src/cli/interrupt.rs crates/thinkthen/src/cli/interrupt crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/cli/mod.rs crates/thinkthen/tests/backend/interrupt.rs specification/channels.md specification/records.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md CHANGELOG.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0169: A signal stops the command plainly

Status: ready for review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

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

It replaces a backend cause with `Failure::Cancelled`. The backend causes are `Transport`, `Status`, `TokenLimit`, `ReplyTooLarge` and `Reply`, the failures that say exit 4 about a sent request. A `Stopped` run keeps its counts and gets `Cancelled` as its cause. Every other failure returns unchanged. `told` in `cli/mod.rs` calls it first when `environment.cancel.fired()`.

### The stop line after a signal names no record

In `stopped`, a `Cancelled` cause prints:

```text
thinkthen: stopped by a signal; {finished} {noun} finished{recording_clause}{withheld}
```

The recording and withheld clauses keep today's words. Every other cause keeps `stopped at record {at}; ...`. The finished count is what a reader can count. The record after it may never have arrived, so the line does not name it. After ticket 0162, `at` becomes a line number and `finished` stays a count of sent records. This line reads only `finished`, so 0162's numbering does not change it.

### Pages

`specification/channels.md` line 5, from "On SIGINT" to the end of the paragraph, becomes:

> On SIGINT or SIGTERM, it stops starting work. Each request already sent finishes, within its attempt timeout. The command then writes the output it finished, prints the stop line and ends by the same signal, so a shell reports 130 or 143. A backend failure that ends a sent request after the signal is reported as the signal's stop. A second SIGINT or SIGTERM ends the command at once by that signal. It prints no stop line, and output not yet written is lost.

The exit table gains a row after 70:

> | 130, 143 | SIGINT or SIGTERM stopped the command. It ends by that signal, and a shell reports 128 plus the signal's number |

`specification/records.md` line 109 gains, after its first sentence:

> A run stopped by SIGINT or SIGTERM names no record, because the next record may never have arrived. Its line reads `thinkthen: stopped by a signal; 2 records finished`.

ADR 0017 gains an amendment after the SIGINT one:

> ## Amendment, 2026-09-27: SIGTERM stops the command as SIGINT does
>
> Ticket 0169 extends the amendment of 2026-09-22 to SIGTERM. The command stops starting requests, lets sent requests finish within their attempt timeout, prints completed output and the stop line, then re-raises the signal that stopped it. A shell sees 130 or 143. A second SIGINT or SIGTERM takes its default action at once. A backend failure after either signal is reported as the stop. The CLI alone still owns signal registration and default-signal emulation. No library or host signal policy changes. Ian can overturn it.

`CHANGELOG.md` gains one line for the three changes.

## Decisions

Each is the ticket author's call. Ian can overturn any of them.

1. **SIGTERM follows the SIGINT rule and re-raises SIGTERM.** Supervisors send SIGTERM first and read 143 as a clean stop. Re-raising SIGINT would tell them the wrong signal. SIGHUP and SIGQUIT keep their defaults.
2. **One armed flag serves both signals.** Any second signal ends the run at once, whatever the first was. A supervisor that sends SIGTERM and then SIGKILL is unaffected.
3. **The first signal still waits for sent requests.** Ian ruled this for SIGINT. The second signal is the escape, and the page now says so. A notice on the first signal is deferred.
4. **A backend failure after a signal becomes the stop.** Ticket 0166 decision 1 rules the same for a library call whose token fired. Local failures and defects keep their lines, because each names something the user must repair.
5. **The signal stop line gives a count and no record.** A count is what the reader can check against standard output.

## Edge cases

Rows 1 to 6 run in `tests/backend/interrupt.rs` through its `held` helper, which waits for the requests, signals the child, waits for the acknowledgment file, then releases the replies. The helper gains the signal to send as a parameter.

| # | Input | Expected |
| --- | --- | --- |
| 1 | `decide --lines --jobs 1 --timeout 1 --max-retries 0` over `first\n`. The reply waits 3 s after release. SIGINT after 1 request | Standard output is empty. Standard error is exactly `thinkthen: stopped by a signal; 0 records finished\n`. The child ends by SIGINT. 1 request. Today standard error holds the timeout line and `stopped at record 1; 0 records finished` |
| 2 | `decide` over one document, the same hung reply and flags without `--lines`, SIGINT after 1 request | Standard output and standard error are empty. The child ends by SIGINT. 1 request. Today standard error holds the timeout line |
| 3 | The existing `record_finishes_the_started_row_stops_before_another_and_completes_cache` | Its standard error becomes exactly `thinkthen: stopped by a signal; 1 record finished, 0 records from a recording\n`. Everything else it pins holds |
| 4 | Row 3 with SIGTERM | The same standard output and standard error as row 3. The child ends by SIGTERM. 1 request. The cache holds one complete entry. Today the child dies at once with empty output |
| 5 | Row 3's run with SIGINT, then SIGTERM after the acknowledgment, while the reply stays held | The child ends by SIGTERM before the release. Standard output and standard error are empty. 1 request |
| 6 | A stop no signal caused, such as a 503 on record 3 | Unchanged. The demos and the existing stop tests pin `stopped at record N; ...` |

Row 5 waits for the child to end, under the test deadline, before it releases the reply.

## Tests and proof

| Test | What it proves | Deliberate breaks that turn it red |
| --- | --- | --- |
| `a_signal_after_a_hung_request_is_the_stop`, new, a two-row table | Rows 1 and 2 | (a) `told` skips `after_signal`: both rows print the timeout line |
| `record_finishes_the_started_row_stops_before_another_and_completes_cache`, existing, its line updated | Row 3 | (b) `stopped` keeps `stopped at record {at}` for a signal: rows 1, 3 and 4 turn red |
| `sigterm_stops_a_record_run_as_sigint_does`, new | Row 4 | (c) SIGTERM left out of the routed set: the child dies at once with empty output. (d) `emulate` re-raises SIGINT: the child ends by SIGINT |
| `a_second_signal_of_either_kind_ends_the_run_at_once`, new | Row 5 | (e) SIGTERM's conditional default reads its own flag: the child waits for its held reply and misses the deadline |

The build runs each deliberate break by hand, confirms the named test turns red, and records the failing line.

Overlap was checked.

- `sent_single_decide_and_aggregate_commands_flush_before_sigint_status` releases the reply at once, so no request fails after the signal. It still passes.
- `sigint_during_retry_wait_makes_exactly_one_request` ends in a retry wait, not a failed send. It prints nothing today and after.
- `cli/interrupt/tests.rs` pins `ACTIONS`, the install prefixes and the armed follow-up SIGINT through a child. It does not send SIGTERM, and its expected results do not change.
- No test sends SIGTERM or lets a send fail after a signal.

The four questions:

- **What behavior does it protect?** A stopped run says a signal stopped it and names no record it cannot show. SIGTERM stops a run cleanly and ends by SIGTERM. A second signal escapes at once.
- **What credible regression fails it?** Each deliberate break above.
- **Why does no existing test catch it?** No test sends SIGTERM, and no test lets a sent request fail after a signal.
- **Does it need a test-only hook?** No. It uses the existing acknowledgment variable, the loopback listener and `kill`.

The gate ladder `sdlc/scripts/{install,lint,test,spec,surfaces}` runs before handing back.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`.

- `cli/interrupt.rs`: at most 18 net.
- `cli/failure.rs`: at most 22 net. `lint` caps a file at 500 nonblank lines. After 0146, 0154 and 0162 land, if this would pass 500, `after_signal` and the signal line move into a module under `cli/failure/`.
- `cli/mod.rs`: at most 3 net.
- `tests/backend/interrupt.rs`: at most 70 net.
- `sdlc/ratchet.json`: at most 113 above main at build time.
- Pages: `channels.md`, `records.md`, the ADR 0017 amendment and one `CHANGELOG.md` line.
- No dependency. `signal_hook::flag::register_usize` is in the crate the command already uses. No paid call.

Each ratchet moves to the measured total in the commit that needs it, and that commit says what grew. The build looks first for duplication to delete in `register` and `emulate`.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs the engine, the public API, a library or an extension.
3. Stop if an existing test other than row 3's changes its expected result.
4. Stop if any deliberate break stays green.
5. Stop if a row passes or fails by timing alone. Each signal waits for the listener's request count, and each release waits for the acknowledgment or the child's end.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

1. Tickets 0146 and then 0162 land first. 0162 edits `cli/failure.rs`, `cli/schedule.rs` and the engine's stop outcome. This ticket reads only `finished` from a signal stop, so it does not depend on 0162's `at`. It edits `stopped` beside 0162's edits, so it rebases on 0162 to keep one merge.
2. Ticket 0154 opens `cli/failure.rs` and `records.md`. Ticket 0153 opens `cli/mod.rs`. Tickets 0146, 0152, 0153, 0156 and 0167 open `channels.md`. Tickets 0148, 0152 and 0155 open ADR 0017. This ticket touches `stopped`, `told`, `channels.md` line 5 and the exit table, `records.md` line 109, and a new ADR section. The second ticket to land merges these lines.
3. Ticket 0168 shares no file with this one and builds on its own.

## Scope and exclusions

Excluded: a cancellable socket read, a notice on the first signal, SIGHUP and SIGQUIT, the libraries' and extensions' signal rules, killed runs' temporary cache files (local experiment 284 file 47, ticket 0163), and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and the code.

## Complexity

Contract 2; State/timing 3; Reach 2; Proof 2; Cost of error 2; Total 11. Minimum floor: level 3 for signals. Final level: 3. The risks are a second signal that no longer escapes, which row 5 guards, and a lost cache entry after SIGTERM, which row 4 guards.

## Deferred gaps

- The first signal still waits up to the attempt timeout for a hung request. A cancellable socket read is the full fix, and nothing schedules it.
- The first signal prints no notice that it is waiting. Report 11 suggested "finishing N requests in flight; press Ctrl-C again to stop now".
- No row plants a local failure after a signal, so no test pins that one keeps its line. The rule is one `match` with five named arms.
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
- Changes: SIGTERM stops a run as SIGINT does and ends by SIGTERM. A backend failure after a signal is reported as the stop. The signal stop line gives the finished count and names no record. `channels.md`, `records.md` and ADR 0017 state both signals, the second-signal escape and the exit statuses.
- Proof: Rows 1 to 5 in `tests/backend/interrupt.rs`, five deliberate breaks each run by hand, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: A cancellable socket read, a first-signal notice, a planted local failure after a signal, and SIGHUP and SIGQUIT.
