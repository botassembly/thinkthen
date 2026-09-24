# 0082: Close the 0.1 command contract

Status: built on `ticket/0082-close-command-contract`, not merged. Awaiting the final-diff reviews the ticket names.

## Result

The built help, the README, the how-to index, the green how-tos, the crate description, and the specification now use the approved words. `decide --help` teaches the four outcomes as `The exit code is 0 for yes, 1 for no, 3 for not sure, and any other code when the run is broken or interrupted.` `specification/decide.md` defines the formal term once: `` `unresolved` is the formal name for a not sure answer. `` `recognize --help` carries the record-run sentence. `relate --help` carries `A run that answers some relation questions and fails others prints what it has and exits 6. A run whose relation questions all fail prints nothing and exits 4.` `annotate` opens with `Answer a saved set of questions about every record.` No result key, request byte, behavior, or exit code changed. One diagnostic changed its wording after the code review: the `--jobs` refusal on a single input now says `a single text sends one request`, matching the help.

`sdlc/scripts/demos` (rung 3) now checks the approved vocabulary over the built root help, `-h` and `--help` for all ten functions, the root README, the how-to index, and every green how-to. It prints `demos: FILE:LINE: "WORD" breaks the RULE rule` for every hit and fails on any. Its unsanctioned-hit count on this branch is 0 (`demos: 21 green, 0 red`). Before the fix it printed 93 hits.

Commits: `331e85f7` (the wording, the pins, the check), `3ef97c06` (row 41), `a7987762` (row 39), `48a57d84` (this record), `3358001c` (the clippy fix below), and `05223ae1` (main merged after 0090 landed).

## Baseline and byte diff

The baseline is main at `ccc947d5`, which holds the 0088 and 0089 landing records. The branch rebased onto it before any product edit. `sdlc/records/0082-help-before.txt` holds the 21 captures from that binary: root `--help`, then `-h` and `--help` for each function in root order. `sdlc/records/0082-help-after.txt` holds the same 21 captures from this branch. Every capture exited 0 before and after. Clap prints trailing spaces on blank lines inside long help. The committed files drop them so `git diff --check` passes, and no other byte changed.

| Command | Capture | Exit | First sentence before | First sentence after |
|---|---|---|---|---|
| root | `--help` | 0 | Semantic commands for the shell: if, grep, and sort that understand meaning | same |
| decide | `-h`, `--help` | 0 | Answer one yes or no question about a text. | same |
| filter | `-h`, `--help` | 0 | Keep the records where the answer is yes | same |
| rank | `-h`, `--help` | 0 | Sort records by how likely the answer is yes. | same |
| choose | `-h`, `--help` | 0 | Pick one option from your list | same |
| find | `-h`, `--help` | 0 | Pick the one line or record that best answers a question. | same |
| score | `-h`, `--help` | 0 | Place a text on a scale you name | same |
| tag | `-h`, `--help` | 0 | Name every label that fits | same |
| annotate | `-h`, `--help` | 0 | Fill out a question set for every record | Answer a saved set of questions about every record |
| recognize | `-h`, `--help` | 0 | Find every name in a text and assign one of the given kinds | same |
| relate | `-h`, `--help` | 0 | Find named relations across one complete entity set | same |

`diff sdlc/records/0082-help-before.txt sdlc/records/0082-help-after.txt` changes 150 lines. Each belongs to one authorized row:

- Row 1: the `decide` teaching sentence, `did not reach the cut`, `A broken run then never permits anything`, and `--threshold` `whose middle answers not sure`. In `choose`: `Exit 0 is an option and exit 3 is not sure.`, the `--raw` line `nothing when not sure`, and the `case` example, which now reads `pick=not_sure` and tells a not sure pick from a broken run.
- Row 6: `choose` says option for its pick in eight lines and `text` for `document`. `find` says line or record in eleven lines. `filter` says `CSV and TSV records`. The shared `--details`, `--field`, and `--jobs` text in nine commands says `each answer`, `a single text`, and `a single text` in place of `a row`, `one document`, and `one document`.
- Row 12: the `recognize` record-run sentence and the `relate` whole-set sentence.
- Row 43: the `score` measurement sentence says `the first System One model` and `rubric scores`.
- Row 45: the `annotate` introduction in `-h`, `--help`, and the root row, and its argument line `the named questions to ask`.

## Audit of the 45 items

Each row was observed on this branch's binary or owning file. Dispositions use exactly `already fixed`, `stale`, `fixed here`, or `moved`.

