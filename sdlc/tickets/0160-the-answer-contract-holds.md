---
flow: build
priority: 160
opens: crates/thinkthen/src/core/adapters/systemone/response.rs crates/thinkthen/src/core/adapters/systemone/request.rs crates/thinkthen/src/core/question_file/fields.rs crates/thinkthen/tests/backend/annotate/partial_failure.rs crates/thinkthen/tests/backend/tag/matrix.rs crates/thinkthen/tests/backend/choosing.rs crates/thinkthen/tests/backend/asked.rs crates/thinkthen/tests/question_file/structured.rs specification/fixtures/check/requests.jsonl specification/backends.md specification/question-file.md specification/check.md specification/result.md spec/result.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0160: The answer contract holds at both ends

Status: COMPLETE.

Opened as: 2026-10-11. 2026-09-26 (`sdlc/records/0160-build-the-answer-contract-holds.md`). A fresh read-only code review accepted it with no defects. The coordinator accepted it on 2026-09-26 after three fresh read-only reviews and a coordinator fix to the ratchet budget. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Four promises about the wire and the result start to hold.

1. A reply in which one answer lacks its probability member fails that one question. The other answers stand, and the run exits 6, as `specification/backends.md` promises.
2. A reply that names one answer or one label twice is refused. The tool no longer picks the last copy without a word.
3. A score level whose description is `null` is sent in a form the reference backend accepts, so `check` passes against it. The level's name never stands in for the `null`, as ADR 0039 rules.
4. `specification/result.md` states which changes keep `thinkthen.result/1` and names each row shape. An executable page holds that list to rows the binary writes.

Each issue below blocks 0.1 by the placement in `sdlc/planning/backlog-0-1-2026-09-26.md`. Local experiment 273 found them. Ian can overturn each design choice.

## What happens today

Read from `origin/main` `e24f7324`.

