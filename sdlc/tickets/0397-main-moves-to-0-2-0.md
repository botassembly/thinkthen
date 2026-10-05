# 0397: Main moves to 0.2.0 and release/0.1 freezes

Status: landed. Lane: claude-0, before the three 0.2 lanes start. Parent: Ian's ruling of 2026-10-04 on the 0.2 plan. It closes no issue. It narrows `sdlc/issues/2026-10-04-each-patch-release-costs-two-hand-passes.md` and `sdlc/issues/2026-10-04-a-release-needs-two-approvals.md`.
Landed: 8bd84282b2d4218e64b07f471d3df75df699c203

Milestone: 0.2

## Outcome

1. Main's version metadata reads 0.2.0. `python3 sdlc/scripts/versions` passes and reports 70 places at 0.2.0.
2. Main's public install text still names 0.1.2, the latest published release. That covers the `install.sh` usage comment and its site copy, the release names in the Ada, COBOL, C#, Go, JVM and Objective-C READMEs, and the site's Rust `Cargo.toml`, which keeps `thinkthen = "0.1"`. The deployed site keeps naming 0.1.2 until 0.2.0 ships.
3. The site's binding smoke builds Rust samples against main's 0.2.0 crate while the page keeps `"0.1"`. Every stale sample is proved again offline, and `node scripts/check-binding-proofs.mjs --strict` passes in `site/` with no warning.
4. `CHANGELOG.md` opens with an `## Unreleased: 0.2.0` heading. `libraries/dart/CHANGELOG.md` opens with an unreleased 0.2.0 heading.
5. The release process says what Ian ruled: no 0.1.x releases, `release/0.1` is frozen, nothing is cherry-picked to it, and main always carries the next version. ADR 0116 records the ruling as an amendment. The 0.2 "Blockers" text in `milestones.md` no longer says a 0.1 fix is cherry-picked.
6. The two release issues that the ruling touches say what changed and stay open for what remains.

## Evidence

