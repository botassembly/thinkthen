# Adopt the new pm features Quick Fix

Lane claude-1, branch `ticket/quick-fix-pm-adoption`, off origin/main `4e880cdf6`. The pm team asked thinkthen to adopt its mailroom declaration, milestones, records-reader fixes and tolerance rules, and to test them. This Quick Fix changes only `sdlc/pm.json` and adds this record.

## Changes to `sdlc/pm.json`

- `mailroom` names the mailroom checkout `/home/ian/workspace/repos/sdlc` and the mail name `thinkthen`. pm documents `mailroom.repo` as a local checkout root, not a remote (`src/commands/mailroom-identity.ts`).
- `milestones` points at `planning/milestones.md` with the values `0.1`, `0.2` and `later`.
- The `ready` and `in progress` mappings go. pm now tallies both states natively (`TICKET_STATUSES` in `src/records/status.ts`), and `pm counts` shows 7 ready and 17 in progress where it showed 26 open.
- The five `**`-prefixed mappings go. pm strips emphasis before it reads a status token (`statusToken`). Status, issue and ticket counts did not change when they went.
- `todoExcludes` skips `libraries/dart/flutter/example/linux/flutter/**`. Flutter generates that CMake file, and its TODO was the only `todo-comment` finding.

The `landed`, `done`, `resolved` and `deferred` mappings stay. No thinkthen script or workflow calls pm, so the new exit contract needs no script change.

## Numbers at `4e880cdf6`

| Reading | Expected | Actual |
| --- | --- | --- |
| Open reviews (`pm daily`) | about 67 | 67 |
| `ticket-landed` findings | about 190 | 195, of which 13 are false (pm bug 2 below); 182 true |
| `issue-resolution` findings | about 66 | 67 |
| Due debt (`pm release`) | 0 | none unpaid |
| Next free ticket | above 0388 | 0389 |
| `status` and `counts` agree | yes | yes: tickets 2 open, 7 ready, 17 in progress, 323 complete, 1 withdrawn; issues 18 open, 374 closed, 3 non-issue, 1 unknown |

Total findings fell from 275 to 274 when the Flutter TODO was excluded.

## What works

- `pm inbox` lists all 50 messages in `inbox/thinkthen` on origin/main. `pm replies` lists our 31 sent messages.
- `pm send` wrote and pushed one genuine message to the pm team: botassembly/sdlc `b26ca21`, `inbox/pm/2026-10-02-thinkthen-thinkthen-adoption-test-what-works-and-the-bugs-.md`.
- `pm milestone`, `pm counts --milestone 0.1` and `pm release` read the outcome and blockers from `milestones.md` and roll up the `Milestone:` lines. The 0.1 open count, 17, equals the 17 open items the file lists.
- `pm release` reports no rc tag, the latest checkpoint `checkpoint/surfaces/2026-10-01-3`, no due debt and the 0.1 rollup.

## Bugs mailed to the pm team

The message above holds exact commands and output for each.

1. `ticket-landed` reads trailers from local `main`, which thinkthen never moves. It sits 272 commits behind origin/main, so tickets 0370 to 0388 that landed with trailers get false findings.
2. From inside a lane worktree, `pm lanes` and `pm next` report that lane missing.
3. `pm daily` shows every lane busy when no lock is declared, while `pm next` shows them free.
4. `pm daily` misses Quick Fix and slice landings.
5. `pm lint FAMILY` prints every records finding, not just the named family.
6. One-line `From: ... To: ... Sent ...` headers give a garbled sender and an unknown date, and `pm ping` joins the senders into one line of about 600 characters.
7. A lone `Answer:` line followed by a numbered list is read as more asks.
8. Ping, counts, inbox and lint mailroom count open mail four different ways.
9. `pm milestone` leaves `Kind: idea` issues out of the open count and lists no items or exit criteria.
10. Smaller items: `ticket-name` flags `sdlc/tickets/README.md`; `pm idea list --due` wording; missing final newlines; daily's Blocked view ignores ready and in progress tickets; `pm lanes` detached and upstream display; `pm send` has no dry run and exits 1 on an undeclared mailroom.

## Can `pm milestone` replace `milestones.md`?

No. pm reads `milestones.md` for each milestone's outcome and blockers, so it depends on the file. It prints neither the item lists nor the exit criteria nor the path to 0.1. Its 0.2 open count is 8 where the file lists 13 items, because it drops the five ideas. The `grep` sentence in the file's preamble can now point at `pm milestone` and `pm counts --milestone`. The coordinator decides whether to change that wording.

## Review and integration

A fresh code review accepted `32225fd0e`. It checked each key against pm's source, the status split, the glob's reach and this record. No build or gate ran, because the change touches only records configuration.