- `core/adapters/systemone/response.rs:37-56` declares `noul` and `probabilities` as required fields of `ResponseAnswer`. Serde refuses the whole `Response` when one answer lacks its member, so the reply fails at exit 4. `backends.md` line 100 says decode "marks one logical question failed when its answer ... lacks a probability", and `result.md` line 140 lists `missing_probability`. Report 05, finding 2.3, ran it (`sdlc/issues/closed/2026-09-26-one-missing-answer-member-sinks-the-whole-reply.md`).
- `response.rs:25` reads `answers` into a `BTreeMap` with default serde, which keeps the last of two equal keys. Choice and score `probabilities` use the same map. Report 06, finding I-4, saw `decide` print `true` from a reply naming `q1` twice (`sdlc/issues/closed/2026-09-26-a-duplicate-answer-name-is-accepted-and-the-last-wins.md`). The `probabilities` half was not reproduced. `core/json.rs` holds a reader that refuses a repeated member name, but it does not fit here. `Json::parse` at `json.rs:88` takes `&str` while the body is `&[u8]`, and its `JsonError::DuplicateName` at `json.rs:30-33` carries only a path, with no line or column.
- `response/observed.rs:11-16` parses the body once with `serde_json::from_slice` and turns any serde error into `DecodeError::Malformed(line, column)`. That error prints "the response is not a systemone response: the JSON at line L column C is not one" and never quotes the body.
- ADR 0039 line 12 rules: "Null map descriptions remain null, never a fallback to the name." `core/question_file/fields.rs:126-136` enforces it in the core. Its `described` function keeps a `null` as the score level's description. `core/adapters/systemone/request.rs:258-268`, the score arm of `RequestQuestion::asking`, sends a level with no description as its name and a described level as its description, `null` included. So a map value of `null` reaches the wire as `null`. `backends.md` line 123 and `question-file.md` line 67 say the same. `specification/check.md` line 30 says the score probe sends "`criteria` as an array holding a `null`, a string, and an object", and `core/check.rs:38` builds it from `{"fair":null,...}`. `specification/fixtures/check/requests.jsonl` line 3 holds that body, and both `spec/check.md` line 11 and `tests/backend/check.rs:261` compare the dry run against it. Report 06, finding I-3, ran `check --url https://api.typesafe.ai/v1` live and got exit 4 with `critical score: ... status 422`. A live `score` file with a `null` level description got 422, and the same file with an object description passed. The choice probe's `null` passed. This ticket did not rerun it, because a live call is paid (`sdlc/issues/closed/2026-09-26-check-and-score-get-422-on-a-null-description.md`).
- ADR 0039 omits a `null` tag description only in the structured form. There a tag label with a `null` description sends `{"label":NAME}` with no `description` member (`request.rs:223-231`, `tag_instructions`). The entry keeps the label's name, because the tag entry has a `label` member of its own. A string-only tag question sends its sentence and no criteria at all, so it has no description to omit.
- Every detailed row names `thinkthen.result/1`. ADR 0036 line 10 renamed `meta.replayed` to `meta.cached` and kept `/1` "in this unreleased interface". `annotate` rows carry `answers` and no `question`, and `recognize` and `relate` rows carry their own `answer` shapes. No page says which changes keep `/1`. Report 05, finding 2.4, and report 09, issue 9, found it (`sdlc/issues/closed/2026-09-26-the-result-schema-identifier-never-versions.md`).
- No `thinkthen.result/1` line sits in any `spec/*.md` page or `demos/*/README.md` expectation. The pinned rows live on `specification/result.md`, which no rung executes, and in Rust tests at `tests/backend/choosing.rs:159`, `exchange.rs:131` and `find.rs:69`. The example rows on `result.md` lines 31, 57, 67, 75, 83, 123, 129 and 137 lack `meta.failed_questions`, which the binary always writes (`exchange.rs:131` pins `"failed_questions":0`). So the page has already drifted from the binary.
- `filter` and `rank` ask a `decide` question of every record (`core/question_file/resolve.rs:23`). Their detailed rows print `question.verb` `decide` and carry an `input` member. `result.md` says so at line 35, and its example rows at lines 123 and 129 show it. Demos 03 and 06 print the same. `core/result.rs:332` makes `input` optional on every single-judgment row, and `result_json.rs:48` sets it for a record. So a `decide`, `choose`, `tag` or `score` row carries `input` in record mode and lacks it on one document. `question.verb` alone cannot tell a `filter` row from a `decide` record row.
- Every result shape has a green demo that replays a committed recording with no key and no network: `decide` in demo 01, `choose` in 02, `filter` in 03, `rank` in 06, `annotate` in 14, `find` in 15, `score` in 17, `tag` in 39, `recognize` in 44, and `relate` in 45.

## Design

### One missing member fails one question

`ResponseAnswer`'s `noul` becomes `Option<f64>` with `#[serde(default)]`. The `probabilities` of `Choice` and `Score` become `Option<BTreeMap<String, f64>>`, read as the next section says. Decode maps an absent member to `DecodeError::MissingProbability(place)`, which `cause` already turns into `missing_probability`. The same holds inside `read_tag` at `response.rs:173`. A tag label whose `noul` is absent fails that tag with `missing_probability`, as the first failure `read_tag` meets. A present `null` member reads as absent. Today it refuses the whole reply. For `noul`, `Option<f64>` already reads `null` as `None`. For `probabilities`, `unique_some` reads the value as an option first, so a `null` becomes `None` before any map is read, and the question fails alone with `missing_probability`. The rule that a reply with no valid answer is refused at exit 4 stays. `result.md` and `backends.md` need no new sentence, because they already promise this.

### A repeated name is refused

`response.rs` gains one function, `unique`, and the two maps read through it. There is no second parse and no use of `core/json.rs`.

```rust
/// Read one JSON object into a map, refusing a member name it already holds.
fn unique<'de, D: Deserializer<'de>, V: Deserialize<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, V>, D::Error>
```

It calls `deserializer.deserialize_map` with a private visitor. The visitor's `visit_map` reads each entry with `next_entry::<String, V>()`, inserts it into a `BTreeMap`, and returns `A::Error::custom("a member name repeats")` when `insert` returns `Some`. A second function, `unique_some`, serves the optional distributions. It reads the value as an `Option` first, through a private newtype whose one field reads through `unique`. A `null` becomes `None`, and an object reads through `unique` and becomes `Some`. Calling `unique` straight on a `null` would refuse the whole reply with "invalid type: null, expected an object", which a scratch run with `serde_json` 1.0.151 showed.

