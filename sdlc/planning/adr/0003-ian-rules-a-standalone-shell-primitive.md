# ADR 0003: Ian rules a standalone shell primitive, built now

- Status: Accepted. ADR 0007 replaces the grammar `thinkthen decide VERB` with flat verbs. The rest stands.
- Date: 2026-09-18

Ian made these rulings in dictation on 2026-09-18. They answer questions 1, 2, 3, 4, 5, 8, 9, and 11 of the design study. Only Ian overturns them.

## Context

The design study asked ten questions and an eleventh about paid calls. It recommended a small first release built around a botassembly shadow gate. Ian ruled differently on the purpose and agreed on the grammar and the backend shape.

## Decision

**thinkthen is a standalone primitive for the shell.** It is designed and built in isolation, for Bash, right now. Many people beyond Ian's own agents should be able to use it. No botassembly need shapes it.

**The platform question is closed for now.** The platform Ian named is botassembly. Every decision about how thinkthen joins it waits. Questions 2 and 8 of the study are deferred, and question 9 is answered: the first job is to be a good Bash tool.

**The whole command line is planned.** The `decide` family is built first. The other families in Ian's captures follow, each with its own specification before any ticket. Nothing is parked for lack of a use.

**The grammar keeps the family word.** A command reads `thinkthen decide if`.

**A backend is a URL and a simple adapter.** Configuration stays easy. Nothing in the tool is hard-coded to one vendor. ADR 0004 records the shape an agent chose to meet this ruling.

**The design gets time.** Ian wants to think through the high-level design. The `specification/` folder is where he reads it, and code follows the specification.

**Live testing has more budget.** Ian granted more budget for paid calls during testing and named no number. An agent set the cap at twenty million input tokens for this repository, under one dollar at the first vendor's price. `sdlc/planning/plan.md` tracks the spend. Ian raises the cap by saying so.

## Still open

Question 6, the form of a saved question file. Question 7, the home of a pass mark. Question 10, relations with the first vendor. The timing and license of the public release.

## Consequences

The botassembly shadow gate is no longer the first proof. The first proof is a stranger's shell pipeline that works from the documents alone. The size ceiling and the strict gates still apply to every family, so a wide plan has to be paid for in small, reviewed slices.
