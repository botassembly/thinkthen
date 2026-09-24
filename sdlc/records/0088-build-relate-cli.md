# 0088: Build the public `relate` command

Status: The first review (`0088-review-claude.md`) rejected the handed-off diff. The builder fixed findings 1 through 9 on the ticket branch. A fresh independent review and landing remain open. Finding 10 is outside this ticket and is open as `sdlc/issues/2026-09-24-a-target-side-choice-asks-the-reversed-relation.md` on main.

## Result

The tenth command reads one complete entity set from a JSON document, lines, JSONL, CSV, or TSV. It accepts ordered inline relation rules or one closed version-one `@entities` question file. Command-line field, kind, threshold, and model values independently override file values. The command validates the complete set before it sends anything. It delegates wildcard expansion, method selection, option and request-byte fallback, relation state, splitting, the cut, and edge assembly to shared owners. Bare output writes name-and-kind edges. Dry run writes the `thinkthen.relate-plan/1` report. Detailed output writes the Option A `thinkthen.result/1` object. A mixed recoverable logical failure prints its output and exits 6. A reply with no valid logical answer exits 4 without output.

The handed-off state is commit `328c2f25`. Everything below records the review fixes after it.

## Review findings and fixes

1. Target-side asker. The shared `QuestionMap::Choice` now carries its asker, and relate reads roles from it (`f7aac994`). Red: `a_target_side_asker_keeps_its_roles_and_the_declared_edge_direction` failed with the asker reported as `source`.
2. `@FILE` beside inline rules. Any `@` argument beside another rule exits 2 with a fixed sentence and zero sends (`6b117f17`). Red: the test exited 0 for `calls @q.json`.
3. Bare name holding `:`. The inline grammar refuses it (`a1169a15`). Red: the core grammar test accepted `works_for:person`.
4. Shared secrecy and refusal suites. Relate is a row of the shared verb table (`7eacd9cc`). Every backend and recording route runs over it in document and JSONL framings, and over lines, CSV, and TSV, in both views. That covers success, dry run, default and explicit cache, record, replay, replay miss, damaged and hostile entries, a refused address, transport failure, status and decode failures, retries, storage failure, and a missing key. The sweep now points the platform default cache into each case folder, reads what it wrote, and requires the success route to write it. Every shared refusal row runs on relate, with its own sentence where the grammar differs, and nine relate rows pin complete-set refusals with zero sends. The profile refusal sweep includes relate (`b0a2c0bf`), and the `Debug` sweep reads relate failures and relation entities (`dd27952c`). The mixed logical failure route stays in `secrecy_relate.rs`, so the `secrecy` filter runs it (`489a277c`). The 500-line secrecy page split first (`a6920ef3`). Red: the refusal sweep found that relate accepted `--jobs` and sent a request, and the `Debug` sweep found that `RelationEntity` printed its name and kind. A planted `eprintln!` of the entities failed both secrecy tests before it was removed.
5. Second fallback and threshold owners. `SettledRelation::settle` in `engine/prepared_request.rs` is now the one fallback owner, and recognize and relate both call it. The assembler exports its one cut, `reaches_cut`, which relate's detailed entries call (`de73902f`). The unreachable limit arm is gone. This is a refactor under a behavior test that passed before and after, `a_backend_profile_option_limit_falls_back_per_concrete_relation_in_the_plan`.
6. Digest. The core owns `RelateQuestion`, including null fields under `--lines`, and production hashes it (`22ab8f5a`). Red: the core test failed to compile because the question did not exist. A backend test pins the production `meta.question_sha256` for a lines run, which `sha256sum` confirmed from the printed question bytes.
7. Budget. Every added `#[rustfmt::skip]` and the `too_many_arguments` expectation are gone (`032ba90e`, `de73902f`). The failure conversions moved out of `cli/failure.rs` before relate's variant, and the two deleted doc comments are back.
8. Number format. Detailed entries are serde structures, so edges and entries both write `1.0` (`22ab8f5a`). Red: the exact Option A test failed on the old `"probability":1`.
9. Dead code. `Meta::new` discarded `failed_questions`, so relate's string replacement was not dead. `Meta::new` now keeps the count, and the replacement is gone (`22ab8f5a`). The unreachable arm went with finding 5. A valid question file for another verb exits 2, and an invalid one exits 5 (`31d90282`). Red: the mixed test reported `failed_questions` 0 once the replacement was removed, and an invalid decide file exited 2.

Further proof added: the exact dry-run object and its request digest equal to the digest a real run sends, an H entry at the inclusive cut, bare exit 6, wildcard concrete order, default and named cache reuse with zero sends on the second run, and a runtime backend profile that leaves saved calibration provenance alone.

## Decisions Ian can overturn

- Relate refuses `--jobs` at exit 2, because it sends its requests in order. Before this it accepted the flag and ignored it.
- An invalid file that names another verb is exit 5, like any invalid question file.
- Relation file rules and cuts now use recognition's readers. A malformed relation in a relate file reports the relate relation sentence, and a malformed shape reports the closed-file sentence.

## Budget

Measured with `git diff cdfd0e5e HEAD`, nonblank Rust lines under `crates`:

- Production: 25 files changed, 1,446 lines added, 276 removed, 1,170 net. The cap is 15 files and 1,200 lines.
- Test-only: 18 files changed, 1,344 lines added, 244 removed, 1,100 net. The cap is 11 files and 1,100 lines.
- Total net growth is 2,270, within the 2,300 cap. Added lines count code moved between files.
- The ceiling in `sdlc/ratchet.json` is 43,315, up from main's 41,045. The handed-off diff had set 43,020 with the formatter suppressed.
- No Rust file exceeds 500 nonblank lines. The largest changed files are `core/result.rs` at 499 and `tests/backend/refusals.rs` at 469.

The file counts exceed the ticket because the review's fixes live in shared owners. Those owners are `core/relation.rs` (asker, cut, withheld `Debug`), `engine/prepared_request.rs` (fallback owner, outside the ticket's `opens` list), `cli/recognize/relation.rs`, `core/result.rs`, `core/recognize_file.rs`, `cli/failure/convert.rs`, and `Common::framing` in `cli/args.rs`, which replaced four copies in `annotate.rs`, `asking.rs`, `recognize.rs`, and `recognize/config.rs`. These change 0081's shared owners without changing recognition output. The ticket's stop rule asks for a re-score when that happens, so the ceiling raise, the file counts, and the 0081 owner changes need second-agent review.

## What is not proven

- Line restrictions, duplicates, absent kinds, and the 255/256 boundary are pinned by the shared refusal sweep and backend tests, not by `--lib relate_file`.
- The help test checks fragments with `contains`.
- The request digest identity is pinned for one choice relation, not for every method.
- Relate output for one-way cross-kind rules where the target side asks depends on the open finding 10 issue.
- `RecognizedName` still derives `Debug` with its name. It belongs to recognition and is outside this ticket.

## Gates

No live, paid, or external call ran. Tests used replay and loopback listeners.
