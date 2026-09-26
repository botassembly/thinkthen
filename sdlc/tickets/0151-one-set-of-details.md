---
flow: build
priority: 151
opens: crates/thinkthen/src/public/results.rs conformance/cases.json conformance/README.md conformance/consumer/consumer/tests/public/cases.rs libraries/typescript/index.d.ts libraries/typescript/tests/types.test.ts libraries/typescript/tests/cases.mjs libraries/typescript/README.md libraries/python/tests/conformance.py libraries/python/README.md libraries/ruby/tests/conformance.rb libraries/ruby/README.md libraries/r/tests/conformance.R libraries/r/README.md libraries/c/tests/door/cases.rs libraries/c/README.md libraries/rust/README.md libraries/polars/README.md databases/duckdb/src/scalars.rs databases/duckdb/src/scalars/calls.rs databases/duckdb/tools/conformance.py databases/duckdb/tools/verbs_suite.py databases/duckdb/tools/settings_suite.py databases/duckdb/README.md databases/duckdb/ratchet.json databases/sqlite/tests/conformance.py databases/sqlite/README.md databases/postgresql/tests/runner.py databases/postgresql/README.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0151: One set of details on every surface

Status: ready for review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A DuckDB user who calls `thinkthen_details` gets the same `thinkthen.result/1` line that SQLite, PostgreSQL, the libraries and `--details` give. It holds the tokens, the request digests, the address, every probability and `confidence`. A Rust or TypeScript user reads `confidence` and `url` from the typed details. The shared conformance cases fail any surface that drops or changes `usage`, `requests_sent`, `cached`, `confidence` or `url`. Each surface's README says where these facts come from.

This is row L5 of `sdlc/planning/backlog-0-1-2026-09-26.md`: "one set of details on every surface". It settles asks 5 and 6 of `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`. Ian ruled that issue's shape on 2026-09-26. Ask 5 reads: "DuckDB's details match the others. The typed views in TypeScript and Rust expose `confidence` and `url`. The conformance cases check `usage`, `requests_sent`, `cached`, and `confidence`, so drift fails a gate." Ask 6 reads: "Each surface's README and the site say where run facts come from." The site part of ask 6 belongs to the marketing lead (`sdlc/planning/ownership.md`), so decision 9 hands it over.

The backlog orders L5 before B12a, the Rust library's batching ticket, so that B12a builds on the full details.

## What happens today

- The command's `--details` line holds `meta.model`, `meta.usage`, `meta.requests_sent`, `meta.cached`, `meta.requests`, `meta.url`, `meta.question_sha256`, `meta.failed_questions`, every probability, and `answer.confidence` when the backend sends it (`specification/result.md`, "`--details`" and "`meta`").
- Rust's `Details::to_json` gives that line (`crates/thinkthen/src/public/results.rs:238`). Python, TypeScript, Ruby, R, C, SQLite and PostgreSQL pass that line through.
- DuckDB returns a struct of eight members instead: the yes probability, the answer word, `value`, `nearest`, `model`, `question_sha256`, `requests_sent` and `cached` (`databases/duckdb/src/scalars.rs:56-67`, `src/scalars/calls.rs:113-140`). It drops `usage`, `requests`, `url`, `confidence`, and the distributions of choose, score and tag. Ticket 0110 decision 12 chose that struct. `sdlc/planning/databases/README.md` line 35 still says `thinkthen_details` returns "the `--details` object" on all three databases.
- DuckDB's `thinkthen_annotate` already returns JSON text (`AnnotatedRecord::value_json`).
- The Rust `Details` holds no `confidence` and no `url` (`results.rs:170-182`). Ticket 0084's normative block lists its accessors, and `sdlc/scripts/inventory` checks the built API against that block.
- The TypeScript `Details` type declares `meta.url` but not `answer.confidence` (`libraries/typescript/index.d.ts:74-98`). The line carries both at run time.
- `conformance/cases.json` checks four details fields: `answer`, `model`, `question_sha256` and `requests`. Every reply in it carries `usage`. No reply carries `confidence`.
- The runners check those four fields in two ways. TypeScript, R, SQLite and C compare the expected `answer` whole or member by member, so a new member in the expected answer is checked with no code change. Rust, Python, Ruby and PostgreSQL check a fixed list of answer members. DuckDB's runner reads only `.value` (`databases/duckdb/tools/conformance.py:56-64`).
- The command's in-process runner compares the whole decoded answer (`crates/thinkthen/src/cli/conformance_tests/outcomes/outcomes.rs:22-30`). It replays, so it sees `requests_sent` 0.
- The Rust, Python and Polars READMEs never mention `details` or `usage`.

