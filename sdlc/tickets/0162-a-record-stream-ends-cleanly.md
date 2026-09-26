---
flow: build
priority: 162
opens: crates/thinkthen/Cargo.toml Cargo.lock crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/schedule crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking crates/thinkthen/src/core/records.rs crates/thinkthen/tests/backend/stream_ends.rs crates/thinkthen/tests/backend/main.rs specification/records.md specification/decide.md specification/filter.md specification/rank.md specification/choose.md specification/tag.md specification/score.md specification/annotate.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0162: A record stream ends cleanly

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. A second agent also reviews the dependency feature it adds, as the repository `CLAUDE.md` asks.

## Outcome and authority

Three things about a record stream start to hold.

1. When the reader downstream closes the pipe, `filter` stops scheduling at once, as `specification/records.md` line 141 promises. Today it keeps paying until its next kept record.
2. A blank line in a `--lines` stream no longer cuts the run short. It is skipped and still counted, and every page that takes `--lines` says so.
3. `records.md`'s advice to set `--jobs 3` names the reply time it assumes, so the request rate it implies is honest.

Each issue blocks 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The blank-line issue leaves the rule to the queue owner, and this ticket picks it. Ian can overturn each choice.

## What happens today

Read from `origin/main` `ebd28382`.

- `records.md` line 141: "When the program downstream closes the pipe, the tool stops reading and stops scheduling." The command learns of a closed pipe only when a write fails: `cli/edge.rs:278`, `write_line`, returns `false` on `BrokenPipe`. `filter` writes only the records it keeps. Local experiment 273, report 01, issue 1, fed one `keep first` line and 300 `drop N` lines on loopback. `filter ... | head -1` sent 301 requests, and `decide --lines | head -1` sent 5. The only test of the promise runs `decide` (`sdlc/issues/2026-09-26-filter-keeps-sending-after-the-reader-closes-the-pipe.md`).
- `records.md` line 14 says only "Each line is one text record. A trailing newline ends the last record." Under `--lines`, an empty or white-space line stops the run at exit 2 with `the evidence is empty or blank`, from `core/text.rs:19`. A blank line in the middle drops every record after it, and a trailing `\n\n` fails after the last real line. Reports 01 (issue 2) and 11 (issue 1) ran it with spaces and with a CRLF blank line (`sdlc/issues/2026-09-26-a-blank-line-stops-a-lines-run.md`). `audit` skips blank lines and still counts them (`audit.md` line 41). `relate` refuses a blank line before any request (`relate.md` line 34), so it cuts nothing short.
- `records.md` line 131 says a run that must stay inside the documented limit "sets `--jobs 3`". Experiment 206's rates behind it came from short lines whose replies took about 0.18 seconds. Report 07, finding I1, measured `--jobs 3` at 1,285 requests a minute on loopback with 100 ms replies, and the default `--jobs 4` at 1,519 a minute live (`sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`).
- Ticket 0146 rewrites the record loop and `records.md`'s request paragraphs for batching. After it, a request carries a batch of records and `--jobs` counts batches in flight.

## Design

### A closed pipe stops scheduling

Between dispatches, the record loop asks whether standard output is still open without writing to it. On Unix it polls the output descriptor with a zero timeout and reads an error or hang-up as a closed pipe. That takes the path a failed write takes today: the run stops reading and scheduling, lets requests already sent finish, and exits with the code it earned. `nix`, already a Unix dependency, gains its `poll` feature. Nothing else is added. Standard output that is a file or a terminal never reports a hang-up, so those runs do not change. On a platform without the poll, the run keeps today's write-time check. The release targets are all Unix.

`records.md` line 141 keeps its sentence and adds: "The tool notices between requests, so a command such as `filter`, which prints only some records, stops as soon as the reader is gone."

### A blank line is skipped and counted

Under `--lines`, a line that is empty or holds only white space, a carriage return included, makes no request and prints no row. It still takes its record number, so "record N" in a stop line still means line N. This covers `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate`. `relate` keeps its rule, which refuses the whole set before any request, and `find` keeps its unit rules. `--jsonl` keeps "No blank lines". `records.md` line 14 becomes: "Each line is one text record. A line that is empty or holds only white space is skipped, makes no request and prints nothing, and still counts toward record numbers. A trailing newline ends the last record." Each verb page that takes `--lines` links that sentence.

### `--jobs 3` names its reply time

`records.md` line 131's advice becomes a statement of the arithmetic, in the words 0146 leaves on the page. The rate is about `jobs` requests for each reply time. At about 0.18 seconds a reply, experiment 206's short lines, a throttle of 3 sent about 980 requests a minute. A backend that answers in 0.1 seconds takes `--jobs 3` past 1,200 a minute, as report 07 measured on loopback. Each number names its record. The page says that no setting caps requests a minute, and it points to the pacer issue.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A closed pipe is found by polling standard output between dispatches.** It needs no write and no new dependency.
2. **A blank `--lines` line is skipped and counted.** A grep user expects it, `audit` already does it, and a refusal would still pay for a prefix of the input first. No working pipeline depends on today's stop, because it fails at exit 2.
3. **`--jsonl`, `relate` and `find` keep their rules.** Each already refuses before a request or has its own unit rule.
4. **The `--jobs 3` sentence becomes arithmetic with measured rates.** The pacer stays out of 0.1.

