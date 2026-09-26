---
flow: build
priority: 150
opens: crates/thinkthen/src/public/question.rs crates/thinkthen/src/public/bulk.rs crates/thinkthen/src/public/results.rs crates/thinkthen/src/public/set.rs crates/thinkthen/src/core/question_set.rs crates/thinkthen/src/engine/facade/annotate.rs crates/thinkthen/src/engine/facade_tests.rs crates/thinkthen/src/engine/facade_tests/annotate_order.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/conformance_tests.rs crates/thinkthen/src/cli/conformance_tests/support/conformance.rs crates/thinkthen/tests/polars/cases.rs conformance/cases.json conformance/README.md conformance/consumer/consumer/tests/public libraries/python libraries/typescript libraries/ruby libraries/r libraries/c databases/duckdb/tools/conformance.py databases/duckdb/NOTES.md databases/sqlite/tests/conformance.py databases/postgresql/tests/runner.py databases/postgresql/NOTES.md sdlc/planning/libraries/javascript.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0150: Find's none option and annotate over record parts

Status: ready. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Owner: Claude. It builds after ticket 0151 lands, and it lands before ticket 0146's build starts. See "Overlap with tickets in flight".

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A Python user writes `tt.find(question, passages, none=True)` and gets what `thinkthen find QUESTION --none` gives. The model may answer that no passage fits, and the call then returns `None`. The same user passes a question set whose members name `on` pointers, with records as JSON text. Each question sees only its part of the record, as `thinkthen annotate` does. The same holds in Rust, C, TypeScript, Ruby and R. Annotate over parts also works on the Rust Polars door, the Python frame doors, DuckDB, SQLite and PostgreSQL.

