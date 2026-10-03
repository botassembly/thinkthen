# 0396: Move main's public install text to 0.1.2

Status: landed. Lane claude-0. Branch `ticket/0396-main-install-text-0-1-2`. Parent: release-process.md section 5, step 5 of the 0.1.x list.

Milestone: 0.1

## Outcome

1. Main's public install text names 0.1.2: the `install.sh` usage comment and its site copy, and the release names in the Ada, COBOL, C#, Go, JVM and Objective-C READMEs.
2. Main's version metadata stays at 0.1.0. `sdlc/scripts/versions` still passes on main.
3. Lines that name the checkout's own build keep 0.1.0. The Go README's local `pkg-config` line is one.
4. The site's Rust install `Cargo.toml` keeps `thinkthen = "0.1"`. A reader's Cargo takes 0.1.2.
5. Every site sample whose proof went stale is proved again offline with `site/scripts/smoke-bindings.mjs`. `node scripts/check-binding-proofs.mjs --strict` passes with no warning.

## Evidence

- Starts from: ThinkThen 0.1.2 is published to the GitHub release `v0.1.2`, every registry, and the Go tag `libraries/go/v0.1.2`. `release/0.1` commit `d532f2697` set every version to 0.1.2, and `abac3ce61` re-proved the Rust install samples there. Ticket 0392 did this step for 0.1.1. On main `652f36ed8`, `check-binding-proofs.mjs` reports 22 pages to prove again, and its strict mode flags 117 more function-page samples. The site's catalog writes `VERSION` in archive names and names no release number.
- Keeps: every version file `versions` checks, `release/0.1`, the site's Rust `Cargo.toml`, every sample's code and saved output, and the Pages workflow.
- Changes: `install.sh`, `site/public/install.sh`, six binding READMEs, `site/examples/bindings-proof.json`, and this ticket.
- Proof: offline checks only. No live call.
  - `python3 sdlc/scripts/versions`.
  - `node scripts/smoke-bindings.mjs` on each stale page's samples, then `node scripts/check-binding-proofs.mjs --strict`.
  - `cargo build --release --locked --bin thinkthen`, then `npm run build` in `site/` with `THINKTHEN_BIN` set, as the Pages workflow runs it.
  - `sdlc/scripts/lint` and `python3 sdlc/scripts/tickets`.
  - `git grep -n '0\.1\.1'` outside changelogs, `sdlc` records, tickets, issues and planning, `probes`, locks and Cargo metadata finds no public install text.
- Defers: the Pages redeploy from main. The coordinator dispatches it after this ticket lands.

## What Ian can overturn

- Main keeps 0.1.0 as its version while its public text names 0.1.2.

## What the build taught us

- The non-strict check named 22 install pages. The strict check also flagged 117 function-page samples, since engine and binding changes since 0.1.1 staled them. `smoke-bindings.mjs` replayed 163 samples offline, and the strict check reports 350 samples matching their proofs and no page to prove again.

## Review

- Ticket and code review: one fresh read-only reviewer read both. ACCEPT, with no findings. In a clean checkout, `check-binding-proofs.mjs --strict` matched 350 samples and `sdlc/scripts/lint` passed.