- `Response.answers` takes `#[serde(deserialize_with = "unique")]`.
- `Choice.probabilities` and `Score.probabilities` take `#[serde(default, deserialize_with = "unique_some")]`, so an absent member reads as `None` and a present one refuses a repeat.

The custom error carries no position of its own, and `serde_json` gives it the line and column where the reader stood. It flows through the one `from_slice` call at `observed.rs:11` into `DecodeError::Malformed(line, column)` at `observed.rs:16`, the path that already refuses a reply that is not a `systemone` response. The run exits 4 with that message. It never names the repeated member, because a label can be the user's own text. A repeated name cannot be pinned to one question, because either copy could be the answer the backend meant. The derived struct readers already refuse a repeated `model`, `usage`, `type`, `noul` or `confidence` member. `backends.md` gains one sentence after line 98: "A reply that names one member twice, in `answers` or inside a distribution, is refused whole, because two readers could take different values from it."

### A `null` level description is sent with its description omitted

The rule lives in one place, the score arm of `RequestQuestion::asking` in `request.rs`. A level whose description is `null` keeps its place in the `criteria` array as an empty object, `{}`. That is the level's entry with its description member omitted. ADR 0039's structured tag form omits a `null` description the same way, but a tag entry keeps its `label` member, so it still names the label. A score level has no name member in the criteria array, so omitting its description leaves an empty object. The level's name is never sent in its place, so this keeps Ian's ruling in ADR 0039 line 12. A level with no description, which only a list of names makes, still sends its name. A described level still sends its description. The choice arm keeps its `null`, which the backend accepts.

`fields.rs` keeps its behavior, because the core still holds the `null`. Its doc comment on `described` at lines 124-128 says the wire carries a score description. It changes to: "`score` keeps `null` as the description the map named, and the adapter sends it as an empty object in the level's place. `score` refuses a blank string, because the wire would carry it."

A description written as `{}` sends the same bytes as a `null`. Both mean the level carries nothing beyond its place. The question digest still tells them apart, because it reads the question file, not the request.

Pages:

- `backends.md` line 123: "`type` `score`, with the `criteria` array in level order: a level from a list of names sends its name, a described level sends the description the map held, and a `null` description sends an empty object in its place. The name never stands in for a `null`."
- `question-file.md` line 67: "A map value of `null` is no description. The request sends an empty object in that level's place, and the level's name is never substituted for it."
- `specification/check.md` line 30: the score probe's array holds "an empty object for a `null` description, a string, and an object". Its example body at line 38 shows `{}` in place of `null`.
- `specification/fixtures/check/requests.jsonl` line 3 shows `{}` in place of `null`, so `spec/check.md` line 11 and `tests/backend/check.rs:261` still match the dry run byte for byte.
- `core/check.rs:38` keeps its question. The `null` in its map is what the probe now proves is sent as an empty object.

Score requests that held a `null` description change bytes, so their recording digests change. No tracked recording holds one. The build confirms that with a scan and stops if it finds one.

The one paid `check` run below proves the backend accepts the empty object. If the backend refuses it too, the build stops, and the coordinator brings Ian the name fallback as a reversal of ADR 0039. The ticket does not choose the name on its own.

### The result's compatibility rule

`result.md` gains a section, "Compatibility", with this rule: "Under `thinkthen.result/1` a release may add a member to a row. It never renames or removes one, and it never changes a member's type or meaning. A change of that kind moves every row to `thinkthen.result/2`, and the changelog names it. A reader ignores a member it does not know. The rule binds from 0.1. ADR 0036's rename came before it. Compare `value` and the probabilities between runs, not the bytes, because a release can add a member."

