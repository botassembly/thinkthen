# `filter` keeps sending after the reader closes the pipe

Status: closed by ticket 0162 on 2026-09-27. The command polls a closed downstream pipe between reads and requests, including while its input remains open. Linux compiled-binary proofs pass; the specification states the platform fallback honestly.

## What happens

`specification/records.md` line 141 says: "When the program downstream closes the pipe, the tool stops reading and stops scheduling." The command notices a closed pipe only when a write fails (`crates/thinkthen/src/cli/edge.rs:278`, `write_line`). `filter` writes only the records it keeps. So after `head` exits, `filter` keeps reading and sending until its next kept record, which can be the end of the file.

The report measured it on loopback with a dummy key. The input was one `keep first` line and 300 `drop N` lines. `thinkthen filter ... | head -1` sent 301 requests. `thinkthen decide ... --lines | head -1` over the same input sent 5. An offline replay over 3,002 records ran to a replay miss at record 3,002, long after `head` had gone.

Seven tests on main close the output pipe: `tests/backend/parallel.rs:359`, `table.rs:470`, `scheduling.rs:261`, `tag/matrix.rs:245`, `annotate/scheduling.rs:324`, `find.rs:283` and `relate.rs:311`. Each runs a command that writes a row for every record or writes its one result at once. A failed write stops each of them, so none sees a pipe close while the command writes nothing.

## Checked on main

Verified by reading the code: `write_line` returns `false` only on a `BrokenPipe` write error. The loopback counts come from the report. They were not rerun here.

## What would fix it

Check whether standard output is still open without writing to it, between dispatches. A poll for an error or hang-up on the output descriptor is one way. Add an outside-in test that runs `filter` with a selective question into `head -1` and counts the requests a loopback listener receives.

Batching ticket 0146 rewrites the same record loop. The fix can land inside 0146 or right after it. That choice belongs to the queue owner.

## Done when

`filter ... | head -1` over a selective input stops scheduling within the requests already in flight, and a test counts the requests.

## Resolution

The independently reviewed code at `66da9fb1` and focused passing checks are recorded in `sdlc/records/0162-a-record-stream-ends-cleanly.md`. The coordinator also ran the selected 31-ID command boundary: 25 passed, six deliberate not-run cases, zero failures.
