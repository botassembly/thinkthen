---
flow: build
priority: 90
opens: crates/thinkthen/src/schedule.rs crates/thinkthen/tests specification/records.md README.md sdlc/ratchet.json
---

# 0024: Bound record scheduling and serve the loop

Status: ready

## Outcome

Streaming record mode returns each next answer before it blocks for more input, and at most `jobs` dispatched rows wait between input and ordered output. `rank` still holds its rows to compute the final order.

## Current Facts

The scheduler fills an open request slot before draining completed rows. With `--jobs 1`, a local listener answered record 1 and the command printed nothing until record 2 arrived. With record 1 held and `--jobs 4`, all 100 input records reached the listener before record 1 finished. `records.md` promises at most `jobs` waiting rows, and the README names a request-and-reply `coproc` as the steady-loop form.

## Scope

- Drain every ready row in order before reading another input record.
- Bound dispatched rows that have not reached their output position, including completed rows waiting behind an earlier record.
- Preserve output order, the stop at the earliest failure, recording of requests already sent, the closed-pipe stop, and `rank`, which intentionally holds its final order.
- Keep standard threads and the existing channels. Add no dependency and change no command or option.

Excluded: changing `jobs`, rate limiting, recording identity, retries, and changing `rank` to stream.

## Acceptance

- A deterministic integration test writes one line under `--jobs 1`, observes its answer without closing input or writing line 2, then repeats for line 2.
- A deterministic integration test holds record 1 under `--jobs 4` and proves no fifth request starts until record 1 is released and printed. It then proves all rows print in input order.
- Existing focused tests cover earliest failure, a closed downstream pipe, and byte-identical output across job counts.
- The source ceiling equals the measured total, and the full gate ladder passes.

## Dependencies

None.

## Complexity

- Contract score: 1
- State and timing score: 2
- Reach score: 1
- Proof score: 2
- Cost of error score: 2
- Total: 8
- Minimum level floor: level 3
- Final level: 3
- Reasons: two public promises cover the interactive loop and bounded waiting rows; concurrency and ordered streaming require deterministic synchronization; a wrong fix can retain unbounded data and continue paid requests.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: accepted. The reviewer confirmed the scheduler owns both defects, the two deterministic tests prove the outcome, the ticket outranks the prior polish work, and level 3 with Sol medium is correct.
- Code review: pending
