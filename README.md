# thinkthen

`thinkthen` puts a decider model in the shell. A decider model never writes text. It reads a state, answers a typed question, and returns probabilities that a script can branch on. The question is a yes/no, a pick from a list, or a rating on a scale. A command names the job, asks the question, and reads the evidence on standard input.

```sh
thinkthen decide 'Does the customer ask for a refund?' < message.txt
thinkthen choose 'Which kind of request is this?' bug feature question other < issue.txt
thinkthen filter 'Does this describe a bug that can be reproduced?' --jsonl --field /body < issues.jsonl
```

The first prints `true`, `false`, or `null`, and its exit code works in a shell `if`. The second prints a label. The third prints the records that pass. `--details` adds the probabilities behind any answer.

Those commands are the design. `specification/` is the contract, and code follows it. The code that has landed still speaks an earlier grammar, and `sdlc/planning/plan.md` says where the change stands.

## What it will and will not do

- The shell sequences programs. `jq` reshapes data. `thinkthen` judges meaning and does nothing else.
- Code parses the command line. The model reads only the question, the options, and the evidence.
- A yes, a no, an unresolved answer, and an error stay four different outcomes in the output and in the exit code.
- A backend is an address that speaks one wire shape, System One. TypeSafe's Jev is the first decider model. `THINKTHEN_API_KEY` holds the key and `THINKTHEN_BASE_URL` names the address. A local model is reached by a small server that presents the same shape.
- A run can be recorded and replayed with no network. A threshold is measured against labeled cases before anyone trusts it.

## What it is not for

- **A loop that needs many decisions a second.** One measured call took over 300 ms, and a shell tool adds a process start on top of that. No pipeline of separate processes reaches that rate. Record mode through a `coproc` serves a steady loop from one long-lived process, and that is the ceiling.
- **A call from inside a program written in another language.** Records, recordings, transforms, and exit codes buy a program nothing, because the program already holds its data. The honest answer there is a library over the same pure core, and `specification/roadmap.md` holds it for after version one.

`sdlc/planning/ten-use-cases.md` measured both against ten real uses.

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

The documentation has three kinds of page. [`demos/README.md`](demos/README.md) is the list of how-tos, and each one is a real shell job that the gate runs. `specification/` is the reference. This README is the tutorial and the explanation.

These seven pages show every command and all three question types, from the simplest use to the strongest. ADR 0016 chose them, and the rest of the 27 are in the list.

| How to | The job | |
| --- | --- | --- |
| Gate a script step on a yes/no answer | A support desk sends the messages that ask for money back to the refunds queue | [01](demos/01-refund-gate/) |
| Branch on a label with `choose` and `case` | A ticket goes to one of four teams, and a folder of notes is filed the same way | [02](demos/02-route-a-ticket/) |
| Lint a change by meaning and fail the build | A house rule no linter can check is asked of every changed hunk, and the build fails on the ones that break it | [43](demos/43-lint-a-change/) |
| Put the best matches first | A search brings back six wiki pages and the best three go to the reader | [06](demos/06-top-search-hits/) |
| Find the line that answers a question | One line of a long document answers it, or nothing fits and the tool says so | 15, coming |
| Build a triage pipeline that drafts, blocks, or asks a person | One request answers several questions at once, and a `jq` policy names the action | 16, coming |
| Pick a threshold from labeled cases | A cut you can defend, chosen on cases a person already answered | [13](demos/13-pick-a-threshold/) |

The how-to list also has a section on evals: grading a batch against a reusable definition, keeping a run that can be traced and replayed, picking a threshold, comparing two runs, checking the judge against human labels, and knowing what a run cost.

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
```

Cheapest rung first. No gate touches the network. `.github/workflows/gate.yml` runs the same four rungs on every push and every pull request.

## License

MIT. [`LICENSE`](LICENSE) holds the text.
