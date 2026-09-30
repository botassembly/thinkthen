# ADR 0085: A tuned threshold names its batch setting

- Status: Accepted design in ticket 0171; corrected implementation awaits follow-up code review
- Date: 2026-09-27

## Context

ADR 0048 item 8 requires a warning when a threshold runs at another batch setting. ADR 0053 item 3 makes a threshold-bearing file with no `batch` count as tuned at 1. A default record run now fills requests at `max`. Without a warning, a cut tuned one record per request can silently move. Experiment 284 file 14 and the batching evidence record identify that risk. The batch setting stays out of the question digest by Ian's ruling.

## Decision

1. A `decide`, `filter`, or `rank` record run compares its resolved setting with the setting saved beside a file's threshold. A threshold-bearing file with no `batch` was tuned at 1. A file with no threshold has no tuned setting. A structured JSON question runs one record per request and compares as 1. A single document has no batch warning.
2. On a mismatch, standard error names both settings once at the first successful logical result, even when `filter` prints no row. A first failed result produces no warning. Each detailed result row carries `meta.batch_warning` after `meta.batch_setting`. The existing profile warning remains independent and precedes the batch warning when both apply. No setting or question digest changes.
3. `audit` and `diff` read the set of `meta.batch_setting` values in saved results, or `meta.batch.setting` in rows saved before ADR 0111 (amended by ticket 0349). A line without `meta.batch` adds 1 only when no line names another setting. Mixed settings warn once, in ascending numeric order with `max` last. `diff` now prints at most three warnings.
4. `audit --write` records a single batched setting when it writes a threshold into one `decide` file. It never writes `batch: 1`. A run entirely at 1 removes an old `batch` key, preserving all other file bytes. Mixed settings keep the key and receive a report line. Other question kinds and question sets gain no batch key.

## Consequences

The default remains `max`, so the warning carries calibration risk without slowing a run. Existing result bytes are unchanged when the tuned and running settings match. A file tuned at 1 stays without `batch` and warns at the batched default. Libraries and SQL surfaces wait for their batching tickets. Ticket 0172 adds context without revising this warning rule.

This ADR amends ADR 0048 item 8 and ADR 0053 item 3. Ian can overturn the default, warning policy, and file-write policy in a later ADR.
