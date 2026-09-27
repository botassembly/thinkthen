---
flow: build
priority: 161
opens: crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/annotate/plan.rs crates/thinkthen/src/cli/annotate crates/thinkthen/src/core/question_set.rs crates/thinkthen/src/core/records.rs crates/thinkthen/src/core/mod.rs crates/thinkthen/src/core/plan_document.rs crates/thinkthen/src/public/bulk.rs crates/thinkthen/src/cli/conformance_tests.rs crates/thinkthen/src/engine/facade_tests/annotate_order.rs crates/thinkthen/tests/backend/annotate_on.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/backend/annotate/splitting.rs conformance/consumer/consumer/tests/public/parts.rs specification/annotate.md spec/annotate.md spec/fixtures/annotate demos/16-triage-pipeline/README.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0161: `annotate` reads what its set names, and its plan shows what it sends

Status: landed 2026-09-26 (`sdlc/records/0161-build-annotate-reads-what-it-names.md`). A fresh read-only code review found two fixes, both made. The coordinator accepted the ticket on 2026-09-26 after three fresh read-only reviews and a coordinator fix to the document refusal line. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A question set's `on` pointer reads inside the JSON value the record selected. It never re-reads a string as a JSON document. So a record's result no longer depends on whether a string happens to hold JSON. The command and the libraries follow one rule, and each parses a record once. A set that points `on` into text is refused with a sentence that names the question. Every example on `specification/annotate.md` runs from a new executable page and exits as the page says. Under a backend profile, `--dry-run` prints the first request the run would really send and the number of requests each group makes.

Three issues from local experiment 273, report 03, block 0.1 under the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The spec-example issue asks that it and the `on` issue be fixed together. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `e24f7324`, after ticket 0150 landed.

