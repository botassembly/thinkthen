# 0024: Bound record scheduling and serve the loop

Branch `ticket/0024-bound-record-scheduling`. Built 2026-09-20.

## What landed

Record mode now returns an answer while standard input stays open under one job and under the default four jobs. Streaming output keeps at most `jobs` dispatched rows between input and their ordered output position. `rank` keeps its prior rule: completed work opens another worker place because the command must hold every row before it can sort.

`schedule.rs` now receives input and completed requests as events on one coordinator channel. A detached input reader owns the input and reads one record only when the coordinator grants a permit. A blocked read can no longer hide a completed request, a backend failure, or a closed output pipe. The coordinator stops granting permits after a failure or a closed pipe and waits only for requests it already dispatched.

No command, option, wire byte, recording identity, retry rule, or dependency changed.

## Red then green

The first incremental test wrote one record with `--jobs 1`, kept input open, and timed out before the fix. The first bound test held row 1 under `--jobs 4` and observed all eight supplied records reach the listener before row 1 was released.

The first implementation drained completed rows before it dispatched again and counted dispatched but unprinted rows. Its test passed under one job. Independent review repeated the case with the default four jobs and found the same hang: dispatch blocked reading rows 2 through 4 before the scheduler received row 1's completed answer.

The accepted implementation moves that blocking read behind a permit-driven reader. The incremental test now runs under one job and the default. The bound test buffers eight rows, observes requests 1 through 4, and observes responses 2 through 4 while row 1 is held. After row 1 is released, one shared event channel proves its output arrives before request 5 starts. No elapsed sleep supplies that proof.

Two more tests protect the reason the reader is detached. One keeps standard input open after a backend failure and observes exit 4. The other reads row 1, closes the output pipe, writes row 2, keeps input open, and observes exit 0. Both kill and reap the child if the expected exit does not arrive.

## Review

The design reviewer accepted the ticket after raising its contract score from 0 to 1 and making the `rank` exception explicit. The code reviewer rejected two passes.

1. The first pass found the default-job hang and a timing-based bound test. That finding replaced the drain-first loop with the input and completion event design and replaced the sleep with explicit response and output events.
2. The second pass accepted the production design and found that no repository test held input open through a backend failure or a closed output pipe. Both cases now have integration tests.

The final pass accepted the whole diff with no remaining blocker. It ran the four scheduler tests, the full workspace tests, all four rungs, the ratchet, and the diff check.

## Choices made where the ticket was silent

Ian can overturn these choices.

- **The input reader owns its reader and is detached.** A scoped reader would make the coordinator wait forever for an open input pipe after output closed or a request failed. Owning the reader required the internal input boundary to be `Send + 'static` and changed `main` to pass `Stdin` rather than `StdinLock`.
- **Only one input read may wait at once.** The coordinator grants one permit and waits for either that input event or a worker event. This keeps read-ahead inside the same scheduling window and needs no polling or added dependency.
- **`rank` counts live requests while streaming commands count dispatched but unprinted rows.** A final sort cannot print a row early. Keeping its existing rule preserves whole-input ranking and its bounded worker count.
- **A row already read when another row fails is discarded before dispatch.** The run stops at the earliest dispatched failure and sends no new paid request after it learns of the failure.

## Gates and size

The final ceiling is 15,183 measured Rust lines, up from 14,830. The production change adds the event and permit path. Most growth is the deterministic scheduling tests and the listener observations they need. No shared helper or repeated production path could be removed without hiding the event order under test.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `15183/15183` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.

## What proved wrong

The ticket's instruction to drain ready rows before reading another row was necessary but incomplete. With more than one job, a loop that fills every open place still blocks on input before it can receive an answer. Correct interactive behavior requires the scheduler to wait for input and completions together.
