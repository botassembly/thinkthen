---
flow: build
priority: 160
opens: crates/thinkthen/src/core/adapters/systemone/response.rs crates/thinkthen/src/core/adapters/systemone/request.rs crates/thinkthen/src/core/adapters/systemone crates/thinkthen/src/core/check.rs crates/thinkthen/tests/backend/reply_members.rs crates/thinkthen/tests/backend/main.rs crates/thinkthen/tests/score_levels.rs specification/fixtures/systemone specification/backends.md specification/question-file.md specification/check.md specification/result.md spec/check.md sdlc/scripts/result-shapes sdlc/scripts/spec sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0160: The answer contract holds at both ends

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Four promises about the wire and the result start to hold.

1. A reply in which one answer lacks its probability member fails that one question. The other answers stand, and the run exits 6, as `specification/backends.md` promises.
2. A reply that names one answer or one label twice is refused. The tool no longer picks the last copy without a word.
3. A score level whose description is `null` is sent in a form the reference backend accepts, so `check` passes against it.
4. `specification/result.md` states which changes keep `thinkthen.result/1`, names each row shape, and a check holds the pages to that list.

Each issue below blocks 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. Local experiment 273 found them. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `ebd28382`.

- `core/adapters/systemone/response.rs:37-56` declares `noul` and `probabilities` as required fields of `ResponseAnswer`. Serde refuses the whole `Response` when one answer lacks its member, so the reply fails at exit 4. `backends.md` line 100 says decode "marks one logical question failed when its answer ... lacks a probability", and `result.md` line 140 lists `missing_probability`. Report 05, finding 2.3, ran it (`sdlc/issues/2026-09-26-one-missing-answer-member-sinks-the-whole-reply.md`).
- `response.rs:25` reads `answers` into a `BTreeMap` with default serde, which keeps the last of two equal keys. Choice and score `probabilities` use the same map. Report 06, finding I-4, saw `decide` print `true` from a reply naming `q1` twice (`sdlc/issues/2026-09-26-a-duplicate-answer-name-is-accepted-and-the-last-wins.md`). The `probabilities` half was not reproduced. `core/json.rs` already holds a reader that refuses a repeated member name, and question files use it.
- `core/adapters/systemone/request.rs:258-268` sends a score level with no description as its name, and a described level as its description, `null` included. `backends.md` line 123 and `question-file.md` line 67 say the same. `check.md` line 30 says the score probe sends "`criteria` as an array holding a `null`, a string, and an object", and `core/check.rs:38` builds it from `{"fair":null,...}`. Report 06, finding I-3, ran `check --url https://api.typesafe.ai/v1` live and got exit 4 with `critical score: ... status 422`. A live `score` file with a `null` level description got 422, and the same file with an object description passed. The choice probe's `null` passed. This ticket did not rerun it, because a live call is paid (`sdlc/issues/2026-09-26-check-and-score-get-422-on-a-null-description.md`).
- Every detailed row names `thinkthen.result/1`. ADR 0036 line 10 renamed `meta.replayed` to `meta.cached` and kept `/1` "in this unreleased interface". `annotate` rows carry `answers` and no `question`, and `recognize` and `relate` rows carry their own `answer` shapes. No page says which changes keep `/1`. Report 05, finding 2.4, and report 09, issue 9, found it (`sdlc/issues/2026-09-26-the-result-schema-identifier-never-versions.md`).

## Design

### One missing member fails one question

`ResponseAnswer`'s `noul` and `probabilities` become optional at parse time. Decode maps an absent member to `missing_probability` for that logical question, the cause `result.md` already lists. The rule that a reply with no valid answer is refused at exit 4 stays. `result.md` and `backends.md` need no new sentence, because they already promise this.

### A repeated name is refused

The response is read through `core/json.rs`'s duplicate-refusing reader before serde maps it. A repeated name anywhere in the reply refuses the whole reply at exit 4, through the path that already refuses a reply that is not a `systemone` response. The message names the line and column. It never names the repeated member, because a label can be the user's own text. A repeated name cannot be pinned to one question, because either copy could be the answer the backend meant. `backends.md` gains one sentence after line 98: "A reply that names one member twice, in `answers` or inside a distribution, is refused whole, because two readers could take different values from it."

### A `null` level description sends the level's name

In the score arm of `request.rs`, a description that is `null` counts as no description, so the level sends its name. That is what a list of level names already sends, and the backend accepts it. The choice arm keeps its `null`, which the backend accepts. Pages:

- `backends.md` line 123: "a level with no description, or a `null` one, sends its name".
- `question-file.md` line 67: "A map value of `null` is no description, so the level sends its name, as a list entry does."
- `check.md` line 30: the score probe's array holds "a level name, a string, and an object", and its example body at line 38 shows `"fair"` in place of `null`. `core/check.rs:38` keeps its question, because the `null` in the map is what the probe now proves is sent as a name.

