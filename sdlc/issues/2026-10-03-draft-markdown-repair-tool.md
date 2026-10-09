# Draft command-line tool: Markdown repair

Status: open. Draft. Filed 2026-10-03 from the docs message "Proxy, terms and function drafts decided" and its addenda, on Ian's product decisions of 2026-10-02. No build work until the experiment reports. Owner: the queue owner.
Kind: idea
When: experiment 0013 reports
Milestone: later

Ian wants a Markdown repair tool in the command line, as a tool and not a library function. It would recover the structure of a damaged Markdown document in two passes.

The experiments repository's experiment 0013 tests the two-pass structure recovery.

The tool judges structure and never runs anything, so it keeps the rule that `thinkthen` judges and never acts. It writes only the file the user names.

Moved to milestone later on 2026-10-09 under the 0.2 scope freeze in [the binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). Only bugs and TCGA blockers enter 0.2.
