# 0146: Build the command batches decide, filter and rank

Status: landed 2026-09-27 after a fresh code review accepted it. Owner: Claude.

Branch `ticket/0146-command-batches-decide-filter-rank`, in the worktree `worktrees/thinkthen-0146`. The ticket is `sdlc/tickets/0146-command-batches-decide-filter-rank.md`. The build merged `origin/main` at `18f0381e` first. ADR 0048, ADR 0053 and ADR 0055 rule the behavior. Ian can overturn every decision the ticket lists. No live call ran. Every rung and plant ran with `THINKTHEN_API_KEY` unset, against the loopback backend.

## Baseline

"S1 live run 1" is `probes/speed/runs/s1-live-2/`, committed on this branch as the baseline. Its log ends: "Target, filter over 306 titles under 0.5 s at the default: not met (11.664, 12.48, 12.28 s)". The "B4 live run" after landing measures against it.

## Result

- `decide`, `filter` and `rank` over a stream fill each request to the limit by default. `--batch N|max`, `THINKTHEN_BATCH` and a `decide` file's `batch` set the limit, in that order, then `max`.
- `core/batch.rs` sends ADR 0055's sentence, `Each question quotes the text it asks about.`, as the evidence of a decide batch of two or more distinct records. `choose`, `tag` and `score` keep the records list, and a context keeps the context. A batch closes at 4,096 members and on a pause. The three `batch-` fixtures changed to match, and `batch-choose` and `batch-context` did not move.
- `cli/asking/batched.rs` is the one batched path. A record thread parses records ahead through a bounded channel. The batch reader plans a batch for each ask of the engine scheduler and sends the open batch after 50 ms without input, in every mode. A worker asks the batch through `Engine::ask_batch` and builds each row through the shared `row_of`.
- The engine scheduler counts records, not items. `Completed` carries `records` and `stop`, so a partial reply prints the rows before the failed record and stops there.
- Each row carries an even share of the batch's usage and attempts. The earliest records carry the remainder.
- A failed batch of two or more records prints one stop line naming its range. A partial reply prints its own line at exit 4. A batch of one, a cancellation and an input refusal keep today's lines.
- `QuestionFile::parse_top` takes `batch` off the top of a `decide` file, so the question digest does not change. The schema's `decide` entry gains `batch`.
- `probes/speed/functions.jsonl` drops the `list` field from the three rows. The speed gate now needs one request for each 12-line workload.
- The pages named in the ticket carry its sentences. `grep -rnE "Not built yet, by ADR 0048 item (1|2)[ :]" specification` finds nothing. The design issue's test 9 now describes the quoted form. Its B5 row and three pause rows already read as the ticket asks.

## Pins

The first full test run under the default, before any pin, failed in 22 files, under stop rule 5's 40. They were `audit_model`, `audit`, `audit_verbs`, `audit_write`, `demo_runner`, `speed`, the unit `cli/schedule/width_tests`, and backend `cache_identity`, `default_cache/usage`, `interrupt`, `keeping`, `keeping/default_framing`, `loopback_cases`, `parallel`, `profile/warnings`, `recording_conflicts`, `refused`, `resend`, `scheduling`, `streaming`, `streaming/record_rows` and `table`.

- `tests/backend/harness` gains `spawn_one`, which sets `THINKTHEN_BATCH=1`. Twelve files import it as `spawn`: `keeping`, `keeping/default_framing`, `parallel`, `streaming`, `table`, `profile/warnings`, `recording_conflicts`, `cache_identity`, `refused`, `resend`, `loopback_cases` and `default_cache`'s lock test through `--batch 1`.
- `tests/support/measure.rs` and `interrupt.rs` set `THINKTHEN_BATCH=1`. `audit.rs`, `cli/schedule/width_tests.rs`, `parallel.rs`'s piped child, the `table.rs` closed-pipe test and `scheduling.rs`'s ordered-output test pass `--batch 1`.
- Demos 03, 06, 12, 13, 41 and 43 pin `--batch 1` in their pages and `record.sh`. The dry-run expectations of 03 and 06 show the batch plan. Demo 41 lost seven words of prose to stay under ADR 0016's 900.
- `spec/audit.md` and `spec/result.md` pin `--batch 1`. `spec/decide.md`'s dry run expects the batch plan.
- The intended difference under stop rule 3 changed one assertion. In `profile/warnings.rs` a refused first record at `--jobs 2` now sends nothing after it, where today the second record went out.
- The C library's door test compares its bytes with the command's. It sets `THINKTHEN_BATCH=1`, because the libraries ask one record a request until B12a. The `surfaces` rung found it. `libraries/c` is outside the ticket's opened files, and its ratchet rises 2 lines to 2,211.
- The speed test's plants moved off the three batched rows. One adds `--batch 1` to `filter`, and one marks `choose`'s entry as landed.

## Tests

`crates/thinkthen/tests/backend/batching.rs` and `batching/tiers.rs` hold the seven named tests. The tier test splits in two for clippy's function length: `the_batch_setting_follows_its_tiers` and `a_file_batch_leaves_the_question_digest_and_stays_off_annotate_entries`. Two rows sit elsewhere. The `--record` and `--replay` rows join `an_answer_arrives_before_the_next_record_at_one_job_and_the_default` in `scheduling.rs`. The interrupted batch is `an_interrupted_batch_finishes_and_starts_no_other` in `interrupt.rs`, which holds the signal helpers.

