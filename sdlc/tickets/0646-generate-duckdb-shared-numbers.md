# 0646: Take the DuckDB bridge's function numbers from the generated C header

Status: WITHDRAWN.

Opened as: 2026-10-19, superseded by 0693.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0693

Superseded by: 0693

## Outcome

- Changes: `databases/duckdb/cpp/src/`, `databases/duckdb/cpp/CMakeLists.txt`, `databases/duckdb/bridge/src/ffi/`.
- The C++ layer includes `libraries/c/include/thinkthen.h` and passes the request function numbers it names (`THINKTHEN_COMPLETE_REQUEST_FUNCTION_*`) across the bridge. The Rust bridge matches on the same numbers from the Rust source the header is generated from. No DuckDB-only generator output is added.
- No bare function number is left in `cpp/src` or in the bridge's `ffi` modules, and the private numbering (0 decide, 4 choose, 5 score, 6 tag, 8 rank or recognize, 9 relations) is gone.

### Proof

- On the M5 natively: `pm job run --machine m5 0646 SHA --memory 8 -- cargo test --locked --offline --manifest-path databases/duckdb/bridge/Cargo.toml` passes, and `sdlc/scripts/lint` passes. `sh libraries/c/check.sh` runs `generate-c-header.py --check`, which keeps the header the bridge reads current.
- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/duckdb/check.sh` passes its rank, nested, listed and image suites on both pinned DuckDB versions.

## Evidence

- The DuckDB C++ layer and the Rust bridge agree on call kinds through bare integers kept in step by hand. `databases/duckdb/cpp/src/portable.cpp` defines `RANK_KIND = 8` with the comment "the bridge reads the same number", and `bridge/src/ffi/portable_many/ffi.rs` defines `const RANK: i32 = 8`. `bridge/src/ffi/nested.rs` `ask` matches `8` and `9` for recognize and relations. `bridge/src/ffi/listed.rs` `set` matches `4`, `5` and `6`. `cpp/src/images.cpp` (`Answer` and the kind chosen from the function name) and `cpp/src/listed_result.cpp` (`Next`) branch on `0`, `4`, `5` and `6`. `cpp/src/bridge.hpp` passes `int32_t kind` with no named type. A change on one side compiles cleanly and breaks at run time.
- The generated header `libraries/c/include/thinkthen.h` names every function, with different numbers: decide 1, choose 2, tag 3, score 4, filter 5, rank 6, find 7, annotate 8, recognize 9, relate 10. The DuckDB C++ build does not include it today.
- 0693 deletes the C++ JSON parsing in `images.cpp` and `listed_result.cpp`, so some of these branches go with it; this ticket covers the numbers left after 0693. The header's `_V1` suffixes may drop under the 2026-10-19 no-version ruling before the freeze; use the names the header has at 0688.
