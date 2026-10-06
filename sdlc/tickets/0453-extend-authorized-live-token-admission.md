# 0453: Extend authorized live token admission without losing charges

Status: landed.
Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh High read-only review.
Risk: High, spending authority and concurrent ledger updates.

## Outcome

The owner can add approved token admission through the live helper while preserving all existing charges. This supports independently approved experiments; it does not make 0036 or 0037 a 0.2 release dependency.

## Evidence

- Starts from: Mail `2026-10-06-experiments-0036-and-0037-need-token-admission-under-their-approved-caps.md`, asks 1–4. Current status is 476,000,000 allowed, 475,786,939 charged. Encoded image estimates exceed the old remainder while provider billing is much smaller. Ian approved the separate experiment caps and later authorized $20 total for builder work.
- Keeps: One Git-common-directory authority, every prior charge, exclusive locking, private modes, durable append, partial-write refusal, no refunds, and manual authorization for paid jobs. Never edit, copy, reset or replace the ledger by hand.
- Changes: Add `live --add-tokens N`, requiring positive canonical integer admission under the same lock. Append an extension row and validate extensions alongside charges; preserve prior rows. Support bounded signed-64-bit token totals so prepared image estimates do not hit the old nine-digit ceiling. The control command reads no key and launches no job. Owner checks approved monetary caps and prepared request reserves; admission is not new spending permission.
- Proof: Extend the existing live fixtures for preserved charges/remainder, repeated and concurrent extension/charge operations, invalid/overflow amounts, lock/mode and partial-write failures. Reuse current secrecy and no-job checks. One fresh High ticket review and code review; no receipts or self-verification tooling.
- Defers: Image-billing estimator redesign, refunds, credential changes, new benchmark cohorts and actual paid runs. Add no admission until the owner checks the signed run’s full reserve and remaining monetary authority. The completed data-deck second pass no longer needs its old requested raise.
