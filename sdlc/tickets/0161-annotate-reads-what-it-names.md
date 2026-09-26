---
flow: build
priority: 161
opens: crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/annotate/plan.rs crates/thinkthen/src/cli/annotate crates/thinkthen/src/core/plan_document.rs crates/thinkthen/tests/backend/annotate_on.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/annotate_plan.rs specification/annotate.md spec/annotate.md spec/fixtures/annotate demos/16-triage-pipeline/README.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0161: `annotate` reads what its set names, and its plan shows what it sends

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A question set's `on` pointer reads inside the JSON value the record selected. It never re-reads a string as a JSON document. So a record's result no longer depends on whether a string happens to hold JSON. A set that points `on` into text is refused with a sentence that names the question. Every example on `specification/annotate.md` runs from a new executable page and exits as the page says. Under a backend profile, `--dry-run` prints the first request the run would really send and the number of requests each group makes.

Three issues from local experiment 273, report 03, block 0.1 under the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. The spec-example issue asks that it and the `on` issue be fixed together. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `ebd28382`.

- `cli/annotate.rs:299-300` builds a nested `Reading` over `base_evidence.as_text()?.as_bytes()` and parses it again as a JSON document. So a `/body` string holding JSON text is parsed and `on` sends part of it. A `/body` string of plain text stops the run at exit 2 with `the input is not valid JSON`, and the message names neither the question nor `on`. A `--lines` line that looks like JSON gets members, though `records.md` says a text line has none. Finding 2-2 ran all three (`sdlc/issues/2026-09-26-annotate-on-reparses-selected-text-as-json.md`).
- `core/records.rs::annotation_record` reads a whole document as a JSON value when it parses and as text otherwise. So `annotate set.json < record.json` with a JSON object works today, and `annotate.md`, "What it prints", promises that an object document gains the answers.
- `annotate.md` line 30 says `on` "is a JSON Pointer inside the evidence that `--field` selected", and "an object or a list travels as that JSON value".
- `annotate.md` defines `triage.json`, whose `unresolved` question reads `"on": "/body"` (line 23). Its examples run the set on a text document (`< issue.txt`, line 81) and on `--jsonl --field /body` (line 85). Both exit 2 with `the input is not valid JSON`. `spec/` has no annotate page, so no rung runs them. Finding 2-1 ran them (`sdlc/issues/2026-09-26-annotate-spec-example-fails-on-its-own-inputs.md`).
- `cli/annotate/plan.rs:58-62` calls `facade::split` for each group, discards the chunks, and prints `plans.first()`, the unsplit group. Under a profile that caps questions per request, the dry run shows one request holding every question. Finding 2-3 printed 1,500 questions under a profile of 100. Live, a 250-question set under a 100-question profile showed one request and sent 3 (`sdlc/issues/2026-09-26-annotate-dry-run-under-a-profile-prints-the-unsplit-request.md`). `annotate.md` line 94 says the dry run "prints the first request", and how-to 16 says it "shows the exact request".

## Design

### `on` reads the selected value

`plan_for` applies each question's `on` pointers to the value that the base reading selected. It does not serialize and re-parse it. The base selection is one of these:

- A JSON object or array: the whole JSONL record, a CSV or TSV row, a document that parsed as JSON, or an object or array that `--field` selected. `on` reads inside it by the `state` rule of `records.md`, as `annotate.md` line 30 says.
- Text: a `--lines` line, a document that did not parse as JSON, or a string, number, boolean or `null` that `--field` selected. Text has no members.

A set with a question whose `on` is not the root, run under `--lines`, is refused before any input is read, at exit 2: ``question `NAME` reads `on`, and a --lines record is text with no members``. The same question over a record whose selection is text refuses that record at exit 2, before any request for it, with ``question `NAME` reads `on`, and this record's evidence is text with no members``. That follows `annotate.md`'s rule for an input error in one record. `annotate.md` line 30 gains: "A string is text, even when it holds JSON. `on` never parses it."

The change reuses the command's existing input and usage refusals, so `cli/failure.rs` does not change.

### The page's examples run

`triage.json` keeps `"on": "/body"`, because it shows the feature. The examples change their input to match it:

- `thinkthen annotate triage.json < issue.json`, where `issue.json` is one JSON object with `id` and `body`. A document object gains the answers.
- `thinkthen annotate triage.json --jsonl < issues.jsonl`, with no `--field`.
- `thinkthen annotate triage.json --dry-run < issue.json`.

A new executable page, `spec/annotate.md`, runs each example on the page. It replays hand-built `local-1` entries under `spec/fixtures/annotate/`, as `spec/relate.md` replays `spec/fixtures/relate-partial`. The page names `--model local-1 --no-cache --replay`, so no key or network is read. It also runs the `--lines` refusal and pins its sentence.

### The dry run prints the request it would send

Under a profile, `--dry-run` prints the first chunk of the first group, which is the first request a live run sends. The plan object gains two members after `on`: `request_count`, the number of requests the first record makes, and `group_requests`, the number each group makes, in group order. Without a profile, or when every group fits, the plan prints the single request as today and adds the two counts. `recognize-plan` and `relate-plan` already name `request_count`, so the word matches.

`annotate.md`, "`--dry-run`", states both members and says the printed request is the first chunk under a profile. How-to 16's "shows the exact request" then holds, and the how-to adds the two members to its example if it prints one.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **`on` reads JSON values only.** A string is text even when it holds JSON. This keeps a record's result independent of its data.
2. **A whole document that parses as JSON keeps its members.** `annotate` already reads such a document as a JSON value, and the page promises it.
3. **`--lines` with a non-root `on` is refused before reading.** Every line would fail, so the run should not start.
4. **The spec examples change their input, not the set.**
5. **The plan gains `request_count` and `group_requests`.**

