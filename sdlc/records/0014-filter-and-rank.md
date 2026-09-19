# 0014: `filter` and `rank`

Branch `ticket/0014-filter-rank`. Built 2026-09-19.

## What landed

Two verbs entered the surface. `filter QUESTION (--lines|--jsonl)` asks one yes/no question of each record and prints the records that reach the cut, byte for byte, in input order. `rank QUESTION (--lines|--jsonl)` asks the same question of each record, sorts by the probability of yes, and prints the records highest first, with exact ties in input order. `--top N` slices the order after it exists.

Neither verb added a way to read, send, or record. Both run over the record loop of tickets 0012 and 0013, both take `@FILE`, `--true`, `--false`, `--field`, `--input`, `--jobs`, `--record`, `--replay`, `--cache`, `--dry-run`, and `--details`, and both send the `decide` request. A recording made by `decide` replays under either verb.

How-tos 03, 06, and 43 are green over sixteen live exchanges. Page 09 is deleted, because page 03 now carries its `--dry-run` proof of what leaves the machine.

## What went in the core

ADR 0017 asks that the cut, the sort, the tiebreak, and the slice be pure functions, so a library in another language is held to the same cases.

`crates/thinkthen-core/src/order.rs` is the sort, the tiebreak, and the slice in one function, `ranking(of: &[f64], top: Option<usize>) -> Vec<usize>`. It returns places, never rows, so it knows nothing of records. Two `proptest` properties hold it: the result is a permutation of the input places or a prefix of one, and the probabilities never rise along the order while exact ties keep input order.

`Answer::yes()` returns the probability of a yes/no answer and `None` for the other two shapes, so the binary never reaches into `Shape`.

The cut already existed. `Passed` in `judgment.rs` does it, and `filter` calls it, so nothing was written twice.

The refusals went in the core too, because the sentence table, the `named()` prefix that says which home a value came from, and the `Source` that picks the exit code all live there. `Cutting` on the resolver names what a verb allows: `AsTheVerbAllows` for `decide`, `OneCut` for `filter`, `NoRule` for `rank`. `threshold_of` refuses `BandOnFilter` and `RuleOnRank` from it, so the same sentence comes out of the command line at exit 2 and out of a question file at exit 5.

## What went in the binary

`Output` in `schedule.rs` is the sink the scheduler writes through. `Streaming` writes each row as it settles. `Ordered` holds the rows, and on a clean end it calls `ranking` and writes the order. A held run that stops prints nothing, so its line on standard error says ", and nothing was printed because an order needs every record". The enum was chosen over a trait because `rank` needs no second scheduler and the stop line needs to know the rows were held.

`judge.rs` reached 627 non-blank lines and the ceiling is 500. The split is `judge.rs` (which verb, what it keeps, what view) and `asking.rs` (one record to one request, one reply to one row). The seam allowance in `sdlc/scripts/policy.py` moved with the vendor words.

The ratchet went from 11563 to 12953 on the branch, and to 14823 after main was merged twice and the review was answered. What grew: `order.rs` and its properties, the two verbs and their argument groups, the sink, the four refusal sentences, and about 700 lines of integration tests, which are the larger half. Duplication was looked for first in three places and deleted in two. `Record::as_it_arrived` is now one method, and `record()` calls it instead of repeating the line-ending rule. `yes_no` in `asked.rs` is one settler for all three verbs. `over_kept` is one path for `filter` and `rank`, which earned an `#[expect(clippy::too_many_arguments)]` with a reason, because splitting the call would split the flow.

## Red then green

- `error[E0432]: unresolved import super::ranking`, then `error[E0599]: no method named yes found for struct Answer`. Both are the intended first red.
- `error[E0004]: non-exhaustive patterns: &Some(args::Command::Filter(_))` on the dispatch.
- `--top 0` under clap's ranged parser printed "0 is not in 1..18446744073709551615", which names a machine's limit and no rule a person has. The parser was widened and `Failure::TopIsZero` says it in the tool's own words.
- `size: crates/thinkthen/src/judge.rs has 627 non-blank lines and the ceiling is 500`.
- `seam: crates/thinkthen/src/asking.rs names 'systemone' outside the adapter`, and the same run said `judge.rs` no longer names it.