Ian's goal 3 for 0.1: every language library matches the command wherever it can. This ticket is row L4 of `sdlc/planning/backlog-0-1-2026-09-26.md` and proposal E3 of `sdlc/planning/library-equivalence-2026-09-26.md`, without E3's SQL find aggregate. It closes item 5 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`.

## What happens today

- `Engine::find_with` builds its request with `none` fixed at `false` (`crates/thinkthen/src/public/bulk.rs:148-175`). `Question::find` has no switch (`public/question.rs:382-389`). The command's `--none` passes the flag to the same `core::Find::new` (`cli/find.rs:88`).
- `Found::new` pairs each probability with a unit and refuses any other count (`public/results.rs:466-484`). `Candidate` already has a synthetic `none` form with no input (`results.rs:427-454`), and nothing builds it.
- `QuestionSet::from_json` refuses a member whose `on` is not the root (`public/set.rs:28-48`). The library's `annotated` sends the whole text to every group (`public/bulk.rs:278-304`).
- The engine already groups a set's questions by `on` (`core/question_set.rs:263`). It asks one request per group (`engine/facade/annotate.rs:91-107`). It prepares a group and sends it before it prepares the next.
- The command reads each group's part in `cli/annotate.rs::plan_for` (`:281-312`). A member with a non-root `on` parses the record's evidence as one JSON document and takes its pointers. The command prepares every group of a record before it sends any (`cli/annotate_schedule.rs:90-113`).
- The two shared find cases, `18-find-second` and `19-find-none`, both set `none: true`. So no library runs any find case today. `18-annotate-two-groups` is skipped on every surface. The skip lists are in the Rust consumer (`conformance/consumer/consumer/tests/public/cases.rs:31-36`), C (`libraries/c/tests/door/cases.rs:28-34`), Python (`libraries/python/tests/conformance.py:29-38`), TypeScript (`libraries/typescript/tests/cases.mjs:13-15`), Ruby (`libraries/ruby/tests/conformance.rb:13-15`), R (`libraries/r/tests/conformance.R:20-22`), the Rust Polars door (`crates/thinkthen/tests/polars/cases.rs:57-60`), DuckDB (`databases/duckdb/tools/conformance.py:28-36`), SQLite (`databases/sqlite/tests/conformance.py:26-30`) and PostgreSQL (`databases/postgresql/tests/runner.py:30`).
- The case `18-annotate-two-groups` names no record. Each exchange holds one part as its evidence.
- Every binding builds its find question from the question's text and calls `Engine::find_with`. Every binding sends annotate through `Engine::annotate_with` with a set from `QuestionSet::from_json` or `load`.

## Design

### Find's none option

`Question` gains one method:

```rust
/// This find question with a `none` candidate beside the units, as
/// `find --none` asks. The model may then answer that no unit fits.
pub fn offering_none(self) -> Result<Self, Error>;
```

It returns `Error::Usage` for any question that is not a find question. On a find question it sets a private kind, `Kind::FindNone`. Three places read the private kind:

- `Kind::public` (`public/question.rs:32-43`) maps `FindNone` to `QuestionKind::Find`, as it maps `Banded` to `Decide`.
- `QuestionSetBuilder::question` (`public/set.rs:89`) refuses `FindNone` beside `Rank` and `Find`, with its existing sentence.
- `find_with` (`public/bulk.rs:158`) accepts both find kinds and passes `none` to `core::Find::new` when the kind is `FindNone`. A none question takes 2 to 254 units, and the refusal sentence is `a find question offering none takes 2 to 254 units`.

`Found::new` expects one more probability when the question offered none. It appends the synthetic candidate last with no input. `selected()` already reads `None` when the selector picks `none` or ties with it. `candidates()` then holds the units in input order and `none` last, as item 2 of ADR 0017's 2026-09-21 amendment on the shared case contract fixes.

Each binding gains one keyword on its find call. Its glue calls `offering_none` when the keyword is true.

| Surface | Spelling | Nothing fits |
| --- | --- | --- |
| Rust | `Question::find(text)?.offering_none()?` | `selected()` is `None` |
| C | `"none": true` beside `"find"` in the request object, the case file's own shape | `null` |
| Python | `tt.find(question, units, none=True)` | `None` |
| TypeScript | `tt.find(question, units, { none: true })` | `null` |
| Ruby | `ThinkThen.find(question, units, none: true)` | `nil` |
| R | `tt_find(question, units, none = TRUE)` | `place` is `NA` |

Each binding already returns its empty value when nothing is selected. That path is unchanged.

### Annotate over record parts

`core::QuestionSet` gains one pure method that holds the command's rule once:

```rust
/// The evidence one group sees: the record itself when the group reads
/// the root, or else the parts its pointers select from the record as JSON.
pub(crate) fn group_evidence(&self, group: &[usize], record: &Evidence) -> Result<Evidence, PartError>;
```

`PartError` is a small `core` enum with three arms, one for each step of today's `plan_for`:

- `Reading(ReadingError)`, from `Reading::new(Framing::Document, on)`. The set's parser already checked every pointer and every key clash (`core/question_set.rs:203`), so this arm cannot happen with a parsed set.
- `Render(RenderError)`, from `Evidence::as_text`. A library record is always text, so this arm cannot happen in the library. The command can reach it only through `--field` with a structured selection, as today.
- `Record(RecordError)`, from reading the record as JSON and taking its pointers. This is the one arm a caller's record can reach.

The command maps each arm to the `Failure` it builds today, so its exit codes and sentences do not change. The library maps them as follows:

| Arm | Public error | Sentence |
| --- | --- | --- |
| `Reading` | `Error::Defect` | `a checked question set could not read its parts` |
| `Render` | `Error::Defect` | The same sentence |
| `Record` | `Error::Usage` | The `RecordError`'s own `Display` text, unchanged, as `Error::usage(error.to_string())` |

The `Record` sentences are the command's. They name a pointer or a position and never the record's text. `Missed` reads ``the record holds nothing at `/body` ``. Text that is not JSON reads `the input is not valid JSON: the JSON at line 1 column N is not one`, from `InputJson`. A JSON record with a duplicate key or a number past `f64` reads `JsonError`'s sentence.

`cli/annotate.rs::plan_for` calls `group_evidence` in place of its own nested reading. `QuestionSet::from_json` drops its refusal.

The public `annotated` (`public/bulk.rs:278-304`) computes every group's evidence through `group_evidence` before it calls `engine.annotate`. It keeps them in set-group order, and the plan closure takes the group's evidence from that list. So a bad part fails with its own `Usage` sentence and nothing sent. That is the library's guard, and it holds whatever the engine does.

`engine/facade/annotate.rs::annotate` also prepares every group before it answers any, as the command does (`cli/annotate_schedule.rs:90-113`). This is the engine's guard. It covers a failure the public layer cannot see first: a later group that `split` refuses under a backend profile, as `ProfileLimit`. The public `annotated` is the engine function's only product caller.

A record whose set reads the root stays one whole text, JSON or not. A record whose set names a part must be JSON text. Like a blank record today, a bad record's error is that record's item in the batch.

No binding changes code for parts. Each already passes its set and its texts through `annotate_with`. Records are JSON text on every surface, one record per string, as a JSONL line is to the command. A frame door reads JSON text in its `on` column. A SQL call reads JSON text in its argument.

### The shared case

`18-annotate-two-groups` gains `"record": {"summary": "Short note.", "body": "Please refund me."}`. A runner serializes it and passes it as the one record. Every expected answer then belongs to record 0. The case's exchanges, requests and answers do not change.

The command's conformance runner (`cli/conformance_tests.rs::annotate`) reads a case's `record` through `group_evidence` when the case has one. Its request bytes must still equal each exchange's recorded request. So the new field cannot drift from the exchanges. `Case` gains the optional field, and `validate_shape` allows it only on an `annotate` case.

Each runner gains a find branch. It reads `question.find`, `question.none` and `question.units`, and compares what its surface returns with `operation.selected` and that unit's probability. The Rust runner also compares every candidate probability, `none` last, with `operation.probabilities`.

### Surfaces that cannot take part

- **Find on the Rust Polars and Python frame doors.** The equivalence page, section 3: "Rank and find have no column form. A rank sorts records, and a find picks one unit of a set. The Polars and pandas doors return one value per row, so they leave both functions to the list call." The Python list call carries `none`.
- **Find on DuckDB, SQLite and PostgreSQL.** No SQL surface has a find function (equivalence page, F7). A none switch has nothing to switch. The page's proposal E3 adds a find aggregate, and that is a new verb on three surfaces. This ticket defers it as a gap. The three SQL runners keep `18-find-second` and `19-find-none` in their not-run lists, with the reason reworded to "no SQL find function yet".

After this ticket each not-run list holds these entries. Each surface's other entries stay for their own tickets.

- The Rust consumer holds `25-defect-fault` alone.
- C, TypeScript and Ruby hold `25-defect-fault` and `30-local-question-file`. Their case 30 is the question-file gap Q1 to Q3 of the equivalence page.
- Python holds `25-defect-fault`.
- R holds `20-usage-fault`, `22-local-fault`, `23-cancelled-fault`, `25-defect-fault` and `30-local-question-file`.
- The Rust Polars door keeps its entries for filter, rank, find, recognize, relate, the faults, cases 29 and 30, and case 40. It loses `18-annotate-two-groups`.
- DuckDB, SQLite and PostgreSQL keep `18-find-second` and `19-find-none` with the reworded reason, and every other entry they hold today. Each loses `18-annotate-two-groups`.

## Decisions

1. **None rides on the question.** `offering_none` changes a find question. The free functions and `find_with` keep their signatures. A binding that holds a built question can still switch it. The command's flag sits on the call, so a host keyword is the binding's spelling, and every binding maps that keyword to the one Rust method.
2. **A private kind, not a field.** `Kind::FindNone` adds one variant to a private enum. `Question` keeps its four fields and its public `QuestionKind`. `Question` derives `PartialEq` over its fields, and the kind is one of them. So a find question that offers none and the same question without it compare unequal. That is correct, because they send different requests.
3. **The record is JSON text.** No host map, dictionary or frame row becomes a record in this ticket. The command reads a JSON line, and JSON text is the one form every surface already passes. Host sugar waits for a user.
4. **One rule for parts.** `group_evidence` in `core` replaces the command's inline reading. The command and the library call the same function.
5. **Two guards before sending.** The public layer reads every part before it asks, so a bad part spends nothing and keeps its own sentence. The engine's annotate also prepares every group before it sends, as the command does, so a later group over a profile limit spends nothing.
6. **The shared case names its record.** Nine runners read one field instead of rebuilding a record from pointers in nine languages. The command runner proves the field against the recorded requests.
7. **The SQL find aggregate waits.** It is a new verb, not the none switch. Its own ticket can size it once a user asks.
8. **Single questions and recognize keep refusing `on`.** `Question::from_json` and `Recognize::from_json` still read their evidence whole (`public/question.rs:390-404`, `public/recognize.rs:138-150`). The issue asks for annotate parts alone.

## Edge cases

Each row runs through the public Rust API against the conformance loopback, counting requests at the listener.

| Input | Result | Requests |
| --- | --- | --- |
| `offering_none` on a rank question | `Usage`: `only a find question offers none` | 0 |
| `offering_none` on a decide question | The same `Usage` sentence | 0 |
| `offering_none` twice on one find question | The same question as once | 0 |
| A none question over 254 units | Answers. The last candidate `is_none()` | 1 |
| A none question over 255 units | `Usage`: `a find question offering none takes 2 to 254 units` | 0 |
| A plain find question over 255 units | Answers, as today | 1 |
| A none question over 1 unit | `Usage`, the same sentence | 0 |
| A two-group set over a record missing `/body` | `Usage`: ``the record holds nothing at `/body` `` | 0 |
| A two-group set over text that is not JSON | `Usage` with the record parser's sentence | 0 |
| A two-group set whose `/summary` is a number | The number's JSON text as evidence, as the command sends | 2 |
| A root-only set over text that is not JSON | Answers from the whole text, as today | 1 |
| A set with one root group and one `/body` group | Two requests: the whole record, then its body | 2 |
| Each refusal above, printed and `Debug` | No record text appears | 0 |

## Proof

Every test runs against the loopback backend in `conformance/backend`, sends nothing live, and uses a fake key.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| The shared cases, Rust consumer | `18-find-second`, `19-find-none` and `18-annotate-two-groups` pass through the public API. `SKIPPED` holds only `25-defect-fault` | (a) `find_with` passes `false` again: the request lacks the `none` criterion, and the case arm refuses it. (b) `Found::new` drops the `none` candidate: 19's probabilities fail. (c) The public `annotated` sends the whole record to every group: the requests' `state` differs, and both groups fail |
| The shared cases, the command runner | `18-annotate-two-groups` reads its `record` through `group_evidence`, and each request equals its exchange | (d) Change `/body`'s text in the case's `record`: the request digest differs |
| The shared cases, C, Python, TypeScript, Ruby and R | The three cases leave each not-run list and pass | (e) Each binding ignores its `none` keyword: 18 and 19 fail on that surface. Plant (c) turns 18-annotate red on every one |
| The shared cases, the Rust Polars door, Python frames, DuckDB, SQLite and PostgreSQL | `18-annotate-two-groups` leaves each not-run list and passes | Plant (c) turns each red. This proves each runner really runs the case |
| `find_none_and_parts_follow_the_command`, a new module `parts.rs` in the Rust consumer | Every row of the edge table | (g) Put the record text in the bad-record message: the secrecy row fails. (h) Keep 255 as the none limit: the 255-unit row answers. (j) The public `annotated` reads each part lazily inside the plan closure and maps a `RecordError` to `Error::Backend`: the missing-`/body` row reads `Backend` with a different sentence |
| `a_later_group_over_the_profile_sends_nothing`, a new module `engine/facade_tests/annotate_order.rs` | The facade's `annotate` over the two-group record, on an engine whose `Settings` carry the profile of `conformance/backend-profiles.json` case `annotate-later-group-over` (`max_evidence_bytes: 16`). The second group's 17-byte part fails `split` with `ProfileLimit`. The call returns that error, and the loopback counts 0 | (f) Answer each group as soon as it is prepared, as the engine does today: the first group sends, and the count is 1 |
| `sdlc/scripts/inventory` | The 0084 normative block names `offering_none`, and the export list matches | (i) Leave 0084 unamended: the inventory rung fails |

The four questions, answered once for the shared cases and once for the edge module:

- **What they protect.** A library's find with none sends the command's request and reads the `none` candidate. A library's annotate sends each group its own part, as the command does. The two cases run the same on every surface that has the function.
- **What regression fails them.** A binding that drops its `none` keyword. A `Found` that loses the none candidate or misreads the selection. An annotate that sends the whole record, or sends a group before a later part fails. A record field in the case that drifts from its exchanges.
- **Why no existing test catches it.** The three cases are skipped on every library today, and the library refuses both features. The command's tests prove its own flag and its own reader, and never reach a library. No test holds the library to the command's prepare-all order.
- **Does it need a test-only hook.** No. Each test calls the public spelling, counts at the real loopback listener, and matches the recorded request bytes. The case's `record` field is input, not an expectation, and the command runner checks it against the recorded requests.
- **Why the order test sits at the facade.** The public builder sets no backend profile until ticket 0148 lands (`public/settings.rs:235`). Ticket 0150 lands first. So no public call can reach a `ProfileLimit` in a later group. The facade's `Settings.profile` is the real field the builder fills after 0148, not a hook. The facade test module already builds engines this way (`engine/facade_tests.rs:31-45`).

`parts.rs` holds no copy of a runner check. The shared cases pin the happy path. The module pins the refusals, limits, ordering and secrecy that no shared case reaches.

## Pages, comments, and docs

- Ticket 0084: a dated amendment. It adds `offering_none` to the `Question` block. It replaces "JSON Pointer extraction stays command-only in 0.1" (line 25) with a line saying annotate reads `on` parts from a record's JSON text, and single questions and recognize still read their evidence whole.
- `conformance/README.md`: one sentence for the `record` field.
- Doc comments: `QuestionSet::from_json`, `Engine::find`, `Engine::annotate`.
- Each binding's own doc for find: `thinkthen/__init__.pyi` and the docstring, `index.d.ts`, Ruby's method comment, R's roxygen comment, and `include/thinkthen.h`. Each says records with parts are JSON text.
- Notes that say the library cannot: `libraries/typescript/NOTES.md:16`, `libraries/r/NOTES.md`, `databases/duckdb/NOTES.md:26`, `databases/postgresql/NOTES.md:23`, and item 10 of `sdlc/planning/libraries/javascript.md`.
- The dated equivalence page and backlog page are left as written.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src` product code, doc lines included: at most 100 added and at most 70 net. The estimate: `offering_none`, the kind and the `Kind::public` and `set.rs:89` arms (15), `find_with` (6), `Found::new` (8), `group_evidence` and `PartError` (30), the public `annotated` reading every part first and mapping `PartError` (18), `plan_for` mapping `PartError` (5), and the engine's prepare-all order (8). About 21 lines leave: the set refusal (12), the inline reading in `plan_for` (6), and the interleaved loop (3).
- `engine/facade_tests/annotate_order.rs`: at most 60, and one `mod` line in `engine/facade_tests.rs`.
- The command runner and `Case`: at most 20 added.
- `conformance/cases.json`: one added line.
- The Rust consumer runner: at most 40 added. `parts.rs`: at most 110.
- Binding production code, docs and type files: at most 15 added on each of C, Python, TypeScript, Ruby and R.
- Each binding runner: at most 35 added on C, Python, TypeScript, Ruby and R, for the find branch and the record. At most 10 on each of the Rust Polars door, DuckDB, SQLite and PostgreSQL.
- Pages: at most 12 lines for the 0084 amendment, 2 for `conformance/README.md`, and one changed line for each note.
- `sdlc/ratchet.json` and each binding's ceilings move to the measured totals in the commit that adds the code. The commit says what grew. Before adding, the builder deletes the command's inline reading and the set refusal.
- No dependency. The public surface widens, so the `surfaces` rung runs.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green.
3. Stop if ticket 0151 has not landed on main when the build starts, or if the 0146 branch holds build commits beyond its ticket. Merge `origin/main` first.
4. Stop if the prepare-all order changes any existing annotate test or recording. It must change only when a request is sent, never what it holds.
5. Stop if `group_evidence` cannot serve both the command and the library unchanged. Report the difference. Two readers would drift.
6. Stop if a binding needs more than its keyword and pass-through for either feature. That means the Rust design is wrong.
7. Never run `sdlc/scripts/live`. Unset `THINKTHEN_API_KEY` for every rung.

