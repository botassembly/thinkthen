# Preserve started deadline budgets

Source: `2ac8de370`. The installed reproduction integrates only SQLite migration `f502db48d`, cherry-picked as `8fc24e873`.

Given a configured relative deadline, when a composed call starts and reuses its options, then its error retains the configured budget and each child observes the same absolute deadline. Repeating `started` must preserve that deadline. Zero budgets and past absolute deadlines send nothing; clearing a started deadline permits the next call.

## First failure

The native regression `started_deadlines_keep_the_configured_budget_without_restarting` failed before the repair. After a started 200 ms budget expired, the error said `the deadline of 0 s passed before the call answered` instead of `the deadline of 200 ms passed before the call answered`. The counted listener received zero requests.

The first SQLite invocation used the system Python SQLite 3.45.1 and failed the extension's minimum version check. The pinned SQLite 3.50.0 host requires its library folder in `LD_LIBRARY_PATH`. The warm extension then passed, showing that a warm artifact cannot establish behavior of the newly integrated source. The reproduction rebuilds that source before drawing a conclusion.

The rebuilt migrated SQLite extension reproduced the unchanged held-call regression: it returned a deadline of `199947176 ns` where the test requires `200 ms`. Its zero-budget and retry-allowance cases passed.

## Repair

`CallOptions` retains the existing private `Deadline` after starting a relative budget. Reusing or restarting the options copies the same budget and instant. Absolute deadlines retain their existing contract. Deadline clearing and replacement continue through the same setters. The SQLite test and error conversion remain unchanged.

## What the build taught us

Preserve the complete deadline value across composition. A remaining duration is a transport limit, not the caller's configured budget. Verify a rebuilt installed extension with the pinned host; a warm library can hide a source change.

## Evidence

On repair revision `c2a037b11`, `cargo fmt --all --check`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, and `node sdlc/scripts/ratchet.mjs` passed. Policy reports the existing large-file warnings; the extended public-controls file owns the counted control boundary cases.

`cargo test --locked --offline -p thinkthen --test public_controls deadline -- --nocapture` passed all three selected native deadline cases. `cargo clippy --locked --offline -p thinkthen --lib --test public_controls -- -D warnings` passed.

The repaired source plus the isolated SQLite migration rebuilt with `cargo build --locked --offline --release --manifest-path databases/sqlite/Cargo.toml` and the existing home-path remap. A copy at `target/0509-installed/libthinkthen0.so` passed all three unchanged `databases/sqlite/tests/test_deadline.py` cases with the pinned SQLite 3.50.0 host. Its SHA-256 is `0f02310a72964ed54b375d3f802a533e39a851338ddbae69f1e08d222cbf7af9`.

Heavy runs used user scopes with an 8 GiB memory cap, a 1 GiB swap cap and two build jobs. Lane output totals remained below 40 GiB. Every launched job completed and was reaped.

The full repository gates and fresh independent review belong to the coordinator. This repair changes no engine module, SQL error mapping, SQLite test, dependency lock, credential file or release operation.

## Landing qualification

The coordinator separated the shared repair from its SQLite reproduction dependency and integrated it with stage-context main at `70f5b1115`. Fresh read-only code review accepted that source. Full `sdlc/scripts/lint`, `test` and `spec` all exited zero, including the native complete public consumer, child and installed-surface smoke checks and replayed demos. The existing ratchet measured 173,815 lines after combining the two independent source changes. All owned gate sessions finished and were reaped; no product source changed after qualification.

The installed SQLite reproduction was also rebuilt with the current main plus this repair in 0499 at `c98d15d13`: all existing selected legacy modules, all 255 shared SQL cases and owning-facts checks passed. The unchanged 200 ms held-call assertion passed. That SQLite code retains its separate ticket, fresh review and partial landing; it is not included in the 0509 merge.
