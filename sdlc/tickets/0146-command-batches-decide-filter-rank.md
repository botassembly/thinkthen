---
flow: build
priority: 146
opens: crates/thinkthen/src/core/batch.rs crates/thinkthen/src/core/batch crates/thinkthen/src/core/question_file.rs crates/thinkthen/src/core/question_file crates/thinkthen/src/core/relate_file.rs crates/thinkthen/src/core/result.rs crates/thinkthen/src/result_json.rs crates/thinkthen/src/public/results.rs crates/thinkthen/src/public/question.rs crates/thinkthen/src/public/batch.rs crates/thinkthen/src/cli/audit/write.rs crates/thinkthen/src/cli/conformance_tests/runner.rs crates/thinkthen/src/engine/facade.rs crates/thinkthen/src/engine/schedule.rs crates/thinkthen/src/engine/annotate_schedule.rs crates/thinkthen/src/engine/facade_tests.rs crates/thinkthen/src/engine/facade_tests crates/thinkthen/src/engine/deadline_tests crates/thinkthen/src/public/bulk.rs crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/judge.rs crates/thinkthen/src/cli/asked.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/batched.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/schedule crates/thinkthen/src/cli/annotate_schedule.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure crates/thinkthen/tests specification/records.md specification/backends.md specification/channels.md specification/question-file.md specification/question-file.schema.json specification/result.md specification/decide.md specification/filter.md specification/rank.md specification/settings.md specification/fixtures/systemone spec/decide.md spec/audit.md demos probes/speed/functions.jsonl sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0146: The command batches decide, filter and rank

Status: ready for review. Owner: Claude.

ADR 0055 reopened it after acceptance, because a batch of `decide`, `filter` or `rank` now sends one fixed evidence sentence in place of the records list.

Its build waits for tickets 0158 and 0161 to land. It builds only after tickets 0143, 0144, 0145 and 0161 land, after "S1 live run 1" runs on main, and with ADR 0053 on main and ticket 0158 landed. Ticket 0161 must land first because it calls `Reading::batch_record` and removes that function's `expect(dead_code)` in `core/records.rs`. This ticket does not open `core/records.rs`. If it built first, its reader's call would leave that mark unfulfilled, and clippy with `-D warnings` in `sdlc/scripts/lint` would fail.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `thinkthen filter` over 306 song titles and the tool sends one request, not 306. `decide`, `filter` and `rank` over a stream of records fill each request with as many records as fit under the backend's limits. `--batch N` caps a request at N records, and `--batch 1` gives back today's requests byte for byte.

This is ticket B4 of `sdlc/issues/2026-09-26-batching-design.md`. Its row reads: "The command batches `decide`, `filter` and `rank`, filling to the limit by default. `--batch`, `THINKTHEN_BATCH`, the question file's `batch`, precedence, jobs over batches, order, pause, failure line, record, replay, cache, dry run. It removes `decide`, `filter` and `rank` from S1's list. Proof: tests 4, 5, 7 and 8; S1's gate part. Depends on B3, S1. Covered by B0."

ADR 0048 is B0. Its ticket table gives B4 items 1 and 2 on the command, and items 3, 4, 5, 6 and 13 for `decide`, `filter` and `rank`. Item 7 gets one row per function, and B4 builds the row for these three.

ADR 0055 amends ADR 0048 item 1 and ADR 0053 item 5 for these three verbs, by Ian's ruling of 2026-09-26. Its consequences name this ticket as the one that builds it. So B4 also changes the planner's evidence for these verbs, before any batch reaches the command.

Ian's rulings of 2026-09-26 set the frame. Ian can overturn each.

- Batching is automatic and maximal. The default is `max`.
- Speed wins over accuracy. The default fills to the limit although it costs accuracy. B6 measures that cost.
- The throttle keeps its default of 4, set by `--jobs`, 1 to 32. The batch is the speed lever.
- The target: `filter` over the 306 songs in under half a second at the default throttle, on a named build, measured live.

**Order and preconditions.** The coordinator set these within Ian's ruling 9, and Ian can overturn them.

1. Ticket 0143 (J1) lands first. B4 edits `engine/facade.rs`, which 0143 changes.
2. Ticket 0144 (B3) lands first. B4 calls its `Batcher`, `BatchRecord`, `Batch` and `Reading::batch_record`.
3. Ticket 0145 (S1) lands first. B4 removes the `list` field from the `decide`, `filter` and `rank` rows of `probes/speed/functions.jsonl`. The rows stay.
4. "S1 live run 1" runs on a main commit after S1 lands and before B4's build starts. Its record under `probes/speed/runs/` is the baseline B4 is measured against. S1 decision 13 makes this B4's precondition.
5. ADR 0053 is on main.
6. Ticket 0158, which keeps a reply that failed a question out of the cache, lands before B4's build starts. The coordinator ruled this order on 2026-09-26. Ticket 0154 builds after B4 lands, as its decision 15 says.

## What happens today

Reading from `origin/main` `7850db3f` and the three ticket branches.

- `cli/asking.rs::run` reads a stream in a reader thread (`cli/schedule.rs::read_records`). The engine scheduler (`engine/schedule.rs::run_cancelled`) hands each raw record to a worker. The worker parses it and asks one request through `Engine::judge` (`engine/facade.rs:220`). Rows print in input order. `--jobs N` bounds the items in flight, and each item is one record.
- `Outcome::Stopped` counts places in items: `at` is the failed item plus one, and `finished` counts items. `cli/failure.rs::stopped` prints the cause on one line and `stopped at record {at}; {finished} records finished` on the next.
- `decide`, `choose`, `tag` and `score` share `Keeping::Answers` and the `judging` flow in `cli/judge.rs`. `filter` and `rank` share `over_kept`. All four argument structs flatten `Common`, which holds `--jobs`.
- Record-mode `--dry-run` reads the first record and prints its plan (`cli/asking.rs::plan`).
- Ticket 0144's branch adds the pure planner in `core/batch.rs`. `Batcher::new(backend, profile, question, setting, context)` checks the context. At its commit `95293ef4`, `push(&mut self, record: BatchRecord, closed: &mut Vec<Batch>) -> Result<(), BatchError>` adds the batches that close to `closed`, zero, one or two. A batch that closes before a refusal stays in `closed`, and a record that fails the profile alone is refused on its own push. `finish()` closes the open batch and returns `None` when it is empty. Each `Batch` holds its plan, exact body, digest, each record's first wire question, and why it closed: `Content`, `Size`, `Limit` or `End`. `Reading::batch_record(&Record)` gives each record its evidence and its JSON value. The module carries `expect(dead_code)` for B4 to remove. Ticket 0161 calls `batch_record` and removes its function-level mark. The module mark in `core/batch.rs` stays for this ticket to remove.
- On main at `a18a8345`, `Batcher::plan` sends `{"records":[…]}` as the evidence of every batch without a context, whatever the question kind. `Batcher::built` sends a batch of one distinct record without a context as today's request of that record. The fixtures `batch-three`, `batch-csv` and `batch-duplicate` under `specification/fixtures/systemone/` pin the records list for decide questions. `batch-choose` pins it for a pick-one question. `batch-context` pins a context as the evidence.
- A reply decodes into one `AnswerOutcome` per planned question (`core/reply.rs`). A question the backend failed beside good answers is `AnswerOutcome::Failed`. A reply where every question failed is a whole-reply decode error.
- Ticket 0145's gate lists `decide`, `filter` and `rank` for ticket B4. Each sends 12 requests for 12 lines where 1 fits.
- The demos under `demos/` and their recordings were made one record a request. So were the tests' recorded folders.

## Design

### The evidence of a batch

ADR 0055 item 1 fixes the evidence of a plain batch of `decide`, `filter` and `rank`. A batch of two or more distinct records without `--context` sends this sentence as its evidence, as plain text:

```text
Each question quotes the text it asks about.
```

It no longer sends `{"records":[…]}`. The sentence is 44 bytes, and it does not grow with the records. Each record appears once, inside its own question. The question is ADR 0048 item 1's question, byte for byte: `The text is `, the record's compact JSON, `. `, then the user's question unchanged. Equal records inside one batch still share one question.

Three rules keep everything else as it is.