## Design

### DuckDB returns the line

`thinkthen_details(question, text)` returns the `thinkthen.result/1` line as text, the bytes `Details::to_json` gives. `details_row` becomes one call to `to_json`, and the `DETAILS` struct type goes. The return type is DuckDB `VARCHAR`, as `thinkthen_annotate` returns today. A NULL question or text still gives a NULL row. A reader takes a member with DuckDB's JSON functions:

```sql
SELECT thinkthen_details('Is it a refund?', 'refund now') ->> '$.meta.usage.input_tokens';
```

The DuckDB suites that read struct members read the line instead:

- `tools/conformance.py`: `single` parses each line and compares the whole answer and the meta fields below.
- `tools/verbs_suite.py`: `probability_equals_details_with_no_added_send` reads `$.answer.probability`. `r3_11_details_read_null_for_an_absent_member` goes, because shared cases 07 and 12 now prove that an absent member stays absent. The secrecy row selects the line in place of `d.*`.
- `tools/settings_suite.py` line 109 reads `$.meta.cached`.

### Rust and TypeScript expose `confidence` and `url`

`Details` gains two private fields and two accessors, filled in `Details::of` from the same answer and backend the line uses:

```rust
/// The backend's own confidence in a choice or score, when it sent one.
pub fn confidence(&self) -> Option<f64>;
/// The address that answered.
pub fn url(&self) -> &str;
```

Ticket 0084's normative block gains these two lines, so `inventory` accepts them.

TypeScript's `Details.answer` gains `confidence?: number`. `meta.url` is already declared.

### The shared cases check the run facts

Every `single` answer's `details` in `cases.json` gains three fields:

- `usage`: the exchange reply's `usage`, copied.
- `requests_sent`: 1.
- `cached`: `false`.

Two replies gain a `confidence`, and their expected answers carry it: `06-choose-billing` gets 0.82, and `11-score-middle` gets 0.74. The reply of `12-score-upper` loses its `usage`, and its expected details carry none. No case is added, so `case_count` stays 54 and no skip list changes.

Every runner that reads details checks five things for each `single` case:

- `answer.confidence` equals the expected value, or both are absent.
- `meta.usage` equals the expected value, or both are absent.
- `meta.requests_sent` and `meta.cached` equal the expected values.
- `meta.url` equals the address the runner served, `BASE/systemone`.

The nine runners are the Rust consumer, C, Python, TypeScript, Ruby, R, SQLite, PostgreSQL and DuckDB. `conformance/README.md` gains one paragraph for these fields and the url rule.

### READMEs

Each of the ten surface READMEs gains one short paragraph, "Run facts". It says:

- the details call returns the `thinkthen.result/1` line;
- `model`, `usage`, every probability and `confidence` come from the backend's reply;
- the engine counts `requests_sent` and `cached`;
- `requests` holds the recording digest of each request, and `url` names the address that answered;
- the usage call returns this process's running totals;
- no call reports cost or time yet.

The Polars README and the Python frames paragraph say a frame call returns values only, and point to the details and usage calls.

## Decisions

