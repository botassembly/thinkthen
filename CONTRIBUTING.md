# Contributing

Thank you for helping. Read `README.md`, then `specification/README.md`. The specification is the contract, and code follows it.

## Before you open a pull request

Run the gate ladder, cheapest rung first:

```sh
sdlc/scripts/install
sdlc/scripts/lint
sdlc/scripts/test
sdlc/scripts/spec
sdlc/scripts/surfaces
```

No rung touches the network. Tests replay recorded responses, so you need no key.

## How work is recorded

`sdlc/` is the record. A problem goes in `sdlc/issues/`. Authorized work is a ticket in `sdlc/tickets/`. An architecture decision is an ADR in `sdlc/planning/`. A change that alters behavior updates the specification and its pages in the same commit.

## Reporting a bug

Use the bug template. Give the command, the output, and `thinkthen --version`. Report a security problem privately, as `SECURITY.md` says.
