# Agent instructions for thinkthen

Read `README.md`, `specification/README.md` and `sdlc/planning/rust-standards.md`. Follow the specification and `sdlc/planning/milestones.md`. Read status, order and lanes through pm. Load workspace role skills from `repos/agents/skills/`.

## Build and review

- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`, offline. Run focused checks per change, `spec` and affected surfaces as needed, and `lint` and `test` once at closure. Large-input cases, full parity and load are release-only: `sdlc/decisions/2026-10-09-two-test-suites.md`.
- Tests are outside-in through the Rust API, the CLI and each installed library. Red, green, remove: delete scaffolding unit tests before landing.
- `policy.py` enforces source file limits; `sdlc/ratchet.json` holds the measured source ceiling. Explain warnings and growth in the commit; avoid mechanical splits.
- Require explicit reviewer acceptance for a source-ceiling increase before landing, and lower the ceiling when deleting source.
- Keep the handwritten complete readers in Python, Ruby, R and JavaScript unchanged except for confirmed regressions until their generated migrations replace them.
- Before Rust code review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` for file caps and the adapter-word boundary.
- Run checks that read Git history after committing the changes, and finish them before committing again.
- Beelink is primary. M5 may run experiments and Mac-specific checks, including before candidates, not after every ticket. Reduce jobs under pressure; isolate lane output and keep toolchain/cache mutation locks.
- Cap each lane at 40 GB, including `libraries/` and `databases/`. Delete its regenerable output and caches at the cap or when a build needs room. Preserve other lanes' active work. Keep 50 GB free and branch-specific source copies.
- Delete dead code, scaffolding tests and old language APIs once replacements pass routine installed checks. Close migrations after those checks, API deletion, one `lint` and `test` run and fresh code review. Qualify platforms at the candidate: `sdlc/decisions/2026-10-10-drive-0-2-to-done.md`.

## Boundaries

Add commands and options only for demos. Replay documentation examples when their text or output changes. Add no verification-runner features, frozen fingerprints, forged-receipt controls or receipt reviews.

Each engine resolves one endpoint, one effective key and one provider API type. Business routing, model groups, fallback providers, A/B policy, curation and automatic threshold tuning belong to the proxy. Keep explicit direct model/reading choices and offline analysis as caller controls; add no SDK business policy.

`crates/thinkthen/src/core` touches no file, environment, socket, clock or process. Parse at the command edge; pass typed values inward. Attributes and `policy.py` enforce dependencies.

`thinkthen` judges without running commands or free-text instructions. It writes only user-named files, its platform cache and count-only platform-state usage totals. Never create or edit read-only configuration.

Tests replay saved responses. A paid call runs only through `sdlc/scripts/live`, by hand, under a token cap and Ian's authorization. Git's common-directory live ledger is the sole runtime authority: never manually edit, replace, remove, or copy it. Audit with `live --status`; only a ticket permits migration under `sdlc/scripts/README.md`.

Read the key from `THINKTHEN_API_KEY`, or from a named backend's own key variable (ADR 0114). Never commit, log, hash, echo, record, or put it in a plan; send it only to the user-named address. Recordings store request and response bodies, never headers. This repo will be public: name no private project or customer.

## Proof

- Test secrecy on every command, failure path, and `Debug` line. Prove "sends nothing" by counting loopback requests, not `--plan`. Pin exact sentences, row counts, and exit codes.
- `jq //` treats false as missing. When disabling `set -e`, pin the captured exit code.
- Retain distinct parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions until a stronger replacement lands.

## Where decisions live

`CONTRIBUTING.md` defines terms; `sdlc/README.md` maps the repo. The queue owner owns the whole repo, `site/` included, per `sdlc/planning/ownership.md`. Lasting rulings live in `sdlc/decisions/`; accepted ADRs and specifications retain product contracts. Preserve source links and name what Ian can overturn.

Release process: `sdlc/planning/release-process.md`. Ian's 2026-10-10 ruling permits coordinator candidate tags, nonpublishing workflows and GitHub tests for platforms unavailable locally. Publishing needs his permission: `sdlc/decisions/2026-10-10-drive-0-2-to-done.md`, superseding the hold in `sdlc/decisions/2026-10-08-preserve-planning-rulings.md`.

## Outside agents

Open a GitHub issue with a repository label and fork link, not a pull request. Use the discussion board for questions when enabled. See [CONTRIBUTING.md](CONTRIBUTING.md#external-contributions).
