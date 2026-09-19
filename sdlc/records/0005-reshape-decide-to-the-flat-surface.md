# Record 0005: Reshape `decide` to the flat surface

- Ticket: `sdlc/tickets/0005-reshape-decide-to-the-flat-surface.md`
- Branch: `ticket/0005-flat-decide`
- Landed: 2026-09-19

## What was built

`thinkthen decide QUESTION` answers one yes/no question about the evidence and carries the answer in the exit code on every run. Standard output holds `true`, `false`, or `null`. `--details` prints the result object of `specification/result.md` with `value`, `question.text`, `threshold`, and `meta.profile`. Nothing about the wire format, the HTTP edge, recording, or replay changed.

The core gained one module.

- `threshold.rs`: `Threshold`, `Outcome`, and `ThresholdError`. `Threshold` is a struct over a private `Rule`, so the only way in from outside the crate is the checked parse, and `cut` and `band` are crate-private. It parses `T` as a cut and `LOW:HIGH` as a band through `FromStr`, applies the inclusive rule, and defaults to the cut of one half. It serializes as a number for a cut and as the string `"LOW:HIGH"` for a band, and its `Display` writes what `--threshold` takes back. `judge` reads one answer and gives `Yes`, `No`, or `Unresolved`, and `Outcome::value` gives the bare value that outcome prints.

The core lost three modules and two names.

- `pass_mark.rs`, `policy.rs`, and `assessment.rs` are deleted. The symmetric pass mark, the `Policy` enum, and the `unassessed` outcome are gone with them.
- `Condition` is `QuestionText` and `BackendName` is `ProfileName`, so the types carry the words the pages use.
- `Question::new_if` is `Question::new_decide`, and the verb on the wire report is `decide`.
- `resolve_backend` takes one source. The five `THINKTHEN_*` backend variables are removed with the empty-variable rule that served them, so the resolver no longer tracks which source a value came from. That table from record 0004 is gone, and `BackendError::NameWithUrl` is now the plain rule: a profile name beside a URL is a usage error.
- `Meta.backend` is `meta.profile`, and the plan document's `backend` is `profile`.

The binary changed at every edge.

- `args.rs`: `Command::Decide(DecideArguments)` is flat. The everyday options are `--threshold`, `--quiet`, `--details`, `--dry-run`, and `--profile`. The advanced ones carry `hide_short_help`, so `-h` shows the everyday set and `--help` adds the rest. The long help warns about `set -e` and says that a single cut never means the model is sure of no.
- `decide.rs`: `--plan` is `--dry-run`, and the view chooses between the bare value, the object, and nothing at all after the answer is in hand, so no view changes a request byte.
- `edge.rs`: `Environment` reads one variable, the hidden test retry wait.
- `failure.rs`: `Mark` is `Threshold` with the message `--threshold: {error}`, `PlanWithRecording` is `DryRunWithRecording`, and `QuietWithDetails` is new.

`spec/decide-if.md` is `spec/decide.md`, rewritten for the new surface. The `if-urgent` fixtures are `decide-urgent`. The three demo-runner fixture pages are written to the new surface with their evidence, their question, and their model unchanged, so the recording under `01-replay-gate/` answers them as it did.

## Red then green

Each rule below was watched failing at the code before the code was right.

- `the_worked_boundaries_follow_the_specification`: making the cut exclusive failed it with `probability 0.5 under Cut(0.5)`, `left: No`, `right: Yes`. Making the band's low side exclusive failed it with `probability 0.1 under Band { low: 0.1, high: 0.9 }`, `left: Unresolved`, `right: No`.
- `every_answer_prints_its_bare_value_and_earns_its_own_exit_code` and the rest of the integration suite were watched failing against the landed binary before the arguments were reshaped, with `error: unexpected argument 'asks for a refund' found`.
- `the_two_options_over_one_folder_are_a_cache_that_calls_once` failed at `run 0: true` while it still read `"replayed":false` out of a bare value, which is the view change the ticket asked for.

## How each acceptance bullet is proven

