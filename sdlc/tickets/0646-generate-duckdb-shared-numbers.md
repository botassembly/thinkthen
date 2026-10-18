# 0646: Take the DuckDB bridge's kind numbers from the generated C header

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0693

## Outcome

- The C++ code and the bridge's `ffi.rs` files use the kind names from the generated C header. No DuckDB-only generator output is added.
- No bare kind integer is left in `cpp/src` or the bridge's `ffi.rs` files.

### Proof

- On the M5 through pm job, the installed DuckDB extension passes its rank, nested, listed and image suites on both pinned DuckDB versions.
- `generate.py --check` in `lint` covers the header the bridge reads.

## Evidence

The DuckDB C++ layer and the Rust bridge agree on result and call kinds through bare integers that are kept in step by hand. `databases/duckdb/cpp/src/portable.cpp` defines `RANK_KIND = 8` with the comment "the bridge reads the same number", and `bridge/src/ffi/nested.rs:81` checks `kind == 8`. `cpp/src/listed_result.cpp:41-58` and `cpp/src/images.cpp:100-103` branch on kinds 0, 4, 5 and 6 as literals. `cpp/src/bridge.hpp:72-101` passes `int32_t kind` with no named type. A change to the numbering on one side compiles cleanly and breaks at run time. The C header that 0637 generates already names these kinds.
