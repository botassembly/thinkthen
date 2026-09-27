---
flow: build
priority: 162
opens: crates/thinkthen/Cargo.toml sdlc/scripts/policy.py crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/annotate_schedule.rs crates/thinkthen/src/cli/recognize.rs crates/thinkthen/src/cli/recognize/dry_run.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/core/records.rs crates/thinkthen/src/engine/schedule.rs crates/thinkthen/src/engine/annotate_schedule.rs crates/thinkthen/src/engine/deadline_tests/schedule.rs crates/thinkthen/src/engine/facade_tests.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/backend/closed_pipe.rs crates/thinkthen/tests/backend/blank_lines.rs specification/records.md specification/filter.md specification/rank.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0162: A record stream ends cleanly

Status: done on 2026-09-27. Fresh code and independent ceiling/dependency/policy reviews accepted `66da9fb1`; focused checks passed. Owner: Codex.

2026-09-27 build amendment: The closed-pipe and blank-line compiled-binary regressions failed for the named behaviors before the fix and pass now. A third compiled-binary table pins the stop line after a skipped blank and the unchanged JSONL refusal. Fresh code review found that the batch reader waited forever for another record after the first kept row if standard input stayed open and the output reader closed. A fourth compiled-binary regression reproduced the hang with a two-second deadline before the correction. The coordinator authorized routine budget amendments within the accepted outcome. The measured file and total budgets below replace estimates that understated the command-side numbering and its shared adapter. The coordinator claimed `engine/annotate_schedule/tests.rs` for its obsolete `at` pattern. Ian's priority change keeps the proof focused on observable behavior; this build does not run a mutation or stress campaign.

Review route: a fresh read-only Codex reviewer checks the final diff. A second reviewer also checks the source size ceiling, the `poll` feature on `nix`, and the exact table in `sdlc/scripts/policy.py`. `AGENTS.md` asks for that review on these changes, and the second reviewer names what it checked.

## Outcome and authority

Three things about a record stream start to hold.

1. When the reader downstream closes the pipe, `filter` stops scheduling at once, as `specification/records.md` line 141 promises. Today it keeps paying until its next kept record.
2. A blank line under line framing no longer cuts the run short. The reader skips it before any request, and record numbers still name input lines.
3. `records.md`'s advice to set `--jobs 3` names the reply time it assumes, so the request rate it implies is honest.

Each issue blocks 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The blank-line issue leaves the rule to the queue owner. The coordinator ruled it on 2026-09-26 under ADR 0054 item 2, "Filter at the earliest step", and this ticket builds that ruling. The coordinator also ruled, after the second review, that the pipe check and the line numbers live on the command's side of the scheduler. After the third review, the coordinator allowed the build to delete the engine's `at` field, which nothing reads once the command numbers its own records. After the fourth review, the coordinator placed the blank-line adapter in `cli/asking.rs::run` before 0146's split, so every asking verb numbers its records from one feed. Ian can overturn each choice.

Line framing means each line is one text record. `--lines` asks for it, and `filter` and `rank` use it by default when no pointer is given (`cli/asking.rs::read_by`). Every rule below keys on line framing, never on the typed flag.

## What happens today

Read from `origin/main` `6b9476c1`.