| Item | Disposition | Proof |
|---|---|---|
| 1 | fixed here | README.md line 21 said `an unresolved answer, and an error`; it now says `a not sure answer, and a broken run`, pinned in `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording`. `decide_edge::record_capable_help_pins_run_exit_behavior` pins the teaching sentence once and no `3 for unresolved`; `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` finds the definition only in `decide.md`; the vocabulary check rejects `unresolved` in built help |
| 2 | already fixed | `version::help_opens_with_the_semantic_commands_introduction`; `spec/version.md` |
| 3 | already fixed | `version::each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order` pins all ten; `version::recognize_and_relate_keep_the_beta_warning_the_cuts_and_the_disclosure` |
| 4 | already fixed | The exact root inventory assertion already existed on main in `version::each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order` (`ORDER`, 13 entries, compared whole). The ticket's `fixed here (pin only)` is corrected; nothing was added |
| 5 | already fixed | `tag_edge::tag_and_annotate_help_leads_with_the_description_and_ends_with_examples` |
| 6 | fixed here | The vocabulary check in `sdlc/scripts/demos` and its three cases in `sdlc/scripts/demos-self-test`; `find_edge::help_leads_with_whole_set_disclosure_and_all_three_bounds`; `choose_and_score_edge::the_help_of_each_verb_carries_the_advice_its_page_names` |
| 7 | already fixed | `decide_edge::the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest` (`It defaults to 0.5`); `spec/decide.md` |
| 8 | already fixed | Same test (`[default: 4]`); `spec/decide.md` |
| 9 | already fixed | Same test (`--url` in short help, advanced options only in long help) |
| 10 | already fixed | `find --help` prints `` `find --none` prints nothing and exits 3 when `none` wins or ties for first. ``; pinned in `find_edge::help_leads_with_whole_set_disclosure_and_all_three_bounds` |
| 11 | already fixed | `echo hi \| thinkthen decide q --timeout 0` printed `thinkthen: --timeout takes a whole number of seconds greater than zero` and exited 2 |
| 12 | fixed here | `decide_edge::record_capable_help_pins_run_exit_behavior` counts the record-run sentence once in eight commands, none in `relate`, and the whole-set sentence once in `relate` |
| 13 | already fixed | `decide_edge::the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest` (`no or not sure`) |
| 14 | already fixed | `decide_edge::shared_help_defers_order_and_document_rules_to_each_command` (`most likely yes first`, `An exact tie keeps input order.`) |
| 15 | already fixed | `filter --help` and `rank --help` hold no `without --jsonl` sentence (`grep -i without` finds only the record-run sentence) |
| 16 | already fixed | `thinkthen decide q --lines --input /tmp` printed `` thinkthen: `--input` names a directory, and a directory is not an input file `` alone and exited 5 |
| 17 | already fixed | `printf '\377\n' \| thinkthen decide q --lines` printed `thinkthen: the record is not valid UTF-8`; `find_edge::a_named_recording_appears_in_a_find_preflight_stop` |
| 18 | already fixed | The same run printed `thinkthen: stopped at record 1; 0 records finished` with no recording clause; `backend::from_record` asserts no `from a recording` |
| 19 | already fixed | `backend::tag::matrix` (`is not a systemone response`); `backend::recordings` (`the file is not a recording entry`) |
| 20 | already fixed | `backend::table` pins `the CSV record has 1 field; its header has 2` |
| 21 | already fixed | `thinkthen decide q --record FILE` printed `thinkthen: the recording directory is a file; choose another path or remove the file` and exited 5; `backend::recordings` |
| 22 | already fixed | `specification/annotate.md` states the question-name rule; a set with key `a.b` printed `` thinkthen: `questions.a.b` uses lowercase letters, digits, and underscores, and is not empty `` |
| 23 | already fixed | `printf '  ' \| thinkthen decide q --dry-run` printed `thinkthen: the evidence is empty or blank` and exited 2; `backend::state` |
| 24 | already fixed | A question file with `extra` printed `` thinkthen: a question file holds no key `extra` `` |
| 25 | already fixed | A set without the wrapper printed `` thinkthen: the question set is missing its `questions` object `` |
| 26 | already fixed | `backend::exchange` pins the fixed status-400 phrase `the backend refused the request; check --model` |
| 27 | stale | `sdlc/tickets/0058-make-diagnostics-actionable-and-safe.md` declines printing a backend's refusal body and excludes recording refused exchanges under the secrecy rule |
| 28 | already fixed | `backend::resend` pins `thinkthen: the backend timed out; increase --timeout or try again` |
| 29 | already fixed | `backend::distribution_total` pins `tolerance 0.01` |
| 30 | already fixed | `echo hi \| thinkthen decide q --jobs 1 --dry-run` printed `thinkthen: --jobs bounds the requests in flight, and one document sends one request` and exited 2; `backend::timeout` |
| 31 | already fixed | `specification/records.md` no longer calls 6 unused; `specification/annotate.md` gives exit 6 for a finished run with failed questions |
| 32 | already fixed | `specification/annotate.md` says an empty document is a usage error and an empty line or JSONL stream prints nothing |
| 33 | already fixed | `specification/filter.md` and `rank.md` qualify the empty stream to line and JSONL |
| 34 | already fixed | `specification/threshold.md` says an exact tie in `choose` is unresolved with or without a threshold |
| 35 | already fixed | `specification/result.md` holds no `rank` row with `"value":true`; `specification/tag.md` says its excerpt omits `schema` and `meta` |
| 36 | already fixed | `specification/result.md` says `value` is the cut's boolean for `filter --details` |
| 37 | already fixed | `thinkthen rank @q.json --lines --dry-run` printed `"from":{"question":"file","true":"default","false":"default","on":"default","model":"default"}` with no `threshold`; `specification/channels.md` says `from` names only settings the verb takes |
| 38 | already fixed | `specification/result.md` says `question.verb` is `decide` for `filter` and `rank` |
| 39 | fixed here | Contradicts the ticket's `already fixed`. `demos/README.md` listed eight built functions and left out `recognize` and `relate`, whose how-tos are green. `a7987762` replaces the list with `All ten functions are built.`; `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` pins it |
| 40 | already fixed | `README.md` carries no earlier-grammar clause |
| 41 | fixed here | Contradicts the ticket's `already fixed`. `specification/README.md` said `the four answer kinds` while `result.md` has five. `3ef97c06` changes the word; `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` pins it. The index already lists ten functions |
| 42 | already fixed | No page says `same input, same output`; demo 44's `deterministic` describes a replay |
| 43 | fixed here | `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` scans `Cargo.toml` and `specification/`; the vocabulary check scans README, the index, green how-tos, and built help |
| 44 | moved | Ticket 0090 renames `calibrated` to `tuned_for` |
| 45 | fixed here | `version::annotate_opens_with_the_saved_question_set_sentence_and_keeps_each_part_once`; `version::each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order`; `tag_edge::tag_and_annotate_help_leads_with_the_description_and_ends_with_examples` |

