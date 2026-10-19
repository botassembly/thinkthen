# 0693: DuckDB reads typed packet answers from Rust

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `databases/duckdb/bridge/src/`, `databases/duckdb/cpp/src/`, `databases/duckdb/tools/receipt_types.py`, `databases/duckdb/tools/receipt_adapter.py`, `databases/duckdb/tools/source_checks.py`, `databases/duckdb/check.sh`, `databases/duckdb/NOTES.md`, `build-tools/results/registry.py`.
- The Rust bridge returns typed values built from the settled answer that 0612 puts in the packet: the answer, probability, pick and resolution, and typed rows for the `_many` tables and rank.
- The C++ JSON parsing of bridge results and the generated receipt parsers are deleted: `cpp/src/receipt.hpp`, `cpp/src/receipt_types_generated.hpp` and its four numbered parts, their generator `tools/receipt_types.py` and its `receipt_adapter.py`, the receipt entries in `build-tools/results/registry.py`, and the `receipt_types.py --check` steps in `check.sh` and `tools/source_checks.py`.
- The details column still returns DuckDB `JSON`, written by Rust.

### Proof

- On the M5 natively: `pm job run --machine m5 0693 SHA --memory 8 -- cargo test --locked --offline --manifest-path databases/duckdb/bridge/Cargo.toml` passes, and `sdlc/scripts/lint` passes with the receipt outputs gone from `build-tools/results/generate.py --all --check`.
- `git grep -il receipt databases/duckdb/cpp/src` finds nothing; before the change it lists nine files.
- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/duckdb/check.sh` passes its full suite on both pinned DuckDB versions.

## Evidence

- The DuckDB C++ layer re-parses JSON that Rust produced. In `databases/duckdb/cpp/src`, yyjson appears about 80 times in each of `receipt_types_generated_1.hpp`, `_2.hpp` and `_3.hpp`, and also in `receipt.hpp`, `complete.cpp`, `complete_files.cpp`, `files.cpp`, `images.cpp` and `details_result.hpp`. `native_input.hpp` parses caller input, not bridge results, and stays.
- `bridge/src/ffi/portable_many/ffi.rs` returns the `_many` and rank rows as JSON text for the table macro to unpack (`databases/duckdb/NOTES.md`). `cpp/src/listed_result.cpp` `DecodeListed` and `cpp/src/nested_result.cpp` decode the bridge's packed bytes per kind.
- `build-tools/results/registry.py` registers the `duckdb` `receipt-types` outputs, produced by `databases/duckdb/tools/receipt_types.py` from the native result schema. Each result change therefore lands in Rust, the generated receipt parsers and the C++ glue.
- The settled answer types come from 0612 in `crates/thinkthen` (`SettledAnswer`, read through `Call`). The bridge already depends on `thinkthen` and `thinkthen-host`. No DuckDB file is on the `LEGACY_FILES` list in `build-tools/results/answer_rules.py`, so that list needs no change here.
- The DuckDB tools are declared Linux-only in `sdlc/tools.json`, so the installed extension suites run in the 0619 container, not natively on the M5.
