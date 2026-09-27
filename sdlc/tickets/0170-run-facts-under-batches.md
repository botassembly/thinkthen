---
flow: build
priority: 170
opens: sdlc/planning/adr/0048-records-batch-into-full-requests.md sdlc/planning/adr crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/find.rs crates/thinkthen/src/cli/mod.rs crates/thinkthen/src/cli/facts.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/annotate_schedule.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/batched.rs crates/thinkthen/src/engine/usage.rs crates/thinkthen/src/engine/usage crates/thinkthen/src/engine/request.rs crates/thinkthen/src/engine/error.rs crates/thinkthen/src/core/result.rs crates/thinkthen/src/result_json.rs crates/thinkthen/tests/backend/batching.rs crates/thinkthen/tests/backend/batching crates/thinkthen/tests/backend/facts.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/backend/interrupt.rs specification/records.md specification/result.md specification/channels.md specification/settings.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0170: Run facts under batches

Status: accepted. The coordinator accepted it on 2026-09-27 after a fresh read-only review, with the fixes that review named. Owner: Claude.

Review route: the builder follows the work plan: Claude now, or Codex after the handover. A fresh read-only session from the builder's vendor reviews the final diff.

## Outcome and authority

A user runs `thinkthen filter ... --facts < songs.txt`. The tool prints the kept titles on standard output, as today. The last line on standard error is one `thinkthen.run/1` object. It counts every record and every request the run made, dropped records included. A stopped run's line also says where it stopped, why, and whether a later retry can help. A script reads that one line and never parses prose. Under `--details`, a batched row also says which batch it rode in.

This is batching row B5 of `sdlc/issues/2026-09-26-batching-design.md`: "`meta.batch`, `--facts` and the `thinkthen.run/1` line", proof test 6. Ticket 0146 took the shares out of B5 and builds them. ADR 0048 items 9 and 10 give the shapes. ADR 0052 item 9 adds `retries` to the facts. ADR 0051 item 10 adds `split` to `meta.batch`. `sdlc/issues/2026-09-26-run-facts-b5-owe-cause-retryable-and-stop-record.md` asks for a cause, a retry flag and the stop record. It is Batch D item 4 of `sdlc/planning/work-plan-2026-09-27.md`.

Ian's rulings set the frame. Ian can overturn each.

