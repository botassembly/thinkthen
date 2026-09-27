# Quick Fix: cache setting guidance

Status: built for independent review. Scope: register 11, item 8 of `sdlc/issues/2026-09-26-settings-table-gaps-a-site-reader-hits.md`. This is a specification-page correction; the other issue items remain open.

## Source evidence

- `crates/thinkthen/src/config.rs` declares `cache: Option<bool>` (`null` is treated as absent), defaults it on, and now names a bad value as ``configuration field `cache` must be true or false``. The message change landed before this Quick Fix.
- `crates/thinkthen/src/cli/edge.rs` chooses a nonblank `THINKTHEN_CACHE` folder before the platform folder. `crates/thinkthen/src/cli/asking/folders.rs` applies the configuration switch only when that platform folder would be used; `--cache DIR` and `--no-cache` decide first. `crates/thinkthen/src/cli/status.rs` reports the named environment folder as the enabled source. `crates/thinkthen/tests/backend/cache_configuration.rs::a_named_cache_enables_storage_over_disabled_configuration` and `crates/thinkthen/tests/status.rs::environment_and_configuration_provenance_are_independent_and_hide_the_key` already prove those distinct cases.
- `crates/thinkthen/src/public/settings.rs` seeds `EngineBuilder::from_env()` from the same environment and configuration and accepts explicit `cache_at` or `no_cache`; an unseeded builder does not read the configuration. The constructor forms were checked in `libraries/python/src/engine.rs`, `libraries/typescript/src/door.rs`, `libraries/ruby/lib/thinkthen.rb`, `libraries/r/thinkthen/src/rust/src/ffi.rs`, `libraries/c/src/settings.rs`, `databases/duckdb/src/engines.rs`, `databases/postgresql/src/call.rs`, and `databases/sqlite/src/settings.rs`. The SQL folder settings differ from SQLite's additional NULL off form.

## Change and checks

The Answer cache row now says the configuration value is boolean. The precedence paragraph says which input chooses a folder, which turns caching off, and why a named environment folder overrides configuration `false`. A short surface paragraph records each library and SQL form. Issue item 8 now marks only this page defect corrected and treats its old unnamed error report as historical.

The warm `target/debug/thinkthen` came from the accepted 0163 validation at `0292cba2adb75924cddfb30d419688efe4d9dd52`; `git diff --name-only 0292cba2..97d21366 -- crates/thinkthen/src/cli crates/thinkthen/src/config.rs crates/thinkthen/src/public/settings.rs` returned no paths. Reusing it checks current command-help inventory, not any changed Rust behavior. With `THINKTHEN_API_KEY` unset:

- `PATH="$PWD/target/debug:$PATH" python3 sdlc/scripts/settings`: 42 rows, 52 flags, 5 environment names, 15 question-file keys, 0 failures.
- `python3 sdlc/scripts/pages`: 1 coming, 21 green.
- `git diff --check`: passed.

No runtime source or tests changed. Existing configuration and settings proofs were read, not rerun; this documentation change does not need a new runtime build or port suite.

## What the preparation missed

0163 preparation checked cache implementation and its executable pages but did not reconcile the shared Settings table's Allowed values and precedence text with `Config::cache` and the named-folder override. A table row spanning several surfaces needs its parser type and resolver order traced separately before it is called accurate.
