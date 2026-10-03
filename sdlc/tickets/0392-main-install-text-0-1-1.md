# 0392: Move main's public install text to 0.1.1 and redeploy the site

Status: in progress. Lane claude-4. Branch `ticket/0392-main-install-text-0-1-1`. Parent: ticket 0391's deferred follow-up.

Milestone: 0.1

## Outcome

1. Main's public install text names 0.1.1: the `install.sh` usage comment and its site copy, and the release names in the Ada, COBOL, C#, Go, JVM and Objective-C READMEs.
2. Main's version metadata stays at 0.1.0. Release-process.md section 5 says the 0.1.x bump lands only on `release/0.1` and main keeps its version. `sdlc/scripts/versions` still passes on main.
3. Lines that name the checkout's own build keep 0.1.0. The Go README's local `pkg-config` line describes the C library built from this checkout, which is 0.1.0 on main. `databases/postgresql/NOTES.md` names `thinkthen--0.1.0.sql`, which main's control file still makes.
4. The site's Rust install `Cargo.toml` asks for `thinkthen = "0.1"`. A reader's Cargo takes the newest published 0.1.x, which is 0.1.1. The site smoke patches the crate to this working tree, and main's crate is 0.1.0, so `"0.1.1"` would fail the patch and an offline build. `"0.1"` serves both.
5. The site's R install line names R-universe first: `install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))`. Its note says R-universe builds the package from each release.
6. Every site sample whose proof went stale is proved again offline with `site/scripts/smoke-bindings.mjs`. `node scripts/check-binding-proofs.mjs --strict` passes with no warning.
7. Release-process.md section 5 gains step 5 of the 0.1.x list: after a 0.1.x publishes, a ticket on main moves the public install text, keeps the version metadata, proves the stale site pages and redeploys.
8. Issue `2026-09-25-release-and-install-for-0-1.md` item 4, the site's R line, is marked done.
9. After landing, the builder who owns the site dispatches the Pages workflow by hand from main. Ian approved finishing 0.1 on 2026-10-03, including this redeploy. Its `prove` job sees this commit at thinkthen.dev, and `curl` shows the R-universe line and the 0.1.1 installer comment.

## Evidence

- Starts from: ThinkThen 0.1.1 is published, and the GitHub release `v0.1.1` is Latest. Ticket 0391's Defers: "After 0.1.1 publishes, a follow-up on main moves those public install lines to 0.1.1; until then nobody should deploy Pages." `release/0.1` commit `18458f33b` bumped these copies on the release branch. `9463cef05` re-proved the Rust install samples there. On main `81062bd31`, `check-binding-proofs.mjs` warns that 34 pages need proving again and fails the strict mode the Pages workflow runs. `CHANGELOG.md` and `libraries/dart/CHANGELOG.md` on main already hold the 0.1.1 entries, from ticket 0391. The README's Install section names no version. The site's catalog writes `VERSION` in archive names, so only the Rust `Cargo.toml` and `install.sh` carry a number. `https://botassembly.r-universe.dev/api/packages/thinkthen` answered 404 on 2026-10-03, and the universe lists no package yet.
- Keeps: every version file `versions` checks, `release/0.1`, the measured line in the YouTube how-to that records the 0.1.0 run, every sample's code and saved output, and the Pages workflow.
- Changes: `install.sh`, `site/public/install.sh`, six binding READMEs, `site/examples/install/rust/files/Cargo.toml`, `site/src/data/catalog.mjs`, `site/examples/bindings-proof.json`, `sdlc/planning/release-process.md`, the issue line, and this ticket.
- Proof: offline checks only. No live call.
  - `python3 sdlc/scripts/versions`.
  - `node scripts/smoke-bindings.mjs` on each stale sample and every Rust sample, then `node scripts/check-binding-proofs.mjs --strict`.
  - `cargo build --release --locked --bin thinkthen`, then `npm run build` in `site/` with `THINKTHEN_BIN` set, as the Pages workflow runs it.
  - `sdlc/scripts/lint` with the private-names list, and `python3 sdlc/scripts/tickets`.
  - After the dispatch: the Pages run result, and `curl` of `https://thinkthen.dev/install/r/`, `/install/rust/` and `/install.sh`.
- Defers: the R-universe build. R-universe tracks the latest GitHub release and had not built the package on 2026-10-03. The public install checks confirm it. `release/0.1` needs no change.

## What Ian can overturn

- Main keeps 0.1.0 as its version while its public text names 0.1.1.
- `"0.1"` in the site's Rust `Cargo.toml` in place of an exact `"0.1.1"`.

## What the build taught us
