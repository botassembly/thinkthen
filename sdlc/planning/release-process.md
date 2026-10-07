# Release process

Status: accepted 2026-10-01 on Ian's direction. Ian can overturn any step. This page is the one home of the coordinator's release process. Other pages link here and do not restate it.

## 1. Milestones

[milestones.md](milestones.md) holds the milestones, the `Milestone:` line rule and each milestone's open items.

## 2. Checkpoints

1. The coordinator runs one surface sweep on one main commit per round of landings, and fixes any red at once.
2. When every check passes, the sweep publishes: `sdlc/scripts/surfaces --publish checkpoint/surfaces/YYYY-MM-DD-N` with `THINKTHEN_PUBLISH_ROOT` set to the coordinator's builds folder. The [scripts README](../scripts/README.md) describes what the publish packs and checks.
3. The publish runs on Linux x86_64 only. A macOS publish stops by design on the Linux-only Ada, COBOL and Objective-C checks.
4. `publish-builds` keeps the newest three checkpoint folders and removes no other folder.
5. The coordinator tags the commit under [worktrees.md](worktrees.md), "Landing commits and tags".

## 3. Release QA rounds

1. Each checkpoint goes to the release QA team as one round. The coordinator sends it as a message through the mailroom (`pm send`).
2. QA findings come back as messages (`pm inbox`).
3. Each finding becomes a ticket, and the ticket lands through a lane like any other.
4. The next checkpoint carries the fixes into the next round.

## 4. Rehearsals

1. Stage 1 dispatches the release workflow by hand on the reviewed candidate tag: `gh workflow run release.yml --ref rc/0.2.0-rc.N -f mode=rehearse`. Dispatch `windows.yml` on the same tag. Ian delegated these qualification runs on 2026-10-07. Hosted runs happen at this candidate checkpoint, or for a Windows-only fix that cannot be checked locally. Batch Windows fixes; do not dispatch workflows after each ticket. `release/0.1` is frozen.
2. Rehearse mode builds, smokes and collects a draft. It publishes nothing.
3. Stage 1 passes through `draft` on all five targets: Linux x86_64 and ARM64, macOS ARM64 and Intel, and Windows. Required checks have zero "not run". A failed candidate is fixed on main and qualified under the next candidate tag. M5 is allowed for experiments and Mac-specific checks, including before the candidate; Beelink remains the primary build machine.
4. Ticket 0128 records each attempt and its result.

A rehearsal checks the publish inputs for each registry (ticket 0398 slice A):

| Registry | Rehearsal proof | What waits for release mode |
| --- | --- | --- |
| crates.io | `cargo publish --dry-run --locked --package thinkthen` after source packaging | Upload and trusted publisher |
| PyPI | `twine check --strict` on all four wheels | Upload and trusted publisher |
| npm | `npm stage publish --dry-run` on the exact packed file | Staging upload, trusted publisher and npm approval |
| RubyGems | Four checked platform gems and one matching-version Ruby diagnostic fallback | `gem push` and trusted publisher |
| NuGet | Packed package and offline protocol checks | Push and trusted publisher |
| Maven Central | Packed artifacts and signatures from a throwaway rehearsal key | Upload with the release signing key |
| pub.dev | `dart pub publish --dry-run` | Upload and trusted publisher |
| Homebrew tap | Rendered formula from four checked Unix archives, then `ruby -c` | Tap clone, commit and push |
| GitHub release and Go module | Draft assets and installed-file replay | Public release and Go tag |

Rehearsals hold no OIDC token, use no registry secrets, and require no environment approval. No dry run proves that a trusted publisher exists. Only the release run proves that registry setting. PyPI upload, RubyGems push, NuGet push, Maven Central upload, the tap push, the public GitHub release and the Go tag have no publish dry run in this workflow. Packagist and R-universe read the published repository and have no release job.

## 5. The release candidate and the release branch

[ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md) defines the release candidate and the cut. Its 2026-10-04 amendment freezes `release/0.1`.

The release branch is named for the major and minor version: `release/0.2` for 0.2. Ticket 0128 phase 4 holds the release details. The cut runs in this order:

