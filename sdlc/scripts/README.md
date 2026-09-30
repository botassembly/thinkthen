# sdlc/scripts/

The repository gate and its hand-run support scripts.

| Script | Contract |
| --- | --- |
| `install` | Rung 0. Checks the gate tools, fetches the locked dependency closure, and fetches the advisory database |
| `lint` | Rung 1. Runs the private-name check when `THINKTHEN_PRIVATE_NAMES` names a list outside the repository, then policy, page, size, agents-file size, dependency, format, Clippy, documentation, and bounded source-package/workflow routing checks. It does not run the full `package` checkpoint |
| `test` | Rung 2. Runs every workspace test (`cargo nextest` when installed, else `cargo test`), the doctests, the external consumer, and the shell self-tests, then `sdlc/live-test` on Linux |
| `test-full-cases --list|--run` | Explicit full-functional checkpoint. Lists its work without running it, or runs all root targets, the external Rust consumer, and all 54 shared cases on every surface. Ignored stress tests stay out |
| `test-stress --list|--run` | Explicit repeated load and timing campaign. Lists its selections without running them, or runs the named ignored Rust campaigns and the port stress profile |
| `spec` | Rung 3. Builds the binary, runs the `settings` check, then executable specification pages, transforms, and green how-tos |
| `surfaces` | Rung 4. Runs each landed surface's `check.sh` with one loopback backend's port and reports exit 77 as not run. Default exports `THINKTHEN_TEST_PROFILE=routine` and the absolute 31-ID selector. `--full-functional` exports `full` and no selector; `--stress` exports `stress` and no selector. `--registry` is the rung 1 check of `sdlc/surfaces.txt`, each binding ratchet, and each binding lock under `cargo deny` |
| `allow-list` | Sourced by `heavy-lock` and `lint` (ticket 0133). Unsets every name but `PATH`, `HOME`, `PWD`, `LANG`, `LC_ALL`, `TMPDIR`, `XDG_RUNTIME_DIR`, `XDG_CACHE_HOME`, the toolchain names `CARGO_HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`, `RUSTC_WRAPPER`, `SCCACHE_CONF`, and `R_LIBS_USER`, the two lock names, the path overrides `THINKTHEN_TOOLCHAINS`, `THINKTHEN_DUCKDB_CLI`, and `SQLITE_AMALGAMATION`, and `THINKTHEN_PRIVATE_NAMES`, the path to the private-name list that `lint` reads. `CARGO_TARGET_DIR` is dropped, so each rung that sources it builds in the lane's own `target` folders |
| `heavy-lock` | Sourced by `install`, `test`, `spec`, and `surfaces`. Where `flock` is available, runs a rung under the lock selected by `THINKTHEN_HEAVY_LOCK`; a nested rung skips reacquiring that same lock. Otherwise it warns and runs without that lock. M5 shared mutations need separate exclusion, such as a Python `fcntl` wrapper. Independent lanes use separate locks and may overlap when host capacity permits. First it sources `allow-list` |
| `demos`, `demos-self-test` | Run green how-tos and prove the runner's refusals |
| `pages`, `pages-self-test` | Keep the how-to lists, titles, states, and links aligned |
| `settings`, run by `spec` | Fails rung 3 when `specification/settings.md` lacks a row for a flag in any command's help, a product `THINKTHEN_` variable, or a question-file key, names one that no longer exists, or moves a column (ticket 0140). `--self-test` plants each fault into a copy of the page first |
| `tickets` | Fails rung 1 when a ticket numbered 0120 or higher lacks its five-part Evidence section. `--self-test` runs its planted cases first |
| `recognize-keys` | Fails rung 1 when a line of `specification/fixtures/recognize/names.jsonl` or `relations.jsonl` has a name whose code-point offsets do not cut it out of its text, overlaps the name before, repeats an id, or names an edge endpoint its line lacks (ticket 0164). `--self-test` plants each fault first. `convert names SOURCE` and `convert relations SOURCE` print the converted keys |
| `rekey-model FROM TO FOLDER...` | Ticket 0159. Moves each Git-tracked, clean recording entry whose request asks for FROM and whose reply names TO to the digest of the same request asking for TO. It checks every digest-named entry before it writes any: a symlink or other non-regular file, an untracked or edited file, a name that does not match its bytes, a reply from another model, or an existing target stops the run with nothing moved. It changes only the bytes of the request's model value, renames with `git mv`, and prints each old digest beside its new one. `--self-test`, run by `lint`, re-keys the two entries in `fixtures/rekey-model/` and plants each refusal. It goes when the site's recordings are re-keyed |
| `live` | The hand-run paid-call door. It initializes, reads, locks, validates, and appends the shared ledger, then replaces itself with one charged job |
| `policy.py` | Holds accepted Rust policy tables for rung 1 |
| `catalog.py` | Holds the shipped transform copies, the catalog table, and the source package byte-identical to `transforms/` for rung 1 |
| `package` | Explicit packaging/release checkpoint, required by the manual release `crate` job before artifact upload. Proves the one-package, no-default-feature targets, tests and doctests, stale-archive cleanup, package tree, release panic modes, and fresh unpacked transform catalog. It is not part of routine `lint` |
| `scratch.sh` | Sourced by each script that makes a scratch folder. `scratch_dir` makes a folder with `mktemp` and records it, and a script removes only a recorded folder (`sdlc/planning/worktrees.md` rule 11). `scratch_lint` fails rung 1 on a recursive `rm` outside this file and its named exceptions. It reads `sdlc/scripts/`, `sdlc/live-test`, `install.sh`, and each `*.sh` or `#!` file under `libraries/`, `databases/`, `transforms/`, `probes/`, and `demos/` |
| `ratchet.mjs` | Enforces the Rust source ceiling in `sdlc/ratchet.json`, or in the config its one optional argument names |

