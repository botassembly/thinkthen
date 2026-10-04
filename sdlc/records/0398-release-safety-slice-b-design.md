# 0398 slice B: Install-check design

Date: 2026-10-04
Status: fresh slice B ticket review ACCEPT at `58aee391b351297568aa68ca4ac8f0fdb19fe414`; independent build in progress
Starting revision: `ff047d8c50ce39ed9ab0acd22695c4960a963d38`
Ticket: `sdlc/tickets/0398-release-safety.md`
Lane: `claude-2`

## Assignment

Create the separate manually dispatched install-check workflow and its channel installer, consumer checks and offline edge table. Keep the ticket's eighteen channels and forty-seven Unix runner combinations. The fifth runner is `macos-26`, confirmed in the [official runner table](https://github.com/actions/runner-images). Keep all existing Linux, Linux ARM, macOS 15 ARM and macOS 15 Intel rows. Tool versions follow the ticket. Setup actions use existing commit pins or separately verified official action commits.

The R job uses `rocker/r-ver:4.6.1@sha256:268e4c559c905a78519793f1a8c9ec5c8051a6ac98a16cbf489135ccdad330bb`. [Docker Hub](https://hub.docker.com/layers/rocker/r-ver/4.6.1/images/sha256-268e4c559c905a78519793f1a8c9ec5c8051a6ac98a16cbf489135ccdad330bb) identifies this as the Linux amd64 manifest with R 4.6.1. No image was pulled during preparation.

## Install and proof boundaries

Each channel gets an owned scratch home and clean consumer project. Validate the requested version before any installation. Fetch the requested release's first-run sample and checksums through the existing retry helper. Read installed package metadata or the runtime version, then replay the exact recorded question and text with no key. Commands must return Boolean true. Library and SQL checks must additionally return the integer `requests_sent` value 0. The shared checker rejects missing fields, false, null, coerced truthy strings, wrong versions, placeholders and nonzero request counts. No provider credentials or user configuration enter the check environment.

Homebrew selects the requested formula from the full public tap Git history, copies it into owned scratch and installs that formula through a local Homebrew tap with automatic updates disabled. The current formula never substitutes for an absent historical version. The installed binary supplies the version and replay proof. Offline current, historical and missing-version fixtures prove selection and install routing. This follows [Homebrew's historical formula and local tap guidance](https://github.com/Homebrew/brew/blob/main/docs/How-to-Create-and-Maintain-a-Tap.md). No tap is published.

R-universe keeps only the current indexed package, as its [reproducibility documentation](https://docs.r-universe.dev/install/reproducibility.html) states. Read the binary index before installing. An absent package or listed version older than wanted gets the ticket's retry sentence. A newer listed version gets `R-universe no longer offers thinkthen <requested>; its binary index lists <listed>`. It offers no retry advice. An exact match installs only the resolute Linux binary. Source-only availability refuses. No source fallback or Rust installation is allowed. Historical manual dispatch remains useful for the other channels but cannot promise an archived R-universe package.

SQLite supplies release-asset identity rather than a runtime version. Fetch the exact requested SQLite release archive and its checksum, verify the checksum, and load the exact extension extracted from it. Replay must return true and zero requests. The receipt labels the version proof `release-asset identity`. A C header belongs to another artifact and supplies no SQLite version proof. Offline archive cases prove identity, checksum rejection and exact extracted load path.

The Go proxy and Packagist check their indexes before installing. Missing requested versions get the retry sentence. Every scratch cleanup removes only a path created by this run. No force installation or unowned deletion is allowed.

## Order and review

The first fresh slice B ticket review returned the three findings addressed above: historical Homebrew selection, superseded R-universe versions and SQLite proof qualification. The fresh amendment review returned ACCEPT at `58aee391b351297568aa68ca4ac8f0fdb19fe414` before product code started.

After ACCEPT, build independent new files and their offline tests. Shared release workflow, release helper and validator edits wait for Windows slice A to land or for the coordinator's explicit handoff. Then rebase, add the final publish dispatch step and its narrowly scoped permission, add workflow refusal plants and update release records. Retain the eighteen-job release graph and Windows work. The completed main baseline on `e760432c8` remains applicable; no full checkpoint repeats for design preparation.

No workflow dispatch, registry installation, Docker pull, provider check or credential read ran during preparation. Only local source reads and official read-only references were used. The eventual manual install check and release runner proof still need Ian's approval.
