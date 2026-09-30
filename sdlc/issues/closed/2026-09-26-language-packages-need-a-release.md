# Eleven C-door language packages need a release

Status: Closed on 2026-09-30. Merged into `../2026-09-25-release-and-install-for-0-1.md`, "Language packages", which ticket 0128 owns.

## What exists

Ticket 0249 landed each package's source under `libraries/<lang>/` with its own `check.sh`, and the [integration closure](../../records/0249-integration-closure.md) accepted all eleven. Tickets 0261 to 0266 built local release packages for each. Each package passes its product check on Ubuntu 24.04.3 x86_64 against the current C header. Each typed route returns owned facts (tickets 0277, 0279, 0280, 0282). Nothing is published.

## What every package still needs

1. A rebuild at the final release commit.
2. A real `ubuntu-24.04` GitHub Actions build and release.
3. Checksummed native archives on the GitHub release.
4. Installation through the language's channel, as listed below.
5. Proof on each other host the package claims.

## Each language

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

## Shared limits

Each package was proved on one Linux host with one toolchain version, against synthetic loopback replies. No runtime ABI identity check exists. Sanitizers do not cover Rust allocations. No live model quality is claimed. A consumer's ABI check derives the export list from the header at each rebuild and never pins a count. Experiment folders under `experiments/` hold the stage evidence; the closed issues hold the full history.
