# 0160: Build the answer contract at both ends

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0160-the-answer-contract-holds`, built in lane 2. The ticket is `sdlc/tickets/0160-the-answer-contract-holds.md`, accepted at `798a88d9`. The build merged `origin/main` `31a30ea9` first, after ticket 0159 landed. Ian can overturn every decision the ticket lists. Every test, plant and rung ran with `THINKTHEN_API_KEY` unset, against recordings and loopback backends. One paid `check` run went through `sdlc/scripts/live`, as the last section says.

## Result

- A reply in which one answer lacks `noul` or `probabilities` fails that one question with `missing_probability`. A `null` distribution reads the same way. The other answers stand, and the run exits 6. A reply with no valid answer is still refused at exit 4.
- `answers` and each choice or score distribution read through `unique`, which refuses a repeated member name. The refusal takes the existing path: exit 4, "the response is not a systemone response", with the line and column and never the name.
- A score level whose map value is `null` is sent as `{}` in its place. A list of names still sends names, a described level still sends its description, and a choice `null` still sends `null`. The request unit test that pinned the `null` is deleted, as the ticket says.
- `specification/result.md` gains "Compatibility": the `/1` rule, one shape row per command, and the paragraph on `question.verb`. Its nine `thinkthen.result/1` example rows name their command in the fence, and the eight that lacked it gained `"failed_questions":0`.
- `spec/result.md` is new. It replays one command from each of demos 01, 02, 03, 06, 14, 15, 17, 39, 44 and 45 with `--details`, reads the table, and holds the ten replayed rows and nine example rows to it. Every demo replayed with `--details` added, so stop rule 7 did not fire.
- Pages: `backends.md` (the repeated-name sentence and the score row), `question-file.md` line 67, `check.md` (the score probe row and its example body), and `specification/fixtures/check/requests.jsonl` line 3.
- A scan of every tracked file for a score `criteria` array holding `null` found none outside the files this ticket changes. The same scan at the merge base found exactly those five files. So no recording changes, and stop rule 2 did not fire.

The response module passed its 500-line ceiling with the new reader. Its in-file tests moved unchanged to `response_tests.rs`, beside the two test files it already had. The reader's visitor sits at module level, and a `Wire` type names the distribution, so clippy's nesting and type rules pass.

## Tests

- `a_missing_member_fails_only_its_question` in `tests/backend/annotate/partial_failure.rs` runs edge rows 1 to 5 over one four-question file. Each row pins exit 6, `failed_questions` 1, the failed question's whole `failure` member, and no failure on the other three.
- `a_repeated_answer_name_refuses_the_reply` in `tests/backend/tag/matrix.rs` and `a_repeated_option_name_refuses_the_reply` in `tests/backend/choosing.rs` pin exit 4, empty standard output, and the whole standard error line of edge rows 7 and 8. Both columns, 93 and 165, matched the ticket's.
- The two score tests in `tests/question_file/structured.rs` changed in place. The second is renamed `a_score_map_of_nulls_sends_empty_objects_where_a_list_sends_names`. `tests/backend/asked.rs` pins `{}` in the last place.

The four questions, for the three new tests and the page, are the ticket's. No test needs a test-only hook.

## Plants

Each plant was applied in the working tree, run, and restored from a copy. A `cmp` against the copy confirmed each restore.

| Plant | Result |
| --- | --- |
| (a) Members required again, the reader at `origin/main` | Red: the missing-member table and both repeated-name tests |
| (b) A missing member maps to `missing_answer` | Red: the missing-member table |
| (c) `read_tag` answers a label with no `noul` | Red: the missing-member table |
| (d) `answers` read with default serde | Red: `a_repeated_answer_name_refuses_the_reply` |
| (e) Distributions read without `unique_some` | Red: `a_repeated_option_name_refuses_the_reply` |
| (f) A `null` level sent as `null` | Red: both score tests in `structured.rs` |
| (g) A `null` level sent as its name | Red: both score tests in `structured.rs` |
| (k) `unique_some` calls `unique` on the value directly | Red: the missing-member table |
| (h) `requests_sent` deleted from the `decide` table row | Red: `replayed decide: meta member requests_sent is not in the table`, and the same for both `decide` example rows |
| (i) An unmarked `extra` added to the `score` table row | Red: `replayed score: member extra is missing` and `example score: member extra is missing` |
| (j) `failed_questions` deleted from one example row | Red: `example decide: meta member failed_questions is missing` |
| (l) `filter` written as the `filter` row's verb | Red: `replayed filter: question.verb is decide, the table says filter` |
| An example fence with no command word | Red: `example (no command): no table row` |

## Budgets

Nonblank lines, net against `31a30ea9`.

| Item | Budget | Measured |
| --- | --- | --- |
| `response.rs`, with its moved tests | at most 45 | 43 |
| `request.rs` | at most 3 in the score arm | 3 in the arm, −10 with the deleted unit test |
| `fields.rs` | at most 2 | 1 |
| `partial_failure.rs` | at most 55 | 51 |
| `tag/matrix.rs` | at most 20 | 17 |
| `choosing.rs` | at most 20 | 15 |
| `structured.rs` and `asked.rs` | 0 | 0 |
| `spec/result.md` | at most 60 | 44 |
| Pages | at most 30 | 17 |
| `sdlc/ratchet.json` | at most 135 above main | 117, from 70,193 to 70,310 |

No dependency was added. The new shape table does not repeat the profile warning's shape string.

## Ladder

| Rung | Result |
| --- | --- |
| `lint`, with `THINKTHEN_PRIVATE_NAMES` set | exit 0. Its log holds one bare "Killed" line, as expected |
| `test` | exit 0 |
| `spec` | exit 0, with `spec/result.md` passing and demos 21 green |
| `surfaces` | exit 0, with every landed surface and the release smoke passing |

The first `lint` failed on the response module's size ceiling. The second failed on clippy's nesting and type rules. The commit `b50e0425` fixed both.

## The paid run

Report 06, finding I-3, got exit 4 with `critical score: ... status 422` from `check --url https://api.typesafe.ai/v1` when the score probe sent `null`. This build did not rerun that finding against the old bytes.

With Ian's authorization for this ticket, one job ran `thinkthen check --url https://api.typesafe.ai/v1` from the branch binary at `b50e0425` under `sdlc/scripts/live --max-tokens 5000`. It ran before landing, so a refusal would reach Ian before the change lands. It exited 0. Every probe passed: `ok connection`, `ok key`, `ok endpoint`, `ok noul`, `ok choice`, `ok score`, `ok mixed`, `ok usage`, and `critical 0, warning 0`. The score probe's reply read `{"fair":0.02,"good":0.0,"excellent":0.98}`. The four replies reported 1,644 tokens. The ledger charged the 5,000-token cap, and at $0.042 for a million input tokens (`sdlc/planning/open-concerns.md`) that is well under one cent. The backend accepts the empty object, so stop rule 8 did not fire and no name fallback goes to Ian.

## Left for landing and later

- The lander closes the four issues and findings 1 and 2 of architect review 05, as the ticket's "Closes" says. The null-description issue may close, because the paid run passed.
- `origin/main` moved to `6eb3c524` after the build merged it. The lander merges it again.
- The ticket's deferred gaps stand.