1. **The release candidate holds on main.** Implementation, installed-package parity and local checks are complete. Only release fixes remain. Stage 1 qualifies this candidate; a previous rehearsal does not qualify a changed commit.
2. **The release commit lands on main.** Main already carries the next version, so the release commit does not run `versions --set`. On a ticket branch from main, the agent dates the changelog headings and writes the public install text for the release, as ticket 0396 did for 0.1.2:
   - `CHANGELOG.md`: date the unreleased heading. Remove `(unreleased)` from the matching heading in `libraries/dart/CHANGELOG.md`.
   - Update `install.sh`, its site copy, and the release names in `libraries/{ada,cobol,csharp,go,jvm,objective-c}/README.md`.
   - Update the site's install lines and `site/examples/install/rust/files/Cargo.toml`. The smoke runner patches its scratch copy to the working tree's version. Prove every stale page again before the site deploys.

   Afterwards, `git grep -nE '(^|[^0-9.])OLD_VERSION([^0-9.]|$)'`, with the old version escaped in place of `OLD_VERSION`, outside `sdlc/records`, `sdlc/tickets`, `sdlc/issues`, `sdlc/planning`, `probes`, locks and `.jsonl` fixtures finds only retained history and fixtures. The coordinator lands the commit.
3. **Checkpoint and QA on the cut.** The coordinator runs the checkpoint sweep of section 2 on that main commit. Release QA runs its round on it (section 3).
4. **The cut.** The coordinator tags `rc/0.2.0-rc.1` on that commit under worktrees.md and pushes `release/0.2` from it.
5. **Stage 1 on the candidate tag.** Dispatch `release.yml` with `mode=rehearse` and `windows.yml` from `rc/0.2.0-rc.N`. Both qualify the exact reviewed commit. Fix failures on main and use the next candidate tag. Release QA must pass before stage 2.
6. **Stage 2 after Ian's go.** Tag the qualified commit `v0.2.0` and dispatch `release.yml` with `mode=release` from that tag only after Ian explicitly authorizes publication. The release tag must name the exact commit that passed stage 1 and QA. Section 6 and ticket 0128 phase 4 continue from there.

There are no 0.1.x releases. `release/0.1` is frozen, and nothing is cherry-picked to it. Every fix lands on main and ships in the next release. Right after a release, main moves to the next version. Ticket 0397 moves main to 0.2.0 under Ian's ruling of 2026-10-04.

Main's public install text names the latest published release until the next release ships. A line that names the checkout's own build, such as the Go README's local `pkg-config` file, uses main's version. The site's Rust dependency names the published major and minor version. Its smoke runner rewrites only a scratch copy to build against main.

## 6. The release

1. The real release is dispatched only on Ian's go.
2. Before the dispatch, the coordinator runs `sdlc/scripts/workflows --remote-pins`. It asks GitHub whether each pinned action names a commit. A pin that names a tag object passes the offline gate and fails only in the release run (ticket 0391).
3. Release mode runs from a matching `v*` tag. Resolve refuses before any build unless that exact commit has a completed successful `release.yml` rehearsal dispatch from its matching candidate tag. An unreadable run history refuses. Rehearse mode reads no run history. Each publish job waits for Ian's run-specific approval in the GitHub `release` environment (ticket 0128).

   The resolver admits only matching `rc/VERSION-rc.N` candidate tags with positive N in rehearse mode. Release mode requires a successful manual release-workflow run from a matching candidate tag on the exact release commit. Branch rehearsals and ordinary release-tag runs do not qualify that commit.
4. The npm job stages the packed archive using trusted publishing. Ian reviews and approves the staged version on npmjs.com with two-factor authentication before npm makes it public. The GitHub `publish` job can finish before that approval. The initial public-package install check may report the npm version unavailable until approval; rerun that channel check after approval.
5. After publishing the GitHub release and Go module tag, `publish` dispatches `install-check.yml` from that release tag with the resolved version (ticket 0398 slice B). The separate workflow reads public channels with only `contents: read`. It has no environment approval, OIDC token or secret. The coordinator records its run and every channel result in the release's ticket.
6. A late Go proxy, Packagist or older R-universe index gets one hand dispatch later. R-universe keeps only its current version. A superseded request reports the requested and listed versions and gets no retry advice. Its Linux check accepts only the resolute R 4.6 binary. Hosted runner proof remains pending an approved manual check.

## 7. Registries and the site

The queue owner owns the code, the release workflow, the site (`site/`) and the registry setup steps since Ian's ruling of 2026-10-02. Ian holds the registry logins. The docs team owned the setup steps before that (ruling 15 of [cleanup-2026-09-30.md](cleanup-2026-09-30.md)). The registries are PyPI, npm, crates.io, RubyGems, NuGet, Maven, pub.dev and the Homebrew tap. [ownership.md](ownership.md) names the folders.