## Red then green

Every test below was written first and ran red on the baseline binary for the stated old text, then green after the change.

| Test | Red (observed) |
|---|---|
| `choose_and_score_edge::the_help_of_each_verb_carries_the_advice_its_page_names` | `Exit 0 is an option and exit 3 is not sure.` missing |
| `decide_edge::record_capable_help_pins_run_exit_behavior` | recognize: left 0, right 1 |
| `decide_edge::shared_help_defers_order_and_document_rules_to_each_command` | `On a command that accepts a single text` missing |
| `find_edge::help_leads_with_whole_set_disclosure_and_all_three_bounds` | `Every line or record leaves together and sees every other one` missing |
| `tag_edge::tag_and_annotate_help_leads_with_the_description_and_ends_with_examples` | `annotate -h does not open with its description: Fill out a question set for every record` |
| `version::each_judgment_help_opens_with_its_operation_and_root_lists_them_in_order` | `annotate -h opens with "Fill out a question set for every record"` |
| `version::annotate_opens_with_the_saved_question_set_sentence_and_keeps_each_part_once` | the root row lacked the new sentence |
| `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` | `Cargo.toml says decider model`; then red on `the five answer kinds` until row 41 was fixed, and on `All ten functions are built.` until row 39 was fixed |
| `demos` vocabulary check | 93 hits, exit 1 |
| `demos-self-test` | `standard-ok`, `rule3-other-language`, and `vocabulary-sanctioned` exited 1 on the real README's `rating` and `judgment`; the two planted-failure cases already passed |

After the code review, these ran red first:

| Test | Red (observed) |
|---|---|
| `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` | the README four-outcome line was missing, then `showed rubric scores rejecting` was missing from `score.md` |
| `backend::timeout::a_one_document_run_refuses_jobs_and_sends_nothing` | left `... and one document sends one request`, right `... and a single text sends one request` |
| `demos-self-test` `vocabulary-no-help` | exited 0 with the stand-in's `relate --help` exiting 2 |
| `demos-self-test` `vocabulary-spelling` | exited 0 on a planted `judgement` |

`version::recognize_and_relate_keep_the_beta_warning_the_cuts_and_the_disclosure` guards preserved text and was green before and after.