1. **DuckDB returns the line as text.** The alternative widens the struct with usage, digests, url, confidence and a distribution. That second schema would drift from the line, and drift caused gap 3. The line also matches SQLite and the planned shape in `sdlc/planning/databases/README.md`. This overturns ticket 0110 decision 12. Nothing has shipped, so the change breaks no release (`CHANGELOG.md`, "Breaking changes: None").
2. **The type is `VARCHAR`.** DuckDB's own FFI here has no JSON type (`databases/duckdb/src/ffi.rs:19-27`), and `thinkthen_annotate` already returns JSON as `VARCHAR`. DuckDB's JSON functions read `VARCHAR`.
3. **Rust adds two accessors.** `confidence` returns `Option<f64>` and `url` returns `&str`. They read the same values the line prints, so the typed view and the line cannot disagree. The change widens the public surface by two methods, so the code review names what it checked.
4. **The expectations sit in `cases.json`, one per case.** A rule in prose ("usage equals the reply's usage") would save lines. It would also make each runner compute its own expectation, which the test gate rejects.
5. **`url` is not in `cases.json`.** The loopback port changes each run. Each runner already builds `BASE` from the case id, so it compares `meta.url` to `BASE/systemone`.
6. **The details call is the text's first send.** Each runner makes its details call on an engine with no cache, or on a fresh cache folder before any other call for that text. So `requests_sent` is 1 and `cached` is `false`. A runner that breaks this gets its order or cache setting fixed, in this ticket.
7. **The command's runner keeps its checks.** It replays in process, so `requests_sent` is 0 by construction there, and its file `cli/conformance_tests/runner.rs` belongs to ticket 0146. It already compares the whole answer, so it checks `confidence` on cases 06 and 11 with no edit. The command's `meta` fields stay pinned by its own spec pages.
8. **An absent field stays absent.** No surface writes `null` or 0 for a missing `usage` or `confidence`. Case 07 (a choice with no confidence) and case 12 (a reply with no usage) prove it.
9. **The site part goes to the marketing lead.** The lander files one issue in the marketing repository: show one raw HTTP exchange with its `usage`, say where run facts come from on each language page, and correct "ten `tt_` functions" for R. This repository's issue marks ask 6 settled for the READMEs and names that issue for the site.

## Edge cases

| Input | Expected on every surface |
| --- | --- |
| A choice reply with `confidence` 0.82 (case 06) | `answer.confidence` is 0.82. Rust `confidence()` is `Some(0.82)` |
| A score reply with `confidence` 0.74 (case 11) | `answer.confidence` is 0.74. Rust `confidence()` is `Some(0.74)` |
| A choice reply with no `confidence` (case 07) | No `confidence` key. Rust `confidence()` is `None`. TypeScript reads `undefined` |
| A yes/no or tag reply (cases 01 to 05, 09, 10, 33) | No `confidence` key |
| A reply with `usage` (every single case but 12) | `meta.usage` equals the reply's two counts |
| A reply with no `usage` (case 12) | No `meta.usage` key. Rust `usage()` is `None` |
| The first send of a text | `requests_sent` 1, `cached` `false` |
| A banded question (cases 03, 04) | The line carries the band as `"LOW:HIGH"`, on DuckDB too |
| Any single case | `meta.url` is the runner's `BASE/systemone` |
| DuckDB with a NULL question or text | A NULL row, as today |
| A base address with user information | Refused before any send, as today (`crates/thinkthen/src/core/backend.rs:33-35`). No line carries it |

## Proof

Every test runs against the loopback backend in `conformance/backend`. None sends a live call or reads a real key.

| Test | What it runs | Planted fault that turns it red |
| --- | --- | --- |
| 1. The shared cases, on nine runners | Every `single` case on the Rust consumer, C, Python, TypeScript, Ruby, R, SQLite, PostgreSQL and DuckDB, with the five checks above | (a) A copy of `cases.json` where case 01 expects 332 input tokens, 02 expects `requests_sent` 2, 04 expects `cached` `true`, 06 expects `confidence` 0.81, 07 expects `confidence` 0.5, and 12 expects a `usage`. Every runner names all six cases, or each change runs alone where a runner stops at its first failure. (b) The core line writer drops `/systemone` from `meta.url`: every runner fails on `url` |
| 2. DuckDB gives the whole line | Test 1's DuckDB rows, and the rewritten `verbs_suite.py` probability row | (c) `details_row` strips `meta.usage` from the line: case 01 fails. (d) `details_row` returns the old eight-member struct: every single case fails |
| 3. Rust's typed view | Test 1's Rust consumer rows, which read `confidence()` and `url()` | (e) `confidence()` returns `None`: case 06 fails. (f) `url()` returns the model name: every single case fails |
| 4. TypeScript's declared type | `tsc --noEmit --strict` over `tests/types.test.ts`, which assigns `audit.answer.confidence` to `number \| undefined` and `audit.meta.url` to `string` | (g) `confidence?` leaves `index.d.ts`: the compile fails |

