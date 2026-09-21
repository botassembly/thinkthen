# How-to candidates where one answer feeds the next

Status: Open

Ian raised an idea on 2026-09-21: small answers flow through joins with other small answers, feed another question, and go around again. A shell pipe is the smallest stream processor, and a SQL join is the second smallest. The marketing side proposes four how-tos from it. Page 16 already shows answers feeding the user's rules. None of the 19 green pages shows an answer feeding another question, a join by meaning, or a live stream.

## The four candidates, in the order the marketing side would build them

1. **Join two tables by meaning** (DuckDB or PostgreSQL). Support tickets on one side and open incidents on the other. A cheap ordinary condition narrows the pairs first, such as the same day. Then `thinkthen_decide('Does this ticket describe this incident?', ...)` sits in the `ON` clause. This is the strongest single picture of the idea, and nothing ordinary SQL does comes close. The page states the request count as pairs after narrowing, because a join with no narrowing multiplies.
2. **Group alerts into incidents.** Each new alert runs `find --none` against the list of open incidents, one line each. A match attaches the alert. No match opens a new incident, and the list grows. The answer from one pass changes the input to the next pass. `find` reads up to 255 lines in one request, so the open list costs one request per alert.
3. **Ask a question about the answers.** `annotate` fills the form for each ticket. `jq` groups the answers by customer or by minute into one short text. A second `decide` asks about that text: "Do these tickets describe one outage?" This is a window and a second-level question in three commands.
4. **Watch a live log.** `tail -f app.log | thinkthen filter 'Should a person see this line?' --lines`. This one needs a fact first. See the probe below.

## A probe the fourth candidate needs

The specification says `rank` holds every record until the input ends. It does not say when `filter`, `tag`, `choose`, `score`, and `annotate` print a record whose answer is ready while the input is still open. A live log never ends. The probe: feed `filter --lines` from a pipe that stays open, under `--replay`, and record whether each kept line prints while the pipe is open, how long a lone record waits, and whether order holds with 32 in flight. If output waits for the end of input, that is a finding for the specification, and candidate 4 waits for it. The probe needs no paid call.

## What the pages must not claim

ThinkThen holds no state, no windows, and no delivery promise. The pages say so in one line. The stream processor, the database, or the shell owns those. ThinkThen is the question inside the step.

## What Ian can overturn

The order, and whether any of these enters the first release. The marketing side recommends candidate 1 for the announcement and the talk, and candidates 2 and 3 for the site.
