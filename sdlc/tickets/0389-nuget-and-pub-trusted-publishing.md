# 0389: NuGet and pub.dev publish by trusted publishing

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-2. Branch `ticket/0389-trusted-publishing-nuget-pub`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Parent: ticket 0128, the two docs team asks of 2026-10-01 that its Phase 3 notes name.

Milestone: 0.1

## Outcome

1. The `nuget` job in `.github/workflows/release.yml` reads no secret. It holds `id-token: write`, logs in through the official `NuGet/login` action pinned by commit, and passes that action's short-lived key to `release-workflow nuget-push`. The action's `user` input reads the `release` environment variable `NUGET_USER`, which is a nuget.org profile name and no secret.
2. The `pub` job reads no secret and uses no Google Cloud. It holds `id-token: write`, installs Dart 3.13.4 through the official `dart-lang/setup-dart` action pinned by commit, which mints the pub.dev token from the run's OIDC token, and runs `release-workflow pub-publish`. The Google auth step and both `PUB_DEV_*` variables are gone.
3. Both jobs keep their place and guards: release mode only, after `draft`, environment `release` with Ian's approval, and the `RELEASE_ARMED` first step. `release.yml` still starts only from `workflow_dispatch`. No new workflow and no tag-push trigger appear.
4. `sdlc/scripts/workflows` requires each trusted-publishing job (`crates`, `pypi`, `npm`, `rubygems`, `nuget`, `pub`) to hold `id-token: write` and to read no secret. Two planted cases prove the rule.
5. Ticket 0128's "Ian's setup list" says in plain steps what to set on nuget.org, on pub.dev and in the `release` environment. For NuGet: check the existing policy and add the `release` environment variable `NUGET_USER` = `imaurer`. For pub.dev: a placeholder first upload, then on the package Admin tab repository `botassembly/thinkthen`, tag pattern `v{{version}}`, "Enable publishing from `workflow_dispatch` events" ticked, "push events" unticked, and "Require GitHub Actions environment" ticked with `release`. pub.dev rejects any token whose ref is not `refs/tags/` plus the tag pattern with the version, so a wrong pattern fails the release.
6. Ticket 0355's setup table drops `NUGET_API_KEY`, the `PUB_DEV_*` variables and the Google Cloud row. Its design lines for `nuget` and `pub` and its pub.dev paragraph stay as history with a pointer to this ticket.

## Evidence

