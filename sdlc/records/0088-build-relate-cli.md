# 0088: Build the public `relate` command

Status: The first review (`0088-review-claude.md`) rejected the handed-off diff, and the builder fixed findings 1 through 9. The Codex re-review (`0088-review-codex.md`) rejected `6f0e094c`. The queue owner decided each Codex finding, and the builder fixed all four on the ticket branch. A fresh Opus review (`0088-review-final.md`) accepted `5010dd21`. Its follow-ups F1, F2, and F4 are issues on main.

## Result

The tenth command reads one complete entity set from a JSON document, lines, JSONL, CSV, or TSV. It accepts ordered inline relation rules or one closed version-one `@entities` question file. Command-line field, kind, threshold, and model values independently override file values. The command validates the complete set before it sends anything. It delegates wildcard expansion, method selection, option and request-byte fallback, relation state, splitting, the cut, and edge assembly to shared owners. Bare output writes name-and-kind edges. Dry run writes the `thinkthen.relate-plan/1` report. Detailed output writes the Option A `thinkthen.result/1` object. A mixed recoverable logical failure prints its output and exits 6. A reply with no valid logical answer exits 4 without output.

The handed-off state is commit `328c2f25`. Everything below records the review fixes after it.

## Review findings and fixes

1. Target-side asker. The shared `QuestionMap::Choice` now carries its asker, and relate reads roles from it (`f7aac994`). Red: `a_target_side_asker_keeps_its_roles_and_the_declared_edge_direction` failed with the asker reported as `source`.
2. `@FILE` beside inline rules. Any `@` argument beside another rule exits 2 with a fixed sentence and zero sends (`6b117f17`). Red: the test exited 0 for `calls @q.json`.
3. Bare name holding `:`. The inline grammar refuses it (`a1169a15`). Red: the core grammar test accepted `works_for:person`.
4. Shared secrecy and refusal suites. Relate is a row of the shared verb table (`7eacd9cc`). Every backend and recording route runs over it in document and JSONL framings, and over lines, CSV, and TSV, in both views. That covers success, dry run, default and explicit cache, record, replay, replay miss, damaged and hostile entries, a refused address, transport failure, status and decode failures, retries, storage failure, and a missing key. The sweep now points the platform default cache into each case folder, reads what it wrote, and requires the success route to write it. Every shared refusal row runs on relate, with its own sentence where the grammar differs, and nine relate rows pin complete-set refusals with zero sends. The profile refusal sweep includes relate (`b0a2c0bf`), and the `Debug` sweep reads relate failures and relation entities (`dd27952c`). The mixed logical failure route stays in `secrecy_relate.rs`, so the `secrecy` filter runs it (`489a277c`). The 500-line secrecy page split first (`a6920ef3`). Red: the refusal sweep found that relate accepted `--jobs` and sent a request, and the `Debug` sweep found that `RelationEntity` printed its name and kind. A planted `eprintln!` of the entities failed both secrecy tests before it was removed.
5. Second fallback and threshold owners. `SettledRelation::settle` in `engine/prepared_request.rs` is now the one fallback owner, and recognize and relate both call it. The assembler exports its one cut, `reaches_cut`, which relate's detailed entries call (`de73902f`). The unreachable limit arm is gone. This is a refactor under a behavior test that passed before and after, `a_backend_profile_option_limit_falls_back_per_concrete_relation_in_the_plan`. Codex finding 4 below removes the second preparation this left in recognition.
6. Digest. The core owns `RelateQuestion`, including null fields under `--lines`, and production hashes it (`22ab8f5a`). Red: the core test failed to compile because the question did not exist. A backend test pins the production `meta.question_sha256` for a lines run, which `sha256sum` confirmed from the printed question bytes.
7. Budget. The added production `#[rustfmt::skip]` attributes and the `too_many_arguments` expectation are gone (`032ba90e`, `de73902f`). One skip on the `relate` test module stayed until Codex finding 2 below removed it. The failure conversions moved out of `cli/failure.rs` before relate's variant, and the two deleted doc comments are back.
8. Number format. Detailed entries are serde structures, so edges and entries both write `1.0` (`22ab8f5a`). Red: the exact Option A test failed on the old `"probability":1`.
9. Dead code. `Meta::new` discarded `failed_questions`, so relate's string replacement was not dead. `Meta::new` now keeps the count, and the replacement is gone (`22ab8f5a`). The unreachable arm went with finding 5. A valid question file for another verb exits 2, and an invalid one exits 5 (`31d90282`). Red: the mixed test reported `failed_questions` 0 once the replacement was removed, and an invalid decide file exited 2.