## Scope and exclusions

Excluded: a SQL find function (decision 7). Host maps, dictionaries and frame rows as records (decision 3). `on` on single questions and recognize (decision 8). A `QuestionSetBuilder` step for `on`. Details on every surface, which ticket 0151 (L5) carries. The other not-run entries on each surface. `site/`, which the marketing lead owns.

## Overlap with tickets in flight

**Build order, by the coordinator's ruling of 2026-09-26:** ticket 0151 lands first. This ticket then merges `origin/main` and builds. It does not wait for tickets 0146 or 0148, which are parked behind a paid run. This ticket lands before 0146's build starts. Then 0146 and 0148 merge it.

- **Ticket 0151** (L5). Its files that this ticket also edits:
  - `crates/thinkthen/src/public/results.rs`
  - `conformance/cases.json` and `conformance/README.md`
  - ticket 0084
  - all nine shared-case runners: `conformance/consumer/consumer/tests/public/cases.rs`, `libraries/c/tests/door/cases.rs`, `libraries/python/tests/conformance.py`, `libraries/typescript/tests/cases.mjs`, `libraries/ruby/tests/conformance.rb`, `libraries/r/tests/conformance.R`, `databases/duckdb/tools/conformance.py`, `databases/sqlite/tests/conformance.py` and `databases/postgresql/tests/runner.py`
  - `databases/duckdb/NOTES.md`. 0151's `opens` line names the DuckDB folder's README and runner, not this file. The coordinator counts it as shared, and this ticket treats it so.