- `records.md` line 141: "When the program downstream closes the pipe, the tool stops reading and stops scheduling." The command learns of a closed pipe only when a write fails: `cli/edge.rs:278`, `write_line`, returns `false` on `BrokenPipe`. `filter` writes only the records it keeps. Local experiment 273, report 01, issue 1, fed one `keep first` line and 300 `drop N` lines on loopback. `filter ... | head -1` sent 301 requests, and `decide --lines | head -1` sent 5.
- Seven tests on main already close the output pipe: `tests/backend/parallel.rs:359`, `table.rs:470`, `scheduling.rs:261`, `tag/matrix.rs:245`, `annotate/scheduling.rs:324`, `find.rs:283` and `relate.rs:311`. Each runs a command that writes a row for every record or writes its one result at once. So a write fails right after the reader closes, and none of them sees a pipe close while the command writes nothing. `annotate/scheduling.rs:384` pins that a backend failure after the write fails stays quiet at exit 0.
- `records.md` line 14 says only "Each line is one text record. A trailing newline ends the last record." Under line framing, an empty or white-space line stops the run at exit 2 with `the evidence is empty or blank`, from `core/text.rs:19`. A blank line in the middle drops every record after it, and a trailing `\n\n` fails after the last real line. Reports 01 (issue 2) and 11 (issue 1) ran it with spaces and with a CRLF blank line (`sdlc/issues/closed/2026-09-26-a-blank-line-stops-a-lines-run.md`). `audit` skips blank lines and still counts them (`audit.md` line 41). `relate` refuses a blank line before any request (`relate.md` line 34), so it cuts nothing short. No test feeds a blank line under line framing.
- Today a record number is an input line number under line framing. `edge::Chunks` yields every line, a blank one included, as one item. The engine scheduler (`engine/schedule.rs:153-179`) numbers items in dispatch order and reports `at: place + 1`, and `engine/annotate_schedule.rs:383` reports `at: row + 1` the same way. A blank line is an item that fails, so it holds its place. CSV and TSV rows come from `table::Rows` one data row an item, so a table's record number is its data row counted from 1 after the header.
- Two command files read the engine's `at`: `cli/schedule.rs:183-190` and `cli/annotate_schedule.rs:74-80` copy it from `Outcome::Stopped` into `Failure::Stopped`. The library facade matches the same variant in `public/batch.rs:148` as `Stopped { cause, .. }` and never reads `at`. Outside those two command files, only engine unit tests read it: the tests inside `engine/schedule.rs` and `engine/annotate_schedule.rs`, `engine/deadline_tests/schedule.rs`, and `engine/facade_tests.rs:416`.
- Four command-side readers answer the scheduler's asks. `cli/schedule.rs::read_records` (line 200) serves `over_records`, which runs the one-record verbs, `recognize` over lines, and `recognize` over a table. `cli/annotate_schedule.rs::read` serves `annotate`. Each answers one ask with one item from its iterator. The dry runs read their first record with `chunks.next()` in `cli/asking.rs::run`, `cli/annotate.rs`, and `cli/recognize/dry_run.rs`.
- `records.md` line 131 says a run that must stay inside the documented limit "sets `--jobs 3`". Experiment 206 measured 980 requests a minute at `--jobs 3` over 3,000 short lines. The reply time behind that rate was never measured. It is derived: 3 requests in flight at 980 a minute gives 3 × 60 / 980, about 0.18 seconds a reply. Report 07, finding I1, measured `--jobs 3` at 1,285 requests a minute on loopback with 100 ms replies, and the default `--jobs 4` at 1,519 a minute live (`sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`).
- Ticket 0146 adds `cli/asking/batched.rs`. `cli/asking.rs::run` sends `decide`, `filter` and `rank` over a stream there at every `--batch`, `--batch 1` included, and CSV and TSV take the same path through `over_table` (0146, "The reader builds batches"). Its batch reader answers each scheduler ask with a batch, holds each item's records and first record number, and runs the record-mode dry run. `Outcome::Stopped` then counts `finished` in records, with `at` equal to `finished + 1`. After 0146, `cli/schedule.rs::read_records` serves only `choose`, `tag`, `score` and `recognize`, and each of them prints one line for every record.
- `sdlc/scripts/policy.py:287-293` accepts `nix` as a Unix library dependency only with `default-features = false` and `features = ["signal"]`. The test dependency keeps `["pthread", "signal"]`.

## Design

### A closed pipe stops scheduling

No route without a new feature gives this behavior. A zero-length write to a pipe whose reader has gone succeeds on Linux: `write(fd, "", 0)` returns 0 and raises no `EPIPE`. A one-byte write does raise `EPIPE`, but it puts a byte on standard output, and `filter` must print only kept records. The `EPIPE` path the tool already handles fires only on a kept record, which is today's bug. Local checks on Linux confirmed both results: the zero-length write returned 0, and `poll` on the same descriptor returned `POLLERR`. So the command polls.

`cli/edge.rs` gains a small `Downstream` value that the reader and the output share. Its `gone()` polls standard output through `nix::poll::poll` with a zero timeout, requesting `POLLOUT`. It reads `POLLERR` or `POLLHUP` in the returned events as a closed pipe. Once it sees one, it latches and never polls again. The call is safe code. `nix` gains its `poll` feature, which pulls in no package. Standard output that is a file or a terminal never reports either flag, so those runs do not change. On a platform without the poll, `gone()` is always false, and the run keeps today's write-time check. The release targets are all Unix. The Linux checks above are the only evidence of what `poll` reports. macOS ships, and no check has shown that its `poll` reports a pipe whose reader closed. Where the poll cannot see the close, `gone()` stays false and the run stops at its next write, as today. The build does not claim macOS behavior, and Deferred gaps holds its verification.

