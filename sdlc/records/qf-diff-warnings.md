# Quick Fix qf-diff-warnings: diff warns when nothing pairs or the question digests differ, and McNemar counts every pair that crosses right

Status: landed 2026-09-25 from branch `ticket/qf-diff-warnings`. It closes `sdlc/issues/closed/2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md` and `sdlc/issues/closed/2026-09-25-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md`. The coordinator added the second issue during review, because it touches the same files.

## Prior evidence

Experiment 259 filed the issue. It showed `diff` printing `0 of 0 changed` and exiting 0 when no answer paired. It also showed `diff` pairing two runs of different questions with no word. The issue offered six options. Options 2 and 4 change no standard output byte, and this Quick Fix builds only those two. Options 1, 3, 5, and 6 are declined for now. Options 1 and 5 would change the exit contract. Options 3 and 6 would move the goldens and repeat what the warnings say.

The McNemar issue moved here from the retired prototype. Record 0114 kept the prototype's narrow rule until its owner ruled. The prototype is retired, so no upstream golden holds that rule. The issue recommends option 1: count every pair that is right on one side and not right on the other.

## Behavior kept

- Every standard output byte from the warning work. The warnings move no golden. The McNemar change moves two captures, listed below, and no others.
- Exit 0 on every comparison that reads.
- The pure core. `core/measure/answer.rs` reads the digest from each line, and `core/measure/diff.rs` counts pairs whose digests differ. The count is `#[serde(skip)]`, so the summary JSON is unchanged. Only `cli/diff.rs` writes to standard error.

## Changes

- `core/measure/answer.rs`: `Answer` gains `digest`. `Answer::read` takes it next to the answer name. A plain line gives `meta.question_sha256`. An `annotate` line gives `meta.questions_sha256` to each of its answers, the key `core/result.rs` writes for it.
- `core/measure/diff.rs`: `Summary` gains the skipped count `digests_differ`. A pair counts only when both lines carry a digest and the digests differ.
- `cli/diff.rs`: after standard output, diff writes up to two warning lines to standard error. A failed write to standard error is ignored, so the exit code stays 0.
  - `thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names`
  - `thinkthen: diff: warning: the question digest differs in D of N paired answers. A different question, threshold, or profile gives a different digest.`
- `specification/diff.md` gains a "Warnings" section and drops the claim that diff never checks the question. It says one changed question on an `annotate` line flags every answer on that line. `spec/diff.md` gains two executable blocks: an empty second run and the reworded 249 run.
- `tests/fixtures/measure/small/annotate.jsonl` now carries `questions_sha256`, the key a real `annotate` line carries. The fixtures README records the new checksum, the departure from the prototype copy, and the shared placeholder digest `00`. The annotate goldens still match.
- `core/measure/diff.rs`: `discordant` counts a pair when it is right on one side and wrong, tied, or not sure on the other. Its unit test in `core/measure/tests.rs` covers all sixteen outcome pairs under the new rule.
- Two captures moved: `golden/diff-choose.jsonl` and `golden/table/diff-choose.txt`. Record `c2` goes from tied to right, so it now counts. McNemar p moves from 1.0 to 0.5 at 2 to 4 right of 5. Each file was recaptured from `thinkthen diff`, and the fixtures README gives the new checksums and the departure. A rerun of every diff golden command showed the other six JSON goldens and two table captures unchanged. `golden/extra/diff-annotate.jsonl`, which the old spec expected to move, did not move.
- `specification/diff.md` states the wider McNemar rule and says how to restore the narrow one. `spec/diff.md` gains a block that pins p 0.500 on the `diff-choose` example.
- `sdlc/scripts/policy.py`: one comment now says diff also writes warnings on standard error. No check changed.
- `sdlc/planning/issue-backlog-2026-09-25.md`: row 4 names the closed issue.
- `sdlc/ratchet.json` rises from main's 61764 to 61893 (129 lines). About 53 are source. About 76 are the two new tests.

## Proof

`crates/thinkthen/tests/diff.rs`:

- `warnings_go_to_standard_error_and_leave_the_output_and_exit_alone` is an outside-in edge table. Each row feeds a changed second run on standard input with `--table`, and pins the exit code, standard output, and standard error.

