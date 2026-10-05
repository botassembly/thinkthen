# 0404: Remove the proof spiral

Permanent building rules landed after default-eight concurrency. One whole-change review replaces repeated reviews; substantial fixes involving data loss, credentials, money, memory safety or user-visible correctness may receive another. Dependency review, file caps, ratchets and release approvals remain.

The old plan no longer owns current work. Ticket lessons enforcement is removed; one short landing note owns each ticket. The cleanup ticket now covers machinery removal, record consolidation, behavior-based test names and factual statuses without an inline-test counting rule.

The rules change received one ACCEPT. Focused ticket checks and existing self-tests passed; full tests and lint run on the landing commit. Machinery, record and test cleanup remain in progress.