The check lives in the command's reader that answers each scheduler ask. Before 0146's batch reader in `cli/asking/batched.rs` answers an ask, it calls `gone()`. When the pipe is gone, it answers `Input::End` and pulls no more records. The output's `take` does not read the latch. A write to a closed pipe already fails, and `write_line`'s `BrokenPipe` arm returns `false`. The stop mapping reads the latch. A stop that the run reports after the latch is set is quiet at exit 0, as `annotate/scheduling.rs:384` pins for a failed write today. Requests already sent still finish, and `--record` still writes what was billed.

`cli/schedule.rs::read_records` does not gain the check. After 0146 it serves only `choose`, `tag`, `score` and `recognize`, and each prints a line for every record, so the first write after the close already stops them. No test could tell a check there from none. `cli/annotate_schedule.rs::read` keeps today's write-time stop for the same reason. `rank` holds its rows until the end, so the check lets it stop reading and paying when its reader closes early. The pipe check adds no engine, facade, public or library code, and a library run never polls the command's standard output.

`records.md` line 141 keeps its sentence and adds: "Where the platform reports the closed pipe, the tool notices between requests, so a command such as `filter`, which prints only some records, stops as soon as the reader is gone. Where it does not, the tool stops at its next write." Each clause holds on every platform, so the page makes no claim about macOS.

### A blank line is skipped before dispatch

The coordinator's ruling, by ADR 0054 item 2: under line framing, the reader drops a line that is empty or holds only white space, a carriage return included, before anything else sees it. A blank line is then:

- never sent and never part of a request,
- never a member of a batch, so it never counts toward ADR 0053's cap of 4,096 members,
- never a row, so `rank`'s `Output::ended` never ranks it and `filter` never keeps it,
- never counted in "N records finished".

`core/records.rs` gains `Reading::skips(&[u8]) -> bool`. It is true only under line framing, for a line whose text, after today's `ended` strips the line feed and carriage return, is valid UTF-8 and trims to nothing. The trim is `text.rs`'s rule, so "blank" means the same everywhere. A line that is not UTF-8 is not blank and is refused as today.

`cli/edge.rs` gains one adapter over `Chunks`. It numbers each chunk from 1, drops the chunks `Reading::skips` names, and yields each kept chunk or read error with its number. Every reader that streams under line framing uses it. `cli/asking.rs::run` applies it once, where it builds `Chunks`, before the dry-run branch and before 0146's split. That one feed serves the asking dry run, 0146's `batched.rs` for `decide`, `filter` and `rank`, and `over_records` for `choose`, `tag` and `score`. So every asking verb's items get their numbers from one place. `cli/annotate.rs` uses the adapter to feed `cli/annotate_schedule.rs` and its dry run. `cli/recognize.rs` uses it to feed `over_records`, and `cli/recognize/dry_run.rs` uses it for its dry run. A dry run skips blank lines before it plans, as a run does. `find` and `relate` keep their own readers and rules. `core/batch.rs` needs no change, because the planner never sees a blank line.

A stream of only blank lines is an empty stream: no request, no output, exit 0.

### Record numbers ride on the command's items

Record numbers keep today's meaning, and the command owns them. The engine's `finished` does not change.

How each framing numbers its records:

- Line framing: the input line number. The adapter counts every line it reads, blank ones included, so a kept record carries its own line number.
- JSONL: the input line number, as today. The adapter skips nothing, so the number equals today's item count. A blank JSONL line is still refused.
- One document: record 1, as today.
- CSV and TSV, whether through `over_records` for `recognize`, 0146's `over_table` for `decide`, `filter` and `rank`, or `annotate`: the data row counted from 1 after the header, in the order `table::Rows` yields it, as today. Each reader numbers the rows with `zip(1..)`. Nothing is skipped, because the blank-line rule applies only under line framing.

Where the number travels:

- Each reader's item carries the numbers of its records. An item of `over_records` carries its one number. A 0146 batch holds each record's number beside the record. `annotate`'s record and each of its group works carry the record's number.
- `cli/schedule.rs` gains `Placed`, the error type the command hands the engine. It holds a `Failure` and the line it names, when the command made it. `From<engine::Error>` builds it with no line. The worker closures in `over_records`, `batched.rs` and `annotate_schedule.rs` wrap each error with their item's number: the first record's number when the item failed whole, and the named record's number for a partial stop through 0146's `Completed.stop`. A reader wraps a refusal with the refused line's number.
- When the run stops, the command builds `Failure::Stopped` with `at` taken from the cause's line. A cause the engine made, a cancellation or a defect, has no line. Its `at` is the line after the last record the output took, or 1 when none finished. With no blank line in the input, every one of these equals today's `finished + 1`, so every stop line over such input keeps its bytes.
- `finished` stays the engine's count. It counts records the output took, and a blank line is never one.
- 0146's batch stop line takes its range `records A to B` from the first and last records' line numbers.

