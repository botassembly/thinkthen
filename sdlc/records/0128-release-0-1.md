# 0128: the 0.1 release

Ticket 0128 phase 4 names this record. It records the three release runs, what each registry holds, and what stays open. Ticket 0391 fixed the failures of the first run.

## The checklist

| Step | Owner | Result |
| --- | --- | --- |
| 1. Ian's setup list | Ian | Done. Milestone 0.1 exit criterion 3 lists the registry setup. Ian added the crates.io, RubyGems, PyPI and npm trusted publishers between the two runs |
| 2. Rehearsal on main | Ian | Done: run 36998358908 on main `f65faea4e` |
| 3. The release commit | Agent, coordinator | Done: ticket 0387 at `ff7120f89` |
| 4. Rehearsal on the release commit | Ian | Done: run 37010060315 from `release/0.1` at `4e880cdf6`. For 0.1.1, run 37048376945 from `release/0.1` at `18458f33b`. For 0.1.2, run 37126990511 from `release/0.1` at `abac3ce61`, the commit before ticket 0395's cherry-pick |
| 5. Tag and dispatch | Ian, agent | Done three times: `v0.1.0` at `b691a2bc6` (run 37035814818), `v0.1.1` at `9463cef05` (run 37059415069) and `v0.1.2` at `08328c9c0` (run 37130570517). The agent tagged and dispatched 0.1.2 under Ian's approval of 2026-10-03 |
| 6. Approve each publish job and check each registry | Ian, agent | Done for 0.1.1 and 0.1.2. For 0.1.2, Ian's first approval did not register; at Ian's "approve it", the agent approved the `release` environment. The registry list below gives each result |
| 7. Run `pages.yml` | Agent | Done. Pages run 37141801797 deployed main `8f5310e9c` (ticket 0396) on 2026-10-03, and the live `install.sh` names 0.1.2 |
| 8. The public install checks | Agent | Done 2026-10-03. Every live channel installs 0.1.1 and answers offline, except `gem install` on macOS and R-universe. With 0.1.2, `gem install` on macOS passed on the M5. R-universe built 0.1.2, and its Linux binary installed with no Rust in a fresh R 4.6.1 container. See "Public install checks" and the 0.1.2 section below |
| 9. Delete the four local registry tokens and the rehearsal drafts | Ian | Done. Every rehearsal draft is deleted. Ian said on 2026-10-03 that he made no upload tokens, so none exist to delete |

## 0.1.0: run 37035814818

Ian tagged `v0.1.0` at `b691a2bc6` on `release/0.1` and dispatched release mode on 2026-10-02.

- Every build and smoke job passed, and so did `draft`.
- Maven Central, pub.dev and the Homebrew tap published 0.1.0.
- crates.io and RubyGems failed because neither had a trusted publisher yet.
- PyPI failed because its action was pinned to a tag object, not a commit.
- npm failed because `npm publish` read a bare relative path as a GitHub repository.
- NuGet answered 400 because the push sent no protocol header.
- `publish` never ran, so the GitHub release stayed a draft. That draft was later deleted.

The `v0.1.0` tag stays. The Go module proxy already holds it for the root module path, and a tag the proxy holds cannot be withdrawn. Maven Central and pub.dev never replace a version, so the next release had to be 0.1.1.

## 0.1.1: run 37059415069

Ticket 0391 fixed the PyPI pin, the npm path and the NuGet header on main. The fixes were cherry-picked to `release/0.1`, and the version bump landed there alone. Checkpoint `checkpoint/surfaces/2026-10-02-2` passed on `9463cef05`. Rehearsal 37048376945 from `release/0.1` was clean. Ian added the four missing trusted publishers. Then Ian tagged `v0.1.1` at `9463cef05` and dispatched release mode.

1. The first attempt published to every registry except npm. The `npm` job (job 111040245433) failed with `npm error 403 403 Forbidden - PUT https://registry.npmjs.org/thinkthen - OIDC permission denied for this action`. The npm trusted publisher allowed only staged publishing.
2. Ian allowed direct publishing on npmjs.com. The rerun's `npm` job (job 111193360031) published `thinkthen@0.1.1` with a provenance statement.
3. The `publish` job (job 111194288139) then made GitHub release v0.1.1 public, marked Latest.

The release page holds the command, C, SQLite, DuckDB and PostgreSQL 16 archives for all four targets, each with a `.sha256` file. It also holds the wheels, the gems, the npm package, the first-run sample and the Linux x86-64 language packages. The extension files are named `thinkthen-duckdb-0.1.1-<target>.tar.gz`, `thinkthen-sqlite-0.1.1-<target>.tar.gz` and `thinkthen-postgresql16-0.1.1-<target>.tar.gz`. The targets are `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin` and `aarch64-apple-darwin`.

