ACCEPT

# 0088 final independent review (Claude, read-only)

Reviewed `git diff origin/main...HEAD` at `5010dd21` (origin/main is not ahead; no crate change on main since `cdfd0e5e`). Scratch copy at /tmp/claude-1000/0088-final-review with its own target dir. `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. No live or paid call. Paths are relative to `crates/thinkthen/` unless they start with `sdlc/`.

## What I ran

- Baseline: `--lib relate_file` 6/6, `--lib relation` 9/9, `--test backend relate` 16/16, `secrecy` 5/5, `refusals` 3/3, `recogn` 22/22, `--test question_file relate` 2/2, `--test relate_edge` 1/1.
- `sdlc/scripts/lint` exit 0 (policy tables match, ratchet 43,563). `sdlc/scripts/spec` exit 0 (demos 21 green, 0 red, demo 45 3 passed).
- `git diff --unified=0 cdfd0e5e...HEAD -- 'crates/**/*.rs'` nonblank additions: 26 production files, 1,518 lines; 19 test files, 1,544 lines. These match the amended ticket and the record.

## Prior findings: each fix reverted in the scratch copy

| Fix | Mutation | Test that went red |
|---|---|---|
| Codex 1 / Claude 10: target-side wording | `core/relation.rs:238` back to `{asking} {reads} ___` | `core::relation::tests::a_choice_puts_the_blank_on_the_side_the_options_fill` |
| Claude 2: @FILE beside inline rules | `cli/relate/config.rs:219` arm disabled | `relate::a_question_file_beside_inline_rules_is_refused_before_any_send`, and the shared refusal sweep |
| Codex 3: 255 limit | `core/relate_file.rs:230` to `> MAX_ENTITIES + 1` | `a_complete_set_admits_255_entities_and_refuses_the_256th` (the refusal sweep hangs instead of failing; see F1) |
| Codex 3: duplicates / absent kind | each check disabled | `a_complete_set_refuses_blanks_duplicates_and_absent_concrete_kinds` |
| Codex 3: edge wiring | `cli/relate/input.rs` skips `admit` | `every_settled_empty_input_outcome_is_identical_in_normal_and_dry_runs`, and the refusal sweep |
| Claude 3: bare name with `:` | colon branch disabled | `inline_grammar_and_closed_file_shape_are_refused` |
| Claude 1: asker role | `asks_as_source` forced true | `a_target_side_asker_keeps_its_roles_and_the_declared_edge_direction` |
| Claude 4: secrecy | planted `eprintln!` of the first entity name; separately, of all names on the exit-6 path | 4 secrecy tests; `secrecy_relate::a_partial_relation_answer_…` |
| Claude 6: production digest | `fields` never null under `--lines` | `the_detailed_question_digest_hashes_the_printed_line_question` |
| Claude 9: failed_questions | `Meta::new` (`core/result.rs:320`) forced to 0 | `mixed_logical_failure_prints_details_and_exits_six` |
| Claude 9: invalid other-verb file | WrongVerb without the parse guard | `invalid_unreadable_and_wrong_verb_files_keep_their_ruled_exit_codes` |
| Codex 4: second preparation | extra `with_profile` in `cli/recognize/relation.rs` | `recognition_sends_the_chunks_the_settled_relation_prepared` |
| Claude 7 / Codex 2: skip ceiling | a fourth `#[rustfmt::skip]` | `policy.py`: "crates hold 4 rustfmt::skip attributes and the ceiling is 3" |

Claude 5 (one fallback owner, one cut) is a refactor. Code review verifies it: `SettledRelation::settle` is the only fallback owner, and `reaches_cut` is the only cut. Claude 8 is pinned by the exact Option A test. All 13 are fixed.

## New-round checks