The section describes the binary as it is and changes no behavior. It names each shape in one table, with one table row per command: `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `find`, `annotate`, `recognize` and `relate`. Each table row gives the command, the `question.verb` its rows print, the top-level members, and the `meta` members. The `filter` and `rank` rows say plainly that they print `question.verb` `decide` and always carry `input`. The `decide`, `choose`, `tag` and `score` rows list `input?`, because only a record carries it. The `annotate` row prints no `question` and holds `answers`. A member that is sometimes absent carries a trailing `?`, as `usage?` and `profile_warning?` do. Members that are not built yet stay out of the table until the ticket that builds them adds them.

Separate rows per command are the plainer form. One `decide` row with `input?` would hide that `filter` and `rank` always carry `input`, and the reader would have to know that those commands ask `decide`.

The section tells consumers the truth about telling rows apart. It says: "`question.verb` names the kind of question asked, not the command. A `filter` row, a `rank` row and a `decide` record row print the same verb, `decide`, and the same members. A reader that needs the command keeps it from the command line that wrote the rows." A consumer that reads one command's output never needs more.

The build adds `"failed_questions":0` to every example row on `result.md` that lacks it, so the page's rows agree with the binary.

### An executable page holds the table to the binary

A new executable page, `spec/result.md`, runs in the `spec` rung through `mustmatch test spec`. No rung script changes. It has one block.

1. For each of the ten commands, it changes into that command's demo folder listed above and reruns one command the demo's page already runs, with the same `--replay recording` and `--details` added where the demo leaves it out. `--details` changes no request byte (`exchange.rs` pins that), so the committed recording answers it. It keeps the first row and labels it with the command that produced it.
2. It reads that command's row from the table in `specification/result.md`. It matches by the command label, never by `question.verb`.
3. One `jq` program, written once in the block, compares them. It reports each top-level or `meta` member the replayed row holds that the table row does not list. It reports each table member without a `?` that the replayed row lacks. It reports a `question.verb` that differs from the table row's verb.
4. The same program runs over every `thinkthen.result/1` example row on `specification/result.md`. Each example row's fence names its command as the second word of the info string, as in `json rank`. The build adds that word to the fences at lines 30, 56, 66, 74, 82, 122, 128, 136 and 142. The row at line 122 is labelled `decide`, as the sentence after it says, and the row at line 128 is labelled `rank`. The page matches each example row by that label. A `thinkthen.result/1` row in a fence with no command word is itself a finding.
5. The block pipes the findings to `mustmatch ""`, so the page fails with each finding line naming the command and the member.

The chain is short. The binary writes the replayed rows, and the table must match them. The example rows must match the table.

The shapes keep one identifier. Separate identifiers for `annotate`, `recognize` and `relate` would change every aggregate row the week before the first release. A consumer already knows the command it ran.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A missing member fails one question.** The spec already says so.
2. **A repeated name refuses the whole reply.** Neither copy can be trusted.
3. **A `null` score description is sent as its level's entry with the description omitted, an empty object.** This follows ADR 0039's structured tag form, which omits a `null` description and keeps the `label` member. A score level has no such member, so its entry is empty. It keeps Ian's ruling in ADR 0039 line 12 that a `null` never falls back to the name. The one paid `check` run proves the backend accepts it. If the backend refuses it, the name fallback goes to Ian as a reversal of ADR 0039.
4. **Additions keep `/1`. Renames, removals and changes of meaning move to `/2`.**
5. **One identifier names every shape.** The command that wrote a row names its shape. `question.verb` names the question kind, so `filter` and `rank` rows print `decide`.
6. **No JSON Schema file for the result.** The table and its executable page carry the contract. A published schema can follow once a consumer asks.
7. **An executable page replaces a new check script.** Demo recordings already produce every shape's row with no network, so `mustmatch` can hold the table to the binary without a script or a self-test.

## Edge cases

| Input | Expected |
| --- | --- |
| `annotate` over four questions, a `decide`, a `choose`, a `score` and a two-label `tag`, where the `decide` answer lacks `noul` | The `decide` question fails with `missing_probability`, the other three stand, exit 6 |
| The same with the choice answer lacking `probabilities` | The choice question fails with `missing_probability`, exit 6 |
| The same with the score answer lacking `probabilities` | The score question fails with `missing_probability`, exit 6 |
| The same with the tag's second label answer lacking `noul` | The tag fails with `missing_probability`, the other three stand, exit 6 |
| The same with the choice answer `{"type":"choice","probabilities":null}` | The choice question fails with `missing_probability`, the other three stand, exit 6 |
| `decide` whose one answer lacks `noul` | Refused at exit 4, as today, because no answer is valid |
| `tag` with labels `billing` and `urgent`, served `{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1},"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}` | Refused at exit 4 with empty standard output. Standard error is exactly `thinkthen: the reply was refused: the response is not a systemone response: the JSON at line 1 column 93 is not one` and a newline. It names neither the name nor either value |
| `choose` over `billing`, `shipping`, `account` and `other`, served `{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"billing","probabilities":{"billing":0.1,"billing":0.9,"shipping":0.04,"account":0.04,"other":0.02}}}}` | Refused at exit 4 with empty standard output. Standard error is exactly `thinkthen: the reply was refused: the response is not a systemone response: the JSON at line 1 column 165 is not one` and a newline |
| A score file with levels `{"low":{"what":"Little disruption."},"high":null}`, `--dry-run` | `"criteria":[{"what":"Little disruption."},{}]` |
| A score file with levels `{"low":null,"high":null}`, beside the list `["low","high"]` | The map sends `[{},{}]` and the list sends `["low","high"]` |
| A choice file with a `null` option description, `--dry-run` | `null`, as today |
| A replayed row holding a member its command's table row does not list | `spec/result.md` fails and names the command and the member |
| A table member without `?` missing from a replayed row | `spec/result.md` fails and names them |
| An example row on `result.md` that disagrees with its command's table row | `spec/result.md` fails and names them |

