---
flow: build
priority: 162
opens: crates/thinkthen/Cargo.toml Cargo.lock crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/schedule crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/annotate_schedule.rs crates/thinkthen/src/cli/recognize.rs crates/thinkthen/src/cli/recognize/dry_run.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/engine/schedule.rs crates/thinkthen/src/engine/annotate_schedule.rs crates/thinkthen/src/engine/deadline_tests crates/thinkthen/src/core/records.rs crates/thinkthen/src/core/batch.rs crates/thinkthen/tests/backend/parallel.rs crates/thinkthen/tests/backend/streaming.rs specification/records.md specification/filter.md specification/rank.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0162: A record stream ends cleanly

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner, and revised the same day after its first review. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. A second agent also reviews the final diff, because it raises the source size ceiling and adds the `poll` feature to `nix`. `AGENTS.md` line 11 asks for that review on either change, and the second agent names what it checked.

## Outcome and authority

Three things about a record stream start to hold.

1. When the reader downstream closes the pipe, `filter` stops scheduling at once, as `specification/records.md` line 141 promises. Today it keeps paying until its next kept record.
2. A blank line under line framing no longer cuts the run short. The reader skips it before any request, and record numbers still name input lines.
3. `records.md`'s advice to set `--jobs 3` names the reply time it assumes, so the request rate it implies is honest.

Each issue blocks 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The blank-line issue leaves the rule to the queue owner. The coordinator ruled it on 2026-09-26 under ADR 0054 item 2, "Filter at the earliest step", and this ticket builds that ruling. Ian can overturn each choice.

Line framing means each line is one text record. `--lines` asks for it, and `filter` and `rank` use it by default when no pointer is given (`cli/asking.rs::read_by`). Every rule below keys on line framing, never on the typed flag.

## What happens today

Read from `origin/main` `8084d38a`.

- `records.md` line 141: "When the program downstream closes the pipe, the tool stops reading and stops scheduling." The command learns of a closed pipe only when a write fails: `cli/edge.rs:278`, `write_line`, returns `false` on `BrokenPipe`. `filter` writes only the records it keeps. Local experiment 273, report 01, issue 1, fed one `keep first` line and 300 `drop N` lines on loopback. `filter ... | head -1` sent 301 requests, and `decide --lines | head -1` sent 5.
- Seven tests on main already close the output pipe: `tests/backend/parallel.rs:359`, `table.rs:470`, `scheduling.rs:261`, `tag/matrix.rs:245`, `annotate/scheduling.rs:324`, `find.rs:283` and `relate.rs:311`. Each runs a command that writes a row for every record or writes its one result at once. So a write fails right after the reader closes, and none of them sees a pipe close while the command writes nothing.
- `records.md` line 14 says only "Each line is one text record. A trailing newline ends the last record." Under line framing, an empty or white-space line stops the run at exit 2 with `the evidence is empty or blank`, from `core/text.rs:19`. A blank line in the middle drops every record after it, and a trailing `\n\n` fails after the last real line. Reports 01 (issue 2) and 11 (issue 1) ran it with spaces and with a CRLF blank line (`sdlc/issues/2026-09-26-a-blank-line-stops-a-lines-run.md`). `audit` skips blank lines and still counts them (`audit.md` line 41). `relate` refuses a blank line before any request (`relate.md` line 34), so it cuts nothing short. No test feeds a blank line under line framing.
- Today a record number is an input line number under line framing. `edge::Chunks` yields every line, a blank one included, as one item. The engine scheduler (`engine/schedule.rs:153-179`) numbers items in dispatch order and reports `at: place + 1`, and `engine/annotate_schedule.rs:383` reports `at: row + 1` the same way. A blank line is an item that fails, so it holds its place.
- `records.md` line 131 says a run that must stay inside the documented limit "sets `--jobs 3`". Experiment 206 measured 980 requests a minute at `--jobs 3` over 3,000 short lines. The reply time behind that rate was never measured. It is derived: 3 requests in flight at 980 a minute gives 3 × 60 / 980, about 0.18 seconds a reply. Report 07, finding I1, measured `--jobs 3` at 1,285 requests a minute on loopback with 100 ms replies, and the default `--jobs 4` at 1,519 a minute live (`sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`).
- Ticket 0146 rewrites the record loop and `records.md`'s request paragraphs for batching. After it, `decide`, `filter` and `rank` fill each request to the limit by default, a request carries a batch of records, and `--jobs` counts batches in flight. 301 short lines then form one request. Its batch reader in `cli/asking/batched.rs` holds each item's records and first record number, and `Outcome::Stopped` counts `finished` in records with `at` equal to `finished + 1`. 0146 pins its own closed-pipe tests at `--batch 1`.

