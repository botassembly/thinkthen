# Rust crates worth knowing for the engine and the ten surfaces

Status: a study note for the rewrite of ADR 0017. It authorizes no dependency. The repository's rule stands: a second agent reviews any change that adds one.

Ian asked on 2026-09-20 whether crates such as Arrow and DataFusion could help with speed or with upkeep. The answer starts from one fact about this tool.

## The tool waits on the network and almost never on the processor

One judgment is a fifth to a third of a second of waiting. The work on this side is microseconds. The vendor documents 1,200 requests a minute, and a width of 3 to 4 already reaches it, measured on 2026-09-20. So a faster parser, a faster hash, or a parallel compute library changes nothing a user can feel. Speed comes from fewer requests: every question about one text in one request, equal pairs asked once, and a cache. A crate earns its place here by cutting copies at a language border, or by cutting code we maintain.

## What the tool uses today, read from `Cargo.toml`

`clap`, `ureq` with `rustls`, `serde`, `serde_json`, `sha2`, and `thiserror`. No async runtime. `ureq` is a blocking HTTP client, and `--jobs` is a set of threads. `rustls` is TLS in pure Rust, so no build links the system's OpenSSL.

**Keep this.** It is the best choice already made for the libraries, for three reasons. Prebuilt binaries for five languages on many platforms need no system TLS library, and a static Linux build works. A blocking client has no runtime to keep alive inside a forked PostgreSQL backend, a forked Python worker, or an R session, and that is the hardest problem the database survey found. The dependency tree stays small enough to audit. The rate limit caps the useful width at a handful of threads, so an async runtime would buy nothing against this backend. The stand-in engine in experiment 205 assumed an async runtime. Its findings should be read with that in mind, and the ADR should compare the two on code size and on fork safety before it adds `tokio`.

## Worth adopting when its surface arrives

| Crate | What it is | Why it helps here | The cost |
| --- | --- | --- | --- |
| The small Arrow crates: `arrow-array`, `arrow-schema`, `arrow-data` with the `ffi` feature | Arrow is a standard memory layout for a column. A column of strings is one block of bytes and a list of offsets. The C data interface passes a column between languages as two pointers | pandas, Polars, R, DuckDB, and DataFusion all speak it. One bulk door in the engine, "here is a column of strings", then serves Python, R, and DuckDB with no copy and no per-item object. The answers fit too: a boolean column with nulls is exactly yes, no, and not sure | The full `arrow` crate is large and slow to build. Take the small crates alone, behind a feature, and keep Arrow at the border. The core stays free of it |
| `csv` | The standard CSV reader, with correct quoting | The planned `--csv` and `--tsv` framing should use it. A hand-written CSV reader is a bug farm | Small |
| The binding generators: `pyo3` and `maturin`, `napi-rs`, `magnus` and `rb-sys`, `extendr`, `cbindgen`, `pgrx`, `duckdb`, `rusqlite` or `sqlite-loadable` | Each writes the glue for one host | Already chosen. They are the largest saving in code we would otherwise maintain | Each pins us to its release rhythm |
| `cargo-dist` | Builds release archives, an installer script, and a Homebrew formula from one config | The `curl` installer Ian ruled on, with no release scripts of our own | Already in the plan |

## Worth knowing, not adopting now

- **DataFusion** is a SQL query engine over Arrow. The tool runs no queries, so the core has no use for it. It matters in one way: several newer databases are built on it, and a DataFusion function takes Arrow columns. Once the Arrow door exists, a DataFusion function is a small eleventh surface.
- **Polars plugins** let a Rust function run inside a Polars expression. The same holds: cheap after the Arrow door, and coupled to Polars releases. The Arrow C interface gives Python users most of the gain with no coupling.
- **`tokio` with `reqwest` or `hyper`**, and HTTP/2. Many requests would share one connection. It would matter against a backend with no rate limit and a need for hundreds of requests in flight. It does not matter against 1,200 a minute. Revisit with the second backend.
- **An embedded store for the cache**, such as `redb` or SQLite through `rusqlite`. The cache is one small file per answer today. A folder of a million small files is slow and sits badly inside a database server. The files are also the reason a recording diffs in a pull request, and that is a selling point. Keep files for recordings. Ask the question again for the cache when the database extensions are built.
- **UniFFI** generates bindings for several languages from one interface file. It is less code and a slower, less native result, and it does not cover Node or R well. It loses to the speed rule.

## Not worth it here

`rayon` for parallel compute, `simd-json` or `sonic-rs` for faster JSON, `blake3` for faster hashing, and a custom allocator. Each speeds up work that takes microseconds beside a wait of hundreds of milliseconds. The digest is SHA-256 by specification in any case.

## What Ian can overturn

All of it. Nothing here is decided. The one firm recommendation is to keep the blocking client and `rustls` until a measurement argues for more.
