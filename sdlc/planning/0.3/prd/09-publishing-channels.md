---
date: 2026-10-13
status: draft PRD
release: 0.3
source: Ian's goal of traction measured by GitHub stars and YouTube views (0.3 README, 2026-10-13); duckdb/community-extensions#2936 and its review; thinkthen tickets 0415 and 0577; [`sdlc/planning/release-process.md`](../../release-process.md) and the release workflows; the registry pages as they read on 2026-10-13; the marketing launch, playbook and copy notes. The 25 worked questions and their sources are in [09-publishing-channels-questions.md](09-publishing-channels-questions.md)
---

# PRD 09: Publishing channels

ThinkThen ships through 21 channels, and few are finished. PyPI's page opens with "ADR 0047". npm shows no README. The DuckDB listing went out under Ian's name without his knowing, read as machine-written, never said what it costs, and failed DuckDB's build. This PRD picks six front doors, finishes each one, and puts Ian's yes in front of every public word.

## Who and what's in it for them

- **The builder** installs in one line from their usual registry and reads a page that says what ThinkThen is, what it costs and how to start. Proof: a reviewer with no repository context names those three facts from each front-door page alone and installs from it.
- **The agent** runs `uvx thinkthen` or `npx thinkthen` with nothing else to install, and finds the 0.3 MCP server in the MCP Registry. Proof: an agent given only the registry entry configures and calls it.
- **The data person** types `INSTALL thinkthen FROM community` in DuckDB 2.0, and finds a build for their PostgreSQL major. Proof: DuckDB's community CI passes; the install check passes on each major.
- **Ian** gets stars from channels that send people to GitHub, and no public text he has not read. Proof: downloads per channel and GitHub's top referrers, printed at each release.

## Use cases

U1, U2 and U11 from PRD 00 need a one-line install; U6 and U7 need agents to find and install the command. This PRD adds no command and no function. `status` gains one line naming the channel a copy came from.

## The design

**Three tiers,** each with one proof per release, so 21 channels never need 21 campaigns.

| Tier | Channels | What each gets |
| --- | --- | --- |
| Front door | GitHub release with `install.sh`, the Homebrew tap, crates.io, PyPI, npm, DuckDB community extensions | A reviewed README and the install check from the public registry. A failure holds the announcement |
| Listed | R-universe, RubyGems, NuGet, Maven Central, pub.dev, Packagist, Go modules, PostgreSQL and SQLite archives | The shared listing text and the install check. A failure is fixed next release |
| Archive | C++, Swift, Zig, Ada, COBOL, Objective-C | A release archive and the release smoke. Reach comes from articles |

Not in 0.3: CRAN, homebrew-core (75 stars, or 225 when the owner submits), PGXN, PGDG, the Swift Package Index, vcpkg, Conan, Alire, CocoaPods and Bioconductor. The unsigned DuckDB archives go once the community listing serves a version.

**One listing source.** The thinkthen repository keeps one file of public facts: the description, the price sentence, each install line, a short example per language, and links. Every registry README comes from it in its language's idiom. Internal notes leave published READMEs, and a check fails any that names an ADR, a ticket, an `sdlc/` path or a stale version.

**The price sentence, everywhere.** "ThinkThen is free and MIT licensed. It calls the model you choose: hosted models bill you per token through your own key, and local models cost nothing. Nothing is sent to us."

**Ian approves exact words.** An agent drafts any outside text. A cold reviewer checks it first: from the page alone, what is it, what does it cost, how do I install it. Ian reads it in chat, and after his yes the PM posts those exact bytes. A version bump uses a one-line body Ian approves once.

**Outside work closes when the outside item closes.** A ticket stays open until its outside PR merges and the package installs. The PM checks open outside items at each daily status and brings any maintainer comment to Ian the same day. The project tracker is asked to list them itself.

**One version rule.** Support each language version, database major and operating system its own project still supports on release day. Build once where a stable binary interface allows: abi3 and Node-API. DuckDB keeps one C++ build per supported DuckDB version: the DuckDB experiment found that 1.4's stable C interface lacks the call ThinkThen needs to bind a SQL function, so one build cannot cover 1.4, 1.5 and 2.0. ThinkThen supports DuckDB 1.5 and 2.0 and drops 1.4. The call is stable from 1.5.6, so a later single build for 1.5.6 onward stays possible but is untested.

**Provenance.** Every release archive gets a GitHub artifact attestation; PyPI gets attestations through trusted publishing. No reproducible-build claim.

**0.2 must:** pass DuckDB community CI on Linux x86-64 and ARM64 with Ian-approved words (ticket 0577). DuckDB lists 2.0.0 for 2026-10-21, tentatively, and builds `ref` only against the latest stable release, so 2.0 is the version that must pass. Split public READMEs from internal notes on the front-door channels and RubyGems, add the price sentence, and watch outside PRs by hand.

**0.3 adds:** the tiers, the listing source and its check; the command inside the PyPI and npm packages, which the packaging experiment showed works; the MCP Registry entry after the one-tool MCP; every supported PostgreSQL major and a container image shared with the proxy (PRD 05 Q21); dropping the unsigned DuckDB archives; channel numbers per release.

## What must be true

| Risk | What retires it |
| --- | --- |
| Public text still reads as machine-written | The cold read passes before Ian sees it |
| The listing breaks when 2.0.0 ships | CI green on `ref` and `ref_next`, reproduced locally with DuckDB's own toolchain and vcpkg before each descriptor change |
| The command in a package breaks the library | Experiment run on Linux: `uvx thinkthen` and `npx thinkthen` print the version and answer an MCP start request offline, and both libraries still pass their recorded checks. The Python download grows from 5.5 to 11.2 MB and npm's from 2.5 to 8.2 MB; each install grows about 15 MB. The release check repeats this on every platform and the current version |
| Channels cost more than they return | A Listed channel near zero for two releases is a drop candidate |
| An outside PR goes unwatched again | The project tracker lists open outside items |

## Out of 0.3

The channels left out above. Reproducible-build claims. SQLite inside pip or npm. A Swift mirror repository.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 10, 11 and 12. Settled: DuckDB 1.4 is dropped, because the experiment showed it cannot share a build.
