# 0480 — cache-convert-replay-coexistence

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

## Outcome

Preserve acknowledged answers across cache conversion and concurrent use. Replay reads a committed snapshot within the existing cancellation and lock-wait bounds.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, ask 2; engine/store/convert.rs removes the live path while a connection can remain open; fixture.rs reads a live store without the writer wait used by conversion. These are source concerns, not reproduced lost writes. Existing cache_convert tests cover the writer-before-converter order.
- Keeps: Normal cache answers, converted content, strict replay, storage errors, cancellation, existing wait bounds and zero network in replay. 0474 independently owns the probe fix and operation-scoped recognition admission.
- Changes: First reproduce an already-open writer followed by conversion and a subsequent write. Inspect SQLite's moved-file protections before asserting data loss. Add the missing behavior regression; fix only demonstrated loss, false success or ambiguous partial conversion. Choose and review the smallest cross-platform design that preserves successful answers or clearly refuses the competing operation. Add bounded/cancel-aware replay snapshot admission when a writer holds the store; do not retry malformed content. Measure repeated replay scans before proposing another optimization.
- Proof: Owned two-process writer/converter order in both directions; every successful answer remains replayable or the writer clearly fails. Successful conversion durably completes live-file removal through the existing platform-aware directory-sync route; a fresh-process restart replays the retained answers. Do not call a process restart a power-loss simulation or add a crash-proof framework. Replay waiting for an EXCLUSIVE writer succeeds after commit, refuses at the existing deadline and stops on cancellation, with zero sends. Retain corruption refusal and old conversion cases. Fresh durability review before implementation and code review afterward.
- Defers: Broad cache redesign, fairness policy changes and new lock machinery without a demonstrated need. If SQLite already prevents the alleged loss, report the rejected finding and retain the regression rather than invent a fix.