- A finished run stays silent on standard error by default. `--facts` controls only what the command prints.
- The fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`, plus `retries` by ADR 0052.
- Simple beats clever. Everything broken gets fixed.

## Prior experiment evidence

- Local experiment 284, file 38: exit 4 covers a spent 429 and a missing key alike, so a script cannot tell whether to retry. Its fix names B5 as the vehicle.
- Local experiment 284, file 94: no command prints a run's own totals. Its acceptance shape: the facts line's totals equal the usage counters' change over the run.
- `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`, gap 5 and ask 2: `filter --details` and `rank --top` hide the tokens of records they do not print. A tally over 2,545 saved reply bodies found `usage` with both token counts in every body.
- `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` fixes the batch shapes this ticket reports.
- This ticket makes no paid call. The loopback reports usage in the same body shape the hosted backend uses, by the tally above. A paid run would prove nothing the loopback cannot.

## Which findings hold

Checked on `origin/main` `40783433`.

| Finding | Holds? | Evidence | This ticket |
| --- | --- | --- | --- |
| File 38, exit codes mix retryable and permanent failures | Yes | `cli/failure.rs::say` gives exit 4 to `NoKey`, `Transport`, `Status` and `Reply` alike. `specification/channels.md` line 14 says a script never parses standard error. Only the library has `Error::retryable()` (`public/error.rs:97`) | The facts line carries `cause` and `retryable`. The exit table does not change |
| File 94, no run total | Yes | No source file names `thinkthen.run` or `--facts`. `records.md` says a finished run prints nothing on standard error. `engine/usage.rs::Counters::snapshot` holds the process totals and nothing prints them | The facts line reads those totals |
| The B5 amendment issue | Yes | ADR 0048 item 10 lists no cause, retry signal or stop place | The facts line's `stopped` member |

## What happens today

Read from `origin/main` `40783433`, and from tickets 0146, 0154, 0155, 0162 and 0169 on their branches.

- `engine/usage.rs::Counters` sums `requests_sent`, `input_tokens`, `output_tokens` and `cache_answers` for the process. `engine/request.rs:100` counts each attempt before it is sent. Lines 139-153 count a cache answer and the tokens of a live reply that reported usage. A reply that reported none adds nothing, and nothing records that it was missing. Ticket 0155 adds `retries`.
- `cli/mod.rs::entry` runs one command, reports its failure, flushes the usage counters, then lets `interrupt` finish. Ticket 0169 makes that last step re-raise SIGINT or SIGTERM.
- `cli/failure.rs::say` maps each `Failure` to an exit code and one sentence. `stopped` adds the stop line. Ticket 0162 makes the stop line's number the input line, so a skipped blank line keeps its number.
- Ticket 0146 gives each batched row an even share of its batch's tokens and attempts. Its rows carry no batch description. `meta.batch` is excluded there.
- Ticket 0154 halves a batch refused as too large, once. It leaves `meta.batch.split` to this ticket.
- `specification/result.md` lines 38 and 108 and `records.md` line 109 carry the ADR 0048 item 9 and item 10 markers. `channels.md` lines 14 and 32 carry the item 10 markers.

## Retained behavior

- Without `--facts`, every run prints exactly today's standard output, standard error and exit code.
- Every exit code stays. A missing key still exits 4, as `specification/check.md` fixes.
- `--batch 1` rows keep today's bytes. `meta.batch` is absent from a batch of one record with no context.
- The usage file keeps its schema. Nothing this ticket adds is written to it.
- The libraries and SQL surfaces behave as today. B12a builds library `facts`.

## The change

### The facts line

`--facts` joins `Common`, so every verb that asks the backend takes it: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. `FindCommon` gains it too. It sits in the long help only, by ADR 0048's call 18. `check`, `status`, `cache`, `audit`, `diff` and `transform` do not take it.

With `--facts`, the command writes one compact JSON line to standard error as its last line. It prints after the stop line and the usage-counter warning, and before the command re-raises a stopping signal. Keys appear in this order:

```json
{"schema":"thinkthen.run/1","records":25,"requests_sent":3,"retries":0,"cache_answers":0,"input_tokens":300,"output_tokens":30,"seconds":0.041,"model":"jev-1.13.0"}
```

| Field | Holds |
| --- | --- |
| `records` | The results the run finished, as the stop line counts them. One for a finished run on one document, for `find` and for one `relate` set. Zero under `--dry-run` and for a failed run on one document |
| `requests_sent` | Every attempt this process sent, from the usage counters |
| `retries` | The attempts that were retries, from ticket 0155's counter |
| `cache_answers` | Requests a cache answered, from the usage counters |
| `input_tokens`, `output_tokens` | The sums over live replies, from the usage counters. Present only when at least one live reply arrived and every live reply reported usage. Never 0 for a count nobody reported |
| `seconds` | Wall time from the start of `entry` to the line, in seconds with three decimals. It never enters a byte-identity check |
| `model` | The model every reply the run used named, live or from disk. Absent when no reply arrived or when replies named more than one model |
| `stopped` | Present only when the run failed. See below |

The counters are process totals, and one command is one process, so they are the run's totals. That makes file 94's acceptance hold by construction.

**What the counters gain.** `Counters` keeps three more values in memory: live replies that reported usage, live replies that reported none, and the model the replies named (none, one, or more than one). `engine/request.rs` feeds them where it already counts tokens and cache answers. None of them enters the usage file or `Counts`.

**Counting records.** The command counts finished records where it already takes finished rows, in `cli/schedule.rs` and `cli/annotate_schedule.rs`. A `filter` row it drops still counts. A run on one document counts 1 when it succeeds. The command reads `--dry-run` from its arguments and prints 0. If a batched item's rows do not pass one by one through the taking point, the builder adds `finished` to the engine's `Outcome::Complete` instead and records why.

### A stopped run says why and whether to retry

When the run ends in a failure, the line gains `stopped`:

```json
"stopped":{"at":11,"cause":"status","status":503,"retryable":true}
```

- `at` is the number the stop line prints. A failed run on one document has `at` 1.
- A run a signal stopped has no `at`. After ticket 0169 its stop line names no record, so the facts line names none either. A second signal ends the command at once and prints no facts line.
- `status` appears only beside the cause `status`.
- A run that exits 6 finished with a partial result. It has no `stopped` member.

`cause` is one word from this fixed list. The mapping lives in one function beside `say` in `cli/failure.rs`, and it reads the same `Failure` that sets the exit code.

| `cause` | Failures | Exit |
| --- | --- | --- |
| `usage` | Every failure that exits 2, including a refused record | 2 |
| `local` | Every failure that exits 5, including a replay miss | 5 |
| `no_key` | The key variable is unset or blank | 4 |
| `transport` | The backend could not be reached, or the attempt timed out | 4 |
| `status` | The backend answered with a status that is not a success and not a too-large refusal, after its retries | 4 |
| `too_large` | A too-large refusal, as ticket 0154's `Error::too_large()` reads it: status 413, or status 400 naming `max_tokens_exceeded`, after the one halving | 4 |
| `reply` | The adapter refused the reply, the reply passed its size limit, or a record got no usable answer | 4 |
| `backend` | Any other failure that exits 4 | 4 |
| `cancelled` | SIGINT or SIGTERM stopped the run | 130 or 143 |
| `defect` | A defect in the tool | 70 |

`retryable` is true only for the cause `status` with a retried status: 429, 500, 502, 503, 504 or 529. That is the rule `engine/error.rs` retries by and `Error::retryable()` reports. `engine/error.rs` gains one `pub(crate)` function that says whether a status is retried. It reads `RETRIED`, and `Error::retryable()` calls it, so the list is written once. A transport failure reads false, because the attempt may have reached the backend, as the library says.

### `meta.batch` under `--details`

A batched row's `meta` gains `batch`, by ADR 0048 item 9. It follows `profile_warning`, in the order the `result.md` table gives:

```json
"batch":{"setting":10,"records":10,"position":1,"closed":"size","usage":{"input_tokens":624,"output_tokens":175},"requests_sent":1}
```

- `setting` is the run's batch setting, a number or `"max"`. `records` counts the batch's members. `position` counts from 1. `closed` is `content`, `size`, `limit`, `pause` or `end`.
- `usage` is the batch's whole reported usage, absent when the backend reported none. `requests_sent` is the batch's attempts.
- `meta.batch` is absent when the batch holds one member and no context. A batch of two or more members that share one text sends today's single request, by ADR 0055 item 3, but each of its rows carries `meta.batch` with `records` equal to the member count.
- A row that a half of ticket 0154's split answered carries `"split":true` as the last member. Its `records`, `position`, `usage` and `requests_sent` describe the half. `closed` keeps the whole batch's reason. A half of one record carries `meta.batch` too, by ADR 0051 item 10.

`core/result.rs` gains the `Batch` member of `Meta` as a pure value. `cli/asking/batched.rs` fills it from the batch it already walks.

### The ADR

ADR 0048 says a change to one of its items takes a new ADR. The build writes one, numbered with the next free number in the builder's range: up to 0079 for Claude, 0080 to 0099 for Codex. It adds `stopped` to item 10 with the cause list and the retry rule above. It records decisions 1 to 4 below. ADR 0048 item 10 gains `(Amended by ADR NNNN.)`.

### Pages

- `records.md` line 109: the item 10 marker becomes the rule. It names `--facts`, the last-line placement and the `stopped` member.
- `result.md`: the `meta` paragraph and the `batch` row lose their item 9 markers. A new section, "The run facts line", holds the table above and the cause table.
- `channels.md`: the standard error row reads "Diagnostics for a person, never parsed by a script, except the one `--facts` line". Line 32 moves `--facts` into the advanced options list.
- `settings.md`: `--facts` moves from "Settings on the way" into the table. Its library and SQL cells read "not on this surface" and name B12a and the SQL deferral.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The exit table stays, and the facts line carries the retry signal.** A new exit code for retryable failures would change the Settled exit table, `check.md`'s exit 4 for a missing key, and every script that reads 4. A script that needs the signal passes `--facts`, as the run-facts issue asks.
2. **A missing key keeps exit 4 and reads `no_key`.** File 38 suggests moving it to another class. The facts line already separates it from a rate limit, which is file 38's success criterion. Moving the code breaks `check.md` for no gain.
3. **`retryable` uses the library's rule.** A transport failure reads false. A script that wants to requeue a timeout reads the cause `transport`.
4. **The cause list is ten words keyed to the exit code.** A finer list, one word per sentence, would grow with every new message. These ten answer "what next" for a script.
5. **The facts come from the process counters.** The rows undercount under `filter` and `rank --top`. The counters see every attempt, cache answer and reply.
6. **`--facts` goes on `Common`.** Every asking verb takes it with one flag. The batch verbs gain nothing special.
7. **The line is the last on standard error, and it prints before a signal is re-raised.** A script reads the last line. A signal must not eat it.
8. **Model and token presence follow one rule.** A field no reply reported, or that replies disagree on, is absent. The line never guesses.

## Edge cases

| Input | Expected |
| --- | --- |
| Any run without `--facts` | Today's bytes and exit code |
| `filter --facts` over 25 lines at `--batch 10`, 11 kept, loopback usage 100 and 10 a request | 11 lines out. Facts: `records` 25, `requests_sent` 3, `retries` 0, `cache_answers` 0, `input_tokens` 300, `output_tokens` 30 |
| `rank --top 5 --facts` over 20 lines | 5 lines out. Facts: `records` 20 |
| The same filter run replayed from its folder | Facts: `requests_sent` 0, no token fields, `model` present |
| The same run twice under `--cache DIR` | The second run's facts: `requests_sent` 0, `cache_answers` 3, no token fields |
| A backend that reports no usage | No token fields |
| One reply with usage and one without | No token fields |
| `decide --facts` on one document | `records` 1 |
| `decide --facts --dry-run` | The plan on standard output. Facts: `records` 0, `requests_sent` 0 |
| 503 on the second batch after one retry, `--batch 10 --max-retries 1` | Rows 1 to 10. The stop line, then facts with `records` 10, `retries` 1, and `"stopped":{"at":11,"cause":"status","status":503,"retryable":true}`. Exit 4 |
| Status 401 | `"cause":"status","status":401,"retryable":false`. Exit 4 |
| Key unset | `"stopped":{"at":1,"cause":"no_key","retryable":false}`. Exit 4 |
| Connection refused | `"cause":"transport","retryable":false`. Exit 4 |
| A malformed JSONL line at record 7 | Rows 1 to 6. `"stopped":{"at":7,"cause":"usage","retryable":false}`. Exit 2 |
| `--replay` of a folder missing a batch | `"cause":"local"`. Exit 5 |
| `decide --lines --jobs 1` with its one request held, SIGTERM after the loopback counted 1 request | `thinkthen: stopped by a signal; 0 records finished`, then facts with `records` 0, `requests_sent` 1 and `"stopped":{"cause":"cancelled","retryable":false}`. The command ends by SIGTERM, 143 |
| A second signal during that wait | The command ends at once. No facts line |
| A batch of one record refused with 413 | `"stopped":{"at":1,"cause":"too_large","retryable":false}`. Exit 4 |
| The usage folder cannot be written, because a file stands where the folder should be | The usage warning line, then the facts line, as the last two lines of standard error |
| `Come Together` twice, default, `--details` | One request, today's single-record body. Both rows carry `meta.batch` with `records` 2, positions 1 and 2 |
| `annotate` exit 6 | Facts with no `stopped` |
| `--details` at `--batch 10` | Each row carries `meta.batch` with its place. The rows' shares of a batch sum to `meta.batch.usage` |
| `--details` at `--batch 1` | No `meta.batch`. Today's row bytes |
| A 5-record batch refused with 413, halves answer | Rows 1 to 3 carry `records` 3 and `split` true. Rows 4 and 5 carry `records` 2 and `split` true. All carry `closed` of the whole batch |

## Tests and proof

Every test drives the compiled binary against the in-process loopback. New tests live in `crates/thinkthen/tests/backend/facts.rs`. The `meta.batch` rows join ticket 0146's `tests/backend/batching.rs` and its folder.

| Test | What it proves | Deliberate break that turns it red |
| --- | --- | --- |
| `the_facts_line_counts_every_record_and_request` | The `filter` row of the edge table under a private usage folder. The last standard error line equals the pinned object with `seconds` removed. Its `requests_sent`, `input_tokens`, `output_tokens` and `cache_answers` equal the change in the `total_*` fields of `thinkthen status` over the run, and the loopback's request count. The `rank --top` row pins `records` 20. The unwritable usage folder row pins the warning line and then the facts line as the last two lines. Without `--facts`, standard error is empty | (a) Count printed rows: `records` 11. (b) Sum the rows' shares: tokens fall. (c) Print the line before the usage warning: the unwritable-folder row ends with the warning. (d) Print it without `--facts`: standard error is not empty |
| `facts_say_nothing_nobody_reported` | The replay, cache, no-usage and mixed-usage rows | (a) Write 0 for an unreported count. (b) Count a replayed reply's tokens. (c) Keep tokens when one reply lacked usage |
| `a_stopped_run_says_why_and_whether_to_retry` | An edge-case table: the 503, 401, 413, unset key, refused connection, malformed record, replay miss and SIGTERM rows. Each pins the whole `stopped` member and the exit code. The SIGTERM row runs in `tests/backend/interrupt.rs` on ticket 0169's held-reply loopback and its acknowledgment variable. It signals only after the loopback counted 1 request, and pins `records` 0 and `requests_sent` 1 | (a) Mark every exit 4 retryable: the 401 row. (b) Map `NoKey` to `status`: the key row. (c) Take `at` from `finished`: the 503 row reads 10. (d) Mark a transport failure retryable. (e) Re-raise the signal before the line: the SIGTERM row has no facts line. (f) Give a cancelled stop an `at`. (g) Map 413 to `status`: the 413 row |
| `each_batch_row_names_its_batch`, design test 6 | A `decide --details` run at `--batch 10` over design test 3's fixture, `specification/fixtures/batching/grouping.txt`, recorded. Its 25 lines hold content cuts at positions 8 and 17, so the batches are 1 to 8 (`content`), 9 to 17 (`content`) and 18 to 25 (`end`). For each batch, the rows' shares sum to `meta.batch.usage` and `meta.batch.requests_sent`, and each row's `setting`, `records`, `position` and `closed` match those batches. The `--facts` totals equal the sum over batches. A replay of the folder prints no token fields. At `--batch 1` no row carries `meta.batch`. The `Come Together` twice row pins `records` 2 on both rows | (a) Emit `meta.batch` on a batch of one: the `--batch 1` row. (b) Position from 0. (c) Put the row's share in `meta.batch.usage`. (d) Leave out `meta.batch` when the members share one text: the `Come Together` row |
| `a_split_batch_marks_its_halves` | Ticket 0154's 413 row with `--details`. Rows 1 to 5 carry `split` true with their half's counts and the whole batch's `closed` | (a) No `split`. (b) The whole batch's `records` on a half's row |

The four questions:

- **What behavior does each protect?** The facts line's counts, presence rules and stop member, and `meta.batch`. A script's retry decision and a user's spend total rest on them.
- **What credible regression fails each?** The breaks in the table. Each is a one-line slip in counting, mapping or placement.
- **Why does no existing test catch them?** Nothing prints facts or `meta.batch` today. Ticket 0146's `each_row_carries_its_share` checks one batch's split at the row. This ticket checks sums over recorded runs and the run total, which 0146 leaves to test 6.
- **Does any need a test-only hook?** No. The loopback, a private usage folder and a real signal drive them.

Existing tests without `--facts` keep proving that standard error does not change.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`, net against main after the tickets under "Build order" land.

