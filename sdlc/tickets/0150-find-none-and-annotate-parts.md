---
flow: build
priority: 150
opens: crates/thinkthen/src/public/question.rs crates/thinkthen/src/public/bulk.rs crates/thinkthen/src/public/results.rs crates/thinkthen/src/public/set.rs crates/thinkthen/src/core/question_set.rs crates/thinkthen/src/engine/facade/annotate.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/conformance_tests.rs crates/thinkthen/src/cli/conformance_tests/support/conformance.rs crates/thinkthen/tests/polars/cases.rs conformance/cases.json conformance/README.md conformance/consumer/consumer/tests/public libraries/python libraries/typescript libraries/ruby libraries/r libraries/c databases/duckdb/tools/conformance.py databases/duckdb/NOTES.md databases/sqlite/tests/conformance.py databases/postgresql/tests/runner.py databases/postgresql/NOTES.md sdlc/planning/libraries/javascript.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0150: Find's none option and annotate over record parts

Status: ready for review. Owner: Claude. It builds after tickets 0146 and 0148 land.

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

It returns `Error::Usage` for any question that is not a find question. On a find question it sets a private kind, `Kind::FindNone`, which reads `QuestionKind::Find` in public. `find_with` accepts both find kinds and passes `none` to `core::Find::new`. A none question takes 2 to 254 units, and the refusal sentence is `a find question offering none takes 2 to 254 units`.

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
pub(crate) fn group_evidence(&self, group: &[usize], record: &Evidence) -> Result<Evidence, RecordError>;
```

`cli/annotate.rs::plan_for` calls it in place of its own nested reading. The public `annotated` calls it with the record's text. `QuestionSet::from_json` drops its refusal.

`engine/facade/annotate.rs::annotate` prepares every group before it answers any. A record whose second part is missing then fails with nothing sent, as the command does. The public `annotated` is its only product caller.

A record whose set reads the root stays one whole text, JSON or not. A record whose set names a part must be JSON text. A bad record is `Error::Usage` with the command's sentence, such as ``the record holds nothing at `/body` ``. Those sentences name a pointer and never the record. Like a blank record today, the error is that record's item in the batch.

No binding changes code for parts. Each already passes its set and its texts through `annotate_with`. Records are JSON text on every surface, one record per string, as a JSONL line is to the command. A frame door reads JSON text in its `on` column. A SQL call reads JSON text in its argument.

### The shared case

`18-annotate-two-groups` gains `"record": {"summary": "Short note.", "body": "Please refund me."}`. A runner serializes it and passes it as the one record. Every expected answer then belongs to record 0. The case's exchanges, requests and answers do not change.

The command's conformance runner (`cli/conformance_tests.rs::annotate`) reads a case's `record` through `group_evidence` when the case has one. Its request bytes must still equal each exchange's recorded request. So the new field cannot drift from the exchanges. `Case` gains the optional field, and `validate_shape` allows it only on an `annotate` case.

Each runner gains a find branch. It reads `question.find`, `question.none` and `question.units`, and compares what its surface returns with `operation.selected` and that unit's probability. The Rust runner also compares every candidate probability, `none` last, with `operation.probabilities`.

### Surfaces that cannot take part

- **Find on the Rust Polars and Python frame doors.** The equivalence page, section 3: "Rank and find have no column form. A rank sorts records, and a find picks one unit of a set. The Polars and pandas doors return one value per row, so they leave both functions to the list call." The Python list call carries `none`.
- **Find on DuckDB, SQLite and PostgreSQL.** No SQL surface has a find function (equivalence page, F7). A none switch has nothing to switch. The page's proposal E3 adds a find aggregate, and that is a new verb on three surfaces. This ticket defers it as a gap. The three SQL runners keep `18-find-second` and `19-find-none` in their not-run lists, with the reason reworded to "no SQL find function yet".

After this ticket the not-run lists hold only `25-defect-fault` on Rust, and add only the two find cases on the three SQL surfaces and the frame doors. Each surface's other not-run entries, such as C's case 30 and R's cases 20 to 23, stay for their own tickets.

## Decisions

1. **None rides on the question.** `offering_none` changes a find question. The free functions and `find_with` keep their signatures. A binding that holds a built question can still switch it. The command's flag sits on the call, so a host keyword is the binding's spelling, and every binding maps that keyword to the one Rust method.
2. **A private kind, not a field.** `Kind::FindNone` adds one variant to a private enum. `Question`'s fields, `PartialEq` and its public `QuestionKind` stay as they are.
3. **The record is JSON text.** No host map, dictionary or frame row becomes a record in this ticket. The command reads a JSON line, and JSON text is the one form every surface already passes. Host sugar waits for a user.
4. **One rule for parts.** `group_evidence` in `core` replaces the command's inline reading. The command and the library call the same function.
5. **Prepare every group before sending.** The engine's annotate follows the command's order. A bad part spends nothing.
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
| The model leads with `none` | `selected()` is `None`. `candidates()` has one more entry than the units | 1 |
| `none` ties the top unit | `selected()` is `None`, by `specification/find.md` | 1 |
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
| `find_none_and_parts_follow_the_command`, a new module `parts.rs` in the Rust consumer | Every row of the edge table | (f) Answer groups as the engine does today, before preparing the next: the missing-`/body` row counts 1. (g) Put the record text in the bad-record message: the secrecy row fails. (h) Keep 255 as the none limit: the 255-unit row answers |
| `sdlc/scripts/inventory` | The 0084 normative block names `offering_none`, and the export list matches | (i) Leave 0084 unamended: the inventory rung fails |

The four questions, answered once for the shared cases and once for the edge module:

- **What they protect.** A library's find with none sends the command's request and reads the `none` candidate. A library's annotate sends each group its own part, as the command does. The two cases run the same on every surface that has the function.
- **What regression fails them.** A binding that drops its `none` keyword. A `Found` that loses the none candidate or misreads the selection. An annotate that sends the whole record, or sends a group before a later part fails. A record field in the case that drifts from its exchanges.
- **Why no existing test catches it.** The three cases are skipped on every library today, and the library refuses both features. The command's tests prove its own flag and its own reader, and never reach a library. No test holds the library to the command's prepare-all order.
- **Does it need a test-only hook.** No. Each test calls the public spelling, counts at the real loopback listener, and matches the recorded request bytes. The case's `record` field is input, not an expectation, and the command runner checks it against the recorded requests.

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

- `crates/thinkthen/src`, doc lines included: at most 70 added and at most 30 net. That covers `offering_none` and its kind (15), `find_with` (6), `Found::new` (8), `group_evidence` (20), the prepare-all order (8), and the set refusal and inline reading removed.
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
3. Stop if 0146 or 0148 has not landed on main when the build starts. Merge `origin/main` first.
4. Stop if the prepare-all order changes any existing annotate test or recording. It must change only when a request is sent, never what it holds.
5. Stop if `group_evidence` cannot serve both the command and the library unchanged. Report the difference. Two readers would drift.
6. Stop if a binding needs more than its keyword and pass-through for either feature. That means the Rust design is wrong.
7. Never run `sdlc/scripts/live`. Unset `THINKTHEN_API_KEY` for every rung.

## Scope and exclusions

Excluded: a SQL find function (decision 7). Host maps, dictionaries and frame rows as records (decision 3). `on` on single questions and recognize (decision 8). A `QuestionSetBuilder` step for `on`. Details on every surface, which ticket 0151 (L5) carries. The other not-run entries on each surface. `site/`, which the marketing lead owns.

## Overlap with tickets in flight

- **Ticket 0146** opens `public/bulk.rs`, `public/question.rs`, `public/results.rs`, `cli/conformance_tests/runner.rs` and `crates/thinkthen/tests`. This ticket edits the first three and `crates/thinkthen/tests/polars/cases.rs`. It also edits `cli/annotate.rs` and `cli/conformance_tests.rs`, which sit under the `cli/` tree 0146 changes. 0146 changes `Completed` in `bulk.rs`, which `annotated` builds.
- **Ticket 0148** opens `conformance/consumer`, `conformance/README.md`, every library folder, every database folder and ticket 0084. This ticket edits the case runner in `conformance/consumer`, each library's find glue and runner, each database runner, `conformance/README.md` and 0084.
- No file overlaps `crates/thinkthen/src/core/question_set.rs` or `engine/facade/annotate.rs`. Neither 0146 nor 0148 lists them.
- **Build order:** 0146, then 0148, then this ticket. The coordinator already set 0148 after 0146. This ticket merges `origin/main` after both land and builds in one lane.
- **Ticket 0151** (L5) has a worktree and no ticket on origin yet. It will likely touch the same runners to check more detail fields. The coordinator sequences the two.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change widens the public surface on six languages and changes the frozen 0084 inventory, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 1; cost of error 1; total 7. Final level: 2. The risk is an annotate that sends a group before a later part fails, which spends a request the command would not. Plant (f) guards it. The reach is wide, and each binding's change is one keyword.

## Deferred gaps

- A SQL find aggregate on DuckDB, SQLite and PostgreSQL, proposal E3's second bullet. Its own ticket, when a user asks.
- Host records: a Python `dict`, a Ruby `Hash`, an R list or data frame row, a TypeScript object, or a frame row read as a record. Today each passes JSON text.
- `on` on a single question file and on a recognize file.
- A `QuestionSetBuilder` step that names `on`. A Rust user reaches parts through `from_json` or `load`.
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

- Starts from: Item 5 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. The equivalence page `sdlc/planning/library-equivalence-2026-09-26.md`, notes F6, F7 and F8, proposal E3, and section 3 on rank and find having no column form. Row L4 of `sdlc/planning/backlog-0-1-2026-09-26.md`. The command's measured behavior: `specification/find.md` records `none` answering 4 of 4 blank documents and refusing 0 of 16 answerable ones, and ticket 0040's reach of 254 units with `none`. `specification/annotate.md` records the grouped requests by `on`. The command tests `crates/thinkthen/tests/backend/annotate.rs:300-330` (two groups) and the shared cases 18 and 19. Tickets 0146 and 0148, read from their branches. The code at `origin/main` `a2fe448d`. No experiment ran. The command already proves both engine paths, and this ticket exposes them.
- Keeps: Every command flag, request, output byte and exit code. Every find call without `none`, and every annotate set whose members read the root, byte for byte. Every existing library signature. Each binding's empty value for nothing selected. `Question::from_json` and `Recognize::from_json` refusing `on`. Every recording.
- Changes: `Question::offering_none` and a private find kind. `find_with` passes `none` and `Found` carries the none candidate. `QuestionSet::from_json` accepts `on`. One `group_evidence` rule in `core` serves the command and the library. The engine's annotate prepares every group before it sends. C, Python, TypeScript, Ruby and R gain a `none` keyword on find. `18-annotate-two-groups` names its record. Every runner runs the three cases where the surface has the function. Ticket 0084 gains a dated amendment.
- Proof: The three shared cases on Rust, C, Python, TypeScript, Ruby and R, and the annotate case on the Rust Polars door, Python frames, DuckDB, SQLite and PostgreSQL, each with plants. The command runner's check of the `record` field. The `parts.rs` edge table with its limit, order and secrecy plants. The `inventory` rung for 0084.
- Defers: The SQL find aggregate. Host maps and frame rows as records. `on` on single questions and recognize. A builder step for `on`. The G8 bytes test for parts.
