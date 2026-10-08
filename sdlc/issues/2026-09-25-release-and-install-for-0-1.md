# Release and install for 0.1

Status: open only for carried-forward platform/public-install obligations under 0425 and 0398 in 0.2. Ticket 0128 completed the 0.1 release; the tables and Phase 4 requirements below preserve historical scope.

Milestone: 0.2

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

1. **Phase 3b.** Done 2026-10-02. Rehearsal runs 36945370940 (main `94d0500c0`), 36998358908 (main `f65faea4e`, version 0.1.0) and 37010060315 (`release/0.1` at `4e880cdf6`) passed every job through `draft` on all four targets, with no surface "not run". They are the macOS proof of record and close error-index row R5-37: the Ruby gem built and passed on both Macs. Ticket 0128 phase 3b records every attempt.
2. **Phase 4.** Steps 2 to 4 are done. Ticket 0387's release commit `ff7120f89` set 0.1.0 everywhere and wrote the README install section. The site owner's quick fix `1dd3d971d` moved the site's Rust example to 0.1.0. Both are on `release/0.1`. Still open: trusted publishing to crates.io, PyPI, npm and RubyGems, then deleting the local tokens. The tap formula. The history reset. The public install checks, including R-universe and Packagist.
3. **Homebrew on Linux.** `sdlc/scripts/release-workflow` writes a formula with Linux branches, and the post-publish check runs `brew install` only on the M5. The site calls Homebrew a Mac option. Whether Linux Homebrew is a promised channel is Ian's call.
4. **The site's R line.** Done 2026-10-03 by ticket 0392. `site/src/data/catalog.mjs` now names the R-universe repository before CRAN. R-universe had not built the package on 2026-10-03; the public install check confirms it.
5. **Language registries.** NuGet for C#, Packagist for PHP, Maven Central for the JVM, and pub.dev for Dart. Ticket 0355 built their jobs and rehearse paths and the Go module tag. The `registries` job first passed in rehearse mode in run 36853575250. The documentation team owns the accounts and settings its "Needs from the release setup" table lists. Two docs team asks of 2026-10-01 still change `release.yml`: the `nuget` job moves to NuGet trusted publishing, and the pub.dev upload moves to a job started by Ian's `v0.1.0` tag push. Until they land, the `nuget` job reads a secret that will not exist and the `pub` job uses Google Cloud, which Ian ruled out. Zig, Swift, Ada, COBOL and Objective-C install from GitHub Releases. Each package's remaining items are under "Language packages" below.
6. **Package proofs carried from closed issues.** Done 2026-10-02, except the package gates below. Run 37010060315's four smoke jobs ran ticket 0374's panic-isolation and token-cap cases on every installed library and the command. Tickets 0226, 0227 and 0299 closed on that proof. Tickets 0149 and 0157 still need their settings and request-size cases on native Intel macOS and macOS 15. The old list kept one more item:
   - Package gates (ticket 0313): the Python pandas 2 lane needs one networked `uv pip install pandas==2.3.3` on each machine that runs it, or its check exits 77 and a release counts that as a failure. The lane passed on this host on 2026-09-30 after the install. Minimal child environments for the release families, with UTF-8 arguments, compiler overrides, unchanged dependency locks and no ambient `PYTHONWARNINGS`. The release smoke runs the installed wheel, not the pandas 2 lane, so this item belongs to the `surfaces` sweep.
7. **macOS package defects.** Done; kept for the record. Ticket 0336 closed `closed/2026-09-30-postgresql-extension-does-not-build-on-macos.md`. Ticket 0337 closed the Objective-C header collision. Ticket 0351 closed `closed/2026-09-30-static-library-exports-sqlite-symbols.md`, with its M5 proof. The rehearsal's macOS jobs run these checks like any other.

The tag ruleset and Actions billing no longer block the rehearsal or the release. The repository is public now. Rulesets apply to it, and standard GitHub-hosted runners cost nothing for a public repository.

## Language packages

Merged on 2026-09-30 from eleven "consumer proof needs a supported package" issues, now in `closed/`, then into this issue the same day. Owner: ticket 0128 phases 3b and 4, with the workflow tickets 0268 to 0273, whose static code passed review and whose runner runs remain open.

### What exists

Ticket 0249 landed each package's source under `libraries/<lang>/` with its own `check.sh`, and the [integration closure](../records/0249-integration-closure.md) accepted all eleven. Tickets 0261 to 0266 built local release packages for each. Each package passes its product check on Ubuntu 24.04.3 x86_64 against the current C header. Each typed route returns owned facts (tickets 0277, 0279, 0280, 0282). Nothing is published.

### What every package still needs

1. A rebuild at the final release commit.
2. A real `ubuntu-24.04` GitHub Actions build and release.
3. Checksummed native archives on the GitHub release.
4. Installation through the language's channel, as listed below.
5. Proof on each other host the package claims.

