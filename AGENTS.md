# Agent instructions for thinkthen

The binary is `thinkthen`. The crates are `thinkthen-core` and `thinkthen`. Read `README.md` first, then `specification/README.md`, then `sdlc/planning/rust-standards.md`. The specification is the contract, and code follows it.

## Building

- Red-green test-driven development. Write the failing test, watch it fail for the stated reason, make it pass, then clean up.
- Build the simplest thing that works. YAGNI, DRY, locality of behavior, separation of concerns. A command family enters only with a real use behind it.
- The gate ladder is `sdlc/scripts/{install,lint,test,spec}`. Run the cheapest rung first and the whole ladder before handing back.
- `sdlc/ratchet.json` holds the source size ceiling. The ceiling equals the measured total. The commit that raises it says what grew, why it earns its lines, and where you looked for duplication to delete first.
- A second agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency. The review names what it checked.
- Commit as soon as a change is whole and push right away. Commit messages are imperative and active.
- Never add agent attribution to a commit or a pull request: no trailer, no co-author line, no "generated with".

## The pure core

`thinkthen-core` touches no file, no environment variable, no socket, no clock, and no process. Its `clippy.toml` bans them. The binary parses at the edge and hands typed values inward. Do not weaken either lint table. `lint` compares them against the accepted copies.

## The tool judges and never acts

`thinkthen` never runs a command, never treats free text as an instruction, and never writes a file the user did not name. A ticket that asks for any of those is wrong and stops.

## No network in a gate

Tests replay recorded responses. A live call to a paid backend runs only from `sdlc/scripts/live`, by hand, under a token cap, with Ian's authorization.

## Credentials

A key is read from the environment variable its backend profile names. It is never committed, logged, hashed, echoed in a plan, written to a recording, or sent to a host other than its profile's. A recording stores request bodies and responses and never headers.

## Public hygiene

This repository will go public. Never name a private project or a customer. Describe a consumer generically.

## Where decisions go

Every decision lands in `sdlc/`: an ADR for an architecture decision, an issue for a problem found, a ticket for work authorized. Tickets are numbered from 0001 in this repository, and a ticket lands through a worktree. A decision Ian cannot find later was not made. Say which ones he can overturn.
