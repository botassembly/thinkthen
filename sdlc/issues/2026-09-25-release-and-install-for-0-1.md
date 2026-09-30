# Release and install for 0.1

Status: open. Owner: ticket 0128, whose Phase 3b (Ian's first rehearsal dispatch) and Phase 4 (Ian's release run) remain. Shortened 2026-09-30. Ian's ruling 10 of 2026-09-30: no public release before 0.1, and 0.1 waits for every surface and binding. Local and internal releases are fine. The eleven C-door language packages merged in on 2026-09-30; see "Language packages" below.

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
5. **Language registries.** NuGet for C#, Packagist for PHP, Maven Central for the JVM, and pub.dev for Dart. Go, Zig and SwiftPM install from a GitHub tag. Ian's one-time account setup is in his to-dos. Each package's remaining items are under "Language packages" below.
6. **Package proofs carried from closed issues.** Each needs a passing receipt on its target.
   - Panic secrecy (tickets 0306 and 0310): macOS C and SQLite packages, DuckDB ARM64 and macOS packages, and the Python, Ruby, TypeScript and R target packages.
   - Token cap (ticket 0311): installed-package proof on each surface.
   - Package gates (ticket 0313): the Python pandas 2 lane needs one networked `uv pip install pandas==2.3.3` on each machine that runs it, or its check exits 77 and a release counts that as a failure. The lane passed on this host on 2026-09-30 after the install. Minimal child environments for the release families, with UTF-8 arguments, compiler overrides, unchanged dependency locks and no ambient `PYTHONWARNINGS`.
   - SQL settings (tickets 0149 and 0157): DuckDB's ARM64 and macOS packages still take the C API path.
7. **macOS package defects.** Ticket 0336 closed `closed/2026-09-30-postgresql-extension-does-not-build-on-macos.md`; the rehearsal's macOS jobs run its full check. Ticket 0337 closed the Objective-C header collision, which no release runner met. The macOS static library and R package still export SQLite's names: `2026-09-30-static-library-exports-sqlite-symbols.md`, which the rehearsal's macOS jobs must also pass.

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
