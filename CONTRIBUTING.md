# Contributing

Thank you for helping. Read `README.md`, then `specification/README.md`. The specification is the contract, and code follows it.

## Build the command from a checkout

Build the command and put it on your `PATH`:

```sh
cargo build --locked --release -p thinkthen --bin thinkthen
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/thinkthen "$HOME/.local/bin/thinkthen"
export PATH="$HOME/.local/bin:$PATH"
```

## Before you open a pull request

Run the gate ladder, cheapest rung first:

```sh
sdlc/scripts/install
sdlc/scripts/lint
sdlc/scripts/test
sdlc/scripts/spec
sdlc/scripts/surfaces
```

No rung touches the network. Tests replay recorded responses, so you need no key. To run the first four rungs in GitHub Actions, start `.github/workflows/gate.yml` by hand.

## The four names

ADR 0015 fixes four names, and other pages link this table.

| Name | What it is | What runs it |
| --- | --- | --- |
| question file | JSON that holds what to ask, with its options, levels, and cuts | `thinkthen` |
| transform | one `.jq` file that reads saved rows | `jq` |
| how-to | a folder under `demos/` that holds a worked example you can run again | the `spec` gate |
| pipeline | a Bash script that runs your own job over your own input | Bash |

A transform is a metric or a policy. A metric reads a whole run and prints numbers. A policy reads one row and names an action. A question file holds one question. A question set holds several named questions, and `annotate` reads one.

## How work is recorded

`sdlc/` is the record. A problem goes in `sdlc/issues/`. Authorized work is a ticket in `sdlc/tickets/`. An architecture decision is an ADR in `sdlc/planning/`. A change that alters behavior updates the specification and its pages in the same commit.

- `sdlc/planning/design-study.md` says what the tool is, what version one holds, and how it fits with botassembly.
- `sdlc/planning/rust-standards.md` says how the code is judged. Every rule names the tool that enforces it.
- `sdlc/planning/plan.md` holds the build order and its state.
- `sdlc/planning/adr/` holds the decisions.

## Reporting a bug

Use the bug template. Give the command, the output, and `thinkthen --version`. Report a security problem privately, as `SECURITY.md` says.