## Edge cases

| Input | Expected |
| --- | --- |
| `--jsonl` record `{"body":"{\"x\":1}"}`, `--field /body`, a question with `on: /x` | Refused for that record at exit 2 with the text sentence. Today it sends `1` |
| `--jsonl` record `{"body":"plain words"}`, `--field /body`, the same question | Refused at exit 2 with the text sentence. Today it says the input is not valid JSON |
| `--jsonl` record `{"body":{"x":1}}`, `--field /body`, `on: /x` | The question reads `1`, as today |
| `--lines` with a line `{"x":1}` and a question with `on: /x` | Refused before reading at exit 2 with the `--lines` sentence |
| `--lines` with a set whose questions have no `on` | Runs as today |
| A document `{"body":"text"}` with `on: /body` | The question reads `text` |
| A plain-text document with `on: /body` | Refused at exit 2 with the text sentence |
| A 250-question set, a profile capping 100 questions a request, `--dry-run` | The printed request holds 100 questions. `request_count` is 3. `group_requests` is `[3]` |
| A set with two `on` groups, no profile, `--dry-run` | The first group's request, `request_count` 2, `group_requests` `[1,1]` |

## Proof

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `on_reads_the_selected_value`, new in `tests/backend/annotate_on.rs` | Edge rows 1 to 7. Each row pins standard output, the whole standard error line and the exit code, and counts the loopback's requests, so a refused record sends none | (a) Re-parse text again: rows 1 and 2 differ. (b) Refuse every non-root `on`: rows 3 and 6 fail. (c) Refuse `--lines` per record, after reading: row 4 counts requests or prints a row first |
| `the_plan_shows_each_request`, new in `tests/annotate_plan.rs` | Edge rows 8 and 9 through `--dry-run`, pinning the question count of the printed request and both new members | (d) Print the unsplit group: row 8 holds 250 questions. (e) Count groups, not chunks: row 8's `request_count` is 1 |
| `spec/annotate.md`, new, in the `spec` rung | Every example on `annotate.md` and the `--lines` refusal | (f) Put `--field /body` back in the JSONL example: the page exits 2 |

The four questions:

- **What behavior does it protect?** `on` reads inside JSON and never re-reads text, the page's examples, and a dry run that shows the sent request and its count.
- **What credible regression fails it?** A return to re-parsing, a refusal that also blocks real JSON selections, and a plan that prints the unsplit group or counts groups.
- **Why does no existing test catch it?** No test puts JSON text inside a string for `on`, no rung runs the page's examples, and no dry-run test uses a profile that splits.
- **Does it need a test-only hook?** No. The loopback counts requests, `--dry-run` is the real plan, and the page replays ordinary entries.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `cli/annotate.rs` and `cli/annotate/`: at most 40 net.
- `core/plan_document.rs`: at most 15 net.
- `tests/backend/annotate_on.rs`: at most 150, new, and one `mod` line. `tests/annotate_plan.rs`: at most 80, new.
- `spec/annotate.md`: at most 90, new. `spec/fixtures/annotate/`: hand-built `local-1` entries only.
- Pages: at most 15 net.
- `sdlc/ratchet.json` moves to the measured total, at most 55 above main. The commit says what grew.
- No dependency. No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs `cli/failure.rs`, `public/`, or a file that ticket 0146 opens.
3. Stop if a whole JSON document, a JSONL record or a CSV row loses its members under `on`.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Build order

It builds after tickets 0150 and 0159 land, because 0150 opens `cli/annotate.rs` and 0159 re-keys the fixtures. It may build beside ticket 0160. It lands before ticket 0146 builds, or after 0146 lands.

## Scope and exclusions

Excluded: the mixed-model cache fix, library `on` handling (the libraries take a set through `public/set.rs`, which ticket 0150 holds), and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 0; reach 1; proof 1; cost of error 1; total 4. Final level: 2.

## Deferred gaps

- The libraries' annotate path. `public/set.rs` builds evidence without the command's nested reading. The builder checks whether it re-parses text too and files an issue if it does.

## What Ian can overturn

- Decision 1: `on` never parses a string.
- Decision 3: a non-root `on` under `--lines` is refused before reading.
- Decision 5: the two new plan members.

## Closes

`sdlc/issues/2026-09-26-annotate-on-reparses-selected-text-as-json.md`, `sdlc/issues/2026-09-26-annotate-spec-example-fails-on-its-own-inputs.md`, and `sdlc/issues/2026-09-26-annotate-dry-run-under-a-profile-prints-the-unsplit-request.md`.

## Evidence

- Starts from: Local experiment 273, report 03 findings 2-1, 2-2 and 2-3, as the three issues record them. The code at `origin/main` `ebd28382`: `cli/annotate.rs:299-300`, `cli/annotate/plan.rs:58-62`, `core/records.rs::annotation_record`. `annotate.md` lines 23, 30, 81, 85 and 94.
- Keeps: `on` over JSON records, rows and documents. Every run whose `on` reads JSON. The single-request plan bytes when nothing splits, beside the two new members.
- Changes: `on` never re-parses text and refuses text with a named question. The page's examples. An executable annotate page. The dry run prints the first real chunk and the request counts.
- Proof: Two outside-in tests and one executable page with six plants, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: A check of the libraries' annotate path.
