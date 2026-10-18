# 0643: Set engine settings by name and value through one core parser

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0626

## Outcome

- `crates/thinkthen/src/public/settings.rs` gains one setter that takes a setting name and its raw text value, validates it with the core's own messages, and applies it to the `EngineBuilder`. `from_settings_json` applies each entry through that setter, so the JSON form and the name-and-value form share one parser.
- PostgreSQL GUCs, SQLite settings calls and DuckDB settings pass raw name and value pairs to that setter. The C library keeps passing its settings object to `from_settings_json`. The `Plan`, `Raw` and `Stored` structs, `throttle_refusal`, the DuckDB throttle parse and the host crate's own settings parsing are deleted.
- A new engine setting needs a change only in the core, plus one line per database that registers its name.
- Refusal text in database tests comes from one shared expected-messages file, not from literals in each suite.

### Proof

- On the M5 through pm job, the installed PostgreSQL 16, 17 and 18 builds, the SQLite loadable extension, the DuckDB extension and the installed C library each refuse a throttle of 33 with the same sentence.
- Each accepts every setting in the shared settings cases.

## Evidence

Each database extension repeats the same settings model and checks. `databases/postgresql/src/call/settings.rs:31-48` (`Plan`) and `databases/sqlite/src/settings.rs:15-33` (`Stored`) declare the same fields: `base_url`, `prices`, `refresh_cache`, `throttle`, `batch`, `cache`, `backend`, `model`, `timeout`, `max_retries`, `profile`, `record` and `replay`. `databases/duckdb/bridge/src/engines.rs:306-310` parses throttle again. Each repeats the core's sentence "a throttle is a whole number from 1 through 32", which `crates/thinkthen/src/public/settings.rs:233` already owns. The PostgreSQL test copy of that sentence is in `check.sh:1242` and the DuckDB copy in `tools/settings_suite.py:28`. Adding a setting means editing three extensions and their tests.

The core already parses a settings object for the C library: `EngineBuilder::from_settings_json` in `crates/thinkthen/src/public/settings.rs`, called from `libraries/c/src/settings.rs:8`. The host crate keeps its own settings code too (`crates/thinkthen-host/src/complete_native/settings.rs`, "used by PostgreSQL and DuckDB"), which 0627 then removes.

## Progress

- 2026-10-18 started
