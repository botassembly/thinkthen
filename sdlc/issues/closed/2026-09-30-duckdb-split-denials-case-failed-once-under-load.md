# The DuckDB split-denials case failed once under load

Status: closed on 2026-09-30 by ticket 0340. The cause was the test's HTTP/1.0 server: it closed each connection without saying so, ureq reused the connection, and the second send failed as "the backend did not answer". The fixture now sends `Connection: close`. The dependency bug is `2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`.

Kind: debt

Pay when: the case fails again, or before the 0.1 release candidate.

Debt: 005

Severity: low

Paid: 2026-09-30

Keeping it risks a real race hiding behind a rerun.

## What happened

`b13c_try_details_split_denials` (`databases/duckdb/tools/verbs_budget.py:81`) failed once at total 2 with every slot failed, while the load average was near 14. The rerun passed, and the case then passed 90 of 90 runs alone and six at a time. The cause is unknown. Ticket 0304's slice 3a notes hold the record. `closed/2026-09-28-duckdb-try-details-raises-on-a-spent-request-total.md` fixed an earlier bug on the same path.

## Next step

When it fails again, keep the full output and the load, and compare the send budget's order of denials with a passing run.
