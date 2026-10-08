# Release process

Status: accepted 2026-10-01 on Ian's direction. Ian can overturn any step. This page is the one home of the coordinator's release process. Other pages link here and do not restate it.

## 1. Milestones

[milestones.md](milestones.md) holds the milestones, the `Milestone:` line rule and each milestone's open items.

## 2. Checkpoints

1. The required local checkpoint is the complete installed-package campaign, its generated support table and the applicable test, lint, specification and documentation checks. Every required public consumer runs without missing or skipped cells. Reuse passing checks while their relevant code, inputs, environment and assumptions remain unchanged; rerun affected checks after a fix. Do not repeat a complete source matrix to prepare a QA handoff.
2. Send release QA the reviewed commit, qualified package locations, generated table, known rulings and outstanding platform checks through the mailroom. State which source and package revisions each check covers. A current table does not qualify an untested platform, and metadata changes do not manufacture a new product test result.
3. The existing `surfaces --publish TAG` helper can archive a separate debug-build checkpoint under `THINKTHEN_PUBLISH_ROOT`. It runs on Linux x86_64 only, keeps its newest three checkpoint folders and removes no other folder. Its debug packages do not replace the release packages tested by the required installed campaign. This archive is optional; release QA does not require another debug-build sweep or an auxiliary checkpoint tag.
4. If a surface checkpoint is tagged, follow [worktrees.md](worktrees.md): every named check must have passed on that exact commit. Release candidate tags follow section 5 and precede hosted rehearsal.

## 3. Release QA rounds

1. Give the release QA team one round on the final reviewed candidate through `pm send`. Supply the completed local package checkpoint and subsequent hosted qualification results. QA must pass before publication; a previous candidate's result does not qualify changed behavior.
2. Read QA findings through `pm inbox`. Fix consequential defects within the agreed release scope through tickets and lanes. Other newly found gaps become unscheduled tickets under Ian's ticket-intake rule; they do not silently expand the release.
3. Recheck affected behavior after a fix and complete the next candidate's required platform checks. Retain applicable checks and reviews for unchanged work. Add no receipt framework or separate per-language write-ups.

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

1. **The release candidate holds on main.** Implementation, installed-package parity and local checks are complete. Only release fixes remain. Stage 1 qualifies this candidate; a previous rehearsal does not qualify a changed commit. The existing `sdlc/scripts/test-full-cases --run --artifacts DIR` checkpoint must pass all required public variants against the release packages without missing or skipped cells before cutting the candidate. Its generated `target/parity/matrix.md` reports current functions, files, images and remaining work. Run the retained release-pack/release-smoke checkpoint separately; do not repeat a complete source matrix. A routine or Rust-only hosted gate does not replace this checkpoint.
2. **Candidate documentation lands on main.** Main already carries the next version, so candidate preparation does not run `versions --set`. On a ticket branch, update the changelog's implemented behavior and known limits. Keep `## Unreleased: 0.2.0` in `CHANGELOG.md` and `(unreleased)` in the matching Dart heading. The publication date remains unknown. Keep public install text and installer defaults at the latest published release, currently 0.1.2. The coordinator lands the reviewed documentation before qualification.
3. **Checkpoint and QA on the cut.** Give QA the completed installed-package checkpoint and its actual source/package revisions under sections 2 and 3. Retain applicable local checks; do not rerun an unchanged source matrix merely to archive debug packages. Final hosted qualification must cover the reviewed candidate commit.
4. **The cut.** The coordinator tags `rc/0.2.0-rc.N` on that reviewed commit and pushes `release/0.2` from it under worktrees.md. Candidate numbers are positive and increase after fixes. Create the candidate tag before dispatching rehearsal.
5. **Stage 1 on the candidate tag.** Dispatch `release.yml` with `mode=rehearse` and `windows.yml` from `rc/0.2.0-rc.N`. Both qualify the exact reviewed commit. Fix failures on main and use the next candidate tag. Release QA must pass before stage 2.
6. **Stage 2 after Ian's go.** Tag the qualified commit `v0.2.0` and dispatch `release.yml` with `mode=release` from that tag only after Ian explicitly authorizes publication. The release tag must name the exact commit that passed stage 1 and QA. Do not insert a date or public-install commit between qualification and the release tag. Section 6 and ticket 0128 phase 4 continue from there.
7. **After actual publication.** On a reviewed ticket branch from main, record the actual publication date in `CHANGELOG.md` and remove `(unreleased)` from the matching Dart heading. Update `install.sh`, its site copy, release names in `libraries/{ada,cobol,csharp,go,jvm,objective-c}/README.md`, the site's install lines and `site/examples/install/rust/files/Cargo.toml` to the published release. Keep checkout-local build references on main's version. The smoke runner patches only its scratch copy. Check affected examples before the site deploys. This documentation commit follows publication and does not change the qualified release commit.

   Afterwards, search for the escaped previous version outside `sdlc/records`, `sdlc/tickets`, `sdlc/issues`, `sdlc/planning`, `probes`, locks and `.jsonl` fixtures. Retain historical references and fixtures. The coordinator lands the documentation commit. The current release workflow uses its resolved version for the draft title and notes; it does not require a dated changelog heading before rehearsal or publication.

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
