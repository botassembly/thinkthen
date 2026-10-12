# 0558: Cut Rust source well below the ceiling

Status: OPEN.

Milestone: 0.2

Depends on: 0550, 0551, 0552, 0553, 0554, 0556

Reviews: revision c4b747f05cf35f05d5ca2a376cb003a96820ac07, accept

Reviews: revision e7f771bac41dfda0199a06e11b36b03af85a7981, accept

Reviews: revision 52d652f, accept

Reviews: revision 6712593, accept

Reviews: revision c2626ca, accept

## Outcome

The nonblank Rust line count that `sdlc/scripts/ratchet.mjs` measures falls to 187,000 or fewer, and `sdlc/ratchet.json` falls with it. The cut comes from deleting dead code, unpublished duplicate methods and repeated test setup. Public behavior and contract coverage stay the same.

## Evidence

- Starts from: the measured count and the reviews below.
  - Combined lanes 0550 to 0556 measure 190,417 on `da353c648` against a ceiling of 189,500. 0549 removes 37 more.
  - The 2026-10-11 adversarial review found that the ceiling was raised eight times without paying any raise back.
  - A read-only survey of `da353c648` on 2026-10-11 found the items under Changes, and a ticket review checked them. The line counts below are approximate.
- Keeps: public behavior and contract coverage, as follows.
  - Every public, documented Rust method that shipped in 0.1.2. Twelve of the wrapper methods below shipped: `choose_many`, `choose_many_with`, `decide_many`, `decide_many_with`, `details_many`, `details_many_with`, `filter`, `filter_with`, `score_many`, `score_many_with`, `tag_many` and `tag_many_with`. `#[doc(hidden)]` host items sit outside the reviewed public inventory under 0359, so this ticket may delete them when nothing calls them.
  - Every binding, CLI, MCP and database case.
  - `rustfmt.toml` as it is. Changing format settings lowers the count without removing logic, so it does not count toward this outcome.
  - Every distinct regression, under the AGENTS.md rule. A test leaves only when another test checks the same behavior.
  - Unit tests in `src` that no outside test covers, including `http/tests.rs`, `mcp/tests/`, the schema golden tests and the fork, process and Windows tests.