- **Ticket 0146.** Its `opens` line holds these files that this ticket edits: `public/bulk.rs`, `public/question.rs`, `public/results.rs`, `engine/facade_tests.rs` (one `mod` line here), and `crates/thinkthen/tests/polars/cases.rs` under its `crates/thinkthen/tests` entry. It also changes `Completed` in `bulk.rs`, which `annotated` builds. This ticket's `cli/annotate.rs`, `cli/conformance_tests.rs` and `cli/conformance_tests/support/conformance.rs` are not on 0146's list. They sit in the `cli/` tree 0146 changes, so its merge reads them.
- **Ticket 0148.** Its `opens` line holds `conformance/consumer`, `conformance/README.md`, every library folder, every database folder and ticket 0084. This ticket edits the consumer runner, each library's find glue and runner, each database runner, `conformance/README.md` and 0084.
- No ticket in flight lists `core/question_set.rs`, `engine/facade/annotate.rs` or the new `engine/facade_tests/annotate_order.rs`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change widens the public surface on six languages and changes the frozen 0084 inventory, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 1; cost of error 1; total 7. Final level: 2. The risk is an annotate that sends a group before a later part or a later group fails, which spends a request the command would not. Plants (f) and (j) guard it. The reach is wide, and each binding's change is one keyword.