Once the command takes `at` from its own items, no product code reads the engine's `at`. Rustc then warns that the field is never read, and `clippy -D warnings` fails `lint`. The coordinator ruled that the build deletes `at` from `Outcome::Stopped` in `engine/schedule.rs` and `engine/annotate_schedule.rs`. It also deletes `at` from the engine unit tests that match it, in those two files, `engine/deadline_tests/schedule.rs` and `engine/facade_tests.rs`. The test at `engine/facade_tests.rs:416-419` keeps its stop-position check. It asserts `finished: 2` in place of `at: 3`, because `at` always equalled `finished + 1`. The engine loses only that field and gains no behavior. `public/batch.rs` never reads `at`, so no facade, public or library file changes.

Stop lines name line numbers wherever they name a record: `stopped at record N`, a batch's range, and the record a partial reply failed.

`records.md` line 14 becomes: "Each line is one text record. A line that is empty or holds only white space is skipped. It makes no request, joins no batch and prints nothing. It still takes its line number, so a stop line's record numbers are line numbers, and "records finished" counts only the records sent. A trailing newline ends the last record." `filter.md` line 15 and `rank.md` line 13 already say that their default framing reads lines. Each adds "A blank line is skipped, as [records.md](records.md) gives." The other verbs already defer to `records.md` for framing.

### `--jobs 3` names its reply time

`records.md` line 131's advice becomes a statement of the arithmetic, in the words 0146 leaves on the page. The rate is about `jobs` requests for each reply time. Experiment 206 measured 980 requests a minute at `--jobs 3` over short lines. That rate implies about 0.18 seconds a reply, which the page states as derived from the rate and not measured. A backend that answers in 0.1 seconds takes `--jobs 3` past 1,200 a minute, as report 07 measured on loopback at 1,285. Each number names its record. The page says that no setting caps requests a minute, and it points to the pacer issue.

### The policy table widens by one feature

`sdlc/scripts/policy.py:287-293` pins the Unix `nix` features to exactly `["signal"]`, so the `poll` feature fails `lint`. The build changes that check to accept exactly `["poll", "signal"]`. Its comment gains "Ticket 0162: the record reader polls standard output for a closed pipe." Its message becomes "nix is a Unix library dependency with only its poll and signal features". `crates/thinkthen/Cargo.toml` line 50 lists the features in the same order. The coordinator authorized this edit as a named, reviewed change. It adds one feature to an exact list and weakens no ban. The target dependency set, the test dependency's `["pthread", "signal"]`, and every other table stay as they are. The second reviewer named in Routing checks it.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A closed pipe is found by polling standard output in the batch reader.** No write-based route works, as the design shows. It needs no package. It adds one feature word to `nix` and one to the policy table that pins it.
2. **The check lives only in 0146's batch reader.** The coordinator placed the check in the command's readers. The one-record and `annotate` readers serve only verbs that print a line for every record, so a check there changes nothing a test can see.
3. **A stop after the latch is quiet.** It matches what a failed write does today.
4. **A blank line under line framing is skipped before dispatch.** The coordinator ruled it by ADR 0054 item 2. A grep user expects a skip, `audit` already skips, and a refusal would still pay for a prefix of the input first. No working pipeline depends on today's stop, because it fails at exit 2.
5. **Record numbers stay line numbers, and "records finished" counts kept records.** The coordinator ruled it, and ruled that the numbers ride on the command's items and errors. The simpler rule renumbers kept records and costs nothing, but then "stopped at record 4" would name a line that is not line 4.
6. **The rule keys on line framing.** `filter` and `rank` read lines by default, so a flag test would miss their most common use.
7. **`--jsonl`, CSV, TSV, `relate` and `find` keep their rules.** Each already refuses before a request or has its own unit rule.
8. **The `--jobs 3` sentence becomes arithmetic.** The 0.18-second reply time is labelled derived. The pacer stays out of 0.1.
9. **The engine loses `at`.** The coordinator ruled it. Nothing but engine unit tests reads the field once the command numbers its records, and a dead field fails `lint`.
10. **Two new test files.** `tests/backend/parallel.rs` holds 429 nonblank lines and `streaming.rs` holds 487, and `lint` caps a file at 500. So the pipe rows go in `tests/backend/closed_pipe.rs` and the blank-line rows go in `tests/backend/blank_lines.rs`, each with a `mod` line in `tests/backend/main.rs`.

