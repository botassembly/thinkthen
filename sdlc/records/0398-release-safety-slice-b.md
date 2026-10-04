# 0398 slice B: Published install checks

Date: 2026-10-04
Status: independent build in progress; shared release integration and fresh code review pending
Ticket: `sdlc/tickets/0398-release-safety.md`
Starting revision: `58aee391b351297568aa68ca4ac8f0fdb19fe414`

## Review and implementation

The fresh slice B ticket review accepted amendment `58aee391b351297568aa68ca4ac8f0fdb19fe414`. Its prior findings required historical Homebrew selection, distinct older and superseded R-universe indexes, and qualified SQLite release-asset identity. The implementation retains those boundaries.

The independent workflow retains eighteen channels over forty-seven Unix runner combinations. Each channel installs its requested public package or release archive, checks installed metadata or version, and asks the recorded question with strict replay. The consumer gets a new scratch home and no provider key or user configuration. The result check requires Boolean true and integer zero requests. Scratch cleanup belongs to the current check alone.

Homebrew reads the full public tap history and installs the selected historical formula through an owned local tap with automatic updates disabled. R-universe checks the resolute binary index before installation. An older or absent index reports the retry sentence. A newer version names the requested and listed versions and reports that the registry no longer offers the requested version. Source-only availability and archives without Linux R 4.6 binary metadata refuse without installing Rust or compiling source. SQLite checks the exact named archive and checksum, loads its exact extracted extension, and labels its proof `release-asset identity`.

## Focused offline proof

The independent edge table passes nine test methods. It covers all eighteen channels' answer and version contract, wrong versions, placeholders, false, null, missing and nonzero requests, invalid version rejection before installer imports, delayed indexes, superseded R versions, source-only R refusal, current and historical Homebrew selection, absent historical formulas, archive checksum and path escape refusals, exact SQLite load routing, clean environments and all forty-seven workflow cells. The fixture installers run no external package command.

The real workflow validator, ticket check, catalog check and child-environment check pass. C, Node, Ruby and PHP consumer syntax checks pass. The Rust consumer compiles against the local public API through offline Cargo in a lane lock and a user systemd scope with 6 GiB high and 8 GiB maximum memory. These checks do not establish public registry or hosted runner acceptance.

## Verified tool sources

The existing pins supply Rust 1.95.0, Python 3.13, uv 0.9.17, Node 22.22.3, Ruby 3.4.11, Go 1.27.1, Dart 3.13.4 and DuckDB 1.5.5. Read-only official Git refs verified the setup-dotnet v5 and setup-java v5 commit pins. .NET 8, Java 21 and Maven 3 retain the ticket's major version bounds.

The [official PGDG noble package index](https://apt.postgresql.org/pub/repos/apt/dists/noble-pgdg/main/binary-amd64/Packages.gz) supplied PostgreSQL 16.15 server and client package names and SHA-256 values. The runtime unpacks those packages into the check's own tree and starts a private socket-only cluster. The [official SQLite download page](https://www.sqlite.org/download.html) supplied the Linux x64 3.53.4 tool archive and SHA3-256. The ticket permits SQLite 3.50.4 or newer; the checked runtime pins 3.53.4. Preparation read metadata only and did not download or install these runtime packages. R uses the previously verified R 4.6.1 amd64 image digest recorded in the accepted design.

## What the build taught us

The CLI refuses a cache option beside explicit replay, so the consumer supplies replay alone. Python facts use a read-only mapping, Ruby facts use symbol keys, and Rust facts use accessor methods. The JVM archive has no implementation-version manifest; its actual resolved Maven POM supplies the installed package version. R installs the exact downloaded binary archive through R CMD INSTALL after checking its binary metadata. It never asks install.packages to select a source package.

## Pending work and authority

Shared edits to the release workflow, release helper and workflow validator wait for Windows slice A to land or the coordinator's explicit handoff. The publish dispatch, dedicated workflow guard plants and gate registration remain pending. The eighteen-job release graph remains unchanged in this candidate.

No workflow dispatch, public package installation, image pull, provider call, credential read or publication ran. Hosted runner proof and the eventual real install-check dispatch remain pending Ian's approval. No full checkpoint was repeated. The coordinator recorded the main baseline's complete pass separately.
