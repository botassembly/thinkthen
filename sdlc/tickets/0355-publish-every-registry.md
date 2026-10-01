# 0355: Publish jobs for every registry

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-2, ruling 15.

## Outcome

`release.yml` can publish every language package that has a registry, and a rehearsal proves each one without a secret. A `rehearse` run builds, packs and checks the NuGet package, the Maven Central bundle (signed with a throwaway key), the pub.dev package (`dart pub publish --dry-run`), the Packagist manifest and the Go module path. A `release` run, after Ian's approvals, pushes the NuGet package, uploads the Maven Central bundle signed with the real key, publishes to pub.dev, and tags the Go module. Packagist and R-universe need no job: each reads the published repository.

This is pre-0.1 item 11 in `sdlc/planning/grading-2026-09-30/README.md` and item 5 of `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`.

## Evidence

- Starts from: ticket 0128 phase 3a's `release.yml`, with its rules in `sdlc/scripts/workflows`: dispatch only, every action pinned, outward jobs in environment `release` behind the `RELEASE_ARMED` first step, contents write only in `draft` and `publish`. Tickets 0261 to 0273 pack `thinkthen-csharp` (the `Botassembly.ThinkThen` nupkg), `thinkthen-jvm` (the POM and three JARs) and `thinkthen-dart` (the pub source) on the x86 Linux build, and the smoke and draft gates check their contents against the resolved commit. Ian's registrations of 2026-09-28 chose the NuGet `Botassembly.` prefix with a `NUGET_API_KEY` secret, the Maven namespace `io.github.botassembly` with `MAVEN_CENTRAL_*` secrets and a GPG key, Packagist submission of this repository with no secret, and pub.dev trusted publishing. The R check already proves the R-universe shape: the package folder built alone, against the packed crate (`libraries/r/check.sh`, ticket 0128). Three rehearsal dispatches on 2026-09-30 never reached a language package; the third stopped on account billing.
- Keeps: every existing job, gate and rule. No new job holds a secret or `id-token` outside environment `release`. Every new outward job requires release mode and checks `RELEASE_ARMED` first. Each registry publish fails without change when the registry already holds the version. Rehearse mode references no secret and no environment.
- Changes: listed below.
- Proof: `workflows --self-test` with new cases for the job graph, the armed first step of each new outward job, and the registry job's checks. A new fixture test, `release-registry-self-test.py`, runs each new `release-workflow` operation on planted archives: the happy path, a wrong NuGet id or version, a wrong POM identity, a bad checksum, a Packagist manifest that drifts from `libraries/php/composer.json`, a Go module path that differs, rehearsal signing refusing when a key variable is set, release signing and uploads refusing without their secrets, and the Go tag refusing a tag at another commit. One manual `dart pub publish --dry-run` of the packed Dart source on this host. `release-pack jvm` and the JVM check after the POM change. `policy.py`, `tickets`, lint in a clean checkout.
- Defers: the first real proof is the next rehearsal, which waits for GitHub billing. `thinkthen_flutter` stays unpublished: it depends on `thinkthen_dart` by path, and its dry run cannot resolve until `thinkthen_dart` is on pub.dev. A Maven javadoc JAR with generated pages: the bundle ships a README-only javadoc JAR, which Central accepts. A smaller Composer download: Packagist serves this whole repository's zip, about 19 MB; a split repository would need a deploy key. Kotlin and Scala JARs ship as classifiers of one artifact and declare no Kotlin or Scala runtime dependency.

## Changes

`release.yml`:

- New job `registries`, needs `resolve` and `build`, both modes, no environment, read-only. It checks out the resolved commit, downloads the x86 Linux platform files, installs the pinned Dart, and runs `release-workflow registry-pack`, `maven-sign rehearse` and `pub-dry-run`. It uploads `registry-packages`.
- `draft` also needs `registries`, so a registry package that fails its check stops the draft.
- New outward jobs, each `needs: [resolve, draft]`, `if: inputs.mode == 'release'`, environment `release`, with the armed first step:
  - `nuget`: pushes the nupkg with `NUGET_API_KEY` through NuGet's push endpoint. A 409 for an existing version fails the job.
  - `maven`: refuses when the version is already on `repo1.maven.org`, signs the bundle with `MAVEN_CENTRAL_GPG_PRIVATE_KEY` and `MAVEN_CENTRAL_GPG_PASSPHRASE`, uploads it to the Central Portal with `MAVEN_CENTRAL_USERNAME` and `MAVEN_CENTRAL_PASSWORD` as `AUTOMATIC`, and waits for `PUBLISHED` or fails on `FAILED`.
  - `pub`: gets a Google identity token for `https://pub.dev` through `google-github-actions/auth` with `id-token: write`, adds it with `dart pub token add`, and runs `dart pub publish --force` in the packed package folder.