## Design

### A closed pipe stops scheduling

Between dispatches, the record loop asks whether standard output is still open without writing to it. On Unix it polls the output descriptor with a zero timeout and reads an error or hang-up as a closed pipe. That takes the path a failed write takes today: the run stops reading and scheduling, lets requests already sent finish, and exits with the code it earned. `nix`, already a Unix dependency, gains its `poll` feature. No package is added. Standard output that is a file or a terminal never reports a hang-up, so those runs do not change. On a platform without the poll, the run keeps today's write-time check. The release targets are all Unix.

The check runs before each dispatch in the engine scheduler's loop, so it covers a batch at 0146's default, a batch at `--batch 1`, and every verb that shares the scheduler.

`records.md` line 141 keeps its sentence and adds: "The tool notices between requests, so a command such as `filter`, which prints only some records, stops as soon as the reader is gone."

### A blank line is skipped before dispatch

The coordinator's ruling, by ADR 0054 item 2: under line framing, the reader drops a line that is empty or holds only white space, a carriage return included, before anything else sees it. A blank line is then:

- never sent and never part of a request,
- never a member of a batch, so it never counts toward ADR 0053's cap of 4,096 members,
- never a row, so `rank`'s `Output::ended` never ranks it and `filter` never keeps it,
- never counted in "N records finished".

Record numbers keep today's meaning. Today a record number under line framing is the input line number, as "What happens today" shows. So the reader counts every line it reads, and each kept record carries its own line number. A stop line names line numbers wherever it names a record: `stopped at record N`, a batch's range `records A to B`, and the record a partial reply failed. "N records finished" counts kept records only. Other framings number records as today, because they skip nothing here.

Where it lives:

- `core/records.rs` gains `Reading::skips(&[u8]) -> bool`. It is true only under line framing, for a line whose text, after today's `ended` strips the line feed and carriage return, is valid UTF-8 and trims to nothing. The trim is `text.rs`'s rule, so "blank" means the same everywhere. A line that is not UTF-8 is not blank and is refused as today.
- `cli/edge.rs` gains one adapter over `Chunks` that numbers each chunk from 1 and drops the ones `Reading::skips` names. Every record reader that streams under line framing uses it: `cli/asking.rs` for the one-record-a-request verbs and the dry run, 0146's `cli/asking/batched.rs`, `cli/annotate.rs` with `cli/annotate_schedule.rs`, and `cli/recognize.rs` with `cli/recognize/dry_run.rs`. `find` and `relate` keep their own readers and rules.
- The scheduler's item carries its records' line numbers. `engine/schedule.rs` reports `Outcome::Stopped.at` as the line number of the record the run stopped at: an item's first record when the item failed whole, the named record of a partial stop, or the line the reader refused. `finished` keeps counting finished records. `engine/annotate_schedule.rs` does the same for `annotate`. `cli/asking/batched.rs` and `cli/failure.rs` take a batch's range from its first and last records' line numbers.
- `core/batch.rs` needs no logic change, because the planner never sees a blank line. Its doc comment on the member cap says a member is a kept record.

A dry run skips blank lines before it plans, as a run does. A stream of only blank lines is an empty stream: no request, no output, exit 0.

`records.md` line 14 becomes: "Each line is one text record. A line that is empty or holds only white space is skipped. It makes no request, joins no batch and prints nothing. It still takes its line number, so a stop line's record numbers are line numbers, and "records finished" counts only the records sent. A trailing newline ends the last record." `filter.md` line 15 and `rank.md` line 13 already say that their default framing reads lines. Each adds "A blank line is skipped, as [records.md](records.md) gives." The other verbs already defer to `records.md` for framing.

### `--jobs 3` names its reply time

