# thinkthen

`thinkthen` puts a System One model in the shell. The model never writes text. It reads a state, answers a typed question, and returns probabilities that a script can branch on. The question is a yes/no, one pick, every applicable label, or a level on a scale. A command names the job, asks the question, and reads the evidence on standard input.

```sh
thinkthen decide 'Does the customer ask for a refund?' < message.txt
thinkthen choose 'Which kind of request is this?' bug feature question other < issue.txt
thinkthen tag 'Which topics?' billing urgent < message.txt
thinkthen filter 'Does this describe a bug that can be reproduced?' --jsonl --field /body < issues.jsonl
thinkthen find 'Which line answers the question?' --lines < handbook.txt
```

The first prints `true`, `false`, or `null`, and its exit code works in a shell `if`. The second prints one label. The third prints every applicable label as a JSON array. The fourth prints the records that pass. The fifth sends the bounded set together and returns the best original unit. The [type contract](specification/types.md) names their meanings; the [result contract](specification/result.md) fixes detailed fields.

Those commands are the design. `specification/` is the contract, and code follows it. `annotate` reads a saved question set when several questions belong on the same input.

## First run

From a source checkout, this sample needs no key or network. It holds one bug report and the recorded answer to one question about it.

```sh
cargo build --release -p thinkthen --bin thinkthen
cd demos/27-test-with-no-network
../../target/release/thinkthen decide \
  'Does this report say what the person did before the problem appeared?' \
  --replay recording < report.txt
```

It prints `true`. `demos/27-test-with-no-network` shows how a test replays a recording.

## What it will and will not do

- The shell sequences programs. `jq` reshapes data. `thinkthen` judges meaning and does nothing else.
- Code parses the command line. The model reads only the question, the options, and the evidence.
- A yes, a no, a not sure answer, and a broken run stay four different outcomes in the output and in the exit code.
- A backend is an address that speaks one wire shape, System One. TypeSafe's Jev is the first System One model. `THINKTHEN_API_KEY` holds the key and `THINKTHEN_BASE_URL` names the address. A local model is reached by a small server that presents the same shape.
- The default address sends the question and evidence to TypeSafe. Its [customer agreement](https://typesafe.ai/legal/mca), [data processing addendum](https://typesafe.ai/legal/data-processing), and [privacy policy](https://typesafe.ai/legal/privacy-policy) describe data handling. The published privacy policy, checked 2026-09-28, gives no fixed API-input retention period. Check the terms governing your account before sending sensitive text.
- A run can be recorded and replayed with no network. A recording holds the evidence that was sent, so committing one publishes it. A threshold is measured against labeled cases before anyone trusts it.

The answer cache is on by default. Cache entries contain the complete request and response, including the evidence being judged. Filesystem access and backups can copy that evidence. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. A platform-default cache is created for its owner alone and an existing Unix folder must already have mode `0700`; an explicitly named `--cache` folder keeps its user-owned mode. The first write binds a folder to the resolved backend address. Reusing it with another address fails before any request and tells the user to restore the old settings or choose another folder. No key enters an entry. Use `--no-cache` for a run that must neither read nor write cached answers.

`thinkthen status` reports the resolved configuration, cache size, and local request, retry, token, and cache-answer counts for the current UTC month and in total. It counts only what the command sends. A library, SQL extension, or data frame keeps its counts in memory for its own process, and `status` never sees them. The count-only usage files live beside the platform cache and contain no judged evidence or key. Older builds can read the monthly files; newer builds keep retry totals in separate sidecars. They are local conservative statistics rather than an account bill.

## What it is not for

- **A loop that needs many decisions a second.** Each decision waits on a network round trip to a model, and a shell tool adds a process start to each one. A pipeline of separate processes pays both for every decision. For a `coproc` loop that sends one line and waits for one reply, use `decide --lines --batch 1`: it prints one result per nonblank input line and flushes it. `filter` prints only kept records, so a dropped line gives the loop no reply. `rank` waits for the complete input before it prints an order. Record mode keeps one process alive for the loop, but each decision still waits on the model.
- **A call from inside a program written in another language.** Rust, Python, TypeScript, Ruby, R, C, and Polars have libraries under `libraries/`. DuckDB, PostgreSQL, and SQLite have extensions under `databases/`. Their APIs return a value and run facts within the host program.

`sdlc/planning/ten-use-cases.md` measured both against ten real uses.

## Use it from Rust

The same crate is a library. A dependency on `thinkthen` with `default-features = false` leaves out the command and its argument parser. Every call blocks and returns `Result<_, thinkthen::Error>`. The error has six kinds, and `retryable()` says whether the same call may succeed later.

```rust
let engine = thinkthen::Engine::from_env()?;
let question = thinkthen::Question::decide("Asks for money back.")?.cut_at(0.9)?;
for row in engine.filter(&question, ["Please refund my order.", "Where is my parcel?"]) {
    println!("{}", row?);
}
```

`Engine::from_env` reads the same variables and configuration file as the command, including `cache: false`. A bare `Engine::builder()` starts with library defaults and does not read that file. Call its `no_cache()` setter to turn the cache off. `throttle(n)` caps the requests in flight for the whole process. The bulk calls `filter`, `decide_many`, and `annotate` read any iterator lazily and return rows in input order. `sdlc/planning/libraries/rust.md` holds the goals, and ticket 0084 holds the frozen declarations.

## Four names

These four words name the four things a user writes or runs. ADR 0015 fixed them, and every other page links here.

| Thing | Name | What it is | What runs it |
| --- | --- | --- | --- |
| What to ask, with its options, levels, and cuts | question file | JSON | `thinkthen` |
| `jq` that reads saved rows | transform | One `.jq` file | `jq` |
| A whole worked example that can be run again | how-to | A folder under `demos/`: the page, the inputs, the question, the transform, the recording | The spec rung |
| A user's own job over the user's own input | pipeline | A Bash script | Bash |

A transform is one of two kinds. A metric reads a whole run and prints numbers. A policy reads one row and names an action.

A question file holds one question. A question set holds several named questions, and each entry has the shape of a question file. `annotate` reads a question set.

## Where to read

The documentation has three kinds of page. [`demos/README.md`](demos/README.md) is the list of how-tos, and each green one is a real shell job that the gate runs. `specification/` is the reference. This README is the tutorial and the explanation.

The flagship leads these seven pages. The remaining pages move from the simplest command to its supporting details. ADR 0018 chose the original twenty, and accepted ticket 0088 added the `relate` how-to as page 45.

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
- `sdlc/planning/design-study.md`: what the tool is, what version one holds, how it fits with botassembly, and the questions waiting on Ian.
- `sdlc/planning/rust-standards.md`: how the code is judged. Every rule names the tool that enforces it.
- `sdlc/planning/plan.md`: the build order and its state.
- `sdlc/planning/adr/`: decisions made.

## Gates

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
