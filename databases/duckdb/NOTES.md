# DuckDB surface notes

The shipped extension is the C++ extension in `cpp/`, linked with the Rust static library in `bridge/`. The older pure Rust C API extension is retired. Its notes stay at tag `surfaces-wave7-frozen-2026-09-24b` as history.

## Layout

- `cpp/src/thinkthen.cpp` loads the extension and registers every SQL form. `portable.cpp` holds the scalars, the `_many` tables and the `thinkthen_rank` table macro. `find.cpp`, `nested.cpp`, `plan.cpp`, `relate.cpp` and `usage.cpp` hold their own forms. `removed.cpp` registers `thinkthen_warm` and `thinkthen_probability`, which only refuse with their replacements.
- `cpp/src/bridge.hpp` declares every Rust function the C++ side calls. Each one lives in a file named `ffi.rs` under `bridge/src/`, because `policy.py` allows `unsafe` only there.
- `bridge/src/ffi/portable_many/ffi.rs` reads one keyed JSON object, runs the packed call or the rank, and returns its rows as JSON text for the table macro to unpack. `bridge/src/ffi/scalar/` and `portable_scalar/` run grouped scalar calls.
- `src/engines.rs` and `src/signal.rs` are the bridge's engine registry and SIGINT handler. The rest of `src/` and the root `Cargo.toml` are the retired workspace, kept for the binding policy.

## Choices

- A struct result writes every member. A member the verb lacks is `NULL`.
- A `NULL` member inside a list argument makes that row `NULL`.
- An `@file` that is not UTF-8 reads `thinkthen local: the question file PATH was not read: it is not UTF-8 text`.
- A member verb whose one answer failed reads `thinkthen backend: the backend's answer could not be read: CAUSE`, with the cause in plain words.
- `thinkthen_rank` is a table macro over the bridge's keyed reader, so it shares the `_many` input rules and ends in `ORDER BY rank` (ticket 0378).

## Limits

- A SIGINT that lands between a chunked query's last engine call and its return cancels nothing.
- Main has no per-call request limit. Under `thinkthen_max_requests_total`, a call sends only as many texts as the total leaves, then refuses with the total's sentence.
- `thinkthen_annotate` takes each record as JSON text. A set member's `on` pointer reads its part of the record.
- `thinkthen_recognize` takes kind names with no descriptions. The conformance runner checks recognize cases through `thinkthen_relations`, which reads the case's own question file.

## Tests

Every suite under `tools/` starts its own loopback backend and runs each child with a fake key, a loopback address, and fresh XDG folders. `harness.guard` refuses anything else. `check.sh` runs them after the builds, the source checks and one call through the stock CLI.

## 0403 preparation

The accepted design retains ADR 0081's C++ API and the Rust bridge. Both version builds use the unchanged C++ sources. Header compatibility is untested at this preparation checkpoint; add a guard only after a demonstrated difference. The old `older-host/` cache and pre-existing build folders remain untouched. New consumers resolve matching tools beneath `duckdb/<version>/`.
