# Agent instructions for thinkthen

Read `README.md`, `specification/README.md`, then `sdlc/planning/rust-standards.md`. The specification is the contract. Current scope lives in `sdlc/planning/milestones.md`; current order and lane assignments live in `sdlc/planning/team-0-2-2026-10-04.md`.

## Build and review

Use one fresh review of the whole ticket or slice, then fix and land. A second review is allowed only when a substantial fix touches data loss, credentials, money, memory safety or user-visible correctness. A new dependency also takes the second reviewer required by the Rust standards. Run full tests and lint on the landing commit. Replay docs examples only when docs or their outputs change. Add no verification-runner features, frozen fingerprints, forged-receipt controls or receipt reviews. Write one short record per ticket at landing; name tests by behavior. Keep release rehearsal and Ian's approvals.

- Build simply: YAGNI, DRY, local behavior, separate concerns. Add commands and options only for demos. Land outside-in CLI/API, edge-table, contract, or prior-failing regression tests; delete scaffolding. See workspace decision `2026-09-24-tests-earn-their-place.md`.
- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`. Run focused checks per change. Run full tests and lint on the landing commit; run `spec` and affected surface checks when the change needs them. Load and timing run only through `test-stress --run`. Gates use no network.
- Hand-written Rust and binding source/test files warn at 500–999 nonblank lines and fail at 1,000. Explain a warning in the change’s commit; avoid mechanical splits. Generated source, vendored dependencies and build output remain excluded. `sdlc/ratchet.json` equals the measured source total; explain growth. These checks require no additional reviewer.
- Before Rust code review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`. Compilation and Clippy miss file caps and the adapter-word boundary.
- Keep checks that protect behavior, secrecy, spend, boundaries, or ticket evidence. A check that only polices prose may go; the commit says why.
- Linux and M5 builds may overlap when load, memory, and I/O permit. Reduce jobs under pressure; isolate outputs and lane locks. Keep the shared toolchain and cache mutation locks when builds overlap.
- Tickets follow `sdlc/tickets/README.md`. Commit and push whole changes without agent attribution. Use a claimed lane per `sdlc/planning/worktrees.md`; the lander frees it.

## Boundaries

Each engine resolves one endpoint, one effective key and one provider API type. Business routing, model groups, fallback providers, A/B policy, curation and automatic threshold tuning belong to the proxy. Keep explicit direct model/reading choices and offline analysis as caller controls; add no SDK business policy.

`crates/thinkthen/src/core` touches no file, environment, socket, clock, or process. The command parses at the edge and passes typed values inward; attributes and `policy.py` enforce inward dependencies.

`thinkthen` judges and never acts: it runs no commands or free-text instructions. It writes only user-named files, its platform cache, and count-only usage totals in its platform state folder; it never creates or edits read-only configuration.

Tests replay saved responses. A paid call runs only through `sdlc/scripts/live`, by hand, under a token cap and Ian's authorization. Git's common-directory live ledger is the sole runtime authority: never manually edit, replace, remove, or copy it. Audit with `live --status`; only a ticket permits migration under `sdlc/scripts/README.md`.

Read the key from `THINKTHEN_API_KEY`, or from a named backend's own key variable (ADR 0114). Never commit, log, hash, echo, record, or put it in a plan; send it only to the user-named address. Recordings store request and response bodies, never headers. This repo will be public: name no private project or customer.

## Proof

- Test secrecy on every command, failure path, and `Debug` line. Prove "sends nothing" by counting loopback requests, not `--plan`. Pin exact sentences, row counts, and exit codes.
- `jq //` treats false as missing. When disabling `set -e`, pin the captured exit code.
- Retain distinct parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions until a stronger replacement lands.

## Where decisions live

`CONTRIBUTING.md` defines terms; `sdlc/README.md` maps the repo. The queue owner owns the whole repo, `site/` included, per `sdlc/planning/ownership.md`. A product manager agent sends tickets and edits nothing here. Decisions live in `sdlc/`: ADRs, issues, tickets. An unfindable decision was not made. Name what Ian can overturn.

The release process lives in `sdlc/planning/release-process.md`.