- Changes: these deletions and helpers.
  - Delete dead host code in `crates/thinkthen-host/src/source/`: `execute`, `execute_prepared`, `dispatch` and everything only they reach, including all of `relate_output.rs`. That is about 510 lines. Keep `parse`, `Selection`, `Selection::read`, `usage`, `defect` and `request_source`, which `libraries/c` uses at `plan.rs:92` and elsewhere. Delete the out-of-date `allow(dead_code)` reasons at `mod.rs:33,76,91,408`.
  - Delete the hidden `Engine::annotate_each_with` and its helpers, `public/bulk/annotation.rs:280-432`, about 148 lines. Nothing calls it since DuckDB moved to a `Request` session. Keep `rendered`, `Annotating`, `record` and `admit_declarations`.
  - Cut the 58 public `Engine` wrapper methods that did not ship in 0.1.2. They are the `*_complete_with`, `*_records_complete_with`, `try_*` and image `*_input*` families in `public/complete/`, `public/images/calls.rs`, `public/images/many.rs`, `public/bulk.rs` and `public/bulk/details.rs`.
    - Delete each method that only tests call, and move its tests to `Request`.
    - Make each method that crate code calls `pub(crate)`, or inline it at its single caller. The internal callers include `public/engine.rs:265,296,323,392`, `public/frame/typed_batches.rs:27-183`, `public/bulk/complete.rs:72,145`, `public/rank_set.rs:94`, `public/recognize.rs:597` and `public/frame/inputs.rs:34`.
    - Move the C library's calls to `try_filter_with` and `try_details_many_with` at `libraries/c/src/current/execute.rs:221,235` onto `Request`, the way 0552 moved its other entry points. Move its calls to `try_annotate_with`, `try_rank_with` and `try_find_with` the same way, through existing `Request` APIs with no new `RequestRows` variants. If one of them cannot move without a new public API, it stays public, and a Progress line names it and the missing API.
    - Move `libraries/rust/examples/recognize_observe.rs` to `Request`. Update `sdlc/scripts/inventory` and the specs that name the removed methods.
  - In `crates/thinkthen/tests/backend`, replace repeated setup with shared helpers in `harness/mod.rs`, about 2,500 lines in all:
    - one `exits(&output, n)` helper for the 157 copies of the six-line exit-code check, with an optional message for the copies that carry their own, such as `address.rs:420`;
    - one annotate runner for the 48 repeated `spawn(&["annotate", …])` calls and the three local `run()` copies;
    - one copy each of `folder()`, `printed`, `said`, `text` and `code`.
  - Delete the tests in `decide_edge.rs` and `choose_and_score_edge.rs` that repeat rows in `refusals.rs` and `secrecy.rs`. First add the four rows the tables lack: `--raw --quiet`, `--threshold 90`, the reversed band `0.9:0.1` and empty evidence. Keep the plan output, model flag, removed environment variable, estimated-input, plan shape and range tests in those files.
  - In the other integration tests, add helpers for the repeated `Engine::builder()` chains, `RecordInput` literals and session result reads, about 950 lines. A builder helper takes the key as an argument, because some tests check that their own key never leaks.
  - Keep every unit test in `src`. This includes the store privacy test in `src/engine/store/tests.rs`, since no library-only test asserts the store modes. Keep `src/core/question_file/tests.rs`, the usage tests in `src/engine/usage/tests.rs` and `src/core/records/tests.rs`. Their outside covers in `tests/backend` build only with the `cli` feature (`tests/backend/main.rs:8`), and the code they check builds without it. Idea 0545 handles them after 0.2.
  - If these changes leave the count above 187,000, take the remaining cut from the test-helper families the survey rated medium confidence: stderr string compares, closed output pipes, profile runs, retry tables, cache races and hand-built launchers in `tests/backend`.
- Proof: these checks.
  - The ratchet passes at 187,000 or fewer, and `sdlc/ratchet.json` equals the measured total.
  - Rust public cases, CLI cases, installed C checks, Rust Clippy in both library configurations and policy pass.
  - `cargo check --workspace --all-targets` passes.
  - A search finds none of the deleted public wrapper names in public signatures outside `sdlc/`.
  - The refusal and secrecy tables hold every row that the deleted edge tests checked.
- Defers: removing the twelve wrapper methods that shipped in 0.1.2, and DuckDB's `details_many_recoverable_with` path, which needs DuckDB on a `Request` session first. Idea 0545 keeps the remaining test cuts after 0.2.

PM amendment, 2026-10-11. `rank_set_complete_with` and `relate_records_complete_with` stay public, with their tests. A find wrapper stays too if `find_admission.rs` checks the same contract through it. These wrappers are the only API that hands a caller's own non-Clone, non-Serialize originals back unchanged. Two tests check that: `complete_set_rank_keeps_all_members_originals_partial_usage_and_independent_turns_order` in `native_complete/set_rank.rs`, and `source_relations_keep_nonserializable_nonclone_originals_and_refuse_mixed_sources_before_sending` in `source_relations.rs`. Request feeds expose `QuestionInput` originals. `RequestRows` has no rank, rank-set, find or relate variant. So these wrappers carry a contract that `Request` lacks. They are not duplicates. Every other wrapper follows the Changes rule. This ticket adds no `RequestRows` variants and no whole-set execution support. Idea 0559 carries that API work after 0.2. The 187,000 target stands, and the named fallback cuts cover the lines these wrappers keep.

PM amendment, 2026-10-11, second. `find_complete_with` joins the kept set, for `find_snapshots_each_caller_evidence_once_and_restores_originals` (`native_complete/find_admission.rs:195`). The developer may instead move each ownership test to a documented 0.1.2 find, rank or relate method and then delete the unpublished wrapper. A move counts only if the 0.1.2 method takes the same caller-owned, non-Clone, non-Serialize originals and the moved test keeps every ownership assertion: the `Rc` strong count, the marker, and exactly one engine snapshot of the evidence. The `Request` cases keep the facts and value assertions. A wrapper stays when its test cannot move intact.