### Each language

| Language | Package form and channel | Also open | Limits to keep in the docs | Build record, experiment |
| --- | --- | --- | --- | --- |
| Zig 0.15.2 | Source package plus a separate native archive; GitHub is the registry (`build.zig.zon`) | Other hosts | Shared mode embeds the library path and needs a rebuild if it moves. Static mode is not a fully static executable. Static mode links with LLD (issue `2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md`) | `0249-swift-zig-build.md`, 273 |
| Go 1.22 | Go module with cgo; module tag, proxy.golang.org | Module tags, other hosts | The wrapper pins the OS thread around each call and error copy. A context deadline returns code 3 and a cancel returns code 5. Static C is not a fully static executable | `0249-go-cpp-build.md`, 274 |
| JVM (Java 21, Kotlin, Scala) | JARs plus a shared native archive; Maven Central as `io.github.botassembly:thinkthen-jvm` | Maven publication, other hosts | Java 21 FFM is a preview. Virtual threads need the calling-thread pin. Concurrent engine close is unsupported. The one-MiB JSON bound is a refusal. The FFM loader cannot use a static archive. Kotlin and Scala do not repeat every Java matrix check | `0249-csharp-jvm-build.md`, 289 |
| C# (.NET 8) | `Botassembly.ThinkThen` NuGet package; native archive separate | NuGet publication, other hosts | None beyond the shared list | `0249-csharp-jvm-build.md`, 290 |
| PHP 8.3 | Composer package with `ext-ffi`; Packagist | Packagist, other hosts, in-flight cancellation | PHP cannot fire a token while a blocking FFI call holds the VM. Only a pre-fired token works; a worker-process design is an open choice | `0249-php-build.md`, 291 |
| COBOL (GnuCOBOL 4) | Copybook facade plus a C JSON tokenizer; GitHub release only | Other hosts | Single-threaded CALL cannot fire an in-flight token. Pre-fired tokens and deadlines work | `0249-cobol-build.md`, 292 |
| Ada (GNAT 13) | GNAT `.gpr` source package; Alire later if asked | Other hosts | None beyond the shared list | `0249-ada-objc-cobol-build.md`, 293 |
| Swift 6.4 | SwiftPM with a system-library target; GitHub tag | macOS consumer proof | A checkout build needs one manual copy of the C header (ticket 0332) | `0249-swift-zig-build.md`, 294 |
| Objective-C (GNU gobjc) | Source package, no Foundation; GitHub release | Apple Objective-C, which needs its own design | None beyond the shared list | `0249-ada-objc-cobol-build.md`, 295 |
| Dart 3.13 and Flutter | `thinkthen_dart` on pub.dev; native archive separate | pub.dev dry run and trusted publishing, other Flutter hosts | No `path:` dependencies at publish. One native archive per platform. No untested runtime download. The README names the archive, version, library discovery and platforms | `0249-dart-build.md`, 300 |
| C++17 | Header-only CMake package with `find_package`; GitHub release | Direct CMake install, other hosts | Decimal and exponent doubles follow the documented parser rules | `0249-go-cpp-build.md`, 301 |

### Shared limits

Each package was proved on one Linux host with one toolchain version, against synthetic loopback replies. No runtime ABI identity check exists. Sanitizers do not cover Rust allocations. No live model quality is claimed. A consumer's ABI check derives the export list from the header at each rebuild and never pins a count. Experiment folders under `experiments/` hold the stage evidence; the closed issues hold the full history.

## Done when

Done when a tagged 0.1.0 release installs from every named channel on a clean machine, and each package passed the replay cases on its own platform before it was published.

Phase 4 also runs each site install page's command once on a clean machine after the first publish. `SURFACES` in `site/src/data/catalog.mjs` holds every line. Owner: ticket 0128's release run. Added 2026-09-30 at the site team's request. A package rename is announced to the site team first, because the site build checks each page's package name against the binding's metadata.

## Reconciliation, 2026-10-08

[0128](../tickets/0128-release-and-install.md) records the completed public 0.1.2 release and its actual channel checks. Public install text still names 0.1.2. This issue no longer asks to publish 0.1 or finish its old Phase 4. Final 0.2 Windows/macOS qualification, public package installation and release QA remain with [0425](../tickets/0425-sdk-consistency-0-2-plan.md) and [0398](../tickets/0398-release-safety.md). The original failure/platform limits below remain history, not new 0.2 qualification. Ian’s 2026-10-08 release hold governs every candidate tag, workflow, release branch and publication. Native Windows cache admission/coexistence remains explicitly owed under [0474](../tickets/0474-recognize-cache-admission.md) and [0480](../tickets/0480-cache-convert-replay-coexistence.md).