## Deviations from the ticket

- The ticket's premise that the README carries no four-outcome sentence was false. README.md line 21 taught `an unresolved answer, and an error`. The first build missed it and the code review caught it (B1). The line now uses not sure and broken.
- The code review (B2) found the help scan piped into awk without pipefail, so a failing help command passed as clean. The scan now captures the help first and fails with `demos: thinkthen ARGS failed, so its help was not checked`. A self-test case proves it.
- On the coordinator's instruction after the code review, three follow-ups landed here. The `--jobs` refusal says `a single text` (`cli/failure.rs`, a fourth production file), pinned at its exact exit code and sentence with and without `--dry-run`, and a counted loopback listener sees zero connections and zero requests (`backend::timeout::a_one_document_run_refuses_jobs_and_sends_nothing`). The check also rejects the spelling `judgement`. `specification/score.md` says `rubric scores`. Follow-ups 2 and 5 are filed as issues on main.
- Rust tests touch 6 files, one over the cap of 5, because the diagnostic pin lives in `tests/backend/timeout.rs`.
- Row 4 is `already fixed`. Main already held the exact inventory assertion, so no pin was added.
- Row 41 is `fixed here`. The audit found `four answer kinds` still live in `specification/README.md`. One word changed, in scope as result documentation.
- Row 39 is `fixed here`. The how-to index named eight built functions of ten. One sentence replaced the list, in a file row 6 already touched.
- The one-document sanction is not needed. The Budgets section names `document` at `args.rs` 79, 154, and 341 as row 6 text, so all three and `command.rs` now say `a single text`. No `document` remains in built help.
- The check sanctions `LABEL=DESCRIPTION` in `choose` help. The placeholder is shared with `specification/choose.md`, `question-file.md`, and the `--option` refusal. Renaming it would change a diagnostic and clap's own usage errors, which the ticket excludes.
- `tag` label and saved detailed `rows` pass by scope and need no phrase. `label` is checked only in `choose` help, and context words are checked only in built help. `failed question` passes because no rule names `failed`.
- `recognize` and `relate` help hold no cost sentence. The pin keeps the beta warning, the cut text, and the `relate` whole-set disclosure. Adding a cost sentence would be new wording, which row 3 forbids.
- `decide`'s record-run sentence sits in its first paragraph, so the root row and `decide -h` carry it. This was already on main and is pinned by `record_capable_help_pins_run_exit_behavior`. This ticket does not move it.

## Budget

- Production Rust: 4 files (`cli/args.rs`, `cli/args/command.rs`, `cli/args/find.rs`, `cli/failure.rs`), 59 lines added, +8 net nonblank. Cap: 4 files and 60 lines.
- Rust tests: 6 files (`version.rs`, `decide_edge.rs`, `tag_edge.rs`, `find_edge.rs`, `choose_and_score_edge.rs`, `backend/timeout.rs`), 146 lines added, +107 net nonblank. Cap: 5 files and 220 lines; the sixth file is a departure above.
- Vocabulary enforcement: `sdlc/scripts/demos` +64 and `demos-self-test` +40 nonblank, 104 in all. Cap: 140.
- Public prose: 16 files, 0 net nonblank lines. The files are `Cargo.toml`, `README.md`, `demos/README.md`, demos 02, 16, 17, 19, 28, and 40, and `specification/` `README.md`, `channels.md`, `choose.md`, `decide.md`, `result.md`, `score.md`, and `threshold.md`. Cap: 16 files and 120 lines.
- Ratchet: +115 over main, equal to the measured Rust increase. The first build measured +110; the review fixes add 5 (the README and score pins and the two-mode diagnostic test). The ceiling is 43897.

## Gates

The first lint run at `48a57d84` failed in clippy: `expect_used` on the shared `help` helper in `tests/version.rs`, which is not a test function. `3358001c` makes it return an `io::Result`. Then 0090 landed on main, and `05223ae1` merged main. The merged binary prints the same 21 captures as `sdlc/records/0082-help-after.txt`.

At `05223ae1`, run one rung at a time with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset:

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint`: exit 0. Ratchet `crates 43892/43892`.
- `sdlc/scripts/test`: exit 0. 729 Rust tests passed and 0 failed. The script self-tests and `live-test: all cases passed`. The annotate global-queue test did not flake.
- `sdlc/scripts/spec`: exit 0. `demos-self-test: 19 cases pass` and `demos: 21 green, 0 red`, with zero vocabulary hits.
- `git diff --check`: exit 0 after the capture whitespace fix.

No live or paid command ran.