- `cli/facts.rs`: at most 70, new. It builds and writes the line.
- `cli/failure.rs` and its folder: at most 40 net, for the cause map.
- `cli/args.rs` and `cli/args/find.rs`: at most 12 net.
- `cli/mod.rs`: at most 15 net.
- `cli/schedule.rs`, `cli/annotate_schedule.rs` and `cli/asking.rs`: at most 15 net together, for the record count.
- `cli/asking/batched.rs`: at most 20 net, for `meta.batch` and `split`.
- `engine/usage.rs`, `engine/request.rs` and `engine/error.rs`: at most 35 net together.
- `core/result.rs` and `result_json.rs`: at most 40 net together.
- Product code: at most 247 net.
- Tests: at most 320 net across `tests/backend/facts.rs` and the batching tests.
- `sdlc/ratchet.json` moves to the measured total, at most 567 above main after the build order lands. The commit says what grew.
- Pages: at most 40 net lines under `specification/`. One ADR of at most 50 lines.
- No dependency. No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if any run without `--facts` changes one byte of standard output, standard error or its exit code.
3. Stop if a `--batch 1` row changes one byte.
4. Stop if the facts need a field in the usage file or a new `Counts` member.
5. Stop if any break stays green.
6. Stop if a ticket under "Build order" has not landed on main.
7. Stop if the build needs a live call. None is authorized here.