- A batch of one distinct record without a context stays today's request, by ADR 0055 item 3. The record is the evidence and the question is unquoted. `--batch 1`, `THINKTHEN_BATCH=1`, a question file's `"batch": 1` and a stream of one record therefore send today's bytes.
- A context is unchanged, by ADR 0055 item 2. B4 passes no context, and B7 builds `--context`.
- `choose`, `tag` and `score` questions keep the records list in the planner, by ADR 0055 item 4. B4 does not batch them on the command. B8 and B9 measure their form on long records.

**The planner change.** `core/batch.rs` gains the constant `QUOTED`, the sentence above. `Batcher` holds the batched evidence as `shared: Option<Evidence>`. It is the context when one is given. Otherwise it is `QUOTED` for a `Question::Decide`, and none for other question kinds. `plan()` sends `shared` when it is set and the records list only when it is not. `separators()` and `listed()` read `shared` where they read the context today, so a quoted batch counts one separator a record and no list bytes. `built()` still treats a single distinct record as batched only beside a real context. `context_fits` still checks only a real context, so a record over a limit alone still gets today's profile refusal. The byte check in `built()` keeps checking every batched body against its counts.

**Why the planner does not get simpler.** The records list, `records()`, `Open::lines` and `Joined::line_len` stay, because `choose`, `tag` and `score` still use them. Removing them would settle B8's and B9's form before they measure it, against ADR 0055 item 4. The change adds about 8 lines to `core/batch.rs`. Once B8 and B9 settle their form, the list may go.

**Fixtures to regenerate.** Three fixtures under `specification/fixtures/systemone/` hold decide questions and change. Each keeps its `model` and `questions` byte for byte. Only its `state` member changes, from the records list to the sentence as a JSON string. The builder writes each by hand and checks it with `jq -c`.

- `batch-three.request.json`: 454 bytes with its trailing newline, down from 475.
- `batch-csv.request.json`: 513 bytes, down from 561.
- `batch-duplicate.request.json`: 320 bytes, up from 313, because two short titles are shorter than the sentence. Its exact bytes:

```json
{"state":"Each question quotes the text it asks about.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is \"Come Together\". The text is the title of a song by the Beatles."},"q2":{"type":"noul","instructions":"The text is \"Because\". The text is the title of a song by the Beatles."}}}
```

`batch-choose` and `batch-context` keep their bytes. The fixture README's sentence "Without a context the evidence is `{"records":[…]}`" becomes: "Without a context, a batch of decide questions sends the evidence `Each question quotes the text it asks about.`, by ADR 0055. A batch of choose, tag or score questions sends `{"records":[…]}`." Its `batch-duplicate` line then says the copy adds no question. Its `batch-csv` line says the rows are quoted as JSON objects, not listed.

**Planner tests that change.** The planner's tests in `core/batch/tests.rs` change their expected values, not their shape.

- `each_batch_body_matches_its_fixture` reads the regenerated fixtures.
- `limits_close_batches_by_exact_bytes_and_the_ceiling` takes its evidence limit from the sentence, 44 bytes. `max_evidence_bytes` 44 keeps the three records in one batch. `max_evidence_bytes` 43 gives three batches of one, each today's request of its record: `(1, Limit)`, `(1, Limit)`, `(1, End)`. The request-bytes rows take their limit from the regenerated `batch-three`. The builder recomputes the pair 817 and 816 for the twelve one-letter records by hand from the new form, checked with `jq -c`, and keeps the two rows one byte apart.
- `the_ceiling_closes_batches_at_the_built_in_address_only` changes three rows, because each record now counts once against 96,000 bytes. Three records of 20,000 bytes share one batch: `(3, End)`. Three of 40,000 bytes give `(2, Limit)`, `(1, End)`. The same under a profile of 200,000 request bytes gives `(3, End)`. One record of 100,000 bytes and the loopback row keep their results.
- `a_batch_of_one_is_todays_request`, `batches_close_where_the_readme_says` and `questions_copies_and_refusals_follow_the_batch_rules` keep their expected values. The tag row there still counts wire questions, and the context row still sends the context.

`batch` is `max` or a whole number of at least 1, as ADR 0048 item 3 fixes. `core/batch.rs` gains `Setting::parse(&str)`. It takes `max` and a decimal whole number from 1 up. It refuses `0`, a sign, a fraction, white space and any other text.

**Where it is typed.** `DecideArguments`, `FilterArguments` and `RankArguments` each flatten one new `Batching` struct holding `--batch <N|max>`. It sits in the long help only, by ADR 0048's call 18. `choose`, `tag` and `score` do not get it. Clap refuses `--batch` there as an unknown option at exit 2. B8 and B9 add the flatten to their verbs.

**Where it is read,** in ADR 0048 item 4's order:

1. `--batch`.
2. `THINKTHEN_BATCH`, read once in `cli/edge.rs` beside `THINKTHEN_BASE_URL`.
3. The question file's `batch`, at the top level of a `decide` question file. `filter` and `rank` read `decide` files, so all three verbs see it.
4. `max`.

The read-only configuration file holds no `batch` key and adds no tier. `THINKTHEN_BATCH` is ignored on every verb but these three, including `choose`, `tag` and `score`, until B8 and B9. `settings.md` states this in the row.

**The file key stays off annotate entries.** An `annotate` question set parses each entry through `QuestionFile::parse`, with the same `Verb::Decide.keys()`. So B4 does not add `batch` to `Verb::Decide.keys()` or to `EVERY_KEY`. `core/question_file.rs` gains `QuestionFile::parse_top(text) -> Result<(QuestionFile, Option<Json>), QuestionFileError>`. It takes `batch` out of the top level of a `decide` file, returns its raw value, and parses the rest through `parse` unchanged. A `batch` in a `choose`, `tag` or `score` file stays in the object, so `parse` refuses it as today. `parse_top` has four callers:

- `cli/asked.rs`, which reads the command's `@FILE`. It checks the raw value with `Setting::parse`'s rule and feeds the file tier.
- `cli/audit/write.rs`, which checks a file before `audit --write`. It ignores the value. `audit --write` edits the file's text in place, so the key stays in the file.
- `public/question.rs`, the library's question file reader. It ignores the value until B12a, and `settings.md` says so in the row.
- `core/relate_file.rs`, whose wrong-verb check asks whether the text is a question file. A `decide` file holding `batch` then still gets the wrong-verb refusal from `relate`.

`QuestionSet` keeps calling plain `parse`. An annotate entry that holds `batch` is refused exactly as today, with `a question file takes no key `batch``. B10 decides the question set's one top-level `batch`. Because the key leaves the object before parsing, it never reaches the question digest, and `meta.question_sha256` does not change. The schema gains `batch` on the top-level `decide` file entry only, the string `max` or an integer of at least 1, and not in any definition an annotate set refers to.

**Refusals.** Each sentence is pinned in a test.

| Where | Value | Exit | Message |
| --- | --- | --- | --- |
| `--batch` | `0`, `1.5`, `fill`, empty | 2 | `--batch takes max or a whole number of at least 1` |
| `THINKTHEN_BATCH` | the same | 2 | `THINKTHEN_BATCH takes max or a whole number of at least 1` |
| Question file | `0`, `1.5`, `"10"`, `"fill"`, `true` | 5 | `` `batch` in the question file takes max or a whole number of at least 1 ``, through today's `QuestionFileError::Shape` form. It names the key and never the value |
| `--batch` on one document | any | 2 | `--batch groups the records of a stream, and a single text is one record` |

`THINKTHEN_BATCH` or a file's `batch` on one document is not typed for that run and is ignored. `--jobs` follows the same split today (`Failure::JobsOutsideRecords`).

A bad `THINKTHEN_BATCH` is refused only on a stream of records. A run of one document never reads it.

### The reader builds batches

The batched path lives in one new file, `cli/asking/batched.rs`. `cli/asking.rs::run` sends `decide`, `filter` and `rank` over a stream there. Every other verb and every run on one document keep today's path.

**Parsing moves to the reader.** The planner needs each record's value before a request can form. So the reader parses each record, builds its `BatchRecord` through `Reading::batch_record`, and pushes it into the `Batcher`. The reader keeps the open batch's parsed records and their arrived bytes beside the planner, because `filter` prints records as they arrived and `--details` prints `input`. The member cap bounds the held records of the open batch. When `push` adds batches to `closed`, the reader splits its held records by each batch's record count, `questions.len()`. An item holds the batch, its first record number, and its records. CSV and TSV runs take the same path from `TableRows` through `over_table`.

**The reader protocol.** Two threads sit on the command's side.

