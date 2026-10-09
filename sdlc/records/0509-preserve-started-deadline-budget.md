# Preserve started deadline budgets

Source: `2ac8de370`. The installed reproduction integrates only SQLite migration `f502db48d`, cherry-picked as `8fc24e873`.

Given a configured relative deadline, when a composed call starts and reuses its options, then its error retains the configured budget and each child observes the same absolute deadline. Repeating `started` must preserve that deadline. Zero budgets and past absolute deadlines send nothing; clearing a started deadline permits the next call.

## First failure

The native regression `started_deadlines_keep_the_configured_budget_without_restarting` failed before the repair. After a started 200 ms budget expired, the error said `the deadline of 0 s passed before the call answered` instead of `the deadline of 200 ms passed before the call answered`. The counted listener received zero requests.

The first SQLite invocation used the system Python SQLite 3.45.1 and failed the extension's minimum version check. The pinned SQLite 3.50.0 host requires its library folder in `LD_LIBRARY_PATH`. The warm extension then passed, showing that a warm artifact cannot establish behavior of the newly integrated source. The reproduction rebuilds that source before drawing a conclusion.

The rebuilt migrated SQLite extension reproduced the unchanged held-call regression: it returned a deadline of `199947176 ns` where the test requires `200 ms`. Its zero-budget and retry-allowance cases passed.

## Repair

`CallOptions` retains the existing private `Deadline` after starting a relative budget. Reusing or restarting the options copies the same budget and instant. Absolute deadlines retain their existing contract. Deadline clearing and replacement continue through the same setters. The SQLite test and error conversion remain unchanged.

## Lessons

Preserve the complete deadline value across composition. A remaining duration is a transport limit, not the caller's configured budget. Verify a rebuilt installed extension with the pinned host; a warm library can hide a source change.
