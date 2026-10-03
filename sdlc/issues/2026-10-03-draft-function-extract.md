# Draft function: extract, with a table mode

Status: open. Draft. Filed 2026-10-03 from the docs message "Proxy, terms and function drafts decided" and its addenda, on Ian's product decisions of 2026-10-02. No build work until the experiment reports. Owner: the queue owner.
Kind: idea
When: experiment 0008 and 0009 reports
Milestone: 0.2

`extract` would pull values out of the text. It works like `recognize`: tag the pieces, then classify each span by field, keeping one value per field. `annotate` differs, because its answers come from options written in advance.

Its table mode replaces the separate table tool of the first draft. It adds same-row, same-column and same-cell pair questions, the way `relate` adds relations after `recognize`.

The experiments repository's experiment 0008 tests the form mode, and experiment 0009 tests the table mode.
