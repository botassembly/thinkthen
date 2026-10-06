# 0393: npm publishes through staged publishing

Status: ready. Reserved 2026-10-03 from the 0.1.1 release run. Not started.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

1. The release workflow's `npm` job publishes with `npm stage publish`, as npm recommends for trusted publishing. The package waits on npmjs.com until a maintainer approves it there.
2. Ian can untick direct publishing on the npm trusted publisher for `thinkthen`, and the next release still publishes npm.
3. `release-process.md` section 6 says that npm holds each release until Ian approves it on npmjs.com. The GitHub `release` environment approval still releases the job.

## Evidence

- Starts from: release run 37059415069, dispatched from tag `v0.1.1` at `9463cef05` (record `sdlc/records/0128-release-0-1.md`).
  - The first attempt's `npm` job, job 111040245433, ran `npm publish "./npm-dist/thinkthen-0.1.1.tgz" --provenance --access public` with npm 11.6.4. npm answered `npm error code E403` and `npm error 403 403 Forbidden - PUT https://registry.npmjs.org/thinkthen - OIDC permission denied for this action`. Every other publish job passed.
  - The npm trusted publisher allowed only staged publishing. Ian then allowed direct publishing on npmjs.com, and the rerun's `npm` job, job 111193360031, published `thinkthen@0.1.1` with a provenance statement.
  - Direct publishing stays allowed today, so an approved release-mode run publishes to npm with no second approval on npmjs.com. npm recommends staged publishing for trusted publishers.
- Keeps: every guard the `npm` job has today. It runs in release mode only, after `resolve` and `draft`, in the `release` environment, behind the `RELEASE_ARMED` first step. It logs in by trusted publishing and reads no secret. It publishes the same `./npm-dist/thinkthen-$VERSION.tgz` the `npm-pack` job built and dry-ran, with provenance and public access. `sdlc/scripts/workflows` keeps refusing a bare relative path (ticket 0391).
- Changes: the `npm` job, its workflow rule and the release steps.
  - `.github/workflows/release.yml`: the `npm` job installs an npm release that has `npm stage publish` and runs it on the same file with the same flags. The `npm-pack` dry run uses the same npm release and the same command, if npm offers a dry run for it.
  - `sdlc/scripts/workflows`: its `npm publish` rules also read `npm stage publish` lines, so the bare-path refusal still holds. One planted case proves it.
  - `sdlc/planning/release-process.md` section 6: the npm approval step on npmjs.com, and that the GitHub release's `publish` job can run before npm's approval.
- Proof: local workflow checks, a rehearsal, and the next release run.
  - `python3 sdlc/scripts/workflows --self-test` and `python3 sdlc/scripts/workflows`, with the planted bare-path case on the new command.
  - `python3 sdlc/scripts/workflows --remote-pins` before the dispatch, by hand.
  - A rehearsal from the release branch passes `npm-pack` with the new dry run.
  - The next release run: the `npm` job stages the package with direct publishing unticked, Ian approves it on npmjs.com, and `npm view thinkthen version` names the new version. The ticket records the job number and the staged result.
- Defers: the npm version, the setting on npmjs.com, and the other registries.
  - The npm release and the exact command form. The builder reads npm's documentation for staged publishing and pins the first npm release that has it. If npm has no dry run for staging, `npm-pack` keeps `npm publish --dry-run` and the ticket says so.
  - Unticking direct publishing on npmjs.com. Ian does that after this lands, before the next release.
  - Staged publishing on the other registries. Each registry's own approval feature is a separate decision.

## What Ian can overturn

- Staged publishing on npm at all. Direct publishing works today.
- A second approval on npmjs.com after the GitHub `release` environment approval.


## Current planning boundary

PM ask 8 keeps this existing ticket as npm’s owner. Registry setup and direct publishing already exist; the missing outcome is staged publishing. Validate the exact supported npm command/version against official documentation before changing the workflow. Plan and offline workflow tests do not change registry settings or publish a package. Integration rehearsal and actual staging are later authorized qualification steps; no such run is scheduled by 0425.
