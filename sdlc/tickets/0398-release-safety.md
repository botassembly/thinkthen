# 0398: Release safety: publish dry runs, install checks after publishing, and a rehearsed tag

Status: in progress. Slices A, B, B2 and C landed through `ddfcbc74c`; B2 fixes the R binary contribution paths. Hosted rehearsal and installed-package proof remain pending.

Milestone: 0.2

## Outcome

1. Slice A. A rehearsal runs every publish step that has a dry run, and publishes nothing.
2. Slice B. After `publish` passes, the release run starts a separate install-check workflow that installs every channel from its live registry on fresh runners, reads the version, and replays the first-run sample with no key.
3. Slice C. Release mode refuses a `v*` tag whose commit has no successful rehearsal run of `release.yml`.
4. Gates stay offline. The live parts are proved by a rehearsal and the 0.2 release run.

## Evidence

- Starts from: the three 0.1 release runs in `sdlc/records/0128-release-0-1.md` and the design record `sdlc/records/0398-release-safety.md`.
- Done: slices A, B, B2 and C, with their offline tables in the gates. B2 fixes the R binary contribution paths.
- Changes: no product files remain. The remaining items record live runs in this ticket.
- Keeps: every guard the release workflow has. Rehearse mode publishes nothing, holds no `id-token`, reads no secret except `GITHUB_TOKEN`, and asks for no approval. The nine outward jobs, environment `release`, the `RELEASE_ARMED` first step, trusted publishing and both approvals stay.
- Remaining work:
  - The rc.13 rehearsal under 0543 passes the crates.io dry run, `twine check` and the tap formula, publishes nothing and asks for no environment approval. Record its run number here.
  - Open question for the coordinator: does the hand dispatch of `install-check.yml` against the published 0.1.2 still apply? Check whether R-universe still indexes 0.1.2. If it does, dispatch it and record the run. If R-universe has superseded 0.1.2, use the current published version for the full passing channel table and record the older-version refusal separately, or rule the check replaced by the 0.2 release run, and say which here.
  - Run slice C's query by hand and read-only on the 0.1.2 commits. For `abac3ce61` it finds rehearsal 37126990511. For `08328c9c0` it finds successful release-tag run 37130570517, whose branch is `v0.1.2`, and no eligible branch rehearsal, so 0.1.2's tag would have been refused.
  - The 0.2 release run under 0425. `resolve` accepts the rehearsed tag. `publish` starts the install check, and every row of the channel table passes. Each trusted publisher is first proved here.
- Proof: Each remaining item is recorded here with its run number or query result.
- Defers:
  - A rehearsal check that each trusted publisher exists. It needs an OIDC token, which `workflows` allows only in environment `release`, behind an approval. A separate `rehearse` environment would change that rule. It is Ian's call and is not proposed here.
  - TestPyPI, a validate-then-drop Maven Central upload and a test NuGet feed. Each needs its own trusted publisher or a stored secret, so each needs an approved environment.
  - A checkpoint tag check in release mode. Issue `2026-10-03-doc-tests-gate-checkpoints-and-releases.md` owns it.
  - Install checks on hosts below each version floor, such as Ruby 3.3 and macOS's Python 3.9. Issue `2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md` owns them, and 0425 checks the diagnostic failure after publication.
  - Staged npm publishing. Ticket 0393 owns it.
  - Issues `2026-10-04-r-install-on-linux-and-before-release.md` and `2026-10-04-two-release-secrets-remain.md` stay separate.

## What Ian can overturn

- No rehearsal check of trusted publishers, and no `rehearse` environment.
- The install check as a separate workflow that `publish` starts, rather than jobs inside `release.yml`.
- Counting a rehearsal whose failed jobs were rerun to success as clean.
