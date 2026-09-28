# Release and install for 0.1

Status: Open.

## Ian's rulings, 2026-09-25

- "I definitely want to move publishing to GitHub Actions as an intentional release process, and we'll try to do that with 0.1." Release publishing runs on GitHub Actions with trusted publishing to crates.io, PyPI, npm, and RubyGems. This overturns the 2026-09-22 pause for release jobs only. The gate stays run by hand. The local registry tokens are deleted once trusted publishing works.
- The Homebrew tap lives under `botassembly`, not `genomoncology`. The site's install line changes to match.
- R ships through R-universe. Ian: "if R is easier, our universe is easier, that's fine."
- No separate Rust Polars crate. Ian asked why ThinkThen would not carry Polars itself. The Rust Polars door moves into the `thinkthen` crate behind an optional `polars` feature, the way Python ships `thinkthen[polars]`. Users who skip the feature never compile Polars. This amends ADR 0047's `thinkthen-polars` binding at `libraries/polars`. Ian can overturn this reading. Settled by ticket 0130: the door is the `polars` feature of `thinkthen`, and no `thinkthen-polars` crate exists to publish.

No release ticket exists. The release build is queue item 6 of `sdlc/planning/one-line-plan-2026-09-24.md`, and the plan says "Nothing tickets this today." Seven surfaces landed on main on 2026-09-25, so the release is now the biggest gap before 0.1. This issue merges five files into one list for that ticket: `2026-09-25-release-and-install-for-0-1.md`, `2026-09-25-release-and-install-for-0-1.md`, `2026-09-25-release-and-install-for-0-1.md`, `2026-09-25-release-and-install-for-0-1.md`, and the release, install, and README parts of `2026-09-25-release-and-install-for-0-1.md`. Each item was checked against main on 2026-09-25.

Ian's rulings that govern this work:

- 2026-09-20: "first version release will be 0.1 across all libs/exts. until then number 0.0.1 thru 0.0.9999 as necessary."
- 2026-09-20: one crate named `thinkthen`, and no `thinkthen-cli`. The install path is a `curl` installer that downloads a release from GitHub. The repository stays at `botassembly/thinkthen`.
- 2026-09-21: "one Homebrew line and one download script for the first release, and a public GitHub repository for the Homebrew formula is approved."
- 2026-09-22: "no GitHub Actions for now. All testing runs on the local machines."
- 2026-09-24: "Package names, the tap, the site, and papers are Ian's. Claude's job is the code: the main line and every surface."
- 2026-09-24 (ticket 0111): the release checklist counts a surface check that reports "not run" as a failure.

## 1. Release binaries per platform

Today: no workflow, script, or ticket builds a release. `.github/workflows/` holds `gate.yml` and `pages.yml` alone. `crates/thinkthen/Cargo.toml` sets `publish = false`. `sdlc/planning/libraries/command.md` names `cargo-dist` and these targets: static `musl` Linux on x86-64 and aarch64, macOS on both chips, and Windows on x86-64. The 2026-09-21 install issue recommended Linux and macOS on both chip types, with Windows waiting for demand.

Asked: one archive per platform holding the binary alone. A checksum file sits beside every archive, made with `shasum` on macOS runners. Every release job builds the tagged commit and never a moving `main`. The BioMCP checklist recommends starting from BioMCP's `release.yml` and adding the `musl` targets. It weighs `cargo-dist` after the launch, because a new release tool in the shipping cycle is the risk BioMCP hit. Decided by default: start from BioMCP's workflow, and ship Linux and macOS on both chips with no Windows build. Ian can overturn either.

Done when: a dry release of the tagged commit uploads four archives and four checksum files to a draft GitHub release.

## 2. The download script

Today: no `install.sh` exists in the repository or in `site/public/`. `site/src/data/catalog.mjs` shows `curl -fsSL https://thinkthen.dev/install.sh | sh` as "Coming with 0.1". `site/src/pages/install.astro` says the script "will be served from this site at `/install.sh`" and "matches the repository's copy byte for byte."

Asked: copy BioMCP's `install.sh` almost whole. It refuses a missing or wrong checksum. It runs the staged binary's `version` and compares it to the requested version before it replaces the old binary. It resolves "latest" by skipping a release whose archive for this platform is not uploaded yet. It falls back to the redirect address when the API or `jq` is missing. It writes a two-step receipt, pending then installed. It refuses a symlinked install folder or binary. It prints the `PATH` line and never edits a shell profile. It runs under `set -euo pipefail` with a cleanup trap. A check compares the root `install.sh` with the site's copy byte for byte. The install page gains one line that names the file to delete for an uninstall.

