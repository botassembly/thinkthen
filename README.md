# ThinkThen

ThinkThen answers questions about text, from your shell or your code. Ask decide whether a support message requests a refund, and it says yes, no or not sure.

## Install

macOS or Linux:

```sh
curl -fsSL https://thinkthen.dev/install.sh | sh
```

Windows PowerShell:

```powershell
irm https://thinkthen.dev/install.ps1 | iex
```

Homebrew users run `brew install botassembly/thinkthen/thinkthen`. [Installation](https://thinkthen.dev/install/) covers Cargo and the release archives.

Each question is a paid call to Jev, the default model. ThinkThen reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one, and how to use OpenAI instead.

## Ask a question

Pipe in the text, and put the question in quotes:

```sh
echo 'Please refund my order.' |
  thinkthen decide 'Does the customer request a refund?'
```

Output:

```text
true
```

decide prints `true` for yes and `false` for no. Set a low and a high bar, and it prints `unsure` between them, so you can send that message to a person.

## The ten functions

- [decide](https://thinkthen.dev/functions/decide/) answers a yes-or-no question: yes, no or not sure.
- [choose](https://thinkthen.dev/functions/choose/) picks the one option from your list that fits, or not sure.
- [filter](https://thinkthen.dev/functions/filter/) keeps only the texts that pass a yes-or-no question.
- [tag](https://thinkthen.dev/functions/tag/) gives every label from your list that fits.
- [score](https://thinkthen.dev/functions/score/) places a text on your levels, as a number from 0 to 1.
- [rank](https://thinkthen.dev/functions/rank/) orders texts by your levels, highest first.
- [find](https://thinkthen.dev/functions/find/) points to the line that answers your question, or none.
- [recognize](https://thinkthen.dev/functions/recognize/) pulls out names, each with its kind.
- [relate](https://thinkthen.dev/functions/relate/) links names by the relations you allow.
- [annotate](https://thinkthen.dev/functions/annotate/) answers several questions about each text, like a form.

## SDKs

The same ten functions, in your code or your database:

- [Python SDK](https://thinkthen.dev/sdks/python/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/python.md)
- [TypeScript SDK](https://thinkthen.dev/sdks/typescript/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/typescript.md)
- [Go SDK](https://thinkthen.dev/sdks/go/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/go.md)
- [Java SDK](https://thinkthen.dev/sdks/java/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/java.md)
- [Kotlin SDK](https://thinkthen.dev/sdks/kotlin/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/kotlin.md)
- [C# SDK](https://thinkthen.dev/sdks/csharp/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/csharp.md)
- [R SDK](https://thinkthen.dev/sdks/r/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/r.md)
- [Ruby SDK](https://thinkthen.dev/sdks/ruby/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/ruby.md)
- [Rust SDK](https://thinkthen.dev/sdks/rust/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/rust.md)
- [Swift SDK](https://thinkthen.dev/sdks/swift/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/swift.md)
- [Zig SDK](https://thinkthen.dev/sdks/zig/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/zig.md)
- [C SDK](https://thinkthen.dev/sdks/c/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/c-cpp.md)
- [C++ SDK](https://thinkthen.dev/sdks/cpp/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/c-cpp.md)
- [DuckDB](https://thinkthen.dev/sdks/duckdb/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/duckdb.md)
- [SQLite](https://thinkthen.dev/sdks/sqlite/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/sqlite.md)
- [PostgreSQL](https://thinkthen.dev/sdks/postgresql/), with its [guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/postgresql.md)

The [command guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/bash.md) covers every option of the command, and [Where your text goes](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/service.md) covers keys and connection settings.

## Verify a download

Download `SHA256SUMS` with the release archive and verify the archive against its matching entry before extracting or executing it. On Linux, run `sha256sum --check --ignore-missing SHA256SUMS` in the download directory and require the archive’s `OK` result.

With a current [GitHub CLI](https://cli.github.com/manual/gh_attestation_verify), verify the downloaded archive’s provenance with `gh attestation verify /path/to/archive --repo botassembly/thinkthen --signer-workflow botassembly/thinkthen/.github/workflows/release-prepare.yml`. Require a successful result for the expected release before using the archive. Assets with release-assembly provenance require `--predicate-type https://github.com/botassembly/thinkthen/attestations/release-assembly/v1`; inspect its recorded original product sources as well as the assembly source. A matching checksum establishes file integrity; the attestation also verifies the signing workflow’s identity.
