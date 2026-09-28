# thinkthen

ThinkThen answers typed questions about text. Use the command in a shell script or a binding inside a program. The model returns structured judgments, not generated prose. A failed call stays separate from an answer.

## The ten functions

| Function | It asks | It prints |
| --- | --- | --- |
| `decide` | one yes/no question about one text | `true`, `false`, or `null` |
| `choose` | which one option fits best | a label, or `null` when none wins |
| `tag` | which labels apply | a JSON array of labels |
| `score` | where the text falls on a scale | a number on that scale |
| `filter` | the same yes/no of each record in a stream | the records that pass, byte for byte, in input order |
| `rank` | the same question of each record | the records, best fit first |
| `find` | which unit best answers the question | that unit, or `null` when an offered none wins |
| `annotate` | a saved set of named questions of each record | records with named answers or failure markers |
| `recognize` | which words name a thing, and what kind | names, kinds, offsets, and optional relations |
| `relate` | which relations hold between named entities | the edges that pass the rule |

Some functions use a threshold; others rank or select without one. A `null` value is a valid outcome where the function allows it, never a failed call. The [type contract](specification/types.md) names the answers and failures; the [result contract](specification/result.md) gives their full fields.

```sh
thinkthen decide 'Does the customer ask for a refund?' < message.txt
thinkthen filter 'Does this describe a bug that can be reproduced?' --jsonl --field /body < issues.jsonl
thinkthen annotate triage.json --jsonl < issues.jsonl
```

A command names the job, asks the question, and reads text on standard input.

## Install the command

Build it from a checkout and put it on your `PATH`:

```sh
cargo build --locked --release -p thinkthen --bin thinkthen
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/thinkthen "$HOME/.local/bin/thinkthen"
export PATH="$HOME/.local/bin:$PATH"
```

Then check it on a recorded answer. This sample needs no key and no network; `--replay` reads the answer from disk.

```sh
cd demos/27-test-with-no-network
thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --replay recording < report.txt
```

It prints `true`. `demos/27-test-with-no-network` shows how a test replays a recording.

## Languages and bindings

The command and bindings use the Rust engine. Python, TypeScript, Ruby, and R each have a native Rust adapter. C exposes a separate C API. The database extensions call the engine inside their hosts. Each linked README shows how to use that binding from source.

| Language or surface | What it is | README |
| --- | --- | --- |
| Rust | the crate; the command is one consumer of it | [libraries/rust](libraries/rust/README.md) |
| Python | the package, with pandas and Polars doors | [libraries/python](libraries/python/README.md) |
| TypeScript | a Node binding with a native addon | [libraries/typescript](libraries/typescript/README.md) |
| Ruby | a Ruby binding with a native extension | [libraries/ruby](libraries/ruby/README.md) |
| R | the package | [libraries/r](libraries/r/README.md) |
| C | a header and a library for C callers | [libraries/c](libraries/c/README.md) |
| Polars | the Rust feature | [libraries/polars](libraries/polars/README.md) |
| DuckDB | the extension | [databases/duckdb](databases/duckdb/README.md) |
| SQLite | the extension | [databases/sqlite](databases/sqlite/README.md) |
| PostgreSQL | the extension | [databases/postgresql](databases/postgresql/README.md) |

## How it works