1. The record thread runs today's `edge::Chunks` or `TableRows`. It sends each raw record down a bounded `sync_channel`, so it reads at most a fixed number of records ahead. The builder names the bound, at most 64.
2. The batch reader answers the scheduler's asks. The scheduler asks for one item at a time, as today, only while fewer than `--jobs` items are waiting.
   - Closed batches wait in a queue in input order. An ask takes the front of the queue when it is not empty. A push can close two batches, so the queue can hold one ahead.
   - With the queue empty, the reader pulls records from the channel and pushes them until a batch closes, then answers the ask with it.
   - The reader pulls records only while an ask is outstanding. With no ask, records wait in the bounded channel, and the open batch does not grow.
   - The 50 ms timer runs only while an ask is outstanding and the open batch holds a record. So a batch never closes on a pause while every job is busy.
   - At end of input the reader calls `finish()`, queues the last batch, then answers the next ask with `Input::End`.

**The pause.** ADR 0048 item 2's pause belongs to the edge, because the core reads no clock. `Batcher` gains `pause()`, which closes the open batch as the new `Closed::Pause` and returns it, or `None` when the batch is empty. The batch reader waits on the record channel with `recv_timeout` of 50 ms under the protocol's conditions, and a timeout calls `pause()`. The value is fixed and has no option.

The pause fires in every mode, by ADR 0053 item 1. A typed `--cache`, `--record` or `--replay` folder does not turn it off, and neither does the default cache or `THINKTHEN_CACHE`. A file or a pipe that never waits 50 ms forms the same batches every run, and a recording or cache answers them. A live pipe that pauses forms batches by its timing. A later run with other timing can cut a batch elsewhere. `--replay` then stops at exit 5 at that batch, and `--cache` and the default cache pay for it again.

### The member cap

`Batcher` gains the fixed cap `MEMBERS`, 4,096, by ADR 0053 item 2. After a content cut and the size, `push` closes the open batch as `Closed::Limit` when it holds 4,096 members, repeats included. A stream of a few distinct values then forms batches of at most 4,096 records. A run then holds at most `--jobs` batches in flight, one queued batch, the open batch, and the channel's read-ahead.

**A record the planner or the parser refuses.** B4 reads 0144's `push` at `95293ef4`. On a refusal, `closed` holds every batch that closed before it, and the refused record joins no batch. After a refusal the open batch is empty, so `finish()` returns `None`. The reader queues what `closed` holds, drops the refused record, and answers the next ask after the queue with the refusal as `Input::Failed`. A parse refusal happens before any push, so the reader calls `finish()`, queues the batch it returns, and answers the same way. The records before the refused one then print, and the run stops at the refused record with today's cause and exit code. `BatchError::Profile` becomes today's profile refusal. `BatchError::Defect` becomes `Failure::Defect`. B4 never passes a context, so the context errors cannot arise here.

**The row's question.** Each row's `question` and `meta.question_sha256` come from the user's question, unquoted, as today. The quoted wire question stays inside the batch plan. A question file whose question text is a JSON object or list gets batches of one record in today's form, by 0144's rule, so its requests match today's.

**Dry run.** Record-mode `--dry-run` pushes records until the first batch closes by content, size, limit or end of input. It never waits on a pause. It prints that batch's plan in today's plan document, with today's `input` field. A stream of one record, or `--batch 1`, prints today's plan byte for byte.

### The engine asks a batch

`Engine` gains `ask_batch(&Batch, &Cancel) -> Result<Answered, Error>`. It builds a `PreparedRequest` from the batch's body and digest and sends it through `request::ask_sent`, as `ask_chunks` sends a chunk. So retries, recording, replay, the cache and the usage counters all act on the batch as one request. A retried status resends the whole body. The cache key is the batch's request digest, by today's rule. No new cache or recording code is needed.

### The scheduler counts records

The scheduler already bounds items in flight and emits them in order. With batches as items, `--jobs N` means N batches in flight, and the output buffer holds at most N batches of rows. That is ADR 0048 item 5 with no change to the bound.

Two counts change, because an item now carries many records.

- `Completed` gains `records`, the rows the item finished, and `stop`, an optional cause after those rows. A new constructor, `Completed::one(value, replayed, partial_failure)`, sets `records` to 1 and `stop` to none. Each existing builder of `Completed` changes one struct literal to that call, so its outcome does not change. The builders are `cli/schedule.rs`, `cli/annotate_schedule.rs` and `public/bulk.rs` in product code, and `cli/schedule/width_tests/facade_tests.rs`, `engine/schedule.rs`'s tests, `engine/facade_tests.rs`, `engine/facade_tests/contract_tests.rs` and `engine/deadline_tests/schedule.rs` in unit tests. `stop` holds the run's error type, so `Completed<R>` becomes `Completed<R, E>`. Every place that names the type gains the second parameter: `engine/annotate_schedule.rs`, `public/batch.rs`'s `Answer<V>` alias, `public/bulk.rs`, and `cli/conformance_tests/runner.rs`. The group scheduler ignores the two new fields, so its outcome does not change.
- `Outcome::Stopped` counts records. `finished` is the sum of `records` over emitted items. `at` is `finished + 1`. `replayed` sums `records` over replayed items.

An item with a `stop` emits its rows and then stops the run at the next record. This carries ADR 0048 item 6's partial reply without a second scheduler.

### Replies back to rows

A worker asks the batch, then walks its records in order. Record `i` reads `outcomes[questions[i]]`. So a copy reads its first copy's answer.

- An answered record builds its row exactly as `Judging::finish_row` does today. The row code is shared, not copied.
- The first record whose outcome is `Failed` ends the walk. The item carries the rows before it and a `stop` naming that record.
- A request failure, a whole-reply decode failure, or a replay miss fails every record of the batch. The item is an error, and the run stops at the batch's first record.

**Usage shares.** A row must not claim the batch's whole usage. Each row carries an even share of the batch's input tokens, output tokens and requests sent, with any remainder to the earliest records, by ADR 0048 item 9. A token field the backend did not report stays absent. `core/result.rs` gains `Usage::share(records, position)` and the same rule for a count, as pure functions. `result_json.rs::decision` takes the row's share in place of the reply's whole usage and attempts. A batch of one passes a share of the whole, so its row bytes stay today's. `meta.requests` holds the batch digest. B5 adds `meta.batch` and `--facts`.

### Stop lines

Today a stopped run prints its cause on one line through `say`, then `stopped at record {at}; {finished} records finished{recording clause}{withheld clause}` on the next.

**The one-line form.** A batch of two or more records that fails as a whole prints one line in place of those two. Its template:

```text
thinkthen: stopped at record {at}; the request for records {first} to {last} failed: {cause}; {finished} records finished{recording clause}{withheld clause}
```

`{cause}` is the sentence `say` prints today for that failure, without its `thinkthen: ` prefix. The two clauses are today's. The exit code is the cause's code, as today. A 503 on the second batch at `--batch 10`, after the retries, prints:

```text
thinkthen: stopped at record 11; the request for records 11 to 20 failed: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries; 10 records finished
```

Three causes use the one-line form, on a batch of two or more records:

- A request failure: a transport failure, a timeout, or a status after its retries, including status 400 `max_tokens_exceeded`.
- A whole-reply decode failure: a reply that is not a systemone response, names no model, or fails every question.
- A replay miss: `--replay` finds no recording for the batch's digest.

Every other stop keeps today's two lines, on a batch of any size:

- Cancellation: an interrupt or a host stop.
- Input and parse refusals: a malformed record, a record over a profile limit alone, a pointer that finds nothing, and every other refusal the reader sends as `Input::Failed`. Each names one record, not a batch.
- `Failure::RecordingStorage`, which today prints its cause and no stop line.

A failed batch of one record prints today's two lines, whatever the cause. So `--batch 1` keeps standard error byte for byte.

**A partial reply.** A reply that answers some records and fails one prints the good rows before it, then one line, at exit 4:

```text
thinkthen: stopped at record 13; the reply for records 11 to 20 gave record 13 no usable answer; 12 records finished
```

The recording and withheld clauses follow as today. Neither line echoes a record, and neither names the failed question's wire place.

### Pages

Each rule below becomes true for `decide`, `filter` and `rank`. The builder deletes today's sentence where the new rule replaces it, and deletes the marker. Where the rule stays untrue for another verb, a marker stays for that verb only.

