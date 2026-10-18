# Agent instructions for thinkthen

Read `README.md`, `specification/README.md` and `sdlc/planning/rust-standards.md`. Follow the specification and `sdlc/planning/milestones.md`. Get status, order and lanes from pm; role skills from `repos/agents/skills/`.

## Build and review

- Gates: `sdlc/scripts/{install,lint,test,spec,surfaces}`, offline. Per change, run focused checks, `spec` and affected surfaces as needed; run `lint` and `test` once at closure. Large-input cases, full parity and load are release-only: `sdlc/decisions/2026-10-09-two-test-suites.md`.
- Before requesting release approval, run (`test-full-cases`, `package`, `test-stress`) through `pm job run --machine m5` on the pushed candidate. Run each in the background with a log; never watch or poll it. Read logs after exit.
- Write tests as `repos/agents/skills/outside-in-tests.md` says.
- Before Rust review, run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` for the adapter-word boundary and dependency rules. Code review judges size; avoid mechanical splits.
- Run Git-history checks after each commit, before the next.
- Run everyday builds and checks on M5 through `pm job run --machine m5 NNNN SHA --disk GIB --memory GIB -- COMMAND`. Linux checks use the pinned arm64 container on M5; x86_64-specific proof uses the pinned container on Yellow through `pm job run --machine yellow`. `sdlc/scripts/linux-check` builds Linux images and runs checks. Container CPU and memory caps: `sdlc/tools.json` under `linux`; pm jobs book matching memory. Yellow uses `linux.yellow_memory_gib` and runs one heavy job at a time. Hosted workflows run by hand for releases after local checks pass. Only pm jobs admit builds.
- Cap each lane at 40 GB, `libraries/` and `databases/` included. At the cap or when builds need room, delete the lane's regenerable output and caches, never other lanes' active work. Keep 50 GB free and branch-specific source copies.
- After replacements pass routine installed checks, delete dead code, scaffolding tests and old language APIs. Close migrations after those checks, API deletion, one `lint` and `test` run and fresh code review. Qualify platforms at the candidate: `sdlc/decisions/2026-10-10-drive-0-2-to-done.md`.
- Keep no backward compatibility before 1.0. Delete old code when its replacement lands: `sdlc/decisions/2026-10-12-no-compatibility-before-1-0.md`.

## Boundaries

Add commands and options only for demos. Replay docs examples when their text or output changes. Add no verification-runner features, frozen fingerprints, forged-receipt controls or receipt reviews.

Each engine resolves one endpoint, effective key and provider API type. The proxy owns business routing, model groups, fallback providers, A/B policy, curation and automatic threshold tuning. Callers control explicit model/reading choices and offline analysis. Add no SDK business policy.

`crates/thinkthen/src/core` touches no file, environment, socket, clock or process. Parse at the command edge; pass typed values inward. Attributes and `policy.py` enforce dependencies.

`thinkthen` judges without running commands or free-text instructions. It writes only user-named files, its platform cache and count-only platform-state usage totals. Never create or edit read-only configuration.

Tests replay saved responses. Paid calls run only through `sdlc/scripts/live`, by hand, under a token cap and Ian's authorization. Git's common-directory live ledger alone governs runtime; never edit, replace, remove or copy it by hand. Audit with `live --status`; migration needs a ticket under `sdlc/scripts/README.md`.

Read the key from `THINKTHEN_API_KEY` or a named backend's own key variable (ADR 0114). Never commit, log, hash, echo, record or plan it; send it only to the user-named address. Recordings store request and response bodies, never headers. Public repo: name no private project or customer.

## Proof

- Test secrecy on every command, failure path and `Debug` line. Prove "sends nothing" by counting loopback requests, not `--plan`. Pin exact sentences, row counts, and exit codes.
- `jq //` treats false as missing. When disabling `set -e`, pin the captured exit code.
- Retain distinct parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions until a stronger replacement lands.

## Decisions

`CONTRIBUTING.md` defines terms; `sdlc/README.md` maps the repo. The queue owner owns the repo and `site/`: `sdlc/planning/ownership.md`. Rulings live in `sdlc/decisions/`; accepted ADRs and specifications own contracts. Keep source links and name what Ian can overturn.

Release process: `sdlc/planning/release-process.md`. Ian's 2026-10-10 ruling permits coordinator candidate tags, nonpublishing workflows and GitHub tests for platforms unavailable locally. Publishing needs his permission.

## Outside agents

Open GitHub issues with a repository label and fork link, never pull requests. Ask questions on the discussion board when enabled. See `CONTRIBUTING.md`.
