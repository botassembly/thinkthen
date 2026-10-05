# 0404: Remove the proof spiral

Permanent building rules landed after default-eight concurrency. One whole-change review replaces repeated reviews; substantial fixes involving data loss, credentials, money, memory safety or user-visible correctness may receive another. Dependency review, file caps, ratchets and release approvals remain.

The old plan no longer owns current work. Ticket lessons enforcement is removed; one short landing note owns each ticket. The cleanup ticket now covers machinery removal, record consolidation, behavior-based test names and factual statuses without an inline-test counting rule.

The rules change received one ACCEPT. Focused ticket checks and existing self-tests passed; full tests and lint run on the landing commit. Machinery removal landed after one ACCEPT, a passing site build, shell replay, focused library/SQL replay and release-workflow checks. Record and test consolidation remain in progress.

Removed the binding fingerprint manifest, tree/sample/answer hash tracking, answer-removal narrowing, recipe receipt writer, completed paid-check harness and three receipt files. Existing samples now run and compare outputs.

Kept environment filtering against credential exposure; sample coverage and package names against broken examples/install instructions; recipe visibility and evidence checks against accidental drafts and unsupported claims; process cleanup against hung tools and leaked children; R binary checks against installing the wrong channel; release archive, permission, version and checksum checks against unsafe or broken releases. Spend controls, secrecy, cancellation, memory safety, file caps, ratchets and rehearsal are unchanged.