| Page | Marker, by its opening words | What B4 does |
| --- | --- | --- |
| `records.md`, "Records never share model context" | item 1: "records of one batch share one request and see each other" | Built for `decide`, `filter` and `rank` in ADR 0055's form. Add: "On `decide`, `filter` and `rank`, records of one batch share one request. A batch of two or more distinct records sends the evidence `Each question quotes the text it asks about.`, and each record appears once, inside its own question: `The text is `, the record's compact JSON, `. `, then the question. A record is not evidence for any other record in its batch, by ADR 0055. A batch of one record sends the record as the evidence, as before batching." ADR 0053 item 5's caution does not go on these verbs, by ADR 0055 item 5. It moves into the marker by item 7 that stays for `choose`, `tag` and `score`: "`choose` and `tag` batches will list every record in the evidence, so each record's text is evidence for every other record in that batch, and `--batch 1` keeps them apart" |
| `records.md`, "Order and requests", "One request normally carries one piece of evidence" | item 2: "by default each request fills with records" | Requests fill to the limit on the three verbs. A marker by item 7 stays for the other three |
| `records.md`, the request table row "`decide`, `choose`, `tag`, `score`, `filter`, `rank` over N records" | item 2: "one for each batch" | The row splits. `decide`, `filter`, `rank`: one request a batch, N at `--batch 1`. `choose`, `tag`, `score`: N, with a marker by item 7 |
| `records.md`, "A run that stops early prints one line" | item 6: "when a batch fails, the line names the range" | Built, with the template above. The item 10 marker for `--facts` stays |
| `records.md`, `jobs` | item 5: "`jobs` counts batches in flight" | Built |
| `records.md`, "Order and requests", after the item 2 sentence | none, new text by ADR 0053 items 1 and 2 | Add: "A live run sends the open batch after 50 ms with no new record, in every mode, with or without a folder. A file or a fast pipe never pauses, so it forms the same batches every run. A live pipe forms batches by its timing, so a replay fed with other timing can miss a batch and stop at exit 5 naming its range, and a cache pays for the batches that moved. A batch also closes at 4,096 records, repeats included, so a stream of a few repeated values never holds more than 4,096 records in one batch." |
| `records.md`, "Resume", the sentence "A first run that stops at record 400 leaves 399 entries. The same command run again replays those 399 and pays for the rest." | none, corrected for batches | Replace it with: "At `--batch 1`, a first run that stops at record 400 leaves 399 entries, and the same command run again replays those 399 and pays for the rest. Under batching, `decide`, `filter` and `rank` leave one entry for each batch that finished. A rerun replays those batches and pays again from the first record of the batch that failed." Ticket 0158 lands first and appends "A record whose reply failed a question has no entry, so the resumed run asks for it again." B4 rewords that sentence for batches: "A batch whose reply failed a question has no entry, so the resumed run asks for the whole batch again. At `--batch 1` that batch is one record." |
| `backends.md`, "A relation plan at the built-in address also splits" | item 2: "a batched record plan at the built-in address also closes its batch at the ceiling" | Built. It adds: a batch of `decide`, `filter` or `rank` carries each record once, inside its quoted question, beside the 44-byte evidence sentence of ADR 0055. Records fill a batch until the next one would pass 96,000 request bytes |
| `backends.md` | item 2 and 6: "a profile's limits close batches at every address" | Built |
| `channels.md`, "Advanced options appear in the long help alone" | item 3, 10 and 11: "`--batch N`, `--facts`, and `--context FILE` join" | `--batch N` joins the advanced options. The marker stays for `--facts` and `--context` |
| `channels.md`, "In record mode `--dry-run` prints the plan" | item 13: "it plans the first batch" | Built |
| `question-file.md`, the settings table | item 3: "the batch setting" | Built for `decide` files. A marker by item 7 stays for `choose`, `tag` and `score` files |
| `question-file.md`, "Precedence" | item 4 and 8: "`--batch` replaces the file's `batch`" | Item 4 is built. The item 8 marker stays for the calibration part, which is B16's |
| `result.md`, the `usage` and `requests_sent` rows | item 9: "a batched row carries its even share" and "its share of the batch's attempts" | Built. The `meta` paragraph and the `batch` row keep their item 9 markers for B5 |

After B4, `grep -rnE "Not built yet, by ADR 0048 item (1|2)[ :]" specification` returns nothing. The pattern catches `backends.md`'s "item 2 and 6:" and passes over items 10 and 11.

`decide.md`, `filter.md` and `rank.md` each gain these sentences, which state ADR 0055's measured table and its one named loss: "By default a stream of records shares requests, filling each to the backend's limits. The evidence of a batch is one fixed sentence, and each record appears once, inside its own question. Every run moves a few answers, and batching moves a few more. ADR 0055 records local experiment 275, which asked four yes/no questions over the 306 Beatles songs. It ran each batched form three times on the same bytes and once on each of three shuffled record orders. The batched form stayed within 4 right answers across repeats and orders on every task. It never fell more than 3 right answers below one song a request. On "It appears on the album Abbey Road" it scored 283 to 287 right of 306, where one title a request scored 286. It sent 11,468 input tokens for the 306 titles, where one title a request sent 88,933. Its one measured loss came on "It was released before 1965". There it scored 255 to 259 right, and one title a request scored 253. The earlier batch form, which listed every record in the evidence, scored 276 to 283 in the table's own order and 258 to 270 over shuffled orders. `--batch 1` asks one record a request and sends the requests the tool sent before batching." `decide.md` also gains, after its paragraph on planted claims: "A batch keeps each record in its own question and out of the evidence. A planted claim in one record is therefore not evidence for another record, by ADR 0055 item 5." B4 adds no sentence that records of one batch are evidence for each other on these pages, by ADR 0055 item 5. `settings.md` moves `batch` from "Settings on the way" into the table. Its library and SQL cells read `not on this surface` and name tickets B12a to B13e. The row also says: "Until the libraries and SQL extensions batch, they ask one record a request, so their answers and cache entries match the command's only at `--batch 1`." The row also says the library's question file reader accepts a `decide` file's `batch` and ignores it until B12a. `question-file.schema.json` gains `batch` on the top-level `decide` file only. `settings.md` also adds `batch` to its list of precedence orders on record: `--batch`, then `THINKTHEN_BATCH`, then the question file's `batch`, then `max`, by ADR 0048 item 4.

### Demos, pages and tests recorded one record a request

A demo or page command that replays or caches a folder recorded one record a request would miss its recording under the default. B4 adds `--batch 1` to each such command in its `README.md`, and to the matching command in its `record.sh`, so a later re-recording makes the same requests. `spec/audit.md` replays `transforms/rows/recording` through `decide --jsonl` and gets the same pin. A dry-run step needs no recording, so its expected output changes to the default's batch plan.

**Existing tests.** The first build step turns the default on and runs the `test` rung once, before any pin. It counts the test files that fail and records the list. A realistic estimate is 20 to 32 files, from the 92 test files that name one of the three verbs and the ones among them that count requests or replay folders. A file whose subject is not batching gets `--batch 1`. Where the file builds its commands through one shared helper, such as `tests/backend/support.rs::decide` or a file's own `run` function, the pin goes in the helper once. Otherwise each command gets it. The build record gives the real count and every pin.

In `tests/backend/scheduling.rs` the `--batch 1` pin goes only on `ordered_output_bounds_every_dispatched_row`. The shared `interactive` helper gets no pin. `an_answer_arrives_before_the_next_record_at_one_job_and_the_default` and its two new `--record` and `--replay` rows run at the default, so they guard the pause under batching.

### The design issue's B5 row and test 9

B4 builds the shares, so the B5 row of `sdlc/issues/2026-09-26-batching-design.md` drops "Shares". It now reads "`meta.batch`, `--facts` and the `thinkthen.run/1` line". This ticket's commit makes that edit.

The same commit rewrites acceptance test 9 of that issue for ADR 0055 item 6. B6 still runs it, and it still gates nothing. Its default and `--batch 10` arms now send the fixed evidence sentence. Its shuffled arms grow from two orders to three, with fixed seeds, as local experiment 275 ran them. It reports each batched setting's three same-bytes repeats and its three shuffled orders side by side: right answers, false yeses, misses, calibration error, and the answers that cross the cut. Its comparison figures become ADR 0055's table for the title task "It appears on the album Abbey Road": 286 right at one title a request, 266 to 277 in the records-list form, and 283 to 287 in the quoted form. It names the release-year title task as the one measured loss, with ADR 0055's figures. The planted-claim arm stays. It reports how many other answers crossed the cut. Under ADR 0055 item 5 the planted record sits only in its own question, so the arm checks that claim and gates nothing.