## Build order

1. Tickets 0146, 0148, 0155 and 0154 land first, in that order, by the coordinator's ruling of 2026-09-26. This ticket builds on 0146's `cli/asking/batched.rs`, shares and stop lines, 0155's `retries` counter, and 0154's halves.
2. Tickets 0152 Part B, 0162 and 0169 land first. 0162 and 0169 open `cli/failure.rs`, `cli/mod.rs`, `engine/schedule.rs` and `records.md`. 0162 sets the stop line's number, and 0169 sets the signal exit.
3. Ticket 0153 opens `cli/mod.rs` and `cli/args.rs` on other lines. The second to land merges.
4. Ticket 0171 (B16) builds next. Ticket 0172 (B7) follows it. Both edit `meta`.

## Scope and exclusions

Excluded: library and SQL facts (B12a to B13e), cost (run-facts ask 4), per-request time (the 2026-09-23 issue), a `retries` share on rows, `meta.batch_warning` (0171), `meta.context_sha256` and `--context` (0172), and `site/`.

## Routing

Builder: the agent the work plan names, Claude now or Codex after the handover. Reviewer: a fresh read-only session from the builder's vendor for the code. The change raises the ceiling and widens a public surface, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 1; total 8. Final level: 2. The risks are counts that miss dropped records, a zero for an unreported count, and a wrong retry signal. The first three tests guard them.

