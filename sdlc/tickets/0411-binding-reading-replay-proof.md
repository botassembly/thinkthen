# 0411: Reread stored answers under changed rules on every surface

Status: in progress. Existing SDK adoption landed in 0431 at e2641d87c. The other family checklists retain their required changed-reading cache/replay cases; close when all required surfaces pass.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

All named SDK, SQL and dataframe routes reread stored answers under each free reading rule with zero additional calls.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Extend existing canonical saved cases and public settings adapters; function contracts without a reading cut are explicit, never skipped.
- Proof: Change cuts with identical wire identity; pin output/counts for native details, member rules, entity/relation dependencies, frames and R index conversion.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0408 owns missing details; 0432 owns common enforcement; no new fingerprint or store-verification framework.

## Answer identity when rereading

0450 requires a different answer ID for a different normalized reading of stored observations, even if the bare value stays equal; restoring the original rule restores its ID. Full typed, SQL and frame consumers test this with zero sends. IDs do not change merely because retrieval origin, host index spelling or call ID changes.

Adopt the ordered image and storage contracts from 0447, 0448 and 0444. Saved multi-image decide and choose cases must reread changed cuts with unchanged request identity, expected results and zero sends on every full-result surface. Restoring a cut restores the answer ID. Image score explicitly has no reading cut; its saved ordered images and full result replay with zero sends. Image implementation stays with its existing owners.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.

The Rust/Python/JavaScript/TypeScript/Ruby/R family checklist is complete under [0431](../records/0431-complete-existing-typed-sdks.md), including its seven installed/archive qualifications. This ticket remains open for the other SDK, SQL and dataframe surfaces; 0431 does not close their adoption.
