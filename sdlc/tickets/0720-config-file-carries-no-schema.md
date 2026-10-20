# 0720: Configuration files carry no schema member

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision 374c5df, accept

## Outcome

- Changes: `conformance/children/backend_cases.py`, `crates/thinkthen/src/config.rs`, `crates/thinkthen/src/config/backend_setups_tests.rs`, `crates/thinkthen/src/config/backends.rs`, `crates/thinkthen/src/config/tests.rs`, `crates/thinkthen/tests/backend/cache_configuration.rs`, `crates/thinkthen/tests/backend/check.rs`, `crates/thinkthen/tests/backend/facts/priced.rs`, `crates/thinkthen/tests/backend/named_backends/`, `crates/thinkthen/tests/backend/rank_set_backend.rs`, `crates/thinkthen/tests/backend/status.rs`, `crates/thinkthen/tests/library/public_env.rs`, `crates/thinkthen/tests/library/public_env/batch.rs`, `crates/thinkthen/tests/windows/privacy.rs`, `crates/thinkthen/tests/windows/remaining.rs`, `databases/duckdb/tools/backend_setups.py`, `demos/28-what-a-run-cost/README.md`, `libraries/python/tests/backend_settings.py`, `site/examples/install/backends/system-one/files/bad/thinkthen/config.json`, `site/examples/install/backends/system-one/files/config/thinkthen/config.json`, `site/examples/install/configuration/files/config/thinkthen/config.json`, `site/src/pages/start/configuration.astro`, `specification/backends.md`, `specification/question-file.md`, `specification/recording.md`, `specification/settings.md`.
- Configuration files have no `schema` member (`thinkthen.config/1`). A file that still has one is refused with one sentence naming the member to delete.
- The site's configuration page and examples, the backends, settings, question-file and recording specifications, and demo 28 drop the member. `sdlc/scripts/spec` and `sdlc/scripts/demos` replay them.

### Proof

- On the M5 through `pm job run --machine m5 0720 SHA --memory 8 -- cargo nextest run -p thinkthen config`, a configuration file without `schema` loads and one with `"schema":"thinkthen.config/1"` is refused with the exact sentence. Before the change, the first is refused with "configuration field `schema` must be `thinkthen.config/1`".
- On the M5 through pm job, `sdlc/scripts/spec` and `sdlc/scripts/demos` pass. The DuckDB backend setup suite runs at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Split from 0706.

Starts from `crates/thinkthen/src/config.rs`: the `SCHEMA` and `UNKNOWN` messages, the `schema` field of `Config`, `Config::parse`, `Config::validate`, and the field check that requires `schema`. Backend setups in `crates/thinkthen/src/config/backends.rs` write the member. About 20 Rust test files under `crates/thinkthen/tests/backend/` write config files with it.

Shares `specification/settings.md` and `specification/backends.md` with 0706, 0721 and 0723; any order lands.

## Progress

- 2026-10-19 started
