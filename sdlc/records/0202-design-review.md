# Accept the fixture lifetime design

Status: design accepted, 2026-09-27. Owner: Codex. No runtime fix is claimed.

The first fresh read-only Codex reviewer requested two corrections to ticket 0202: cancel and join pending backend wait tasks on input close without breaking the existing exit-under-one-second regression, and isolate strict descriptor/thread measurements from parallel test activity. The author added those requirements, the actual executed test count, and one full test run at the 1,024-file limit that also serves as the test rung.

A second fresh read-only `gpt-6-sol` reviewer at medium effort accepted the corrected design at `1046d8cacbf9166c2b93b50ce12e55b54ec6cde0`. It checked the issue, source, regressions, caller ownership boundary, cancellation-aware retirement, and lack of overlap with the database lane. The second review ran through `codex exec` in a read-only session because the collaboration tool refused another thread. Its prompt, runner output, and verdict are retained under `target/codex-reviews/0202-design/` in Codex-3. No builds, tests, paid calls, or runtime edits were part of that review.

The queue owner accepts the ticket within Ian's instruction to settle every item. Ian can overturn its fixture ownership boundary. The implementation must prove late-client isolation while owned, finite synchronous retirement, prompt binary exit, and the full test rung at the normal file limit before the issue is closed.

## File layout amendment

A fresh read-only Sol Medium reviewer accepted `50b5d814` on 2026-09-27. It checked the proposed `lifetime.rs` and `lib.rs` file split against the accepted ownership contract and current source. The existing listener has 492 nonblank lines against the 500-line ceiling. The private module holds the already-required shutdown, tracked connections and workers, and cancellable rendezvous. This changes no retirement, cancellation, ordering or product-surface contract. The builder resumes only after the new exact file claim lands on main.

The queue owner extended the file list to `crates/thinkthen/tests/backend/recognize.rs` for one mechanical cleanup: removing the unused `Barrier` import after the already-approved `stores.rs` migration. A source search found no remaining use in that module or its children. This adds no caller, lifecycle rule, or design decision. The runtime diff remains subject to fresh code review, and the file claim lands before editing.