`records.md` line 131's advice becomes a statement of the arithmetic, in the words 0146 leaves on the page. The rate is about `jobs` requests for each reply time. Experiment 206 measured 980 requests a minute at `--jobs 3` over short lines. That rate implies about 0.18 seconds a reply, which the page states as derived from the rate and not measured. A backend that answers in 0.1 seconds takes `--jobs 3` past 1,200 a minute, as report 07 measured on loopback at 1,285. Each number names its record. The page says that no setting caps requests a minute, and it points to the pacer issue.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A closed pipe is found by polling standard output between dispatches.** It needs no write and no new package. It adds one feature word to `nix`.
2. **A blank line under line framing is skipped before dispatch.** The coordinator ruled it by ADR 0054 item 2. A grep user expects a skip, `audit` already skips, and a refusal would still pay for a prefix of the input first. No working pipeline depends on today's stop, because it fails at exit 2.
3. **Record numbers stay line numbers, and "records finished" counts kept records.** The coordinator ruled it. The reader carries each record's line number, so a stop line names the line a user can find in the input. Carrying the number costs about 60 lines in the reader, the two schedulers and the stop line. The simpler rule renumbers kept records and costs nothing, but then "stopped at record 4" would name a line that is not line 4.
4. **The rule keys on line framing.** `filter` and `rank` read lines by default, so a flag test would miss their most common use.
5. **`--jsonl`, `relate` and `find` keep their rules.** Each already refuses before a request or has its own unit rule.
6. **The `--jobs 3` sentence becomes arithmetic.** The 0.18-second reply time is labelled derived. The pacer stays out of 0.1.

## Edge cases

Rows 1 to 4 run with `--jobs 4`. Loopback replies wait 20 ms (`Canned::ok(..).after(20)`), as `annotate/scheduling.rs:324` does, so a request stays in flight long enough to count.

| Row | Input | Expected |
| --- | --- | --- |
| 1 | `filter --batch 1` over `keep first` and 300 `drop N` lines, output into a reader that reads one line and closes | At most 12 requests. Exit 0 |
| 2 | The same at `--batch 10`, 31 batches | At most 12 requests. Exit 0 |
| 3 | `decide --jsonl` into a reader that closes after one line | It stops, as today |
| 4 | `filter --batch 1` over the same 301 lines into a file | 301 requests. The file holds `keep first`. Exit 0 |
| 5 | `printf 'keep a\n\nkeep c\n' \| filter` with no framing flag, `--batch 1` | Two requests. Standard output is `keep a` and `keep c`. Exit 0 |
| 6 | A line of spaces, a `\r\n` blank line, and a trailing `\n\n`, under `decide --lines --batch 1` | Each is skipped. One request for each other line. Exit 0 |
| 7 | 4,096 kept lines with ten blank lines among them, read by `decide --lines --no-cache --input FILE` at the default batch, so no pause cuts the batch | One request |
| 8 | `decide --lines --batch 1 --jobs 1 --max-retries 0` over `a`, blank, `c`, `d`, with status 503 on the third request | Rows for lines 1 and 3. Standard error is exactly the row 8 block below. Exit 4 |
| 9 | `decide --lines --batch 2 --jobs 1 --max-retries 0` over `a`, blank, `c`, `d`, blank, `f`, with status 503 on the second request | Batches hold lines 1 and 3, then 4 and 6. Rows for lines 1 and 3. Standard error is exactly the row 9 block below. Exit 4 |
| 10 | The same input and batches, with the second reply missing line 6's answer | Rows for lines 1, 3 and 4. Standard error is exactly the row 10 block below. Exit 4 |
| 11 | `decide --lines` over only blank lines | No request, no output, exit 0 |
| 12 | `decide --jsonl --field /body` over one good record and a blank line | One request. Standard error is exactly the row 12 block below. Exit 2, as today |

The whole standard error the tests pin. The 503 cause is today's sentence from `cli/failure/status.rs`, and the one-line and partial forms are 0146's templates.

Row 8:

```text
thinkthen: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries
thinkthen: stopped at record 4; 2 records finished
```

Row 9:

```text
thinkthen: stopped at record 4; the request for records 4 to 6 failed: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries; 2 records finished
```

Row 10:

```text
thinkthen: stopped at record 6; the reply for records 4 to 6 gave record 6 no usable answer; 3 records finished
```

Row 12:

```text
thinkthen: the record is not valid JSON
thinkthen: stopped at record 2; 1 record finished
```

