<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg">
  <img src="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg" alt="ThinkThen" width="64" height="64">
</picture>

# thinkthen

ThinkThen answers typed questions about text. Ask from a shell script or from your own program, and get back `true`, `false`, a label, or a number. A failed call never looks like an answer. Use it to gate a script, label records, or grade answers in an eval.

```sh
thinkthen decide 'Does the customer ask for money back?' < message.txt
```

```text
true
```

The exit code is 0 for yes, 1 for no, and 3 for not sure. A shell `if` can branch on it. [How-to 01](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate) runs this question against a recorded answer.

## Install

```sh
curl -fsSL https://thinkthen.dev/install.sh | sh
```

On a Mac, Homebrew works too:

```sh
brew install botassembly/thinkthen/thinkthen
```

With Rust installed, `cargo install thinkthen` builds the command from crates.io.

A live run needs a backend. A hosted backend needs a key. [Backends](https://thinkthen.dev/install/backends/) shows how to get a TypeSafe key or point ThinkThen at another server, such as Ollama on your own machine.

### Try it with no key

Download the release's sample, then replay its recorded answer with no key and no network:

```sh
curl -fsSL https://github.com/botassembly/thinkthen/releases/latest/download/thinkthen-first-run.tar.gz | tar -xz && cd thinkthen-first-run
thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --replay recording < report.txt
```

```text
true
```

[How-to 27](https://github.com/botassembly/thinkthen/tree/main/demos/27-test-with-no-network) shows how a test uses a recording.

## The ten functions

| Function | It answers |
| --- | --- |
| `decide` | yes, no, or not sure |
| `choose` | the one option that fits best |
| `tag` | every label that applies |
| `score` | a number on a scale |
| `filter` | the records that pass a yes/no question |
| `rank` | the records, best first |
| `find` | the one unit of text that answers the question, or nothing |
| `annotate` | a saved set of named questions for each record |
| `recognize` | the names in a text and what kind each is |
| `relate` | the relations between named things |

The [specification](https://github.com/botassembly/thinkthen/blob/main/specification/README.md) gives each function's contract, its exit codes, and its failures.

## Languages

Every language uses the same Rust engine. Each page below gives the install line and a first call.

| Kind | Install pages |
| --- | --- |
| Command line | [Bash](https://thinkthen.dev/install/shell/) |
| Libraries | [Python](https://thinkthen.dev/install/python/), [pandas](https://thinkthen.dev/install/pandas/), [Polars](https://thinkthen.dev/install/polars/), [TypeScript](https://thinkthen.dev/install/typescript/), [Ruby](https://thinkthen.dev/install/ruby/), [R](https://thinkthen.dev/install/r/), [Rust](https://thinkthen.dev/install/rust/), [C](https://thinkthen.dev/install/c/), [C++](https://thinkthen.dev/install/cpp/), [C#](https://thinkthen.dev/install/csharp/), [Go](https://thinkthen.dev/install/go/), [Java](https://thinkthen.dev/install/java/), [Kotlin](https://thinkthen.dev/install/kotlin/), [Scala](https://thinkthen.dev/install/scala/), [Swift](https://thinkthen.dev/install/swift/), [Zig](https://thinkthen.dev/install/zig/), [PHP](https://thinkthen.dev/install/php/), [Dart](https://thinkthen.dev/install/dart/), [Ada](https://thinkthen.dev/install/ada/), [Objective-C](https://thinkthen.dev/install/objective-c/), [COBOL](https://thinkthen.dev/install/cobol/) |
| Databases | [DuckDB](https://thinkthen.dev/install/duckdb/), [SQLite](https://thinkthen.dev/install/sqlite/), [PostgreSQL](https://thinkthen.dev/install/postgresql/) |

Each folder under [libraries/](https://github.com/botassembly/thinkthen/tree/main/libraries) and [databases/](https://github.com/botassembly/thinkthen/tree/main/databases) has a README that builds that binding from source.

On 2026-10-01 a call to Jev took a median of 138 ms. ThinkThen's own work took about 2 ms of it. Each binding added a median of 3 ms or less, and the slowest single function added 6.7 ms. [Overhead](https://thinkthen.dev/learn/overhead/) gives every measurement and its spread.

## How-tos

- [Build a triage pipeline that drafts, blocks, or asks a person](https://github.com/botassembly/thinkthen/tree/main/demos/16-triage-pipeline)
- [Gate a script step on a yes/no answer](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate)
- [Branch on a label with `choose` and `case`](https://github.com/botassembly/thinkthen/tree/main/demos/02-route-a-ticket)
- [Lint a change by meaning and fail the build](https://github.com/botassembly/thinkthen/tree/main/demos/43-lint-a-change)
- [Grade an assistant's answers with a rubric](https://github.com/botassembly/thinkthen/tree/main/demos/14-grade-a-batch)

[All how-tos](https://github.com/botassembly/thinkthen/blob/main/demos/README.md) lists every page. Each green one is a real shell job that the gate runs.

## Your data

By default ThinkThen sends your question and text to TypeSafe. [Backends](https://thinkthen.dev/install/backends/) links TypeSafe's terms. Read them before you send sensitive text.

## Contributing

[CONTRIBUTING.md](https://github.com/botassembly/thinkthen/blob/main/CONTRIBUTING.md) explains how to build from a checkout, run the gate, and record work. It also defines the four names this repository uses. No gate touches the network.

## License

MIT. [LICENSE](https://github.com/botassembly/thinkthen/blob/main/LICENSE) holds the text.