## Proof

The backend rows run the compiled binary against the in-process loopback in `tests/backend/harness`, which serves a fixed reply body. No row needs a new conformance arm, a new test file, or a new `mod` line.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_missing_member_fails_only_its_question`, one table test added to `tests/backend/annotate/partial_failure.rs`, reusing its `questions` and `run` helpers at lines 10-37 | Edge rows 1 to 5 over one four-question file. Each row removes one member from one base reply, or sets it to `null` in row 5, and pins the exit code, `failed_questions` and the failed question's whole failure member | (a) Keep the members required: every row exits 4 with no row. (b) Map a missing member to `missing_answer`: the markers differ. (c) Let `read_tag` skip a label with no `noul`: row 4 answers the tag. (k) Let `unique_some` call `unique` on the value without reading it as an option first: row 5 exits 4 with no row |
| `a_repeated_answer_name_refuses_the_reply`, one test added to `tests/backend/tag/matrix.rs` beside `each_invalid_tag_reply_is_a_backend_failure`, calling that file's `tag` helper with no extra arguments | Edge row 7. It pins exit 4, empty standard output, and standard error equal to the whole line in row 7 | (d) Read `answers` with default serde: the last `q1` wins, and the command prints `["billing"]` at exit 0 |
| `a_repeated_option_name_refuses_the_reply`, one test added to `tests/backend/choosing.rs` beside `a_reply_that_answers_with_the_wrong_shape_or_leaves_an_option_out_is_refused`, calling that file's `choose` helper with no extra arguments | Edge row 8. It pins exit 4, empty standard output, and standard error equal to the whole line in row 8 | (e) Read only `answers` through `unique`: the last `billing` wins, and the command prints `"billing"` at exit 0 |
| The two score tests in `tests/question_file/structured.rs` at lines 99-128, changed in place | Edge rows 9 and 10 through `--dry-run`, pinning the whole `criteria` member. The second test's name becomes `a_score_map_of_nulls_sends_empty_objects_where_a_list_sends_names` | (f) Send `null` again: both fail. (g) Send the level's name: both fail |
| `spec/result.md` in the `spec` rung | Edge rows 12 to 14 against real replayed rows | (h) Delete `requests_sent` from the `decide` table row: the page fails. (i) Add an unmarked `extra` member to the `score` table row: the page fails. (j) Delete `failed_questions` from one example row: the page fails. (l) Write `filter` as the verb in the `filter` table row: the page fails. The build runs each plant by hand and records the finding line |

Overlap was checked, and these rows are not new tests.

- Edge row 6 is the contract `a_group_with_no_valid_answer_remains_a_backend_failure` at `partial_failure.rs:120` already pins. That test fails every answer by wrong kind. A missing member reaches the same all-failed check, so a second test adds nothing.
- Edge row 11 is `structured.rs:91-95`, which pins the choice `null` today.
- `tests/choose_and_score_edge.rs:75-85` pins a list of level names. It holds no `null` and stays as it is.
- `tests/question_file/corpus.rs:32` checks only each corpus case's exit code, so it overlaps nothing here.
- `tests/backend/tag/matrix.rs:105` refuses invalid tag replies for a missing answer, a wrong kind, a nonfinite number and a probability out of range. It holds no repeated name, and it checks only that standard error contains a phrase. Line 117 already pins "response is not a systemone response" for the nonfinite number. Edge row 7 goes in a test of its own beside it, because it pins the whole line.
- `choosing.rs:217` refuses a wrong shape, a missing option and an out-of-range probability for `choose`. It checks that standard error names `q1`, which a repeated name never does, so edge row 8 goes in a test of its own beside it.

Other pins that change with the wire:

- `tests/backend/asked.rs:252` pins `[{"what":"No impact."},"Partial.",null]`. It becomes `{}` in the last place.
- The request unit test `a_score_map_writes_its_descriptions_in_order_and_null_stays_null` at `request.rs:429` is deleted. The first `structured.rs` score test covers the same rule outside-in.
- `specification/fixtures/check/requests.jsonl` line 3 changes as the design says.

The four questions:

- **What behavior does it protect?** Per-question failure, a refused ambiguous reply, a score file the backend accepts with no name fallback, and pages that match the rows the binary writes.
- **What credible regression fails it?** A required member creeping back, default map reading, a `null` level sent again or replaced by its name, or a renamed `meta` member with no page change.
- **Why does no existing test catch it?** The partial-reply tests break answers that still carry their member. No test sends a repeated name. The score tests pin the `null` that the backend refuses. Nothing reads real rows against a member list, and the page has already drifted on `failed_questions`.
- **Does it need a test-only hook?** No. The loopback serves ordinary bytes, `--dry-run` is the real plan, and the page replays committed recordings.

The last proof is paid. After the build lands, one `thinkthen check --url https://api.typesafe.ai/v1` run under `sdlc/scripts/live` confirms that the score probe passes with the empty object. It waits for Ian's authorization. The ticket's record states report 06's live finding, marked as not rerun. The null-description issue closes only when that run exits 0.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `core/adapters/systemone/response.rs`: at most 45 net, the `unique` reader included.
- `core/adapters/systemone/request.rs`: at most 3 net in the score arm, beside the deleted unit test.
- `core/question_file/fields.rs`: the doc comment only, at most 2 net.
- `tests/backend/annotate/partial_failure.rs`: at most 55 net.
- `tests/backend/tag/matrix.rs`: at most 20 net.
- `tests/backend/choosing.rs`: at most 20 net.
- `tests/question_file/structured.rs` and `tests/backend/asked.rs`: at most 0 net.
- `spec/result.md`: at most 60, new.
- Pages: at most 30 net, the compatibility section and the shape table included.
- `sdlc/ratchet.json` moves to the measured total, at most 135 above main. The ratchet counts test files too, so this is the sum of the per-file budgets above. The commit says what grew. The new shape table must not repeat the string `{"tuned_for":NAME,"running":NAME}`, because `tests/backend/profile.rs:498` counts it once in `result.md`.
- No dependency. The `surfaces` rung runs, because the adapter that every surface shares changes.
- Paid calls: one `check` run, only with Ian's authorization.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if a tracked recording holds a score request with a `null` level description.
3. Stop if a missing member changes the exit code of any existing partial-reply test.
4. Stop if any plant stays green.
5. Stop if the build needs a live call. Never run `sdlc/scripts/live` unless Ian authorizes the one `check` run.
6. Stop if the change needs `cli/failure.rs`, `public/error.rs` or a file that ticket 0146 opens other than `backends.md`, `question-file.md` and `result.md`.
7. Stop if a demo's command cannot replay with `--details` added. Name the demo.
8. Stop if the paid `check` run fails. Report its finding lines. If the score probe is refused with the empty object, bring Ian the name fallback as a reversal of ADR 0039, with its trade-offs. Do not send the name without his ruling.