## 0.1.2: run 37130570517

Ticket 0394 drops the macOS version from the macOS gem platforms, so one gem serves every macOS. Ticket 0395 makes the R package build on R-universe: `tools/config.R` runs `cargo update --workspace`, and `check.sh` builds against a crates.io stand-in. Both landed on main and were cherry-picked to `release/0.1`, and the 0.1.2 bump landed there alone. Checkpoint `checkpoint/surfaces/2026-10-03-1` passed on `abac3ce61`. Rehearsal 37126990511 from `release/0.1` at `abac3ce61` was clean, and its two Mac smoke jobs installed the new `arm64-darwin` and `x86_64-darwin` gems. Ticket 0395's cherry-pick then landed as `08328c9c0`. Neither the checkpoint nor the rehearsal ran it. The release run built and smoked `08328c9c0` itself, and the R package's first real test is its R-universe build. The agent tagged `v0.1.2` at `08328c9c0` and dispatched release mode.

Every build, smoke and publish job passed on the first attempt. The `publish` job made GitHub release v0.1.2 public and created tag `libraries/go/v0.1.2`.

R-universe synced after the release went public. Its build run 37142825335 passed. In a fresh `rocker/r-ver` container with R 4.6.1 and no cargo, `install.packages("thinkthen")` from `https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/` installed 0.1.2, and `library(thinkthen)` loaded it. The plain address `https://botassembly.r-universe.dev` served the source package there, which failed for lack of cargo. The container and its image were removed afterwards.

The M5 check, 2026-10-03: with Ruby 3.4.11 on macOS 26, `gem install thinkthen` in a scratch gem folder installed `thinkthen-0.1.2-arm64-darwin`, and `require "thinkthen"` loaded it. The scratch folder was removed.

## What each registry holds

| Registry | Name | Version |
| --- | --- | --- |
| crates.io | `thinkthen` | 0.1.2 |
| PyPI | `thinkthen`, four abi3 wheels | 0.1.2 |
| npm | `thinkthen` | 0.1.2 |
| RubyGems | `thinkthen`, gems for `x86_64-linux`, `aarch64-linux`, `x86_64-darwin` and `arm64-darwin` | 0.1.2. The `ruby` platform gem is still the 0.0.1 placeholder |
| NuGet | `Botassembly.ThinkThen` | 0.1.2 |
| Maven Central | `io.github.botassembly:thinkthen-jvm` | 0.1.2 |
| pub.dev | `thinkthen_dart` | 0.1.2 |
| Homebrew | `botassembly/homebrew-thinkthen` formula | 0.1.2 |
| Packagist | `botassembly/thinkthen` | v0.1.2 |
| Go | tag `libraries/go/v0.1.2` | 0.1.2 |
| R-universe | `thinkthen` under `botassembly` | 0.1.2. Build run 37142825335 in `r-universe/botassembly` built the source package and every Linux, macOS and Windows binary. Only the WebAssembly build failed. ThinkThen does not ship that target |
| GitHub | release v0.1.2 | public, Latest |

Each registry also keeps its earlier 0.1 versions.

## Public install checks

The install-check builder owns this section and ticket 0128 phase 4 step 8. It writes its results here. This record's author leaves this section to that builder.

Results, 2026-10-03. Each check installed one channel the way its README or install page says, read the version, and replayed the release's first-run sample with no key and no network. The Linux checks ran in fresh x86-64 containers from public images, one per channel, removed afterwards. The macOS checks ran on the M5, an Apple Silicon Mac on macOS 26.4, in scratch folders removed afterwards. The library checks called `decide` with the sample's question and `report.txt`, with the engine's replay setting on the sample's `recording` folder. Every answer was true with `requests_sent` 0. Each library's no-key call failed with the usage error `no key is set; configure an API key for the engine (THINKTHEN_API_KEY)`.