PM amendment, 2026-10-11, third. This rules on `rank_set_complete_with` and `relate_records_complete_with` only.
- If crate code outside tests calls a method's implementation, make that implementation `pub(crate)`. Move its exact ownership test, with every assertion, into a library-only test inside the crate. Then delete the public method.
- If only tests call it, the method stays public with its test, as the first amendment says.
- A Progress line names which rule each method took and the production caller that justified the first rule.
- Applied after the caller audit:
  - `rank_set_complete_with` becomes private. `Engine::rank_set_with` calls it.
  - `relate_records_complete_with` and `try_relate_records_complete_with` are deleted. The `Request` relate path calls the private `execute_relate_records_complete_with`, and the relate ownership test moves there unchanged.
  - These two ownership tests are the only exception to moving the affected integration tests onto public `Request`. They may use a library-test loopback backend inside the crate, which code review checks.

PM amendment, 2026-10-11, fourth. The third amendment's rule now covers every wrapper whose ownership test cannot move intact to `Request` or to a 0.1.2 method. The first such test found after that amendment was `complete_envelopes_keep_owned_originals_and_actual_facts_without_changing_legacy_call` on `decide_many_complete_with`. The second was `complete_find_serialization_retains_the_original_payload_selected_by_native_execution` on `find_complete_with`.
- If crate code outside tests calls the private implementation under the wrapper, the test moves unchanged onto that implementation as a library-only test inside the crate, and the public wrapper is deleted. `find_complete_with` takes this rule, because `try_find_with` calls the private fallible complete primitive.
- If nothing outside tests reaches the implementation, the wrapper stays public with its test unchanged. `decide_many_complete_with` takes this rule unless the developer finds a production caller of its implementation.
- No test assertion changes. No fake `Call`, new collector or `RequestRows` variant is added. A Progress line names each wrapper, the rule it took and the production caller when there is one.

PM amendment, 2026-10-11, fifth. The fourth amendment's rule applies only when a test cannot move intact. `complete_envelopes_keep_owned_originals_and_actual_facts_without_changing_legacy_call` passes unchanged through the existing public `RequestRows::Decisions(batch).into_call()`, so it moves there. `decide_many_complete_with` is then deleted under the original Changes rule. The fourth amendment's keep ruling for it no longer applies.

PM amendment, 2026-10-11, sixth. Target ruling. The cleanup closes at 187,611, measured at `f3ab2eb93`. That is 2,806 lines below the combined 190,417 start. No further cuts are required to meet the outcome. The ceiling stays at 187,611 and does not rise.
- The 0551 and 0553 corrective fixes pay for their own lines. Read `Request`, `RequestInput`, `RequestItem`, `RequestQuestion` and the options through one private decode helper that takes the classifier. Report one sentence for an unknown function: "unknown function" or "invalid request function", whichever the existing tests already pin. Stop generating the PHP envelope classes and `REQUEST_VERSION` that nothing calls since `fa83d9035`. The core and PHP ratchets then pass at or under their unchanged ceilings.
- Close four gaps found by a fresh review of the cleanup:
  - Give the internal `native == true` branch of `validate` in `src/public/request/native_feed.rs` one test for each sentence it reports. These are the find and relate per-record control sentences and "record seed spans are admitted only for recognize". The shipped 0.1.2 methods and the C library still reach that branch.
  - Restore a check that each filtered row was kept in `tests/public_batches/interactive.rs`, for example `result().value() == true`.
  - Keep one direct test on public `try_find_with` for the count cap, the unread tail and the 16 MiB limit, matching `try_rank_with` in the same loop.
  - Remove the repeated door-loop calls that run the same call under labels naming deleted wrappers. Use `expect_err` for those calls.
- No other test or assertion changes.

## Progress

