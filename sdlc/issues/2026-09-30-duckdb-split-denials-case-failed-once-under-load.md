# The DuckDB split-denials case failed once under load

Status: open. Found by ticket 0304 slice 3a's first surface sweep. Owner: none.

Kind: debt

Pay when: the case fails again, or before the 0.1 release candidate.

Keeping it risks a real race hiding behind a rerun.

## What happened

`b13c_try_details_split_denials` (`databases/duckdb/tools/verbs_budget.py:81`) failed once at total 2 with every slot failed, while the load average was near 14. The rerun passed, and the case then passed 90 of 90 runs alone and six at a time. The cause is unknown. Ticket 0304's slice 3a notes hold the record.

## Next step

When it fails again, keep the full output and the load, and compare the send budget's order of denials with a passing run.
