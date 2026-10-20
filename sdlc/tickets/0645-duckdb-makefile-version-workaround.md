# 0645: Remove the DuckDB Makefile version override

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `Makefile`, `databases/duckdb/tools/version.env`, `databases/duckdb/tools/inputs.py`, `databases/duckdb/tools/source_hosts.py`, `databases/duckdb/tools/input_version_cases.py`.
- `databases/duckdb/tools/version.env` names, for each pinned DuckDB version, its source ref, its ABI identifier and its test runner flags. `source_hosts.py` passes them to `make` directly, with no parse-time Python call and no unexporting.
- `inputs.py abi-override`, the `abi_override` function, and both conditional blocks in the root `Makefile` (the `THINKTHEN_ABI_OVERRIDE` block and the `grep` of `TEST_RUNNER_SCRIPT`) are deleted.
- DuckDB version pins are tool pins and stay under the 2026-10-19 no-version ruling.

### Proof

- On the M5 natively: `pm job run --machine m5 0645 SHA --memory 8 -- sdlc/scripts/lint` passes, including `python3 databases/duckdb/tools/input_version_cases.py`, which covers the new `version.env` fields.
- At the 0619 container checkpoint, in the arm64 container on the M5: `python3 databases/duckdb/tools/source_hosts.py` builds both pinned hosts and runs `test_release` with `--require thinkthen`, and `sh databases/duckdb/check.sh` passes on both versions. The loaded extension reports the pinned DuckDB version through `pragma version`. Before the change, removing the `abi-override` block makes the 2.0 preview host build with the wrong ABI identifier and fail to load the extension.

## Evidence

- The root `Makefile` runs `databases/duckdb/tools/inputs.py abi-override` at parse time. When that prints anything (only when `DUCKDB_VERSION` equals `DUCKDB_ROUTINE_PREVIEW_SOURCE` in `version.env`), it copies `DUCKDB_VERSION` into `DUCKDB_COMMIT`, unexports `DUCKDB_VERSION` and adds `-DDUCKDB_EXPLICIT_VERSION=` to `EXT_FLAGS`, to work around how upstream 2.0 forwards its checkout ref. The same file probes the upstream test runner script with `grep '"--test-flags"'` to choose test flags. Both depend on upstream internals and change behavior silently when the submodule moves.
- `databases/duckdb/tools/source_hosts.py` builds each pinned host with `make -f Makefile ... release` and `test_release`. `databases/duckdb/tools/version.env` holds the two pins (`v1.5.6` and `v2.0-cyanoptera`) with their source, tools and Python commits; `inputs.py` `selected()` already returns each version's `metadata_version`.
- The DuckDB community build also enters through the root `Makefile` (`databases/duckdb/community/README.md`). Started ticket 0577 works on that build on its own branch; this ticket lands after it or rebases on it.
- The DuckDB tools are declared Linux-only in `sdlc/tools.json`, so host builds run in the 0619 container, not natively on the M5.