## Edge cases

Rows 1 to 4 run with `--jobs 4` unless a row names another width. Loopback replies wait 20 ms (`Canned::ok(..).after(20)`), as `annotate/scheduling.rs:324` does, so a request stays in flight long enough to count. `decide`'s closed-pipe stop keeps its existing test, `tests/backend/parallel.rs:358`.

| Row | Input | Expected |
| --- | --- | --- |
| 1 | `filter --batch 1` over `keep first` and 300 `drop N` lines, output into a reader that reads one line and closes | At most 12 requests. Exit 0 |
| 2 | The same at `--batch 10`, 31 batches | At most 12 requests. Exit 0 |
| 3 | `filter --batch 1` over the same 301 lines into a file | 301 requests. The file holds `keep first`. Exit 0 |
| 4 | `filter --batch 1 --jobs 3 --max-retries 0` over `keep first`, `drop 2`, `drop 3` and `drop 4`. The loopback answers line 1 after 100 ms, line 2 after 300 ms, line 3 with status 500 after 600 ms, and line 4 at once. The reader reads one line and closes | Exit 0. Standard error is empty. At most 4 requests |
| 5 | `printf 'keep a\n\nkeep c\n' \| filter` with no framing flag, `--batch 1` | Two requests. Standard output is `keep a` and `keep c`. Exit 0 |
| 6 | A line of spaces, a `\r\n` blank line, and a trailing `\n\n`, under `decide --lines --batch 1`, and the same input under `choose QUESTION yes no --lines` | The input is the bytes of `printf 'a\n   \nb\n\r\nc\n\n'`. Each blank line is skipped. Each run sends 3 requests, prints three rows for `a`, `b` and `c` on standard output, writes nothing to standard error, and exits 0 |
| 7 | The exact input below, read by `decide --lines --no-cache --input FILE` at the default batch | One request. Its body holds the fixed evidence sentence `Each question quotes the text it asks about.` and exactly 3 quoted questions. Standard output holds 4,096 rows. Exit 0 |
| 8 | `decide --lines --batch 2 --jobs 1 --max-retries 0` over `a`, blank, `c`, `d`, blank, `f`, with status 503 on the second request | Batches hold lines 1 and 3, then 4 and 6. Rows for lines 1 and 3. Standard error is exactly the row 8 block below. Exit 4 |
| 9 | The same input and batches, with the second reply missing line 6's answer | Rows for lines 1, 3 and 4. Standard error is exactly the row 9 block below. Exit 4 |
| 10 | `decide --lines` over only blank lines | No request, no output, exit 0 |
| 11 | `decide --jsonl --field /body` over one good record and a blank line | One request. Standard error is exactly the row 11 block below. Exit 2, as today |
| 12 | `annotate FILE --lines --jobs 1 --max-retries 0` over `a`, blank, `c`, where `FILE` holds one `decide` entry, so each record sends one request. The loopback answers 503 to the request that quotes `c` | Line 1's row. Standard error is exactly the row 12 block below. Exit 4 |
| 13 | `recognize person --lines --jobs 1 --max-retries 0` over `Ann`, blank, `Bob`. The loopback answers each request that quotes `Ann` with a valid reply and answers 503 to each request that quotes `Bob` | Line 1's row. Standard error is exactly the row 12 block below. Exit 4 |
| 14 | `decide --lines --dry-run` over a blank line, then `first` and `second` | No request. Exit 0. Standard output equals the same command's output over `first` and `second` alone |
| 15 | `annotate FILE --lines --dry-run` over the same input | The same three results |
| 16 | `recognize person --lines --dry-run` over a blank line, then `Ann met Bob` | The same three results against `Ann met Bob` alone |

Row 7's exact input is 4,106 lines. Kept line k, for k from 1 to 4,096, is `apple` when k leaves 1 on division by 3, `banana` when it leaves 2, and `cherry` when it leaves 0. One empty line follows kept lines 400, 800, and so on to 4,000, which makes ten blank lines. The batch planner hashes each record's JSON value, so `core/batch.rs` lines 188 to 190 hash `"apple"`, `"banana"` and `"cherry"` with their quotation marks. The first eight bytes of their SHA-256 digests, read big-endian, leave 20, 3,040 and 1,078 modulo `CUT`, which is 4,096. None is 0, so none is a content cut. Repeats add no bytes, so size never closes the batch. Reading a file never pauses. ADR 0053's cap closes the batch after exactly 4,096 records, and the empty open batch then sends nothing. Under ADR 0055 the one request carries the fixed sentence as its evidence and one quoted question for each of the three texts. If blank lines joined the batch, the cap would close the first batch early and a second request would carry the last 10 kept lines.