## Acceptance, bullet by bullet

| Bullet | The test that proves it |
| --- | --- |
| Kept records unchanged byte for byte | `keeping::filter_prints_every_kept_record_byte_for_byte_and_in_input_order` and the odd-spacing and trailing-space cases |
| Order kept under `--jobs` | `keeping::every_number_of_jobs_prints_the_bytes_one_job_prints` |
| The band refused | `refusals::a_band_is_refused_by_filter_in_either_home_and_names_the_way_to_three_piles` |
| Each refused option with its message | `refusals::each_view_and_the_missing_framing_are_refused_in_the_tools_own_words` and `refusals::rank_refuses_a_rule_from_either_home_and_names_the_command_that_cuts` |
| `--top` bounds | `refusals::top_takes_a_whole_number_of_one_or_more_and_says_so` over `0`, `half`, and `-1` |
| Ties in input order | `keeping::an_exact_tie_keeps_input_order_and_top_prints_the_first_places` |
| The stop at a failed record | `keeping::a_failed_record_stops_filter_after_a_prefix_and_leaves_rank_printing_nothing` |
| The empty input | `keeping::an_empty_record_stream_prints_nothing_and_sends_nothing_for_either_command`, which counts the listener's requests |
| The plan under `--dry-run` | `keeping::the_plan_under_dry_run_shows_the_first_record_and_sends_nothing` |
| The two properties | `keeping::filter_prints_a_subsequence_of_its_input_and_rank_a_permutation_of_it`, and `order::proptest` in the core |
| One `decide` recording replays under both, no request sent | `refusals::a_recording_made_by_decide_replays_under_both_commands_with_no_request_sent`, which asserts `listener.requests().len() == 0` |
| The key and the evidence never appear | `refusals::no_message_from_either_command_ever_carries_the_key_or_a_record` over fourteen refused command lines, both channels, with `RUST_BACKTRACE=full` |

## The three pages

Page 03 filters an issue export down to the reports that give steps. Its first step is the `--dry-run` plan, which shows that `--field /body` sends the body and never the reporter's address, and a second block asserts the address is absent. That is page 09's argument, inside the page that uses it.

Page 06 ranks six made-up wiki hits against a fixed query carried in each record, with `--field` given twice. Its `--details` block shows `"threshold":null` on every row, which is the page's way of saying an order is a reading suggestion and not a claim about any page.

Page 43 is ADR 0018's design. `convention.json` holds the house rule, `hunks.jsonl` holds five made-up changed hunks, `filter` keeps the two that break the rule, and `lint.sh` exits 1 when any hunk came back. The first block is the whole thing in six lines and prints two file names. The one-liner that cuts the fixture out of a real diff is in the Input section, after the first result.

Every scenario is ordinary office or software work. No real person, company, or private project is named.

## Choices where the pages were silent

Each of these is Ian's to overturn.

- **The refusals live in the core, not the binary.** The alternative was a check in `judge.rs`. The core already owns the sentence table, the home prefix, and the exit code, and a check in the binary would have copied all three.
- **`Output` is an enum, not a trait.** A trait would have made a second scheduler possible and bought nothing the ticket asks for.
- **`--top 0` is refused by the tool and not by clap.** Clap's message names a range that goes to the width of a machine word.
- **The binary's properties are a deterministic loop, not `proptest`.** `proptest` is a dev-dependency of the core alone, and adding it to the binary is a dependency change no ticket authorized. The real property cases sit in `order.rs`, where the pure function is.
- **The refusals of the two record verbs sit in `tests/backend/refused.rs`, beside main's `refusals.rs`.** Ticket 0019 landed a table that drives every refusal over every verb from one row each, and the table walks `VERBS`, which is the three verbs that take operands and answer on one document. `filter` and `rank` take no operands, refuse the two views the table drives, and need a framing, so a row for them would have needed a second table inside the first. Each file's first lines name the other.
- **`Reading`'s tests moved to `crates/thinkthen-core/src/records/tests.rs`.** Ticket 0019 added the record size limit to the same function this ticket made reusable, and the merged file passed the 500-line ceiling. `question_file/tests.rs` is the same shape.
- **Page 43 took the `filter` seat in the README's front window, and page 03 left it.** ADR 0018 puts 43 in the window. Page 03 stays in the demo index and page 43 links back to it as the plain introduction.
- **`record.sh` on each page uses `--cache`, and page 43's `clean.jsonl` is a subset of `hunks.jsonl`.** A rerun of a recording script pays only for what is missing, and the clean fixture pays nothing.
- **Page 03's fixture changed, and the question did not.** ISS-104 first read "Reproduced four times" with no steps and scored 0.4, which is the right answer to the question asked. The made-up report now gives steps and scores 0.97.

