# 0404: Remove the proof spiral

Permanent building rules landed after default-eight concurrency. One whole-change review replaces repeated reviews; substantial fixes involving data loss, credentials, money, memory safety or user-visible correctness may receive another. Dependency review, file caps, ratchets and release approvals remain.

The old plan no longer owns current work. Ticket lessons enforcement is removed; one short landing note owns each ticket. The cleanup ticket now covers machinery removal, record consolidation, behavior-based test names and factual statuses without an inline-test counting rule.

The rules change received one ACCEPT. Focused ticket checks and existing self-tests passed; full tests and lint run on the landing commit. Machinery removal landed after one ACCEPT, a passing site build, shell replay, focused library/SQL replay and release-workflow checks. Full tests and lint passed on both the permanent-rules and machinery landings. Record and test consolidation now lands after one whole-change review; it found stale descriptions of the deleted artifacts, which were removed before landing. Focused regression suites, full tests and lint passed on the consolidation landing.

Removed the binding fingerprint manifest, tree/sample/answer hash tracking, answer-removal narrowing, recipe receipt writer, completed paid-check harness and three receipt files. Existing samples now run and compare outputs.

Kept environment filtering against credential exposure; sample coverage and package names against broken examples/install instructions; recipe visibility and evidence checks against accidental drafts and unsupported claims; process cleanup against hung tools and leaked children; R binary checks against installing the wrong channel; release archive, permission, version and checksum checks against unsafe or broken releases. Spend controls, secrecy, cancellation, memory safety, file caps, ratchets and rehearsal are unchanged.

Consolidated current release records into one note per ticket. Kept the readable ten-function matrix and report; removed the 21,550-line cell dump and obsolete designs without code references. Renamed ticket-numbered tests, merged the rank and display cases, and retained every distinct secrecy, refusal, cancellation and error case. Updated eight stale ticket statuses and removed obsolete ticket review histories.

The consolidation branch removes 27,491 lines and adds 1,790 before landing metadata, a net reduction of 25,701 lines. Machinery removal removes 9,928 and adds 203 on its landing, a net reduction of 9,725. Source commits comprise two permanent-rule commits, two machinery cleanup commits and three consolidation/correction commits; landing and integration merges are separate. Default-eight product completion preceded these changes. Its landing also imported older work-in-progress history; those older proof commits were not created by this cleanup.

Integration exposed remaining shared-paragraph bookkeeping: literal paragraph and route exceptions blocked correct backend documentation. The checker now keeps its grid and blog checks without tracking prose fingerprints; the associated tests and stale instructions were removed after one fresh review. The remaining layout fixtures and actual built pages passed. This source change removes another 358 net lines.
