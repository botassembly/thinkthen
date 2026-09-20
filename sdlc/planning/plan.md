# Plan

Updated 2026-09-20, after the full-project review began the foundation repairs. ADR 0010 records Ian's earlier rulings. Version one is six commands: `decide`, `choose`, `score`, `filter`, `rank`, and `annotate`. ADR 0015 accepted `find`, and slice 11 builds it after `rank`. `segment` and `report` left the plan and sit on `specification/roadmap.md`. `jq` transforms come before any `report` command. The tool speaks one wire shape, and two variables name the backend: `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`.

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
| 3 | The flat surface on the landed code: `decide QUESTION`, the bare value, `--details`, `--threshold`, exit 0, 1, and 3 on every run, `--quiet`, `--dry-run`, `--profile` | `channels.md`, `threshold.md`, `result.md`, `decide.md` | 01 | Done. Ticket 0005 landed on 2026-09-19 after an independent review. The first live answers came through its binary |
| 4 | The two variables, `sdlc/scripts/live` with the spend limit inside it, and demo 01 recorded live and turned green | `backends.md` | 01 | Done. Ticket 0006 landed on 2026-09-19 after an independent review. Demo 01 is the first green demo |
| 5 | `choose` with `--raw`, then `score`. The adapter keeps every option's probability and the vendor's `confidence` | `choose.md`, `score.md`, `result.md` | 02, 17 | Done. Ticket 0009 landed on 2026-09-19 after an independent review. All three question types are in the shell. The source ceiling is 5,862 lines, and the review judged the rise earned. Ticket 0018 merged how-to 05 into 02 and gave 17 the routing scenario |
| 6 | The live probe: `find` against `rank --top 1`, `confidence` against the winning probability, `score` against trusted levels, option order, an added irrelevant option, and hostile text in the evidence | | | Done. Ticket 0011, and ADR 0014 records what it changed. The cap on questions in one request waits for `annotate` |
| 7 | Records: `--input`, `--lines`, `--jsonl`, `--field` with one pointer or several, the small input rules, and `meta.tool` (ticket 0012). Then `--jobs`, `--cache DIR`, and `choose --options POINTER` (ticket 0013). Order kept, stop at the first failure | `records.md`, `choose.md`, `result.md` | 12, 21 | Done. Tickets 0012 and 0013 landed, and how-tos 04, 12, and 21 are green. ADR 0018 dropped 04, whose folder stays until 16 is green. Ticket 0013 also made one shared connection, the wait a rate limit asks for, the stop when the reader closes the pipe, and the refusal of a control character in a label. Ticket 0024 repaired the interactive loop and bounds dispatched rows that have not printed |
| 7b | Every question option has two homes: `--true` and `--false`, `--option LABEL=DESCRIPTION`, the question file as `@FILE`, the page corrections of the interface audit, and four live checks (ticket 0017) | `question-file.md`, `decide.md`, `choose.md`, `score.md`, `result.md`, `backends.md` | 40, 41 | Done. Ticket 0017 landed on 2026-09-19 after an independent review, and how-tos 40 and 41 are green. Three live checks ran, and none changed the design: the true and false texts sharpen a probability and flipped no answer on forty easy cases, a description per option lifted 58 of 60 picks to 59, and evidence sent as a JSON object matched evidence sent as text at a slightly higher cost, so the text form stays. The fourth check ran after landing: the service accepted one request of 33,663 input tokens, so the larger of the vendor's two published budgets holds |
| 8 | `filter`, then `rank` (ticket 0014) | `filter.md`, `rank.md` | 03, 06, 43 | Done. Ticket 0014 landed on 2026-09-19 after an independent review, and how-tos 03, 06, and 43 are green. A ranked row under `--details` carries `value: null`, because `rank` orders and never selects. How-to 03 absorbs 09, how-to 43 lints a change by meaning and fails the build, and ADR 0018 dropped 42 into one sentence on 12 |
| 9 | `annotate`, with several pointers on `on` and provenance in `meta`. The flagship triage demo lands here with its policy transform | `annotate.md`, `result.md` | 39, 14, 16 | Ticket 0015 builds the command with how-tos 39 and 14 and waits on ticket 0014. A later ticket then writes the flagship with no further code. How-to 39 absorbs 08, and 14 absorbs what 07 held |
| 10a | Metric transforms as folders under `transforms/`, tried on live `decide --details` rows: counts, accuracy, precision, recall, and F1 at a cut, a sweep of cuts, accuracy at coverage, calibration bands, a comparison of two runs by case id, and the cost of a run | | 13, 25, 28 | Done. Ticket 0008 landed on 2026-09-19 after an independent review that recomputed every number. The judge scored accuracy 0.9744 and F1 0.9744 over 39 labeled cases at the default cut. Six transforms are fine as files. The comparison of two runs is the clumsy one. Ticket 0018 merged how-to 38 into 25, and ADR 0018 merged 24 into 41 |
| 10b | First `compare` and `sweep` learn to read any `value`, so one transform serves all three verbs, as ADR 0014 item 6 rules. Then the policy transform, the monitors, the grouped sweep, and the check of the judge against human labels. The slice ends with a verdict on Ian's five outcomes for `report` | | | After slice 9. ADR 0016 folded 26, 34, 35, and 36 into 25, 07, 25, and 13, and ADR 0018 folded 30 and 37 into 14 and 25 |
| 10c | The how-to form for green demos, the check that enforces it, and two how-tos that need only `decide` | | 01, 19, 27 | Done. Ticket 0010 landed on 2026-09-19 after an independent review. Ticket 0018 added the standard of ADR 0016 to the same check and rewrote 19 as a gate that fails closed |
| 11 | `find`, with a `none` option. Its ticket first repeats the comparison with `rank --top 1` on documents of 100 to 250 lines | `find.md` | 15 | Accepted by ADR 0014 and by Ian in ADR 0015. `find.md` is Settled. The probe held on documents of 11 to 14 lines |
| 12 | Remove profiles, the configuration file, `config`, `--profile`, `--adapter`, and `--key-env`. Ian accepted this part of ADR 0010 on 2026-09-19 | `backends.md`, `result.md`, `channels.md` | 10 left | Done. Ticket 0007 landed on 2026-09-19. The source ceiling fell from 4,520 to 4,193 lines |
| 13 | The release pass: every help text, a manual page, and the public switch. The license and the check on every push landed with ticket 0016 | | all, 18 | Waits on Ian's release ruling |

