# Accept the fixture lifetime design

Status: design accepted, 2026-09-27. Owner: Codex. No runtime fix is claimed.

The first fresh read-only Codex reviewer requested two corrections to ticket 0202: cancel and join pending backend wait tasks on input close without breaking the existing exit-under-one-second regression, and isolate strict descriptor/thread measurements from parallel test activity. The author added those requirements, the actual executed test count, and one full test run at the 1,024-file limit that also serves as the test rung.

A second fresh read-only `gpt-6-sol` reviewer at medium effort accepted the corrected design at `1046d8cacbf9166c2b93b50ce12e55b54ec6cde0`. It checked the issue, source, regressions, caller ownership boundary, cancellation-aware retirement, and lack of overlap with the database lane. The second review ran through `codex exec` in a read-only session because the collaboration tool refused another thread. Its prompt, runner output, and verdict are retained under `target/codex-reviews/0202-design/` in Codex-3. No builds, tests, paid calls, or runtime edits were part of that review.

The queue owner accepts the ticket within Ian's instruction to settle every item. Ian can overturn its fixture ownership boundary. The implementation must prove late-client isolation while owned, finite synchronous retirement, prompt binary exit, and the full test rung at the normal file limit before the issue is closed.