| Bullet | Test |
| --- | --- |
| The rule at p of 0, 0.1, 0.5, 0.9, and 1 under no threshold, `0.9`, and `0.1:0.9` | `threshold::tests::the_worked_boundaries_follow_the_specification` |
| The rule holds for any p and any valid threshold | `threshold::tests::a_cut_answers_yes_at_or_above_the_mark_and_no_below_it`, `a_band_answers_yes_above_it_no_below_it_and_nothing_inside_it`, `a_printed_threshold_parses_back_to_the_same_rule` |
| The three exit codes with their bare values | `decide_exchange::every_answer_prints_its_bare_value_and_earns_its_own_exit_code`, `a_single_cut_answers_yes_or_no_and_never_leaves_a_run_unresolved` |
| `--quiet` prints nothing with the same exit code | `decide_exchange::quiet_prints_nothing_and_keeps_the_exit_code_the_answer_earned` |
| `--details` sends the same request bytes as the bare run | `decide_exchange::details_prints_the_result_object_and_sends_the_bytes_the_bare_run_sends` |
| `--dry-run` prints six fields with `profile` and opens no connection | `decide_edge::the_plan_prints_six_fields_and_opens_no_connection`, `the_plan_of_a_named_profile_names_its_key_variable_and_needs_no_key` |
| Each removed option exits 2 | `decide_edge::every_option_the_old_surface_carried_is_a_usage_error` |
| The removed environment variables change no plan | `decide_edge::the_backend_environment_variables_are_gone_and_change_no_plan` |
| The recording digest equals the pinned value from ticket 0004 | `recording::tests::the_digest_of_the_fixture_request_is_the_name_the_entry_keeps` |
| The key and the evidence never reach an error or a `Debug` line | `decide_edge::no_diagnostic_ever_carries_the_key_or_the_evidence`, `decide_exchange::a_named_key_variable_is_sent_as_a_bearer_token_and_never_printed`, `http::tests::an_exchange_shows_neither_the_key_nor_the_evidence_it_carries`, `record_and_replay::a_recorded_exchange_replays_with_no_listener_and_no_key` |
| The ratchet equals the measured total | `sdlc/scripts/ratchet.mjs`, run by `lint` |
| Demos 01 and 09 parse against the new binary | By hand. See below |
| The whole ladder is green | The table at the end |

`spec/decide.md` holds nineteen more examples of the same surface, and the `spec` rung runs them against the compiled binary.

## The review leftovers

Four were touched by this rework and are fixed.

1. **The three decode errors carried a wire name as a `String`.** They now carry the place number, and `thiserror` renders it through `wire_name`, so the message is the same text and nothing parses it back. `each_refused_response_names_its_own_cause` asserts the rendered `` `q1` ``.
2. **Public items with no caller outside the crate.** `Threshold::cut` and `Threshold::band` had no outside caller and are now crate-private, and the enum they built is private too, so no caller outside can write a rule past the range check. Every other export is reachable from a public signature the binary uses: `Answer` and `Usage` through `Reply`, `UnknownAdapterError` through `BackendError` and `Adapter::from_str`, `EmptyPlanError` through `Plan::new`, and `ProfileName` through `Backend::profile` and `Meta::new`. Making any of them crate-private would put a private type in a public signature. The leftover is answered, and nothing moved.
6. **`Probability` and `PassMark` repeated a newtype shape.** `PassMark` is deleted, so `Probability` stands alone and the repetition is gone.
10. **The demo runner split a harvested `--replay` folder name on white space.** It now reads one name a line at a time and fails the rung when a name inside the loop finds no folder.
12. **`record_and_replay.rs` repeated one record call five times, and the wrong-assertion fixture copied a whole demo page.** Four of the five calls now go through one `recorded` helper that gives back the listener, the entry name, and the entry text. The wrong-assertion fixture is one block with one wrong exit code.
13. **`backend.rs` was 411 lines and its first documentation line joined two jobs.** It is 344 lines with the environment source gone, and the line reads "The backend a request goes to."

The rest still wait, and none of them is in this ticket's path.

