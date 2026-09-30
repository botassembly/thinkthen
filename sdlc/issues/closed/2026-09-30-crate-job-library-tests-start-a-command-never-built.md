Status: Closed by the quick fix landed as `Land quick fix: release builds clear host setup, apt, offline crates, and library tests`. Found by the second release rehearsal, run 36780048676, on 2026-09-30. Owner: ticket 0128 Phase 3b. Resolution: The five tests, and the three helpers only they use, carry `#[cfg(feature = "cli")]`. They still run in every default-feature test run, which builds the command. `sdlc/scripts/package` runs the library-only tests in their own target folder, `library-only` under the target folder, where no earlier build can leave a command binary to hide a missing gate. `sdlc/scripts/package` passed with a fresh `CARGO_TARGET_DIR`.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it fails the release `crate` job on every fresh runner, so no crate is packed or smoked.

# The library-only test run starts a command binary it never built

## The problem

The `crate` job runs `sh sdlc/scripts/package`. Its library-only step runs `cargo test --package thinkthen --no-default-features --features bundled-sqlite --all-targets`. Without the `cli` feature, cargo builds no command binary but still sets `CARGO_BIN_EXE_thinkthen` to its path. Five tests in the merged `library` test binary start that path and failed with `No such file or directory`:

- `ca_bundle::bounded_bundle_refusals_precede_send_and_replay_folder_work`
- `ca_bundle::genuine_tls_trust_replaces_default_and_keeps_hostname_verification`
- `key_address::command_refuses_before_plan_status_or_recording_output`
- `public_env::usage_totals::a_cached_rerun_sends_nothing_and_adds_a_cache_answer`
- `public_env::usage_totals::the_command_and_a_seeded_engine_add_to_one_total`

The script's comment says every test that starts the command carries `#![cfg(feature = "cli")]`. These five tests never carried it. Local runs pass because an earlier build left `target/debug/thinkthen` in place. The runner had a fresh target folder.

## A fix

Gate each test that starts the command with `#[cfg(feature = "cli")]`, as `public_env.rs` line 462 does. Run `sdlc/scripts/package` with a fresh `CARGO_TARGET_DIR` once to prove it.
