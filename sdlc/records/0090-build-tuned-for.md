# 0090: Rename the profile-warning key `calibrated` to `tuned_for`

Status: built on `ticket/0090-tuned-for`, not merged. It waits for a fresh Claude code review and the Astra review the ticket names. Owner: Claude.

## Result

`meta.profile_warning` is now exactly `{"tuned_for":NAME,"running":NAME}`. The standard-error line reads `thinkthen: warning: threshold tuned for profile old is running under profile new`. One commit renames the key in the tool, `specification/result.md` line 94, the four cases in `conformance/backend-profiles.json`, and `site/src/pages/reference.astro` line 76. ADR 0032 gains a dated amendment. Internal Rust names follow: the `ProfileWarning` field and accessor, the `Mismatch` field and accessor, `asking.rs`'s `tuned_for_profile`, and the conformance case field. The two `calibrated under` doc comments say `tuned under`. Request bytes, digests, recordings, exit codes, and the question-file key `profile` are unchanged. The digest tests pass without edits.

Item 44 of `sdlc/issues/2026-09-22-command-wording-and-help-fixes-for-0-1.md` lands its tool, specification, conformance, and site part here. The nine surfaces are not on main. Each surface ticket in queue item 10 inherits `tuned_for` from the conformance file. Claude, the queue owner, owns that part, and the issue stays open until the last surface lands.

## Red then green

- `profile::a_profile_mismatch_warns_once_and_reaches_detailed_metadata` first failed on the sentence (`threshold calibrated for` against `threshold tuned for`). With only the sentence changed, it failed on the key: left `{"calibrated":"old","running":"new"}`, right `{"tuned_for":"old","running":"new"}`.
- The inline `cli::profile::tests::one_notice_prints_once_at_the_output_boundary` failed on the old sentence.
- The four `matches("warning: threshold tuned for")` counts (`tests/backend/profile.rs` in `replay_warns_once_without_a_key_and_keeps_request_identity`, and the three in `profile/warnings.rs`) failed with 0 against 1.
- New `profile::contract_pages_name_the_tuned_for_key_and_never_the_old_one` failed on `specification/result.md`. It reads `result.md`, `reference.astro`, and `backend-profiles.json`, refuses `"calibrated"` and `calibrated:`, and requires the exact shape once in each page. A planted old line fails it inside the test.
- All pass after the rename. `shared_profile_cases_cross_the_production_parser_and_encoder` keeps its four `warning` outcomes.

## Budget

Production Rust touched six files with 0 net nonblank lines. Rust tests touched three files (`tests/backend/profile.rs`, `tests/backend/profile/warnings.rs`, and the inline test in `cli/profile.rs`) and added 22 net nonblank lines, all in the new fixed-string test. Prose touched four files. No dependency. The ratchet rises from 43754 to 43776, the measured total. The 22 lines hold one guard. It stops the old key from returning to the three contract pages a surface copies. Duplication checked first: no existing test reads these pages, and the conformance test checks behavior, not page text.

## `calibrat` left on purpose

`git grep -n calibrated -- crates specification conformance spec demos site README.md` returns only these:

- `core/digest.rs:302` and `:305`, `let calibrated` in the calibration-identity digest test. It names a question with a saved profile and does not name the field.
- `tests/backend/profile.rs:309`, the fixture file name `calibrated-question.json`. It names a temporary file.
- `tests/backend/profile.rs:488`, `:489`, and `:504`, the new guard's refused strings and its planted old line.
- `demos/25-check-the-judge/README.md:103`, which speaks of real model calibration.

The noun `calibration` stays for a later wording decision, outside the ruling:

- `calibration name`: `cli/args.rs:93`, `cli/args/find.rs:49`, `core/result/profile_warning.rs:1`, `specification/result.md:94`, `specification/roadmap.md:89`.
- `calibration profile`: `core/question_file/profile.rs:1`, `specification/annotate.md:69`, `specification/question-file.md:9` and `:17`, `specification/relate.md:22` and `:66`, `specification/result.md:42` and `:107`.
- `calibration identity`: `core/digest.rs:294`, `specification/question-file.md:90` and `:98`, `specification/relate.md:24`, `specification/question-file.schema.json:265`.
- Other: `core/backend_profile.rs:9` (threshold calibration), `specification/result.md:38` (a calibration mismatch), `conformance/README.md:13` (calibration cases), and the case id `no-calibration` in `conformance/backend-profiles.json:16`.
- The `calibration` transform measures real calibration and keeps its name.

## Gates

At `dcf0354e`, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, one rung at a time on a machine shared with one other build: `install` 0, `lint` 0 (ratchet 43776/43776), `test` 0 (library 275 passed and 2 ignored, backend 352 passed, live-test all cases passed with dummy keys), `spec` 0 (21 how-tos green, 0 red), and `git diff --check` clean. No live or paid command ran. The annotate global-queue test passed on the first run.
