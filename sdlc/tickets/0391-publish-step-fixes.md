# 0391: Fix the PyPI, npm and NuGet publish steps that failed in the 0.1.0 release

Status: in progress. Lane claude-3. Branch `ticket/0391-publish-step-fixes`. Parent: ticket 0128 phase 4. Cherry-picked to `release/0.1` under ADR 0116 item 5.

Milestone: 0.1

## Outcome

1. The `pypi` job pins `pypa/gh-action-pypi-publish` to commit `ed0c53931b1dc9bd32cbe73a98c7f6766f8a527e`, the commit that tag `v1.13.0` points to. Every other `uses:` pin in `.github/workflows/*.yml` is a commit, checked against GitHub.
2. `sdlc/scripts/workflows --remote-pins` asks GitHub whether each pinned SHA is a commit and names any pin that is not. The coordinator runs it by hand before a release dispatch. The gate run of `workflows` stays offline and does not run it.
3. The `npm` job publishes `./npm-dist/thinkthen-$VERSION.tgz`. A path starting with `./` makes npm read a file, not a GitHub shorthand.
4. The `npm-pack` job, which a rehearsal runs, dry-runs `npm publish` on that same path. A wrong path then fails the rehearsal.
5. `sdlc/scripts/workflows` refuses an `npm publish` whose package argument is a bare relative path, with one planted case.
6. `release-registry.py nuget-push` sends the header `X-NuGet-Protocol-Version: 4.1.0`. A NuGet refusal prints the status, the reason and up to 500 characters of NuGet's reply, never the key. A 409 no longer claims the version exists, because nuget.org also answers 409 for a reserved prefix.
7. `release-registry-self-test.py` proves with a stubbed reply that the push sends the header and the key, and that a 400 and a 409 print NuGet's message without the key.
8. A separate commit on `release/0.1` only sets the version to 0.1.1, with the CHANGELOG entry. Main keeps 0.1.0 (release-process.md section 5: "After the cut, main carries 0.1.0 and takes 0.2 work").

## Evidence

- Starts from: release run 37035814818, dispatched from tag `v0.1.0` on `release/0.1` at `b691a2bc6`.
  - Builds, smokes and `draft` passed. `maven`, `pub` and `tap` published 0.1.0. The Go module proxy also holds `v0.1.0`, so the tag cannot move and the next release is 0.1.1.
  - `rubygems` and `crates` failed because neither registry has a trusted publisher yet. Ian sets those up; this ticket changes no code for them.
  - `pypi`: the runner ran `docker run ... ghcr.io/pypa/gh-action-pypi-publish:106e0b0b7c337fa67ed433972f777c6357f78598` and Docker answered "manifest unknown". `gh api repos/pypa/gh-action-pypi-publish/git/tags/106e0b0b7c337fa67ed433972f777c6357f78598` shows an annotated tag object `v1.13.0` pointing at commit `ed0c53931b1dc9bd32cbe73a98c7f6766f8a527e`. The action tags its image by the commit, so the tag object's SHA names no image. `gh api .../git/commits/<sha>` confirms all 16 other distinct pins in `gate.yml`, `pages.yml`, `release.yml` and `windows.yml` are commits.
  - `npm`: `npm publish npm-dist/*.tgz --provenance --access public` failed with `npm error command git --no-replace-objects ls-remote ssh://git@github.com/npm-dist/thinkthen-0.1.0.tgz.git`. npm reads `owner/name` with no leading `./` as a GitHub repository. A local dry run with npm 10.9.8 reproduces it offline: `npm publish --dry-run dist/x.tgz` exits 128 with the same git error, and `npm publish --dry-run ./dist/x.tgz` exits 0. The other npm commands (`npm pack` in `release-workflow npm-assemble` and `release-pack`) pack the current folder and take no path.
  - `nuget`: `NuGet/login` printed "Successfully exchanged OIDC token for NuGet API key". Then `release-workflow nuget-push registry/nuget/Botassembly.ThinkThen.*.nupkg` printed only "release-registry: NuGet answered 400".