- `Asking::chunks` (`cli/asking/request.rs:88`) keeps recognition's order: send, then read the outcomes, then fold. A logical failure still fails the record before the fold. Every chunk is still prepared before the first send. The only change is one preparation instead of two. The recognize suites and every spec page are green.
- The core stays pure. `core/relate_file.rs` imports only core, serde, sha2, and thiserror. `policy.py` passes, and its only change adds the skip ceiling, which strengthens the check.
- `Meta::new` now keeps `failed_questions`. Every other caller passes `RequestMeta::new`, which starts at 0, so other commands do not change.

## Findings (none blocks)

F1. Follow-up, high: a 255-entity set takes minutes to plan. `engine/prepared_request.rs:44-50` (landed shared splitter) encodes every prefix `1..=remaining` of the question list. With no profile limit, that costs O(q²) encodings, and a same-kind set has q = n(n-1), so the total is O(n⁴). Measured with `relate linked --lines --dry-run`: release build 0.47 s at 40 entities and 7.65 s at 80. Debug build 0.3 s at 20, 5 s at 40, 81 s at 80. Scaling to the allowed 255 gives about 13 minutes of release CPU before the dry-run output or the first send. The same cost turns a regressed "256th entity" refusal row into a test hang instead of a failure. No request is sent, so money is safe. The ticket forbids changing the splitter, so this belongs in an `sdlc/issues/` file. Smallest fix: try the whole remainder first and take it when it passes. When it does not, binary-search the largest passing count, since the limits are monotone.

F2. Follow-up, low: the secrecy marker sits only in the first entity (`tests/backend/secrecy.rs:61-74`). A planted `eprintln!` of the last entity's name (`Acme`) passed all 5 secrecy tests. Smallest fix: put a second marker in the other entity and check for both.

F3. Follow-up, low: relate's `add_meta` (`cli/relate.rs:163-182`) repeats `Aggregate::add_answered` (`cli/recognize.rs:409-437`). On a model mismatch it reports `ModelsDiffer(None)` and drops the two model names that recognition prints. So the record's line "fold replies the same way" is not quite true. Smallest fix: move `Aggregate` beside `Asking` and let relate's `Execution` hold one. That cuts about 25 lines.

F4. Follow-up, cosmetic: the ticket status line (`sdlc/tickets/0088-build-relate-cli.md:9`) does not mention the Codex round. `--either` with `@FILE` (`cli/relate/config.rs:227-229`) exits 2 with the inline-grammar sentence, which names the wrong cause. The backend relate suite does not pin the target-side request wording. Only the core unit test pins it, which is enough.

## Budget: second-agent review of the ceiling raise

The amendment is justified. The growth comes from four things: the command, the Option A and dry-run schemas, the shared secrecy and refusal matrices, and the required file splits. Moved code counts as gross (secrecy routes 212 lines, failure/convert 74, relation tests about 110). Net nonblank growth is 2,518, which equals the ceiling raise. I found about 35 lines of concrete duplication: F3 (about 25), the `dry_run::Relation` fields that copy `RelationRule` and could be `#[serde(flatten)]` (about 5), and the shared `Reading::new` in `input.rs` `document`/`stream` (about 4). That is under 2%. None of it needs to be cut before landing. No file is over 500 lines. No dependency was added. The skip count equals main's 3, and a check now enforces it.

## Paid-request safety and secrecy

Every refusal (grammar, @FILE mix, `--jobs`, the complete-set rules, lines rules, pointers, profile limits) happens before `Folders` and before any send. Planning and preparation of every relation finish before the first send. Dry run returns before building the recorder or the client. A transport, status, or replay failure mid-run returns an error with no partial output. `RelationEntity` `Debug` withholds the name and kind. Failure sentences carry no entity text, only user pointers. I found no key, header, or evidence leak.

## Record claims the diff does not prove

- "Growth … fold replies the same way": off on the mismatch message (F3).
- "715 Rust tests" and the full `test` rung: I did not re-run the full `test` rung. I ran lint, spec, and the focused suites.
