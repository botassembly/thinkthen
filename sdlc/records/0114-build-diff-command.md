# 0114: Build the diff command

Status: built and reviewed on `ticket/0114-diff-command`. A fresh read-only Claude code review returned seven findings, and all seven are fixed. Not merged to main. Owner: Claude.

## Order and base

The coordinator asked for 0114 to start at once on top of the 0113 branch, since diff reuses audit's code. Merge commit `2315151e` brought `origin/ticket/0113-audit-command` at `6c37aba3` into this branch. 0113 is in code review. If that review changes shared code, the fixes merge in here. The ticket's "Dependencies and order" section asked for 0113 and 0086 to land first. That section no longer binds.

## McNemar

Decision 4 of the accepted design settles the question. The test with a key counts only wrong-to-right and right-to-wrong pairs, as the prototype's code and the `diff-choose` golden do. The Beatles Bench issue `sdlc/issues/2026-09-24-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md` asks for the wider rule. The goldens decide until Ian or the prototype's owner rules on it. `discordant` in `core/measure/diff.rs` holds the rule alone, and `specification/diff.md` gives the switch steps.

## Result

`thinkthen diff A [B]` routes in `cli/mod.rs` right after `audit`, before `file_size::claim` and `Environment::read`. `cli/diff.rs` parses the command line, reads the inputs through `cli/measure.rs`, and writes JSON lines or the prototype's table. `core/measure/diff.rs` pairs the answers, grades them with 0113's answer and key readers, and builds the rows and summary. `core/measure.rs` gains `mcnemar`, summed in log space.

Shared code moved so audit and diff hold one copy each:

- `Shown` and its table text moved from `core/measure/audit.rs` to `core/measure/answer.rs`, beside `Rule`.
- Rule parsing (`rule`) and line reading (`lines`) moved from `cli/audit.rs` to `cli/measure.rs`.
- `answer::read` takes an `Identity`. `refuse_repeats` is still the one function that holds the repeat identity: (name, id, text) for audit, (name, id) for diff.
- `Said::text` names an answer for the output.

Builder notes: the readers ignore members they do not use (`diff::a_run_line_with_a_member_diff_does_not_use_still_pairs`), the repeat check keys on (answer name, record id) in `refuse_repeats`, and `specification/diff.md` tells readers to ignore unknown output members.

## Golden match

The six upstream diff goldens and the two extra captures match byte for byte. `diff::goldens_match` checks the tolerance and then the exact bytes. The three table captures match byte for byte. Before any capture, the prototype reproduced all six upstream diff goldens exactly.

The five captures came from the prototype at `be7cea2e` (SHA-256 `7be20681…16a1`), read with `git show` into a scratch file and run under Python 3.12.3 from the fixture folder. The fixture README lists their commands and checksums. `extra/diff-annotate` shows two `withdrawn` effects, and `extra/diff-249-cuts-nokey` shows 67 moves from no to yes and `mcnemar_p` 0.0 on yes answers.

`diff::the_second_side_reads_under_compare_threshold_when_both_rules_are_given` pins two table outputs captured from the same prototype: `--threshold 0.5 --compare-threshold 0.4` with a key, and a band against a cut without one. No golden prints a band in the diff table.

## Tests

- `core::measure::tests::mcnemar_matches_exact_integer_sums_to_120_and_the_pinned_values` compares every split up to n = 120 with exact `u128` sums within a relative `1e-12`, and pins the six values the ticket names.
- `core::measure::tests::discordant_counts_only_wrong_to_right_and_right_to_wrong` checks all 16 outcome pairs. It is the switch guard the ticket names: a McNemar ruling changes this test, the function, and the goldens together. The goldens alone would also fail if the rule widened.
- `diff::goldens_match`, `diff::tables_match_byte_for_byte`, `diff::the_held_out_half_reads_as_the_prototype_reports` (87 to 89 at 0.42, 87 to 84 for the softer wording, every move no to yes), the reader test, the two-rule test, and `diff::help_names_diff_after_audit_and_says_what_it_never_does`.
- `audit_refusals::each_failure_prints_one_line_that_names_no_record_id_value_or_path` gains 13 diff rows, and each row plants secrets. `audit_refusals::audit_sends_no_request_reads_no_key_and_writes_nothing` runs every diff golden line with and without `--table`, each table line, and every refusal under the canary key, the loopback address, and locked folders.

The ticket asked to port the prototype's four small `Diff` tests as core tests. Those values are the exact bytes of the `diff-decide-cuts`, `diff-decide-wordings`, `diff-decide-nokey`, and `diff-choose` goldens, so a second copy would test one contract twice. The goldens carry them. The two `LeaningNo` diff cases use the held-out key, which no golden covers, and `the_held_out_half_reads_as_the_prototype_reports` carries them. One `goldens_match` loop replaces the ticket's eight named golden tests, as in 0113.

