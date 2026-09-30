# issues/

Problems found and filed. An issue is not a ticket and authorizes no work.

- This folder holds open issues only. Each file starts with a `Status:` line.
- `closed/` holds every closed issue. Its status line names the ticket, commit, or later issue that closed it.
- Landing a ticket closes the issues it settles. The lander writes the closing status line and moves the file to `closed/` in the landing commit.
- Merge before you add. A new problem that shares a fix with an open issue becomes an item in that issue.
- An idea is an issue with `Kind: idea` and `When: <trigger>`, such as `after the 0.1 release`, and one sentence on what it would buy.
- `../planning/issue-priorities-2026-09-30.md` orders this folder. It replaces the backlog report of 2026-09-25.

## Debt

Ian approved this convention on 2026-09-30, through the coordinator. Debt is a kind of issue, not a separate register. Debt works today but costs later: a workaround, a known-failing marker, a skipped or inverted check, a flaky test, or duplicated code.

- A debt issue's header adds `Kind: debt` and `Pay when: <trigger>`, such as `before 0.1`, `Zig fixes its linker`, `0304 slice 5 lands` or `a user needs it`. One sentence says what keeping it risks.
- The header also carries `Debt: NNN`, the next number at capture, and `Severity: high`, `medium` or `low`, the cost of keeping it. Paying it adds `Paid: YYYY-MM-DD` when the issue moves to `closed/`. An issue merged into another keeps its number and gets no `Paid:` line.
- A builder who finds debt mid-build files a short debt issue with file and line, then keeps going.
- At landing, a ticket's Defers line that no ticket or issue owns becomes a debt issue.
- Code holds no `TODO` or `FIXME` comment. A comment that marks debt names its issue path under `sdlc/issues/`. `policy.py` refuses any other such comment it recognizes, except in one file the Flutter tool generates.
- Find debt with `grep -l '^Kind: debt' sdlc/issues/*.md`.