- Starts from: Ian's ruling of 2026-10-04, in the 0.2 lane-order message from the docs team. "No 0.1.x patch releases. ThinkThen has no real users yet, so all fixes land on main and ship in 0.2. Freeze `release/0.1` and copy nothing to it. A standing release approval is not needed." And: "Move main to 0.2.0 first." No experiment preceded this ticket. The evidence is a dry run and the 0.1 release record.
  - A dry run on a scratch copy of main `562a59701`: `python3 sdlc/scripts/versions --set 0.2.0` wrote 70 places in 49 files, and `versions` then passed. The copy was thrown away.
  - After that run, `git grep -nE '(^|[^0-9.])0\.1\.0([^0-9.]|$)'` outside `sdlc/records`, `sdlc/tickets`, `sdlc/issues`, `sdlc/planning`, `probes`, locks and `.jsonl` files found only these, each kept:
    - `.github/workflows/gate.yml` line 31, `MUSTMATCH_VERSION: 0.1.0`. It pins a test tool's version, not ThinkThen's.
    - `libraries/c/include/thinkthen.h` lines 50 and 68: "Version 0.1.0 freezes the symbol names..." and "Version 0.1.0 is the first release." Both state history that stays true at 0.2.0. The ABI froze at 0.1.0. The three lines `versions` owns move to 0.2.0: line 2, line 67 and `THINKTHEN_VERSION_MINOR`.
    - `libraries/go/README.md` line 17, the local `pkg-config` example. Ticket 0396 kept it because it names the checkout's own build, so it should name the checkout's version. The builder changes it to 0.2.0. The grep then no longer finds it.
    - `sdlc/scripts/installer-test`, the `install.sh` edge table. Its fake releases are named 0.1.0 and 0.2.0 on purpose.
    - `crates/thinkthen/src/core/mod.rs` line 143, a version-parse fixture, `0.1.0-rc.1`.
    - `sdlc/scripts/release-archive-self-test.py` line 41, a refused ref `refs/tags/v0.1.0`.
    - `sdlc/scripts/versions` line 79, a comment naming the 0.1.0 dry run.
    - `site/examples/how-tos/search-youtube-transcripts-by-meaning/files/measured.txt`, which records the version a measurement used.
    - The changelog headings.
  - `site/examples/install/rust/files/Cargo.toml` asks for `thinkthen = "0.1"`, so the grep does not find it. `site/scripts/smoke-bindings.mjs` builds Rust samples in a scratch project. `ARCHIVE.rust` writes a `[patch.crates-io]` entry that points `thinkthen` at `crates/thinkthen`, sets `[net] offline = true`, and copies the root `Cargo.lock`. `FRAGMENT.rust` copies the page's `Cargo.toml` into that project. A Cargo patch applies only when the patched crate's version meets the requirement. On the scratch copy at 0.2.0, `cargo tree` in such a project failed: "failed to select a version for the requirement `thinkthen = \"^0.1\"`; candidate versions found which didn't match: 0.2.0, 0.0.1". With `"0.2"` it resolved to the patched crate. So the Rust samples break unless the smoke changes. The page cannot ask for `"0.2"`, because no 0.2 is published and a reader's Cargo would fail.
  - `CHANGELOG.md` carried `## Unreleased: 0.1.0` before the 0.1.0 release commit `b3f3f5d86` dated it. That is the repo's habit. `libraries/dart/CHANGELOG.md` gained its 0.1.0 heading only in the release commit. The rehearsal's `registries` job runs `dart pub publish --dry-run` through `release-registry.py dry-run`, which fails on any nonzero exit. pub checks that the changelog names the pubspec version. The Dart changelog therefore gains its heading now, so a 0.2 rehearsal from main does not depend on how pub grades that check.
  - `CITATION.cff` holds only `version: "0.1.0"`, which `versions --set` writes. It has no release date to change.
  - `sdlc/planning/release-process.md` section 5 tells a 0.1.x release to cherry-pick each fix to `release/0.1` and to bump the version there alone. Section 4 item 1 says rehearsals run from the release branch after the cut. ADR 0116 items 4 and 5 say every 0.1.x release comes from `release/0.1` with cherry-picks. `milestones.md`'s 0.2 "Blockers" says "A fix that 0.1 needs is cherry-picked to `release/0.1` (ADR 0116)."
  - ADRs in this repo record a later change as a section headed `## Amendment, YYYY-MM-DD: <what changed>` at the end, with a sentence added to the Status line. ADR 0004 is the model.
  - Debt 035 (`2026-10-04-each-patch-release-costs-two-hand-passes.md`) has two costs. Item 1 is the re-proof on `release/0.1` after each 0.1.x bump. The ruling makes item 1 moot. Item 2, moving the install text after a release and proving the site again, still happens once per release. This ticket pays one such pass itself, because the bump changes hashed trees. The docs team's ranking of 2026-10-04 folds debt 035 into the docs story ticket.
  - `2026-10-04-a-release-needs-two-approvals.md` asks whether `publish` needs its own approval. The docs team's ranking of 2026-10-04 answers it: "Keep the two approvals; they are a safety gate," and keep the rest "as debt until a user or a release needs them". Ian's "a standing release approval is not needed" answers a question about patch releases. With no patch releases, no standing approval is wanted.
- Keeps: what stays as it is.
  - `release/0.1` and every published 0.1 tag stay as they are. No commit goes to `release/0.1`.
  - The rehearse rule in `sdlc/scripts/release-workflow`'s `resolve`: main or `release/X.Y`. A 0.2 cut still makes `release/0.2` under ADR 0116 items 1 to 3 and 7.
  - Every line ticket 0396 moved to 0.1.2, and the site's Rust `Cargo.toml` text.
  - `gate.yml`'s `MUSTMATCH_VERSION`, the `thinkthen.h` history sentences, and every fixture listed above.
  - Every sample's code and saved output. The Pages workflow. The site is not redeployed by this ticket.
  - The release workflow's two approvals.