The whole standard error the tests pin. The 503 cause is today's sentence from `cli/failure/status.rs`, and the one-line and partial forms are 0146's templates.

Row 8:

```text
thinkthen: stopped at record 4; the request for records 4 to 6 failed: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries; 2 records finished
```

Row 9:

```text
thinkthen: stopped at record 6; the reply for records 4 to 6 gave record 6 no usable answer; 3 records finished
```

Row 11:

```text
thinkthen: the record is not valid JSON
thinkthen: stopped at record 2; 1 record finished
```

Rows 12 and 13:

```text
thinkthen: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries
thinkthen: stopped at record 3; 1 record finished
```

## Proof

Every new test drives the compiled binary against the in-process loopback in `tests/backend/harness`, which counts requests. These focused proofs replace the proposed eighteen-plant campaign after Ian's priority change.

| Test | Observable proof | Credible regression it catches |
| --- | --- | --- |
| `a_reader_that_closes_the_pipe_stops_filter_between_dispatches` | A real pipe closes after `filter` keeps the first of 301 lines. At both `--batch 1` and `--batch 10`, the command exits 0, stays quiet and sends at most 12 requests at `--jobs 4` | Waiting for the next kept write, or putting the check outside the batch reader, sends all 301 records. This test failed before the fix. |
| `a_closed_reader_ends_filter_while_its_input_remains_open` | The producer sends one kept line and keeps standard input open. The reader takes that line and closes standard output. The command must exit 0, stay quiet and send one request within the bounded wait | An indefinite `raw.recv()` after the first batch never reaches the downstream check. This test failed at its two-second deadline before the correction. |
| `a_blank_line_is_skipped_and_keeps_its_number` | `decide --lines --batch 1` sends exactly three requests for five lines with two white-space lines plus a trailing empty line. It prints three results, exits 0 and stays quiet | Treating a blank as a record or end-of-run fails or sends extra requests. This test failed before the fix. |
| `blank_lines_keep_stop_positions_and_jsonl_still_refuses_them` | After a skipped line, invalid UTF-8 stops at input line 3 with one finished record and one request. A blank JSONL line still stops at line 2 with exit 2, one finished record and one request. Both pin complete stderr and output counts | Renumbering only dispatched records reports line 2 in the first case. Skipping every blank regardless of framing lets the second case continue. |

The four questions:

- **What behavior does it protect?** The closed-pipe promise for `filter`, a blank line's lack of request or output, input-line stop numbering, and JSONL's unchanged refusal.
- **What credible regression fails it?** A check that waits for another kept write, a blank line sent as a record, numbering that counts only dispatched rows, or a blanket skip applied to JSONL.
- **Why does no existing test catch it?** The seven closed-pipe tests listed under "What happens today" each run a command that writes a row for every record or its one result at once. A failed write stops each of them, so none sees a pipe close while the command writes nothing. No test feeds a blank line under line framing.
- **Does it need a test-only hook?** No. The pipe is a real pipe and the loopback counts real requests. The reply delay is an ordinary server choice.

## Budgets

Nonblank lines, measured with `grep -c .`, net against main after 0146 lands.

