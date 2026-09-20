# sdlc/scripts/

The repository gate and its hand-run support scripts.

| Script | Contract |
| --- | --- |
| `install` | Rung 0. Checks the gate tools, fetches the locked dependency closure, and fetches the advisory database |
| `lint` | Rung 1. Runs policy, page, size, dependency, format, Clippy, and documentation checks |
| `test` | Rung 2. Runs Rust tests and documentation tests, then `sdlc/live-test` on Linux |
| `spec` | Rung 3. Runs executable specification pages, transforms, and green how-tos |
| `demos`, `demos-self-test` | Run green how-tos and prove the runner's refusals |
| `pages`, `pages-self-test` | Keep the how-to lists, titles, states, and links aligned |
| `live` | The hand-run paid-call door. It initializes, reads, locks, validates, and appends the shared ledger, then replaces itself with one charged job |
| `policy.py` | Holds accepted Rust policy tables for rung 1 |
| `ratchet.mjs` | Enforces the Rust source ceiling in `sdlc/ratchet.json` |

Build without the credential, then run a charged job:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL cargo build --locked --package thinkthen
sdlc/scripts/live --max-tokens N JOB [ARG...]
```

`N` is a positive canonical decimal no greater than 999,999,999. A relative job is resolved from the checkout. The prebuilt checkout binary leads `PATH`, the job starts in the checkout, and the job receives the caller's environment and `THINKTHEN_API_KEY`. The wrapper adds one durable `charge N` row before it starts the job. A failed or killed job keeps the charge. The wrapper owns no completion line or recovery state.

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
