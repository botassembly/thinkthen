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
3. **The `find` tie row** is the existing `verbs/find.jsonl` line 3, whose tie of `u001` and `none` holds the key `u001`. It gives 1, 1, and 0.5. The ticket's table named a tie with the key `none`; the arithmetic is the same.
4. **The malformed pointer refusal comes after the inputs are read.** The pointer is parsed where the grouping is built. A missing file with a bad pointer reports the file.
5. **The help test moved.** `tests/audit_refusals.rs` passed the 500-line file cap with the four new refusal rows. The help test is not a refusal, so it moved whole to `tests/audit_verbs.rs` and gained the pinned `--by colour` usage error.
6. **Existing pinned values moved.** `each_question_draws_its_own_bootstrap_and_the_held_out_half_reads_as_reported` pins the interval `[0.017405,0.12316]` and the error `0.086`, both from the patched prototype.

## Proof

Each test ran green on the final code. Each plant was applied alone, its test run under the heavy lock, the file restored byte for byte, and its modification time touched. A script outside the repository holds the plants. A grep of the diff for each plant's text found none.

| Test and plant | Result |
|---|---|
| `tie_share`: the share is `1/(k−1)` | RED |
| `tie_share`: a tie missing the key earns a share | RED |
| `calibration_pairs_the_answer_given`: pair by `p ≥ 0.5` | RED |
| `calibration_pairs_the_answer_given`: a nonzero share counts 1 | RED |
| `curve`: compare with `>` | RED |
| `curve`: a tie counts 0 | RED |
| `old_goldens_hold`: drop the bias shift | RED |
| `old_goldens_hold`: drop the widening | GREEN, see below |
| `crossed`: score each part at its own cut | RED |
| `crossed`: tune the held part by accuracy under `--optimize f1` | RED |
| `each_verb_grades`: fill `crossed` on the `tag` pooled row | RED |
| `pooled`: pool the `tag` labels | RED |
| `pooled`: seed the pooled bootstrap with `--seed + 1` | RED |
| `by_pointer`: read the pointer from the line root | RED |
| `by_pointer`: skip the verb split | RED |
| refusal sweep: the pointer refusal names the pointer | RED |
| `audit_verbs::help_names_audit_and_says_what_it_never_does`: `--by colour` falls through as a pointer | RED |

The widening plant stays green. The shifted errors average at least the estimate, because each is floored at 0. The interval then holds the estimate unless a few draws carry most of the mass. A search of 4,500 random pair sets, with one to forty pairs, found no case that moves the interval at six places. The only case found was a single pair, where the shifted value sits one float step below the estimate. The widening stays as the ticket and the benchmark state it, and no test can pin it.

The goldens and tables were recaptured from the prototype with the patch the fixture README prints. Golden bytes changed only in `calibration` and the two tie members. Table bytes changed only in the calibration line's numbers and the ties line.

A release build ran `audit --curve --pooled` over `249/control.jsonl` in 0.02 seconds with the part key and 0.03 seconds with the seeded key. The stop line is one second.

## Budgets

Nonblank added lines against `origin/main`.

| Budget | Limit | Measured |
|---|---|---|
| Production Rust, new and changed | 350 | 448 |
| `core/measure/group.rs` | 60 | 26 |
| Rust tests | 350 | 244 |
| `specification/audit.md` added | 70 | 35 |
| `spec/audit.md` added | 20 | 20 |
| Dependencies | none | none |

**Stop rule crossed.** Production Rust crossed its budget by 98 lines, past the one-tenth line of 385. The first count missed the two new files, so the crossing showed only after the build was complete and green. One trim moved the `--by` pointer into the grouping and named the pooled verbs, and `rustfmt` gave most of it back. Clippy then refused the table function at 106 lines and two tuple types. The fix moved the suggested, steady, crossed, and held lines into `suggested_lines` and the ties line into `ties_line`, and named two types. Those moved lines count as changed, which added 16. What remains is spread across the ticket's own pieces. `pairs.rs` (84) holds the pairs, tie share, and curve. `cli/audit.rs` (117) holds three flags, two refusals, the pointer parser, three table lines, and the moved suggested lines. `core/measure.rs` (63) holds the bias shift, the `Bin` type, and the bins. `audit.rs` (81) holds `Crossed`, `Pooled`, the new row members, and the pointer grouping. The coordinator re-scores.

## Ratchet

The build commit `411cb67a` raised the ceiling from 65,607 to 66,211 and says what grew and where the builder looked for duplication. The trim removed 7 lines. The merge of `origin/main` brought main's ceiling of 65,801. The help test's move added 1 line, its doc comment. The clippy fix added 20: two helper functions and two named types. The measured total is 66,419, and the ceiling equals it.

## Rungs

RUNGS

## Defers

- The widening has no test that can turn it red.
- The lander closes the five issues this ticket names.
