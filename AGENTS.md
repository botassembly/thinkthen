# Agent instructions for thinkthen

Read `README.md`, `specification/README.md`, and `sdlc/planning/rust-standards.md` in order. The specification is the contract.

## Building

- Red-green development: see the stated failure, pass it, then clean up. Before landing, turn scaffold tests into outside-in CLI/API, edge-table, contract, or prior-failing regression tests, or delete them. Review checks this.
- Build simply: YAGNI, DRY, local behavior, separate concerns. A command or option enters only when a demo needs it.
- The gate ladder is `sdlc/scripts/{install,lint,test,spec,surfaces}`. Run focused format, lint and functional checks per change. The coordinator names the related-ticket checkpoint before a full test, spec or surfaces run. Run load, churn, timing and contention only through `test-stress --run`; retain all functional cases in `test-full-cases --run`.
- `sdlc/ratchet.json` equals the measured source total. A raised ceiling records growth, why it earns its lines, and where duplication was sought for deletion.
- A second agent reviews raised ceilings, wider public surfaces, and dependencies, naming what it checked.
- Before handing Rust changes to review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`. Compilation and Clippy do not check the file caps or adapter-word boundary. Reuse this result when only unrelated text changes.
- Independent Linux and M5 builds may overlap when load, memory and I/O have room. Use isolated worktree outputs and lane-specific rung locks; retain real shared toolchain/cache mutation locks. Inspect capacity before substantial work and reduce jobs or defer only under pressure. The work plan records Ian’s capacity ruling.
- A ticket that turns a demo green writes its page in ADR 0011's how-to form, and `sdlc/scripts/demos` checks it.
- Prepare related tickets per `sdlc/planning/ticket-preparation.md`; preserve outcomes. Before landing, add `## What the build taught us` to the ticket. Review its lessons and improve the next brief.
- Commit each whole change and push it at once.
- Never add agent attribution to a commit or pull request.
- Use a lane per `sdlc/planning/worktrees.md`. The lander frees it.

## The pure core

`crates/thinkthen/src/core` touches no file, environment, socket, clock, or process. Attributes and `policy.py` enforce bans and inward dependencies. The command parses at the edge and passes typed values inward. Never weaken lint or policy tables; `lint` checks accepted copies and planted failures.

## The tool judges and never acts

`thinkthen` never runs commands or obeys free text. It writes only user-named files, its platform cache, and adjacent count-only usage totals. It never creates or edits read-only configuration.

## No network in a gate

Tests replay recorded responses. A paid live call runs only from `sdlc/scripts/live`, by hand, under a token cap, with Ian's authorization.

The live ledger under Git's common directory is the sole runtime authority. Never manually edit, replace, remove, or copy it. Audit with `live --status`; only a ticket permits migration under `sdlc/scripts/README.md`.

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
- A new test answers four questions: protected behavior, credible failing regression, why existing tests miss it, and whether it needs a test-only export, flag, or hook. Missing answers reject it; test-only hooks move to the real boundary. Review rejects OpenClaw `test-audit` junk: absent or self-comparing assertions, code-computed expectations, mocks doing asserted work, copied inventories/exports, and the same contract at several layers. See workspace decision `2026-09-24-tests-earn-their-place.md`.
- The build record names each deleted or consolidated test and the stronger routine proof that replaces it. A distinct parser, secrecy, cancellation, cache-miss, invalid-input, or conflict regression stays until a stronger boundary test proves the same behavior.

## Where things are

`README.md` defines terms; `sdlc/README.md` maps the repo. Marketing owns `site/` per `sdlc/planning/ownership.md`. Green demos are ADR 0016 tests. `probes/` holds ruled live measurements.

## Where decisions go

Decisions land in `sdlc/`: ADRs for architecture, issues for problems, tickets for authorized work. Tickets number from 0001 and land through worktrees. An unfindable decision was not made; name what Ian can overturn. Before action, rewrite changed proposals whole, design first. Amend accepted ADRs when history matters.
