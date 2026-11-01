# ThinkThen

ThinkThen gives programs ten functions that judge evidence by meaning.

Install the command on Linux or macOS with `curl -fsSL https://thinkthen.dev/install.sh | sh`, or in Windows PowerShell with `irm https://thinkthen.dev/install.ps1 | iex`. Homebrew users run `brew install botassembly/thinkthen/thinkthen`.

Download the command archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). Extract it and put its command directory on `PATH`. Each archive includes the command and its licenses. Read the release runtime requirements before installing.

Rust applications use `cargo add thinkthen --no-default-features`. Install the command from source with `cargo install thinkthen`.

Package-site installs follow publication of 0.2.0: [PyPI](sdk/python/README.md), [npm](sdk/typescript/README.md), [RubyGems](sdk/ruby/README.md), [NuGet](sdk/csharp/README.md), [Maven Central for Java](sdk/java/README.md) and [Kotlin](sdk/kotlin/README.md), and [r-universe for R](sdk/r/README.md). Use the Go module `github.com/botassembly/thinkthen/sdk/go/thinkthen` at `v0.2.0` or [Swift Package Manager](sdk/swift/README.md) at version `0.2.0`.

The `native` crate feature exposes unstable integration helpers for bindings, proxies and database hosts. These helpers may change or disappear without the public Rust API compatibility guarantees. Applications should use the typed Rust API.

The example uses the stub backend and needs no key or service. On Linux x86-64:

```sh
mkdir thinkthen-0.2.0
tar -xzf thinkthen-0.2.0-linux-x86_64.tar.gz -C thinkthen-0.2.0
printf '%s\n' 'Please refund my order.' > message.txt
./thinkthen-0.2.0/thinkthen decide --backend stub --json \
  'Does the customer ask for money back?' < message.txt
```

Use the matching archive name on Linux arm64 or Apple Silicon macOS. On Windows:

```powershell
Expand-Archive .\thinkthen-0.2.0-windows-x86_64.zip .\thinkthen-0.2.0
Set-Content -Encoding utf8 message.txt 'Please refund my order.'
Get-Content -Raw message.txt | .\thinkthen-0.2.0\thinkthen.exe decide --backend stub --json 'Does the customer ask for money back?'
```

[Command API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/crates/thinkthen/src/cli/grammar.rs). [Rust API reference](https://docs.rs/thinkthen/0.2.0/thinkthen/).

[Where record text goes](docs/surfaces/service.md) describes the default service and connection settings.

Language packages: [Python](sdk/python/README.md), [TypeScript](sdk/typescript/README.md), [Go](sdk/go/README.md), [Java](sdk/java/README.md), [Kotlin](sdk/kotlin/README.md), [C#](sdk/csharp/README.md), [R](sdk/r/README.md), [C](sdk/c/README.md), [C++](sdk/cpp/README.md), [Ruby](sdk/ruby/README.md), [Swift](sdk/swift/README.md) and [Zig](sdk/zig/README.md).

Database packages: [SQLite](extensions/sqlite/README.md), [PostgreSQL](extensions/postgres/README.md) and [DuckDB](databases/duckdb/README.md).

## Verify a download

Download `SHA256SUMS` with the release archive and verify the archive against its matching entry before extracting or executing it. On Linux, run `sha256sum --check --ignore-missing SHA256SUMS` in the download directory and require the archive’s `OK` result.

With a current [GitHub CLI](https://cli.github.com/manual/gh_attestation_verify), verify the downloaded archive’s provenance with `gh attestation verify /path/to/archive --repo botassembly/thinkthen --signer-workflow botassembly/thinkthen/.github/workflows/release-prepare.yml`. Require a successful result for the expected release before using the archive. Assets with release-assembly provenance require `--predicate-type https://github.com/botassembly/thinkthen/attestations/release-assembly/v1`; inspect its recorded original product sources as well as the assembly source. A matching checksum establishes file integrity; the attestation also verifies the signing workflow’s identity.
