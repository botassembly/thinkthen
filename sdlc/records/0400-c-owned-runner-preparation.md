# 0400 C owned runner preparation

Status: focused integration preparation; full campaign not authorized or run. The concrete reviewed launch scripts and receipts are retained under the lane's ignored `target/0400c-integration-runner/`, with the owned runtime directory named in `runner-root.txt`. No product runner or architecture change is introduced.

## Environment and bounded exception

Every launch uses `/bin/bash --noprofile --norc` and `env -i`. Private HOME, Cargo home, scratch/state directories, UTF-8 locale, cached Node 22, Rustup/toolchain paths and the lane lock are explicit. Only cached toolchains, uv managed Python/cache, npm/pub caches and Flutter are linked selectively into the owned HOME. Cargo registry/git/bin caches are linked individually; inherited Cargo configuration/credentials and shell, R and uv startup configuration are excluded. All Cargo calls restore offline mode, two jobs and empty compiler wrappers. The uv wrapper uses `--no-config --offline`; R/Rscript disable all four profile/environment files. Existing R jsonlite 2.0 and system Suggests packages are admitted through explicit library order. No download runs.

Private `.pgrx/config.toml` contains only `[configs]` with `pg16` set to the installed PostgreSQL 16 `pg_config`. Cargo explicitly restores PGRX_HOME and the installed PG configuration path after nested allow-list stripping. No caller PGRX configuration is read or linked. Actual cached server/header pins remain the existing PostgreSQL gate's authority. These prerequisites prove configuration discovery, not a new extension build or server campaign.

The existing local-file refusal plant is reused. The dispatcher admits only `fetch --quiet --manifest-path ABSOLUTE_PATH` beneath this run's private scratch directory, with matching scratch Cargo home, caller ownership and no symlinks. Package manifests must exactly match the existing dependency-free `binding`/`planted` plant: one file-URL dependency, no authority/query/fragment/escapes or alternate destination, no other dependency/build/workspace fields. Empty sources and the exact committed two-file tree are checked. That invocation alone executes real Cargo with offline disabled; ordinary fetch remains offline and reaches real Cargo unchanged. Global/system Git configuration is disabled. External URLs are refused before Cargo executes. This is a bounded local-file exception, not whole-host network isolation.

Owned fixture proof uses real fetch to create lock/checkouts, real cargo-deny exit 8 with `source-not-allowed`, and a counted loopback HTTP URL refusal with exit 1 and zero requests. It also exercises an ordinary locked/offline fetch, cached Node/Python/R prerequisites and actual PGRX configuration/version discovery. Distinct original failed runner setup receipts remain retained. No direct SQLite stress or narrow campaign selector is used.

## Prepared future root run

After fresh source review and a coordinator-named checkpoint, the prepared entry point is `launch.sh stress`, guarded by a required `THINKTHEN_0400C_CHECKPOINT` label. It runs the existing complete `sdlc/scripts/test-stress --run` once. The parent owns a 12 GiB memory / 1 GiB swap user scope. The launch holds the lane-specific heavy lock, shared toolchain and cache mutation locks, and shared Cargo cache lock. Each command preserves its exact exit using an explicit success/failure branch under errexit, records the frozen HEAD before execution and checks it afterward. No overwritten old campaign receipt is used.

The launch's label is a guard, not approval. Root must separately authorize and name the checkpoint. The coordinator must inspect every 77 result: intentionally unsupported stress campaigns remain not run, while missing prerequisite caches remain missing prerequisites. A later green campaign still cannot claim all 21 stress surfaces passed when some intentionally provide no stress gate.

Full test/specification and actual canonical 350 replay/strict/site need their own coordinator checkpoint and receipts. Main's binding proof remains intact until actual canonical re-execution. No manually rebound proof hashes, network namespace, external fetch, paid call, native run, release or publication occurs here.

## What the build taught us

Keep local Cargo refusal plants outside the repository workspace, and derive dispatcher locations from their own script directory so copied launch bundles remain correct. Preserve launch mistakes with their actual exits. Cached prerequisites and passing bounded fixtures do not prove a full stress result. Independent review and a new named checkpoint remain required; Ian can overturn the runner environment choices.