| Row | First run | Second run | Standard output | Standard error |
| --- | --- | --- | --- | --- |
| digest mismatch | `small/decide.jsonl` | `small/decide-b.jsonl`, every digest `01` | the `diff-decide-nokey` table capture | the digest warning, `6 of 6` |
| one side has no digest | `small/decide.jsonl` | `small/decide-b.jsonl`, digests removed | the same capture | empty |
| no pairs | `small/decide.jsonl` | `small/decide-b.jsonl`, ids renamed | `0 of 0 changed`, only in A 6, only in B 7 | the no-pair warning |
| an annotate line's digest | `small/annotate.jsonl` | the same, line `a1` digest `01` | `A -> B: 0 of 5 changed` | the digest warning, `2 of 5` |

- `goldens_match` now pins standard error for each golden. The `diff-249-soft` golden compares two wordings of one question, so it pins the digest warning at `272 of 272`. Every other golden pins empty standard error. `tables_match_byte_for_byte` pins the case where neither warning prints.
- `mcnemar_counts_every_pair_that_becomes_right_or_stops_being_right` pins the issue's example both ways. `choose` against `choose-b` goes from 2 to 4 right with `c2` tied to right. The reverse goes from 4 to 2 right with `c2` right to tied. Both print `McNemar p 0.500 on right answers`.
- The table has no row for "no pairs and a digest mismatch". The digest check needs a pair, so that case cannot happen.

Four questions for the new table. It protects both warnings, their exact sentences, the annotate digest key, and unchanged standard output and exit. A regression that drops a warning, counts a missing digest as a difference, always warns, moves a warning to standard output, or reads the wrong annotate key fails it. No test checked standard error on a successful diff with unpaired or mismatched runs before. It needs no test-only hook; it drives the binary through standard input.

### Plants

Each plant ran `cargo test --test diff` after the review fixes. Each turned red, and each restored file was touched.

| Plant | Red tests |
| --- | --- |
| P1: the no-pair warning never prints (`records == usize::MAX`) | the edge table (no pairs row) |
| P2: digests never count (`&& false` in the guard) | the edge table, `goldens_match` (249 soft) |
| P3: a missing digest counts as different (`(p, q) if p != q`) | the edge table (one side has no digest row) |
| P4: the digest warning prints whenever a pair exists | `goldens_match`, `tables_match_byte_for_byte`, the edge table |
| P5: warnings go to standard output as well | the edge table, `goldens_match`, `the_held_out_half_reads_as_the_prototype_reports` |
| P6: an annotate line reads `question_sha256` | the edge table (annotate row) |
| P7: `discordant` restores the old wrong-to-right and right-to-wrong count | the McNemar edge rows, `goldens_match` (diff-choose), `tables_match_byte_for_byte` (diff-choose) |

## Checks

After merging origin/main at 0c38769e, with `THINKTHEN_API_KEY` unset. Each rung ran once and took the heavy lock itself:

- `install`: exit 0.
- `lint`: exit 0. The ratchet measured 61893.
- `test`: exit 0, 889 tests passed and 0 failed.
- `spec`: exit 0. `spec/diff.md` runs its blocks, and the demos read `21 green, 0 red`.

`surfaces` did not run. No surface or public API changed.

## Deferred gaps

- None inside the two built options.

## What Ian can overturn

- The digest warning's wording and its `D of N` count form.
- Counting a pair only when both lines carry a digest.
- Declining option 1: a distinct nonzero exit when nothing pairs.
- Declining option 3: a null `mcnemar_p` when no test ran.
- Declining option 5: exit 2 on differing digests, with an option to allow them.
- Declining option 6: a `question_mismatch` member in the summary.
- The wider McNemar rule. It counts a pair that moves between right and tied or not sure. Restoring the prototype's narrow rule reverses `discordant`, its unit test, the McNemar edge rows, the two `diff-choose` captures, the McNemar block in `spec/diff.md`, and the fixtures README checksums and note.

## Landing

The landing test run failed twice on flaky tests that were already on main. Neither involves `diff`. `profile::a_structured_dry_run_counts_its_complete_body_at_the_edge` read a profile file that its parallel sibling had just truncated. The dry-run test now writes its own `plan-` files, and the send test keeps the names its pinned sentence uses. This raises the ceiling from 62100 to 62101, and a second agent reviewed the raise. The `find_edge` broken-pipe failure was fixed on main by ticket 0129's landing. After that fix, lint, test, and spec each exited 0.