- `crates/thinkthen/src/cli/edge.rs`: at most 45 net, measured +44. Polling standard output needs the latched check and a separate numbered adapter. I checked `Chunks` and the existing write path; neither can detect a closed pipe before another kept write or discard blank lines while retaining their input positions.
- `crates/thinkthen/src/core/records.rs`: at most 12 net, for `Reading::skips`.
- `crates/thinkthen/src/cli/schedule.rs`: at most 30 net, measured +30. `Placed` carries the command's line number through the engine without changing its library API. I checked the existing `Failure::Stopped` mapping and kept the shared `Placed` type there instead of repeating it in the three command readers.
- `crates/thinkthen/src/cli/asking/batched.rs`: at most 30 net, measured +23. The reader now uses the existing 50 ms batch pause as its bounded idle receive interval. An empty held batch does not close on timeout; it checks the downstream pipe and cancellation before waiting again. This changes no batch pause semantics and adds no polling thread or timer.
- `crates/thinkthen/src/cli/annotate_schedule.rs`: at most 20 net, for the numbered record and works, the wrapped errors, and `at` from the cause.
- `crates/thinkthen/src/cli/asking.rs`, `cli/annotate.rs`, `cli/recognize.rs` and `cli/recognize/dry_run.rs`: at most 30 net together, measured +30. Each command reader must preserve line numbers through its own dry run and table route. I checked the first-record reads and reused `edge::numbered` instead of copying skip logic.
- `crates/thinkthen/src/cli/failure.rs` and its folder: at most 5 net.
- `crates/thinkthen/src/engine/schedule.rs` and `engine/annotate_schedule.rs`: 0 net or fewer each. Each deletes the `at` field, the line that sets it, and the `at` in its own unit tests, about 7 lines together.
- `crates/thinkthen/src/engine/deadline_tests/schedule.rs`, `engine/facade_tests.rs` and `engine/annotate_schedule/tests.rs`: at most 0 net together. The measured changes are -5, +3 and -1. The `facade_tests.rs` pattern swaps `at: 3` for `finished: 2`; rustfmt expands its match by three lines.
- Command, core and engine source and unit-test code total: at most 150 net, measured +145. This remains within ten percent of the accepted +135 total. No file under `public/` or a library binding changes.
- `tests/backend/closed_pipe.rs`: at most 130, measured 127. The extra 55 lines keep the test's producer pipe open and bound the wait so the reviewed hang fails promptly. They reuse this file's answer builder; `tests/backend/harness::start` closes standard input before it returns and cannot exercise this boundary.
- `tests/backend/blank_lines.rs`: at most 225, measured 94.
- `tests/backend/main.rs`: 2 net, the two `mod` lines.
- Pages: at most 20 net.
- `crates/thinkthen/Cargo.toml`: one feature word. `Cargo.lock` does not change, because `nix`'s `poll` feature adds no package.
- `sdlc/scripts/policy.py`: at most 2 net, for the widened list, its comment and its message.
- `sdlc/ratchet.json` moves from main's 75,356 to the measured 75,724 nonblank Rust lines. The +368 comprises +145 in source and unit tests, +221 in the two compiled-binary test files, and +2 module lines. I checked the shared numbered adapter, the stop mapping, the idle receive, the existing child harness and the test loopback answer builders for duplication before raising the ceiling. The second agent reviews this accounting and the feature and policy changes.
- No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a package to `Cargo.lock`.
2. Stop if polling needs `unsafe` code. The crate forbids it.
3. Stop if a run whose output is a file or a terminal changes.
4. Stop if either initially red compiled-binary regression, the line-number and JSONL table, or the open-input closed-output regression fails after its fix.
5. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.
6. Stop if ticket 0146 has not landed.
7. Stop if carrying line numbers needs an engine change beyond deleting `at` from the two `Outcome::Stopped` variants and their unit tests, needs any change under `public/` or a library binding, or changes a stop line over input with no blank line. Hand back the simpler rule under "What Ian can overturn" for a ruling.
8. Stop if the policy edit needs any change beyond the `nix` feature list, its comment and its message.

## Build order

It builds after ticket 0146 lands, because 0146 rewrites the record loop, `cli/edge.rs`, `cli/asking.rs`, adds `cli/asking/batched.rs`, and rewrites `records.md`. Ticket 0158 is on main. 0162 builds on its lines in `specification/records.md` and `sdlc/ratchet.json`. Ticket 0161 lands before 0146 by 0146's order, so 0161 is on main before 0162 builds. 0162 edits 0161's `cli/annotate.rs` and `core/records.rs`, and row 15 compares against 0161's plan document. It never builds beside tickets 0153 or 0154, which open `cli/args.rs`, `cli/edge.rs`, `cli/asking/batched.rs`, `cli/recognize.rs`, `cli/failure.rs`, `core/batch.rs` and `records.md`. The coordinator places it before or after them. It may build beside ticket 0163.

## Scope and exclusions

Excluded: a requests-per-minute pacer, `relate`'s and `find`'s line rules, blank lines under `--jsonl`, CSV and TSV rows, a pipe check in the one-record and `annotate` readers, and `site/`. `recognize` over lines is in scope, and rows 13 and 16 cover it. `choose`, `tag` and `score` over lines are in scope. They share `run`'s one feed with `decide`, and row 6 covers `choose`.

## Routing

Builder: Codex in the lane the coordinator names. Reviewer: a fresh read-only Codex session for the code. A second agent reviews the final diff for the ceiling raise, the `nix` feature and the `policy.py` edit, by `AGENTS.md`, and names what it checked.

