# DuckDB conformance fails three singleton cases

Status: open. Found by the ticket 0318 build.

## What happens

`databases/duckdb/tools/conformance.py` fails `14-filter-none`, `34-annotate-repeated-texts` and `35-annotate-score-repeated-texts`. Each gets `thinkthen backend: the backend answered with status 500 (retryable: yes)`, so the loopback backend received a request that the case does not hold. Each case pins one-record requests.

The three cases fail the same way with main `46bf1059c`'s DuckDB engine and harness, rebuilt in lane claude-3, so ticket 0318 did not cause them. Because `check.sh` stops at the first failing suite, a routine DuckDB check on main never reaches `portable_suite`, `plan_suite`, `find_suite`, `site_examples` or the self-tests. Run in the 0318 lane, those suites pass.

## Likely cause

The DuckDB conformance runner sets `THINKTHEN_BATCH=1` only for `27-decide-many` and `28-decide-many-repeated-texts`. The default is now `max`, so a filter or annotate over several rows sends one packed request that no case holds. PostgreSQL's `check.sh` exports `THINKTHEN_BATCH=1` for its whole run.

## Done when

The three cases pass in `databases/duckdb/check.sh`. Either the runner selects singleton batching for every singleton-pinned case, or ADR 0111 slice 1's rewritten cases replace them.
