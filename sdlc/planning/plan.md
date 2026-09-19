# Plan

Updated 2026-09-19, after Ian accepted the flat-verb surface in ADR 0007. Version one is nine commands: `decide`, `choose`, `score`, `filter`, `rank`, `segment`, `annotate`, `report`, and `config`. Everything else waits on `specification/roadmap.md`.

## How a slice moves

1. The specification page for the slice is Settled. Changing a Settled page takes an ADR.
2. The slice's demos are written or rewritten against that page. They start red. Anything a demo cannot say cleanly goes into `demos/FINDINGS.md` and back into the page.
3. A ticket under `sdlc/tickets/` cites the sections it builds. One Opus agent builds it in its own worktree, red test first.
4. A second Opus agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency. The review names what it checked.
5. The steering agent rebases, runs all four rungs, checks for attribution lines, merges, and pushes.
6. The slice's demos turn green by replaying a recording. No gate touches a network.

A slice is done when its demos are green, the four rungs pass on main, and this page says so.

## Slices

| Slice | Delivers | Pages | Demos | State |
| --- | --- | --- | --- | --- |
| 0 | The workspace, both crates, the lint tables, the ratchet, the ladder, `thinkthen --version` | | | Done |
| 1 | `decide if` on the first surface: the types, the result, the `systemone` adapter, exit codes, the plan document | | | Done. Tickets 0001 to 0003. Slice 3 reshapes it |
| 2 | `--record` and `--replay` over one content-addressed folder | `recording.md` | | Done. Ticket 0004 |
| 3 | The flat surface on the landed code: `decide QUESTION`, the bare value, `--details`, `--threshold`, exit 0, 1, and 3 without a flag, `--quiet`, `--dry-run`, `--profile`, and the end of the five backend variables | `channels.md`, `threshold.md`, `result.md`, `decide.md` | 01, 09 | Ticket 0005 is written |
| 4 | `choose` with `--raw`, then `score` | `choose.md`, `score.md` | 02, 05 | Next |
| 5 | The `chat-logprobs` adapter against a local server, and the first live judgments. It needs no vendor credits | `backends.md` | 10 | Its section needs settling first |
| 6 | The configuration file, profiles from the file, `THINKTHEN_PROFILE`, `THINKTHEN_CONFIG`, and `config path`, `show`, `check` | `config.md` | 10 | |
| 7 | Records: `--input`, `--lines`, `--jsonl`, `--field` on `decide`, `choose`, and `score`. Order kept, stop at the first failure, resume through the cache, bounded parallel requests | `records.md` | 04, 12 | |
| 8 | `filter`, then `rank` | `filter.md`, `rank.md` | 03, 06 | |
| 9 | `annotate` | `annotate.md` | 07, 08 | |
| 10 | `report` | `report.md` | 13 | Draft until demo 13 drives it |
| 11 | `segment` | `segment.md` | 11 | |
| 12 | The release pass: every help text, a manual page, the license, and the public switch | | all | Waits on Ian's release ruling |

The order can change. Slice 5 sits early on purpose. No live judgment has come back yet, and a local server gives one for free. It also proves the backend seam with a second adapter before more verbs lean on the first.

## After version one

`specification/roadmap.md` lists every held verb and option with the reason it is held and what would bring it in. The rule is ADR 0005: a feature enters when a demo cannot be written without it.

## Live testing budget

Ian granted more budget for paid calls on 2026-09-18. The cap for this repository is 20,000,000 input tokens. Spent so far: 0. Every live call adds its tokens here. The first live calls on 2026-09-19 returned a billing error, because the vendor account has no credits, and they spent nothing.

## Levers left open

- **Vendor credits.** Ian adds them at the vendor's billing page. Then `demos/01-refund-gate/record.sh` records demo 01 against the hosted model.
- **The public release.** Ian rules on the timing, the license, and whether to talk to the vendor first. That is question 10 of `design-study.md`. Questions 6 and 7 of the study are settled by ADR 0007: saved questions are JSON, and a threshold lives on the command line or in the saved file.
- **The review leftovers** in `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md`. Slice 3 rewrites much of the code they touch, so the ticket checks each one.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