The same commit checks that the design issue no longer limits the pause to an unrecorded run, by ADR 0053 item 1. The friction row reads "Designed away. A live batch sends after a 50 ms pause in input, in every mode". The close-rule row reads "| Pause | A live run has waited 50 ms with no new record, in every mode |". The edge row reads "| A live stream that pauses, with or without a folder | The open batch sends after 50 ms |". A coordinator commit on main made these three edits on 2026-09-26. If a merge brings the old wording back, this commit restores it.

### S1's list

B4 removes the `list` field from the `decide`, `filter` and `rank` rows of `probes/speed/functions.jsonl`. The rows stay, in the commit that makes them batch. S1's gate then requires at most one request for each of their 12-line workloads.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **`--batch` goes on the three verbs, not on `Common`.** A flag on `Common` would reach `choose`, `tag` and `score`, where it could not act. That needs a refusal that B8 and B9 would delete.
2. **The reader thread builds batches.** Parsing moves there from the workers. The scheduler's items become batches, and its bound and order work unchanged.
3. **The scheduler counts records by weight.** One small change keeps one scheduler. The stop line then names records, not batches.
4. **An item carries an optional stop after its rows.** A partial reply prints its good rows and stops at the failed record. The alternative is a second scheduler.
5. **The pause lives at the edge, with a 50 ms receive timeout that runs only while an ask is outstanding.** The core stays pure. `Batcher::pause()` is the only core addition for it. A timer that ran while every job was busy would cut small batches for no gain in latency, because nothing could send them.
6. **The pause fires in every mode.** ADR 0053 item 1 ruled it, because a loop that writes one record and waits must get its answer under a named folder too. The cost is a replay miss or a paid repeat when a live pipe pauses at a different place than it did when recorded. A file and a steady pipe form the same batches every run.
7. **B4 builds the usage shares.** ADR 0048 gives the shares to B5. But from B4 on, rows come from batches. A row carrying the whole batch's usage would overstate every total a user sums. The share rule is short and already fixed. B5 keeps `meta.batch`, `--facts` and design test 6.
8. **A multi-record failure prints one line. A batch of one keeps today's two.** ADR 0048 item 6 fixes the one-line form. `--batch 1` then keeps today's standard error byte for byte.
9. **A partial reply's line names the record and not the cause.** A failed yes/no question carries a closed cause but no sentence. The line says the record got no usable answer. The exit code is 4, the backend-failure code.
10. **`--batch` on one document is a usage error, as `--jobs` is.** `THINKTHEN_BATCH` and a file's `batch` are ignored there, because neither was typed for that run.
11. **A bad `THINKTHEN_BATCH` exits 2, and a bad file `batch` exits 5.** The file rule follows `question-file.md`: a value a file holds is a local failure. The environment variable acts like a typed value that was set once.
12. **Demo and page commands that replay pin `--batch 1`.** Re-recording them under the default needs a paid run. The pin keeps every recording valid until an authorized run records them again.
13. **B4 lands before the target is measured live.** B4 cannot make a live call. It proves the request count at the loopback. "B4 live run", below, measures the time on main after B4 lands.
14. **`QuestionFile::parse_top` takes `batch` off the top of a `decide` file before parsing, in the core.** Every reader of a whole question file calls it, and `QuestionSet` does not. `Verb::Decide.keys()` also serves annotate entries, so adding the key there would let an annotate entry carry `batch` before B10 decides it. Taking the key off first keeps it out of the question digest by construction.
15. **The reader queues closed batches and pulls records only for an outstanding ask, through a bounded channel.** A run then holds at most `--jobs` batches in flight, one queued batch, the open batch, and the channel's read-ahead, as today's reader is bounded by `--jobs` records. The member cap bounds each batch at 4,096 records.
16. **A batch closes at 4,096 members.** ADR 0053 item 2 ruled it. Without it a repeated value never closes a batch under `max`, and the reader's memory grows with the run.
17. **The planner picks the evidence by question kind.** A decide question without a context gets ADR 0055's sentence. A choose, tag or score question keeps the records list. B4 asks only decide questions, so only they change on the wire. The records list stays for B8 and B9 to measure, by ADR 0055 item 4. The cost is about 8 lines in `core/batch.rs` and the kept list code.
18. **B4 builds ADR 0055 before it lands.** No command sends a batch before B4, so no recording or cache holds a batch of the records-list form. Every recording made one record a request stays valid, because a batch of one is unchanged.

## Edge cases

| Input | Expected |
| --- | --- |
| Empty stream | No output, no request, exit 0 |
| One record, default | Today's exact request and row |
| `Come Together`, `Because` and `Come Together`, `decide` with the question `The text is the title of a song by the Beatles.`, `--model jev-latest`, default, loopback | One request. Its body is exactly the regenerated `batch-duplicate.request.json` above, without the trailing newline. Three rows print, and rows 1 and 3 read question `q1`'s answer |
| Two distinct records, a batch of two, with the default question | The evidence is `Each question quotes the text it asks about.` Each question is `The text is `, the record's compact JSON, `. `, then the question, byte for byte as ADR 0048 item 1 quotes it |
| Two equal lines, default | One distinct record, so today's request of that record with the question unquoted. Both rows print, each with its answer |
| 306 made-up titles, default, loopback | One request. The records list appears nowhere in its body |
| The same, `--batch 10` | 31 requests: 30 of 10, one of 6 |
| The same, `--batch 10`, `--jobs 1` and `--jobs 8` | Identical standard output and standard error |
| `--batch 1` over any stream | Today's requests, rows, standard error and exit code, byte for byte |
| `--batch 0`, `--batch 1.5`, `--batch fill` | Exit 2, the pinned `--batch` sentence |
| `--batch max` | Fill to the limit |
| `THINKTHEN_BATCH=0` | Exit 2, the pinned `THINKTHEN_BATCH` sentence |
| File `"batch": 0`, `1.5`, `"10"` | Exit 5, the question-file refusal naming `batch` |
| File `"batch": 1`, no other setting | One record a request |
| File `"batch": 1`, `--batch 10` | At most 10 a request. The flag wins |
| File `"batch": 1`, `THINKTHEN_BATCH=max` | Fill to the limit. The environment beats the file |
| File `"batch": 2`, `--batch max` | Fill to the limit. `max` undoes the file's number |
| `--batch 5` on one document | Exit 2, the one-document sentence |
| `THINKTHEN_BATCH=5` on one document | Ignored. Today's request |
| `choose --batch 5` | Exit 2, clap's unknown-option refusal |
| `--dry-run` over three lines, default | The plan of one batch of three records. Its evidence is the fixed sentence, and its three questions quote the three lines |
| `--dry-run` over three lines, `--batch 1` | Today's plan of the first record |
| A live pipe with no folder that pauses with 3 records sent | A request of 3 records within 5 seconds |
| The same pipe under `--cache DIR` | A request of 3 records within 5 seconds. The pause fires in every mode |
| The same pipe with the default cache on and no typed folder | A request of 3 records within 5 seconds |
| The same pipe with `THINKTHEN_CACHE` set and no typed folder | A request of 3 records within 5 seconds |
| The same pipe under `--record DIR` | A request of 3 records within 5 seconds |
| A loop that writes one record and waits, under `--record DIR`, then the same loop under `--replay DIR` | Each answer arrives before the next record in both runs. The replay sends nothing |
| 10,000 lines of 5 distinct values, none a content cut, default, `--no-cache` | 3 requests of 4,096, 4,096 and 1,808 members. Under `--details` rows 1, 4,097 and 8,193 carry `requests_sent` 1, and every other row carries 0 |
| The same 10,000 lines at `--batch 5000` | The same 3 requests. The cap closes the batch at 4,096 before the size of 5,000 |
| The same lines, with a content-cut record replacing line 1,000 | 4 requests of 1,000, 4,096, 4,096 and 808 members. The run still holds 10,000 lines. The count starts again after the cut |
| The same 10,000 lines, the bodies compared | Repeats add no bytes. Each of the first two bodies holds the evidence sentence and exactly 5 questions, and the two bodies have the same length. Their bytes may differ, because 4,096 is not a multiple of 5 and the values first appear in another order |
| CSV or TSV rows through `over_table`, default | One batch. Each record is quoted as its compact JSON object. `filter` prints kept rows as compact JSON objects, as today |
| A question file whose question text is a JSON object | Every batch holds one record in today's form. Today's requests and rows |
| A batched row under `--details` | `question` is the user's question, unquoted. `meta.question_sha256` is today's |
| An annotate question set whose entry holds `batch` | Refused as today: `a question file takes no key `batch`` |
| A `decide` file with `"batch": 5` and the same file without it | The same `meta.question_sha256` |
| `audit --write` over a `decide` file holding `"batch": 5` | Writes the threshold and keeps `"batch": 5` in the file |
| `relate @FILE` over a `decide` file holding `batch` | Today's wrong-verb refusal |
| A `choose` file holding `batch` | Refused as today, through `parse`'s unknown-key form: `a question file takes no key `batch`` |
| The library reading a `decide` file holding `batch` | Accepted. The value is ignored until B12a |
| `THINKTHEN_BATCH=5` on `choose`, `tag` or `score` | Ignored. Today's requests |
| A record the planner refuses after a push closed a batch | The closed batch is sent and its rows print. The run stops at the refused record |
| A malformed JSONL line at record 7, default | Records 1 to 6 print. The run stops at record 7 with today's cause and exit code |
| A malformed JSONL line at record 7, `--batch 1 --jobs 4` | The same output, standard error and exit code as today. Records 8 and later are never sent, where today up to three of them may be in flight. This is an intended difference |
| A record over a profile's `max_evidence_bytes` alone | The records before it print. Today's profile refusal at that record |
| 503 on the second batch after retries, `--batch 10` | Rows 1 to 10 print. One stop line naming records 11 to 20. Exit 4 |
| A reply missing record 13's answer in batch 11 to 20 | Rows 1 to 12 print. The partial stop line. Exit 4 |
| The same failure at `--batch 1` | Today's two lines |
| `--replay` of a folder recorded at `--batch 10`, same settings | Every batch from disk, no network, identical output |
| `--replay` of that folder at `--batch 5` | Exit 5 at record 1, naming records 1 to 5 |
| `--cache DIR` run twice | The second run sends nothing |
| A demo recording made one record a request | Replays under `--batch 1` |
| `--details` on a 3-record batch billing 100 input and 10 output tokens | Rows carry 34, 33, 33 input, 4, 3, 3 output, and `requests_sent` 1, 0, 0 |
| A backend that reports no usage | No row carries `usage` |
| `rank --top 5` over 20 lines | Every record judged in batches. Five print |
| `filter` under `--details` | Only kept rows print. Each carries its own share |
| An interrupt mid-run | No new batch starts. Batches in flight finish and print in order |