## Plants

A scratchpad script applied each plant, ran the named test under the heavy lock, and restored the file. `git status` was clean after each.

| Plant | Result |
| --- | --- |
| Close a size batch at N+1 | Red: `order_holds_across_jobs` |
| Close at 4,097 members | Red: `repeats_close_a_batch_at_the_member_cap` |
| No member cap | Red: `repeats_close_a_batch_at_the_member_cap` |
| Keep the records list for a decide batch | Red: `each_row_carries_its_share` |
| The remainder goes to the last record | Red: `each_row_carries_its_share` |
| Every row carries the whole usage | Red: `each_row_carries_its_share` |
| Never pause | Red: `a_pause_sends_the_open_batch` and the scheduling rows |
| Name the range off by one | Red: `a_failed_batch_stops_at_its_first_record` |
| The one-line form for a batch of one | Red: `a_failed_batch_stops_at_its_first_record` |
| The file beats the environment | Red: `the_batch_setting_follows_its_tiers` |

## Deviations

- The shares are built into each record's `Answered`, so `result_json.rs` and `public/results.rs` did not change.
- An empty `THINKTHEN_BATCH` counts as unset, and `settings.md` says so. The ticket's refusal table lists an empty value among the refusals for `--batch` and `THINKTHEN_BATCH`. The command still refuses an empty `--batch`. Ian can overturn this.
- `ReplyTooLarge` keeps the two-line form. The ticket lists it with neither form.
- `replay_answers_every_batch` counts requests at a live loopback instead of stopping it. The count proves the replay sent nothing.
- The replay-miss stop line holds the loopback's port in its entry name, so the test pins the text around the name.
- The scheduling rows share one listener, because a recording's entry name covers the address.
- Demo 03's "A paid request for every record" bullet no longer holds under the default. Its pin keeps it true, and D1 rewrites it.

## Ticket text that turned out wrong

- An annotate question set whose entry holds `batch` prints `the question set holds no key `questions.ok.batch``, not `a question file takes no key `batch``. The test pins the real sentence.
- The budgets for `cli/asking/batched.rs` and for `edge.rs`, `judge.rs` and `asked.rs` were too small. See below.

## Code review

A fresh code review accepted the build. Its fixes landed in `3d52419e`:

- `base_url()` has its doc line back, and `batch()` keeps only its own.
- `settings.md` says an empty `THINKTHEN_BATCH` counts as unset.
- `a_table_shares_one_request_and_filter_prints_its_kept_rows` pins one request for three CSV rows, with the kept row printed as a compact JSON object.
- The `read` wrapper and the single-use `wanted` binding are gone. The `Top` alias stays, because clippy's type-complexity rule refuses the tuple in `read_top`'s signature.
- The batch reader's `ended` flag is gone. A closed channel ends the open batch and answers with what it queued, or `Input::End`.
- `tiers.rs` pins that a `choose` file holding `batch` is refused as today, with `a question file takes no key `batch``.

## Budget

The coordinator ruled the overrun structural, because the reviewer found no duplication beyond the fixes above. The budgets rise to the measured sizes. Ian can overturn this.

Nonblank lines against `origin/main` after the merge, at `f39ca1b3`.

| Area | Ticket budget | Measured, and the new budget |
| --- | --- | --- |
| `cli/asking/batched.rs` | 250 | 337 |
| `edge.rs`, `judge.rs`, `asked.rs` | 35 | 69 |
| `cli/asking.rs` and `cli/asking/plan.rs` | 25 | 46, including the moved plan path's module head and imports |
| Product code total | 561 | 612 |
| `tests/backend/batching.rs` and its folder | 480 | 574 |
| Ratchet over main | 1,193 | 1,260, from main's 72,322 to 73,582 |

The batched path holds the reader thread, the batch queue with its pause, the dry-run plan, the per-record rows and the stop mapping. The build trimmed it before the review: parsing moved to the record thread, which dropped a generic parser, and the tier resolution became one `match`. The lint's 500-line file ceiling moved the plan path to `cli/asking/plan.rs`, the unit tests of `core/result.rs` and `engine/annotate_schedule.rs` into sibling `tests.rs` files, and the tier tests to `tests/backend/batching/tiers.rs`. Clippy's nesting and type rules added `feed`, `Former::next` and three type aliases. The C library's ratchet moves from main's 2,258 to 2,260 for its pin.

## Ladder

Each rung ran once, alone, with the load average under 10, at `314f5e8d` and after.

| Rung | Result |
| --- | --- |
| `lint` | exit 0; ratchet 73,400 of 73,400 |
| `test` | exit 0; 982 passed, 0 failed across 38 test binaries |
| `spec` | exit 0; demos 21 green, 0 red |
| `surfaces` | exit 1 on `libraries/c`, the door's byte comparison under the default. After the pin, exit 0: every surface and the release smoke pass |

`install` did not run. No dependency changed.
