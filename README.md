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

## Where to read

- `specification/`: the contract. Channels and exit codes, the threshold, the result, backends, and one page per command.
- `demos/`: small real shell jobs as executable pages. They drive the design.
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

Cheapest rung first. No gate touches the network.
