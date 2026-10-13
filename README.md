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

The exit code is 0 for yes, 1 for no, and 3 for not sure. A shell `if` can branch on it. The demo [Gate a script step on a yes/no answer](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate) runs this question against a recorded answer.

## The ten functions

| Function | It answers |
| --- | --- |
| [`decide`](https://thinkthen.dev/functions/decide/) | yes, no, or not sure |
| [`choose`](https://thinkthen.dev/functions/choose/) | the one option that fits best |
| [`tag`](https://thinkthen.dev/functions/tag/) | every label that applies |
| [`score`](https://thinkthen.dev/functions/score/) | a number on a scale |
| [`filter`](https://thinkthen.dev/functions/filter/) | the records that pass a yes/no question |
| [`rank`](https://thinkthen.dev/functions/rank/) | the records, best first |
| [`find`](https://thinkthen.dev/functions/find/) | the one unit of text that answers the question, or nothing |
| [`annotate`](https://thinkthen.dev/functions/annotate/) | a saved set of named questions for each record |
| [`recognize`](https://thinkthen.dev/functions/recognize/) | the names in a text and what kind each is |
| [`relate`](https://thinkthen.dev/functions/relate/) | the relations between named things |

The [specification](https://github.com/botassembly/thinkthen/blob/main/specification/README.md) gives each function's contract, its exit codes, and its failures.

## Why not call the model yourself?

- Your code can branch on the answer. `decide` exits 0 for yes, 1 for no and 3 for not sure, and a failure never looks like no. See [How answers work](https://thinkthen.dev/functions/answers/).
- You pay once. 306 song titles went out in one request: [11,468 input tokens instead of 88,933](sdlc/planning/adr/0055-decide-filter-and-rank-batches-send-each-record-once.md) one at a time. Each question keeps its own saved answer, however records group. See [Pay once](https://thinkthen.dev/start/pay-once/).
- Tests replay saved answers with no key and no network. See [Replay in tests](https://thinkthen.dev/start/replay/).
- You tune for free. Change the threshold on saved answers with zero new calls, and let `runs audit` pick it from your own labels. See [Tune for free](https://thinkthen.dev/start/tune/).
- `--plan` prints the first request and counts every request and its expected tokens. It sends nothing. See [See before you send](https://thinkthen.dev/start/plan/).
- One option switches the model or the provider: TypeSafe, another hosted vendor, or a model on your own machine. See [Models](https://thinkthen.dev/models/).
- The same code runs from the command line, 21 languages and 3 databases. In a library, ThinkThen's own work takes about 2 ms of a 138 ms call. See [Speed](https://thinkthen.dev/start/speed/), [Languages](https://thinkthen.dev/languages/) and [Databases](https://thinkthen.dev/databases/).
- ThinkThen is yours under the [MIT license](https://github.com/botassembly/thinkthen/blob/main/LICENSE). Your questions live in your own code and SQL, not in one vendor's database function.

## Install

```sh
curl -fsSL https://thinkthen.dev/install.sh | sh
```

On a Mac, Homebrew works too:

```sh
brew install botassembly/thinkthen/thinkthen
```

With Rust installed, `cargo install thinkthen` builds the command from crates.io. [Start](https://thinkthen.dev/start/) covers every other route, and [Languages](https://thinkthen.dev/languages/) and [Databases](https://thinkthen.dev/databases/) cover each of the 24 bindings.

A live run needs a backend. A hosted backend needs a key. [Models](https://thinkthen.dev/models/) shows how to point ThinkThen at TypeSafe or another server, such as Ollama on your own machine. By default ThinkThen sends your question and text to TypeSafe. Read the terms that page links before you send sensitive text.

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
- [All recipes](https://thinkthen.dev/recipes/) lists every finished job, by reader.
- [CONTRIBUTING.md](https://github.com/botassembly/thinkthen/blob/main/CONTRIBUTING.md) explains how to build from a checkout and run the gate. No gate touches the network.
- [LICENSE](https://github.com/botassembly/thinkthen/blob/main/LICENSE) holds the MIT license text.
