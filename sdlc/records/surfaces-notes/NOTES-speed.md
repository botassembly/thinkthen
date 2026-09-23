# NOTES: lint and gate speed (wave 7)

Measured 2026-09-23 on the Linux box at `f6b8814`, four cargo jobs, sccache on and warm from other lanes. The full table and logs lived in the lane's scratch notes. The numbers below are the ones each decision rests on.

## What the numbers show

- A full `sdlc/scripts/lint` takes 317 s in a fresh worktree and 13 s on a second run. The 20 to 30 minutes lanes saw came from four lanes running the rung at once on one box, not from the rung alone.
- The largest cold step is `package` at 85 s: seven builds of the dependency graph in different shapes. Next are the PostgreSQL workspace (47 s), the Rust surface on its own 1.95 toolchain (38 s), and DuckDB (26 s).
- A one-line edit in one library costs 13 s of full lint on a warm worktree. An edit in the contract costs 23 to 30 s, because every surface rebuilds it.
- A warm `scripts/check_surfaces.sh` takes 286 to 400 s. Test run time dominates: the stand-in's loopback tests, Ruby, R, TypeScript, and the PostgreSQL containers.

## Decisions (each can be overturned)

1. **Add `lint --changed BASE`** (commits `649c1bb`, `b852c15`). It runs the cheap checks in full: policy, pages, both ratchets, format, and the workspace self-test. It runs the package proofs, cargo deny, Clippy, and docs only where a path changed since the merge base with BASE can change their answer. `lint-workspaces --select` holds the rule, and its self-test pins it. A path in one workspace selects that workspace. The contract, core, and stand-in crates, the conformance and specification files compiled in, any lock file, the toolchain, the lint and format rules, and the rung's own scripts select everything. A one-library edit drops from 13 s to 4 to 5 s, and a docs-only change from 13 s to 4 s. The default with no argument is the full rung, unchanged. The full rung still runs before a hand-back.
2. **Reject a shared build folder across checkouts.** With one `CARGO_TARGET_DIR` per workspace shared by two checkouts, the second checkout's full lint fell from 458 s to 52 s. But cargo then passed root Clippy on a planted `clone_on_copy` error whose file time predated the other checkout's build. It reused the other checkout's verdict. That is a false green, so the option was removed before it landed. sccache already shares dependency compiles across worktrees; it cost 2.7 GB of disk in the probe.
3. **Keep the R install's build-folder removal for now** (`5eaac4c` reverted by `401bb7c`). Keeping `src/rust/target` cut the R check from 84 s to 18 s. The gate's home-path scan then read the kept `libthinkthen.a` and failed. The follow-up is to scan only the installed library, then keep the folder.
4. **Leave the gate's order alone.** It runs the workspace lint first on purpose, so a red Clippy cannot buy a green gate. The pass costs 7 to 18 s once lint has run.

## Open

- A fresh worktree fails the Ruby surface as "not set up": `check.sh` compares the Dockerfile's file time with a stamp, and a checkout gives the Dockerfile a new time. A content check (a Dockerfile hash as an image label) would fix it without a rebuild.
- Two timing tests failed in two of three warm gates under load and passed in the first: the Python cancelled-batch signal test and the DuckDB long-query interrupt.
- Running lanes' full rungs one at a time, or giving each `-j4`, is the largest lever the box has. The rung alone is 5 minutes cold.