- Starts from: the NuGet investigation.
  - The `registry-packages` artifact of run 37035814818 holds `Botassembly.ThinkThen.0.1.0.nupkg`. Its nuspec has id `Botassembly.ThinkThen`, version `0.1.0`, authors, a description, `<license type="file">LICENSE</license>` with the standard deprecation `licenseUrl`, `<readme>README.md</readme>`, `<repository type="git" />` and one empty `net8.0` dependency group. LICENSE and README.md are in the package. No entry is dated in the future, and no path has a double slash.
  - The id is free: `https://api.nuget.org/v3/registration5-semver1/botassembly.thinkthen/index.json` and the flat container both answer 404, and the search service finds no `Botassembly.ThinkThen` and no package under owner `botassembly`.
  - NuGetGallery's `ApiController.CreatePackageInternal` first evaluates the user's push policies and answers 400 with the policy's message when one fails. nuget.org's `DefaultSubscription` applies `RequireMinProtocolVersionForPushPolicy` with version 4.1.0 to every user. That policy reads `X-NuGet-Protocol-Version`, or the older `X-NuGet-Client-Version`, and fails when neither is present. Its message is "A client version '4.1.0' or higher is required to be able to push packages". `nuget-push` sent neither header, so this policy most likely gave the 400. The NuGet client sends `X-NuGet-Protocol-Version: 4.1.0` on every push.
  - Other 400 sources in the same method are package checks: zip entries, `EnsureValid`, the nuspec manifest check, the min client version, and license and readme validation. The nupkg passes each check that can be read offline. A 403 would mean the trusted-publishing policy's scope does not allow a new package id; a 409 would mean a reserved prefix. Neither was the answer.
- Keeps: every job, guard and order in `release.yml`: release mode only, `needs: [resolve, draft]`, environment `release`, the `RELEASE_ARMED` first step, trusted publishing with no secret, and `workflow_dispatch` as the only trigger. `nuget-push` still reads the one-hour key from `NUGET_API_KEY` and sends it only to `https://www.nuget.org/api/v2/package`. `workflows` stays offline in the gates.
- Changes: three publish steps, one workflow rule, one hand-run check and the version bump.
  - `.github/workflows/release.yml`: the `pypi` pin; the `npm` job's publish path and its `VERSION` from `resolve`; a dry-run step in `npm-pack`.
  - `sdlc/scripts/workflows`: the bare-path `npm publish` rule with its planted case, the `--remote-pins` mode, and a comment saying why the gate cannot tell a tag object from a commit.
  - `sdlc/scripts/release-registry.py` and `release-registry-self-test.py`: the header, the reply on refusal, and the stubbed push cases.
  - `sdlc/planning/release-process.md`: section 6 names `workflows --remote-pins` before a release dispatch.
  - On `release/0.1` only, a second commit: `sdlc/scripts/versions --set 0.1.1`, the 0.1.1 CHANGELOG entry, `libraries/dart/CHANGELOG.md`, and the hand-written copies release-process.md section 5 lists.
- Proof: local checks only; no tag and no dispatch.
  - `python3 sdlc/scripts/workflows --self-test` and `python3 sdlc/scripts/workflows`. The planted case puts back `npm publish npm-dist/*.tgz` and fails.
  - `python3 sdlc/scripts/workflows --remote-pins` run by hand once: it names the old `106e0b0b` pin on the parent commit and passes on this branch.
  - `python3 sdlc/scripts/release-registry-self-test.py`: the push sends `X-NuGet-Protocol-Version: 4.1.0` and the key; a stubbed 400 and 409 print NuGet's reply and never the key.
  - `sdlc/scripts/lint` with the private-names list, and `python3 sdlc/scripts/tickets`.
  - On `release/0.1` after the bump: `python3 sdlc/scripts/versions`, `python3 sdlc/scripts/versions --tag v0.1.1`, and the release-process `git grep` for stale versions.
- Defers: the runner proof and the registry settings.
  - The runner proof. A rehearsal skips the publish jobs, so the next release run is the first proof of the `pypi`, `npm` and `nuget` fixes. Only `npm-pack`'s dry run runs in a rehearsal.
  - NuGet's real answer if the header was not the whole cause. The new refusal line prints it. The remaining candidates are a trusted-publishing policy whose scope forbids a new package id (403) and a reserved `Botassembly` prefix (409); both are Ian's nuget.org settings.
  - `twine check` in a rehearsal. The PyPI action already runs it before upload (`verify-metadata: true`), and a rehearsal copy would need a pinned, hash-checked twine install. The smoke jobs already install each wheel.
  - The crates.io and RubyGems trusted publishers, which Ian sets up.

## What Ian can overturn

- `--remote-pins` as a hand-run check before dispatch rather than a step inside the release workflow.
- Keeping 0.1.0 on main after the 0.1.1 bump, per release-process.md section 5.
