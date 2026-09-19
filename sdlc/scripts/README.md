# sdlc/scripts/

How thinkthen installs itself and judges its own work. Edit these freely. Nothing above reads them except by name and exit code.

| Script | Contract |
| --- | --- |
| `install` | Rung 0. Checks that `cargo`, `python3`, `node`, `mustmatch`, `jq`, `script`, and `cargo-deny` are there, then fetches the dependency closure recorded in `Cargo.lock`. Exit 0 when every tool the later rungs need is there and every crate is on disk |
| `lint` | Rung 1. The policy checker, the how-to list check, the ratchet, `cargo deny` over `deny.toml`, `cargo fmt --check`, clippy with warnings denied, and `cargo doc` with warnings denied. Exit 0 when the code is clean |
| `test` | Rung 2. `cargo test` across every target, every feature, and the documentation examples. It needs `mustmatch` on PATH, because the demo runner's own tests run the real runner. Exit 0 when the tests pass |
| `spec` | Rung 3. Builds the binary, runs the Markdown pages in `spec/` and `transforms/README.md` through `mustmatch`, then calls `demos-self-test` and `demos`. Exit 0 when the tool behaves as the pages say |
| `demos` | Called by `spec`. Runs every `demos/NN-name/README.md` whose status line reads `Status: green`, and skips every red one. A green page that names a `--replay` folder it does not hold stops the run, and so does one that breaks a rule of ADR 0016. It takes another root as its one argument, which is how its own tests give it fixture pages |
| `demos-self-test` | Called by `spec` before `demos`. Builds sixteen fixture pages under `target/`, covering every rule of ADR 0016 a script can measure plus a page that breaks none, runs the real `demos` over each, and pins the exit code and the whole sentence. Exit 0 when every check still catches what it was written for |
| `live` | The one door for a paid live call, run by hand and never by a rung. It takes the path of a job script, refuses at the spend limit in `sdlc/live-tokens` and with a blank `THINKTHEN_API_KEY`, builds the binary onto PATH, runs the job, then adds the input tokens of every recording entry the job wrote. `crates/thinkthen/tests/live_script.rs` holds both refusals, and neither needs a network |
| `pages` | Called by `lint`. Holds `demos/README.md`, `sdlc/planning/documentation-plan.md`, the README's front window, and the folders under `demos/` to one list: the same numbers, the same titles, the same state cells word for word, a green page whose own title and status line agree with the list, a leaving page that says on its own first lines which page absorbs it, a front window in ADR 0018's order with a link for a green page and the word coming for a red one, and no relative link that goes nowhere. It refuses a list it could read no table from, so a renamed column cannot silence it. It takes another root as its one argument, which is how its own tests give it small broken trees |
| `pages-self-test` | Called by `lint` before `pages`. Builds sixteen trees under `target/`, each breaking one check, runs the real `pages` over each, and pins the exit code and the whole sentence. Exit 0 when every check still catches what it was written for |
| `policy.py` | The accepted tables, read by `lint`. It checks the toolchain pin, the workspace lint table, both `clippy.toml` copies, lint inheritance, the crate-root attributes, the 500-line file ceiling, and the dependency source, license, and direct sets |
| `ratchet.mjs` | The size ceiling, read from `sdlc/ratchet.json`. Copied rather than written, so lint and the sealed gate apply the same rule |

Cheapest rung first. The whole ladder runs before any hand-back. No rung reaches the network beyond cargo fetching the crates that `Cargo.lock` already names.

The ceiling must equal the measured total, so slack cannot accumulate and a raise is always a deliberate edit. Raising it takes two things in that commit's message: what grew and why it earns its lines, and the confirmation that you looked for duplication to remove first and name what you checked.
