# 0126: Build the wording, help, and doc-claim fixes

Status: built. Plants recorded. Ladder green. Owner: Claude.

Branch `ticket/0126-wording-help-and-doc-claims`. The ticket is `sdlc/tickets/0126-wording-help-and-doc-claims.md`. Its design review accepted it after three rounds. Ian can overturn each choice this record marks as decided.

## Result

- **Help order.** The six admin commands carry clap `display_order` 1000 to 1005, after clap's own `help` at 999. The root help lists the ten functions, then `help`, then `audit`, `diff`, `status`, `check`, `cache`, and `transform`. The enum did not move. `tests/version.rs` changes its `ORDER` array. `tests/audit_refusals.rs` and `tests/diff.rs` now find their own row anywhere in the root help and keep pinning its description. Both test names lost their "after" words.
- **Throttle sentence.** The engine's `WidthActive` says throttle. `public/error.rs` prints the engine's sentence. `cli/schedule/width_tests.rs` and `tests/public_env.rs` pin it. `engine/width_tests.rs` asserts the `WidthActive` value.
- **check.** The report opens with `url`, `provider systemone`, `model asked NAME|unspecified`, and `model sent NAME`. It prints one `reply PROBE JSON` line per decoded reply, before the rows. `core::check::PROVIDER` is `built_in::NAME`, the adapter alias of `systemone::NAME`. Code outside the adapter names it through that alias, as the seam policy asks. `core::check::reply_line` serializes the decoded reply through `json_line`. A failed logical question prints the `FailedValue` marker. The help gains one sentence.
- **recognize --dry-run.** It prints `thinkthen.recognize-plan/1` with `url`, `model`, `key_env`, `words`, `request_count`, and a `requests` list of `digest`, `bytes`, and `body_utf8`. The three backend fields cost 6 nonblank lines, under the 9 the ticket allows. The request entry owns its strings. It borrows nothing from the split chunks, so one report value serves the empty and the full case. Relate keeps its own copy of the struct until 0123 lands.
- **Help sentences.** New sentences cover the recognize cost, the `--jobs` connections, the `--record` conflict stop, and the check models. `tests/version.rs` pins all four as whole sentences. It sits beside the root help test, because `tests/decide_edge.rs` is at its 500-line size cap.
- **Conflict message.** The command prints ``the backend answered the request in entry `NAME` differently from the saved response; record into a fresh folder, or use --cache DIR to answer from the saved entries``. `tests/backend/recording_conflicts.rs` pins it whole. The public layer's own sentence for library callers is unchanged, because a library caller has no `--cache DIR`.
- **Docs.** ADR 0010 gains its 2026-09-25 amendment. `specification/recording.md`, `threshold.md`, `records.md`, `recognize.md`, `channels.md`, and `check.md` changed. So did `spec/check.md` and `spec/recognize.md`, the interface audit's history note, `README.md`, `ten-use-cases.md`, `site/src/pages/backends.astro`, and the three database READMEs.

## Sources checked for each number

- Experiment 212's `RESULTS.md` gives 63 of 100 moved, a largest move of 0.08, and a mean of 0.02 among those that moved. It gives up to 0.08 for the fifty borderline messages and 0.03 for the fifty others. It gives four flips, all between 0.43 and 0.51, and states the sampling limit.
- Experiment 259's `challenge/CHALLENGE.md` and `digest-conflicts.txt` give 178 of 681 repeated digests with different answers, all from `jev-1.13.0`, with a largest gap of 0.09.
- Experiment 218's `wave1/fragments/load.md` gives 30 to 35 open descriptors at `--jobs 32` and 7 at `--jobs 4`.
- The interface audit rows were checked against the built binary on 2026-09-25. `decide --true --false --dry-run` sends `criteria.true` and `criteria.false`. `choose --option a=desc` sends `"a":"desc"`. `engine/http.rs` reads `Retry-After`. Commit `b0cba964` added that reading, and ticket 0064 bounds it.
- **The 300 ms claim.** No record under `probes/`, `sdlc/records/`, or the workspace experiments measured a live call at 300 ms. Every hit is a stub delay or a test wait. By the ticket's rule, `README.md` and `ten-use-cases.md` drop the number. They now say each decision waits on a network round trip and a process start.

