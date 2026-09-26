# 0131 build: audit matches the bench

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0131-audit-matches-the-bench` from the accepted ticket head `d9080ad5`. Ticket 0125 landed at `d15d417c` first. `origin/main` was merged before the build and again before the final run. Ian can overturn every decision below.

## Outcome

audit gives a tie that holds the key its share of `1/k` and prints `tied_holding_key` and `tie_share`. Calibration pairs each answer with its confidence in the answer it gave as run, counts a tie at its share, and shifts the bootstrap interval by its bias. The calibration object prints `by_bin`. `suggested.crossed` scores each part at the bar the other part tuned. `--by` takes a JSON pointer into each input and splits each value by verb. `--curve` adds the coverage curve at every confidence. `--pooled` adds one calibration line over every paired verb. `--curve` with `--table`, and a pointer `--by` with `--write`, are refused.

## Preconditions

`answer.rs` was 448 lines on 0125's code and is 450 now. The branch compiled on 0125's code before any change.

## Where the code lives

0125 moved the per-verb readers into `core/measure/verbs.rs` and row assembly into `rows.rs`. The design moved with them. No decision changed.

- `core/measure/pairs.rs` holds the tie share, the calibration pairs, and the curve. The ticket put the pairs in `rows.rs`. A separate file keeps `rows.rs` to row assembly, and the pooled line reuses the same pairs.
- `core/measure/group.rs` reads the `--by` value of each line.
- The ticket named a new `core/measure/top.rs`. 0125 put `top()` in `verbs.rs`, so `Top` gained its `holders` there and no file was added.
- `optimize.rs` computes `crossed` with the existing tuner and count. `rows.rs` gained `added()`, which shares the measures with `count()`.
- `core/measure.rs` holds the bias shift and the bins.
- `cli/audit.rs` holds the flags, the refusals, and the three table lines.

## Deviations from the ticket

1. **`crossed` is absent where the measure does not apply.** The ticket put `crossed` last in `suggested`. The object for a measure that does not apply has kept four members since 0125, and a test pins it. `crossed` follows `steady` and is absent there, null where no bar was tuned, and the object otherwise.
2. **`rank` calibration is null.** Decision 14 says so. 0125 had printed a calibration for `rank`. `audit_verbs::each_verb_grades` now pins it null.
3. **The `find` tie rows.** `verbs/find.jsonl` line 3 ties `u001` and `none` and holds the key `u001`, so it earns 0.5. Line 5, added after code review, ties `u001` and `u002` and prints `u001`, as `specification/find.md` does for a tie among real units. It reads as `u001`, is right, and earns no share. The row prints `tied` 1, `tied_holding_key` 1, and `tie_share` 0.5.
4. **The malformed pointer refusal comes after the inputs are read.** The pointer is parsed where the grouping is built. A missing file with a bad pointer reports the file.
5. **The help test moved.** `tests/audit_refusals.rs` passed the 500-line file cap with the four new refusal rows. The help test is not a refusal, so it moved whole to `tests/audit_verbs.rs` and gained the pinned `--by colour` usage error.
6. **Existing pinned values moved.** `each_question_draws_its_own_bootstrap_and_the_held_out_half_reads_as_reported` pins the interval `[0.017405,0.12316]` and the error `0.086`, both from the patched prototype.

## Proof

Each test ran green on the final code. Each plant was applied alone, its test run under the heavy lock, the file restored byte for byte, and its modification time touched. A script outside the repository holds the plants. A grep of the diff for each plant's text found none.

| Test and plant | Result |
|---|---|
| `tie_share`: the share is `1/(k−1)` | RED |
| `tie_share`: a tie missing the key earns a share | RED |
| `each_verb_grades`: a `find` tie that printed a unit earns a share | RED |
| `calibration_pairs_the_answer_given`: pair by `p ≥ 0.5` | RED |
| `calibration_pairs_the_answer_given`: a nonzero share counts 1 | RED |
| `curve`: compare with `>` | RED |
| `curve`: a tie counts 0 | RED |
| `old_goldens_hold`: drop the bias shift | RED |
| `old_goldens_hold`: drop the widening | GREEN, see below |
| `core::measure::tests::the_calibration_interval_holds_the_error_of_one_pair`: drop the widening | RED |
| `crossed`: score each part at its own cut | RED |
| `crossed`: tune the held part by accuracy under `--optimize f1` | RED |
| `each_verb_grades`: fill `crossed` on the `tag` pooled row | RED |
| `pooled`: pool the `tag` labels | RED |
| `pooled`: seed the pooled bootstrap with `--seed + 1` | RED |
| `by_pointer`: read the pointer from the line root | RED |
| `by_pointer`: skip the verb split | RED |
| refusal sweep: the pointer refusal names the pointer | RED |
| `audit_verbs::help_names_audit_and_says_what_it_never_does`: `--by colour` falls through as a pointer | RED |

The widening plant leaves the goldens green. The shifted errors average at least the estimate, because each is floored at 0, so the interval holds the estimate unless a few draws carry most of the mass. A search of 4,500 random pair sets found one case: a single pair at 0.94, right, whose shifted value sits one float step below the estimate. The core edge test pins that pair, and the plant turns it red.

The goldens and tables were recaptured from the prototype with the patch the fixture README prints. Golden bytes changed only in `calibration` and the two tie members. Table bytes changed only in the calibration line's numbers and the ties line.

A release build ran `audit --curve --pooled` over `249/control.jsonl` in 0.02 seconds with the part key and 0.03 seconds with the seeded key. The stop line is one second.

## Budgets

Nonblank added lines against `origin/main`.

| Budget | Limit | Measured |
|---|---|---|
| Production Rust, new and changed | 425, re-scored | 422 |
| `core/measure/group.rs` | 60 | 26 |
| Rust tests | 350 | 289 |
| `specification/audit.md` added | 70 | 35 |
| `spec/audit.md` added | 20 | 20 |
| Dependencies | none | none |

**Stop rule and re-score.** The first build measured 448 against 350, past the one-tenth line of 385. Coordinator re-scored: ticket pieces need about 425; accepted after reviewer trim list. The trim took these from the list:

- `pair()` returns nothing for `rank` and `score`, so `pairs_verb` is gone and `pairs()` is one `filter_map`.
- `pair()` reads `Answer::confidence()` and flips it for a no.
- One `bin()` gives each bin's bounds, counts, and confidence sum, and both the error and `by_bin` read it.
- `crossed` reads `rows::cut` directly.
- The table keeps 0125's suggested block, and the ties and crossed lines ride on the agreement and steady lines through two helpers.
- The `--by` sentence is one constant.

Two list items were tried and dropped, because they added lines under the count of new and changed lines. A shared `graded()` helper replaced 0125's own lines in `rows.rs`, and a shared `calibration_text()` replaced the unchanged row calibration line. Making `tied` a method adds as many lines as it removes. The measured count is 422.

## Ratchet

The build commit `411cb67a` raised the ceiling from 65,607 to 66,211 and says what grew and where the builder looked for duplication. The trim removed 7 lines. The merge of `origin/main` brought main's ceiling of 65,801. The help test's move added 1 line, its doc comment. The clippy fix added 20: two helper functions and two named types. The review fixes and the trim moved the total to MEASURED, and the ceiling equals it after the merge of `origin/main`.

## Rungs

Each rung ran directly on the branch at `9a4dc0a3`, merged with `origin/main` at `2c4fc294`. A later merge of `origin/main` at `3f7b51ac` brought one issue file and no code.

- `install`: exit 0.
- `lint`: exit 0. The first run found `tests/audit_refusals.rs` past 500 lines, then clippy's table-length and type limits. Both are fixed above.
- `test`: exit 0. The first run failed in `demo_runner`: the `--curve` help said "row", a word the help check refuses. It now says "group".
- `spec`: exit 0, 51 page blocks passed, and 21 demos green. The first run failed the pooled block, because `mustmatch` read the expected JSON arrays as JSON. The block now prints text.
- `surfaces`: exit 1. Every surface passed except `libraries/c`, which failed on both runs in `door::cases::a_retried_status_is_retryable_and_a_refused_key_is_not` with `Text file busy`. `sdlc/issues/2026-09-25-two-gate-failures-in-a-root-container.md` item 2 records this launch race. The C surface does not reach audit. The `--by`, `--curve`, and `--pooled` help lines are the public surface this ticket changed, and `test` checks them.

## Code review fixes

1. A `find` tie among real units printed the first unit, so it counted as right and earned a share too. A tie now earns a share only when the answer reads as tied as run. `verbs/find.jsonl` line 5 covers it, and planting the old rule turns `each_verb_grades` red.
2. The widening has its edge test, above.
3. The `--by` sentence lives in one constant in `cli/measure.rs`.

## Defers

- The lander closes the five issues this ticket names.