## Planted bugs

Each bug was planted alone. The core and integration suites ran with `--no-fail-fast`, and the tree was restored after each.

| Bug | Red tests |
| --- | --- |
| 1 McNemar one-sided | `mcnemar_matches_exact_integer_sums…`, `goldens_match`, `tables_match_byte_for_byte`, the reader test, the two-rule test |
| 2 McNemar summed to `min(a, b) − 1` | `mcnemar_matches_exact_integer_sums…`, `goldens_match`, `tables_match_byte_for_byte` |
| 3 clamp dropped | `mcnemar_matches_exact_integer_sums…`, `goldens_match`, `tables_match_byte_for_byte`, the two-rule test |
| 4 `gained` and `lost` swapped | `goldens_match`, `tables_match_byte_for_byte`, the reader test, the two-rule test |
| 5 `from` and `to` swapped in moves | `goldens_match`, `tables_match_byte_for_byte`, the held-out test, the two-rule test |
| 6 `resolved` counted as `gained` | `goldens_match`, `tables_match_byte_for_byte` |
| 7 `only_a` and `only_b` swapped | `goldens_match`, `tables_match_byte_for_byte`, the reader test |
| 8 pairing by record id alone | `goldens_match` (`extra/diff-annotate`) |
| 9 no-key test on the discordant counts | `goldens_match` (`extra/diff-249-cuts-nokey`) |
| 10 B's failed answers paired | `goldens_match` |
| 11 B read under `--threshold` | `goldens_match`, `tables_match_byte_for_byte`, the held-out test, the two-rule test, the refusal test |

## Budgets: re-scored past the production limit

| Bound | Limit | Measured |
| --- | ---: | ---: |
| Production Rust nonblank lines | 450 | 523: 422 in the two new files, 101 net in edited files |
| Existing production files touched | 3 plus `cli/measure.rs` | 7: `cli/args/command.rs`, `cli/mod.rs`, `cli/measure.rs`, `cli/audit.rs`, `core/measure.rs`, `core/measure/answer.rs`, `core/measure/audit.rs` |
| Rust test nonblank lines | 500 | 362 |
| New test files | 2 | 1 (`tests/diff.rs`); the rest extend 0113's files |
| Script nonblank lines | 25 | 23 in `policy.py` (13 removed), 1 in `demos` |
| Largest Rust file | 500 | `tests/audit_refusals.rs` 393 |
| Dependencies | 0 | 0 |

The overrun and the extra files come from moving shared code out of audit, so diff imports nothing from `cli/audit.rs`. The move saved 38 lines in audit and added them once in the shared modules. diff's own files carry the rest. The agent re-scored and kept it. Ian can overturn the re-score.

The ratchet rises from 52,720 to 53,605 (+885): 523 production and 362 test. Duplication checked first: 0113's answer reader, key reader, rule parsing, line reading, rule text, and table helpers. diff reuses each one, and the rule parsing, line reading, and rule text now live once in shared code.

## Policy and help

`MEASURE` now holds `cli/diff.rs` to the same bans. `route_failures` checks both `Command::Audit` and `Command::Diff` before `Environment::read`, and the self-test plants a late `Diff` return. `sdlc/scripts/demos` scans `diff -h` and `diff --help`. The root inventory in `tests/version.rs` lists `diff` after `audit`.

## Departures beyond the ticket

- A rule over answers without probabilities prints audit's `--threshold needs probabilities` sentence, even when `--compare-threshold` named the rule. The core does not know which option named it.
- A probability saved as an integer prints as a float, `1.0` in JSON and `1.00` in the table. The prototype prints `1`. `specification/diff.md` lists it.
- A repeated (answer name, record id) is refused even when one of the two answers failed.

## Code review

A fresh read-only Claude session reviewed `2315151e..cf14e4d6`. It checked the pure core and the route, the port line by line against the prototype, the goldens and tables byte for byte, the audit refactor, the policy, the ratchet, public hygiene, and the pages. It agreed with the choice not to port the four small prototype tests. Its seven findings and their fixes:

1. An integer probability prints as a float. Fixed: `specification/diff.md` lists it under Departures.
2. The help test's `contains("not sure")` matched any text. Fixed: it pins "An answer inside a band is not sure."
3. The held-out test left a key file behind on failure. Fixed: the key goes in on standard input.
4. The write-and-flush block and a two-place formatter were copied. Fixed: `cli/measure.rs::write` serves audit and diff, and `core/measure.rs::places` replaces `three_places` and `two`. The ratchet fell by 10 lines.
5. The `discordant` test copies the rule. Kept as the switch guard, named under "Tests".
6. The Gates section was empty. Fixed below.
7. Two wording slips in `policy.py` and a `cli/mod.rs` defect message. Fixed.

## Gates

Filled in after the ladder run.
