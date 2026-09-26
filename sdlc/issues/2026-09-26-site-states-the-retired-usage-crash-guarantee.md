# The site states the retired usage crash guarantee

Status: Open. Filed 2026-09-26 by ticket 0141 for the website agent.

## What happens

`site/src/pages/backends.astro:48` says "A crash can undercount tokens or overcount one request that was already charged."

## Why it matters

Ticket 0141 and ADR 0049 stopped writing each request's count to disk before it is sent. `specification/recording.md` now says: "Counting never holds back a request. A crash can undercount the requests and tokens counted after the last write that finished." The site sentence promises a guarantee the tool no longer gives.

## What is asked

Replace the site sentence with the specification's wording. The ticket did not edit `site/`, which the website agent owns.
