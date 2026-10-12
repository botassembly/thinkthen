<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg">
  <img src="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg" alt="ThinkThen" width="64" height="64">
</picture>

# thinkthen

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It works whether the model runs on your own servers, at TypeSafe or at another provider. ThinkThen makes these models simple to call from any language or database. Ask a typed question from a shell script or from your own program, and get back `true`, `false`, a label, or a number. A failed call never looks like an answer.

```sh
thinkthen decide 'Does the customer ask for money back?' < message.txt
```

```text
true
```

The exit code is 0 for yes, 1 for no, and 3 for not sure. A shell `if` can branch on it. The how-to [Gate a script step on a yes/no answer](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate) runs this question against a recorded answer.

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

## Why not call the model yourself?

- You write the question in plain words. Jev needs no labelled data ahead of time. [decide](https://thinkthen.dev/functions/decide/) shows one question and its answer.
- Jev is fast and cheap. On 2026-10-01 a call to Jev through a library took a median of 138 ms. ThinkThen's own work took about 2 ms of it. [Overhead](https://thinkthen.dev/learn/overhead/) gives every measurement and the published price.
- ThinkThen batches for you. Hand it a list or a stream of records, and it packs the records into as few requests as the batch settings allow. See [Functional patterns](https://thinkthen.dev/learn/functional/).
- ThinkThen caches and replays. A repeated question can reuse its saved answer. A test replays recorded answers with no key and no network. See [Caching and replay](https://thinkthen.dev/learn/caching/).
- You can swap the model and the provider. One `--backend` option points the same call at TypeSafe, another hosted vendor, or a model on your own machine. See [Backends](https://thinkthen.dev/install/backends/).
- ThinkThen is yours under the [MIT license](https://github.com/botassembly/thinkthen/blob/main/LICENSE). It runs from the command line and 24 bindings, 3 of them databases. Your questions live in your own code and SQL, not in one vendor's database function. See [Install](https://thinkthen.dev/install/).

## Install

```sh
curl -fsSL https://thinkthen.dev/install.sh | sh
```

On a Mac, Homebrew works too:

```sh
brew install botassembly/thinkthen/thinkthen
```

With Rust installed, `cargo install thinkthen` builds the command from crates.io. [Install](https://thinkthen.dev/install/) covers every other route and each of the 24 bindings.

A live run needs a backend. A hosted backend needs a key. [Backends](https://thinkthen.dev/install/backends/) shows how to get a TypeSafe key or point ThinkThen at another server, such as Ollama on your own machine. By default ThinkThen sends your question and text to TypeSafe. Read the terms that page links before you send sensitive text.

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

[Test a script with no network](https://github.com/botassembly/thinkthen/tree/main/demos/27-test-with-no-network) shows how a test uses a recording.

## Links

- [thinkthen.dev](https://thinkthen.dev/) teaches each function and every binding.
- The [specification](https://github.com/botassembly/thinkthen/blob/main/specification/README.md) holds the contracts.
- For agents, the [agent skill](skills/thinkthen/SKILL.md) explains function selection, evidence framing, abstention and bounded requests.
- For agent builders, the [MCP guide](libraries/mcp/README.md) runs the ten functions as tools over local stdio.
- [All how-tos](https://github.com/botassembly/thinkthen/blob/main/demos/README.md) lists every worked shell job.
- [CONTRIBUTING.md](https://github.com/botassembly/thinkthen/blob/main/CONTRIBUTING.md) explains how to build from a checkout and run the gate. No gate touches the network.
- [LICENSE](https://github.com/botassembly/thinkthen/blob/main/LICENSE) holds the MIT license text.
