# Draft function: navigate

Status: open. Draft. Filed 2026-10-03 from the docs message "Proxy, terms and function drafts decided" and its addenda, on Ian's product decisions of 2026-10-02. No build work until the experiment reports. Owner: the queue owner.
Kind: idea
When: experiment 0010 reports
Milestone: 0.2

`navigate` would search across many documents: pick documents, cut chunks, find passages, add context, and write a manifest an agent can use. It likely composes `filter`, `rank` and `find`, and builds on the search features issue `2026-10-03-search-features-positions-several-questions-and-grep-style-output.md`. It may be `find --tree`.

It may start as a command-line command. Whether a library function follows is undecided.

The experiments repository's experiment 0010 tests whether walking a tree loses evidence, against an answer key from the docs team's experiment 422 and the QASPER set.