The order can change.

## After version one

The [temporary prospective plan for Bash, Rust, and Python](prospective-bash-rust-python-plan.md) sketches the remaining foundation work and library direction. Tickets are written as work begins, and the library design remains proposed in ADR 0017.

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
| 2026-09-19 | Ticket 0018: the recordings of how-tos 19, 17, and 21 under their new scenarios, ten exchanges | 3,565 |
| 2026-09-19 | Ticket 0017: three live checks on made-up labeled cases and the recordings of how-tos 40 and 41 | 127,295 |
| 2026-09-19 | The token budget check of ticket 0017, one request | 33,663 |
| 2026-09-19 | Ticket 0014: the recordings of how-tos 03, 06, and 43, four runs | 5,744 |
| 2026-09-19 | End-of-day live smoke test: one small call per built command, 27 requests. The script counted 4,178 from the detailed rows, and the coordinator added 3,207 by hand for the runs that printed no usage, because the job did not use `--record` | 7,385 |

Spent so far: 429,118 tokens, which is under two cents. `sdlc/live-tokens` holds the same two numbers for the script to read, and the script adds every call it makes.

## Levers left open

- **The interface audit.** Done. `interface-audit.md` compares the vendor's client and pages with the tool, row by row, and `ten-use-cases.md` judges what ten circulating use cases ask of it. Ticket 0017 closes the gaps: the true and false texts on `decide`, a description per option on `choose`, the question file as `@FILE`, and four live checks. Ticket 0013 landed. The order is now 0017, 0018, and 0019 side by side where their files allow, then 0014, 0015, and `find`.
- **The order after the full-project review.** The foundation repairs in the temporary Bash, Rust, and Python plan now come before the message and page repairs in tickets 0022 and 0023. Ticket 0024 landed first and repaired the record scheduler. The remaining foundation work rejects invalid distributions, reconciles recording with cache behavior, corrects evaluation handling of labels and question changes, and enforces the live spending limit. Tickets are written as each repair begins. Then come 0022, 0023, 0015 for `annotate`, the long-document test and `find`, the flagship how-to 16, the transforms of slice 10b, and the release pass.
- **The how-to standard.** ADR 0016 cuts the list of how-tos from 40 to 27, sets limits that a script checks, and names the seven pages the README features. Ticket 0018 applied it: `sdlc/scripts/demos` measures rules 1, 2, 3, 4, 5, 7, and 8, 05 merged into 02 and 38 into 25, and 19, 17, and 21 took new scenarios. Ian can overturn any line.
- **The list of twenty.** Ian read the 27 and asked for fewer and better. ADR 0018 keeps 20, gives every command and option a page and every use case a page, and names three themes: tune a question file (page 41), grade an assistant's answers with a rubric in place of a second model (page 14), and lint a change by meaning and fail the build (page 43). The front window is 01, 02, 15, 43, 06, 14, 16. `how-to-portfolio-study.md` holds the study, which also found that the tool has every command and option that fifteen software-development uses of a decider need. Ticket 0017 landed pages 40 and 41, and tickets 0014 and 0015 carry the rest. Ticket 0021 rewrote `documentation-plan.md`, `demos/README.md`, and the README's front window to the list of twenty, deleted pages 20 and 24 into 40 and 41, moved page 24's doctored comparison onto `transforms/README.md`, and added `sdlc/scripts/pages` to the `lint` rung so the two lists, the front window, and the folders cannot drift. Ian can overturn any line.
- **The security review.** Closed. Ticket 0013 stopped a run that kept paying after its reader left and refused a control character in a label. Ticket 0019 landed the rest on 2026-09-19 after three review passes: plain `http://` is refused unless the host is this machine, a record over 16 MiB is refused, a user at a terminal is told what the command waits for, the build checks its dependencies offline against a database the install rung fetches, the GitHub actions are pinned by commit, and one shared test proves that no key and no evidence leaks from any command on any failure path. The reviewer found four holes the ticket had not named, and all four are closed: a proxy variable that carried a plain request off the machine with the key in it, the tail of an oversized record sent as a record of its own, error messages that echoed bytes from a file or a reply, and a port past 65535 that was silently dropped. `sdlc/issues/2026-09-19-small-leftovers-from-the-security-ticket.md` holds three small leftovers for the release pass.
- **ADR 0014.** Ian accepted item 1, `find`. The agent decided the other five items from the live probe, and Ian can overturn each: the cut stays on the winning probability, `score` loses its warning about rating, option order stays fixed once a cut is tuned, the pages say that a planted claim moves the judge, and the transforms learn to read a pick before `report` is judged again.
- **The first verdict on `report`.** Ticket 0008's record judges six transforms fine as files. The builder judged the comparison of two runs clumsy enough to earn a command. The reviewer agreed it is the clumsiest and judged that showing built-in transforms, option B of ADR 0012, fixes it far more cheaply. The second verdict comes with slice 10b.
- **ADR 0013.** Written again whole on 2026-09-19, with the design first. Ian accepted its nine items: a question file holds one question, a command reads it as `@FILE`, every structural option has a home on the command line and in the file, the command line beats the file and the file beats the default, a question set holds several, and JSON is the only format. Ticket 0017 builds them. One thing is open: a block of rules in the question set. The agent recommends declining it for version one and writing the flagship how-to once, with its policy as a tested `jq` transform.
- **ADR 0017, libraries for other languages.** Proposed, and it waits for Ian. `sdk-design-study.md` holds the design: the same six names as functions in Python, TypeScript, and Rust, the same question files, rows, and recordings, one backend interface with no vendor's client behind it, and a folder of shared cases that every library must pass. The study recommends plain ports. The agent first agreed, Ian challenged it, and the agent now recommends binding the one Rust core into each language, Python first, after version one of the shell tool. The ADR gives the three reasons. Ticket 0020 moves the vendor's constants into the adapter's module after tickets 0017 and 0019 land.
- **The public release.** Ian ruled MIT and ruled that no talk with the vendor is owed. He ruled on 2026-09-19 that the timing is his alone. No agent raises it, asks about it, or lists it as open.
- **Ideas carried from the design captures.** `sdlc/issues/2026-09-19-ideas-carried-from-the-design-captures.md` lists advice, transform material, and small input rules, each with the slice that picks it up.
- **The review leftovers** in `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md`. Seven still wait, with reasons in `sdlc/records/0005-reshape-decide-to-the-flat-surface.md`.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