## Budgets

In nonblank lines added, net of removals:

| Part | Budget | Measured |
| --- | --- | --- |
| `cli/check.rs` and `core/check.rs` | 50 | 45 |
| `cli/recognize/dry_run.rs` | 30 | 11 |
| `cli/args/command.rs` and `cli/args.rs` | 20 | 20 |
| `engine/mod.rs`, `public/error.rs`, `cli/failure/recording.rs` | 5 | -2 |
| Tests | 170 | 150 |

The ratchet rises by 224 lines. It went from 61768 to 61992 on the branch. After the merge of main at `f3176b5d`, which brought 0122 and 0124, it went from 61972 to 62196. Moving the help sentence test into `tests/version.rs` saved one line. Replacing an index in `tests/backend/check.rs` for clippy added one back, so the branch stood at 62196 after the merge of `3a86d814`. Main then fell to 61971 with 0129, and the final figure is 62195, measured after the merge of `6acf7d80`. The ticket allowed 280. The code, the check replies, and their test pins make up most of the growth. The builder deleted the public error's throttle copy and the engine test's two text pins first.

## Plants

Each plant was applied alone, its test run, the file restored and touched. All eleven turned RED. The runner is outside the repository and applied each plant as one exact text replacement.

| # | Plant | Test that turned red |
| --- | --- | --- |
| 1 | Drop `display_order` from `status` | `version::each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order` |
| 2 | Print no reply line for a reply with a failed answer | `check::a_missing_answer_is_critical_in_every_probe_and_names_the_tag_range` |
| 3 | Print the sent model in the reply line | `check::the_report_names_the_model_asked_the_model_sent_and_the_model_each_reply_names` |
| 4 | Print the resolved model as `model asked` | `check::a_backend_that_carries_every_field_passes_on_four_requests` |
| 5 | Leave the `provider` line out | `spec/check.md`, the dry-run block |
| 6 | Build the recognize dry-run plan without the kind questions | `recognize::the_dry_run_prints_the_requests_a_live_run_sends` |
| 7 | Keep the recognize field named `tokens` | `spec/recognize.md`, the dry-run block |
| 8 | Put `width` back in the engine's sentence | `cli::schedule::width_tests::the_command_selects_only_an_explicit_jobs_and_refuses_a_later_different_one` |
| 9 | Give `public/error.rs` its own `width` copy | `public_env::a_seed_leaves_the_throttle_omitted_and_an_explicit_one_registers_at_build` |
| 10 | Drop the `--cache` clause from the conflict message | `recording_conflicts::whitespace_padded_responses_with_different_values_conflict` |
| 11 | Drop the `--jobs` connection sentence | `version::the_long_help_names_connections_conflicts_paid_requests_and_models` |

## Ladder

The final run was on merge `78d50824`, after main's `6acf7d80` brought 0129. Each rung ran once, and none was wrapped in the heavy lock: lint 0, install 0, test 0, and spec 0. The test rung ran all 32 test binaries, 370 of them in `backend`. The spec rung ran the pages and 21 green demos. The surfaces rung did not run. No surface changed, because the public throttle sentence reads as the surfaces already pin it.

Earlier runs found four problems, all fixed before the final run.

- Lint stopped on `tests/decide_edge.rs` at its 500-line cap. The help sentence test moved to `tests/version.rs`. Plant 11 was rerun against the moved test and turned red.
- Clippy refused an index in the new check test. It now reads the model with `get`.
- `tests/transform.rs` pinned `transform` between `cache` and `audit` in the root help. It now finds its own row anywhere, like the audit and diff tests. `tests/version.rs` pins the order.
- One test run failed `annotate::scheduling::a_backend_failure_after_the_output_pipe_closes_stays_quiet`, exit 4 against 0. That run overlapped a plant run on the same worktree under load. The test passed five times alone and in the final run. This ticket does not touch annotate scheduling. No stop rule was crossed.

## Found on the way

- `sdlc/planning/interface-audit.md` and `sdlc/planning/how-to-portfolio-study.md` name a private caller's repository and path. This ticket left them as written, because both are history and outside its scope. The repository goes public, so the release ticket should replace the name with a generic consumer.
