# DuckDB community build

The repository root uses DuckDB's extension template build. It compiles the SQL adapter and Rust bridge from source against the selected DuckDB checkout. DuckDB supplies the static library, loader and metadata footer. The installed checks and release packages stage artifacts from the same approved source hosts.

Prepare public source and Rust inputs with network access:

```sh
git submodule update --init --recursive
make configure_ci
```

`configure_ci` installs Rust 1.95.0, its pinned components and the selected target, then fetches locked bridge dependencies. Community Linux CI runs this preparation inside its build container. With `LINUX_CI_IN_DOCKER=1`, Cargo and Rust homes default to `build/community-rust` in the mounted checkout, so the separate release container retains the prepared inputs. Explicit caller homes still take precedence. macOS CI selects `OSX_BUILD_ARCH`; preparation installs that target for the pinned Rust toolchain.

Build and test after preparation:

```sh
make release
make test
```

The root `vcpkg.json` follows DuckDB’s extension template. Community CI enables `VCPKG_BUILD` and supplies its pinned vcpkg toolchain; the manifest retains the template’s OpenSSL dependency and CI tool overlays. A build without that toolchain does not reproduce community CI.

The 1.5.6 build also includes DuckDB's bundled JSON extension, which the retained SQL macros require. DuckDB 2.0 includes JSON in its default configuration. The test target requires ThinkThen to load and adapts required-extension flags to DuckDB's selected test runner; the newer runner fails on requirement-based skips and uses two workers.

The bridge builds offline with its release profile, bundled SQLite and position-independent objects. Cargo checks source changes on each build. Rust's reported native libraries follow the archive in the link command. Linux hides archive symbols; macOS hides the Rust archive and retains DuckDB's initialization export. DuckDB appends the footer after linking.

The source authority selects DuckDB 1.5.6 and the pinned 2.0 preview. Prepare their source and Python hosts with `sh databases/duckdb/tools/setup.sh --fetch`. The preview Python client compiles against the same approved core source as the extension and statically includes `core_functions` and `json`. Compiler commands use the shared build admission and two jobs. A C++ extension requires a matching DuckDB host version and platform. A signed artifact for one DuckDB version does not establish compatibility with a different version embedded in dbt.

The native target paths cover Linux x86-64/ARM64 and macOS ARM64/Intel. The Apple build forwards DuckDB's deployment target and SDK to Rust and its native dependencies. It refuses a Rust standard library with a higher deployment requirement. Windows and WebAssembly need a bridge port because the bridge uses POSIX signals and pipes. Optional musl targets disable Rust's default static C runtime when linking into the shared extension.

Build and load qualification is recorded in the ticket's landing record. The adapter alone establishes no platform qualification, signed publication or available community installation. DuckDB controls acceptance, signing and publication.