## Deferred gaps

- A SQL find aggregate on DuckDB, SQLite and PostgreSQL, proposal E3's second bullet. Its own ticket, when a user asks.
- Host records: a Python `dict`, a Ruby `Hash`, an R list or data frame row, a TypeScript object, or a frame row read as a record. Today each passes JSON text.
- `on` on a single question file and on a recognize file.
- A `QuestionSetBuilder` step that names `on`. A Rust user reaches parts through `from_json` or `load`.
- The order test through the public API. Once 0148 gives the builder a profile, `a_later_group_over_the_profile_sends_nothing` can move from the facade to `parts.rs`.
- The G8 bytes test (`crates/thinkthen/tests/backend/public_json.rs:101`) still skips sets with `on`. The shared helper keeps the command and library readers one function, so this ticket leaves that guard.

## What Ian can overturn

- Decision 1: none on the question as `offering_none`, and a call keyword in each host.
- Decision 3: JSON text as the one record form for parts.
- Decision 7: the SQL find aggregate deferred.
- Decision 8: single questions and recognize keep refusing `on`.
- The 0084 amendment, which reverses 0084's line that pointer extraction stays in the command for 0.1.

## Closes

- `sdlc/issues/2026-09-25-public-library-api-gaps.md`, item 5. The issue stays open for items 1, 2, 3, 6, 7 and 9. The lander marks item 5 settled, as item 8 is marked.

