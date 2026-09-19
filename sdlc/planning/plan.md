# Plan

Updated 2026-09-19, after Ian ruled on `open-concerns.md`. ADR 0010 records the rulings. Version one is six commands: `decide`, `choose`, `score`, `filter`, `rank`, and `annotate`. ADR 0015 accepted `find`, and slice 11 builds it after `rank`. `segment` and `report` left the plan and sit on `specification/roadmap.md`. `jq` transforms come before any `report` command. The tool speaks one wire shape, and two variables name the backend: `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`.

## How a slice moves

1. The specification page for the slice is Settled. Changing a Settled page takes an ADR.
2. The slice's demos are written or rewritten against that page. They start red. Anything a demo cannot say cleanly goes into `demos/FINDINGS.md` and back into the page.
3. A ticket under `sdlc/tickets/` cites the sections it builds. One Opus agent builds it in its own worktree, red test first.
4. A second Opus agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency. The review names what it checked.
5. The steering agent rebases, runs all four rungs, checks for attribution lines, merges, and pushes.
6. The slice's demos turn green by replaying a recording. No gate touches a network. A green demo takes the how-to form of ADR 0011, because the demo, the how-to, and the test are one file.

A slice is done when its how-tos in `documentation-plan.md` are green, the four rungs pass on main, and this page says so.

## Slices

A row records what its slice delivered on the day. A later slice that removed something says so in its own row.

| Slice | Delivers | Pages | Demos | State |
| --- | --- | --- | --- | --- |
| 0 | The workspace, both crates, the lint tables, the ratchet, the ladder, `thinkthen --version` | | | Done |
| 1 | `decide if` on the first surface: the types, the result, the `systemone` adapter, exit codes, the plan document | | | Done. Tickets 0001 to 0003 |
| 2 | `--record` and `--replay` over one content-addressed folder | `recording.md` | | Done. Ticket 0004 |
| 3 | The flat surface on the landed code: `decide QUESTION`, the bare value, `--details`, `--threshold`, exit 0, 1, and 3 on every run, `--quiet`, `--dry-run`, `--profile` | `channels.md`, `threshold.md`, `result.md`, `decide.md` | 01, 09 | Done. Ticket 0005 landed on 2026-09-19 after an independent review. The first live answers came through its binary |
| 4 | The two variables, `sdlc/scripts/live` with the spend limit inside it, and demo 01 recorded live and turned green | `backends.md` | 01 | Done. Ticket 0006 landed on 2026-09-19 after an independent review. Demo 01 is the first green demo |
| 5 | `choose` with `--raw`, then `score`. The adapter keeps every option's probability and the vendor's `confidence` | `choose.md`, `score.md`, `result.md` | 02, 05, 17, 20 | Done. Ticket 0009 landed on 2026-09-19 after an independent review. All three question types are in the shell. The source ceiling is 5,862 lines, and the review judged the rise earned |
| 6 | The live probe: `find` against `rank --top 1`, `confidence` against the winning probability, `score` against trusted levels, option order, an added irrelevant option, and hostile text in the evidence | | | Done. Ticket 0011, and ADR 0014 records what it changed. The cap on questions in one request waits for `annotate` |
| 7 | Records: `--input`, `--lines`, `--jsonl`, `--field` with one pointer or several, the small input rules, and `meta.tool` (ticket 0012). Then `--jobs`, `--cache DIR`, and `choose --options POINTER` (ticket 0013). Order kept, stop at the first failure | `records.md`, `choose.md`, `result.md` | 04, 12, 21 | Done. Tickets 0012 and 0013 landed, and how-tos 04, 12, and 21 are green. Ticket 0013 also made one shared connection, the wait a rate limit asks for, the stop when the reader closes the pipe, and the refusal of a control character in a label |
| 7b | Every question option has two homes: `--true` and `--false`, `--option LABEL=DESCRIPTION`, the question file as `@FILE`, the page corrections of the interface audit, and four live checks (ticket 0017) | `question-file.md`, `decide.md`, `choose.md`, `score.md`, `result.md`, `backends.md` | two new how-tos | Ticket 0017 is ready |
| 8 | `filter`, then `rank` (ticket 0014) | `filter.md`, `rank.md` | 03, 06, 09 | Ticket 0014 is written and waits on ticket 0017 |
| 9 | `annotate`, with several pointers on `on` and provenance in `meta`. The flagship triage demo lands here with its policy transform | `annotate.md`, `result.md` | 07, 08, 16, 22, 23, 29, 31, 32 | Ticket 0015 builds the command with how-tos 07 and 08 and waits on ticket 0014. A later ticket then writes the flagship and the first eval how-tos with no further code |
| 10a | Metric transforms as folders under `transforms/`, tried on live `decide --details` rows: counts, accuracy, precision, recall, and F1 at a cut, a sweep of cuts, accuracy at coverage, calibration bands, a comparison of two runs by case id, and the cost of a run | | 13, 24, 25, 28, 38 | Done. Ticket 0008 landed on 2026-09-19 after an independent review that recomputed every number. The judge scored accuracy 0.9744 and F1 0.9744 over 39 labeled cases at the default cut. Six transforms are fine as files. The comparison of two runs is the clumsy one |
| 10b | First `compare` and `sweep` learn to read any `value`, so one transform serves all three verbs, as ADR 0014 item 6 rules. Then the policy transform, the monitors, the grouped sweep, and the check of the judge against human labels. The slice ends with a verdict on Ian's five outcomes for `report` | | 14, 16, 26, 30, 34, 35, 36, 37 | After slice 9 |
| 10c | The how-to form for green demos, the check that enforces it, and two how-tos that need only `decide` | | 01, 19, 27 | Done. Ticket 0010 landed on 2026-09-19 after an independent review. Three how-tos are green |
| 11 | `find`, with a `none` option. Its ticket first repeats the comparison with `rank --top 1` on documents of 100 to 250 lines | `find.md` | 15 | Accepted by ADR 0014 and by Ian in ADR 0015. `find.md` is Settled. The probe held on documents of 11 to 14 lines |
| 12 | Remove profiles, the configuration file, `config`, `--profile`, `--adapter`, and `--key-env`. Ian accepted this part of ADR 0010 on 2026-09-19 | `backends.md`, `result.md`, `channels.md` | 10 left | Done. Ticket 0007 landed on 2026-09-19. The source ceiling fell from 4,520 to 4,193 lines |
| 13 | The release pass: every help text, a manual page, and the public switch. The license and the check on every push landed with ticket 0016 | | all, 18 | Waits on Ian's release ruling |