## Deferred gaps

1. Library `facts` on every call. B12a writes the run-facts ADR and reuses the counters' in-memory values.
2. A transport failure reads `retryable` false. A script that requeues timeouts reads the cause.
3. Report 11, issue 10, asks for a catalog of every message's cause. The ten words here cover the facts line only.
4. Cost stays out until a price source is decided, by the run-facts issue's ask 4.
5. A row's share of retries. ADR 0052 item 9 leaves rows without it.

## What Ian can overturn

- Decision 1: the retry signal in the facts line, not a new exit code.
- Decision 2: a missing key keeps exit 4.
- Decision 3: transport failures read not retryable.
- Decision 4: the ten-word cause list.
- Decision 6: `--facts` on every asking verb.

## Closes

`sdlc/issues/2026-09-26-run-facts-b5-owe-cause-retryable-and-stop-record.md`. The build moves it to `closed/` and records the route chosen. It settles run-facts ask 2 and gap 5 of `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`, which stays open for asks 1, 3 and 4. It settles local experiment 284 files 38 and 94. The batching design issue stays open.

## Evidence

- Starts from: Local experiment 284 files 38 and 94, each checked on `origin/main` `40783433`: `cli/failure.rs::say`, `engine/usage.rs::Counters`, `engine/request.rs:100-153`, `engine/error.rs:84-145`, `public/error.rs:97`, `channels.md` line 14, `records.md` line 109 and `result.md` lines 38 and 108. The B5 amendment issue. The run-facts issue's tally of 2,545 reply bodies. ADR 0048 items 9 and 10, ADR 0051 item 10 and ADR 0052 item 9. Tickets 0146, 0154, 0155, 0162 and 0169 on their branches.
- Keeps: Every run without `--facts`, byte for byte. Every exit code, including exit 4 for a missing key. `--batch 1` rows. The usage file's schema. The libraries and SQL surfaces.
- Changes: `--facts` on every asking verb prints one `thinkthen.run/1` line last on standard error, from the process counters. A failed run's line carries `stopped` with the stop number, one of ten causes and a retry flag. Batched rows carry `meta.batch`, and a half of a split batch carries `split`. A new ADR amends ADR 0048 item 10. Four pages lose their markers.
- Proof: Five outside-in tests at the loopback, each with deliberate breaks, including design test 6 and file 94's acceptance that the line equals the usage counters' change. The `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: Library and SQL facts, a retryable transport failure, a full cause catalog, cost, and a row share of retries.

## Build preflight, 2026-09-27

The [shared preflight](../records/2026-09-27-batching-ticket-preflight.md) compares executable main `b7efdcc9` with the pending 0154 branch `d10f5d22`. It supplements the older `40783433` snapshot above; it does not change this ticket's accepted outcome.

- Follow `Common` through `cli/judge.rs`, `cli/asking.rs`, and `cli/mod.rs`, and follow `FindCommon::as_common` through `cli/find.rs`. The distinct `FindCommon` adapter must carry `--facts`. Other asking verbs flatten `Common`; this is a file-inventory check prompted by the 0154 missed adapters.
- `cli/mod.rs::entry` reports a transformed failure, flushes counters, then calls `activation.finish`. Place the facts line after the usage warning and before a first signal is re-raised. Check the second-signal path for the accepted no-line rule. Build the cause mapping from the current `Failure` variants, including `ModelsDiffer` and `UsageOverflow` from 0169, and the `Stopped` and `BatchFailed` wrappers. This addresses the 0169 missed-conversion incident.
- `engine/request.rs` installs `Counters::attempt_sent(retry)` as the HTTP attempt hook. On main `b7efdcc9`, `engine/http.rs:160-168` calls that hook before `cancel.remaining()?` and `send`. `remaining()` checks the deadline alone, so expiry during the hook can count an unsent attempt. The accepted facts contract requires sent attempts. [Accepted 0149](https://github.com/botassembly/thinkthen/blob/ticket/0149-sql-settings/sdlc/tickets/0149-sql-settings.md) design item 4 owns final stop, reservation, count and transport ordering; its send-boundary correction must land before exact 0170 facts even if the SQL host settings wait. `engine/deadline_tests.rs::accounting_that_outlasts_the_budget_sends_nothing` already proves no request goes out when the hook exceeds the deadline, but does not inspect usage. Extend that bounded case or an equivalent with a counter assertion: zero listener requests must mean zero counted sends and retries. Keep no-key, replay, refused-plan and stopped-wait cases separate, and read one old `thinkthen.usage/1` row with defaulted `retries`. This does not declare the runtime fixed.
- `schedule::Output::take` can hold or drop printable rows, while `engine.records` owns completion. Count finished input records, including filtered rows and `rank --top` omissions, at the completion boundary; use the accepted `Outcome::Complete` alternative only if the batched callback bypasses the taking point. This addresses the 0162 stopped-record lesson.
- Walk the full path before coding: `Cli` parsing of `Common` or `FindCommon` -> the verb dispatcher in `cli/mod.rs::run` -> `judge`, `find`, `annotate`, `recognize` or `relate` -> `asking`/`schedule` or that verb's completion path -> `engine/request.rs` attempt callback and `Counters::snapshot` -> `cli/mod.rs::entry` final stderr and signal handling. A dry run takes the planning branch before the engine send; it still needs the accepted zero-record facts line. A live run may exit with a value code, a partial result, a typed error or a signal. Pin the line in all relevant classes without duplicating per-verb counting machinery. This is a path checklist, not a new requirement.
- Keep four units separate: finished rows, logical batch requests, actual HTTP attempts (including retries), and replayed records. Main `engine/schedule.rs::Completed.replayed` is a Boolean and `Run::drain` charges all `completed.records` or none; a halved batch can have one replayed and one live half. The 0154 builder is changing this private representation on its pending branch. Verify the landed representation before adding facts, and assert the accepted whole-run attempt/retry totals independently from row shares and replay counts. This is an observed representation constraint, not a landed fix or a new facts field.