## Build order

It builds after ticket 0159 lands, because 0159 re-keys the fixtures and edits `check.md`. It may build beside ticket 0161. The two share only `sdlc/ratchet.json`. It lands before ticket 0146 builds, or after 0146 lands, because 0146 opens `backends.md`, `question-file.md` and `result.md`.

## Scope and exclusions

Excluded: a result JSON Schema file, separate identifiers per shape, a check script for the shapes, tie policy and other severity 3 items of review 05, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 2; total 7. Final level: 2. The risk is a parse that accepts a reply it should refuse, which edge rows 6 to 8 guard.

## Deferred gaps

- A check that the shape table under `/1` only grows. Review enforces it until 0.1 is tagged. A later check can compare against the table at the tag.
- A published result JSON Schema per shape.
- The batching tickets apply the compatibility rule to `meta.batch` when they land (report 05, finding 2.6), and add their members to the shape table.
- The Rust tests that pin whole rows keep their own copies. They agree with the table through the binary, and nothing ties them to it directly.

## What Ian can overturn

- Decision 2: a repeated name refuses the whole reply.
- Decision 3: a `null` score description is sent as an empty object. It keeps his ADR 0039 ruling. The name fallback would reverse that ruling and comes to him only if the paid run refuses the empty object.
- Decision 4: the compatibility rule. This one is outward-facing and binds consumers from 0.1.
- Decision 5: one identifier for every shape.
- Decision 7: an executable page over demo recordings, in place of a check script.