- 3, 4, 5: `Reply` keeps its names, `encode` keeps its owned strings, and `EncodeError` stays because deleting it means an `unwrap` the lint table bans. Each has the same cost it had.
- 7, 8: standard input has no size cap and an oversized or cut-short body is retried like a transport failure. Both live in `http.rs` and `edge.rs`, which this ticket did not open, and no settled page sets a rule for either.
- 9: a defect and a render failure still share exit code 70, which `channels.md` gives them.
- 11: `EntryError::Unwritable` cannot fire and `EntryError::Schema` echoes a hand-edited file's own schema string. `recording.rs` is untouched but for one test's question name.

## What the specification got wrong or left unclear

- **A threshold has no null form on `decide`.** `result.md` says `threshold` is `null` when none applies, and `threshold.md` says `--threshold 0.5` and no threshold at all name the same rule. On `decide` a rule always exists, so the field is always a number or a band string. The reading closest to ADR 0007 is that `null` is there for the verbs that take no threshold. Ian can overturn it.
- **What refuses an empty side.** `threshold.md` names an empty side as a usage error and does not say which error. `--threshold 0.1:` and `--threshold :0.9` both fail as "a threshold is a decimal fraction, or two of them as LOW:HIGH", because an empty side is not a number. `--threshold 0.1:0.5:0.9` fails the same way.
- **What a band's own bounds are.** ADR 0007 gives `0 ≤ LOW < HIGH ≤ 1` and gives the cut `0 < T ≤ 1`. A cut of 0 is therefore refused and a band low of 0 is accepted, which reads odd side by side and is what the table says. Both are implemented as written.
- **Where the short help ends and the long help begins.** `channels.md` names the two sets and does not say which flag shows which. `-h` prints the everyday set and `--help` prints everything, which is clap's own convention.
- **`--dry-run` beside a view option.** No page said what happens. The steering agent ruled on 2026-09-19 that `--dry-run` may be added to any command line that is valid without it, and that it prints the plan whatever view option stands beside it. `specification/channels.md` says so beside the `--quiet` rule, and `a_dry_run_prints_the_plan_whichever_view_option_stands_beside_it` holds it. Ian can overturn this.
- **The `--quiet` and `--details` message.** ADR 0007 makes the pair a usage error and gives no message. It is `--quiet prints nothing, so it does not take --details`.
- **`meta.tool` and `confidence` are Draft** and are not built, as the ticket says.

## Demos 01 and 09

Checked by hand against the built binary, line by line. Neither page was changed, and both stay `Status: red`.

- **Demo 01.** The page used `--input FILE` on every `decide` line, and `--input` belongs to `records.md`, which this ticket excludes. Each of those lines exited 2 with `unexpected argument '--input' found`. The page and `record.sh` now redirect standard input instead, and the bullet that argued for `--input` over a redirect now points at the records slice. Every `decide` line parses: `--threshold 0.1:0.9`, `--quiet`, `--details`, and `--replay recording/` are all accepted, and the run then fails at exit 5 naming the entry the missing `recording/` folder does not hold. The last block's `--quiet --details` needs no recording and exits 2, as the page expects. The two thresholds name the same entry as the run without one, which is the page's own claim that a threshold changes no request byte.
- **Demo 09.** The page holds no `thinkthen decide` line. Every command on it is `thinkthen filter`, which no ticket has built, so every line exits 2 with `unrecognized subcommand`. Nothing on it is this ticket's to answer.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Ninety tests, one documentation test, nineteen spec examples, and the demo runner over fifteen red demos pass. The largest file is `crates/thinkthen/tests/record_and_replay.rs` at 385 non-blank lines, against a ceiling of 500.

## The ratchet

The ceiling was 4080 and is 4068. The pass mark, the acceptance policy, the assessment, and the environment half of the resolver went out. The threshold, the flat arguments, and the exit-code tests came in. Duplication was looked for in the core and the binary before a line was added, and five repeated record calls in `record_and_replay.rs` were folded into one helper.

## Dependencies

None added, none removed.
