# Configuration owner warning Quick Fix build record

Status: candidate for fresh independent code and dependency review. Branch `ticket/qf-config-owner-warning`, based on main `cef35e46`. The owner-warning direction came from the review recorded in `sdlc/records/qf-command-edges-and-prune.md`, file 120. This Quick Fix does not change the warning sentence, refuse a configuration file, or change the group-only and non-Unix rules. The security-boundary issue stays open until review and landing.

## Reproduction and change

The existing Unix predicate in `crates/thinkthen/src/config.rs` checked only mode `0o002`. A readable `0644` configuration owned by another Unix user therefore looked private even though its owner could edit the destination address and model. The new predicate compares the file owner's UID with `nix::unistd::geteuid()` and retains the world-write test. The command's existing `cli/mod.rs` warning path consumes the same `Config::shared` bit and prints its existing name-only sentence; library `from_env()` continues to print nothing. Group-only write on a file owned by the effective user stays quiet. On non-Unix, the predicate still returns false.

The Unix `nix` 0.29.0 production dependency gains only feature `user` beside `poll,signal`; the exact accepted feature list in `sdlc/scripts/policy.py` changes with it. The dev dependency remains `pthread,signal`, and Cargo's feature union includes `user` when tests use the production dependency. Offline locked `cargo tree` resolves nix 0.29.0 with `poll,pthread,signal,user` and its existing internal `feature,process` features. `Cargo.lock` did not change; no package or version was added. Codex-2 released this one policy entry and still owns its separate pending DuckDB policy rules. The settings page names Unix ownership and keeps the existing warning sentence; the security issue records the candidate without closing it.

## Focused proof and limits

The retained unit edge table in `config.rs::tests::a_different_owner_can_change_a_readable_configuration` was red against the old mode-only predicate: `0644`, foreign owner returned false where true was required. It passed after the owner comparison. Its same-owner `0600` and group-only `0664` rows preserve the two quiet cases. This table takes mode and UID values through the predicate used by production; there is no test-only export or second metadata parser. The existing compiled CLI case `cache_configuration::a_configuration_another_user_can_write_is_warned_about` passed. It pins the exact warning for `0666`/`0602` and silence for same-owner `0664`/`0600` over the actual command boundary. No new CLI harness or duplicate mode test was added; no existing functional test was deleted.

An actual other-owned readable JSON fixture was not produced on this host: `unshare -Ur true` failed to write its UID map with `Operation not permitted`, and `sudo -n -u nobody true` required a password. The unit table proves the owner comparison and the compiled CLI case proves the existing warning channel; they do **not** amount to a cross-user CLI run. The check remains advisory: `Config::read` reads then gets path metadata, and this Quick Fix makes no same-inode or race guarantee. No paid or network provider call ran. Backend credentials were removed from the test and lint environments without printing them.

## Checks and size

- `cargo test --locked -p thinkthen --lib config::tests::a_different_owner_can_change_a_readable_configuration -- --exact`: failed before the fix on foreign owner `false` versus `true`, then passed after.
- `cargo test --locked -p thinkthen --test backend cache_configuration::a_configuration_another_user_can_write_is_warned_about -- --exact`: passed, one existing CLI test.
- `cargo fmt --all -- --check`, `python3 sdlc/scripts/policy.py`, strict `cargo clippy --locked -p thinkthen --lib --tests --features cli -- -D warnings` and `--lib --no-default-features -- -D warnings`, offline locked nix feature tree, and `node sdlc/scripts/ratchet.mjs`: passed. The settings table was unchanged, so its executable page checker was not rerun. Full rungs and stress were not run.

`config.rs` grew from 358 to 376 nonblank lines, below its 500-line cap. The source aggregate grew from 79,580 to 79,598 nonblank lines, and `sdlc/ratchet.json` now equals the measurement. The 18 lines earn the owner comparison and one distinct edge table. I checked the existing CLI mode table and config predicate for reuse: the fix retains both, replaces the old single-bit expression, and adds no duplicate parser, test harness, public option, or extra source path. No separate scaffold remains.

## What the build taught us

A permission-bit test cannot prove ownership. The existing CLI warning test already covers message shape and mode compatibility, but this host lacks a safe alternate-owner fixture. The next brief should identify that host capability before promising a cross-user command proof, and keep dependency policy ownership coordinated before changing an accepted feature list.
