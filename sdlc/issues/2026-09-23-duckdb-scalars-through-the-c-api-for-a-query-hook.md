# Register the DuckDB scalars through the C API to get a per-query hook

Found: 2026-09-23, review 7 fixer lane (databases). Severity: low. Status: open, follow-up.

## What was observed

A SIGINT that lands after a chunked query's last engine call cancels nothing. The next call already belongs to the next query, so the handler cannot tell the two queries apart. Review 6 and review 7 both recorded this as a known limit. The `CANCEL` comment in `databases/duckdb/src/lib.rs` and the host-signal section of `databases/duckdb/NOTES.md` pin the boundary.

The handler sees scalar invokes and engine calls. It never sees where a query begins or ends. Its rules (in flight, inside an invoke, or inside a 10 ms burst) come as close as they can to telling queries apart without that.

## What would fix it

DuckDB 1.5.5's C API has a per-execution hook for scalar functions: `duckdb_scalar_function_set_init` gives each execution a state with a destroy callback. A state built at a query's start and destroyed at its end marks the query's lifetime. A signal would then belong to the query whose state is alive, and a late signal would be spent when that state is destroyed.

duckdb-rs registers `VScalar`s without this hook. It keeps the function pointer and the chunk constructor private. So the fix registers every scalar through raw FFI: `duckdb_create_scalar_function`, its parameters and return type, `duckdb_scalar_function_set_function`, `duckdb_scalar_function_set_init`, and a destroy callback for the state. The chunk reading moves from duckdb-rs's `DataChunkHandle` to the C API's vectors, as `connections::text_rows` already does for query results.

## Cost and scope

- Every scalar the surface registers (decide, choose, score, tag, annotate, recognize, relations, and the probability forms) moves to one registration helper.
- The rearm suite, `tools/host_signal.py`, and the signal unit tests keep their proofs. A new test sends a SIGINT after a query's last call and asserts the next query answers and the late signal was spent.
- No public name changes.

## Decision needed

None from Ian. The steering session schedules it when an embedding host reports a lost or late interrupt, or before the surface leaves beta.
