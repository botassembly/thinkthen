# `rank --top` holds every record it will never print

Status: Open. Filed 2026-09-26 by the coordinator after a read-only review of how each command uses memory on long files.

## Problem

`rank` holds every record and its score until the input ends (`specification/rank.md`, "What it prints"; `cli/schedule.rs`). That is needed without `--top`, because every record prints. With `--top N`, only N records print. The tool still holds all of them, so memory grows with the file even when the caller asked for ten rows.

## Proposed outcome

With `--top N`, `rank` keeps only the N best rows seen so far. A row that cannot reach the top N is dropped as soon as its score arrives. Ties keep input order, as today, so a later row with an equal score never displaces an earlier one. Output and exit codes stay byte-identical to today. Requests stay the same, because every record must still be judged.

## Proof a ticket would give

- An edge-case table over ties at the Nth place, `--top` larger than the input, and `--top 1`, each pinning the same standard output as today's build.
- A plant that keeps every row fails no output test, so the ticket needs a check that bounds held rows at the real boundary without a test-only hook, or it states why output parity is the whole contract.

## Not in scope

Ranking without `--top` still holds every record. A bound there needs the records written somewhere other than memory, and no demo needs that yet.
