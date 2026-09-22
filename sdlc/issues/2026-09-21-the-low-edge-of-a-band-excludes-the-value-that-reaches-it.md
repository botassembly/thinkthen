# The low edge of a band excludes the value that reaches it

Status: Closed on 2026-09-22. The fix landed in ticket 0056.

`specification/threshold.md` line 19 says: "Boundaries are inclusive. A value meets its mark when it reaches it." Four of the five boundaries follow that sentence. The fifth does not: the low edge of a band sends a probability that reaches it to no, not to not sure.

## Reproduction

Verified on the rebuilt release binary at main `db19349`, 2026-09-21, against a loopback stand-in answering an exact probability:

    $ thinkthen decide 'Q?' --threshold 0.1    < ev.txt   # probability exactly 0.1
    true
    exit 0                                    # the cut is inclusive, as the page says
    $ thinkthen decide 'Q?' --threshold 0.1:0.9 < ev.txt
    false
    exit 1                                    # the low edge excludes 0.1
    $ thinkthen decide 'Q?' --threshold 0.9    < ev.txt   # probability exactly 0.9
    true
    exit 0                                    # the high edge is inclusive

A probability of 0.1 under a band of 0.1:0.9 is answered no. The page's own rule says it meets its mark, and the mark it meets is the boundary of not sure.

## Expected

Either the low edge is inclusive like every other boundary, and 0.1 under 0.1:0.9 is not sure, or `threshold.md` says plainly that the low edge sends a reached value to no. The first matches the page and the intuition of a band; the second is a one-sentence fix to a surprise. The stranger's report stands either way: the same number passes a cut and fails a band that starts at that cut.

## How bad it is for a user

Minor. It moves one boundary case, and a user tuning a band to route borderline records will route exactly-the-low records to no without knowing.

Found by experiment 218, wave 1.5, the blind seat (finding 12), verified by the orchestrator.
