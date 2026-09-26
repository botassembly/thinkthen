# The recording page says a failure is never recorded

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, reports 03 (2-4), 06 (I-1), 08 (2) and 11 (5). Blocks 0.1 under goal 4. This issue covers only the wrong sentence. Ticket 0158 carries the partial-reply cache fix.

## What happens

`specification/recording.md` line 59 says: "Only an exchange that succeeded and decoded is recorded. A failure is never recorded."

Line 84 of the same page says the opposite for a partial reply: "A recorded partial reply replays the same good answers, failed markers, failure count, and exit 6." So a reply with one failed answer is recorded, and the answer cache is on by default. Four reports reproduced the result: after a transient bad answer, every rerun on the cache replays the failed marker at exit 6 with no request sent.

A whole refused reply, at exit 4, is not stored. Report 11 checked that.

## Checked on main

Verified: lines 59 and 84 read as quoted. The replay runs come from the reports.

## What would fix it

Rewrite line 59 so it says which failures are never recorded and that a partial reply is. Once ticket 0158 lands, make the page match what the cache then does with a partial reply, and say how a user retries one failed question.

## Done when

`recording.md` says the same thing in both places, and it matches the behavior 0158 lands.