## The live spend

Four live runs through `sdlc/scripts/live`: 1525 for page 03's first cut, 337 for its one changed record, 1901 for page 06, and 1981 for page 43. The total is 5744 input tokens against the cap of 40,000 the session set. `sdlc/live-tokens` moved from 415989 to 421733 against the limit of 476000000.

`apikey_`, `authorization`, and `bearer` each appear zero times in every committed recording. The two probe scripts that name the header and hold no key are the only files under `demos/` and `probes/` that match.

## The review

A second agent with fresh context read the ticket, the whole diff against `origin/main`, and "What reviewers keep finding". Its verdict was **not ready**, with nine blocking findings and six nits. Every one is answered below. Eight of the fifteen changed code or a page.

| Finding | What happened |
| --- | --- |
| The worktree sat in an unresolved merge | Finished. Main landed tickets 0019, 0020, and 0021 while this branch ran, and both merges are now in. |
| The ceiling is neither side of the conflict | Measured on the merged tree, twice. It is 14823. |
| `refusals.rs` collides with main's file and checks less | Partly stands. See the choice below. The file is `refused.rs` now, and it calls `nothing_leaked`, the reader main's sweep owns, so it reads the files a refused run wrote and both header names. |
| `rank --details` printed a `value` from a cut nobody named | Fixed. A ranked row carries `"value": null` beside `"threshold": null`. `specification/rank.md`, page 06, and the test all pin it. |
| Nothing rendered the new `Debug` lines | Fixed. `Output::Ordered` holding a row goes through `no_debug_line_shows_the_key_or_the_evidence`, and the five new `Failure` variants through `no_diagnostic_holds_the_key_or_the_evidence`. |
| `rank -h` did not state the method | Fixed. The first line of the help is now the method, so `-h` carries it. |
| Page 43 named a pathspec that is not on the page | Fixed. The one-liner cuts with `-- '*.rs' '*.ts'` and the bullet quotes it. |
| A test named "sends nothing" proved it with `--dry-run` | Fixed. It points at a listener, counts zero requests and zero connections, and is named for what it proves. |
| The README window is not ADR 0018's | Answered by the merge. Ticket 0021 rewrote the window, and this ticket moved only the rows of pages 43 and 06 inside it. |
| One case checked a substring | Fixed. It pins the whole line. |
| `specification/rank.md` contradicted itself on `--details` under `--top` | Fixed. The row reads "for each record it prints". |
| The `--details` test only counted lines | Fixed. It pins all four records, their order, each `value`, and the cut. |
| `--top 0` was outside the secrecy sweep | Fixed. `--top 0` and `--top half` are rows in it. |
| The two verbs ordered their refusals differently | Fixed. Both refuse a view that prints no record first, before a question file is opened. |
| Page 03 conflated two kinds of narrowing | Fixed. The bullet now says to cut the file before `filter` and drops the back-reference. |

The reviewer also read page 43 cold for thirty seconds. It understood the page: a rule no linter can express, asked of each changed hunk, with the breaking hunks kept and the build failed. It saw the result, a beat late, because the result is the argument to `mustmatch` rather than printed output, and the sentence under the block carried it. It wanted three things the page does not say: what `thinkthen` is, what `mustmatch` is, and whether running the page costs money. The third is now one clause in the Input section. The first two are the same on all twenty pages, so they belong to `demos/README.md` and not to one page.
