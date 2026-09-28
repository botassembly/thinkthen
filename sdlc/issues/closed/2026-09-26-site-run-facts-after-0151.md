# The site should say where run facts come from

Status: closed. Confirmed by ticket 0241 code review at `db7e2418`. See [the acceptance record](../../records/0241-code-review.md).

Original report status: Open. Filed 2026-09-26 by the coordinator for the marketing lead, who owns `site/` (`sdlc/planning/ownership.md`). Ticket 0151 decision 9 hands over the site part of ask 6 in `2026-09-26-every-surface-should-give-back-run-facts.md`.

Ticket 0151 landed one set of details on every surface. Each surface's README now has a "Run facts" section naming the call that gives back `usage`, `requests_sent`, `cached`, `confidence` and `url`. The site does not say this yet.

Asked:

1. Show one raw HTTP exchange with its `usage`, so a reader sees what Jev sends back.
2. On each language page, say where run facts come from. Take the call names from that surface's README "Run facts" section.
3. Correct the R blurb at `site/src/data/catalog.mjs:461`, which says "Ten `tt_` functions". The R package exports 14: the ten verbs plus `tt_question`, `tt_details`, `tt_usage` and `tt_engine` (`libraries/r/thinkthen/NAMESPACE`).

Close this issue when the three items land on the site.