Done when: the script from `thinkthen.dev/install.sh` installs the draft release into a clean prefix on Linux and macOS, and the byte-for-byte check passes.

## 3. Homebrew

Today: no tap repository or formula exists. The site shows `brew install genomoncology/thinkthen/thinkthen`, which points at a GenomOncology tap while the code lives at `botassembly/thinkthen`. BioMCP's tap is `genomoncology/homebrew-biomcp`.

Asked: one public tap repository with one formula per Ian's 2026-09-21 ruling. The release job updates the formula's version and checksums. The tap and its owner are Ian's under the 2026-09-24 ruling.

Done when: `brew install` of the site's line installs the draft release on macOS on both chips, and the formula update runs from the release job.

## 4. crates.io with trusted publishing

Today: the name `thinkthen` is claimed on crates.io at 0.0.1 as a placeholder published by hand. Trusted publishing from GitHub Actions is not set up. A local token exists. `crates/thinkthen/Cargo.toml` sets `publish = false`. Ticket 0120 left the Rust Polars crate at `publish = false` and names the release ticket as the place that brings Ian its name. The recommendation is `thinkthen-polars`.

Asked: publish through GitHub's OIDC with no stored token. The 2026-09-20 checklist left crates.io unchecked for trusted publishing, so the ticket confirms support first. `cargo install thinkthen` then works as a side effect and needs no promotion. The local token gets deleted once trusted publishing works.

Done when: the release job publishes `thinkthen` 0.1.0 through trusted publishing, and the local crates.io token is deleted.

## 5. PyPI with trusted publishing

Today: the name `thinkthen` is claimed on PyPI at 0.0.1 as a placeholder published by hand. Trusted publishing is not set up. A local token exists. `libraries/python/pyproject.toml` carries version 0.0.1. The site shows `pip install thinkthen` and `pip install thinkthen[polars]`.

Asked: publish wheels through a `pypi` environment and GitHub's OIDC, as BioMCP does, with no stored token. The local token gets deleted once trusted publishing works.

Done when: the release job publishes the wheels for every platform in item 1 through trusted publishing, and the local PyPI token is deleted.

## 6. npm with trusted publishing

Today: the name `thinkthen` is claimed on npm at 0.0.1 as a placeholder published by hand. Trusted publishing is not set up. A local token exists. `libraries/typescript/package.json` carries version 0.0.1.

Asked: publish through GitHub's OIDC with no stored token. The local token gets deleted once trusted publishing works.

Done when: the release job publishes the npm package with its native parts for every platform in item 1 through trusted publishing, and the local npm token is deleted.

## 7. RubyGems with trusted publishing

Today: the name `thinkthen` is claimed on RubyGems at 0.0.1 as a placeholder published by hand. Trusted publishing is not set up. A local token exists. `libraries/ruby/thinkthen.gemspec` reads its version from the crate.

Asked: publish through GitHub's OIDC with no stored token. The 2026-09-20 checklist left RubyGems unchecked for trusted publishing, so the ticket confirms support first. The local token gets deleted once trusted publishing works. The gem needs the Mac build in item 9.

Done when: the release job publishes the gem for Linux and macOS through trusted publishing, and the local RubyGems token is deleted.

## 8. The other surfaces' downloads

Today: the site lists channels with no registry claimed. R shows `install.packages("thinkthen")`, and ticket 0108 defers CRAN, R-universe, and prebuilt binaries to the release ticket. The C surface shows one archive per platform with the header, both libraries, and a `.pc` file. DuckDB loads unsigned. SQLite loads a file. PostgreSQL shows `CREATE EXTENSION thinkthen;`. Ticket 0111 left `package.sh` and its tarball at the `surfaces-wave7` tag and moved them here. It also moved the `package.sh` halves of error-index rows R3-32 and R4-10: GNU `sed -i` and hard-coded `.so` names.

Asked: one download per surface that a user can install on Linux and macOS. The C archive stays separate from the command archive per the 2026-09-21 recommendation.

Done when: each surface has a named channel and a release asset or registry entry, and each installs from that channel on a clean machine.

## 9. The Ruby Mac build and other Mac builds

Today: ticket 0112 ports Ruby for Linux alone. `libraries/ruby/check.sh` line 79 exits 77 with "Linux only at 0.1" on any other host. `libraries/ruby/build.sh` already derives the extension name from `RbConfig::CONFIG["DLEXT"]`. `libraries/ruby/.cargo/config.toml` holds the `dynamic_lookup` flags, and no Mac build has proved them. `setup-ruby.sh` and `toolchain.env` name no Mac toolchain. `check.sh` calls GNU `timeout` on lines 110 to 124. Error-index row R5-37 says the Mac build recipe is not runnable as written, and 0108 moves the R Mac run here too.

