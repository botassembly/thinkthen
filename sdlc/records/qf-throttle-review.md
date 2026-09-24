ACCEPT

# Review of Quick Fix qf-throttle

Reviewer: a fresh read-only Opus session that did not write the work. Its first reply, on the first commit, listed findings. Its second reply, on `8082212f`, accepted the fixes. This page restates both.

It confirmed the ADR 0017 amendment states Ian's ruling and scope, carries every rule of "one width for the process" to the new name, and matches that amendment's per-loaded-copy sentence. Ticket caps hold. Every number on the pages is unchanged. The new text follows the writing rules and names no private project or customer.

Findings, each fixed in the next commit:

1. The range message `a width is a whole number from 1 through 32` reaches a library user. The amendment now tells 0086 to reword it with the conflict message.
2. Port-guide row R1-8 described future work as "width". It now says throttle.
3. ADR 0032 listed "width" among what a profile never selects. It gained a dated amendment.
4. Dropping 0084's duplicated paragraph lost the rule that `ErrorDetail` has no string conversion. The compile-fail line now bans it.

Declined, not blocking: the reference page and demo 12 define the throttle without "per loaded copy". The command loads one copy.
