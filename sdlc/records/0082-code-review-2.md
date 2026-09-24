ACCEPT

# 0082 code re-review: close the 0.1 command contract

Reviewed `ticket/0082-close-command-contract` at c57177ac (code at 56a9c764). This was a fresh, read-only session. I read the first review, the ticket, and the updated build record. Every check ran in a scratch clone with its own CARGO_TARGET_DIR, and THINKTHEN_API_KEY and THINKTHEN_BASE_URL unset. The clone is deleted. `sdlc/scripts/live` never ran.

## 1. B1 and B2

- B1 fixed. README.md:21 now reads `- A yes, a no, a not sure answer, and a broken run stay four different outcomes in the output and in the exit code.` `version::the_specification_defines_unresolved_once_and_keeps_the_closed_wording` pins it. With README.md restored from 2dd4d6d1, that test fails at version.rs:218. The record corrects the false premise in its deviations list and in the row 1 proof.
- B2 fixed. I put a wrapper `thinkthen` on PATH that exits 2 for `relate --help` and runs the real binary otherwise. The new `demos` printed `demos: thinkthen relate --help failed, so its help was not checked` and exited 1. The old `demos` from 2dd4d6d1 printed `demos: 21 green, 0 red` and exited 0 against the same wrapper. A wrapper that fails root `--help` also exits 1. Against the old `demos`, the new self-test fails 2 cases (`vocabulary-no-help` and `vocabulary-spelling`).

## 2. Follow-ups 1, 3, 4

- F1: the `--jobs` refusal now says `a single text sends one request`. Restoring the old `cli/failure.rs` fails `backend::timeout::a_one_document_run_refuses_jobs_and_sends_nothing` at timeout.rs:79, with old text on the left and new text on the right. The test covers both the dry run and a real run. In each, it checks exit 2, empty stdout, the full stderr sentence, zero connections, and zero requests. This meets the ticket's rule for changed refusals.
- F3: with the `judgement` rule removed, `demos-self-test` reports `1 cases failed`.
- F4: with the old `specification/score.md` restored, the same version test fails at version.rs:220.

## 3. Ratchet

The nonblank `.rs` total under `crates` is 43782 at 915119b9 and at 538fe510, and 43897 at c57177ac. `sdlc/ratchet.json` max is 43897. Lint printed `ratchet: crates 43897/43897`. The increase over main is +115, and the commit message defends the +5.

## 4. Six test files against a cap of 5

Acceptable. The sixth file is `tests/backend/timeout.rs`. It already held the only test of this refusal, and the fix rewrote that test in place. The ticket requires any changed refusal to be pinned with a counted listener. Moving the pin into one of the five help test files would split one refusal's tests across two files for no gain. The line budget holds (146 of 220). The follow-up was added on the owner's instruction, and the record lists the overrun as a departure.

## 5. Issues on main

origin/main e161d129 adds `sdlc/issues/2026-09-24-relate-help-lists-jobs-and-relate-refuses-it.md` (F2) and `sdlc/issues/2026-09-24-recognize-and-relate-help-carry-no-cost-sentence.md` (F5). Both are Open and state the problem accurately.

## 6. Merge

`git merge-tree --write-tree origin/main c57177ac` is clean. Main moved from 538fe510 to e161d129 by one commit, and that commit adds only the two issue files. It touches no file on the branch, so the check and review results still cover the landing commit.

## 7. Ladder at c57177ac

- `install`: rc 0.
- `lint`: rc 0, `ratchet: crates 43897/43897`.
- `test`: rc 0. 729 Rust tests passed and 0 failed. `live-test: all cases passed`.
- `spec`: rc 0. `demos-self-test: 21 cases pass`. `demos: 21 green, 0 red`.

## Nits (not blocking)

- N1. Audit row 30 in the build record is out of date. It still quotes the refusal as `one document sends one request` and marks it `already fixed`. This branch changed that sentence, so the row should cite the new text and the `a_one_document_run_refuses_jobs_and_sends_nothing` test. It could also be marked `fixed here`. Fix this in the record when convenient.
- N2. Some internal doc comments still say "one document" (`cli/judge.rs:56`, `cli/asking.rs:482`, `core/render.rs:14`, and others). None of them reaches help or a diagnostic, so they are outside the vocabulary scope.
