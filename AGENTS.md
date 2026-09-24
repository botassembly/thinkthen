# Agent instructions for thinkthen

The package and binary are both `thinkthen`. Read `README.md` first, then `specification/README.md`, then `sdlc/planning/rust-standards.md`. The specification is the contract, and code follows it.

## Building

- Red-green test-driven development. Write the failing test, watch it fail for the stated reason, make it pass, then clean up. A red-green test is scaffolding. Before a ticket lands, turn each one into an outside-in behavior test through the command line or public API, a table of edge cases, a contract check, or a regression test that failed before its fix. Delete the rest. Code review checks this.
- Build the simplest thing that works. YAGNI, DRY, locality of behavior, separation of concerns. A command or an option enters only when a demo cannot be written without it.
- The gate ladder is `sdlc/scripts/{install,lint,test,spec}`. Run the cheapest rung first and the whole ladder before handing back.
- `sdlc/ratchet.json` holds the source size ceiling. The ceiling equals the measured total. The commit that raises it says what grew, why it earns its lines, and where you looked for duplication to delete first.
- A second agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency. The review names what it checked.
- A ticket that turns a demo green writes the page in the how-to form of ADR 0011. `sdlc/scripts/demos` checks it.
- Commit as soon as a change is whole and push right away. Commit messages are imperative and active.
- Never add agent attribution to a commit or a pull request: no trailer, no co-author line, no "generated with".

## The pure core

`crates/thinkthen/src/core` touches no file, no environment variable, no socket, no clock, and no process. Module attributes and `policy.py` enforce the bans and the inward dependency direction. The command parses at the edge and hands typed values inward. Do not weaken the lint or policy tables. `lint` compares them against the accepted copies and runs planted policy failures.

## The tool judges and never acts

`thinkthen` never runs a command and never treats free text as an instruction. It writes only files the user named, its resolved platform cache, and count-only usage totals beside that cache. The command never creates or edits the read-only configuration file.

## No network in a gate

Tests replay recorded responses. A live call to a paid backend runs only from `sdlc/scripts/live`, by hand, under a token cap, with Ian's authorization.

The live ledger under Git's common directory is the only runtime authority. Do not edit, replace, remove, or copy it by hand. Use `live --status` to audit it. Follow the migration in `sdlc/scripts/README.md` when a ticket explicitly authorizes an authority change.

## Credentials

A key is read from `THINKTHEN_API_KEY`. It is never committed, logged, hashed, echoed in a plan, or written to a recording. It goes only to the address the user named. A recording stores request bodies and responses and never headers.

## Public hygiene

This repository will go public. Never name a private project or a customer. Describe a consumer generically.

## What reviewers keep finding

- A secrecy test covers every command and every failure path, and it reads every `Debug` line too.
- A test that claims "sends nothing" counts the requests on the loopback listener. `--dry-run` proves nothing about a live path.
- A test pins the exact sentence it checks. `contains("3")` passes on any text with a 3 in it.
- A check script strips fenced code blocks before it reads a title or a status line.
- A block that turns `set -e` off pins the exit code it captured.
- A `jq` transform never uses `//` for a three-way rule, because `false` and a missing value read alike under it.
- A number on a page names the record that measured it, and a change in behavior changes its pages in the same commit.
- A script that checks something runs from a rung, or it rots.
- A new test answers four questions: what behavior it protects, what credible regression makes it fail, why no existing test already catches it, and whether it needs an export, flag, or hook that only tests use. A missing answer rejects it. A test that needs a test-only hook moves to the real boundary. The source is the workspace decision `2026-09-24-tests-earn-their-place.md`.
- Review rejects the junk patterns from the OpenClaw `test-audit` skill (https://github.com/openclaw/openclaw/tree/main/.agents/skills/test-audit). These include a test with no assertion, a value compared to itself, an expected value computed by the code under test, a mock that implements the asserted behavior, a copied inventory or export list, and one contract tested at several layers.

## Where things are

`crates/thinkthen` holds the `core`, `engine`, and `cli` modules and the binary. `specification/` is the contract. `spec/` holds executable pages that `mustmatch` runs. `demos/` holds the how-tos, and each green one is also a test held to ADR 0016. `transforms/` holds `jq` files over saved rows. `probes/` holds the live measurements behind a ruling. `sdlc/` is the record: `planning/adr/` for decisions, `tickets/` for authorized work, `records/` for what landed and its review, `issues/` for problems found, and `scripts/` for the gate ladder. `README.md` holds the names: question file, question set, transform, how-to, pipeline.

## Where decisions go

Every decision lands in `sdlc/`: an ADR for an architecture decision, an issue for a problem found, a ticket for work authorized. Tickets are numbered from 0001 in this repository, and a ticket lands through a worktree. A decision Ian cannot find later was not made. Say which ones he can overturn. A proposal that changes before anyone acts on it is written again whole, with the design and its decisions first. An amendment is for an accepted ADR, where the history matters.