Build without the credential, then run a charged job:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL cargo build --locked --package thinkthen
sdlc/scripts/live --max-tokens N JOB [ARG...]
```

`N` is a positive canonical decimal no greater than 999,999,999. A job is a shell script. Its first line must be exactly `#!/bin/sh`, or the wrapper refuses it before any charge. A relative job is resolved from the checkout. The prebuilt checkout binary leads `PATH`, the job starts in the checkout, and the job receives the caller's environment and `THINKTHEN_API_KEY`. The wrapper adds one durable `charge N` row before it starts the job. A failed or killed job keeps the charge. The wrapper owns no completion line or recovery state.

`sdlc/scripts/live --status` takes the shared lock, validates the durable initialization marker and every ledger row, and prints the limit, charge, and remainder. A missing ledger after initialization, a partial row, an unterminated row, or an unknown row disables live work. If appending or syncing reports failure, the charge may still exist. Run `--status` or inspect the ledger under the lock before any retry.

## One-time migration from version one

Run this only after ticket 0035 lands and while no paid job runs. Keep the key unset. Hold `.git/thinkthen-live/lock` exclusively and verify that `state.json` is active, has no pending run, and exactly matches `sdlc/live-tokens`: 476,000,000 allowed and 429,118 charged. Refuse an existing `initialized`, `ledger`, or `state-v1-retired.json`.

Rename `state.json` to `state-v1-retired.json`, sync `.git/thinkthen-live/`, and release the lock. Then run:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live --init
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live --status
```

Status must report 476,000,000 allowed and 429,118 charged. A failure before rename leaves the old authority. A failure after rename and before initialization disables both launchers. `--init` refuses while `state.json` exists, and every new action refuses if it reappears. Do not edit the authority by hand after migration.

The ceiling in `sdlc/ratchet.json` equals the measured Rust total. A raise records what grew, why it earns its lines, and where duplication was checked first.