## Closes

`sdlc/issues/closed/2026-09-26-one-missing-answer-member-sinks-the-whole-reply.md`, `sdlc/issues/closed/2026-09-26-a-duplicate-answer-name-is-accepted-and-the-last-wins.md`, `sdlc/issues/closed/2026-09-26-the-result-schema-identifier-never-versions.md`, and `sdlc/issues/closed/2026-09-26-check-and-score-get-422-on-a-null-description.md` once the paid run passes. Findings 1 and 2 of `sdlc/issues/2026-09-26-architect-review-05-answer-contract.md`.

## Evidence

- Starts from: Local experiment 273, report 05 findings 2.3 and 2.4, report 06 findings I-3 and I-4, and report 09 issue 9, as the four issues and the review 05 file record them. ADR 0039 line 12 and its tag omission rule. The code at `origin/main` `e24f7324`: `response.rs:25`, `:37-56` and `:173`, `response/observed.rs:11-16`, `request.rs:258-268`, `question_file/fields.rs:124-136`, `core/check.rs:38`, `core/json.rs:30-33` and `:88`. `backends.md` lines 100 and 123, `question-file.md` line 67, `specification/check.md` lines 30 and 38, `specification/fixtures/check/requests.jsonl` line 3, `result.md` line 140 and its example rows, ADR 0036 line 10, and the ten demo recordings.
- Keeps: Every reply that decodes today and names no member twice decodes the same. A reply with no valid answer is still refused. The choice `null`. The core's `null` score description and its question digest. Every row's members, each `question.verb`, and the identifier `thinkthen.result/1`. `filter` and `rank` rows still print `decide` and carry `input`.
- Changes: A missing member or a `null` distribution fails one question. A repeated name refuses the reply. A `null` score level is sent as an empty object. `result.md` states the compatibility rule and names each command's shape, its example rows gain `failed_questions` and a command label on their fences, and `spec/result.md` holds the table to replayed rows.
- Proof: One table test and two single-case tests added to existing files, two score tests changed in place, an executable page with four hand-run plants, the `install`, `lint`, `test`, `spec` and `surfaces` rungs, and one paid `check` run with Ian's authorization.
- Defers: A growth check at the 0.1 tag, a result JSON Schema, the batching tickets' use of the rule, and a direct tie between the Rust row pins and the table.
