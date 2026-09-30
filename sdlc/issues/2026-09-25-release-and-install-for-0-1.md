# Release and install for 0.1

Status: open. Owner: ticket 0128, whose Phase 3b (Ian's first rehearsal dispatch) and Phase 4 (Ian's release run) remain. Shortened 2026-09-30. Ian's ruling 10 of 2026-09-30: no public release before 0.1, and 0.1 waits for every surface and binding. Local and internal releases are fine.

## Rulings that govern it

- 2026-09-20: the first release is 0.1 on every surface; until then versions run 0.0.1 upward. One crate, `thinkthen`. The install path is a `curl` script that downloads a GitHub release.
- 2026-09-21: one Homebrew line and one download script.
- 2026-09-24: package names, the tap, the site and papers are Ian's.
- 2026-09-24 (ticket 0111): a surface check that reports "not run" fails the release.
- 2026-09-25: release publishing runs on GitHub Actions with trusted publishing to crates.io, PyPI, npm and RubyGems; the gate stays manual. The tap lives under `botassembly`. R ships through R-universe. Rust Polars is the `polars` feature of `thinkthen` (ticket 0130).
- 2026-09-26: a fresh one-commit history before the public release, after Ian names the go-live commit.
- 2026-09-28: the Maven Central artifact is `io.github.botassembly:thinkthen-jvm`.

## Built

Ticket 0128 phases 1, 2 and 3a landed `release.yml` (dispatch only, every action pinned, trusted-publishing jobs), `install.sh` and its site copy, `installer-test`, `release-pack`, `release-smoke`, the version check `sdlc/scripts/versions`, the first-run sample built from demo 27, the community files, the issue template, and a pinned `pages.yml`. Tickets 0261 to 0273 added the language package and workflow parts. Ticket 0319 proved the command archive and the no-key first run on this host through a loopback install.

## Still open

1. **Phase 3b.** Ian dispatches `rehearse` on main. Every job passes on all four targets with zero "not run". This run is the macOS proof of record and closes error-index row R5-37, the Ruby Mac build.
2. **Phase 4.** The version bump to 0.1.0 everywhere. The README install section with the Homebrew line and the download script, the badges, and the site's install lines, all in one commit. Trusted publishing to crates.io, PyPI, npm and RubyGems, then deleting the local tokens. The tap formula. R-universe. The history reset. The public install checks.
3. **Homebrew on Linux.** `sdlc/scripts/release-workflow` writes a formula with Linux branches, and the post-publish check runs `brew install` only on the M5. The site calls Homebrew a Mac option. Whether Linux Homebrew is a promised channel is Ian's call.
4. **The site's R line.** `site/src/data/catalog.mjs` shows `install.packages("thinkthen")` with no `repos=`, which resolves against CRAN. It needs the R-universe repository once Ian sets it up. This belongs to marketing.
5. **Language registries.** NuGet for C#, Packagist for PHP, Maven Central for the JVM, and pub.dev for Dart. Go, Zig and SwiftPM install from a GitHub tag. Ian's one-time account setup is in his to-dos. Each package's remaining items are in `2026-09-26-language-packages-need-a-release.md`.
6. **Package proofs carried from closed issues.** Each needs a passing receipt on its target.
   - Panic secrecy (tickets 0306 and 0310): macOS C and SQLite packages, DuckDB ARM64 and macOS packages, and the Python, Ruby, TypeScript and R target packages.
   - Token cap (ticket 0311): installed-package proof on each surface.
   - Package gates (ticket 0313): the Python pandas 2 lane needs one networked `uv pip install pandas==2.3.3` on each machine that runs it, or its check exits 77 and a release counts that as a failure. The lane passed on this host on 2026-09-30 after the install. Minimal child environments for the release families, with UTF-8 arguments, compiler overrides, unchanged dependency locks and no ambient `PYTHONWARNINGS`.
   - SQL settings (tickets 0149 and 0157): DuckDB's ARM64 and macOS packages still take the C API path.
7. **macOS package defects.** `2026-09-30-postgresql-extension-does-not-build-on-macos.md`. Ticket 0337 closed the Objective-C header collision, which no release runner met.

Done when a tagged 0.1.0 release installs from every named channel on a clean machine, and each package passed the replay cases on its own platform before it was published.
