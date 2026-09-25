# Quick Fix qf-diff-warnings: diff warns when nothing pairs or the question digests differ

Status: built on branch `ticket/qf-diff-warnings`. It awaits review and landing. It closes `sdlc/issues/closed/2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md`.

## Prior evidence

Experiment 259 filed the issue. It showed `diff` printing `0 of 0 changed` and exiting 0 when no answer paired. It also showed `diff` pairing two runs of different questions with no word. The issue offered six options. Options 2 and 4 change no standard output byte, and this Quick Fix builds only those two.

## Behavior kept

- Every standard output byte. The eight JSON goldens and three table captures still match byte for byte.
- Exit 0 on every comparison that reads.
- The pure core. `core/measure/answer.rs` reads the digest from each line, and `core/measure/diff.rs` counts pairs whose digests differ. The count is `#[serde(skip)]`, so the summary JSON is unchanged. Only `cli/diff.rs` writes to standard error.

## Changes

- `core/measure/answer.rs`: `Answer` gains `digest`, read from the line's `meta.question_sha256`. An `annotate` line gives the one row digest to each of its answers. The change is 12 lines.
- `core/measure/diff.rs`: `Summary` gains the skipped count `digests_differ`. A pair counts only when both lines carry a digest and the digests differ.
- `cli/diff.rs`: after standard output, diff writes up to two warning lines to standard error. A failed write to standard error is ignored, so the exit code stays 0.
  - `thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names`
  - `thinkthen: diff: warning: the question digest differs in D of N paired answers. A different question or threshold gives a different digest.`
- `specification/diff.md` gains a "Warnings" section and drops the claim that diff never checks the question. `spec/diff.md` gains two executable blocks: an empty second run and the reworded 249 run.
- `sdlc/ratchet.json` rises from 61768 to 61858 (90 lines). About 34 are source and about 56 are the new test.

## Proof

`crates/thinkthen/tests/diff.rs`:

- `warnings_go_to_standard_error_and_leave_the_output_and_exit_alone` is an outside-in edge table. It runs `small/decide.jsonl` against a changed `small/decide-b.jsonl` on standard input with `--table`, and it pins the exit code, standard output, and standard error of each row.

| Row | Second run | Standard output | Standard error |
| --- | --- | --- | --- |
| neither | as saved | the `diff-decide-nokey` table capture | empty |
| digest mismatch | every digest `01` | the same capture | the digest warning, `6 of 6` |
| one side has no digest | digests removed | the same capture | empty |
| no pairs | ids renamed | `0 of 0 changed`, only in A 6, only in B 7 | the no-pair warning |
| no pairs and a digest mismatch | ids renamed, digests `01` | the same as no pairs | the no-pair warning only |

- `goldens_match` now pins standard error for each golden. The `diff-249-soft` golden compares two wordings of one question, so it pins the digest warning at `272 of 272`. Every other golden pins empty standard error.

Four questions for the new table. It protects both warnings, their exact sentences, and unchanged standard output and exit. A regression that drops a warning, counts a missing digest as a difference, always warns, or moves a warning to standard output fails it. No test checked standard error on a successful diff with unpaired or mismatched runs before. It needs no test-only hook; it drives the binary through standard input.

### Plants

Each plant ran `cargo test --test diff`. Each turned red, and each restored file was touched.

| Plant | Red tests |
| --- | --- |
| P1: the no-pair warning never prints (`records == usize::MAX`) | the edge table (no pairs row) |
| P2: digests never count (`&& false` in the guard) | the edge table (digest row), `goldens_match` (249 soft) |
| P3: a missing digest counts as different (`(p, q) if p != q`) | the edge table (one side has no digest row) |
| P4: the digest warning prints whenever a pair exists | `goldens_match`, `tables_match_byte_for_byte`, the edge table |
| P5: warnings go to standard output as well | the edge table, `goldens_match`, `the_held_out_half_reads_as_the_prototype_reports` |

## Checks

With `THINKTHEN_API_KEY` unset, under `flock -o /run/user/1000/thinkthen-heavy.lock`:

- `install`: exit 0, silent.
- `lint`: exit 0 on its second run. The first run failed on `cargo fmt --check` over the new test; `cargo fmt` fixed it, and the ceiling rose 6 lines to 61858.
- `test`: exit 0, 888 tests passed and 0 failed.
- `spec`: exit 0. `spec/diff.md` runs 4 blocks, and the demos read `21 green, 0 red`.

`surfaces` did not run. No surface or public API changed.

## Deferred gaps

- No test reaches the `annotate` digest path. The row digest is shared by each answer, and the committed `annotate` fixture only diffs against itself.
- Options 1, 3, 5, and 6 of the issue change the diff contract. Each needs Ian's ruling.
- `sdlc/scripts/policy.py` describes diff as writing only standard output. Its token check does not ban standard error, and this change left the file alone.

## What Ian can overturn

- The digest warning's wording and its `D of N` count form.
- Counting a pair only when both lines carry a digest.
- Warnings on standard error at exit 0 instead of a nonzero exit (option 1 or 5).