| Channel | Install | Where | Version seen | Result |
| --- | --- | --- | --- | --- |
| Download script | `curl -fsSL https://thinkthen.dev/install.sh \| sh` | Ubuntu 24.04; macOS 26 | `thinkthen 0.1.1` | Pass |
| Homebrew | `brew install botassembly/thinkthen/thinkthen` | Linuxbrew image | `thinkthen 0.1.1` | Pass. The M5 has no Homebrew. Each of the formula's four URLs matches its sha256 and the release's `.sha256` file |
| Cargo | `cargo install thinkthen` | Rust 1.99 | `thinkthen 0.1.1` | Pass |
| crates.io library | `cargo add thinkthen` | Rust 1.99 | `cargo tree`: 0.1.1 | Pass |
| PyPI | `pip install thinkthen`, `uv add thinkthen` | Python 3.13 on Linux and macOS | 0.1.1 | Pass |
| npm | `npm install thinkthen` | Node 22 on Linux, Node 26 on macOS | 0.1.1 | Pass |
| RubyGems | `gem install thinkthen` | Ruby 3.4.11 on Linux | 0.1.1 `x86_64-linux` | Pass |
| RubyGems | `gem install thinkthen` | Ruby 3.4.6 on macOS 26 | 0.0.1 `ruby` | Fail. RubyGems matches `arm64-darwin-24` only to darwin 24, and macOS 26 is darwin 25. Ticket 0394 |
| NuGet | `dotnet add package Botassembly.ThinkThen --version 0.1.1`, with the C archive | .NET 8 | 0.1.1 | Pass |
| Maven Central | `io.github.botassembly:thinkthen-jvm:0.1.1`, with the C archive | Java 21, Maven 3 | 0.1.1 | Pass. The Kotlin and Scala classifier JARs resolve |
| pub.dev | `dart pub add thinkthen_dart`, with the C archive | Dart stable | 0.1.1 | Pass |
| Packagist | `composer require botassembly/thinkthen`, with the C archive | PHP 8.3 with FFI | v0.1.1 | Pass |
| Go | `go get github.com/botassembly/thinkthen/libraries/go@v0.1.1`, with the C archive | Go 1.27 | v0.1.1 | Pass |
| R-universe | none yet | | | Not built for 0.1.1. R-universe built 0.1.2, as the 0.1.2 section records. Its last update ran on 2026-10-02, before any release existed, and failed to find `*release`. It had not run again by the time of these checks |
| C library | `thinkthen-c-0.1.1-x86_64-unknown-linux-gnu.tar.gz` | gcc on Ubuntu 24.04 | header and `thinkthen.pc`: 0.1.1 | Pass |
| SQLite | `thinkthen-sqlite-0.1.1-x86_64-unknown-linux-gnu.tar.gz` | SQLite 3.50.4 | archive name only | Pass. Ubuntu 24.04's SQLite 3.45.1 refuses the extension with a message naming 3.50.0 |
| DuckDB | `thinkthen-duckdb-0.1.1-x86_64-unknown-linux-gnu.tar.gz` | DuckDB 1.5.5 | `extension_version` 0.1.1 | Pass |
| PostgreSQL 16 | `thinkthen-postgresql16-0.1.1-x86_64-unknown-linux-gnu.tar.gz` | PostgreSQL 16.15 | `extversion` 0.1.1 | Pass |

All 41 release files match their `.sha256` files.

Findings:

1. The macOS gems name darwin 24, so a Ruby on any other macOS installs the 0.0.1 placeholder. Ticket 0394 drops the version from the macOS gem platform. Users get the fix with 0.1.2.
2. The placeholders catch older hosts with no message. Ruby 3.3 on Linux installs the 0.0.1 `ruby` gem. macOS's own Python 3.9 installs the 0.0.1 PyPI release. The open placeholder issue holds the choices, and a yank is Ian's step.
3. The install pages for SQLite, DuckDB and PostgreSQL named no release archive and no steps to place it. The Python and Ruby pages named no version floor. This Quick Fix adds each.
4. The R page's line pointed at CRAN, which holds no copy. Ticket 0392 points it at R-universe.
5. Python 0.1.1 has no `thinkthen.__version__`. The package metadata gives the version.

## What stays open

- The R install line on Linux. R-universe serves Linux the source package, which needs Rust's `cargo` and `rustc`. The Linux binaries live at a separate address. A quick fix names that address on the install page and makes the package say when cargo is missing.
- Issue `2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md`: the `ruby` platform gem on RubyGems is still the 0.0.1 placeholder. A host with no platform gem, or a Ruby older than 3.4, installs it with no message. Ian has no RubyGems yank step today, and he ruled on 2026-10-03 that this waits unless it is a security problem. It is not one: the placeholder holds no code that runs.
- Ticket 0393: npm publishes with staged publishing, so Ian can turn direct publishing off again on npmjs.com.
- Issue `2026-10-03-polars-deadline-test-races-its-deadline-under-load.md`: one Polars test failed once under load during the 0.1.2 checkpoint.

## Closed decisions

- The history reset. Ian's ruling of 2026-09-26 asked for a fresh one-commit history before public release. The repository went public with its full history, and the tags, the Go module proxy and the registries' provenance statements now name its commits. On 2026-10-03 Ian left the choice to the agent. The agent dropped the step, because a reset would break every published 0.1 version's provenance.
- The local registry tokens. Ian made none, so phase 4 step 9 has nothing to delete. Trusted publishing needs no token.