Asked: a Mac toolchain setup under `~/.cache/thinkthen-toolchains/`, pinned by sha256 as the Linux one is. The `dynamic_lookup` flags proved by a Mac build. A per-file bound in `check.sh` that does not rely on GNU `timeout`. One recorded Mac run of the Ruby check. The same Mac run for every other surface that ships a Mac build.

Done when: `libraries/ruby/check.sh` passes on a Mac, the run is recorded, and R5-37 closes with that record.

## 10. The release workflow and CI

Today: `gate.yml` runs only by hand, under Ian's 2026-09-22 ruling. Both workflows were disabled by hand on GitHub the same day. `pages.yml` still names a push trigger on `site/**`. Its actions use tags such as `actions/checkout@v4`, while `gate.yml` pins every action to a commit. The 2026-09-24 plan says the site needs a deploy path Ian approves. The README's Gates section still says `gate.yml` "runs the first four rungs on every push and every pull request", and that sentence is wrong.

Asked: a release workflow started by a version tag. Ian's 2026-09-25 ruling above authorizes GitHub Actions for release publishing in items 4 to 7; the ordinary gate remains manual. Every action gets pinned to a commit. The release workflow does not change in the cycle that ships. A full dry release to a draft runs before the real one.

Done when: the release workflow runs a dry release from a tag, `pages.yml` pins its actions, and the README's Gates sentence matches `gate.yml`.

## 11. Smoke tests and the release checklist

Today: `sdlc/scripts/package` proves the source package alone. `sdlc/planning/mainline-readiness-2026-09-23.md` says "the source-package check is not a release installer." No checklist exists.

Asked: each archive, wheel, gem, and package runs the conformance cases under replay before it is published. BioMCP's 0.9.0 wheel crashed on one command while its release binary did not. The checklist counts a surface check that reports "not run" as a failure, per ticket 0111.

Done when: the release job installs each built file into a clean place, runs the replay cases against it, and stops on any failure or "not run".

## 12. The version bump and one version check

Today: every package reads 0.0.1: `crates/thinkthen`, `libraries/python`, `libraries/typescript`, `libraries/ruby` through the crate, `libraries/r` `DESCRIPTION`, `libraries/c`, `libraries/rust`, `databases/sqlite`, `databases/postgresql`, and the PostgreSQL control file. No script compares them. The four registries already hold a 0.0.1 placeholder, so 0.0.1 is spent there. The 2026-09-20 version issue had planned the placeholders at 0.0.0.

Asked: one number on every surface, per the 2026-09-20 ruling. A version-sync check compares every package file, the C header, the extensions, and the site against `crates/thinkthen/Cargo.toml`, as BioMCP's `scripts/check-version-sync.sh` does. The release job fails on a mismatch. The release sets 0.1.0 everywhere.

Done when: the version-sync check runs in the ladder and the release job, and every surface reads 0.1.0 at the release tag.

## 13. A first run with no key

Today: the install page says "Ask for a key first. It can take a day to arrive." `--replay` opens no connection and reads no key, and `demos/27-test-with-no-network` shows it. No release archive carries a recording. Stumble-register row 18 stays open.

Asked: the release archive or the README's first example carries one small recording folder and the input it answers. A new user runs a real command under `--replay` with no key and no network.

Done when: a clean install with no key runs the README's first example from the shipped recording and prints the recorded answer.

## 14. The README rows

Today: `README.md` has no install section. It names `THINKTHEN_API_KEY` and says nothing about who issues a key or what a judgment costs. It says the exit code "works in a shell `if`" and has no exit-code table. `specification/channels.md` holds seven codes, 0 to 6. The README has no badges. Stumble-register rows 1, 2, and 7 stay open.

Asked: an install section above the first example with the Homebrew line and the download script. One paragraph on who issues the key, the listed price, and the one address that swaps the backend. A table of the outcomes and their exit codes near the top. Three badges: the gate, the version, and the license.

Done when: the README carries all four, and stumble-register rows 1, 2, and 7 close with the commit.

## 15. Community files and repository settings