- `core/question_set.rs:293-309`, `QuestionSet::group_evidence`, holds the parts rule. For a group whose `on` is not the root, it writes the evidence it was given back to text with `record.as_text()` and parses that text again with `reading.record(text.as_bytes())`, under document framing. Four callers hand it an `Evidence`: the command's `plan_for` (`cli/annotate.rs:283-311`, the call at 293-299), the libraries' `annotated` (`public/bulk.rs:283-311`, the call at 294), the command's conformance runner (`cli/conformance_tests.rs:127`) and the facade order test (`engine/facade_tests/annotate_order.rs:37`).
- In the command, a `/body` string holding JSON text is parsed, and `on` sends part of it. A `/body` string of plain text stops the run at exit 2 with `the input is not valid JSON`, and the message names neither the question nor `on`. A `--lines` line that looks like JSON gets members, though `records.md` says a text line has none. Finding 2-2 ran all three (`sdlc/issues/2026-09-26-annotate-on-reparses-selected-text-as-json.md`).
- In the libraries, `public/set.rs` accepts a set whose members name parts, since 0150. Ticket 0150's decision 3 makes each library record JSON text. `public/bulk.rs` wraps that text as `Evidence`, and `group_evidence` parses it once more for each group that reads a part. A record that is not JSON text is refused with `the input is not valid JSON: the JSON at line 1 column 1 is not one` (`conformance/consumer/consumer/tests/public/parts.rs:114`).
- `core/records.rs::annotation_record` reads a whole document as a JSON value when it parses and as text otherwise. So `annotate set.json < record.json` with a JSON object works today, and `annotate.md`, "What it prints", promises that an object document gains the answers.
- `core/records.rs`: `Reading::evidence` cannot tell the builder what the base reading selected. The private `whole` (line 468) turns a whole JSON record into compact text, so its `Evidence` looks the same as a `--field` string that holds JSON. `Reading::batch_record` (line 417) returns that evidence beside the selected JSON value. It carries `expect(dead_code)` (line 415), reserved for ticket 0146.
- `core/records.rs:46` and `:86`: `ReadingError::TextHasNoMembers` and `RecordError::TextHasNoMembers` carry no question name, and their sentence begins `--field:`. `cli/failure.rs` maps `Failure::Reading` and `Failure::Record` to exit 2 with the error's own sentence (lines 219 and 223). `Failure::Usage` holds only a `&'static str`.
- `annotate.md` line 34 says `on` "is a JSON Pointer inside the evidence that `--field` selected", and "an object or a list travels as that JSON value".
- `annotate.md` defines `triage.json`, whose `unresolved` question reads `"on": "/body"` (line 23). Its examples run the set on a text document (`< issue.txt`, line 81) and on `--jsonl --field /body` (line 85). Both exit 2 with `the input is not valid JSON`. `spec/` has no annotate page, so no rung runs them. Finding 2-1 ran them (`sdlc/issues/2026-09-26-annotate-spec-example-fails-on-its-own-inputs.md`).
- `cli/annotate/plan.rs:58-62` calls `facade::split` for each group, discards the chunks, and prints `plans.first()`, the unsplit group. Under a profile that caps questions per request, the dry run shows one request holding every question. Finding 2-3 printed 1,500 questions under a profile of 100. Live, a 250-question set under a 100-question profile showed one request and sent 3 (`sdlc/issues/2026-09-26-annotate-dry-run-under-a-profile-prints-the-unsplit-request.md`). `annotate.md` line 94 says the dry run "prints the first request". Line 96 says "One record makes one request per distinct `on`, and the plan shows one request". How-to 16 says the dry run "shows the exact request".

## Design

### `on` reads the selected value

`group_evidence` takes the record's evidence and its selected JSON value together, as a `BatchRecord`, and never parses text. Its signature becomes `group_evidence(&self, group: &[usize], record: &BatchRecord) -> Result<Evidence, PartError>`.

- A group that reads the root gets `record.evidence`, as today.
- When `record.value` is an object or an array, the group's pointers select inside it by the `state` rule of `records.md`, as `annotate.md` line 34 says. `core/records.rs` gains `Reading::part(&self, value: &Json) -> Result<Evidence, RecordError>`. It runs the two JSON arms of `Reading::selected` over a value. `selected` calls the same helper for a JSON record, so the rule lives once.
- Otherwise the value is text: a string, a number, a boolean or `null`. Text has no members. `group_evidence` returns `PartError::Record(RecordError::TextPart(name))`, where `name` is the first question of the group.

`PartError::Render` goes, because `group_evidence` no longer writes text. Both callers drop its match arm.

The value comes from `Reading::batch_record`, which already returns the selection with its type kept. A `--lines` line, a document that did not parse, and a selected string all become `Json::String`. A whole JSON record keeps its value, not its compact text. A CSV or TSV row is an object. This ticket calls `batch_record` from the command, so it removes that function's `expect(dead_code)` in `core/records.rs`. The module mark on `core/batch.rs` stays, because the batcher is still unused. Ticket 0146 then calls `batch_record` from its reader as its design says, and finds no mark to remove in `core/records.rs`, a file it does not open.

The two callers read each record once:

- The command: `plan_for` builds `base.batch_record(record)?` in place of `base.evidence(record)?` and passes it to `group_evidence`. Nothing is parsed again. `batch_record` copies the selected value once for each group, and a set has few groups.
- The libraries: `public/bulk.rs::annotated` parses the record only when the set has a part group, that is when `set.first_part().is_some()`. Then it reads the record text once with `Reading::new(Framing::Document, vec![])` and `annotation_record`, which is the command's rule for a whole document. It takes `batch_record(&record)?.value`. The record's `evidence` stays `evidence(text)?`, so a root group sends the text as the library sends it today. It builds one `BatchRecord` and passes it to every group. A record error maps to `Error::usage` with its own sentence, as `PartError::Record` does today.
- A root-only library set keeps today's path, byte for byte. `bulk.rs` builds its `BatchRecord` from `evidence(text)?` and the text as a `Json::String`, and it never parses. Every group reads the root, so no group reads the value. `annotation_record` (`core/records.rs:385-392`) returns `Err` for `TooLarge` and for the parser's `DuplicateName` and `NotFinite`. A parse would refuse `{"a":1,"a":2}`, `[1e999]` and text over 16 MiB, which a root-only set answers from text today. Skipping it also follows ADR 0054: no parse runs whose result nothing reads. This is the coordinator's ruling.
- `cli/conformance_tests.rs` and `engine/facade_tests/annotate_order.rs` build their `BatchRecord` the same way. `core/mod.rs` re-exports `BatchRecord` if the callers need it.

### Refusals name the question

`core/records.rs` gains two variants. Each holds the question name, which comes from the set file and never from a record.

- `ReadingError::LinesPart(String)`, displayed as ``question `{0}` reads `on`, and a --lines record is text with no members``.
- `RecordError::TextPart(String)`, displayed as ``question `{0}` reads `on`, and this record's evidence is text with no members``.

`core/question_set.rs` gains `QuestionSet::first_part(&self) -> Option<&str>`, the name of the first question in file order whose `on` is not the root. `cli/annotate.rs::run` checks it right after `reading(...)` and before `edge::source`. Under `--lines` a set with such a question is refused through `Failure::Reading(ReadingError::LinesPart(name))`, before any input is read. A record whose selection is text is refused through `Failure::Record(RecordError::TextPart(name))`, before any request for it. That follows `annotate.md`'s rule for an input error in one record. Both pass through the existing arms at exit 2, so `cli/failure.rs` does not change.

The whole standard error, for the set on `annotate.md`:

- Under `--lines`, one line, printed before any input is read, with no request:

  ```text
  thinkthen: question `unresolved` reads `on`, and a --lines record is text with no members
  ```

- For a stream record (`--jsonl`, `--csv` or `--tsv`) whose selection is text, two lines. The refusal stops the stream at that record, and `cli/failure.rs` adds the stop line, as `tests/backend/annotate.rs:170-175` pins for another refused record:

  ```text
  thinkthen: question `unresolved` reads `on`, and this record's evidence is text with no members
  thinkthen: stopped at record 1; 0 records finished
  ```
- For a document whose selection is text, one line. The document path also runs through `annotate_schedule::run`, but `cli/annotate.rs` passes `reading.streams()`, which is false for a document. `engine/annotate_schedule.rs::outcome` then returns `Outcome::Failed`, not `Outcome::Stopped`, so `failure.rs::stopped` never runs:

  ```text
  thinkthen: question `unresolved` reads `on`, and this record's evidence is text with no members
  ```

Each stream row pins both lines, and the document row pins its one line. A library caller sees the first sentence of the stream refusal alone, as `Usage`.

### The page's examples run

`triage.json` keeps `"on": "/body"`, because it shows the feature. The examples change their input to match it:

- `thinkthen annotate triage.json < issue.json`, where `issue.json` is one JSON object with `id` and `body`. A document object gains the answers.
- `thinkthen annotate triage.json --jsonl < issues.jsonl`, with no `--field`.
- `thinkthen annotate triage.json --dry-run < issue.json`.

A new executable page, `spec/annotate.md`, runs each example on the page. It replays hand-built `local-1` entries under `spec/fixtures/annotate/`, as `spec/relate.md` replays `spec/fixtures/relate-partial`. The page names `--model local-1 --no-cache --replay`, so no key or network is read. It runs only the page's examples. Edge row 4 in `annotate_on.rs` already pins the `--lines` refusal.

### The dry run prints the request it would send

Under a profile, `--dry-run` prints the first chunk of the first group, which is the first request a live run sends. The plan object gains two members after `on`: `request_count`, the number of requests the first record makes, and `group_requests`, the number each group makes, in group order. Without a profile, or when every group fits, the plan prints the single request as today and adds the two counts. `recognize-plan` and `relate-plan` already name `request_count`, so the word matches.

### Page edits

`specification/annotate.md`:

- Line 34 becomes: "`on` is a JSON Pointer inside the value that the record selected: the whole record, or what `--field` selected. A string is text, even when it holds JSON, and `on` never parses it. A text record or a selected string has no members, and a question that reads `on` in one is refused at exit 2 with a sentence naming the question." The rest of the paragraph stays.
- Lines 81, 85 and 89: the three examples above.
- Line 94: the dry run prints the first request a live run sends, which is the first chunk under a profile, and states `request_count` and `group_requests`.
- Line 96 becomes: "One record makes at least one request per distinct `on`, and a profile can split a group into several. The plan prints one request: the first that the first record's first `on` set sends, taking the questions in file order. `request_count` and `group_requests` count the rest." The sentences after it stay.

How-to 16's "shows the exact request" then holds. The how-to adds the two members to its example if it prints one.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **`on` reads JSON values only.** A string is text even when it holds JSON. This keeps a record's result independent of its data.
2. **A whole document that parses as JSON keeps its members.** `annotate` already reads such a document as a JSON value, and the page promises it.
3. **`--lines` with a non-root `on` is refused before reading.** Every line would fail, so the run should not start.
4. **The spec examples change their input, not the set.**
5. **The plan gains `request_count` and `group_requests`.**
6. **One parse, through `batch_record`.** The value a batch quotes is the value `on` reads, so the one function serves both. It keeps the type that `Evidence` loses.
7. **The libraries read a record as the command reads a document.** A library record that is not JSON text now gets the named text sentence instead of the parser's sentence, when the set has a part group. A root-only set never parses the record, so it answers from the whole text exactly as today.

## Edge cases

| Input | Expected |
| --- | --- |
| `--jsonl` record `{"body":"{\"x\":1}"}`, `--field /body`, a question with `on: /x` | Refused for that record at exit 2 with the two stream lines. No request. Today it sends `1` |
| `--jsonl` record `{"body":"plain words"}`, `--field /body`, the same question | Refused at exit 2 with the two stream lines. No request. Today it says the input is not valid JSON |
| `--jsonl` record `{"body":{"x":1}}`, `--field /body`, `on: /x` | The question reads `1`, as today |
| `--lines` with a line `{"x":1}` and a question with `on: /x` | Refused before reading at exit 2 with the one `--lines` line and no stop line. No request |
| `--lines` with empty input and a question with `on: /x` | Refused before reading at exit 2 with the one `--lines` line. A check on each record would see no record and exit 0 with nothing printed |
| `--lines` with a set whose questions have no `on` | Runs as today |
| A document `{"body":"text"}` with `on: /body` | The question reads `text` |
| A plain-text document with `on: /body` | Refused at exit 2 with the one document line. No request |
| A 250-question set, a profile capping 100 questions a request, `--dry-run` | The printed request holds 100 questions. `request_count` is 3. `group_requests` is `[3]` |
| A set with two `on` groups, no profile, `--dry-run` | The first group's request, `request_count` 2, `group_requests` `[1,1]` |
| Library, the two-group set over `a private note` | `Usage`: ``question `summary` reads `on`, and this record's evidence is text with no members``. No request |
| Library, a root-only set over the record `{"a":1,"a":2}` | Answers as today. Its one request sends that text, byte for byte. The duplicate name is not refused |

## Proof

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `on_reads_the_selected_value`, new in `tests/backend/annotate_on.rs` | Edge rows 1 to 8. Each row pins standard output, the whole standard error and the exit code, and counts the loopback's requests, so a refused record sends none. Rows 1 and 2 pin both stream lines. Row 8 pins the one document line. Rows 4 and 5 pin the one `--lines` line | (a) Re-parse text again: rows 1 and 2 differ. (b) Refuse every non-root `on`: rows 3 and 7 fail. (c) Refuse `--lines` in each record, after reading: row 5 exits 0 with nothing printed, and row 4 gains the stop line. `annotate_schedule.rs:95-104` prepares every group before anything is sent, so the plant sends no request and prints no row, and the counts cannot catch it. (g) Pass `Reading::evidence` to `group_evidence` and read its text: row 1 sends `1` |
| `the_plan_shows_each_request`, new in `tests/backend/annotate/splitting.rs` | Edge rows 9 and 10 through `--dry-run`, pinning the question count of the printed request and both new members | (d) Print the unsplit group: row 9 holds 250 questions. (e) Count groups, not chunks: row 9's `request_count` is 1 |
| `each_group_sees_its_part_and_a_bad_part_sends_nothing`, in the Rust consumer's `parts.rs` | Edge row 11 replaces its row that pins the parser's sentence. Edge row 12 is new. Its other rows and the secrecy check stay | (h) Keep the parser's refusal in `bulk.rs` before `group_evidence`: row 11 reads the old sentence. (i) Parse every library record, root-only sets included: row 12 is refused for its duplicate name |
| `spec/annotate.md`, new, in the `spec` rung | Every example on `annotate.md` | (f) Put `--field /body` back in the JSONL example: the page exits 2 |

The four questions:

- **What behavior does it protect?** `on` reads inside JSON and never re-reads text, on the command and the libraries. The page's examples. A dry run that shows the sent request and its count.
- **What credible regression fails it?** A return to re-parsing, a refusal that also blocks real JSON selections, and a plan that prints the unsplit group or counts groups.
- **Why does no existing test catch it?** No test puts JSON text inside a string for `on`, no rung runs the page's examples, and no dry-run test uses a profile that splits. The library row pins today's parser sentence.
- **Does it need a test-only hook?** No. The loopback counts requests, `--dry-run` is the real plan, and the page replays ordinary entries.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `cli/annotate.rs` and `cli/annotate/`: at most 40 net.
- `core/question_set.rs`: at most 15 net.
- `core/records.rs`: at most 25 net. `core/mod.rs`: at most 2 net.
- `core/plan_document.rs`: at most 15 net.
- `public/bulk.rs`: at most 15 net, for the part-group check and the root-only path.
- `cli/conformance_tests.rs` and `engine/facade_tests/annotate_order.rs`: at most 8 net together.
- `tests/backend/annotate_on.rs`: at most 150, new, and one `mod` line. `the_plan_shows_each_request` in `tests/backend/annotate/splitting.rs`: at most 80 net. The consumer's `parts.rs`: at most 30 net, for rows 11 and 12.
- `spec/annotate.md`: at most 90, new. `spec/fixtures/annotate/`: hand-built `local-1` entries only.
- Pages: at most 20 net.
- `sdlc/ratchet.json` moves to the measured total in the commit that needs it, and that commit defends the number as `ratchet.mjs` requires. The budgets above bound it.
- No dependency. No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs `cli/failure.rs`, `public/set.rs`, or `core/batch.rs`.
3. Stop if a whole JSON document, a JSONL record, a CSV row or a library JSON record loses its members under `on`.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

It builds after tickets 0150 and 0159 land, because 0150 opens `cli/annotate.rs` and 0159 re-keys the fixtures.

It lands before ticket 0146 builds. 0146's reader calls `batch_record`. If 0146 built first, the function-level `expect(dead_code)` on `batch_record` at `core/records.rs:413-416` would go unfulfilled, and `cargo clippy -- -D warnings` in `sdlc/scripts/lint` (line 137) would fail. 0146 does not open `core/records.rs`, so it cannot remove that mark. This ticket calls `batch_record` and removes it. 0146 also opens `public/bulk.rs`, `engine/facade_tests` and all of `crates/thinkthen/tests`.

It never builds beside ticket 0162. 0162 opens `core/records.rs` and `cli/annotate.rs`, which this ticket also opens.

It may build beside ticket 0160. 0160 also opens `tests/backend/main.rs`, where this ticket adds one `mod` line. The second of the two to land merges that line.

## Scope and exclusions

Excluded: the mixed-model cache fix, host maps and frame rows as library records (0150 decision 3 defers them), `on` on single questions and `recognize` (0150 decision 8), and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 1; total 6. Final level: 2. The risks are a JSON record that loses its members, which rows 3 and 7 and the consumer's existing rows guard, and a library change, which rows 11 and 12 pin.

## Deferred gaps

- A binding test of the named text sentence. Each binding calls the Rust library, so the rule reaches it. At `e24f7324` no binding test pins the parser's sentence. `git grep` finds it only in the Rust consumer's `parts.rs` and in `records.md`. The Rust consumer's row is the one library proof.

## What Ian can overturn

- Decision 1: `on` never parses a string.
- Decision 3: a non-root `on` under `--lines` is refused before reading.
- Decision 5: the two new plan members.
- Decision 7: a library record that is not JSON text gets the named text sentence.

## Closes

`sdlc/issues/2026-09-26-annotate-on-reparses-selected-text-as-json.md`, `sdlc/issues/2026-09-26-annotate-spec-example-fails-on-its-own-inputs.md`, and `sdlc/issues/2026-09-26-annotate-dry-run-under-a-profile-prints-the-unsplit-request.md`.

## Evidence

- Starts from: Local experiment 273, report 03 findings 2-1, 2-2 and 2-3, as the three issues record them. The code at `origin/main` `e24f7324`: `core/question_set.rs:293-309`, `cli/annotate.rs:283-311`, `public/bulk.rs:283-311`, `cli/annotate/plan.rs:58-62`, `core/records.rs` lines 46, 86, 381, 415-426 and 468. `annotate.md` lines 23, 34, 81, 85, 94 and 96.
- Keeps: `on` over JSON records, rows, documents and library JSON records. A root-only library set's path, byte for byte. Every run whose `on` reads JSON. A library root group's text. The single-request plan bytes when nothing splits, beside the two new members.
- Changes: `on` never re-parses text, and each record is read once. Text refusals name the question. The library's sentence for text that is not JSON. The page's examples. An executable annotate page. The dry run prints the first real chunk and the request counts.
- Proof: Two outside-in command tests, the library edge rows and one executable page with nine plants, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: A binding test of the named text sentence.
