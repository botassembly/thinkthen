# sdlc/scripts/

How thinkthen installs itself and judges its own work. Edit these freely. Nothing above reads them except by name and exit code.

| Script | Contract |
| --- | --- |
| `install` | Rung 0. Fetches the dependency closure recorded in `Cargo.lock`. Exit 0 when every crate is on disk |
| `lint` | Rung 1. The policy checker, the ratchet, `cargo fmt --check`, clippy with warnings denied, and `cargo doc` with warnings denied. Exit 0 when the code is clean |
| `test` | Rung 2. `cargo test` across every target, every feature, and the documentation examples. Exit 0 when the tests pass |
| `spec` | Rung 3. Builds the binary, runs the Markdown pages in `spec/` through `mustmatch`, then calls `demos`. Exit 0 when the tool behaves as the pages say |
| `demos` | Called by `spec`. Runs every `demos/NN-name/README.md` whose status line reads `Status: green`, and skips every red one. A green page that names a `--replay` folder it does not hold stops the run. It takes another root as its one argument, which is how its own tests give it fixture pages |
| `policy.py` | The accepted tables, read by `lint`. It checks the toolchain pin, the workspace lint table, both `clippy.toml` copies, lint inheritance, the crate-root attributes, the 500-line file ceiling, and the dependency source, license, and direct sets |
| `ratchet.mjs` | The size ceiling, read from `sdlc/ratchet.json`. Copied rather than written, so lint and the sealed gate apply the same rule |

Cheapest rung first. The whole ladder runs before any hand-back. No rung reaches the network beyond cargo fetching the crates that `Cargo.lock` already names.

The ceiling must equal the measured total, so slack cannot accumulate and a raise is always a deliberate edit. Raising it takes two things in that commit's message: what grew and why it earns its lines, and the confirmation that you looked for duplication to remove first and name what you checked.