## Proof

Every test drives the compiled binary against the in-process loopback (`tests/backend/harness`) or with `--dry-run`. The new tests live in `crates/thinkthen/tests/backend/batching.rs` and its folder. The loopback answers each wire question `qK` with a probability made from the record the question quotes, so a row's answer follows its record whatever the batch.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `order_holds_across_jobs`, design test 4 | 306 made-up lines at `--batch 10`. The loopback answers later batches faster, so replies arrive out of order. `--jobs 1` and `--jobs 8` print identical standard output and standard error. The loopback saw 31 requests, sized 30 of 10 and one of 6, and a peak of 8 in flight at `--jobs 8` | (a) Emit rows in reply order. (b) Close a size batch at N+1. (c) Bound the items in flight by records, not batches: the peak falls to 1 |
| `replay_answers_every_batch`, design test 5 | Record 25 lines at `--batch 10` into a temporary folder through the loopback. Replay with the loopback stopped: identical output. Replay at `--batch 5`: exit 5 and the pinned one-line stop naming records 1 to 5. A `--cache DIR` run repeated sends nothing the second time | (a) Digest the plan, not the body, so replay misses. (b) Give the replay-miss stop line the batch number, not the record range |
| `a_pause_sends_the_open_batch`, design test 7 | A real pipe carries 3 records from a writer that then holds the pipe open. The loopback receives one request of 3 records within 5 seconds in three rows: `--no-cache` with no folder, the default cache through `THINKTHEN_CACHE` naming a private folder, and `--cache DIR`. `--record DIR` is left to the scheduling rows below, which run it as a request-and-reply loop | (a) Never pause: no request arrives in 5 seconds. (b) Turn the pause off under a typed folder: the `--cache DIR` row gets no request in 5 seconds. (c) Turn the pause off by `Folders::named`: the default-cache row gets no request in 5 seconds |
| `an_answer_arrives_before_the_next_record_at_one_job_and_the_default`, existing in `tests/backend/scheduling.rs` | Two new rows: the same request-and-reply loop under `--record DIR`, then under `--replay DIR` from that folder. Each answer arrives before the next record. The replay row sends nothing | (a) Turn the pause off under a typed folder: the `--record` row times out |
| `repeats_close_a_batch_at_the_member_cap`, new | The four cap edge rows, `decide --details --no-cache` at the loopback. It pins the request count and members per request, and the `requests_sent` of every row: 1 at rows 1, 4,097 and 8,193 and 0 elsewhere, so a batch closes at exactly 4,096. The `--batch 5000` row pins the same 3 requests. The content-cut row, whose line 1,000 is replaced by a record the batching fixture README names as a cut, pins 1,000, 4,096, 4,096 and 808. It also pins that each of the first two bodies holds the evidence sentence and exactly 5 questions, and that the two bodies have the same length | (a) No cap: 1 request. (b) Cap on distinct records, not members: 1 request. (c) Close at 4,097: row 4,097 carries 0 and row 4,098 carries 1. (d) Let `--batch 5000` override the cap: 2 requests. (e) Keep counting members across a content cut: the content-cut row sees 1,000, 3,096 and so on. (f) Ask a repeat's question again: the first body holds 4,096 questions, not 5 |
| `a_failed_batch_stops_at_its_first_record`, design test 8 | Loopback answers 503 to the second of three batches at `--batch 10`, `--jobs 1`, `--max-retries 0`. Standard output is exactly rows 1 to 10. Standard error is exactly the pinned 503 line from "Stop lines". Exit 4. A second row omits record 13's answer: rows 1 to 12 print, then the pinned partial line, exit 4. A third row runs the 503 at `--batch 1`: standard error is exactly today's two lines. A fourth row interrupts a batched run: standard error keeps today's two-line cancellation form | (a) Name the range from `finished`, off by one. (b) Fail the whole batch on a partial reply: only 10 rows print. (c) Use the one-line form for a batch of one. (d) Use the one-line form for a cancellation |
| `the_batch_setting_follows_its_tiers` | An edge-case table over `--dry-run`, with no network. Each row gives the flag, `THINKTHEN_BATCH` and a question file's `batch`, and reads how many records the first batch plan holds over three lines. The refusal rows pin exit codes and whole sentences. Two rows run through the loopback with `--details`: a `decide` file with and without `"batch": 5` print the same `meta.question_sha256`, and an annotate question set whose entry holds `batch` is refused with today's sentence | (a) The file beats the environment. (b) `THINKTHEN_BATCH` is never read. (c) `0` parses as `max`. (d) `--batch` on one document is ignored. (e) `batch` joins `Verb::Decide.keys()`: the annotate row is accepted |
| `each_row_carries_its_share` | Three records, `Come Together`, `Because` and `Come Together`, at `--batch 3` with `--details` and `--model jev-latest`, loopback usage of 100 input and 10 output tokens. The loopback's one request body equals `specification/fixtures/systemone/batch-duplicate.request.json` without its trailing newline, so the command sends ADR 0055's evidence sentence and each distinct record once. Rows carry 34, 33, 33 and 4, 3, 3, and `requests_sent` 1, 0, 0. A backend reporting no usage gives rows with no `usage` | (a) Every row carries the whole batch's usage. (b) The remainder goes to the last record. (c) A missing usage becomes 0. (d) Keep `{"records":[…]}` as the evidence of a decide batch: the body differs from the fixture. (e) Send the batch's records in the evidence beside the sentence: the body differs |
| The planner's tests in `core/batch/tests.rs`, existing, with the regenerated fixtures and the changed limit rows from "The evidence of a batch" | A decide batch of two or more distinct records sends the sentence, and each question quotes its record as ADR 0048 item 1 does. A choose batch keeps the records list. A context batch keeps the context. A batch of one stays today's request. Evidence and request limits count the sentence once and each record once | (a) Keep the records list for a decide batch: `batch-three` differs. (b) Send the sentence for a choose question: `batch-choose` differs. (c) Send the sentence in a batch of one distinct record: `a_batch_of_one_is_todays_request` fails. (d) Send the sentence beside a context: `batch-context` differs. (e) Count two separators a record in a quoted batch: `built()` returns its byte-count defect. (f) Count the records' list bytes against `max_evidence_bytes` in a quoted batch: the 44-byte row splits |
| S1's gate part, `crates/thinkthen/tests/speed.rs` | With the `list` field removed from their rows, `decide`, `filter` and `rank` each send 1 request for 12 lines | (a) The default setting is `Records(1)`: the gate fails rule 3 for all three |
| The demo runner, `crates/thinkthen/tests/demo_runner.rs` | Every green demo replays under its pins | (a) Quote the record in a batch of one: every pinned demo misses its recording |