## Complexity

Contract 2; state and timing 2; reach 2; proof 2; cost of error 1; total 9. Final level: 2. The timing risk is a poll that reports a hang-up on a live pipe, which row 3 and the existing `decide` test at `parallel.rs:358` guard. The reach risk is a record number that drifts in one of the three stop paths or a reader left off the adapter, which rows 6 and 8 to 16 guard.

## Deferred gaps

- The pacer itself. `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md` keeps it past 0.1.
- The closed-pipe check on platforms without `poll`. None is a release target.
- macOS verification of the closed-pipe check. macOS ships, and only Linux checks show `poll` reporting a closed reader. Until a check runs on macOS, a macOS run may stop only at its next write, as today.
- A measured reply time for experiment 206's short lines. The page states the derived figure.
- A pipe check in the one-record and `annotate` readers. A verb that prints only some records there would need it.

## What Ian can overturn

- Decision 2: the pipe check lives only in 0146's batch reader.
- Decision 4: a blank line under line framing is skipped before dispatch, in place of a clearer refusal. The coordinator ruled it by ADR 0054 item 2.
- Decision 5: record numbers stay line numbers and "records finished" counts kept records. The coordinator ruled it. The simpler rule numbers kept records only, and a stop line then names a record that is not its input line.
- Decision 7: `--jsonl` keeps refusing a blank line.
- Decision 9: the engine loses `at`. The coordinator ruled it. Keeping it would need an allow on dead code or a reader the command does not need.

## Closes

`sdlc/issues/closed/2026-09-26-filter-keeps-sending-after-the-reader-closes-the-pipe.md` and `sdlc/issues/closed/2026-09-26-a-blank-line-stops-a-lines-run.md`. The wording item of `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`, which then holds only the pacer.

## Evidence

- Starts from: Local experiment 273, report 01 issues 1 and 2, report 11 issue 1, and report 07 finding I1, as the three issues record them. The code at `origin/main` `6b9476c1`: `cli/edge.rs:278`, `core/text.rs:19`, `cli/schedule.rs::read_records`, `cli/annotate_schedule.rs::read`, the `at` readers in `cli/schedule.rs:183-190`, `cli/annotate_schedule.rs:74-80` and `public/batch.rs:148`, `engine/schedule.rs:27-28` and `153-179`, `engine/annotate_schedule.rs:21-22`, `engine/annotate_schedule.rs:383`, `core/batch.rs:32-33` and `188-190`, `cli/asking.rs::read_by`, and `sdlc/scripts/policy.py:287-293`. The seven closed-pipe tests on main and `annotate/scheduling.rs:384`. `records.md` lines 14, 131 and 141, `audit.md` line 41 and `relate.md` line 34. Experiment 206's rate as `records.md` cites it. Ticket 0161's `annotate` dry run and its `--lines` refusal for an `on` group, read from `origin/ticket/0161-annotate-reads-what-it-names`. Ticket 0146's batch reader, dry run, stop lines, member-cap rows and `--batch 1` pins, read from `origin/ticket/0146-command-batches-decide-filter-rank`. ADR 0053 item 2 and ADR 0054 item 2. A local Linux check that a zero-length write to a widowed pipe returns 0 and that `poll` reports `POLLERR` on it.
- Keeps: Every run whose output is a file or a terminal. `decide`'s closed-pipe stop. The quiet end after a closed pipe. Record numbers as line numbers, and table numbers as data rows. `--jsonl`, CSV, TSV, `relate` and `find` rules. Output order and exit codes. Every stop line over input with no blank line. Every engine behavior. Every file under `public/`.
- Changes: A closed pipe stops the batch reader between dispatches. A blank line under line framing is skipped before dispatch and before a dry run plans, joins no batch, prints no row, counts in no total, and keeps its line number. Stop lines take their record number from the command's item. The `--jobs 3` sentence states its arithmetic and labels its reply time derived. `policy.py` accepts `nix` with `["poll", "signal"]`. The engine's `Outcome::Stopped` loses its unread `at` field in both schedulers.
- Proof: Four focused compiled-binary regressions in two new files. The first two failed before the first fix for their stated behaviors. The third pins a line-number and JSONL boundary. The fourth failed before the reviewed idle-wait correction and pins an open-input closed-output run. Strict formatting, policy, all-target Clippy and the focused backend tests run in the build lane. The coordinator runs required landing gates after independent review.
- Defers: The pacer, the check on platforms without `poll`, macOS verification of the closed-pipe check, a measured reply time, and the check in the other two readers.