The four questions:

- **What behavior does it protect?** One set of run facts, with one set of names, on every surface. A typed reader sees `confidence` and `url`.
- **What credible regression fails it?** A surface that builds its own details shape and drops a field, as DuckDB did. A binding that loses `usage` or reports a cached answer as a send. A typed view that falls behind the line.
- **Why does no existing test catch it?** The shared cases check four fields. DuckDB's runner checks only `value`. No test reads `confidence` or `url` through a typed view.
- **Does it need a test-only export, flag, or hook?** No. Each runner calls the public details call through its own surface. The loopback backend already serves case replies, and the compiler is the boundary for a declared type.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `crates/thinkthen/src/public/results.rs`: at most 16 net.
- `conformance/cases.json`: at most 100 added and 4 removed.
- `conformance/consumer/consumer/tests/public/cases.rs`: at most 20 net.
- Each other runner: at most 12 net.
- `databases/duckdb/src`: net negative.
- `databases/duckdb/tools`: at most 5 net.
- `libraries/typescript/index.d.ts`: 1. `tests/types.test.ts`: at most 3.
- Each README: at most 8. `conformance/README.md`: at most 6.
- Ticket 0084's block: 2.
- The record: at most 80. The run-facts issue: at most 4 changed lines. The marketing issue: at most 20.
- `sdlc/ratchet.json` and `databases/duckdb/ratchet.json` move to the measured totals. The commit says what grew.
- No dependency. No setting, so `specification/settings.md` does not change.

## Stop rules

1. Stop if ticket 0146 or 0148 has build commits on its branch when this build starts. Ask the coordinator for the order before editing `public/results.rs`, which 0146 opens, or any folder 0148 opens.
2. Stop if adding `confidence` to cases 06 and 11, or removing `usage` from case 12, changes any request digest, question digest or bare value on any surface. If some other test reads case 12's `usage`, move the no-usage reply to another synthetic single case and say which.
3. Stop if a surface's line differs from the command's for any field. That is an equivalence bug, and it needs its own decision.
4. Stop if a runner cannot make its details call the text's first send.
5. Stop if the DuckDB build the suites load lacks its JSON functions.
6. Stop before crossing a budget by more than a tenth, or adding a dependency.
7. Stop if any plant stays green.
8. Never run `sdlc/scripts/live`, read `auth.json`, or make a paid call. Unset `THINKTHEN_API_KEY` for every rung.

## Build order

The coordinator can overturn this order.