Score requests that held a `null` description change bytes, so their digests change. No tracked recording holds one. The build confirms that with a scan and stops if it finds one.

### The result's compatibility rule

`result.md` gains a section, "Compatibility", with this rule: "Under `thinkthen.result/1` a release may add a member to a row. It never renames or removes one, and it never changes a member's type or meaning. A change of that kind moves every row to `thinkthen.result/2`, and the changelog names it. A reader ignores a member it does not know. The rule binds from 0.1. ADR 0036's rename came before it. Compare `value` and the probabilities between runs, not the bytes, because a release can add a member."

The section also names each shape in a table. A row whose top level holds `answers` is an `annotate` row. Every other row holds `question.verb`, and the verb names its shape: `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `find`, `recognize` or `relate`. For each shape the table lists its top-level members and its `meta` members.

A new check, `sdlc/scripts/result-shapes`, reads that table. It reads every `thinkthen.result/1` line that a rung already checks against the binary: the `mustmatch` expectations in `spec/*.md` and `demos/*/README.md`. It fails when such a line holds a member its shape does not list, or lacks one its shape lists as always present. The `spec` rung runs it with `--self-test` and then over the repository, as it runs `sdlc/scripts/settings`. The chain holds the table to the binary: the rungs hold those lines to the binary, and this check holds the table to those lines.

The shapes keep one identifier. Separate identifiers for `annotate`, `recognize` and `relate` would change every aggregate row the week before the first release, and `question.verb` and `answers` already tell them apart.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A missing member fails one question.** The spec already says so.
2. **A repeated name refuses the whole reply.** Neither copy can be trusted.
3. **A `null` score description sends the level's name.** The backend refuses `null` there and accepts a name. The other route, refusing the file locally, would reject a file the spec allows today.
4. **Additions keep `/1`. Renames, removals and changes of meaning move to `/2`.**
5. **One identifier names every shape.** `question.verb` and `answers` name the shape.
6. **No JSON Schema file for the result.** The table and its check carry the contract. A published schema can follow once a consumer asks.

## Edge cases

| Input | Expected |
| --- | --- |
| `annotate` over three questions, a `noul`, a choice and a score, where the `noul` answer lacks `noul` | The `noul` question fails with `missing_probability`, the other two stand, exit 6 |
| The same with the choice answer lacking `probabilities` | The choice question fails with `missing_probability`, exit 6 |
| The same with the score answer lacking `probabilities` | The score question fails with `missing_probability`, exit 6 |
| `decide` whose one answer lacks `noul` | Refused at exit 4, as today, because no answer is valid |
| A reply naming `q1` twice | Refused at exit 4. The message names a line and column, and neither the name nor either value |
| A choice answer naming one label twice in `probabilities` | Refused at exit 4 |
| A score file with levels `{"low":null,"high":"Work stops."}`, `--dry-run` | `"criteria":["low","Work stops."]` |
| A choice file with `{"a":null,"b":"x"}`, `--dry-run` | `"criteria":{"a":null,"b":"x"}`, as today |
| A pinned row that gains a member its shape does not list | `result-shapes` fails and names the page, the shape and the member |
| A shape's always-present member missing from a pinned row | `result-shapes` fails and names them |

## Proof

The backend rows run the compiled binary against the in-process loopback in `tests/backend/harness`, which serves a fixed reply body. No row needs a new conformance arm.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `one_missing_member_fails_one_question`, new in `tests/backend/reply_members.rs` | Edge rows 1 to 4, each pinning standard output, the failure marker and the exit code | (a) Keep the members required: rows 1 to 3 exit 4 with no row. (b) Map a missing member to `missing_answer`: the markers differ. (c) Accept a reply with no valid answer: row 4 exits 6 |
| `a_repeated_name_refuses_the_reply`, same file | Edge rows 5 and 6, each pinning exit 4 and the whole standard error line | (d) Read `answers` with default serde: row 5 prints `true` at exit 0. (e) Check only `answers` and not `probabilities`: row 6 passes |
| `a_null_level_sends_its_name`, new in `tests/score_levels.rs` | Edge rows 7 and 8 through `--dry-run`, pinning the whole `criteria` member | (f) Send `null` again: row 7 fails. (g) Apply the rule to choice too: row 8 fails |
| `result-shapes --self-test` in the `spec` rung | Edge rows 9 and 10 on copies of a real page, pinning each sentence | (h) Drop the unknown-member rule: row 9's plant passes. (i) Drop the always-present rule: row 10's plant passes |

The existing request test `a_score_map_writes_its_descriptions_in_order_and_null_stays_null` in `request.rs` is deleted, because `a_null_level_sends_its_name` covers the same rule outside-in. The existing `spec/check.md` pins update to the new score body.

The four questions:

- **What behavior does it protect?** Per-question failure, a refused ambiguous reply, a score file the backend accepts, and pages that match the result contract.
- **What credible regression fails it?** A required member creeping back, default map reading, a `null` level sent again, or a renamed `meta` member with no page change.
- **Why does no existing test catch it?** The partial-reply tests break answers that still carry their member. No test sends a repeated name. The request unit test pins the `null` that the backend refuses. Nothing reads the pages' rows against a member list.
- **Does it need a test-only hook?** No. The loopback serves ordinary bytes, `--dry-run` is the real plan, and the check reads real pages.

The last proof is paid. After the build lands, one `thinkthen check --url https://api.typesafe.ai/v1` run under `sdlc/scripts/live` confirms that the score probe passes. It waits for Ian's authorization. The ticket's record states report 06's live finding, marked as not rerun. The null-description issue closes only when that run exits 0.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `core/adapters/systemone/response.rs`: at most 30 net.
- `core/adapters/systemone/request.rs`: at most 3 net, beside the deleted unit test.
- `tests/backend/reply_members.rs`: at most 160, new, and one `mod` line. `tests/score_levels.rs`: at most 50, new.
- `sdlc/scripts/result-shapes`: at most 140 with its self-test. `sdlc/scripts/spec`: at most 2 lines.
- Pages: at most 40 net, the shape table included.
- `sdlc/ratchet.json` moves to the measured total, at most 40 above main. The commit says what grew.
- No dependency. The `surfaces` rung runs, because the adapter that every surface shares changes.
- Paid calls: one `check` run, only with Ian's authorization.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if a tracked recording holds a score request with a `null` level description.
3. Stop if a missing member changes the exit code of any existing partial-reply test.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. Never run `sdlc/scripts/live` unless Ian authorizes the one `check` run.
6. Stop if the change needs `cli/failure.rs`, `public/error.rs` or a file that ticket 0146 opens other than `backends.md`, `question-file.md` and `result.md`.
7. Stop if the paid `check` run fails. Report its finding lines.

