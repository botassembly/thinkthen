# Recipe: Navigate many documents

Status: open. Publication stays later under Ian’s 2026-10-05 ruling.
Kind: recipe
Milestone: later
Owner: the queue owner
Slug: navigate-many-documents
When: later measurements support publication

## Disposition

Held-out transcript search found 13/29 passages against broad search’s 15/29, with unequal reading lengths. The earlier result did not generalize. Navigation and transcript-search publication stay later; no general recall advantage is promised.

## Original draft history

### Draft function: navigate

Historical disposition: open draft. Filed 2026-10-03 from the docs message "Proxy, terms and function drafts decided" and its addenda, on Ian's product decisions of 2026-10-02. No build work until the experiment reports. Owner: the queue owner.
Historical kind: idea
Historical trigger: experiment 0010 reports
Historical milestone: later

`navigate` would search across many documents: pick documents, cut chunks, find passages, add context, and write a manifest an agent can use. It likely composes `filter`, `rank` and `find`, and builds on the search features issue `2026-10-03-search-features-positions-several-questions-and-grep-style-output.md`. It may be `find --tree`.

It may start as a command-line command. Whether a library function follows is undecided.

The experiments repository's experiment 0010 tests whether walking a tree loses evidence, against an answer key from the docs team's experiment 422 and the QASPER set.