- Changes: one commit on the ticket branch, in this order.
  - `python3 sdlc/scripts/versions --set 0.2.0`. The 49 files it writes.
  - `libraries/go/README.md` line 17: the local `pkg-config` file says `Version: 0.2.0`.
  - `CHANGELOG.md`: `## Unreleased: 0.2.0` above `## 0.1.2 (2026-10-03)`, with one sentence that 0.2 is in progress on main. `libraries/dart/CHANGELOG.md`: `## 0.2.0 (unreleased)` with one sentence. Dart changelog headings carry no date. The release commit dates CHANGELOG.md and turns the Dart heading into `## 0.2.0`.
  - `site/scripts/smoke-bindings.mjs`: after the page's `Cargo.toml` lands in the scratch project, the smoke rewrites the scratch copy's `thinkthen` requirement to the working tree's major and minor version, read from `crates/thinkthen/Cargo.toml`. It rewrites only the scratch copy. One helper serves both the install page and the function-page fragments. A comment says why: a Cargo patch applies only within the requirement, and the page names the published release.
  - `site/examples/bindings-proof.json`: every stale page proved again with `node scripts/smoke-bindings.mjs`.
  - `sdlc/planning/release-process.md`:
    - Section 4 item 1: rehearsals run from main until a release branch is cut for the next version. `release/0.1` is frozen.
    - Section 5: replace the "After the cut" paragraph and the 0.1.x list with the rule. There are no 0.1.x releases. `release/0.1` is frozen, and nothing is cherry-picked to it. Every fix lands on main and ships in the next release. Right after a release, main moves to the next version, as ticket 0397 did for 0.2.0. The public install text on main names the latest published release until the next one ships.
    - Step 2: the release commit no longer runs `versions --set`, since main already carries the version. It dates the changelog headings and writes the install text, as ticket 0396 did for 0.1.2. Keep the step's grep, with the old version in place of 0.0.1.
  - `sdlc/planning/adr/0116-release-branches-cut-at-the-release-candidate.md`: a Status sentence and `## Amendment, 2026-10-04: no 0.1.x releases`. It states Ian's ruling: `release/0.1` is frozen, items 4 and 5 no longer apply to 0.1, and main moved to 0.2.0 at once. Items 1 to 3 and 7 stand for the 0.2 cut. Ian can overturn it.
  - `sdlc/planning/milestones.md`, the 0.2 "Blockers": none. `release/0.1` is frozen under Ian's ruling of 2026-10-04, and main carries 0.2.0 (ticket 0397). Fixes land on main and ship in 0.2. Add this ticket to the 0.2 open items.
  - `sdlc/issues/2026-10-04-each-patch-release-costs-two-hand-passes.md`: a Status sentence that item 1 is moot under the ruling, and that item 2 remains once per release. Change `Pay when:` to "before the 0.2 release". Name the docs story ticket as owner, per the docs team's ranking. The issue stays open.
  - `sdlc/issues/2026-10-04-a-release-needs-two-approvals.md`: a Status sentence that both approvals stay as a safety gate, per the 2026-10-04 ranking, and that no standing approval is wanted. Add `Kind: debt`, the next free `Debt:` number, `Severity: low` and `Pay when: a user or a release needs it`. Items 2 and 4 stay as the debt. The issue stays open.
- Proof: offline checks only. No live call.
  - `python3 sdlc/scripts/versions` passes with "70 places read 0.2.0". `python3 sdlc/scripts/versions --self-test` passes.
  - The grep above finds only the kept list, with no Go README line.
  - `git grep -n '0\.1\.2' -- install.sh site/public/install.sh libraries/*/README.md` still finds every line ticket 0396 wrote, and `site/examples/install/rust/files/Cargo.toml` still says `"0.1"`.
  - In `site/`: `node scripts/smoke-bindings.mjs` on the Rust install page and one Rust function page passes. That case fails before the smoke change, so it is the regression. Then the other stale pages, then `node scripts/check-binding-proofs.mjs --strict` with no page to prove again and no warning.
  - `cargo build --release --locked --bin thinkthen`, then `npm run build` in `site/` with `THINKTHEN_BIN` set, as the Pages workflow runs it. The built install page names 0.1.2.
  - `python3 sdlc/scripts/workflows --self-test`, which runs the release resolve cases at the new version.
  - The coordinator's checkpoint names when `sdlc/scripts/test` and `surfaces` run. The bump touches every binding's metadata, so the first checkpoint after landing covers them.
  - `sdlc/scripts/lint` and `python3 sdlc/scripts/tickets`.
- Defers: what this ticket leaves.
  - The 0.2 release commit, its changelog dates and its install text.
  - Debt 035 item 2 and its options, which go to the docs story ticket.
  - A rehearsal from main at 0.2.0. The first 0.2 rehearsal proves the Dart changelog heading and the packaging at 0.2.0. Ticket 0398 changes what a rehearsal checks.
  - Deleting or protecting `release/0.1` on GitHub. Freezing it needs only that no one pushes to it. Branch protection is a GitHub setting Ian holds.

## What Ian can overturn

- Keeping the two `thinkthen.h` history sentences at 0.1.0.
- The Dart changelog heading now, not at the release commit.
- Leaving `release/0.1` unprotected on GitHub.