The four questions:

- **`order_holds_across_jobs`.** It protects ADR 0048 item 5: output keeps input order and `--jobs` counts batches. Out-of-order replies or a jobs bound in records fail it. B3 tests only the planner, and today's order tests send one record an item. No hook: the loopback's reply delay is an ordinary server choice.
- **`replay_answers_every_batch`.** It protects replay and the cache over batches, which every recorded demo and every saved cache depends on. A digest over other bytes, or a stop line in batch numbers, fails it. No existing test replays a batch. No hook.
- **`a_pause_sends_the_open_batch`.** It protects a live stream's latency in every mode. A missing pause, or a pause turned off by a typed or default folder, fails it. Nothing tests timing of input today. No hook: it uses a real pipe.
- **The new scheduling rows.** They protect the request-and-reply loop under a named folder, which the README recommends. A pause turned off by a typed folder fails them. The existing test already runs under the default cache, because `raw_child` sets `XDG_CACHE_HOME` and passes no `--no-cache`, so it guards the default cache. It names no folder, so it cannot catch a pause turned off by `--record` or `--replay`. No hook.
- **`repeats_close_a_batch_at_the_member_cap`.** It protects ADR 0053 item 2 and `records.md`'s promise that the memory of a long run stays flat. A missing cap, a cap that counts distinct records, a cap off by one, a size that beats the cap, or a count that survives a content cut fails it. The planner's tests pin no cap. No hook: the loopback counts requests.
- **`a_failed_batch_stops_at_its_first_record`.** It protects ADR 0048 item 6: which rows print and what the stop line says. A range off by one, a whole-batch failure on a partial reply, or a changed `--batch 1` line fails it. Today's failure tests send one record a request. No hook.
- **`the_batch_setting_follows_its_tiers`.** It protects ADR 0048 items 3 and 4, and the refusals. A swapped tier or a lax parser fails it. Nothing reads the setting today. No hook: `--dry-run` is the real boundary for what a run would send.
- **`each_row_carries_its_share`.** It protects the truth of every row's usage once rows come from batches, and the batch's wire bytes at the loopback. Whole-batch usage on each row, a 0 for a missing count, or a records list in a decide batch fails it. The planner's fixture test checks the planner's bytes. Only this test checks the bytes the command sends through `Engine::ask_batch`. B5's test 6 later checks sums over recorded runs and `--facts`. This test checks one batch's split at the row. No hook.
- **The planner's tests** already exist. B4 changes their expected values for ADR 0055, not their shape. They protect the planner's form for every question kind.
- **S1's gate and the demo runner** already exist. B4 changes their inputs, not their code.

Existing tests pinned to `--batch 1` keep proving what they proved before, one record a request.

## Budgets

Nonblank lines, measured with `grep -c .`. Net lines against main after 0143, 0144, 0145, 0158 and 0161 land.

- `crates/thinkthen/src/core/batch.rs`: at most 46 net, for `Setting::parse`, `Closed::Pause`, `pause()`, the member cap and ADR 0055's evidence sentence, less the `expect(dead_code)`. The sentence takes 8 of the 46: `QUOTED`, the `shared` field, and its choice by question kind.
- `crates/thinkthen/src/core/question_file.rs` and its folder: at most 25 net, for `parse_top`.
- `crates/thinkthen/src/core/relate_file.rs`, `cli/audit/write.rs` and `public/question.rs`: at most 8 net together, one `parse_top` call each.
- `crates/thinkthen/src/core/result.rs`: at most 20 net, for the even shares.
- `crates/thinkthen/src/result_json.rs`: at most 10 net.
- `crates/thinkthen/src/public/results.rs`: at most 3 net, passing a share of the whole.
- `crates/thinkthen/src/engine/schedule.rs`: at most 30 net, for the two fields, `Completed::one` and the record counts.
- `crates/thinkthen/src/engine/facade.rs`: at most 20 net, for `ask_batch`.
- `crates/thinkthen/src/engine/annotate_schedule.rs` and `public/batch.rs`: at most 8 net together, for the second type parameter.
- `crates/thinkthen/src/public/bulk.rs`, `cli/schedule.rs` and `cli/annotate_schedule.rs`: at most 10 net together, one `Completed::one` call per builder and the type parameter.
- `crates/thinkthen/src/cli/asking/batched.rs`: at most 250, new.
- `crates/thinkthen/src/cli/asking.rs`: at most 25 net.
- `crates/thinkthen/src/cli/args.rs`: at most 20 net.
- `crates/thinkthen/src/cli/edge.rs`, `judge.rs` and `asked.rs`: at most 35 net together.
- `crates/thinkthen/src/cli/failure.rs` and its folder: at most 45 net.
- Product code total: at most 561 net. The lines above sum to 555.
- `crates/thinkthen/tests/backend/batching.rs` and its folder: at most 480.
- `crates/thinkthen/src/core/batch/tests.rs` and its folder: at most 10 net, for the changed limit rows. Most changes replace values in place.
- `specification/fixtures/systemone/`: three fixtures change, one line each, and the README changes at most 4 lines.
- Unit tests that build or name `Completed`, including `cli/conformance_tests/runner.rs`: at most 12 net.
- Existing tests: at most 110 net for the `--batch 1` pins, sized for 20 to 32 files at a few lines each, most through a shared helper. At most 20 net more in `tests/backend/scheduling.rs` for the two folder rows.
- Pages under `specification/`, outside its fixtures: at most 46 net lines together. `spec/decide.md`: at most 10 net.
- Demos: at most 30 changed lines, pins and dry-run expectations only.
- `sdlc/ratchet.json` moves to the measured total, at most 1,193 above main after 0143, 0144, 0145, 0158 and 0161 land: 561, 480, 10, 12, 110 and 20. ADR 0055 adds 18 of those lines, 8 in `core/batch.rs` and 10 in its tests. The commit says what grew.
- No dependency.
- The `surfaces` rung runs, because the engine scheduler that the libraries share changes.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if 0143, 0144 or 0145 has not landed on main, if no "S1 live run 1" record is on main, if ADR 0053 is not on main, or if ticket 0158 has not landed on main.
3. Stop if `--batch 1` changes one byte of any request, row, standard error line or exit code that today's build gives. The existing suite and the demos under their pins decide this. One difference is intended: at `--jobs` above 1, a malformed or refused record stops the run before any later record is sent. Today up to `--jobs - 1` later records may already be in flight. Output, standard error and the exit code stay the same.
4. Stop if the design needs a second scheduler or a second row builder.
5. Stop if more than 40 existing test files fail under the default before any pin, or if the pins pass their budget. Hand back the list. The first build step reports the real count either way.
6. Stop if any plant stays green.
7. Stop if the change needs a file in `sdlc/scripts/` or `site/`, or a file that another in-flight ticket opens and that this order does not settle. Ticket 0158 opens `specification/records.md`, `sdlc/ratchet.json` and `crates/thinkthen/tests/backend/main.rs`. It lands before B4 builds, so B4 builds on its lines. Ticket 0154 opens `cli/asking/batched.rs` and `core/batch.rs`. It builds after B4 lands, so it merges B4's lines.
8. Stop if the build needs a live call. None is authorized here. Never run `sdlc/scripts/live`, S1's `job.sh`, or its live mode.
9. Stop if a probe under `probes/NN-*` fails its replay under the default. Hand back which one.
10. Stop if ADR 0055's change moves one byte of `batch-choose` or `batch-context`, or of any batch of one distinct record.

## After landing: "B4 live run"

S1's live job measures the target on a named build once B4 is on main. Its help probing finds `--batch` and adds the `--batch 1` arm with no edit. The coordinator brings Ian S1's `plan` output and the charge, and asks him to authorize "B4 live run" by name. The run's record says `met` or `not met` for `filter` over the 306 titles under half a second at the default throttle. B4 lands without it. A `not met` result becomes an issue. It does not revert B4.

