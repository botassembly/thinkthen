# Plan

Updated 2026-09-19, after Ian accepted the flat-verb surface in ADR 0007. Version one is nine commands: `decide`, `choose`, `score`, `filter`, `rank`, `segment`, `annotate`, `report`, and `config`. ADR 0008 (evals) and ADR 0009 (the check against the vendor's how-to pages) are Proposed. Their parts sit in the slices below and are marked. Everything else waits on `specification/roadmap.md`.

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
| 3 | The flat surface on the landed code: `decide QUESTION`, the bare value, `--details`, `--threshold`, exit 0, 1, and 3 without a flag, `--quiet`, `--dry-run`, `--profile`, and the end of the five backend variables | `channels.md`, `threshold.md`, `result.md`, `decide.md` | 01, 09 | Ticket 0005 is ready. The specification and fifteen demos for the whole surface landed on 2026-09-19 after an independent cross-check |
| 4 | `choose` with `--raw`, then `score`. The adapter keeps every option's probability and the vendor's `confidence` (ADR 0009) | `choose.md`, `score.md`, `result.md` | 02, 05 | Next |
| 5 | The `chat-logprobs` adapter against a local server, and the first live judgments. It needs no vendor credits | `backends.md` | 10 | Its section needs settling first |
| 6 | The configuration file, profiles from the file, `THINKTHEN_PROFILE`, `THINKTHEN_CONFIG`, and `config path`, `show`, `check` | `config.md` | 10 | |
| 7 | Records: `--input`, `--lines`, `--jsonl`, `--field` on `decide`, `choose`, and `score`. Order kept, stop at the first failure, resume through the cache, bounded parallel requests. Proposed: several pointers on `--field` (ADR 0008) and `choose --options POINTER` (ADR 0009) | `records.md`, `choose.md` | 04, 12 | |
| 8 | `filter`, then `rank`. Proposed: `find`, one request over up to 255 units (ADR 0009) | `filter.md`, `rank.md`, `find.md` | 03, 06, 15 | |
| 9 | `annotate`. Proposed: several pointers on `on`, provenance in `meta` (ADR 0008), and structured questions in the file (ADR 0009) | `annotate.md`, `result.md` | 07, 08, 14 | |
| 10 | `report`. Proposed: truth labels, a threshold sweep, accuracy at coverage, and a baseline run (ADR 0008, ADR 0009) | `report.md` | 13, 14 | Draft until demos 13 and 14 drive it |
| 11 | `segment`. Proposed: the whole document in one request, and no `--window` (ADR 0009) | `segment.md` | 11 | |
| 12 | The release pass: every help text, a manual page, the license, and the public switch | | all | Waits on Ian's release ruling |

The order can change. Slice 5 sits early on purpose. No live judgment has come back yet, and a local server gives one for free. It also proves the backend seam with a second adapter before more verbs lean on the first.

## After version one

`specification/roadmap.md` lists every held verb and option with the reason it is held and what would bring it in. The rule is ADR 0005: a feature enters when a demo cannot be written without it.

## Live testing budget

Ian set the limit at $20 on 2026-09-19. The vendor charges $0.042 for a million input tokens, so the limit is about 476 million tokens. Spent so far: $0. Every live call adds its tokens here. Every live attempt on 2026-09-19 returned status 402 with the vendor's message "Your organization has no available TypeSafe API credits", and none spent anything. The last attempt used the key in `~/.zshenv.local`, which equals the key the session already held.

## Levers left open

- **ADR 0008 and ADR 0009.** Both are Proposed and wait on Ian's word. Their specification sections stay Draft until then.
- **Open concerns.** `open-concerns.md` ranks the contested points across the ADRs and this plan, with options and a recommendation for each. It waits on Ian.
- **Vendor credits.** The account still answers 402. Ian adds credits or turns on auto-reload at the vendor's billing page. Then a live probe runs first, as `open-concerns.md` lists, and `demos/01-refund-gate/record.sh` records demo 01 against the hosted model.
- **The public release.** Ian rules on the timing, the license, and whether to talk to the vendor first. That is question 10 of `design-study.md`. Questions 6 and 7 of the study are settled by ADR 0007: saved questions are JSON, and a threshold lives on the command line or in the saved file.
- **The review leftovers** in `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md`. Slice 3 rewrites much of the code they touch, so the ticket checks each one.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