Today: `LICENSE` exists. `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `CHANGELOG.md`, `CITATION.cff`, and an issue template do not exist.

Asked: add all six before the repository goes public. The changelog names breaking changes under their own heading. The issue template asks for the command, the output, and the version. Fill the About box, the topics, and the social preview image the same day.

Done when: the six files exist on main, and the repository settings are filled.

## Already done

- Gate on push: ticket 0100 (`d7d2197a`) made `gate.yml` run only by hand and closed `closed/2026-09-22-stop-github-actions-on-push.md`.
- Docs deploy proof: `c29e4452` added `pages.yml` with the official Pages actions, `site/public/CNAME`, `site/public/version.json`, and a poll that fails the job until the site serves the commit.
- Install lines chosen: `site/src/data/catalog.mjs` and `site/src/pages/install.astro` show the Homebrew line and the download script from the 2026-09-21 ruling, with one row per surface.
- Ask for a key first: `install.astro` carries the note that a key can take a day and that `--dry-run` works without one.
- Name claims: the name `thinkthen` is claimed on crates.io, PyPI, RubyGems, and npm at 0.0.1, published by hand.
- One number for 0.1: `closed/2026-09-20-the-first-release-is-0-1-on-every-surface.md` closed on 2026-09-22 with the ruling recorded.
- Second backend: `closed/2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md` closed on 2026-09-22. Experiment 220 ran a local model behind the wire shape with no change to the repository.
- Launch items built: `annotate` (ticket 0015), `tag` (ticket 0036 and ADR 0029), CSV and TSV reading (ticket 0037), and `status` (ticket 0063).
- Ruby extension name: `libraries/ruby/build.sh` derives it from `RbConfig::CONFIG["DLEXT"]` (ticket 0112).
- The one-dollar marketing authorization of 2026-09-20 is a record and holds no release work. Marketing moved to a separate agent on 2026-09-24.

## Moved to the docs issue

- One page on exit code 1 under `set -e`: the `if` form, the `||` form, what `--raw` prints, and the host that treats exit 2 as a block.
- Four how-tos that need no code: split a file into piles by a `choose` label, a tool-call guard for a coding agent with the three-line exit-code mapping, the long-lived loop from ticket 0024, and text split into paragraphs before a verb reads it.
- Verb hints: a guessed verb such as `thinkthen grep` names `filter`. A CSV file piped into `--jsonl` gets a hint. A file path given as a second argument names standard input and `--input`.
- Site lessons from BioMCP: a broken-link check, `llms.txt` and `llms-full.txt` with a Markdown twin of every page, how-to pages built from `demos/`, and search, a sitemap, and social cards.
- Terminal recordings scripted with VHS under `--replay`.
- A published skill file that teaches an agent the verbs, the exit codes, and `--dry-run`.

Two launch items belong to neither issue and stay with their own open issues: the cap on questions in one request (`closed/2026-09-20-packing-rows-into-one-request-measured.md`) and the `interface-audit.md` refresh (`2026-09-25-docs-how-tos-and-spec-claims-owed.md`).

## Distribution decision input (2026-09-28, from the language-port program)

Registries to add for the language ports, grounded in the nine consumer-proof experiments:

- **NuGet** — C# wrapper. Can carry per-platform natives via runtime identifiers, matching the thin-wrapper + native-archive model.
- **Packagist** — PHP wrapper. Auto-publishes from a GitHub tag via webhook; near-zero setup.
- **Maven Central** — one artifact for Java, Kotlin, and Scala. Namespace verification and GPG signing required; JitPack is a workable stopgap that builds from GitHub tags with no registration.

No new registry needed elsewhere: Go publishes by pushing a tag (proxy.golang.org and pkg.go.dev index it automatically); Zig depends by URL (`build.zig.zon`), so GitHub is the registry; SwiftPM installs from a GitHub tag, with an optional Swift Package Index listing; Ada can join Alire later if asked; no registry exists for COBOL, and CocoaPods fits Apple-platform Objective-C, not the GNU/Linux port.

Base for all eleven languages: a README per language under `libraries/<lang>/` plus GitHub Releases assets (per-platform native archives and wrapper sources). The website install path and every registry entry point at the same release assets; registry entries wrap the thin wrapper and reference the native archive rather than embedding engine builds. Ian's one-time account setup is tracked in his to-dos (`notes/todos/2026-09-28-register-thinkthen-on-nuget-packagist-maven-central.md`).

Dart addition (2026-09-28, Ian): a Dart/Flutter port joins the program (local experiment 300) and publishes through **pub.dev**, which replaces the "no registry needed" answer for that language: trusted publishing links the package to this GitHub repository, or a token secret carries the release job. Iconography for all consumer languages now sits in the mktg deck library (mktg `qf-language-icons`); showing the new marks in the Beatles bench deck is tracked in Ian's to-dos.