## Build order

It builds after ticket 0159 lands, because 0159 re-keys the fixtures and edits `check.md`. It may build beside ticket 0161. The two share only `tests/backend/main.rs`'s module list and `sdlc/ratchet.json`. It lands before ticket 0146 builds, or after 0146 lands, because 0146 opens `backends.md`, `question-file.md` and `result.md`.

## Scope and exclusions

Excluded: a result JSON Schema file, separate identifiers per shape, tie policy and other severity 3 items of review 05, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 2; total 7. Final level: 2. The risk is a parse that accepts a reply it should refuse, which rows 4 to 6 guard.

## Deferred gaps

- A check that the shape table under `/1` only grows. Review enforces it until 0.1 is tagged. A later check can compare against the table at the tag.
- A published result JSON Schema per shape.
- The batching tickets apply the compatibility rule to `meta.batch` when they land (report 05, finding 2.6).

## What Ian can overturn

- Decision 2: a repeated name refuses the whole reply.
- Decision 3: a `null` score description sends the level's name, in place of a local refusal.
- Decision 4: the compatibility rule. This one is outward-facing and binds consumers from 0.1.
- Decision 5: one identifier for every shape.

## Closes

`sdlc/issues/2026-09-26-one-missing-answer-member-sinks-the-whole-reply.md`, `sdlc/issues/2026-09-26-a-duplicate-answer-name-is-accepted-and-the-last-wins.md`, `sdlc/issues/2026-09-26-the-result-schema-identifier-never-versions.md`, and `sdlc/issues/2026-09-26-check-and-score-get-422-on-a-null-description.md` once the paid run passes. Findings 1 and 2 of `sdlc/issues/2026-09-26-architect-review-05-answer-contract.md`.

## Evidence

- Starts from: Local experiment 273, report 05 findings 2.3 and 2.4, report 06 findings I-3 and I-4, and report 09 issue 9, as the four issues and the review 05 file record them. The code at `origin/main` `ebd28382`: `response.rs:25` and `:37-56`, `request.rs:258-268`, `core/check.rs:38`, `core/json.rs`. `backends.md` lines 100 and 123, `question-file.md` line 67, `check.md` line 30, `result.md` line 140, ADR 0036 line 10.
- Keeps: Every reply that decodes today decodes the same. A reply with no valid answer is still refused. The choice `null`. Every row's members and the identifier `thinkthen.result/1`.
- Changes: A missing member fails one question. A repeated name refuses the reply. A `null` score level sends its name. `result.md` states the compatibility rule and names each shape, and a check holds the pages to it.
- Proof: Three outside-in tests and a self-tested check with nine plants, the `install`, `lint`, `test`, `spec` and `surfaces` rungs, and one paid `check` run with Ian's authorization.
- Defers: A growth check at the 0.1 tag, a result JSON Schema, and the batching tickets' use of the rule.