- Starts from: the docs team's two asks of 2026-10-01 and the current workflow.
  - The NuGet ask: Ian made a nuget.org trusted publishing policy on 2026-10-01 for owner `botassembly`, repository `botassembly/thinkthen`, workflow `release.yml`, environment `release`. nuget.org shows it active with the repository ids, so it is past the seven-day pending window. Ian's nuget.org profile name is `imaurer`.
  - The pub.dev ask: Ian ruled out Google Cloud on 2026-10-01. The ask assumes pub.dev accepts only a tag push and proposes a separate tag-push job.
  - [NuGet trusted publishing](https://learn.microsoft.com/en-us/nuget/nuget-org/trusted-publishing): the job needs `id-token: write`; `NuGet/login` takes the policy creator's profile name as `user` and outputs `NUGET_API_KEY`, valid for one hour. `NuGet/login` tag `v1` and `v1.2.0` both resolve to commit `8d196754b4036150537f80ac539e15c2f1028841`; its `src/index.ts` requires `user` and fails when OIDC is unavailable.
  - [pub.dev automated publishing](https://dart.dev/tools/pub/automated-publishing) says pub.dev publishes only from a run started by a tag push. That page is older than the server. pub.dev's server checks the token in `app/lib/package/backend.dart`: the event must be `push` or `workflow_dispatch`, each allowed by its own checkbox on the package admin page; the ref type must be `tag`; the ref must equal `refs/tags/` plus the tag pattern with the version; and a required environment must match. [pub-dev PR 7766](https://github.com/dart-lang/pub-dev/pull/7766), merged 2024-05-30, added the `workflow_dispatch` option and the admin checkbox "Enable publishing from `workflow_dispatch` events", closing [issue 7177](https://github.com/dart-lang/pub-dev/issues/7177). A release-mode run dispatched from tag `v0.1.0` carries event `workflow_dispatch`, ref `refs/tags/v0.1.0`, ref type `tag` and environment `release`, so it passes once that box is ticked.
  - `dart-lang/setup-dart` `v1.8.1` resolves to commit `6afc89df92d6eb3834022f73cd65adc8cdfcb92d`. Its `lib/main.dart` requests an OIDC token for audience `https://pub.dev` when the job may mint one, exports it as `PUB_TOKEN`, and runs `dart pub token add https://pub.dev --env-var PUB_TOKEN`. That is the step the Google route did by hand. It mints the token silently and only when `ACTIONS_ID_TOKEN_REQUEST_URL` is set. A job without `id-token: write` skips the token with no error and fails later inside `dart pub publish`, which is why the workflow check pins `id-token: write` on `pub`.
  - Ticket 0128's outward-step rule and `workflows` allow only `workflow_dispatch`. A tag-push workflow would break both and would publish outside the dispatched run's `draft` and approvals.
- Keeps: every other job and guard.
  - The `release` environment, its approvals, `RELEASE_ARMED`, release mode, `needs: [resolve, draft]`, and the `publish` job's needs.
  - `release-registry.py`: `nuget-push` still reads the key from `NUGET_API_KEY`, now the one-hour key; `pub-publish` still checks the pubspec version, refuses a version pub.dev holds, and publishes the same temporary copy with `dart pub publish --force`. The `registries` job's pub dry run keeps the hash-checked Dart 3.13.4.
  - The other registry jobs, the tap key and the Maven secrets.
- Changes: the two jobs, one workflow rule and the setup records.
  - `release.yml`: the `nuget` job gains `id-token: write` and a `NuGet/login` step just before the push. The `pub` job drops `dart-tools` and the Google auth step, and gains `setup-dart` with `sdk: '3.13.4'`. The `pub` job thereby loses the SHA-256 check `dart-tools` makes on the Dart download. `setup-dart` fetches the same version from the same Dart archive over HTTPS without a hash. The uploaded bytes are still the `registry-packages` artifact that the `registries` job dry-ran with the checked Dart, so Dart here is only the uploader.
  - `sdlc/scripts/workflows`: the trusted-publishing rule, the fixture's registry jobs with `id-token: write`, the fixture's `nuget` login, and two planted cases.
  - Ticket 0128: the setup list, the workflow check table, and the Phase 3 note on the two asks. Ticket 0355: its setup table and its pub.dev paragraph. `sdlc/planning/milestones.md`: the blocker line.
  - This ticket.
- Proof: local checks only; no dispatch.
  - `python3 sdlc/scripts/workflows --self-test` and `python3 sdlc/scripts/workflows`. The planted `nuget-secret` case puts `secrets.NUGET_API_KEY` back and fails; the planted `pub-no-oidc` case drops `id-token: write` from `pub` and fails.
  - `python3 sdlc/scripts/release-registry-self-test.py`, `release-managed-pair-self-test.py`, `release-language-tools-self-test.py` and `release-archive-self-test.py`.
  - `sdlc/scripts/lint` with the private-names list, and `python3 sdlc/scripts/tickets`.
  - `git grep` finds no `secrets.NUGET_API_KEY`, `google-github-actions` or `PUB_DEV_` in `.github`.
- Defers: the runner proof and the registry settings.
  - The first real proof is the release run itself. A rehearsal skips both jobs. Ticket 0128 Phase 4 restarts at step 2, so one more rehearsal from `release/0.1` follows this change.
  - The nuget.org variable and the pub.dev settings are Ian's or the docs team's, in ticket 0128's setup list.
  - The first `thinkthen_dart` upload by hand, which pub.dev needs before it shows automated publishing.

## What Ian can overturn

- The pub.dev route: a release-mode dispatch from the `v*` tag, with pub.dev's `workflow_dispatch` box ticked. The docs team's tag-push job remains the alternative; it would need Ian to allow a second trigger.
- `NUGET_USER` as a `release` environment variable rather than the name written in the workflow.
- `setup-dart` without a hash check in the `pub` job. The alternative keeps `dart-tools` and fetches the pub.dev token with a short `curl` to the runner's OIDC endpoint, adding no action.

## What the build taught us

- dart.dev's automated-publishing page still says pub.dev accepts only a tag push. pub.dev's server has accepted a `workflow_dispatch` run on a tag ref since 2024, behind its own admin checkbox. The server source settled the question, and the docs team's separate tag-push job proved unnecessary. Ticket 0355 had taken the page at its word and built a Google Cloud route for the same reason.
- The fixture in `sdlc/scripts/workflows` gave `crates`, `npm` and `rubygems` no `id-token: write`, so a rule over every trusted-publishing job needed the fixture to match the real workflow first.
- Proof run here: `workflows --self-test` 70 of 70, `workflows`, `release-registry-self-test.py`, `release-managed-pair-self-test.py`, `release-language-tools-self-test.py`, `release-archive-self-test.py`, `tickets`, and `lint` with the private-names list. Lint's "Killed" line comes from the `time-limit` self-test in `surfaces`, which kills its own planted process. No workflow ran; the first runner proof is the release run, after one more rehearsal from `release/0.1`.

## Review

- Ticket review: four findings (the lost Dart hash check, the exact pub.dev settings, ticket 0355's remaining Google lines, and the silent token step); fixed, then ACCEPT.
- Code review: two findings (the milestone line spoke of the cherry-pick as done, and 0355's lesson lacked a pointer); fixed.