## Proof

Every test drives the compiled binary against the in-process loopback in `tests/backend/harness`, which counts requests.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_reader_that_closes_the_pipe_stops_filter_between_dispatches`, new in `tests/backend/parallel.rs` beside `a_reader_that_closes_the_pipe_stops_the_reading_and_the_scheduling` | Edge rows 1, 2 and 4. It reuses the file's `piped` helper, which gains a verb and extra arguments. It holds the child's standard output as a real pipe, reads one line, drops the reader, waits for the exit, and counts the loopback's requests. The bound of at most 12 follows the existing test's `sent <= 12` at `--jobs 4`. Row 2's loopback answers yes only to the question that quotes `keep first`, with 0146's batching loopback | (a) Check the pipe only on write: row 1 counts 301 and row 2 counts 31. (b) Check the pipe only after a batch finishes, not between dispatches: row 2 counts 31. (c) Treat a file as closed: row 4 stops early |
| `a_blank_line_is_skipped_and_keeps_its_number`, new in `tests/backend/streaming.rs`, the file for framing, order and the stop | Edge rows 5 to 12, each pinning standard output, the whole standard error, the exit code and the request count | (d) Refuse the blank line as today: row 5 stops at exit 2. (e) Skip without numbering: row 8 says `stopped at record 3`, and row 9 says `records 3 to 4`. (f) Count a blank line as finished: row 8 says `3 records finished`. (g) Count a blank line as a batch member: row 7 sends 2 requests. (h) Number a partial stop by `finished + 1`: row 10 says `record 4`. (i) Skip under `--jsonl` too: row 12 exits 0 |

The pipe test runs at `--batch 1` and `--batch 10`, not at 0146's default. At the default the 301 short lines form one request, so plant (a) would stay green. Row 2's 31 batches prove the check runs between batch dispatches.

The four questions:

- **What behavior does it protect?** `records.md`'s closed-pipe promise for a command that prints some records, and the blank-line rule with its record numbers.
- **What credible regression fails it?** A pipe check that only a write can trip, a check that waits for a batch to finish, a blank line that stops the run again, a blank line that joins a batch, and record numbers that drift from line numbers.
- **Why does no existing test catch it?** The seven closed-pipe tests listed under "What happens today" each run a command that writes a row for every record or its one result at once. A failed write stops each of them, so none sees a pipe close while the command writes nothing. No test feeds a blank line under line framing.
- **Does it need a test-only hook?** No. The pipe is a real pipe and the loopback counts real requests. The reply delay is an ordinary server choice.

The pipe test goes beside the existing one in `parallel.rs`, so both share `piped` and the same bound. The blank-line test goes in `streaming.rs` beside the record-refusal table it inverts. Neither needs a new test file or a `mod` line.

## Budgets

Nonblank lines, measured with `grep -c .`, net against main after 0146 lands.

- `crates/thinkthen/src/cli/edge.rs`: at most 35 net, for the poll and the numbering adapter.
- `crates/thinkthen/src/core/records.rs`: at most 12 net, for `Reading::skips`.
- `crates/thinkthen/src/engine/schedule.rs`: at most 20 net, for the line numbers and the check before each dispatch.
- `crates/thinkthen/src/engine/annotate_schedule.rs`: at most 10 net.
- `crates/thinkthen/src/cli/asking/batched.rs`: at most 25 net, for each held record's line number and the range.
- `crates/thinkthen/src/cli/schedule.rs`, `cli/asking.rs`, `cli/annotate.rs`, `cli/annotate_schedule.rs`, `cli/recognize.rs` and `cli/recognize/dry_run.rs`: at most 25 net together, for using the adapter and passing each line number.
- `crates/thinkthen/src/cli/failure.rs` and its folder: at most 5 net.
- `crates/thinkthen/src/core/batch.rs`: at most 3 net, the doc comment only.
- Product code total: at most 135 net.
- `tests/backend/parallel.rs`: at most 80 net, for the helper's two new parameters and the three rows.
- `tests/backend/streaming.rs`: at most 130 net.
- Unit tests that build a scheduler item or read `at`, in `engine/schedule.rs`, `engine/deadline_tests` and `cli/schedule`: at most 15 net.
- Pages: at most 20 net.
- `crates/thinkthen/Cargo.toml`: one feature word. `Cargo.lock`: no new package.
- `sdlc/ratchet.json` moves to the measured total in the commit that needs it. The budgets above bound the raise at about 360 lines above main after 0146. The commit says what grew and where it looked for duplication, and the second agent reviews it.
- No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a package to `Cargo.lock`.
2. Stop if polling needs `unsafe` code. The crate forbids it.
3. Stop if a run whose output is a file or a terminal changes.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.
6. Stop if ticket 0146 has not landed.
7. Stop if carrying line numbers needs a second scheduler or changes a stop line at `--batch 1` over input with no blank line. Hand back the simpler rule under "What Ian can overturn" for a ruling.

## Build order

It builds after ticket 0146 lands, because 0146 rewrites the record loop, `cli/edge.rs`, `cli/asking/batched.rs`, `engine/schedule.rs` and `records.md`. Ticket 0158 lands before 0146 starts, by 0146's order, so 0158 is on main before 0162 builds. The two share `specification/records.md` and `sdlc/ratchet.json`, and 0162 builds on 0158's lines. It never builds beside tickets 0153 or 0154, which open `cli/args.rs`, `cli/edge.rs`, `cli/asking/batched.rs`, `cli/recognize.rs`, `cli/failure.rs`, `core/batch.rs` and `records.md`. The coordinator places it before or after them. It may build beside ticket 0163.

## Scope and exclusions

Excluded: a requests-per-minute pacer, `relate`'s and `find`'s line rules, blank lines under `--jsonl`, CSV and TSV rows, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code. A second agent reviews the final diff for the ceiling raise and the `nix` feature, by `AGENTS.md` line 11, and names what it checked.

## Complexity

Contract 2; state and timing 2; reach 2; proof 2; cost of error 1; total 9. Final level: 2. The timing risk is a poll that reports a hang-up on a live pipe, which row 4 and the existing `decide` test guard. The reach risk is a record number that drifts in one of the two schedulers, which rows 8 to 10 guard.

## Deferred gaps

- The pacer itself. `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md` keeps it past 0.1.
- The closed-pipe check on platforms without `poll`. None is a release target.
- A measured reply time for experiment 206's short lines. The page states the derived figure.

## What Ian can overturn

- Decision 2: a blank line under line framing is skipped before dispatch, in place of a clearer refusal. The coordinator ruled it by ADR 0054 item 2.
- Decision 3: record numbers stay line numbers and "records finished" counts kept records. The coordinator ruled it. The simpler rule numbers kept records only, and a stop line then names a record that is not its input line.
- Decision 5: `--jsonl` keeps refusing a blank line.

## Closes

`sdlc/issues/2026-09-26-filter-keeps-sending-after-the-reader-closes-the-pipe.md` and `sdlc/issues/2026-09-26-a-blank-line-stops-a-lines-run.md`. The wording item of `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`, which then holds only the pacer.

## Evidence

- Starts from: Local experiment 273, report 01 issues 1 and 2, report 11 issue 1, and report 07 finding I1, as the three issues record them. The code at `origin/main` `8084d38a`: `cli/edge.rs:278`, `core/text.rs:19`, `engine/schedule.rs:153-179`, `engine/annotate_schedule.rs:383` and `cli/asking.rs::read_by`. The seven closed-pipe tests on main. `records.md` lines 14, 131 and 141, `audit.md` line 41 and `relate.md` line 34. Experiment 206's rate as `records.md` cites it. Ticket 0146's batch reader, stop lines and `--batch 1` pins, read from `origin/ticket/0146-command-batches-decide-filter-rank`. ADR 0053 item 2 and ADR 0054 item 2.
- Keeps: Every run whose output is a file or a terminal. `decide`'s closed-pipe stop. Record numbers as line numbers. `--jsonl`, `relate` and `find` line rules. Output order and exit codes. Every stop line over input with no blank line.
- Changes: A closed pipe stops scheduling between dispatches. A blank line under line framing is skipped before dispatch, joins no batch, prints no row and counts in no total, and keeps its line number. The `--jobs 3` sentence states its arithmetic and labels its reply time derived.
- Proof: Two outside-in tests with nine plants, beside the existing closed-pipe and record-refusal tests, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The pacer, the check on platforms without `poll`, and a measured reply time.