- 2026-10-11 started
- 2026-10-11 next: The accepted host and shared-helper cuts plus reviewed C projections are combined on the cleanup branch at 187998 lines, with further wrapper migrations and approved helper cuts needed before whole-ticket proof.
- 2026-10-11 next: The host and shared-helper cuts plus reviewed C projections are combined at 187998 lines, with further wrapper migrations, approved helper cuts and a fresh whole-code review still required.
- 2026-10-11 next: PM approved the moved Relate test using the existing public Request error sentence while retaining its error kind, refusal, zero-send and facts assertions, with no shared routing or diagnostic change.
- 2026-10-11 next: PM approved an admitted first record in the moved rank-set reader-error fixture and a separate schema refusal, preserving every kind, position, facts and zero-send assertion without Request routing changes.
- 2026-10-11 next: The fourth amendment moves the unchanged find serialization test to the private complete primitive called by try_find_with and deletes find_complete_with; decide_many_complete_with stays public with its unchanged ownership test unless a production caller justifies an internal move.
- 2026-10-11 next: Caller audit assigns find_complete_with to an unchanged library test on try_find_complete_with because try_find_with calls that primitive; decide_many_complete_with has no production caller of its concrete implementation and stays public with its ownership test unchanged.
- 2026-10-11 next: The fifth amendment supersedes the decide keep ruling: its unchanged opaque envelope test passes through existing RequestRows::Decisions and Batch::into_call, so decide_many_complete_with is deleted under the original Changes rule.
- 2026-10-11 next: The remaining C assessment keeps try_annotate_with public because Request lacks compatibility group-failure controls, and try_rank_with and try_find_with public because Request lacks generic originals preserving C raw JSON bytes, under amendment 785ad39; image wrapper cleanup proceeds separately.
- 2026-10-11 next: find_complete_with was deleted after its unchanged serialization test moved onto private try_find_complete_with used by shipped try_find_with, rank_set_complete_with retains its unchanged ownership test on its private implementation used by shipped rank_set_with, and decide_many_complete_with was deleted after its unchanged envelope test moved onto RequestRows::Decisions(batch).into_call under the fifth amendment.
- 2026-10-11 next: Integrated source checkpoint e4d69c82b passed focused checks at a 187792-line ratchet, and the identical baseline classifier measured 80093 product lines against 80643 before; annotation and image cleanup continue to reach the approved ceiling.
- 2026-10-11 next: The committed wrapper checkpoint measures 187751 without a ceiling raise, and remaining wrapper deletions plus approved integration-fixture helper cuts must reach 187000 before exact-revision full test and lint and whole-code review.
- 2026-10-11 next: Checked checkpoint a3ba06ec measures exactly 187705 with matching ratchet, preserving eager aggregation and passing both core configurations and real Polars checks; remaining wrappers and approved setup cuts must reach 187000 before exact-revision full gates.
- 2026-10-11 next: Checked checkpoint 9697db269 measures 187626; finish named wrapper deletions and safe cuts without changing distinct checks or formatting, then if still above 187000 stop and record the final total, cuts and retained items with reasons for the target ruling.
- 2026-10-11 next: Stop cleanup for the target ruling: independently measured f3ab2eb93 is 187611 with the ceiling lowered to match; cuts remove dead host code, unpublished annotation and named image, decision, filter, details, rank, dynamic choose and recognition wrappers plus duplicate fixture setup; retained private production primitives preserve unchanged ownership checks, public C annotation/rank/find routes preserve group failures and raw originals, relate_complete_with preserves typed bounded admission and recoverable members without an existing equivalent carrier, and distinct parser, cancellation, secrecy, range and diagnostic checks remain intact.
- 2026-10-11 next: Safe cleanup remains stopped at f3ab2eb93 with 187611 lines and its cuts and retained checks recorded above; separately reviewed corrective source fa83d9035 measures 187637 including shared R and PHP decoding, with core ceiling unchanged at 187611 and PHP 1916 against unchanged 1902, so no ceiling increase or closure is accepted pending the target ruling.
- 2026-10-12 next: Frozen 6712593f6 measures core source and ceiling at 187606 with all restored coverage and the approved API inventory passing routine test and lint; fresh review accepts every cleanup amendment and affected limit, and landing awaits the same-source rc.12 campaign.
