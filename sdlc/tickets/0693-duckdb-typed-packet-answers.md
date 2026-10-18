# 0693: DuckDB reads typed packet answers from Rust

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- The Rust bridge returns typed values built from the 0612 packet answer fields: the answer, probability, pick and resolution, and typed rows for `_many` and rank.
- The C++ JSON parsing of bridge results and the generated receipt parsers are deleted.
- The DuckDB files leave the 0637 answer-rule check's list of exceptions, if any are on it.

### Proof

- On the M5 through pm job, the installed DuckDB extension passes its full suite on both pinned versions.
- `git grep yyjson databases/duckdb/cpp/src` finds no parsing of bridge results.

## Evidence

The DuckDB C++ layer re-parses JSON that Rust produced. Eleven files in `databases/duckdb/cpp/src` use yyjson, including about 80 uses each in the generated `receipt_types_generated_1.hpp` and `receipt_types_generated_2.hpp`. The `_many` and rank functions return rows as JSON text (`databases/duckdb/NOTES.md`). Each result change therefore lands in Rust, the generated receipt parsers and the C++ glue.