- **Ticket 0146** (the command batches) opens `public/results.rs`. This ticket adds two fields and two accessors there. 0146 changes at most 3 lines there, for a batched row's usage share. It also opens `crates/thinkthen/tests` and `specification/result.md`, which this ticket leaves alone. The shared files `sdlc/ratchet.json`, `sdlc/records`, `sdlc/tickets` and `sdlc/issues` merge as usual.
- **Ticket 0148** (engine settings everywhere) opens `conformance/consumer`, `conformance/README.md`, the whole `libraries/typescript`, `libraries/python`, `libraries/ruby`, `libraries/r` and `libraries/c` folders, `libraries/rust/README.md`, `libraries/polars/README.md`, the three database folders, and ticket 0084. This ticket edits files in each of those. The runners (`cases.rs`, `conformance.py`, `conformance.rb`, `conformance.R`, `cases.mjs`, C's `cases.rs`, the SQL runners) and every README are the closest overlap, because 0148 adds its settings runner beside each and a sentence to each README. `databases/duckdb/tools/settings_suite.py` is edited by both: 0148 removes the `cache_bytes` step at line 91, and this ticket changes line 109. 0148 adds `conformance/settings.json` and does not touch `cases.json`.
- **Order:** this ticket builds now and lands first. 0146 waits for Ian's paid "S1 live run 1", and 0148 waits for 0146. This ticket depends on neither. Both already merge `origin/main` before they build, so they pick up this change. B12a builds after this ticket, as the backlog asks.
- **Tickets 0147 and 0150.** 0147 (the recognize ADR) opens pages only, and none overlaps. 0150 (L4, find's none option and annotate parts) had no ticket on its branch when this was written. It will likely edit `cases.json` and the runners' skip lists. Whichever lands second merges.

## Scope and exclusions

Excluded: facts on every library call (B12a to B12f). The command's `--facts` total (B5). Cost and time (asks 3 and 4). SQL per-call facts, which the run-facts issue defers. Details on a frame or a Polars column. R's `tt_details` keeping only its first text. `site/`.

## Routing

Builder: Claude in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code. The change widens the public Rust surface, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 1; total 6. Final level: 2. The risk is a runner that checks a field the wrong way, which plant (a) guards on every surface.

## Deferred gaps

1. The command's in-process runner does not check `usage`, `requests_sent`, `cached` or `url` (decision 7). The command's spec pages pin them.
2. The shared cases never expect `cached` `true` or `requests_sent` above 1 in a details line. Case 40's counters cover the cache on each surface. A retried details line has no shared case.
3. `meta.failed_questions`, `meta.tool` and `meta.profile_warning` stay unchecked by the shared cases.
4. The site's raw HTTP exchange and its run-facts copy go to the marketing lead (decision 9).
5. Frames and Polars columns have no details until B13a.

## What Ian can overturn

- Decision 1: DuckDB returns the line, not a wider struct. This overturns ticket 0110 decision 12.
- Decision 2: the type is `VARCHAR`.
- Decision 7: the command's runner keeps its four checks.
- Decision 9: the site part goes to the marketing lead as an issue.
- The build order: this ticket before 0146 and 0148.

## Closes

No issue closes. The lander marks asks 5 and 6 of `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md` settled by this ticket in its status line, and names the marketing issue for the site part of ask 6. Asks 1 to 4 stay open for B5, B12a to B12f and the cost and time work.

## Evidence

- Starts from: `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`, gaps 3, 4, 7 and 8 and asks 5 and 6, from the audit at `cc51986b`. `sdlc/planning/library-equivalence-2026-09-26.md`, notes R2 and R4, proposal E6, section 3 on SQL facts, and section 5 on what the cases miss, from the audit at `eba3a72a`. Backlog row L5. The code at `origin/main` `a2fe448d`: `public/results.rs:170-300`, `databases/duckdb/src/scalars.rs:56-100`, `src/scalars/calls.rs:91-140`, `libraries/typescript/index.d.ts:74-98`, every runner's details check, and `cli/conformance_tests/outcomes/outcomes.rs`. Tickets 0146 and 0148, read from their branches. No experiment ran. The line already carries every field, so no live evidence is needed.
- Keeps: The `thinkthen.result/1` line and every surface that passes it through. Every bare value, digest and request. The command's output. DuckDB's other functions. The existing details accessors in Rust.
- Changes: DuckDB's `thinkthen_details` returns the line as text. Rust `Details` gains `confidence()` and `url()`. TypeScript's `Details.answer` gains `confidence`. `cases.json` expects `usage`, `requests_sent` and `cached` on every single case, `confidence` on two, and no `usage` on one. Nine runners check them and `meta.url`. Ten READMEs and `conformance/README.md` say where the facts come from.
- Proof: Tests 1 to 4 under "Proof", with plants (a) to (g).
- Defers: The command runner's meta checks. Cached and retried details in the shared cases. `failed_questions`, `tool` and `profile_warning`. The site part of ask 6. Frame details.