## Edge cases

| Input | Expected |
| --- | --- |
| `filter` over `keep first` and 300 `drop N` lines, output into a reader that reads one line and closes | The loopback receives at most `jobs` requests beyond the first kept record's, and the run exits with the code it earned, as `write_line` documents |
| The same under `--batch 1` after ticket 0146 | The same bound, counted in requests |
| `decide --lines` into a reader that closes after one line | It stops, as today |
| `filter` writing into a file | Every record is judged, as today |
| `printf 'keep a\n\nkeep c\n' \| filter --lines` | Two requests. Rows for records 1 and 3. Exit 0 |
| A line of spaces, a `\r\n` blank line, and a trailing `\n\n` | Each is skipped, and the run exits 0 |
| A run that fails at line 4 after a blank line 2 | The stop line names record 4 |
| `decide --lines` over only blank lines | No request, no output, exit 0 |
| `--jsonl` with a blank line | Refused, as today |

## Proof

Every test drives the compiled binary against the in-process loopback in `tests/backend/harness`, which counts requests.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_closed_pipe_stops_filter`, new in `tests/backend/stream_ends.rs` | Edge rows 1 to 4. The test holds the child's standard output as a real pipe, reads one line, drops the reader, waits for the exit, and counts the loopback's requests | (a) Check the pipe only on write: row 1 counts 301. (b) Treat a file as closed: row 4 stops early |
| `a_blank_line_is_skipped_and_counted`, same file | Edge rows 5 to 9, each pinning standard output, the whole standard error line, the exit code and the request count | (c) Refuse the blank line as today: row 5 stops at exit 2. (d) Skip without counting: row 7's stop line names record 3. (e) Skip under `--jsonl` too: row 9 passes |

The four questions:

- **What behavior does it protect?** `records.md`'s closed-pipe promise for a command that prints some records, and the blank-line rule.
- **What credible regression fails it?** A pipe check that only a write can trip, a blank line that stops the run again, and record numbers that drift from line numbers.
- **Why does no existing test catch it?** The closed-pipe test runs `decide`, which writes every record. No test feeds a blank line under `--lines`.
- **Does it need a test-only hook?** No. The pipe is a real pipe and the loopback counts real requests.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `cli/edge.rs` and the record loop that 0146 leaves: at most 30 net.
- `core/records.rs`: at most 15 net.
- `tests/backend/stream_ends.rs`: at most 170, new, and one `mod` line.
- Pages: at most 20 net.
- `crates/thinkthen/Cargo.toml`: one feature word. `Cargo.lock`: no new package.
- `sdlc/ratchet.json` moves to the measured total, at most 45 above main. The commit says what grew.
- No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a package to `Cargo.lock`.
2. Stop if polling needs `unsafe` code. The crate forbids it.
3. Stop if a run whose output is a file or a terminal changes.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.
6. Stop if ticket 0146 has not landed.

## Build order

It builds after ticket 0146 lands, because 0146 rewrites the record loop, `cli/edge.rs` and `records.md`. It never builds beside tickets 0153 or 0154, which open `cli/args.rs`, `cli/edge.rs` and `records.md`. The coordinator places it before or after them. It may build beside ticket 0163.

## Scope and exclusions

Excluded: a requests-per-minute pacer, `relate`'s and `find`'s line rules, blank lines under `--jsonl`, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code, and a second agent for the `nix` feature.

## Complexity

Contract 1; state and timing 2; reach 1; proof 2; cost of error 1; total 7. Final level: 2. The timing risk is a poll that reports a hang-up on a live pipe, which row 4 and the existing `decide` test guard.

## Deferred gaps

- The pacer itself. `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md` keeps it past 0.1.
- The closed-pipe check on platforms without `poll`. None is a release target.

## What Ian can overturn

- Decision 2: a blank `--lines` line is skipped and counted, in place of a clearer refusal.
- Decision 3: `--jsonl` keeps refusing a blank line.

## Closes

`sdlc/issues/2026-09-26-filter-keeps-sending-after-the-reader-closes-the-pipe.md` and `sdlc/issues/2026-09-26-a-blank-line-stops-a-lines-run.md`. The wording item of `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`, which then holds only the pacer.

## Evidence

- Starts from: Local experiment 273, report 01 issues 1 and 2, report 11 issue 1, and report 07 finding I1, as the three issues record them. The code at `origin/main` `ebd28382`: `cli/edge.rs:278` and `core/text.rs:19`. `records.md` lines 14, 131 and 141, `audit.md` line 41 and `relate.md` line 34. Experiment 206's rates as `records.md` cites them.
- Keeps: Every run whose output is a file or a terminal. `decide`'s closed-pipe stop. `--jsonl`, `relate` and `find` line rules. Output order and exit codes.
- Changes: A closed pipe stops scheduling between dispatches. A blank `--lines` line is skipped and counted. The `--jobs 3` sentence states its arithmetic.
- Proof: Two outside-in tests with five plants, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The pacer, and the check on platforms without `poll`.
