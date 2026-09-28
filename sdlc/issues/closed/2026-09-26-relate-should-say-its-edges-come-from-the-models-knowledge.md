# `relate.md` should say its edges come from the model's knowledge

Status: non-issue; already fixed before 0241. Confirmed by ticket 0241 code review at `db7e2418`. See [the acceptance record](../../records/0241-code-review.md).

Original report status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 10, finding 5. Ticket 0167 carries the sentence. Owner: backlog ticket R8, the recognize manual, or H3 if H3 lands first. Blocks 0.1 under goal 4, honest docs. The `recognize` half is tracked by recognize design section 5 and ticket R5.

## What happens

Standalone `relate` reads a set of names and kinds, with no source text. Each edge it prints is the model's belief about those names. The page never says so. `specification/relate.md` describes the planner, the cut and the output, and it never defines what an edge means or where its evidence comes from.

Report 10 found the same gap in `recognize --relation` live. From "Ringo Starr sang Octopus's Garden.", it gave `wrote(Ringo Starr, Octopus's Garden)` at 0.79. It gave `sang` edges to `Help!` at 0.70 and 0.58 from a sentence that names no song. The recognize design's section 5 makes a `recognize` edge mean "the text states it". That redesign keeps standalone `relate`'s wording, because `relate` has no text to state anything.

A reader who builds a graph with `relate` can take its edges as facts drawn from their data. They come from what the model believes about the names.

## Checked on main

Verified: `specification/relate.md` has no sentence on what an edge means or where its evidence comes from. The live edges come from the report.

## What would fix it

Add one sentence to `relate.md`: a standalone `relate` edge comes from the model's knowledge of the names, not from any text. Point to `recognize --relation` for edges a text states. R8 or H3 carries the sentence. Ticket 0147 is being amended separately, so this issue asks nothing of it.

## Done when

`relate.md` says where a standalone edge comes from.
