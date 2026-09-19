# Plan

Updated 2026-09-19, after Ian ruled on `open-concerns.md`. ADR 0010 records the rulings. Version one is six commands: `decide`, `choose`, `score`, `filter`, `rank`, and `annotate`. `find` waits on a live measurement. `segment` and `report` left the plan and sit on `specification/roadmap.md`. `jq` recipes come before any `report` command. The tool speaks one wire shape, and two variables name the backend: `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`.

## How a slice moves

1. The specification page for the slice is Settled. Changing a Settled page takes an ADR.
2. The slice's demos are written or rewritten against that page. They start red. Anything a demo cannot say cleanly goes into `demos/FINDINGS.md` and back into the page.
3. A ticket under `sdlc/tickets/` cites the sections it builds. One Opus agent builds it in its own worktree, red test first.
4. A second Opus agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency. The review names what it checked.
5. The steering agent rebases, runs all four rungs, checks for attribution lines, merges, and pushes.
6. The slice's demos turn green by replaying a recording. No gate touches a network. A green demo takes the how-to form of ADR 0011, because the demo, the how-to, and the test are one file.

A slice is done when its how-tos in `documentation-plan.md` are green, the four rungs pass on main, and this page says so.

## Slices

| Slice | Delivers | Pages | Demos | State |
| --- | --- | --- | --- | --- |
| 0 | The workspace, both crates, the lint tables, the ratchet, the ladder, `thinkthen --version` | | | Done |
| 1 | `decide if` on the first surface: the types, the result, the `systemone` adapter, exit codes, the plan document | | | Done. Tickets 0001 to 0003 |
| 2 | `--record` and `--replay` over one content-addressed folder | `recording.md` | | Done. Ticket 0004 |
| 3 | The flat surface on the landed code: `decide QUESTION`, the bare value, `--details`, `--threshold`, exit 0, 1, and 3 on every run, `--quiet`, `--dry-run`, `--profile` | `channels.md`, `threshold.md`, `result.md`, `decide.md` | 01, 09 | Done. Ticket 0005 landed on 2026-09-19 after an independent review. The first live answers came through its binary |
| 4 | The two variables, `sdlc/scripts/live` with the spend limit inside it, and demo 01 recorded live and turned green | `backends.md` | 01 | Done. Ticket 0006 landed on 2026-09-19 after an independent review. Demo 01 is the first green demo |
| 5 | `choose` with `--raw`, then `score`. The adapter keeps every option's probability and the vendor's `confidence` | `choose.md`, `score.md`, `result.md` | 02, 05, 17, 20 | Ticket 0009 is ready. It runs after ticket 0007 |
| 6 | The live probe: the real shape of all three answers, the cap on questions in one request, `confidence` against the winning probability, `score` against its measured weakness, whether option order or an added irrelevant option moves a `choose` answer, and whether hostile text inside the evidence moves a judgment | | | After slice 5, through the live script |
| 7 | Records: `--input`, `--lines`, `--jsonl`, `--field` with one pointer or several, `choose --options POINTER`, and `--cache DIR`. Order kept, stop at the first failure, bounded parallel requests | `records.md`, `choose.md` | 04, 12, 21 | |
| 8 | `filter`, then `rank` | `filter.md`, `rank.md` | 03, 06, 09 | |
| 9 | `annotate`, with several pointers on `on` and provenance in `meta`. The flagship triage demo lands here with its policy recipe | `annotate.md`, `result.md` | 07, 08, 16, 22, 23 | |
| 10a | Metric recipes as folders under `recipes/`, tried on live `decide --details` rows: counts, accuracy, precision, recall, and F1 at a cut, a sweep of cuts, accuracy at coverage, a comparison of two runs by case id, and the cost of a run | | 13, 14, 24, 28 | Ticket 0008 is ready. It runs after ticket 0007 |
| 10b | The policy recipe, the monitors, the grouped sweep, and the check of the judge against human labels. The slice ends with a verdict on Ian's five outcomes for `report` and on ADR 0012 | | 16, 25, 26 | After slice 9 |
| 10c | The how-to form for green demos, the check that enforces it, and two how-tos that need only `decide` | | 01, 19, 27 | Ticket 0010 is ready. It runs after ticket 0007 |
| 11 | `find`, if a live run shows that it picks as well as `rank --top 1` | `find.md` | 15 | Draft. Waits on the measurement |
| 12 | Remove profiles, the configuration file, `config`, `--profile`, `--adapter`, and `--key-env`. Ian accepted this part of ADR 0010 on 2026-09-19 | `backends.md`, `result.md`, `channels.md` | 10 leaves | Ticket 0007 is ready, and it runs next because it shrinks the code that every later slice touches |
| 13 | The release pass: every help text, a manual page, the license, and the public switch | | all, 18 | Waits on Ian's release ruling |

The order can change.

## After version one

`specification/roadmap.md` lists every held verb and option with the reason it is held and what would bring it in. The rule is ADR 0005: a feature enters when a demo cannot be written without it.

## Live testing budget

Ian set the limit at $20 on 2026-09-19. The vendor charges $0.042 for a million input tokens, so the limit is about 476 million tokens. The key lives in `THINKTHEN_API_KEY`. Every live call adds its tokens here.

| Date | What | Input tokens |
| --- | --- | --- |
| 2026-09-19 | Four `decide` calls by hand through the ticket 0005 binary. ADR 0010 holds the answers | 1,185 |
| 2026-09-19 | Two `decide` calls through `sdlc/scripts/live`, recording demo 01 | 626 |

Spent so far: 1,811 tokens, which is less than one cent. `sdlc/live-tokens` holds the same two numbers for the script to read, and the script adds every call it makes.

## Levers left open

- **ADR 0012.** Proposed. A recipe is a folder, the tool at most lists and shows built-in recipes, and it never runs them. Ian rules after the recipes have been tried as files.

- **The public release.** Ian rules on the timing, the license, and whether to talk to the vendor first. That is question 10 of `design-study.md`.
- **Ideas carried from the design captures.** `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` lists advice, recipe material, and small input rules, each with the slice that picks it up.
- **The review leftovers** in `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md`. Seven still wait, with reasons in `sdlc/records/0005-reshape-decide-to-the-flat-surface.md`.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
