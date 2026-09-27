# 0201 prerequisite: C++/Rust extension and caller-session aggregate

This is a bounded proof, not the replacement DuckDB surface. It registers one nested scalar and one aggregate. It makes no engine call. The old Rust extension remains the product until ticket 0201's later migration passes review.

## Exact inputs

| Input | Origin and pin | License |
| --- | --- | --- |
| DuckDB source and C++ headers | `https://github.com/duckdb/duckdb.git`, tag `v1.5.5`, commit `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa` | MIT, source `LICENSE` |
| DuckDB release static archives | [Official v1.5.5 asset](https://github.com/duckdb/duckdb/releases/download/v1.5.5/static-libs-linux-amd64.zip), SHA-256 `deb47c5300f3c99725e84cdb14d214c3b12bbd748b613b1698b938c894cb68eb` from the [release API](https://api.github.com/repos/duckdb/duckdb/releases/tags/v1.5.5) | DuckDB MIT |
| Stock DuckDB CLI | Existing `tools/version.env` v1.5.5 pin, binary SHA-256 `3d33b1df037cb049155c393778df7853fafb23e9d49d7c9cacdde4dd67155788` | DuckDB MIT |
| Stock DuckDB Python module | Existing cached Python 3.13 environment, `duckdb==1.5.5` | DuckDB MIT |
| C++ extension template | Read-only reference `https://github.com/duckdb/extension-template.git`, commit `cfaf3e236008e782d27f4341b0ee036002d0a449`; no template source copied into this proof | MIT, template `LICENSE` |

The proof used Rust 1.95.0, CMake 3.28.3, GCC 13.3.0, Python 3.13.5, and the existing Linux x86-64 CLI. Its C bridge uses only the Rust standard library. `bridge.rs` allocates a length-prefixed three-field reply: UTF-8 text, and two UTF-8 list elements. C++ bounds-checks every length, copies all fields into DuckDB-owned `STRUCT(text VARCHAR, tags VARCHAR[])`, and calls the Rust free function exactly once. Rust catches unwind and returns a tagged status; the C ABI contains no DuckDB or Rust layout. The C++ binder retains a weak reference to the calling `ClientContext`, and aggregate update reads `thinkthen_probe_limit` from that context for the current execution. A prepared statement therefore sees a subsequent `SET`.

## Reproduce with cached inputs

Start at the repository root. `DUCKDB_SOURCE` is a checkout at the commit above. `DUCKDB_STATIC` is the unzipped official asset after its SHA-256 check. The build commands are the only heavy steps and use the canonical lock.

```sh
export THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-heavy.lock
export DUCKDB_SOURCE=/absolute/path/to/duckdb-v1.5.5
export DUCKDB_STATIC=/absolute/path/to/verified-static-libs
export PROOF_BUILD="$PWD/target/codex-builds/0201"
mkdir -p "$PROOF_BUILD/rust"
flock -o "$THINKTHEN_HEAVY_LOCK" rustc --edition 2024 -O --crate-type staticlib databases/duckdb/proof/0201/src/bridge.rs -o "$PROOF_BUILD/rust/libthinkthen_probe.a"
flock -o "$THINKTHEN_HEAVY_LOCK" cmake -S "$DUCKDB_SOURCE" -B "$PROOF_BUILD/cmake_make" -G 'Unix Makefiles' -DCMAKE_BUILD_TYPE=Release -DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF -DEXTENSION_STATIC_BUILD=OFF -DDUCKDB_EXTENSION_CONFIGS="$PWD/databases/duckdb/proof/0201/extension_config.cmake" -DTHINKTHEN_PROBE_RUST_STATICLIB="$PROOF_BUILD/rust/libthinkthen_probe.a" -DTHINKTHEN_PROBE_DUCKDB_STATIC_DIR="$DUCKDB_STATIC"
flock -o "$THINKTHEN_HEAVY_LOCK" cmake --build "$PROOF_BUILD/cmake_make" --target thinkthen_probe_loadable_extension -j 2
/home/ian/.cache/thinkthen-toolchains/duckdb/v1.5.5/venv/bin/python databases/duckdb/proof/0201/verify.py --cli /home/ian/.cache/thinkthen-toolchains/duckdb/v1.5.5/duckdb --extension "$PROOF_BUILD/cmake_make/extension/thinkthen_probe/thinkthen_probe.duckdb_extension"
```

## Observed result and limit

The first loadable build left DuckDB C++ symbols unresolved. Stock CLI v1.5.5 rejected it at `typeinfo for duckdb::BaseScalarFunction`. Its dynamic symbol table has no such export. Linking the official static release archives fixed that load without rebuilding DuckDB. The linked proof artifact is 65,247,454 bytes, SHA-256 `c0194a86bbc81064b2b098e3016e0eba844f4c20c3ebff47d7ac44b51001d27e` on this Linux host. Stock CLI returned the nested struct/list and an aggregate value of 7, then 13 on a prepared `EXECUTE` after `SET`. Two Python connections returned 11, 29, 11 in alternating calls. This is direct evidence that aggregate update reads the calling session rather than a kept second connection. The forced Rust panic became tagged error 4 without unwinding across C.

The forced panic **also printed its Rust panic message to stderr twice** through the default hook. This proof does not pass the product secrecy boundary. Before porting any verb, the production bridge needs the existing safe panic/error guard or another reviewed mechanism that prevents panic text, key, and evidence from reaching stderr. Do not copy this proof panic path into the product. The later migration must also prove DuckDB exception, cancellation, query-lifetime, and full value ownership paths; this proof does not establish them.

Observed outer wall times on this host: Rust staticlib compile 0.12 s, CMake configure 2.50 s, first successful C++ compile/link 4.53 s, static-archive relink 1.24 s, final stock CLI/Python verifier 0.12 s. The build timers wrapped `flock -o`, so their queue wait was not separated and they are not pure compile timings. No full DuckDB or ThinkThen gate ran.