- `publish` also needs `nuget`, `maven` and `pub`. Before it turns the draft into the release, it creates the Go module tag `libraries/go/v<version>` at the resolved commit, because the module lives in a subfolder and proxy.golang.org reads only a tag with that prefix. It refuses a tag at another commit and keeps one at the same commit.

`sdlc/scripts/release-workflow` gains these operations:

- `registry-pack PLATFORM_DIR OUT SHA`: checks the checksums of the C#, JVM and Dart archives, then writes `nuget/Botassembly.ThinkThen.<v>.nupkg`, the Maven layout under `maven/io/github/botassembly/thinkthen-jvm/<v>/` (POM, the door JAR as the main JAR, `kotlin` and `scala` classifier JARs, a sources JAR from the resolved checkout, a README-only javadoc JAR, and `.md5` and `.sha1` beside each), and `pub/thinkthen_dart/` without the lock or the package-inputs file. It checks the NuGet id and version, the POM identity and Central's required POM fields, the pub name and version, the Packagist manifest, and the Go module path.
- `maven-sign DIR rehearse|release [OUT]`: rehearse refuses if a Maven key variable is set, signs a scratch copy with a throwaway key, verifies every signature and zips the bundle. Release imports the real key, signs into OUT and zips it.
- `maven-upload BUNDLE`, `nuget-push NUPKG`, `pub-dry-run DIR`, and the Go tag step inside `publish`.

Package files:

- `libraries/jvm/pom.xml`: `jar` packaging and the fields Central requires: `url`, `developers` and `scm`.
- A root `composer.json`, which Packagist reads. It names `botassembly/thinkthen`, the same requirements as `libraries/php/composer.json`, and autoloads `libraries/php/autoload.php`. `registry-pack` refuses drift between the two.
- `sdlc/scripts/release-pack` and the managed pair checks keep reading the POM by identity, so their outputs do not change beyond the POM bytes.

## Needs from the release setup

The documentation team owns these accounts and settings. Ian decides when Actions run. Each item names what a job reads.

| Job | Reads | Kind |
| --- | --- | --- |
| `nuget` | `NUGET_API_KEY`, scoped to push `Botassembly.ThinkThen` | `release` environment secret |
| `maven` | `MAVEN_CENTRAL_USERNAME`, `MAVEN_CENTRAL_PASSWORD` (the Central Portal user token pair) | `release` environment secrets |
| `maven` | `MAVEN_CENTRAL_GPG_PRIVATE_KEY` (ASCII-armored), `MAVEN_CENTRAL_GPG_PASSPHRASE`; the public key on a public keyserver | `release` environment secrets |
| `maven` | Namespace `io.github.botassembly` verified in the Central Portal | account |
| `pub` | `PUB_DEV_WORKLOAD_IDENTITY_PROVIDER`, `PUB_DEV_SERVICE_ACCOUNT` | `release` environment variables |
| `pub` | pub.dev package admin: publishing with that Google Cloud service account enabled | account |
| none | Packagist: this repository submitted; the webhook reads each `v*` tag | account |
| none | R-universe: `botassembly/botassembly.r-universe.dev` with `packages.json` naming `thinkthen` at `subdir` `libraries/r/thinkthen`, and the R-universe GitHub app | account |
| `publish` | nothing new; the Go tag uses the job's `GITHUB_TOKEN` | none |

pub.dev's GitHub trusted publishing accepts only a run started by a tag push. Ian ruled on 2026-09-22 that only `workflow_dispatch` starts a workflow. The `pub` job therefore uses pub.dev's Google Cloud service account route, which a dispatched run can use and which stores no secret. Ian can overturn this by allowing one tag-push workflow for pub.dev.

## What Ian can overturn

- The pub.dev route above.
- Maven `AUTOMATIC` publishing after the approved job; `USER_MANAGED` would add a Publish click in the Central Portal.
- The Go tag created by `publish` rather than by hand.
- Kotlin and Scala as classifiers of `thinkthen-jvm` rather than three artifacts.

## What the build taught us

To be written before landing.
