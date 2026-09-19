# ADR 0016: Fewer how-tos, each held to a standard

- Status: Decided by the agent on 2026-09-19 at Ian's instruction. He can overturn any line. Ticket 0018 applied it. ADR 0018 replaces the list and the front window, and the standard holds
- Date: 2026-09-19

Ian asked that the demos be the best they can be: no two that teach the same thing, each chosen and upgraded for impact, with simplicity and understandability first and the power of the tool still shown. A second agent read all 21 pages as a newcomer with thirty seconds, beside the documentation plan, the ten use cases, and the interface audit. The plan held 40 how-tos. This ADR keeps 27.

## The standard for a how-to

ADR 0011 gives the form. These limits come on top of it, and `sdlc/scripts/demos` checks every one that a script can check.

1. A page holds at most 120 lines and 900 words. A longer page is two pages, or it still holds an argument.
2. The first fenced block starts by line 20, and its first `thinkthen` line sits within five lines of the block's start. Nothing is set up before the first result: no `mktemp`, no `trap`, no heredoc.
3. A page holds at most six runnable blocks, and every block asserts something.
4. A page teaches one command. A second command appears only when the title names the contrast.
5. A page holds one idea and at most four steps. A step that another how-to already teaches becomes a link.
6. The scenario is made up, ordinary office work, and nameable in four words. No two pages share a scenario unless one is the sequel of the other. Nothing is clinical or biological, and no person or company is real.
7. A green page holds no design argument. "What this demo decides" moves to the ticket or the ADR when the page turns green.
8. A page ends with at most four links, each to a different idea.

## The front window

The README features seven pages, in this order. Together they show every command and all three question types, and they run from the simplest use to the strongest.

| # | How to | Shows |
| --- | --- | --- |
| 01 | Gate a script step on a yes/no answer | `decide`, the exit code |
| 02 | Branch on a label with `choose` and `case` | `choose` |
| 03 | Keep only the records that match a meaning | `filter`, `--field`, `--dry-run` |
| 06 | Put the best matches first | `rank --top` |
| 15 | Find the line that answers a question | `find`, and a clean "nothing fits" |
| 16 | Build a triage pipeline that drafts, blocks, or asks a person | `annotate`, a question set with all three question types, a policy transform, audit rows |
| 13 | Pick a threshold from labeled cases | `--details` rows and the sweep transform |

## The list

| Group | Pages that stay | What changes |
| --- | --- | --- |
| Start here | 01, 19, 27, 18 | 01 sheds its band and its audit block. 19 becomes "gate a risky command and fail closed": yes, no, unsure, and could not ask. It takes 01's band and absorbs 32. It answers the use case of gating a tool call |
| Gates and branches | 04, 02, 20, 17, 21, and two new pages from ticket 0017: say what yes and no mean, and tune a question once and use the same file in the test and in the gate | 04 keeps its three piles and sheds its counting and its re-cut, which pages 25 and 13 own. 02 sheds two blocks that test the fixture and absorbs 05 as its closing section. 17 becomes "route a request by how hard it is", with `score` and `jq -e`. 21 takes the scenario of picking the next action from a list that changes at every step |
| Many records | 03, 06, 12, 15, and one new page: serve a loop from one long-lived process | 03 absorbs 09, and one sentence names transcript compaction as the same command. 12 owns `--cache`, and 27 drops its cache bullet |
| Many questions at once | 39 (new), 07, 16 | 39 screens one message for several hazards and a severity in one request, and it absorbs 08. 07 gains a `score` column and absorbs 22 and 34. 16 is the flagship |
| Evals | 14, 30, 23, 13, 24, 25, 37, 28 | 14 is rewritten over `annotate` and the transforms, and it absorbs 29 and 31. 13 absorbs 36. 25 absorbs 35, 38, and 26, because watching a pipeline is the same check run on last week's reviewed rows. 28 is short |

Pages that leave as pages: 05, 08, 09, 22, 26, 29, 31, 32, 34, 35, 36, 38, and the separate pages once planned for compaction and for routing. Every idea in them lands in the page named above. No number is reused.

## The evals section stays first-class

Ian's six capabilities still land: structured cases in 14; reusable definitions in 14 and 30; detailed results in 19 and 25; traceable artifacts in 23 and 07; local reporting and comparison in 25, 13, 24, and 28; validation of the judge in 25 and 37. The section shrinks from fifteen pages to eight because seven of them taught half an idea each.

## Consequences

Ticket 0018 applies the standard to every green page, merges the green pages named here, adds the checks to `sdlc/scripts/demos`, rewrites `documentation-plan.md` and `demos/README.md` to this list, and puts the front window in the README. Every later ticket that turns a page green meets the standard. A merged green page keeps its recordings only where a block still uses them.