- The shell sequences programs. `jq` reshapes data. `thinkthen` judges meaning and does nothing else.
- Code parses the command line. The model reads only the question, the options, and the evidence.
- Eligible record commands can send many records in one request. `--batch max` is their default; `--batch 1` sends one record per request. Records sharing a request can affect each other's answers, and a threshold tuned at one setting warns when it runs at another.
- A backend is an address that speaks one wire shape, System One. TypeSafe's Jev is the first System One model. `THINKTHEN_API_KEY` holds the key and `THINKTHEN_BASE_URL` names the address. A local model is reached by a small server that presents the same shape.
- The default address sends the question and evidence to TypeSafe. Its [customer agreement](https://typesafe.ai/legal/mca), [data processing addendum](https://typesafe.ai/legal/data-processing), and [privacy policy](https://typesafe.ai/legal/privacy-policy) describe data handling. The published privacy policy, checked 2026-09-28, gives no fixed API-input retention period. Check the terms governing your account before sending sensitive text.
- A run can be recorded and replayed with no network. A recording holds the evidence that was sent, so committing one publishes it. A threshold is measured against labeled cases before anyone trusts it.

For a long-lived shell loop, `decide --lines --batch 1` prints and flushes one answer per nonblank input line. `filter` prints only kept records; `rank` waits for the complete input before printing an order. A separate command process for every item adds startup time. Inside a program, use its language binding above.

## The answer cache

The cache is on by default.

- An entry holds the complete request and response, the judged evidence included. Filesystem access and backups can copy that evidence. No key enters an entry.
- Whoever can write the cache or recording folder controls the answers read from it, so keep that folder private to people whose answers you trust. A platform-default cache is created for its owner alone, and an existing Unix folder must already have mode `0700`. An explicitly named `--cache` folder keeps its user-owned mode.
- The first write binds a folder to the resolved backend address. Reusing it with another address fails before any request and tells you to restore the old settings or choose another folder.
- `--no-cache` runs a job that neither reads nor writes cached answers. `cache prune` is the only thing that removes entries.

## Usage counts

`thinkthen status` reports the resolved configuration, cache size, and request, retry, token, and cache-answer counts for the current UTC month and in total. It counts only what the command sends; a library, SQL extension, or data frame keeps its own process counts. The usage files hold no judged evidence and no key. They are local conservative statistics, not an account bill.

## How-tos

[`demos/README.md`](demos/README.md) lists every how-to, and each green one is a real shell job that the gate runs. Seven lead the list.

| How to | The job | |
| --- | --- | --- |
| Build a triage pipeline that drafts, blocks, or asks a person | One request judges three facts about a support ticket, and a tested `jq` policy publishes complete audit rows | [16](demos/16-triage-pipeline/) |
| Gate a script step on a yes/no answer | A support desk sends every message that asks for money back to the refunds queue | [01](demos/01-refund-gate/) |
| Branch on a label with `choose` and `case` | A ticket lands on one of four teams, and a folder of notes is filed the same way | [02](demos/02-route-a-ticket/) |
| Find the line that answers a question | One line of a long handbook answers the question, or nothing does and the tool says so | [15](demos/15-find-the-line/) |
| Lint a change by meaning and fail the build | A house rule nobody can grep for is checked on every changed hunk, and the build fails on the hunks that break it | [43](demos/43-lint-a-change/) |
| Put the best matches first | A search brings back six wiki pages, and the best three go to the reader | [06](demos/06-top-search-hits/) |
| Grade an assistant's answers with a rubric | Last week's assistant replies are graded against five written checks, with no second model asked whether they were good | [14](demos/14-grade-a-batch/) |

The how-to list also has a section on evals: grading answers against a written rubric, picking a threshold, checking the judge against human labels, and knowing what a run cost.

- [`demos/`](demos/README.md): the how-tos. Small real shell jobs as executable pages. They drive the design.
- `specification/`: the contract. Channels and exit codes, the threshold, the result, backends, and one page per command.

## Contributing

These four words name the four things a user writes or runs. ADR 0015 fixed them, and every other page links here.

| Thing | Name | What it is | What runs it |
| --- | --- | --- | --- |
| What to ask, with its options, levels, and cuts | question file | JSON | `thinkthen` |
| `jq` that reads saved rows | transform | One `.jq` file | `jq` |
| A whole worked example that can be run again | how-to | A folder under `demos/`: the page, the inputs, the question, the transform, the recording | The spec rung |
| A user's own job over the user's own input | pipeline | A Bash script | Bash |

A transform is one of two kinds. A metric reads a whole run and prints numbers. A policy reads one row and names an action. A question file holds one question. A question set holds several named questions, and `annotate` reads one.

- `sdlc/planning/design-study.md`: what the tool is, what version one holds, and how it fits with botassembly.
- `sdlc/planning/rust-standards.md`: how the code is judged. Every rule names the tool that enforces it.
- `sdlc/planning/plan.md`: the build order and its state.
- `sdlc/planning/adr/`: decisions made.

```sh
sdlc/scripts/install
sdlc/scripts/lint
sdlc/scripts/test
sdlc/scripts/spec
sdlc/scripts/surfaces
```

Cheapest rung first. No gate touches the network. `.github/workflows/gate.yml` runs the first four rungs on every push and every pull request.

## License

MIT. [`LICENSE`](LICENSE) holds the text.
