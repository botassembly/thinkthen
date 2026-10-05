# DuckDB community build

The repository root uses DuckDB's extension template build. It compiles the SQL adapter and Rust bridge from source against the selected DuckDB checkout. DuckDB supplies the static library, loader and metadata footer. The separate [standalone build](../cpp/README.md) retains its pinned 1.5.4/1.5.5 archives and installation route.

Prepare public source and Rust inputs with network access:

```sh
git submodule update --init --recursive
make configure_ci
```

`configure_ci` installs Rust 1.95.0, its pinned components and the selected target, then fetches locked bridge dependencies. Community Linux CI runs this preparation inside its build container. macOS CI selects `OSX_BUILD_ARCH`; preparation installs that target for the pinned Rust toolchain.

Build and test after preparation:

```sh
make release
make test
```

The bridge builds offline with its release profile, bundled SQLite and position-independent objects. Cargo checks source changes on each build. Rust's reported native libraries follow the archive in the link command. Linux hides archive symbols; macOS hides the Rust archive and retains DuckDB's initialization export. DuckDB appends the footer after linking.

Community source starts at DuckDB 1.5.6. Community CI can select later DuckDB releases and rebuild against them. A C++ extension requires a matching DuckDB host version and platform. A signed artifact for one DuckDB version does not establish compatibility with a different version embedded in dbt.

The native target paths cover Linux x86-64/ARM64 and macOS ARM64/Intel. The Apple build forwards DuckDB's deployment target and SDK to Rust and its native dependencies. It refuses a Rust standard library with a higher deployment requirement. Windows and WebAssembly need a bridge port because the bridge uses POSIX signals and pipes. Optional musl targets disable Rust's default static C runtime when linking into the shared extension.

Build and load qualification is recorded in the ticket's landing record. The adapter alone establishes no platform qualification, signed publication or available community installation. DuckDB controls acceptance, signing and publication.
