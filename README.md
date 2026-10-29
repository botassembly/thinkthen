# ThinkThen

ThinkThen gives programs ten functions that judge evidence by meaning.

Download the command archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). Extract it and put its command directory on `PATH`. Each archive includes the command and its licenses. Read the release runtime requirements before installing.

Rust applications use `cargo add thinkthen --no-default-features`. Install the command from source with `cargo install thinkthen`.

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

Language packages: [Python](sdk/python/README.md), [TypeScript](sdk/typescript/README.md), [Go](sdk/go/README.md), [Java](sdk/java/README.md), [Kotlin](sdk/kotlin/README.md), [C#](sdk/csharp/README.md), [R](sdk/r/README.md), [C](sdk/c/README.md), [C++](sdk/cpp/README.md), [Ruby](sdk/ruby/README.md), [Swift](sdk/swift/README.md) and [Zig](sdk/zig/README.md).

Database packages: [SQLite](extensions/sqlite/README.md), [PostgreSQL](extensions/postgres/README.md) and [DuckDB](databases/duckdb/README.md).
