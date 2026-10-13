# 0578: Give every published package a public README

Status: OPEN.

Milestone: 0.2

## Outcome

Each package registry page that ships with 0.2 shows a short README written for a stranger: what ThinkThen is, one example in that language, what it costs, and how to install it. No registry page names an ADR, a ticket, an `sdlc/` path, an internal crate or a stale version. A release check fails the release if one does.

## Evidence

- Starts from: the marketing lead's live check of the registries on 2026-10-13.
  - PyPI thinkthen 0.1.2 opens its description with "The binding is the unpublished crate thinkthen-python in its own Cargo workspace (ADR 0047). The design page is sdlc/planning/libraries/python.md."
  - npm thinkthen 0.1.2 shows "No README data found!".
  - RubyGems holds only the 0.0.1 name placeholder.
  - The DuckDB maintainer flagged the same problem on the community listing, which 0577 covers.
- Keeps: each library's in-repo developer README, if it differs from the published one.
- Changes:
  - Write one public README per published package: PyPI, npm, RubyGems, crates.io, Maven Central, NuGet, Go, Swift, Packagist, pub.dev, CRAN or r-universe, and the database extensions, for whichever of these 0.2 publishes. Each says what ThinkThen is, shows one example, states that the library is free and MIT licensed while hosted models charge per token and local models are free, and shows the install line.
  - Point each package's metadata at its public README, so the registry renders it.
  - Add a release check that reads every package's published README and fails on an ADR number, a ticket number, an `sdlc/` path, an internal crate or project name, or a version other than the one being released.
  - The PM sends the texts to the marketing lead for Ian's approval before anything publishes.
- Proof:
  - The release check passes on the candidate and fails on a fixture README that names an ADR.
  - Each built package's rendered description matches its approved README.
  - Ian approves the texts.
- Defers: nothing.

## Progress

PM amendment, 2026-10-13. Review of the drafts at 53f10e414 before they go to marketing. Revise every public README this way. Capitalize the language name in each title, as in "ThinkThen for Python". Delete "Failed calls remain separate from answers." Say in one sentence why a developer would use ThinkThen over calling a model directly: answers code can branch on, each question paid once through the cache, and offline replay for tests. Name Jev, SystemOne, OpenAI and local models as backends, per Ian's 2026-10-13 search ruling. Replace "Configure a model backend" with the one line that sets the key, such as `export THINKTHEN_API_KEY=...`. Show one typed answer read in each example, never `results[0]`. Keep each page under 40 lines.