The order can change.

## After version one

`specification/roadmap.md` lists every held verb and option with the reason it is held and what would bring it in. The rule is ADR 0005: a feature enters when a demo cannot be written without it.

## Live testing budget

Ian set the limit at $20 on 2026-09-19. The vendor charges $0.042 for a million input tokens, so the limit is about 476 million tokens. The key lives in `THINKTHEN_API_KEY`. Every live call adds its tokens here.

| Date | What | Input tokens |
| --- | --- | --- |
| 2026-09-19 | Four `decide` calls by hand through the ticket 0005 binary. ADR 0010 holds the answers | 1,185 |
| 2026-09-19 | Two `decide` calls through `sdlc/scripts/live`, recording demo 01 | 626 |
| 2026-09-19 | Ticket 0010: the recordings of how-tos 19 and 27 | 961 |
| 2026-09-19 | Ticket 0008: forty labeled cases judged twice, 80 calls | 23,848 |
| 2026-09-19 | Ticket 0009: one probe of each new type and the recordings of how-tos 02, 05, 17, and 20 | 6,222 |
| 2026-09-19 | Ticket 0011: the six live probes, 639 recorded requests | 214,997 |
| 2026-09-19 | Ticket 0012: the recording of how-to 04, five record-mode calls | 1,453 |
| 2026-09-19 | Ticket 0013: the recordings of how-tos 12 and 21, seven exchanges | 2,174 |

Spent so far: 251,466 tokens, which is about one cent. `sdlc/live-tokens` holds the same two numbers for the script to read, and the script adds every call it makes.

## Levers left open

- **The interface audit.** Done. `interface-audit.md` compares the vendor's client and pages with the tool, row by row, and `ten-use-cases.md` judges what ten circulating use cases ask of it. Ticket 0017 closes the gaps: the true and false texts on `decide`, a description per option on `choose`, the question file as `@FILE`, and four live checks. Ticket 0013 landed. The order is now 0017, 0018, and 0019 side by side where their files allow, then 0014, 0015, and `find`.
- **The how-to standard.** ADR 0016 cuts the list of how-tos from 40 to 27, sets limits that a script checks, and names the seven pages the README features. Ticket 0018 applies it. Ian can overturn any line.
- **The security review.** Three findings had to change before release: plain `http://` to another machine, a run that kept paying after its reader left, and control characters in a label. Ticket 0013 closed the last two, and ticket 0019 carries the first with the smaller findings.
- **ADR 0014.** Ian accepted item 1, `find`. The agent decided the other five items from the live probe, and Ian can overturn each: the cut stays on the winning probability, `score` loses its warning about rating, option order stays fixed once a cut is tuned, the pages say that a planted claim moves the judge, and the transforms learn to read a pick before `report` is judged again.
- **The first verdict on `report`.** Ticket 0008's record judges six transforms fine as files. The builder judged the comparison of two runs clumsy enough to earn a command. The reviewer agreed it is the clumsiest and judged that showing built-in transforms, option B of ADR 0012, fixes it far more cheaply. The second verdict comes with slice 10b.
- **ADR 0013.** Ian accepted the amendment on 2026-09-19: a question file holds one question, a command reads it as `@FILE`, every structural option has a home on the command line and in the file, and the command line beats the file, which beats the default. Ticket 0017 builds it. One thing is still his to rule on: whether a closed block of rules in the question file is tested by writing the triage how-to twice. The rest of the ADR changes nothing: the question file is the one file the tool runs directly, a runtime lowers its stages to it, and JSON stays the only format.
- **The public release.** Ian ruled MIT and ruled that no talk with the vendor is owed. The timing is still his.
- **Ideas carried from the design captures.** `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` lists advice, transform material, and small input rules, each with the slice that picks it up.
- **The review leftovers** in `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md`. Seven still wait, with reasons in `sdlc/records/0005-reshape-decide-to-the-flat-surface.md`.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
