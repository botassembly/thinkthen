# `rank --threshold P` keeps only records at or above a probability

Status: closed.
Promoted: 0510
Kind: idea
When: 0.2 work starts on main
Milestone: 0.2

One command would select and order records, so a user would not chain two commands.

`rank --threshold P` would keep only records whose probability is at least P, then order them. Today users chain `filter Q | rank Q`. Release QA measured this on checkpoint `checkpoint/surfaces/2026-10-01-2`. The second pass sends 0 requests when both commands share a cache, so the chain costs one pass.

Ian ruled on 2026-10-01 that a rank cutoff is a 0.2 idea. In 0.1, `rank` orders records and never selects them.

## Reconciliation, 2026-10-08

The 2026-10-01 Ian ruling retains this 0.2 proposal. The current settled [rank contract](../../specification/rank.md) defines pure ordering and refuses thresholds. The PM must reconcile that conflict before an implementation ticket changes behavior. Records cleanup cannot revoke Ian’s ruling or claim the proposed feature is implemented.

Ian approved the cutoff for 0.2 on 2026-10-09. [Ticket 0510](../tickets/0510-rank-probability-threshold.md) owns the specification and ADR amendment, CLI and Rust implementation, and shared-cache behavior checks. It follows 0475 and 0476.