Further proof added: the exact dry-run object and its request digest equal to the digest a real run sends, an H entry at the inclusive cut, bare exit 6, wildcard concrete order, default and named cache reuse with zero sends on the second run, and a runtime backend profile that leaves saved calibration provenance alone.

## Codex review findings and fixes

1. Reversed target-side wording. When the asker is the target, the choice now asks `___ {reads} {asker}` (`c84051f1`). Red: `a_choice_puts_the_blank_on_the_side_the_options_fill` failed on "Acme is linked to ___". No recorded fixture held a target-side choice, so no digest changed. The issue on main is closed by this commit (`e61ffee9`).
2. Budget. The queue owner amended the ticket's budget line with the gross numbers below. A `#[rustfmt::skip]` ceiling in `policy.py` now fails the lint rung above three (`16e61190`). A planted fourth skip failed it with `format: crates hold 4 rustfmt::skip attributes and the ceiling is 3`. Main holds three: two in `src` and one on the `recognize` test module.
3. `--lib relate_file` cases. `RelateSpec::admit` and `RelateSpec::check_lines` in `core/relate_file.rs` own the 255 limit, blank names, duplicates, absent kinds, and line rules (`f88954d8`). The command edge calls them. Six `relate_file` unit tests pin those rules. The old copy in `cli/relate/input.rs` is gone.
4. Second preparation. Recognition now sends the chunks `SettledRelation::settle` prepared. Recognize and relate both send through `Asking::chunks` in `cli/asking/request.rs` (`714247eb`). Red: a thread-local preparation counter read 2 for one settled relation. It reads 1 now.

## Decisions Ian can overturn

- Relate refuses `--jobs` at exit 2, because it sends its requests in order. Before this it accepted the flag and ignored it.
- An invalid file that names another verb is exit 5, like any invalid question file.
- Relation file rules and cuts now use recognition's readers. A malformed relation in a relate file reports the relate relation sentence, and a malformed shape reports the closed-file sentence.

## Budget

Measured with `git diff --unified=0 cdfd0e5e...HEAD -- 'crates/**/*.rs'`, gross nonblank lines added under `crates`:

- Production: 26 files changed and 1,518 lines added. The original cap was 15 files and 1,200 lines.
- Test-only: 19 files changed and 1,544 lines added. The original cap was 11 files and 1,100 lines.
- Total: 3,062 lines added against the original 2,300 gross cap. Added lines count code moved between files.
- The queue owner amended the ticket to these numbers after the Codex review. Growth is the command, the shared secrecy and refusal matrices, the required file splits, and the 0081 shared fixes.
- The ceiling in `sdlc/ratchet.json` is 43,563, up from main's 41,045.
- No Rust file exceeds 500 nonblank lines.

The shared owners outside the ticket's first list are `core/relation.rs`, `engine/prepared_request.rs`, `cli/recognize/relation.rs`, `cli/asking/request.rs`, `core/result.rs`, `core/recognize_file.rs`, `cli/failure/convert.rs`, and `Common::framing` in `cli/args.rs`. `Common::framing` replaced four copies. These change 0081's shared owners without changing recognition output.

Duplication remains in one place. Relate's `add_meta` repeats recognition's `Aggregate::add_answered`. On a model mismatch, relate reports `ModelsDiffer(None)` and drops the two model names that recognition prints.

## What is not proven

- The help test checks fragments with `contains`.
- The request digest identity is pinned for one choice relation, not for every method.
- `RecognizedName` still derives `Debug` with its name. It belongs to recognition and is outside this ticket.

## Gates

The builder ran `install`, `lint`, `test`, and `spec` one at a time from `bacf6405` with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, then `git diff --check`. Every command exited 0. `lint` reported `ratchet: crates 43563/43563` and `pages: 1 coming, 21 green`. `test` passed 715 Rust tests with 2 ignored and 0 failed across 13 test binaries, plus its script self-tests. `spec` reported `demos: 21 green, 0 red`. Only this record and the ticket's budget line changed after that run.

## Landing

Main fast-forwarded to `71841025`, the merge of `origin/main` into the ticket branch. The builder ran `install`, `lint`, `test`, and `spec` one at a time at that exact commit with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. Every command exited 0. `lint` reported `ratchet: crates 43563/43563` and `pages: 1 coming, 21 green`. `test` passed 715 Rust tests with 2 ignored and 0 failed across 13 test binaries. `spec` reported `demos: 21 green, 0 red`. This landing note is a doc-only commit after that run, so the gate result still applies to the code.

No live, paid, or external call ran. Tests used replay and loopback listeners.
