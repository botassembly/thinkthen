# 0506: Judge caller-supplied recognition spans

The continuation starts from the unverified source checkpoint `ea6ce289b583e31515ea914ff643a788f4500e7f`. Fresh code review accepts product revision `2010023e85dae35b51f17ab807fe30113abf8b31`. Final checked revision `665ad78f21ff8b313d306275ab3769c3029983bb` preserves that product source. Later changes strengthen the nested evidence fixture, reuse an existing consumer fixture setup, declare the twelve approved public items, and reconcile measured source ceilings. Main integrations add reviewed documentation, instructions and contract metadata.

`core/recognize/seeds.rs` defines Unicode scalar span admission and safe per-record selection. The recognition engine merges seeds with decoded proposals, deduplicates identical bounds, retains overlaps, and extends each group's evidence through its maximum end. Supplied kinds remain unconfirmed. Classification asks the complete declared set and decline option, or internal ENTITY and decline without declared kinds. Boundary requests and the existing final strength cut remain unchanged.

The public recognition carrier, RecordInput, RecordReading and typed Request share seed controls and fallback admission. CLI `--seed-spans-field` selects record proposals and includes them in plan bounds. The generated Request schema and recognition/settings specifications describe the settled fields. Existing foreign carrier constructors supply absent seeds; host control adoption remains with the migration tickets.

## Checks and corrections

The earlier restricted execution context could not start the required memory scope or complete compilation and loopback checks. No passing tests belong to that checkpoint. The continuation confirmed both a working user memory scope and loopback bind before compiling.

The first actual compile exits 101 because an automated constructor update added `seed_spans` to DescriptionBuilder. Remove that unrelated field. Add the separately coordinated absent seed field to the C legacy RequestItem constructor. Both fixes precede the accepted product revision.

The first generated-schema check intentionally exits 101 after writing the new schema. Its normal rerun and the two companion strict-decoder/schema contract cases all pass, exit 0. Initial outside-in fixture runs also fail for fixture mistakes: a native recognition request used a text selector instead of an authored definition, an assertion included edge questions among kind questions, and recording was combined with the mutually exclusive cache option. Correct the fixtures without changing product behavior. The seven retained seed cases then pass, exit 0.

Those cases prove native Request/CLI outgoing-body parity, emoji and combining offsets, leading excluded scalars, exact piece boundaries, duplicate and overlapping proposals, supplied-kind correction/refusal, projected evidence, fallback/empty/null selection, invalid pointers/kinds/conflicts, zero loopback sends on refusal, unchanged boundary bytes, classification independent of the final cut, no-send plan bounds, and cached/recorded classification identity. The strengthened nested fixture places a later short proposal after a long proposal and checks the actual outgoing evidence; its affected rerun passes, exit 0.

`target/0506-lint.log` exits 1 when a test-only commit changes HEAD during the archived-release workflow fixture. The fixture expected its planted missing-pair refusal and instead receives `checkout differs from resolved SHA`. `target/0506-workflows-rerun.log` then passes at stable HEAD, exit 0, with 113/113 cases. The successful earlier policy, secrecy, child-environment, ABI and version checks are retained rather than repeated for metadata changes.

`target/0506-lint-tail.log` exits 1 at an omitted Rust binding ceiling. Later registry checks expose the other affected ceilings, including TypeScript and Ruby counting shared R input source. Measure all nine binding ratchets and reconcile only the approved absent-field additions. `target/0506-clippy.log` exits 101 because those additions push one existing consumer test to 92/90 lines. Reuse its existing local record setup instead of adding a suppression; the affected test passes, exit 0. `target/0506-inventory.log` exits 1 for twelve approved declarations absent from the ticket. The coordinator adds the exact declarations through the existing ticket block.

`target/0506-lint-final.log` completes the remaining lint stages, exit 0: the full binding registry, source ceilings, offline dependency checks and source plants, formatting, workspace Clippy with warnings denied, documentation with warnings denied, and inventory with 1,827 declarations and four refused plants. Together with the retained prefix and workflow rerun, every applicable lint stage passes. This is staged recovery of the initial nonzero lint run, not a claim that the original script returned zero.

`target/0506-test.log` exits 0: 1,818 workspace tests, 351 library-only tests and 23 external consumer tests pass, along with doctests, supporting fixture/script checks and all 19 binding smokes. Configured skips remain separate: 27 workspace, four library-only and three consumer entries. Two existing unused-import warnings appear in the library-only test build. The slow image and native consumer cases finish without changing their timeouts or narrowing coverage.

`target/0506-spec.log` exits 0: settings reports zero failures, executable specifications and transform/recognition/relation fixtures pass, probe harness checks pass, and all 24 green demos pass. Existing executable-page skips remain visible.

Heavy checks use user memory scopes with two build jobs and no swap allowance. Final measurements are 23 GB of target output, 7.4 GB of libraries and 5.3 GB of databases; free disk is 74 GB. The extra C surface build variant was not started under the coordinator's resource ruling; the complete functional gate includes its ordinary C smoke. No paid backend, release action, workflow dispatch, publication or other-machine check ran.

## What the build taught us

Seed classification must remain distinct from the boundary strength used by the final cut. Checking only final entities would miss successful classification of a proposal that step 1 rejected.

A nested proposal fixture must end with a later short proposal and inspect actual evidence. Checking whether the request contains the final word can pass because a question itself quotes that word even when the evidence is truncated.

A shared carrier field reaches constructor-heavy tests and several binding ceilings. Some bindings count the same shared R source. Measure all affected limits together and reuse local fixture setup when defaults push a function past its line limit.

Finish source fixes and commits before checks that inspect Git history. Advancing HEAD during an archived-release fixture changes its rejection reason and invalidates that proof, even when the commit changes only tests.

Schema generation can intentionally fail while writing its result. Record that exit separately and run the unchanged comparison afterward. A source checkpoint or a restricted attempt supplies no compilation evidence.
