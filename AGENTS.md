# Agent instructions for thinkthen

Read `README.md`, `specification/README.md`, and `sdlc/planning/rust-standards.md` in order. The specification is the contract.

## Building

- Red-green test-driven development: watch the new test fail for the stated reason, make it pass, clean up. A red-green test is scaffolding. Before a ticket lands, turn it into an outside-in CLI or API test, an edge-case table, a contract check, or a regression test that failed before its fix, or delete it. Review checks this.
- Build the simplest thing that works: YAGNI, DRY, locality of behavior, separation of concerns. A command or option enters only when a demo needs it.
- The gate ladder is `sdlc/scripts/{install,lint,test,spec,surfaces}`. Run the cheapest rung first and all five before handing back.
- `sdlc/ratchet.json` holds the source size ceiling, equal to the measured total. A commit that raises it says what grew, why it earns its lines, and where you looked for duplication to delete first.
- A second agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency, and names what it checked.
- A ticket that turns a demo green writes its page in ADR 0011's how-to form, and `sdlc/scripts/demos` checks it.
- Commit each whole change and push it at once. Commit messages are imperative and active.
- Never add agent attribution to a commit or pull request: no trailer, co-author line, or "generated with".

## The pure core

`crates/thinkthen/src/core` touches no file, environment variable, socket, clock, or process. Module attributes and `policy.py` enforce the bans and inward dependencies. The command parses at the edge and passes typed values inward. Do not weaken the lint or policy tables. `lint` checks them against accepted copies and planted failures.

## The tool judges and never acts

`thinkthen` never runs a command or treats free text as an instruction. It writes only files the user named, its resolved platform cache, and count-only usage totals beside it. It never creates or edits the read-only configuration file.

## No network in a gate

Tests replay recorded responses. A paid live call runs only from `sdlc/scripts/live`, by hand, under a token cap, with Ian's authorization.

The live ledger under Git's common directory is the only runtime authority. Never edit, replace, remove, or copy it by hand. Audit it with `live --status`. Only a ticket authorizes an authority change, through the migration in `sdlc/scripts/README.md`.

## Credentials

The key comes from `THINKTHEN_API_KEY`. It is never committed, logged, hashed, echoed in a plan, or recorded. It goes only to the address the user named. A recording stores request bodies and responses, never headers.

## Public hygiene

This repository will go public. Never name a private project or customer. Describe a consumer generically.

## What reviewers keep finding

- A secrecy test covers every command, failure path, and `Debug` line.
- A "sends nothing" test counts loopback listener requests. `--dry-run` proves nothing about a live path.
- A test pins the exact sentence it checks. `contains("3")` passes on any text with a 3.
- A check script strips fenced code before it reads a title or status line.
- A block that turns `set -e` off pins the exit code it captured.
- A `jq` transform never uses `//` for a three-way rule, because `false` and a missing value read alike.
- A number on a page names the record that measured it. A behavior change updates its pages in the same commit.
- A script that checks something runs from a rung, or it rots.
- A new test answers four questions. What behavior does it protect? What credible regression fails it? Why does no existing test catch it? Does it need a test-only export, flag, or hook? A missing answer rejects it. A test needing a test-only hook moves to the real boundary. Review rejects the OpenClaw `test-audit` junk: no assertion, self-comparison, expectations the code under test computed, mocks doing the asserted work, copied inventories or export lists, and one contract tested at several layers. See workspace decision `2026-09-24-tests-earn-their-place.md`.

## Where things are

`crates/thinkthen` holds `core`, `engine`, `cli`, and the binary. `specification/` is the contract. `spec/` holds executable pages that `mustmatch` runs. `demos/` holds how-tos, and each green one is a test under ADR 0016. `transforms/` holds `jq` files over saved rows. `probes/` holds live measurements behind rulings. `sdlc/` is the record, and `sdlc/README.md` maps it. `README.md` defines the product's terms.

## Where decisions go

Every decision lands in `sdlc/`: an ADR for architecture, an issue for a problem found, a ticket for authorized work. Tickets number from 0001 here and land through a worktree. A decision Ian cannot find was not made. Say which ones he can overturn. Rewrite a proposal whole, design and decisions first, if it changes before anyone acts. Amend only an accepted ADR, where the history matters.