## Scope and exclusions

Excluded: `meta.batch`, `--facts` and design test 6 (B5). The batch form of `choose` and `tag` (B8, B9). The calibration warning, `meta.batch_warning`, and `audit --write` writing `batch` (B16). `--context` (B7). `choose`, `tag`, `score` and `annotate` batching (B8, B9, B10). Recognize (R7). Libraries and SQL, including any library reading `THINKTHEN_BATCH` (B12a to B13e). The accuracy measurement (B6). The documentation page (D1). Re-recording demos under the default. `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code. The change raises the ceiling and adds a setting, so the code review names what it checked.

## Complexity

Contract 2; state and timing 2; reach 2; proof 2; cost of error 2; total 10. Final level: 3. The risks are a `--batch 1` run that drifts from today, which voids every recording, rows printed out of order or stopped at the wrong record, a pause that fires while reading a file and moves every digest, a pause that fails to fire under a folder and hangs a request-and-reply loop, and a decide batch that still sends the records list. The `--batch 1` pins, tests 4, 5, 7 and 8, the scheduling rows, and the demos guard the first four. The regenerated fixtures and `each_row_carries_its_share` guard the last.

## Deferred gaps

1. `meta.batch`, the `thinkthen.run/1` line under `--facts`, and design test 6 over recorded runs. Ticket B5.
2. The batch setting as calibration identity. Ticket B16. The item 8 markers stay.
3. The demos and pages pinned to `--batch 1`. An authorized live run records them again under the default, and the pins come off. Ticket D1, or its own ticket, owns that run.
4. The time target, measured live. "B4 live run" after landing, authorized by Ian.
5. The accuracy cost of the default. Ticket B6.
6. D1's page should say how a batch carries its records. Ticket 0144's deferred gap 2 asked it to say each record goes twice. Under ADR 0055 a batch of `decide`, `filter` or `rank` carries each record once. B4 states it on `backends.md` and `records.md`, and D1 carries it to the page.
7. The libraries still ask one record a request. A library run and a command run over the same records share cached answers only at `--batch 1`, until B12a.
8. A slow disk or network file system that blocks a read for 50 ms can cut a batch early, in every mode. It changes no answer's meaning. It adds a request, and under a folder a replay with other timing can miss that batch at exit 5.
9. `choose`, `tag` and `score` keep their item 7 markers until B8 and B9.
10. The batch form of `choose` and `tag` on long records. Local experiment 275 measured them on short titles only, where the records list won beyond the noise. B8 and B9 measure them on long records, by ADR 0055 item 4. The planner keeps the records list for them until then.
11. The accuracy of ADR 0055's form through the command. Local experiment 275 measured the quoted form before the command could batch. B6's test 9 measures it through the command after B4 lands.

## What Ian can overturn

- Decision 6: the pause fires in every mode, by ADR 0053 item 1. The coordinator ruled it.
- Decision 16: the member cap of 4,096, by ADR 0053 item 2. The coordinator ruled it.
- Decision 7: B4 builds the usage shares, ahead of B5. The first ticket review confirmed it.
- Decision 8: one line for a failed batch of two or more, today's two lines for a batch of one and for cancellation, input refusals and `RecordingStorage`.
- Decision 9: the partial reply's line names the record, not the cause, at exit 4.
- Decision 10: `--batch` on one document is refused.
- Decision 11: a bad `THINKTHEN_BATCH` exits 2.
- Decision 12: demo and page commands that replay pin `--batch 1` until re-recorded.
- Decision 13: B4 lands before the live target run. The first ticket review confirmed it.
- Decision 14: `batch` comes off the top of a `decide` file before parsing, so annotate entries still refuse it.
- Decision 17: the planner picks the evidence by question kind, and keeps the records list for `choose`, `tag` and `score`.
- Decision 18: B4 builds ADR 0055 before it lands, so no batch of the records-list form is ever recorded.
- ADR 0055 itself is Ian's ruling of 2026-09-26, and he can overturn it there. Its fixed sentence is taken unchanged from the arm local experiment 275 measured.
- The intended difference under stop rule 3: a refused record stops later records from being sent.
- The coordinator's order: B4 builds after 0143, 0144 and 0145 land, after "S1 live run 1", with ADR 0053 on main, and after ticket 0158 lands. Ticket 0154 builds after B4 lands.

## Closes

No issue. `sdlc/issues/2026-09-26-batching-design.md` stays open until its last ticket lands.

## Evidence

- Starts from: The B4 row, sections 1 to 6, the edge cases and acceptance tests 4, 5, 7, 8 and 12 of `sdlc/issues/2026-09-26-batching-design.md`, and Ian's rulings 1, 3, 4 and 9 there. ADR 0048 items 1 to 7, 9 and 13, its ticket table, and its marker rule. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` section 7: 306 titles in one request answered in 0.30 to 0.39 s against 13.0 to 14.2 s one a request, and section 9: no content cut among the 306 titles. `specification/settings.md` from ticket 0140. Ticket 0144's planner and its deferred gaps 2 and 3, read from `ticket/0144-engine-plans-batches`. Ticket 0145's gate list and its decision 13, read from `ticket/0145-speed-test`. Ticket 0143's change to `engine/facade.rs`, read from `ticket/0143-relate-runs-at-once`. ADR 0053 items 1, 2, 4 and 5, and section 14 of the evidence record, from the batching review in `sdlc/issues/2026-09-26-batching-design-review-before-0146.md`. ADR 0055, accepted 2026-09-26 by Ian's ruling, and its measured table from local experiment 275: four yes/no tasks over the 306 Beatles songs, three same-bytes repeats and three shuffled orders for each batched form. The quoted form stayed within 4 answers over repeats and orders, never fell more than 3 below one record a request, and cost 16 to 17% fewer input tokens than the records list on titles and 48% fewer on cards. Its one named loss is the release-year title task, 255 to 259 against the records list's 258 to 283, a gain that record order alone erases. `sdlc/issues/2026-09-26-a-batch-sends-each-record-twice-and-its-question-with-every-record.md` holds the argument. ADR 0054 items 3 and 5, which make the cheaper form the default inside the noise. `core/batch.rs` `built()` and `plan()` at `a18a8345`, and the five `batch-` fixtures. The code at `origin/main` `7850db3f`: `cli/asking.rs`, `cli/schedule.rs`, `cli/judge.rs`, `cli/failure.rs`, `engine/schedule.rs`, `engine/facade.rs` and `core/reply.rs`.
- Keeps: Every request, row, standard error line and exit code at `--batch 1` and on a stream of one record. The quoted question of ADR 0048 item 1, byte for byte. The planner's records-list form for `choose`, `tag` and `score`, and its context form. Every run on one document. `choose`, `tag`, `score`, `annotate`, `find`, `recognize` and `relate`. The scheduler's bound and order. Every recording, replayed under its pin. The libraries' behavior.
- Changes: `decide`, `filter` and `rank` over a stream fill each request to the limit by default. `--batch`, `THINKTHEN_BATCH` and the question file's `batch` set the limit, in four tiers. A batch of two or more distinct records sends `Each question quotes the text it asks about.` as its evidence, by ADR 0055, and three `batch-` fixtures change to match. `records.md` and `decide.md` drop the caution that records of one batch are evidence for each other on these verbs. `--jobs` counts batches. The pause sends a waiting live batch after 50 ms in every mode. A batch closes at 4,096 members. A failed batch stops at its first record with a one-line range. Rows carry even usage shares. Dry run plans the first batch. The reader queues closed batches and pulls records only for an outstanding ask, through a bounded channel. `batch` comes off the top of a `decide` file before parsing, so annotate entries still refuse it. The item 1 and 2 markers leave the specification, and the settings table gains `batch`. B4 removes the `list` field from the `decide`, `filter` and `rank` rows of `probes/speed/functions.jsonl`. The rows stay.
- Proof: The six new outside-in tests under "Proof", each with its plants. The planner's existing tests with the regenerated fixtures and their plants. S1's gate with the `list` field removed from the three rows. The demo runner under the pins. The existing suite at `--batch 1` where pinned. The `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: `meta.batch` and `--facts` (B5). The batch form of `choose` and `tag` on long records (B8, B9). Calibration identity (B16). Re-recording pinned demos under the default. The live target run after landing. The accuracy cost (B6). D1's page. Library and SQL batching (B12a to B13e). A slow read cutting a live batch. The item 7 markers for the other verbs.
