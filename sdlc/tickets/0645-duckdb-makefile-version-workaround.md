# 0645: Remove the DuckDB Makefile version override

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Each pinned DuckDB version names its source ref and ABI identifier explicitly in one input file. The extension build passes them directly, with no parse-time Python call and no unexporting.
- The test runner's flags are chosen by the pinned version from that file, not by grepping the runner script.
- `inputs.py abi-override` and both conditional blocks in the `Makefile` are deleted.

### Proof

- On the M5 through pm job, the extension builds and loads, and passes `--require thinkthen` tests, on both pinned DuckDB versions.
- The loaded extension reports the expected DuckDB version through `pragma version`.

## Evidence

The root `Makefile` runs `databases/duckdb/tools/inputs.py abi-override` at parse time. When that prints anything, it copies `DUCKDB_VERSION` into `DUCKDB_COMMIT`, unexports `DUCKDB_VERSION` and passes `-DDUCKDB_EXPLICIT_VERSION=` (`Makefile:8-14`), to work around how upstream 2.0 forwards its checkout ref. The same file also probes the upstream test runner script with `grep` to choose test flags (`Makefile:31-35`). Both depend on upstream internals and change behavior silently when the submodule moves.
