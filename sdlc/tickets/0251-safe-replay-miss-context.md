---
flow: design
priority: 251
opens: sdlc/tickets/0251-safe-replay-miss-context.md sdlc/records/0251-replay-context-preflight.md
---

# 0251: Reconcile safe replay miss context with register 102

Status: COMPLETE.

Opened as: 2026-10-11. Fresh independent High review accepted `0327a13d`; the coordinator adopts the stated disagreed-approach disposition. No new runtime implementation is claimed. [Preflight](../records/0251-replay-context-preflight.md) uses source `3918bba2` and the original experiment 284/102 criterion; root owns the row and count change.

## Outcome and proposed design

Preserve 0221's safe strict-replay diagnostic: the stopped record or packed request range, annotate group ordinal/member count, and exact entry digest, with `Local`/exit 5, zero sends, read-only replay, and existing secrecy. Explicitly **decline** printing a caller-controlled question filename, path or member. The original register 102 set-name criterion remains **unmet**; the absence of a top-level schema name does not satisfy it. The final row status is **nonissue / disagreed approach**, not `done` or already fully fixed. This is a disposition, not a new runtime carrier or claim that the original literal outcome shipped. A later materially justified safe naming design could reopen the row.

## Evidence

- **Starts from:** original register 102 in local experiment 284's `102-replay-miss-does-not-say-why.md` asks for record N, question-set name when a file named it, and exact digest. The current plan row retracts the later invented component-by-component digest criterion.
- **Keeps:** accepted 0221 at reviewed `4a740fdc`, `Failure::ReplayMiss` and its `Stopped`/`BatchFailed` wrappers, command-owned closed source labels, exact digest, Local/exit 5, no send, replay-folder immutability and credential/path/evidence secrecy.
- **Changes:** the coordinator proposes to decline the unsafe literal set-name output while retaining 0221's useful safe context. `QuestionSet::parse` has named members but no top-level set name; that fact does **not** fulfill the original name criterion. No source, test, schema, public API or page change is proposed.
- **Proof:** retained `tests/backend/recordings/replay_context.rs` executes seven request shapes and an exact existing-folder miss. Its first-request cases have empty stdout, exit 5 and zero sends; streaming after a completed record may retain its ordered stdout prefix. No new runtime check, provider call or broad suite is needed for this disposition.
- **Defers:** the unmet literal question-set name, any new safe naming mechanism, component-level digest diagnosis, recognize-stage or relate-chunk provenance. These are not described as implemented; a later materially justified design may reopen safe naming.

## Routing and retained contract

Coordinator disposition after High review: **nonissue / disagreed approach**. Credit 0221 only for the position/range, group context and digest that it actually implemented. The literal set-name criterion stays visibly unmet because printing caller-controlled text is declined for secrecy; do not call the row `done` or already fully fixed. Do not reopen 0221's safe labels or infer a unique question from a grouped request. Root records this status in the item table. No source claim or build follows from this disposition.

## What the build taught us

No build has begun. Preparation found that 0221 carries position and digest, while its remaining component-diagnosis sentence is not an original criterion. High review caught two mistakes in the first note: an absent top-level schema name does not satisfy the original filename criterion, and a streamed annotate miss after record 1 can retain completed stdout. The coordinator adopts the disagreed-approach disposition without a new test or behavior.
