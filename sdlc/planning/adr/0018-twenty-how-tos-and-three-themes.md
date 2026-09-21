# ADR 0018: Twenty how-tos, and three themes they must show well

- Status: Decided by the agent on 2026-09-19 at Ian's instruction. He can overturn any line
- Date: 2026-09-19

ADR 0016 cut the how-tos from 40 to 27 and set a standard that a script now checks. Ian read the 27 and said it is still a lot. He wants the best pages only: together they show every command and every option, each answers a use case people care about, and no two teach the same thing. He named three themes that must be shown well. A second agent read a survey of fifteen launch-week projects that put a decider into the software-development process, a label-tuning tool, his earlier captures on evals, and the list of 27. `sdlc/planning/how-to-portfolio-study.md` holds its report, with one table that gives every command and option a page and one that gives every use case a page. This ADR replaces the list and the front window of ADR 0016. The standard of ADR 0016 holds unchanged.

## Does the tool have what these use cases need

Yes. Every one of the fifteen software-development patterns is covered today or when tickets 0014, 0015, and 0017 land. None asks for a command, an option, or a file format the specification does not hold. Two limits stand, and both are already on the record: five of the fifteen live inside a Python or TypeScript program and need the library of ADR 0017, and a supervisor that judges every step of a fast loop needs more decisions a second than a shell command gives. Every new idea the survey raised was declined under the rule of ADR 0005, including a special exit code for `filter`, because one shell line does it.

## The three themes

1. **Tune a question file, page 41.** The question with its settings is the arguments to a function, and tuning means changing those arguments against labeled cases. The page judges made-up labeled expense claims with `decide @FILE`, scores the run, changes the file and never the command, compares the two runs, and then runs the same file as the gate. It ends with one sentence: an automatic tuner drives the same loop by rewriting one step, and every round is traceable by its digest. The page names no outside project. A one-line conversion from the label-tuning tool's saved result is honest for a yes/no question alone, and the cut must still be swept, so that line stays in `sdlc/issues/` and out of the page.
2. **Grade an assistant's answers with a rubric in place of a second model, page 14.** The evidence is a record that holds the assistant's request, its context, and its reply. The rubric is a question set: three narrow yes/no checks, one pick for the kind of failure, and one placement for severity. `annotate` grades every case in one pass, the metric transforms report, and a few human labels check the judge itself, with page 25 for the judge's accuracy. This page is in the front window.
3. **Lint a change by meaning and fail the build, page 43.** One record per changed hunk goes in, `filter @FILE` keeps the hunks that break a written convention, and the build fails when any are kept. The kept hunks are the report an author or a coding agent reads. No other page shows many records in and a failing build out. This page is in the front window.

## The list of 20

| Group | Page | Owns |
| --- | --- | --- |
| Start here | 01 Gate a script step on a yes/no answer | `decide`, `--quiet`, the exit code |
| | 19 Gate a risky command and fail closed | the band, all four outcomes, `case $?` |
| | 27 Test a script with no network | `--record`, `--replay` |
| | 18 Point the tool at another server and compare two deciders | `--url`, `--model`, `--timeout`, `--max-retries` |
| Gates and branches | 02 Branch on a label with `choose` and `case` | `choose`, `--raw`, `--option LABEL=DESCRIPTION` |
| | 40 Say what yes and no mean | `--true`, `--false`, "not stated" against "false" |
| | 17 Route a request by how hard it is | `score`, levels, `jq -e` |
| | 21 Pick the next action from a list that changes | `choose --options POINTER` |
| | 41 Tune a question file and use the same file in the gate | `@FILE`, precedence, the comparison transform |
| Many records | 03 Keep only the records that match a meaning | `filter`, `--field`, `--dry-run` as proof of what leaves the machine |
| | 43 Lint a change by meaning and fail the build | `filter` as a build gate |
| | 06 Put the best matches first | `rank --top`, several pointers |
| | 12 Resume a long run that stopped | `--cache`, `--jobs`, `--input` |
| | 15 Find the line that answers a question | `find`, `--none`, `--lines` |
| Many questions at once | 39 Screen one message for several hazards | a question set, three question types in one request |
| | 16 Build a triage pipeline that drafts, blocks, or asks a person | the flagship: `annotate`, a policy transform, audit rows |
| Evals | 14 Grade an assistant's answers with a rubric | `annotate` over records, `on`, the count transform |
| | 13 Pick a threshold from labeled cases | the sweep and band transforms |
| | 25 Check the judge against human labels | the score and calibration transforms, hostile cases |
| | 28 Know what a run cost | the cost transform, `meta.usage` |

Eight pages leave the list of 27 and page 43 enters, which is how 27 becomes 20. Every idea lands somewhere: 20 into 40, 24 into 41, 07 into 14 and 16, 30 into 14 as one sentence, 23 into 27 and 14, 37 into 25, 04 into 19, 16, and 13, and 42 into 12 as one sentence. No number is reused.

**A green page leaves only when the page that absorbs it is green.** Pages 04, 20, and 24 are green today. They stay in the repository until 16, 40, and 41 turn green, so no lesson is ever missing.

## The front window

The README features seven pages in this order. Page 16 leads because it is the flagship used by the README, site, and talk. The remaining pages move from the simplest use to the supporting details.

| # | How to | Shows |
| --- | --- | --- |
| 16 | Build a triage pipeline that drafts, blocks, or asks a person | the flagship |
| 01 | Gate a script step on a yes/no answer | `decide`, the exit code |
| 02 | Branch on a label with `choose` and `case` | `choose` |
| 15 | Find the line that answers a question | `find`, and a clean "nothing fits" |
| 43 | Lint a change by meaning and fail the build | `filter`, a question file, a build gate |
| 06 | Put the best matches first | `rank --top` |
| 14 | Grade an assistant's answers with a rubric | `annotate`, evals with no second model, all three question types |

The study's window left `find` out and showed `filter` twice. This one shows each command once. Page 43 takes the `filter` seat from page 03, because the readers of this README are developers and a failing build is the use they will recognize first. Page 03 stays in the list as the plain introduction to `filter`, and page 43 links to it. Pages 13 and 41 sit just outside the window, and pages 14 and 43 link to them.

## Consequences

- Ticket 0017 writes pages 40 and 41 to the designs above, so that 20 and 24 can leave afterward.
- Ticket 0014 turns 03, 06, and 43 green. Page 03 absorbs 09, as ADR 0016 ruled.
- Ticket 0015 turns 39 and 14 green. Page 39 absorbs 08, and page 14 absorbs what ADR 0016 gave to 07.
- Ticket 0021 rewrites `documentation-plan.md`, `demos/README.md`, and the README's front window to this ADR, and it removes each green page whose absorbing page is green by then.
- The flagship, page 16, keeps its own slice after `annotate`.

## Amendment, 2026-09-19, by ticket 0021

The count above read "seven pages leave" and then named eight: 20, 24, 07, 30, 23, 37, 04, and 42. Eight leave and page 43 enters, so the list of 27 becomes 20. The list itself was always right. Ticket 0021 also found that two of the merges have not happened yet: 42's sentence is not on page 12, and page 25 holds no hostile case for 37. `documentation-plan.md` says so on each row, and a later ticket writes them.

## Amendment, 2026-09-21, by ticket 0041

Ian ruled that page 16 is the page the talk, site, and README lead with. It moves from seventh to first without changing the seven pages.
