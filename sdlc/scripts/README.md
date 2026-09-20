# sdlc/scripts/

How thinkthen installs itself and judges its own work. Edit these freely. Nothing above reads them except by name and exit code.

| Script | Contract |
| --- | --- |
| `install` | Rung 0. Checks the gate tools and, on Linux, the live supervisor's exact Python, Git, procfs, machine identity, socket-pair, and `waitid` prerequisites. It then fetches the locked dependency closure and advisory database |
| `lint` | Rung 1. The policy checker, the how-to list check, the ratchet, `cargo deny` over `deny.toml`, `cargo fmt --check`, clippy with warnings denied, and `cargo doc` with warnings denied. Exit 0 when the code is clean |
| `test` | Rung 2. `cargo test` across every target, every feature, and the documentation examples. It needs `mustmatch` on PATH, because the demo runner's own tests run the real runner. Exit 0 when the tests pass |
| `spec` | Rung 3. Builds the binary, runs the Markdown pages in `spec/` and `transforms/README.md` through `mustmatch`, then calls `demos-self-test` and `demos`. Exit 0 when the tool behaves as the pages say |
| `demos` | Called by `spec`. Runs every `demos/NN-name/README.md` whose status line reads `Status: green`, and skips every red one. A green page that names a `--replay` folder it does not hold stops the run, and so does one that breaks a rule of ADR 0016. It takes another root as its one argument, which is how its own tests give it fixture pages |
| `demos-self-test` | Called by `spec` before `demos`. Builds sixteen fixture pages under `target/`, covering every rule of ADR 0016 a script can measure plus a page that breaks none, runs the real `demos` over each, and pins the exit code and the whole sentence. Exit 0 when every check still catches what it was written for |
| `live` | The one door for a paid live call, run by hand and never by a rung. `live --max-tokens N JOB [ARG...]` requires a prebuilt `target/debug/thinkthen`, checks every registered worktree, and precharges one authority under Git's shared common directory. A separately execed `/usr/bin/python3 -I -S` gate receives the key through an anonymous socket after the charge is durable, acknowledges it, and waits for a separate release. `live --status` reports safe totals. `live --recover` clears a pending run only after its exact wrapper and entire recorded session are absent. Neither command creates authority |
| `live-state.py` | Private state, durability, and process-identity support loaded by `live`. Registered worktrees must carry its guarded marker with the launcher |
| `live-migrate` | Coordinator-only retirement and one-time activation. `--verify` checks every registered tree and refuses active historical wrappers or jobs. `--retire LANDED_SHA` detaches each clean legacy linked tree at the guarded landed revision without moving its branch. `--activate AUDITED_LIMIT AUDITED_CHARGED` creates the machine-bound shared authority once. It never audits totals or chooses them |
| `pages` | Called by `lint`. Holds `demos/README.md`, `sdlc/planning/documentation-plan.md`, the README's front window, and the folders under `demos/` to one list: the same numbers, the same titles, the same state cells word for word, a green page whose own title and status line agree with the list, a leaving page that says on its own first lines which page absorbs it, a front window in ADR 0018's order with a link for a green page and the word coming for a red one, and no relative link that goes nowhere. It refuses a list it could read no table from, so a renamed column cannot silence it. It takes another root as its one argument, which is how its own tests give it small broken trees |
| `pages-self-test` | Called by `lint` before `pages`. Builds sixteen trees under `target/`, each breaking one check, runs the real `pages` over each, and pins the exit code and the whole sentence. Exit 0 when every check still catches what it was written for |
| `policy.py` | The accepted tables, read by `lint`. It checks the toolchain pin, the workspace lint table, both `clippy.toml` copies, lint inheritance, the crate-root attributes, the 500-line file ceiling, and the dependency source, license, and direct sets |
| `ratchet.mjs` | The size ceiling, read from `sdlc/ratchet.json`. Copied rather than written, so lint and the sealed gate apply the same rule |

Cheapest rung first. The whole ladder runs before any hand-back. No rung reaches the network beyond cargo fetching the crates that `Cargo.lock` already names.

Build before a live job, with the credential removed from the build process:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL cargo build --locked --package thinkthen
sdlc/scripts/live --max-tokens N JOB [ARG...]
```

The supervisor does not build. It does not scan recordings or other JSON files after a job. The durable charge is the declared maximum. A successful job keeps its own status even when its observed use exceeds that declaration. Job records may report observed use separately.

## Migrating and recovering live authority

Migration runs only after ticket 0034 lands and while no paid call runs. Keep `THINKTHEN_API_KEY` unset. Record each worktree's path, HEAD, and branch, then verify that every tree is reachable, clean, unlocked, and idle:

```sh
/usr/bin/git worktree list --porcelain -z
/usr/bin/git -C TREE rev-parse HEAD
/usr/bin/git -C TREE symbolic-ref -q HEAD
/usr/bin/git -C TREE status --porcelain=v1 -z
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live-migrate --verify
```

Audit every old ledger and every charged run since the last checkpoint. The known baseline is a 476,000,000 allowance and 429,118 charged tokens. Use it only when all evidence agrees. Divergent ledgers require reconciliation from run records. A maximum can omit independent reservations. Missing or unexplained spend stops migration.

After ticket 0034 is on the target revision, retire clean legacy linked trees and verify all registered launchers again. This leaves their branch references unchanged and preserves every directory:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live-migrate --retire LANDED_0034_SHA
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live-migrate --verify
```

Create authority once from the audited totals. This command prints the new authority ID and totals. Record that output in the ticket record and copy the ID and charged total into `sdlc/live-tokens`. Run status and compare all fields before any paid work resumes:

```sh
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live-migrate --activate 476000000 429118
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/live --status
```

Any dirty, locked, prunable, unreachable, active, or newly registered historical tree stops these commands. Never remove or recreate `.git/thinkthen-live`. Activation creates a fence before publishing active state, and runtime refuses while that fence remains. After an interrupted run, call `sdlc/scripts/live --status`, inspect the named processes without signaling them, then call `sdlc/scripts/live --recover`. Recovery sends no signal, retains the full charge, and refuses while the exact wrapper or any member of the recorded session may remain. A missing or damaged state file disables live work. Restore authority only through the later transfer procedure in ADR 0022.

The ceiling must equal the measured total, so slack cannot accumulate and a raise is always a deliberate edit. Raising it takes two things in that commit's message: what grew and why it earns its lines, and the confirmation that you looked for duplication to remove first and name what you checked.
