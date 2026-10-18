# Agent instructions for thinkthen

Read `README.md`, `specification/README.md` and `sdlc/planning/rust-standards.md`. Follow the specification and `sdlc/planning/milestones.md`. Read status, order and lanes in pm. Load role skills from `repos/agents/skills/`.

## Build and review

- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`, offline. Run focused checks per change, `spec` and affected surfaces as needed, and `lint` and `test` once at closure. Large-input cases, full parity and load are release-only: `sdlc/decisions/2026-10-09-two-test-suites.md`.
- Run `test-full-cases --checkpoint MAIN_SHA LOG` through pm job at each main checkpoint. [Release process](sdlc/planning/release-process.md#2-checkpoints) defines targets and placement. Read the one log after the job exits.
- Write tests as `repos/agents/skills/outside-in-tests.md` says.
- Before Rust code review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` for file caps and the adapter-word boundary. `sdlc/ratchet.json` holds the measured source ceiling. Explain warnings and growth in the commit; avoid mechanical splits. A ceiling raise needs explicit reviewer acceptance before landing; deleting source lowers it.
- Run Git-history checks after committing; finish them before the next commit.
- Run builds, tests, lint and release qualification on M5 through `pm job run --machine m5 NNNN SHA --disk GIB --memory GIB -- COMMAND`. The Beelink runs no builds, tests or models. Linux CI runs on each push; Yellow provides occasional Linux checks. PM jobs provide all build admission.
- Cap each lane at 40 GB, `libraries/` and `databases/` included. At the cap or when a build needs room, delete its regenerable output and caches, never other lanes' active work. Keep 50 GB free and branch-specific source copies.
- Delete dead code, scaffolding tests and old language APIs once replacements pass routine installed checks. Close migrations after those checks, API deletion, one `lint` and `test` run and fresh code review. Qualify platforms at the candidate: `sdlc/decisions/2026-10-10-drive-0-2-to-done.md`.
- Keep no backward compatibility before 1.0. Delete old code when its replacement lands: `sdlc/decisions/2026-10-12-no-compatibility-before-1-0.md`.

## Boundaries

Add commands and options only for demos. Replay documentation examples when their text or output changes. Add no verification-runner features, frozen fingerprints, forged-receipt controls or receipt reviews.

Each engine resolves one endpoint, one effective key and one provider API type. The proxy owns business routing, model groups, fallback providers, A/B policy, curation and automatic threshold tuning. Callers control explicit model/reading choices and offline analysis. Add no SDK business policy.

`crates/thinkthen/src/core` touches no file, environment, socket, clock or process. Parse at the command edge; pass typed values inward. Attributes and `policy.py` enforce dependencies.

`thinkthen` judges without running commands or free-text instructions. It writes only user-named files, its platform cache and count-only platform-state usage totals. Never create or edit read-only configuration.

Tests replay saved responses. A paid call runs only through `sdlc/scripts/live`, by hand, under a token cap and Ian's authorization. Git's common-directory live ledger is the sole runtime authority; never edit, replace, remove or copy it by hand. Audit with `live --status`; only a ticket permits migration under `sdlc/scripts/README.md`.

Read the key from `THINKTHEN_API_KEY`, or from a named backend's own key variable (ADR 0114). Never commit, log, hash, echo, record, or put it in a plan; send it only to the user-named address. Recordings store request and response bodies, never headers. This repo will be public: name no private project or customer.

## Proof

- Test secrecy on every command, failure path, and `Debug` line. Prove "sends nothing" by counting loopback requests, not `--plan`. Pin exact sentences, row counts, and exit codes.
- `jq //` treats false as missing. When disabling `set -e`, pin the captured exit code.
- Retain distinct parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions until a stronger replacement lands.

## Where decisions live

`CONTRIBUTING.md` defines terms; `sdlc/README.md` maps the repo. The queue owner owns it, including `site/`: `sdlc/planning/ownership.md`. Rulings live in `sdlc/decisions/`; accepted ADRs and specifications own contracts. Keep source links and name what Ian can overturn.

Release process: `sdlc/planning/release-process.md`. Ian's 2026-10-10 ruling permits coordinator candidate tags, nonpublishing workflows and GitHub tests for platforms unavailable locally. Publishing needs his permission.

## Outside agents

Open a GitHub issue with a repository label and fork link, not a pull request. Use the discussion board for questions when enabled. See `CONTRIBUTING.md`.
