# Agent instructions for thinkthen

Read `README.md`, `specification/README.md`, then `sdlc/planning/rust-standards.md`. The specification is the contract.

## Build and review

- Work red-green: see the failure, pass it, then clean up. Before landing, replace scaffold tests with outside-in CLI/API, edge-table, contract, or prior-failing regressions, or delete them.
- Build simply: YAGNI, DRY, local behavior, separate concerns. Add commands and options only for demos. A green demo gets an ADR 0011 how-to checked by `sdlc/scripts/demos`.
- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`. Run focused format, lint, and functional checks per change. The coordinator names a checkpoint before full `test`, `spec`, or `surfaces`. Run load, churn, timing, and contention only through `test-stress --run`; keep functional cases in `test-full-cases --run`. Gates use no network.
- Rust source and test files cap at 500 nonblank lines; `sdlc/ratchet.json` equals the measured source total. Explain growth and sought duplication. A fresh independent reviewer checks the candidate; a second agent names checks of raised ceilings, public surfaces, and dependencies.
- Before Rust code review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`; reuse it for unrelated text changes. Compilation and Clippy miss file caps and the adapter-word boundary. Never weaken lint or policy tables; `lint` plants failures.
- Linux and M5 builds may overlap when load, memory, and I/O permit. Inspect capacity and reduce jobs under pressure; isolate outputs and rung locks, keeping shared toolchain/cache mutation locks. The work plan records Ian's ruling.
- Prepare tickets per `sdlc/planning/ticket-preparation.md`; preserve outcomes and add `## What the build taught us` before landing. Use the lessons in the next brief. Commit and push whole changes without agent attribution. Use a claimed lane per `sdlc/planning/worktrees.md`; the lander frees it.

## Boundaries

`crates/thinkthen/src/core` touches no file, environment, socket, clock, or process. The command parses at the edge and passes typed values inward; attributes and `policy.py` enforce inward dependencies.

`thinkthen` judges and never acts: it runs no commands or free-text instructions. It writes only user-named files, its platform cache, and adjacent count-only usage totals; it never creates or edits read-only configuration.

Tests replay saved responses. A paid call runs only through `sdlc/scripts/live`, by hand, under a token cap and Ian's authorization. Git's common-directory live ledger is the sole runtime authority: never manually edit, replace, remove, or copy it. Audit with `live --status`; only a ticket permits migration under `sdlc/scripts/README.md`.

Read the key from `THINKTHEN_API_KEY`. Never commit, log, hash, echo, record, or put it in a plan; send it only to the user-named address. Recordings store request and response bodies, never headers. This repo will be public: name no private project or customer.

## Proof and records

- Test secrecy on every command, failure path, and `Debug` line. Prove “sends nothing” by counting loopback requests, not `--plan`. `cache prune --dry-run` is a separate preview. Pin exact sentences, row counts, and exit codes. Checkers strip fenced code and run from a rung.
- Do not use `jq //` for a three-way rule: false differs from missing. When disabling `set -e`, pin the captured exit code. A page's number cites its measurement; update behavior and its pages together.
- For each new test, name its behavior, failing regression, why existing tests miss it, and any test-only hook; move hooks to the real boundary. Reject absent/self-comparing assertions, computed expectations, mocks doing asserted work, copied inventories, and duplicate checks. See `sdlc/planning/ticket-preparation.md` and workspace decision `2026-09-24-tests-earn-their-place.md`.
- Build records name deleted or consolidated tests and stronger replacements. Retain distinct parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions until then. Reconcile source, decisions, and issue criteria before closure.

## Where decisions live

`README.md` defines terms; `sdlc/README.md` maps the repo. Marketing owns `site/` per `sdlc/planning/ownership.md`. Green demos are ADR 0016 tests; `probes/` holds ruled live measurements.

Decisions live in `sdlc/`: ADRs, issues, tickets. An unfindable decision was not made. Name what Ian can overturn. Rewrite changed proposals whole, design first, and amend accepted ADRs when history matters.
