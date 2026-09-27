# 0165: Build diff reads recognize and relate

Status: landed 2026-09-27. Owner: Claude.

Branch `ticket/0147-recognize-adr`, built in lane 3 on top of ticket 0147. The ticket is `sdlc/tickets/0165-diff-reads-recognize-and-relate.md`. `origin/main` was at `e4b7bbee`, which the branch already held, so no merge was needed. No paid call ran. Ian can overturn every decision the ticket lists.

## Result

- `verb()` admits `recognize` and `relate` under diff. `Said::Items` holds a side's kept items under its own cut, in place of `Said::Unresolved`.
- A `recognize` or `relate` line with no `input` takes its one-based line number as its record id under diff too.
- `diff` gains `--match strict|overlap`, default `strict`. It exits 2 over other verbs.
- diff pairs A's kept items against B's, strongest first, one match each. The leftovers pair again for a changed kind at a matching place. Rows print `id`, `name`, `lost`, `gained`, `changed_kind` and `key`. Items print whole, in their line's output order.
- With a key, each side reads the key under its own question. McNemar runs on the key items matched on one side only. `mcnemar_on` is `key names` or `key edges`. Extras print beside the test and stay out of it.
- An item summary adds `key_items`, `items_gained`, `items_lost`, `items_changed_kind`, `extra_a` and `extra_b` after `mcnemar_p`. Summaries of other verbs do not print them, so every `decide` and `choose` golden holds byte for byte.
- Four new refusals: an item verb paired with another verb, item pairs beside other pairs, `recognize` pairs beside `relate` pairs, and `--match` over other verbs. A cut below the run cut and a band refuse with audit's sentence.
- The cannot-grade sentence under diff now ends `diff grades decide, choose, recognize and relate`. The diff `tag` row in `tests/audit_refusals.rs` expects it. That is the one expected change to an existing test.
- audit's key tally and diff's side-against-side pairing share one function, `items::pair`.
- Pages: `specification/diff.md`, `specification/settings.md`, `spec/diff.md`, diff's help text, the fixtures README and `CHANGELOG.md`.

## Tests

- Tests 1, 3 and 6 are `recognize_and_relate_rows_match_the_hand_worked_outputs` in `tests/diff.rs`. It pins ten outputs under `tests/fixtures/measure/golden/items/`, worked by hand before the code ran: two recognize runs with and without the key, under `strict` and `overlap`, their tables, the run with no kinds, the relate runs and their table, and a `decide` answer paired with a `choose` answer.
- Test 2 is `two_cuts_on_a_replayed_recognize_run_lose_the_names_between_them`. It diffs `diff-recognize-key-run.jsonl`, saved once from ticket 0147's replayed five-kind run, at 0.5 and 0.8. The summary gives 200 records, 52 changed and 64 names lost, and every lost `strength` falls in [0.5, 0.8). The fixtures README says how `jq` counts the same numbers from the file.
- Test 4 is `mcnemar_on_key_names_leaves_the_extras_out`. It pins `mcnemar_p` 0.145996 on 9 against 3, `extra_a` 2 and `extra_b` 0.
- Test 5 is `item_verbs_refuse_what_diff_cannot_compare`, with ten refusal rows. Each pins exit 2, the exact sentence and empty standard output.
- The edge-case table is `each_item_edge_row_gains_loses_or_changes_kind_as_the_ticket_says` in `src/core/measure/diff_tests.rs`, with 18 rows.

## Deliberate breaks

Each break was planted in the working tree, run against the diff, audit refusal and library tests, and reverted. The crate was clean after each revert.

| Break | Named check | Result |
| --- | --- | --- |
| Keep `verb()` refusing recognize and relate under diff | Test 1 | Red, with tests 2, 4, 5 and the edge table |
| Keep `Said::Unresolved` for recognize | Test 1 | Red, with tests 2, 4 and the edge table |
| Keep the old tail | Test 5 and the diff `tag` row | Red: both |
| Drop the line-number id for diff | Test 3 | Red |
| Ignore `--match` in diff's arguments | Test 1, the `--match overlap` run | Red, with test 5's `--match` row |
| Count a changed kind as a loss and a gain | Edge row `person` to `work` | Red at that row |
| Ignore `--match overlap` | Edge row `Abbey Road` under `overlap` | Red at that row |
| Pair many to one under `overlap` | Edge row, stronger B name second | Red at that row: no name gained |
| Put extras into McNemar | Test 4 | Red |
| Drop the `either` swap | Edge row, `either` edge both ways | Red at that row |
| Pair B names in output order | Edge row, stronger B name second | Red at that row: the stronger name gained |
| Keep a `relate` line with failed questions | Test 3 | Red |
| Allow recognize pairs beside decide pairs | Test 5 | Red |
| Check the key against side A's question only | Test 5 | Red, with test 1 |
| Print a row only for item changes | Test 1, record 3 under `overlap` | Red |
| Read B at A's cut | Test 2 | Red: `changed` 0 |
| Accept a cut below the run cut | Test 5 | Red, with the edge table's refusal row |
| Send `decide` pairs through the item path | The `decide` goldens | Red: `goldens_match` and `tables_match_byte_for_byte` |

All 18 went red. None stayed green, so stop rule 5 did not fire.

## Ratchet

The ceiling was 71,609 when the build started. The build measures 72,168, +559, inside the ticket's estimate of +290 to +570. The ceiling is now 72,168. By raw diff lines, the core diff module grew about 198, the edge table 122, the command's diff module 105, `tests/diff.rs` 78, `items.rs` 56, and the error types and sentences about 27. The core growth is the item pairing and its key test, the row and summary types, and the refusals. The command growth is `--match` and the table lines. I looked for duplication and found audit's key tally and diff's pairing doing the same one-match-each walk. They now share `items::pair`.

## Rungs

All four ran with `THINKTHEN_API_KEY` unset and exited 0 on the final commit: `lint`, `test`, `spec` and `surfaces`. The `lint` run printed its expected `Killed` line.

## Deviations

1. The edge-case table lives in `src/core/measure/diff_tests.rs`, a sibling module of `diff.rs`, because `diff.rs` would pass the 500-line file cap.
2. Test 1's run with no kinds runs under `--match overlap`, so record r1 shows key matches on both sides, 1 against 1.
3. Test 2 runs without a key. The ticket pins only the summary and the lost strengths.
4. Tests 5 and 6 read their inputs from fixture files, not inline text.
5. Test 5's two mix rows use two cuts on one mixed file, so the role reads `first run`.
6. The build first parsed `--match` through clap's `PossibleValuesParser`. Review cleanup 2 replaced it with audit's `Match` value enum, now shared from `cli/measure.rs`. The core may not use clap.

## Review

A fresh read-only code review accepted the build with no defects. It also served as the second agent for the ceiling raise. It named three cleanups, made in one commit:

1. The table passes the key counts straight into its `write!`.
2. audit's `Match` value enum moved to `cli/measure.rs` with one `From<Match> for Matching`, and audit and diff both parse `--match` through it. diff keeps `--match` optional, because it refuses `--match` over other verbs, so its help names the default in words.
3. diff's help says what `--table` prints for a changed record.

The line count stayed at 72,168.

## Left for later

- The ticket's "Deferred gaps" stand: `compare` for item verbs, record-level effects for item verbs, names inside `annotate` answers, and recognize's `relations`.
