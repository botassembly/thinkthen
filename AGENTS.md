# Agent instructions for thinkthen

Read `README.md`, `specification/README.md` and `sdlc/planning/rust-standards.md`. Follow the specification and `sdlc/planning/milestones.md`. Read status, order and lanes through pm. The workspace instructions load the shared role skills from `repos/agents/skills/`; this file adds repository boundaries and checks.

## Build and review

- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`. Run focused checks per change, plus `spec` and affected surfaces when needed. Load and timing run only through `test-stress --run`. Gates use no network.
- `policy.py` enforces source file limits; `sdlc/ratchet.json` holds the measured source ceiling. Explain warnings and growth in the commit; avoid mechanical splits.
- Before Rust code review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`. Compilation and Clippy miss file caps and the adapter-word boundary.
- Beelink is primary. M5 may run experiments and Mac-specific checks, including before candidates, not after every ticket. Reduce jobs under pressure; isolate lane output and keep toolchain/cache mutation locks.
- Cap each lane at 40 GB total, including `libraries/` and `databases/`. At landing delete only its ticket-owned `target/` scratch/logs; keep warm builds. Fully clean idle lanes only below 50 GB free. Require 50 GB free before full parity. Keep branch-specific source/build copies; sharing mixes branches.

## Boundaries

Add commands and options only for demos. Replay documentation examples when their text or output changes. Add no verification-runner features, frozen fingerprints, forged-receipt controls or receipt reviews.

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

`CONTRIBUTING.md` defines terms; `sdlc/README.md` maps the repo. The queue owner owns the whole repo, `site/` included, per `sdlc/planning/ownership.md`. Lasting rulings live in `sdlc/decisions/`; accepted ADRs and specifications retain product contracts. Preserve source links and name what Ian can overturn.

The release process lives in `sdlc/planning/release-process.md`. Ian held release management on 2026-10-08: candidate tags, manual workflow dispatches, release branch advancement and publication require his permission. See `sdlc/decisions/2026-10-08-preserve-planning-rulings.md`.