## Evidence

- Starts from: Item 5 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. The equivalence page `sdlc/planning/library-equivalence-2026-09-26.md`, notes F6, F7 and F8, proposal E3, and section 3 on rank and find having no column form. Row L4 of `sdlc/planning/backlog-0-1-2026-09-26.md`. The command's measured behavior: `specification/find.md` records `none` answering 4 of 4 blank documents and refusing 0 of 16 answerable ones, and ticket 0040's reach of 254 units with `none`. `specification/annotate.md` records the grouped requests by `on`. The command tests `crates/thinkthen/tests/backend/annotate.rs:300-330` (two groups) and the shared cases 18 and 19. Tickets 0146, 0148 and 0151, read from their branches. The shared profile case `annotate-later-group-over` in `conformance/backend-profiles.json`. The facade's settings seam in `engine/facade_tests.rs:31-45` and the public builder's fixed `profile: None` at `public/settings.rs:235`. The code at `origin/main` `a2fe448d`. No experiment ran. The command already proves both engine paths, and this ticket exposes them.
- Keeps: Every command flag, request, output byte and exit code. Every find call without `none`, and every annotate set whose members read the root, byte for byte. Every existing library signature. Each binding's empty value for nothing selected. `Question::from_json` and `Recognize::from_json` refusing `on`. Every recording.
- Changes: `Question::offering_none` and a private find kind. `find_with` passes `none` and `Found` carries the none candidate. `QuestionSet::from_json` accepts `on`. One `group_evidence` rule in `core` serves the command and the library. The public `annotated` reads every part before it asks, and the engine's annotate prepares every group before it sends. C, Python, TypeScript, Ruby and R gain a `none` keyword on find. `18-annotate-two-groups` names its record. Every runner runs the three cases where the surface has the function. Ticket 0084 gains a dated amendment.
- Proof: The three shared cases on Rust, C, Python, TypeScript, Ruby and R, and the annotate case on the Rust Polars door, Python frames, DuckDB, SQLite and PostgreSQL, each with plants. The command runner's check of the `record` field. The `parts.rs` edge table with its limit, bad-part and secrecy plants. The facade order test with its profile plant. The `inventory` rung for 0084.
- Defers: The SQL find aggregate. Host maps and frame rows as records. `on` on single questions and recognize. A builder step for `on`. The order test through the public API, after 0148. The G8 bytes test for parts.
