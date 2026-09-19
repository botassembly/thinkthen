# thinkn

`thinkn` puts a decider model in the shell. A decider model never writes text. It reads a state, answers a typed question, and returns probabilities that a script can branch on. The question is a yes/no, a pick from a list, or a rating on a scale. The name reads as "think 'n decide".

```sh
thinkn decide if 'the customer explicitly requests a refund' < message.txt
thinkn decide which bug feature question other < issue.txt
thinkn decide where 'describes a reproducible bug' --jsonl --on /body < issues.jsonl
```

Those commands are the design. Nothing above is built yet. The repository holds the plan, the standards, and a scaffold with its gates.

## What it will and will not do

- The shell sequences programs. `jq` reshapes data. `thinkn` judges meaning and does nothing else.
- Code parses the command line. The model reads only the condition, the options, and the evidence.
- A yes, a no, an unsure, and an error stay four different outcomes in the output and in the exit code.
- The backend is a setting. TypeSafe's Jev is the first decider model. Any server that speaks the same small wire format can replace it, hosted or local.
- A run can be recorded and replayed with no network. A pass mark is measured against labeled cases before anyone trusts it.

## Where to read

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
