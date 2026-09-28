# Keep native panic-child environment names test-only

Quick Fix at main `1a6dd02a`. The settings checker scans tracked source text for `THINKTHEN_` names, including inline `#[cfg(test)]` modules. The five native panic child tests used names outside its `THINKTHEN_TEST_` exclusion. Before the edit, `PATH="$PWD/target/debug:$PATH" sdlc/scripts/settings` reported exactly five missing setting rows: `THINKTHEN_DUCKDB_PANIC_CHILD`, `THINKTHEN_NODE_PANIC_CHILD`, `THINKTHEN_RUBY_PANIC_CHILD`, `THINKTHEN_R_PANIC_CHILD`, and `THINKTHEN_SQLITE_PANIC_CHILD` (46 rows, 54 flags, 11 environment names, 15 question-file keys). These were test selectors, not product settings; documenting them as settings or weakening the scanner would misstate the contract.

Only the `CHILD` string constants changed, to `THINKTHEN_TEST_<HOST>_PANIC_CHILD`, in `databases/duckdb/src/errors.rs`, `databases/sqlite/src/worker.rs`, `libraries/ruby/src/diagnostics.rs`, `libraries/r/thinkthen/src/rust/src/calls/diagnostics.rs`, and `libraries/typescript/src/door/diagnostics.rs`. Each constant remains inside `#[cfg(test)]` and feeds both the parent's `.env(CHILD, "1")` and the child's `var_os(CHILD)` gate. The five old names have no remaining source references. No production branch reads a new environment variable; release behavior, error text, panic payload disposal, and host interfaces are unchanged.

The same settings command then passed: 46 rows, 54 flags, 6 environment names, 15 question-file keys, 0 failures. It used the existing `target/debug/thinkthen` on PATH, last built before this test-only edit; its help comes from that binary, while the checker reads current tracked source. A focused SQLite child round-trip passed with the renamed marker: `cargo test --manifest-path databases/sqlite/Cargo.toml worker::tests::caught_callback_and_worker_payloads_stay_out_of_diagnostics -- --exact` (1/1). No other host package was rebuilt. Root and the five affected Rust ratchets remained exact; pages reported 21 green and one coming, tickets reported zero evidence failures, and `git diff --check` passed.

## What the fix taught us

An inline test module is still visible to a source-text settings scan. Child-process selectors must use the established test-only prefix at their declaration, with parent and child sharing one constant. A single representative child test checks dispatch after a spelling change; five host rebuilds would duplicate that boundary without changing product behavior.


## Independent review and landing

Fresh independent Sol Medium code review accepted candidate `cfdd16d5`. It checked all five compile-time test guards, shared parent/child constants, absence of old executable references, unchanged scanner strictness and the recorded proof limits. The combined main candidate passes the settings, page, ticket, root counter and whitespace checks. This fixes an inherited check failure; it adds no product setting and closes no additional register row.
