# 0099: Port the branch ADRs to main

Status: landed. A fresh review of `c1bf5b84` returned one wording finding (`sdlc/records/0099-code-review.md`). The coordinator accepted the wording fix without another review.

## Result

ADRs 0041, 0042, and 0043 came from tag `surfaces-wave7-final` (`f6a7faea`) to `sdlc/planning/adr/` at their numbers and file names. Each keeps its accepted text and its 2026-09-23 amendments. `git show surfaces-wave7-final:<path> | diff - <path>` shows only the path fixes below plus the new amendment at the end.

- 0041 gains one amendment dated 2026-09-24. The contract crate retires. `CallOptions::deadline_seconds` and `CallOptions::deadline_millis` in `thinkthen` (ticket 0095) own the conversion and the sentinel. Ticket 0095 designs `deadline_seconds` and `deadline_millis`, and ticket 0086 builds them. Once 0086 lands, they refuse a budget above 4,294,967,295 seconds (`u32::MAX`) as `usage` before anything is sent. Until then, main has no public deadline door. This rule binds 0095's design and 0086's tests. The amendment says this keeps the rule of 0041's second 2026-09-23 amendment, which replaced the sentence treating an unrepresentable budget as no deadline.
- 0042 and 0043 each gain one amendment naming the R and PostgreSQL surface tickets (queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md`) as owners. Those tickets have no numbers yet. The rulings are unchanged.

## Path fixes

- 0041: `sdlc/records/surfaces-notes/NOTES-rulings-wave.md` now names the tag.
- 0042: `libraries/r/NOTES.md` now names the tag.
- 0043: `databases/postgresql/NOTES.md` now names the tag.
- The ticket also listed `sdlc/records/0070-…` and `sdlc/records/0074-…`. Both files already exist on main with the tag's exact content, so their paths stay as they are. Record 0075 also exists on main. Every other cited path (`one-line-plan-2026-09-24.md`, ADR 0017) exists on main.

## Checks

- `git diff --cached --check`: clean.
- No private project or customer is named in the three ADRs (grep).
- No fenced block and no paragraph with an unbalanced backtick in any of the three.
- `sdlc/scripts/lint`: exit 0. `sdlc/scripts/spec`: exit 0, demos 21 green, 0 red.
- No code changed. No paid call ran.

## Also in this change

- `sdlc/records/2026-09-24-rereview-near.md` carries the 2026-09-24 confirmation that accepted the design.
- `sdlc/records/0075-the-rulings-that-still-await-adrs.md` gains the tag's update paragraph, trimmed to the ADRs main carries. Items 3 and 4 name branch ADRs 0044 and 0045, which stay at the tag.
